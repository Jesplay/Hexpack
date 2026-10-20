//! Window app: draw patterns, recognise and check them, and pack them compactly on demand.

use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::time::{Duration, SystemTime};

use eframe::egui::{self, Align, Align2, Color32, FontId, Layout, Margin, PointerButton, Pos2, Rect, RichText, Rounding, Sense, Shape, Stroke, Vec2};

use crate::check::{Checker, Level, Row};
use crate::db::Db;
use crate::geom::{self, Pt, DIRS};
use crate::{fx, pack, program, render};

const SQ3: f32 = 1.732_050_8;
const RATIOS: [(&str, f64, f64); 5] = [("16:9", 16.0, 9.0), ("16:10", 16.0, 10.0), ("4:3", 4.0, 3.0), ("1:1", 1.0, 1.0), ("21:9", 21.0, 9.0)];
const PALETTE: [Color32; 8] = [
    Color32::from_rgb(140, 130, 235),
    Color32::from_rgb(60, 190, 145),
    Color32::from_rgb(235, 130, 80),
    Color32::from_rgb(90, 160, 235),
    Color32::from_rgb(220, 170, 60),
    Color32::from_rgb(230, 110, 160),
    Color32::from_rgb(140, 200, 80),
    Color32::from_rgb(180, 180, 175),
];
const RED: Color32 = Color32::from_rgb(240, 95, 95);
const YELLOW: Color32 = Color32::from_rgb(232, 192, 80);
const GREEN: Color32 = Color32::from_rgb(110, 205, 130);
const MUTED: Color32 = Color32::from_rgb(130, 120, 160);

struct Pat {
    sig: String,
    path: Vec<Pt>,
}

#[derive(Default)]
struct Analysis {
    /// (name, recognised, quoted)
    labels: Vec<(String, bool, bool)>,
    rows: Vec<Row>,
    msgs: Vec<(usize, Level, String)>,
    costs: Vec<String>,
    final_items: Vec<String>,
    final_unknown: bool,
    errors: usize,
    warns: usize,
}

#[derive(PartialEq, Clone, Copy)]
enum Tab {
    Resolution,
    Patterns,
    Issues,
}

pub struct App {
    vault: String,
    db: Option<Db>,
    db_err: String,
    pats: Vec<Pat>,
    drawing: Option<Vec<Pt>>,
    origin: Vec2,
    scale: f32,
    canvas: Vec2,
    an: Analysis,
    selected: Option<usize>,
    /// pattern under the mouse (canvas or side panel), recomputed every frame
    hovered: Option<usize>,
    ratio: usize,
    secs: f32,
    packing: Option<Receiver<Vec<Vec<Pt>>>>,
    pack_info: String,
    fit_pending: bool,
    import_open: bool,
    import_text: String,
    import_err: Vec<String>,
    vault_open: bool,
    help_open: bool,
    colorful: bool,
    show_numbers: bool,
    tab: Tab,
    search: String,
    search_open: bool,
    search_rect: Rect,
    /// (query, hits) of the last search
    search_cache: (String, Vec<usize>),
    menu_target: Option<usize>,
    /// short message at the bottom (text, time)
    toast: Option<(String, f64)>,
}

pub fn run(vault: PathBuf, preload: Option<String>) -> Result<(), String> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1400.0, 860.0]).with_min_inner_size([900.0, 560.0]).with_title("hexpack"),
        ..Default::default()
    };
    eframe::run_native(
        "hexpack",
        options,
        Box::new(move |cc| {
            fx::apply_theme(&cc.egui_ctx);
            let mut app = App::new(vault);
            if let Some(text) = preload {
                app.import_text = text;
                app.import();
            }
            Ok(Box::new(app))
        }),
    )
    .map_err(|e| e.to_string())
}

fn sig_of_path(path: &[Pt]) -> String {
    let dir = |a: Pt, b: Pt| DIRS.iter().position(|d| (a.0 + d.0, a.1 + d.1) == b).unwrap();
    let dirs: Vec<usize> = path.windows(2).map(|w| dir(w[0], w[1])).collect();
    dirs.windows(2).map(|w| geom::turn_char((w[1] + 6 - w[0]) % 6)).collect()
}

fn is_adjacent(a: Pt, b: Pt) -> bool {
    DIRS.iter().any(|d| (a.0 + d.0, a.1 + d.1) == b)
}

fn edge(a: Pt, b: Pt) -> (Pt, Pt) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

/// Position of a grid point in units (distance between points in a row = √3).
fn unit_pos(p: Pt) -> Vec2 {
    let (q, r) = (p.0 as f32, p.1 as f32);
    Vec2::new(SQ3 * (q + r / 2.0), 1.5 * r)
}

/// Colour of a stack entry by its type.
fn iota_color(t: &str) -> Color32 {
    let t = t.trim();
    if t.contains('|') {
        Color32::from_rgb(150, 150, 190)
    } else if t == "[code]" {
        Color32::from_rgb(230, 120, 200)
    } else if t.starts_with('[') {
        Color32::from_rgb(175, 135, 245)
    } else if t == "num" || t.parse::<f64>().is_ok() {
        Color32::from_rgb(95, 165, 240)
    } else if t == "vec" {
        Color32::from_rgb(70, 200, 160)
    } else if t == "entity" {
        Color32::from_rgb(240, 155, 85)
    } else if t == "bool" {
        YELLOW
    } else if t == "pattern" {
        fx::PINK
    } else {
        MUTED
    }
}

fn chip(ui: &mut egui::Ui, text: &str) {
    let c = iota_color(text);
    egui::Frame::none()
        .fill(c.gamma_multiply(0.16))
        .stroke(Stroke::new(1.0_f32, c.gamma_multiply(0.45)))
        .rounding(Rounding::same(5.0))
        .inner_margin(Margin::symmetric(6.0, 1.0))
        .show(ui, |ui| {
            ui.label(RichText::new(text).monospace().size(12.0).color(fx::lighten(c, 0.35)));
        });
}

fn stack_chips(ui: &mut egui::Ui, items: &[String], unknown: bool) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(4.0, 4.0);
        if unknown {
            chip(ui, "…?");
        }
        if items.is_empty() && !unknown {
            ui.label(RichText::new("empty").small().color(MUTED));
        }
        for it in items {
            chip(ui, it);
        }
    });
}

fn controls_help(ui: &mut egui::Ui) {
    egui::Grid::new("controls").num_columns(2).spacing([16.0, 6.0]).show(ui, |ui| {
        for (k, v) in [
            ("Left drag", "draw a pattern"),
            ("Drag back", "remove the last stroke"),
            ("Right / middle drag", "move the view"),
            ("Mouse wheel", "zoom"),
            ("Click", "select a pattern"),
            ("Delete", "delete the selected pattern"),
            ("Ctrl+Z", "undo the last pattern"),
            ("Right click", "pattern menu (docs, copy, delete)"),
            ("Ctrl+F", "search patterns"),
        ] {
            ui.label(RichText::new(k).monospace().color(fx::ACCENT));
            ui.label(v);
            ui.end_row();
        }
    });
}

/// Small coloured dot (the default font has no ● glyph).
fn dot(ui: &mut egui::Ui, col: Color32) {
    let (r, _) = ui.allocate_exact_size(Vec2::splat(12.0), Sense::hover());
    ui.painter().circle_filled(r.center(), 4.5, col);
}

fn badge(ui: &mut egui::Ui, n: usize, col: Color32) {
    egui::Frame::none().fill(col.gamma_multiply(0.22)).rounding(Rounding::same(9.0)).inner_margin(Margin::symmetric(6.0, 0.0)).show(ui, |ui| {
        ui.label(RichText::new(format!("{n}")).monospace().size(12.0).color(fx::lighten(col, 0.4)));
    });
}

/// Row with hover/selection background. Returns (hovered, clicked).
fn hover_row(ui: &mut egui::Ui, id: usize, active: bool, add: impl FnOnce(&mut egui::Ui)) -> (bool, bool) {
    let bg = ui.painter().add(Shape::Noop);
    let inner = ui.vertical(|ui| {
        ui.add_space(2.0);
        add(ui);
        ui.add_space(2.0);
    });
    let rect = inner.response.rect.expand2(Vec2::new(4.0, 0.0));
    // hover only: buttons inside the row still get their own clicks
    let resp = ui.interact(rect, ui.id().with(("row", id)), Sense::hover());
    let over = resp.contains_pointer();
    let clicked = over && ui.input(|i| i.pointer.primary_clicked());
    let fill = if active {
        Some(fx::ACCENT.gamma_multiply(0.16))
    } else if over {
        Some(Color32::from_white_alpha(10))
    } else {
        None
    };
    if let Some(f) = fill {
        ui.painter().set(bg, Shape::rect_filled(rect, Rounding::same(6.0), f));
    }
    (over, clicked)
}

impl App {
    fn new(vault: PathBuf) -> Self {
        let mut app = App {
            vault: vault.display().to_string(),
            db: None,
            db_err: String::new(),
            pats: Vec::new(),
            drawing: None,
            origin: Vec2::new(500.0, 400.0),
            scale: 26.0,
            canvas: Vec2::new(1000.0, 800.0),
            an: Analysis::default(),
            selected: None,
            hovered: None,
            ratio: 0,
            secs: 3.0,
            packing: None,
            pack_info: String::new(),
            fit_pending: false,
            import_open: false,
            import_text: String::new(),
            import_err: Vec::new(),
            vault_open: false,
            help_open: false,
            colorful: false,
            show_numbers: true,
            tab: Tab::Resolution,
            search: String::new(),
            search_open: false,
            search_rect: Rect::NOTHING,
            search_cache: (String::new(), Vec::new()),
            menu_target: None,
            toast: None,
        };
        app.load_db();
        app.vault_open = app.db.is_none();
        app
    }

    fn load_db(&mut self) {
        match Db::load(std::path::Path::new(&self.vault)) {
            Ok(db) => {
                self.db = Some(db);
                self.db_err.clear();
            }
            Err(e) => {
                self.db = None;
                self.db_err = e;
            }
        }
        self.analyze();
    }

    fn analyze(&mut self) {
        let Some(db) = &self.db else {
            self.an = Analysis::default();
            return;
        };
        let sigs: Vec<String> = self.pats.iter().map(|p| p.sig.clone()).collect();
        let prog = program::from_sigs(&sigs, db);
        let mut chk = Checker::new(db, &prog);
        chk.run();
        let mut msgs: Vec<(usize, Level, String)> = prog.errors.iter().map(|(l, t)| (*l, Level::Error, t.clone())).collect();
        msgs.extend(chk.msgs.iter().map(|m| (m.line, m.level, m.text.clone())));
        msgs.sort_by_key(|m| (m.0, std::cmp::Reverse(m.1 as u8)));
        self.an = Analysis {
            labels: prog.drawn.iter().map(|d| (d.label.clone(), d.known, d.quoted)).collect(),
            errors: msgs.iter().filter(|m| m.1 == Level::Error).count(),
            warns: msgs.iter().filter(|m| m.1 == Level::Warn).count(),
            msgs,
            costs: std::mem::take(&mut chk.costs),
            final_items: std::mem::take(&mut chk.final_items),
            final_unknown: chk.final_unknown,
            rows: std::mem::take(&mut chk.rows),
        };
    }

    fn label_of(&self, sig: &str) -> String {
        match &self.db {
            Some(db) => match program::recognize(sig, db) {
                program::Item::Open(_) => "Introspection".into(),
                program::Item::Close(_) => "Retrospection".into(),
                program::Item::Escape(_) => "Consideration".into(),
                program::Item::Pat { label, .. } => label,
                program::Item::Stack(_) => String::new(),
            },
            None => sig.to_string(),
        }
    }

    fn occupied(&self) -> HashSet<Pt> {
        self.pats.iter().flat_map(|p| p.path.iter().copied()).collect()
    }

    fn to_screen(&self, min: Pos2, p: Pt) -> Pos2 {
        min + self.origin + unit_pos(p) * self.scale
    }

    /// Nearest grid point and its distance in pixels.
    fn from_screen(&self, min: Pos2, pos: Pos2) -> (Pt, f32) {
        let v = (pos - min - self.origin) / self.scale;
        let r = v.y / 1.5;
        let q = v.x / SQ3 - r / 2.0;
        let (x, z) = (q, r);
        let y = -x - z;
        let (mut rx, ry, mut rz) = (x.round(), y.round(), z.round());
        let (dx, dy, dz) = ((rx - x).abs(), (ry - y).abs(), (rz - z).abs());
        if dx > dy && dx > dz {
            rx = -ry - rz;
        } else if dy <= dz {
            rz = -rx - ry;
        }
        let p = (rx as i32, rz as i32);
        (p, (self.to_screen(min, p) - pos).length())
    }

    fn pattern_at(&self, p: Pt) -> Option<usize> {
        self.pats.iter().position(|pat| pat.path.contains(&p))
    }

    fn start_pack(&mut self) {
        if self.pats.is_empty() || self.packing.is_some() {
            return;
        }
        let sigs: Vec<String> = self.pats.iter().map(|p| p.sig.clone()).collect();
        let (_, rw, rh) = RATIOS[self.ratio];
        let secs = self.secs as f64;
        let seed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(1);
        let (tx, rx) = channel();
        std::thread::spawn(move || {
            let shapes: Vec<pack::Shape> = sigs.iter().map(|s| pack::Shape::new(s)).collect();
            let (lay, _) = pack::pack(&shapes, (rw, rh), Duration::from_secs_f64(secs), seed);
            let paths = render::placed(&shapes, &lay).into_iter().map(|p| p.path).collect();
            let _ = tx.send(paths);
        });
        self.packing = Some(rx);
        self.drawing = None;
    }

    fn poll_pack(&mut self) {
        let Some(rx) = &self.packing else { return };
        if let Ok(paths) = rx.try_recv() {
            self.packing = None;
            if paths.len() == self.pats.len() {
                for (pat, path) in self.pats.iter_mut().zip(paths) {
                    pat.path = path;
                }
                let ps: Vec<render::Placed> = self.pats.iter().map(|p| render::Placed { path: p.path.clone(), rot: 0 }).collect();
                let b = render::bounds(&ps);
                let rows = b.3 - b.2 + 1;
                let cols = (b.1 - b.0) / 2 + 1;
                let ratio = ((b.1 - b.0) as f64 * 0.866) / ((b.3 - b.2).max(1) as f64 * 1.5);
                self.pack_info = format!("{rows} × {cols} points · {ratio:.2} : 1");
                self.fit_pending = true;
            }
        }
    }

    fn fit_view(&mut self) {
        let pts: Vec<Vec2> = self.pats.iter().flat_map(|p| p.path.iter().map(|&q| unit_pos(q))).collect();
        if pts.is_empty() {
            return;
        }
        let (mut lo, mut hi) = (pts[0], pts[0]);
        for p in &pts {
            lo = lo.min(*p);
            hi = hi.max(*p);
        }
        let size = (hi - lo).max(Vec2::splat(1.0));
        self.scale = (self.canvas.x * 0.82 / size.x).min(self.canvas.y * 0.82 / size.y).clamp(6.0, 60.0);
        self.origin = self.canvas / 2.0 - (lo + hi) / 2.0 * self.scale;
    }

    fn remove(&mut self, i: usize) {
        if i < self.pats.len() {
            self.pats.remove(i);
            self.selected = None;
            self.analyze();
        }
    }

    fn undo(&mut self) {
        if let Some(i) = self.pats.len().checked_sub(1) {
            self.remove(i);
        }
    }

    fn export_text(&self) -> String {
        let mut out = String::new();
        let mut depth = 0usize;
        for (i, (label, known, _)) in self.an.labels.iter().enumerate() {
            if label == "Retrospection" {
                depth = depth.saturating_sub(1);
            }
            let line = if *known { label.clone() } else { format!("sig: {}", self.pats[i].sig) };
            out.push_str(&"  ".repeat(depth));
            out.push_str(&line);
            out.push('\n');
            if label == "Introspection" {
                depth += 1;
            }
        }
        out
    }

    fn import(&mut self) {
        let Some(db) = &self.db else { return };
        let prog = program::parse(&self.import_text, db);
        self.import_err = prog.errors.iter().map(|(l, e)| format!("line {l}: {e}")).collect();
        if prog.drawn.is_empty() {
            return;
        }
        self.pats = prog.drawn.iter().map(|d| Pat { sig: d.sig.clone(), path: Vec::new() }).collect();
        self.selected = None;
        self.analyze();
        self.start_pack();
    }

    fn pattern_color(&self, i: usize) -> Color32 {
        let (known, quoted) = self.an.labels.get(i).map(|l| (l.1, l.2)).unwrap_or((true, false));
        let err = self.an.msgs.iter().any(|m| m.0 == i + 1 && m.1 == Level::Error);
        if !known || err {
            RED
        } else if self.colorful {
            PALETTE[i % PALETTE.len()]
        } else if quoted {
            fx::QUOTED_GRAD.0
        } else {
            fx::ACCENT
        }
    }

    fn toggle_select(&mut self, i: usize) {
        self.selected = if self.selected == Some(i) { None } else { Some(i) };
    }

    // ---------- toolbar ----------

    fn toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            ui.label(RichText::new("hexpack").size(19.0).color(fx::ACCENT));
            ui.add_space(10.0);
            self.search_field(ui);
            ui.separator();

            let busy = self.packing.is_some();
            let arrange = egui::Button::new(RichText::new(if busy { "Arranging…" } else { "Arrange" }).color(Color32::WHITE)).fill(Color32::from_rgb(98, 70, 165));
            if ui.add_enabled(!busy && !self.pats.is_empty(), arrange).on_hover_text("Pack all patterns as compactly as possible").clicked() {
                self.start_pack();
            }
            if busy {
                ui.spinner();
            }
            egui::ComboBox::from_id_salt("ratio").width(64.0).selected_text(RATIOS[self.ratio].0).show_ui(ui, |ui| {
                for (i, r) in RATIOS.iter().enumerate() {
                    ui.selectable_value(&mut self.ratio, i, r.0);
                }
            });
            ui.add(egui::DragValue::new(&mut self.secs).range(0.5..=30.0).speed(0.1).suffix(" s").max_decimals(1)).on_hover_text("Time budget for arranging");
            ui.separator();

            if ui.button("Undo").on_hover_text("Ctrl+Z").clicked() {
                self.undo();
            }
            if ui.button("Clear").clicked() {
                self.pats.clear();
                self.selected = None;
                self.pack_info.clear();
                self.analyze();
            }
            if ui.button("Fit").on_hover_text("Fit the view to all patterns").clicked() {
                self.fit_view();
            }
            ui.separator();
            if ui.button("Import…").on_hover_text("Paste a hex as text").clicked() {
                self.import_open = true;
            }
            if ui.add_enabled(!self.pats.is_empty(), egui::Button::new("Copy")).on_hover_text("Copy the hex as text").clicked() {
                ui.ctx().copy_text(self.export_text());
            }
            ui.separator();
            ui.toggle_value(&mut self.show_numbers, "Numbers");
            ui.toggle_value(&mut self.colorful, "Colors").on_hover_text("One color per pattern instead of the mod's purple");

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add_space(4.0);
                let help = ui.add(egui::Button::new(RichText::new("?").strong()).min_size(Vec2::new(26.0, 26.0)).rounding(Rounding::same(13.0)));
                if help.on_hover_ui(controls_help).clicked() {
                    self.help_open = !self.help_open;
                }
                let (col, txt) = match &self.db {
                    Some(db) => (GREEN, format!("{} patterns", db.entries.len())),
                    None => (RED, "vault not found".to_string()),
                };
                if ui.add(egui::Label::new(RichText::new(txt).small().color(col)).sense(Sense::click())).on_hover_text("Vault folder").clicked() {
                    self.vault_open = true;
                }
                dot(ui, col);
            });
        });
    }

    // ---------- side panel ----------

    fn side(&mut self, ui: &mut egui::Ui) {
        ui.add_space(10.0);
        self.summary(ui);
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            let issues = self.an.msgs.len();
            for (tab, name) in [(Tab::Resolution, "Resolution".to_string()), (Tab::Patterns, format!("Patterns {}", self.pats.len())), (Tab::Issues, format!("Issues {issues}"))] {
                let on = self.tab == tab;
                let txt = RichText::new(name).color(if on { Color32::WHITE } else { MUTED });
                if ui.add(egui::Button::new(txt).fill(if on { fx::ACCENT.gamma_multiply(0.25) } else { Color32::TRANSPARENT }).stroke(Stroke::NONE)).clicked() {
                    self.tab = tab;
                }
            }
        });
        ui.add_space(2.0);
        ui.separator();

        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            ui.add_space(4.0);
            match self.tab {
                Tab::Resolution => self.resolution_tab(ui),
                Tab::Patterns => self.patterns_tab(ui),
                Tab::Issues => self.issues_tab(ui),
            }
            ui.add_space(12.0);
        });
    }

    fn summary(&mut self, ui: &mut egui::Ui) {
        egui::Frame::none().fill(Color32::from_rgb(30, 26, 45)).rounding(Rounding::same(10.0)).inner_margin(Margin::same(12.0)).show(ui, |ui| {
            ui.set_width(ui.available_width());
            if self.pats.is_empty() {
                ui.label(RichText::new("Nothing drawn yet").strong());
                ui.label(RichText::new("Hold the left mouse button and drag across the dots, or use Import.").small().color(MUTED));
                return;
            }
            let (txt, col) = match (self.an.errors, self.an.warns) {
                (0, 0) => ("Looks good".to_string(), GREEN),
                (0, w) => (format!("{w} warning{}", if w == 1 { "" } else { "s" }), YELLOW),
                (e, _) => (format!("{e} error{}", if e == 1 { "" } else { "s" }), RED),
            };
            ui.horizontal(|ui| {
                dot(ui, col);
                ui.label(RichText::new(txt).strong().color(col));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if !self.pack_info.is_empty() {
                        ui.label(RichText::new(&self.pack_info).small().color(MUTED));
                    }
                });
            });
            ui.add_space(4.0);
            ui.label(RichText::new("Resolves to  (top of stack on the right)").small().color(MUTED));
            stack_chips(ui, &self.an.final_items, self.an.final_unknown);
            if !self.an.costs.is_empty() {
                ui.add_space(4.0);
                let n = self.an.costs.len();
                ui.label(RichText::new(format!("{n} spell{} with media cost, see Issues", if n == 1 { "" } else { "s" })).small().color(MUTED));
            }
        });
    }

    fn resolution_tab(&mut self, ui: &mut egui::Ui) {
        if self.an.rows.len() <= 1 {
            ui.label(RichText::new("Each pattern's effect on the stack will show up here.").color(MUTED));
            return;
        }
        let mut hovered = None;
        let mut clicked = None;
        for (ri, row) in self.an.rows.iter().enumerate() {
            let indent = row.depth as f32 * 16.0;
            if row.header {
                ui.horizontal(|ui| {
                    ui.add_space(indent);
                    ui.label(RichText::new(format!("loop · {}", row.label)).small().italics().color(fx::QUOTED_GRAD.0));
                });
                continue;
            }
            let idx = row.line.checked_sub(1);
            let active = idx.is_some() && (idx == self.selected || idx == self.hovered);
            let col = idx.map(|i| self.pattern_color(i)).unwrap_or(MUTED);
            let (h, c) = hover_row(ui, ri, active, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(indent);
                    if row.depth > 0 {
                        let r = ui.available_rect_before_wrap();
                        ui.painter().vline(r.left() - 8.0, r.y_range(), Stroke::new(1.5_f32, fx::QUOTED_GRAD.0.gamma_multiply(0.35)));
                    }
                    match idx {
                        Some(i) => {
                            badge(ui, i + 1, col);
                            ui.label(RichText::new(&row.label).strong());
                        }
                        None => {
                            ui.label(RichText::new("start").small().color(MUTED));
                        }
                    }
                });
                ui.horizontal(|ui| {
                    ui.add_space(indent + 30.0);
                    stack_chips(ui, &row.items, row.unknown);
                });
            });
            if h {
                hovered = idx;
            }
            if c {
                clicked = idx;
            }
        }
        self.hovered = hovered.or(self.hovered);
        if let Some(i) = clicked {
            self.toggle_select(i);
        }
    }

    fn patterns_tab(&mut self, ui: &mut egui::Ui) {
        if self.pats.is_empty() {
            ui.label(RichText::new("No patterns yet.").color(MUTED));
            return;
        }
        let mut remove = None;
        let mut docs = None;
        let mut hovered = None;
        let mut clicked = None;
        for i in 0..self.pats.len() {
            let (label, quoted) = self.an.labels.get(i).map(|l| (l.0.clone(), l.2)).unwrap_or_default();
            let col = self.pattern_color(i);
            let active = Some(i) == self.selected || Some(i) == self.hovered;
            let sig = self.pats[i].sig.clone();
            let entry = self.entry_for(&sig);
            let (h, c) = hover_row(ui, i, active, |ui| {
                ui.horizontal(|ui| {
                    badge(ui, i + 1, col);
                    ui.label(RichText::new(&label).color(if quoted { fx::lighten(fx::QUOTED_GRAD.0, 0.3) } else { ui.visuals().text_color() }));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.add(egui::Button::new(RichText::new("×").color(MUTED)).frame(false)).on_hover_text("Delete").clicked() {
                            remove = Some(i);
                        }
                        if let Some(e) = entry {
                            if ui.add(egui::Button::new(RichText::new("docs").small().color(MUTED)).frame(false)).on_hover_text("Open in Obsidian").clicked() {
                                docs = Some(e);
                            }
                        }
                        ui.label(RichText::new(if sig.is_empty() { "–".to_string() } else { sig.clone() }).monospace().size(12.0).color(MUTED));
                    });
                });
            });
            if h {
                hovered = Some(i);
            }
            if c {
                clicked = Some(i);
            }
        }
        self.hovered = hovered.or(self.hovered);
        if let Some(e) = docs {
            self.open_docs(e);
        } else if let Some(i) = remove {
            self.remove(i);
        } else if let Some(i) = clicked {
            self.toggle_select(i);
        }
    }

    fn issues_tab(&mut self, ui: &mut egui::Ui) {
        if self.an.msgs.is_empty() && self.an.costs.is_empty() {
            ui.label(RichText::new("No issues.").color(MUTED));
            return;
        }
        let mut hovered = None;
        let mut clicked = None;
        for (mi, (line, lvl, text)) in self.an.msgs.iter().enumerate() {
            let col = match lvl {
                Level::Error => RED,
                Level::Warn => YELLOW,
                Level::Info => MUTED,
            };
            let idx = line.checked_sub(1);
            let active = idx.is_some() && (idx == self.selected || idx == self.hovered);
            // "Name: text" -> name in bold, text below
            let (head, body) = text.split_once(": ").unwrap_or(("", text.as_str()));
            let (h, c) = hover_row(ui, mi, active, |ui| {
                ui.horizontal(|ui| {
                    dot(ui, col);
                    if let Some(i) = idx {
                        badge(ui, i + 1, col);
                    }
                    if !head.is_empty() {
                        ui.label(RichText::new(head).strong());
                    }
                });
                ui.horizontal_wrapped(|ui| {
                    ui.add_space(22.0);
                    ui.label(RichText::new(body).color(fx::lighten(col, 0.2)));
                });
            });
            if h {
                hovered = idx;
            }
            if c {
                clicked = idx;
            }
        }
        if !self.an.costs.is_empty() {
            ui.add_space(10.0);
            ui.label(RichText::new("Media cost").strong());
            for c in &self.an.costs {
                ui.label(RichText::new(c).small().color(MUTED));
            }
        }
        self.hovered = hovered.or(self.hovered);
        if let Some(i) = clicked {
            self.toggle_select(i);
        }
    }

    // ---------- canvas ----------

    fn canvas(&mut self, ui: &mut egui::Ui) {
        let (resp, painter) = ui.allocate_painter(ui.available_size(), Sense::click_and_drag());
        let rect = resp.rect;
        let min = rect.min;
        self.canvas = rect.size();
        if self.fit_pending {
            self.fit_pending = false;
            self.fit_view();
        }
        let busy = self.packing.is_some();

        // zoom around the mouse pointer
        if resp.hovered() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                let mouse = ui.input(|i| i.pointer.hover_pos()).unwrap_or(rect.center()) - min;
                let new = (self.scale * (1.0 + scroll * 0.0015)).clamp(5.0, 90.0);
                self.origin = mouse - (mouse - self.origin) * (new / self.scale);
                self.scale = new;
            }
        }
        // pan
        if resp.dragged_by(PointerButton::Secondary) || resp.dragged_by(PointerButton::Middle) {
            self.origin += resp.drag_delta();
        }

        // draw
        let hit = self.scale * 0.6;
        if !busy && resp.drag_started_by(PointerButton::Primary) {
            if let Some(pos) = resp.interact_pointer_pos() {
                let (p, d) = self.from_screen(min, pos);
                if d < hit && !self.occupied().contains(&p) {
                    self.drawing = Some(vec![p]);
                }
            }
        }
        if !busy && resp.dragged_by(PointerButton::Primary) && self.drawing.is_some() {
            if let Some(pos) = resp.interact_pointer_pos() {
                let (p, d) = self.from_screen(min, pos);
                let occ = self.occupied();
                let path = self.drawing.as_mut().unwrap();
                let last = *path.last().unwrap();
                if d < hit && p != last {
                    if path.len() >= 2 && path[path.len() - 2] == p {
                        path.pop();
                    } else if is_adjacent(last, p) && !occ.contains(&p) {
                        let used: HashSet<(Pt, Pt)> = path.windows(2).map(|w| edge(w[0], w[1])).collect();
                        if !used.contains(&edge(last, p)) {
                            path.push(p);
                        }
                    }
                }
            }
        }
        if resp.drag_stopped_by(PointerButton::Primary) {
            if let Some(path) = self.drawing.take() {
                if path.len() >= 2 {
                    let sig = sig_of_path(&path);
                    self.pats.push(Pat { sig, path });
                    self.selected = None;
                    self.analyze();
                }
            }
        }
        if resp.clicked() {
            if let Some(pos) = resp.interact_pointer_pos() {
                let (p, d) = self.from_screen(min, pos);
                self.selected = if d < hit { self.pattern_at(p) } else { None };
            }
        }

        // right click (without dragging) on a pattern: small menu
        if resp.secondary_clicked() {
            self.menu_target = resp.interact_pointer_pos().and_then(|pos| {
                let (p, d) = self.from_screen(min, pos);
                if d < hit {
                    self.pattern_at(p)
                } else {
                    None
                }
            });
        }
        if let Some(i) = self.menu_target.filter(|&i| i < self.pats.len()) {
            let sig = self.pats[i].sig.clone();
            let entry = self.entry_for(&sig);
            let label = self.an.labels.get(i).map(|l| l.0.clone()).unwrap_or_default();
            let mut action = None;
            resp.context_menu(|ui| {
                ui.label(RichText::new(format!("{}  {label}", i + 1)).strong());
                ui.separator();
                if ui.add_enabled(entry.is_some(), egui::Button::new("Open in Obsidian")).clicked() {
                    action = Some(0);
                    ui.close_menu();
                }
                if ui.button("Copy angles").clicked() {
                    ui.ctx().copy_text(sig.clone());
                    ui.close_menu();
                }
                if ui.button("Delete").clicked() {
                    action = Some(1);
                    ui.close_menu();
                }
            });
            match action {
                Some(0) => self.open_docs(entry.unwrap()),
                Some(1) => {
                    self.remove(i);
                    self.menu_target = None;
                }
                _ => {}
            }
        }

        // keyboard
        if !ui.ctx().wants_keyboard_input() {
            let (del, undo) = ui.input(|i| (i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace), i.modifiers.command && i.key_pressed(egui::Key::Z)));
            if del {
                if let Some(s) = self.selected {
                    self.remove(s);
                }
            }
            if undo {
                self.undo();
            }
        }

        // pattern under the mouse
        if self.drawing.is_none() {
            if let Some(pos) = resp.hover_pos() {
                let (p, d) = self.from_screen(min, pos);
                if d < hit {
                    if let Some(i) = self.pattern_at(p) {
                        self.hovered = Some(i);
                    }
                }
            }
        }

        self.paint(&painter, rect, &resp);
    }

    fn paint(&self, painter: &egui::Painter, rect: Rect, resp: &egui::Response) {
        let min = rect.min;
        let time = painter.ctx().input(|i| i.time);
        painter.rect_filled(rect, 0.0, fx::BG);

        // grid dots; near the mouse they grow and glow like in the mod
        let s = self.scale;
        let occ = self.occupied();
        let mouse = resp.hover_pos().or_else(|| resp.interact_pointer_pos());
        let r0 = ((rect.top() - min.y - self.origin.y) / (1.5 * s)).floor() as i32 - 1;
        let r1 = ((rect.bottom() - min.y - self.origin.y) / (1.5 * s)).ceil() as i32 + 1;
        let base_r = (s * 0.06).clamp(1.0, 2.2);
        let dot = Color32::from_rgb(92, 84, 128);
        for r in r0..=r1 {
            let base = (rect.left() - min.x - self.origin.x) / (SQ3 * s) - r as f32 / 2.0;
            let q0 = base.floor() as i32 - 1;
            let q1 = q0 + (rect.width() / (SQ3 * s)).ceil() as i32 + 2;
            for q in q0..=q1 {
                if occ.contains(&(q, r)) {
                    continue;
                }
                let p = self.to_screen(min, (q, r));
                let near = mouse.map(|m| (1.0 - (m - p).length() / (s * 4.0)).max(0.0)).unwrap_or(0.0);
                let col = fx::lerp(dot, fx::lighten(fx::ACCENT, 0.3), near);
                if near > 0.0 {
                    painter.circle_filled(p, base_r * (1.0 + 3.0 * near), col.gamma_multiply(0.22 * near));
                }
                painter.circle_filled(p, base_r * (1.0 + 1.3 * near), col.gamma_multiply(0.55 + 0.45 * near));
            }
        }

        // patterns
        let focus = self.selected.or(self.hovered);
        let w = (s * 0.12).clamp(1.6, 5.5);
        for (i, pat) in self.pats.iter().enumerate() {
            if pat.path.len() < 2 {
                continue;
            }
            let base = self.pattern_color(i);
            let grad = if base == RED {
                fx::ERROR_GRAD
            } else if self.colorful {
                (base, fx::lighten(base, 0.45))
            } else if base == fx::QUOTED_GRAD.0 {
                fx::QUOTED_GRAD
            } else {
                fx::MOD_GRAD
            };
            let on = focus == Some(i);
            let dim = if focus.is_some() && !on { 0.28 } else { 1.0 };
            let pts: Vec<Pos2> = pat.path.iter().map(|&p| self.to_screen(min, p)).collect();
            fx::zappy(painter, &pts, i as u32 + 1, time, s, if on { w * 1.35 } else { w }, grad, dim);

            if self.show_numbers {
                // small arrow on the first stroke, number a little away from the start
                let (a, b) = (pts[0], pts[1]);
                let dir = (b - a).normalized();
                let n = Vec2::new(-dir.y, dir.x);
                let tip = a + (b - a) * 0.55;
                let head = (s * 0.17).max(4.0);
                let mark = fx::lighten(grad.0, 0.25).gamma_multiply(dim);
                painter.add(Shape::convex_polygon(vec![tip, tip - dir * head + n * head * 0.6, tip - dir * head - n * head * 0.6], mark, Stroke::NONE));
                let at = a - dir * (s * 0.45);
                let rad = (s * 0.21).clamp(6.0, 11.0);
                painter.circle(at, rad, fx::BG.gamma_multiply(0.85), Stroke::new(1.0_f32, mark.gamma_multiply(0.8)));
                painter.text(at, Align2::CENTER_CENTER, format!("{}", i + 1), FontId::proportional((s * 0.26).clamp(8.0, 12.5)), fx::lighten(grad.0, 0.6).gamma_multiply(dim));
            }
        }

        // pattern being drawn: bright, with a beam to the pointer and live recognition
        if let Some(path) = &self.drawing {
            let pts: Vec<Pos2> = path.iter().map(|&p| self.to_screen(min, p)).collect();
            if pts.len() >= 2 {
                fx::zappy(painter, &pts, 999, time, s, w * 1.15, fx::DRAW_GRAD, 1.0);
            } else {
                painter.circle_filled(pts[0], w, fx::lighten(fx::ACCENT, 0.5));
            }
            if let Some(m) = resp.interact_pointer_pos() {
                fx::beam(painter, *pts.last().unwrap(), m, time, s, w);
                if path.len() >= 2 {
                    let sig = sig_of_path(path);
                    self.tag(painter, m + Vec2::new(16.0, 14.0), &self.label_of(&sig), &sig);
                }
            }
        } else if let (Some(i), Some(pos)) = (self.hovered, resp.hover_pos()) {
            if let Some(pat) = self.pats.get(i) {
                let label = self.an.labels.get(i).map(|l| l.0.clone()).unwrap_or_default();
                self.tag(painter, pos + Vec2::new(16.0, 14.0), &format!("{}  {label}", i + 1), &pat.sig);
            }
        }

        if self.pats.is_empty() && self.drawing.is_none() {
            painter.text(rect.center(), Align2::CENTER_CENTER, "Hold the left mouse button and drag across the dots", FontId::proportional(15.0), MUTED);
        }
    }

    fn tag(&self, painter: &egui::Painter, pos: Pos2, title: &str, sig: &str) {
        let mut job = egui::text::LayoutJob::default();
        job.append(title, 0.0, egui::TextFormat { font_id: FontId::proportional(13.5), color: Color32::WHITE, ..Default::default() });
        job.append(sig, 10.0, egui::TextFormat { font_id: FontId::monospace(12.0), color: MUTED, ..Default::default() });
        let galley = painter.layout_job(job);
        let r = Rect::from_min_size(pos, galley.size()).expand2(Vec2::new(8.0, 5.0));
        painter.rect(r, 6.0, Color32::from_rgba_unmultiplied(24, 20, 36, 235), Stroke::new(1.0_f32, fx::ACCENT.gamma_multiply(0.5)));
        painter.galley(pos, galley, Color32::WHITE);
    }

    fn import_window(&mut self, ctx: &egui::Context) {
        let mut open = self.import_open;
        let mut go = false;
        egui::Window::new("Import").open(&mut open).default_width(480.0).collapsible(false).show(ctx, |ui| {
            ui.label("One pattern per line, like in .hex files. Replaces everything and arranges it.");
            ui.label(RichText::new("numbers: num: 10    masks: mask: v-    raw angles: sig: qaq").small().color(MUTED));
            ui.add(egui::TextEdit::multiline(&mut self.import_text).code_editor().desired_rows(16).desired_width(f32::INFINITY));
            if ui.button("Import and arrange").clicked() {
                go = true;
            }
            for e in &self.import_err {
                ui.colored_label(RED, e);
            }
        });
        self.import_open = open;
        if go {
            self.import();
            if self.import_err.is_empty() {
                self.import_open = false;
            }
        }
    }

    fn vault_window(&mut self, ctx: &egui::Context) {
        let mut open = self.vault_open;
        let mut load = false;
        egui::Window::new("Vault").open(&mut open).default_width(520.0).collapsible(false).show(ctx, |ui| {
            ui.label("Folder of the Hexwoven Vault (the one that contains \"02 Patterns\").");
            ui.add(egui::TextEdit::singleline(&mut self.vault).desired_width(f32::INFINITY));
            if ui.button("Load").clicked() {
                load = true;
            }
            match &self.db {
                Some(db) => {
                    ui.colored_label(GREEN, format!("{} patterns loaded", db.entries.len()));
                }
                None => {
                    ui.colored_label(RED, &self.db_err);
                }
            }
        });
        self.vault_open = open;
        if load {
            self.load_db();
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_pack();
        // the lines shimmer: keep repainting
        ctx.request_repaint_after(Duration::from_millis(30));
        self.hovered = None;

        egui::TopBottomPanel::top("toolbar").exact_height(46.0).frame(egui::Frame::none().fill(fx::PANEL).inner_margin(Margin::symmetric(12.0, 6.0))).show(ctx, |ui| self.toolbar(ui));
        egui::SidePanel::right("side")
            .resizable(true)
            .default_width(400.0)
            .min_width(300.0)
            .frame(egui::Frame::none().fill(fx::PANEL).inner_margin(Margin::symmetric(12.0, 0.0)))
            .show(ctx, |ui| self.side(ui));
        egui::CentralPanel::default().frame(egui::Frame::none()).show(ctx, |ui| self.canvas(ui));
        if self.import_open {
            self.import_window(ctx);
        }
        if self.vault_open {
            self.vault_window(ctx);
        }
        if self.search_open && !self.search.trim().is_empty() {
            self.search_popup(ctx);
        }
        self.show_toast(ctx);
        if self.help_open {
            let mut open = true;
            egui::Window::new("Controls").open(&mut open).collapsible(false).resizable(false).anchor(Align2::RIGHT_TOP, Vec2::new(-16.0, 56.0)).show(ctx, controls_help);
            self.help_open = open;
        }
    }
}

// ---------- search, Obsidian, placing ----------

fn or_dash(s: &str) -> &str {
    if s.is_empty() {
        "–"
    } else {
        s
    }
}

/// `obsidian://` link that opens a file by its absolute path (in a new tab).
fn obsidian_url(file: &std::path::Path) -> String {
    let mut enc = String::new();
    for b in file.to_string_lossy().bytes() {
        if b.is_ascii_alphanumeric() || b"-_.~".contains(&b) {
            enc.push(b as char);
        } else {
            enc.push_str(&format!("%{b:02X}"));
        }
    }
    format!("obsidian://open?path={enc}&paneType=tab")
}

/// Opens a link with the program the OS has registered for it.
/// On Windows via ShellExecuteW: spawning a process fails once the app has detached
/// from its console after a double-click (invalid std handles).
#[cfg(windows)]
fn open_external(url: &str) -> std::io::Result<()> {
    #[link(name = "shell32")]
    extern "system" {
        fn ShellExecuteW(hwnd: *mut u8, op: *const u16, file: *const u16, params: *const u16, dir: *const u16, show: i32) -> isize;
    }
    let wide = |s: &str| s.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>();
    let (op, file) = (wide("open"), wide(url));
    let r = unsafe { ShellExecuteW(std::ptr::null_mut(), op.as_ptr(), file.as_ptr(), std::ptr::null(), std::ptr::null(), 1) };
    if r > 32 {
        Ok(())
    } else {
        Err(std::io::Error::other(format!("ShellExecute failed (code {r}), is Obsidian installed?")))
    }
}

#[cfg(not(windows))]
fn open_external(url: &str) -> std::io::Result<()> {
    use std::process::{Command, Stdio};
    #[cfg(target_os = "macos")]
    let mut cmd = Command::new("open");
    #[cfg(not(target_os = "macos"))]
    let mut cmd = Command::new("xdg-open");
    cmd.arg(url).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).spawn().map(|_| ())
}

/// Small glyph preview for the search results.
fn glyph_preview(ui: &mut egui::Ui, sig: &str, size: Vec2) {
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    let pts = pack::Shape::new(sig).path.swap_remove(0);
    let up: Vec<Vec2> = pts.iter().map(|&p| unit_pos(p)).collect();
    let (mut lo, mut hi) = (up[0], up[0]);
    for p in &up {
        lo = lo.min(*p);
        hi = hi.max(*p);
    }
    let span = (hi - lo).max(Vec2::splat(0.01));
    let inner = rect.shrink(5.0);
    let k = (inner.width() / span.x).min(inner.height() / span.y).min(14.0);
    let off = inner.center() - (lo + hi) / 2.0 * k;
    let sp: Vec<Pos2> = up.iter().map(|p| off + *p * k).collect();
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, Rounding::same(6.0), fx::BG);
    let n = sp.len() - 1;
    for i in 0..n {
        let col = fx::lerp(fx::ACCENT, fx::PINK, i as f32 / n.max(1) as f32);
        painter.line_segment([sp[i], sp[i + 1]], Stroke::new(1.8_f32, col));
    }
    painter.circle_filled(sp[0], 2.6, fx::lighten(fx::ACCENT, 0.5));
}

fn spiral(radius: i32) -> Vec<Pt> {
    let mut out = vec![(0, 0)];
    for k in 1..=radius {
        let (mut q, mut r) = (-k, k);
        for d in DIRS {
            for _ in 0..k {
                out.push((q, r));
                q += d.0;
                r += d.1;
            }
        }
    }
    out
}

impl App {
    /// Vault entry for a drawn signature (also for numbers and masks).
    fn entry_for(&self, sig: &str) -> Option<usize> {
        let db = self.db.as_ref()?;
        if let Some(&i) = db.by_sig.get(sig).and_then(|v| v.first()) {
            return Some(i);
        }
        if crate::literals::decode_number(sig).is_some() {
            return db.lookup("Numerical Reflection").ok();
        }
        if crate::literals::decode_mask(sig).is_some() {
            return db.lookup("Bookkeeper's Gambit").ok();
        }
        None
    }

    fn toast(&mut self, msg: impl Into<String>, ctx: &egui::Context) {
        self.toast = Some((msg.into(), ctx.input(|i| i.time)));
    }

    fn open_docs(&mut self, entry: usize) {
        let Some(db) = &self.db else { return };
        let e = &db.entries[entry];
        let (name, url) = (e.name.clone(), obsidian_url(&e.file));
        let msg = match open_external(&url) {
            Ok(()) => format!("Opening \"{name}\" in Obsidian"),
            Err(err) => format!("Could not open Obsidian: {err}"),
        };
        self.toast = Some((msg, f64::NAN));
    }

    /// Puts a pattern at the first free spot near the centre of the view (with some margin).
    fn add_pattern(&mut self, sig: String) {
        let pts = pack::Shape::new(&sig).path.swap_remove(0);
        let mut blocked: HashSet<Pt> = HashSet::new();
        for p in self.occupied() {
            blocked.insert(p);
            for d in DIRS {
                blocked.insert((p.0 + d.0, p.1 + d.1));
            }
        }
        let (center, _) = self.from_screen(Pos2::ZERO, (self.canvas / 2.0).to_pos2());
        let n = pts.len() as f32;
        let avg = (pts.iter().map(|p| p.0 as f32).sum::<f32>() / n, pts.iter().map(|p| p.1 as f32).sum::<f32>() / n);
        let base = (center.0 - avg.0.round() as i32, center.1 - avg.1.round() as i32);
        for (dq, dr) in spiral(60) {
            let off = (base.0 + dq, base.1 + dr);
            if pts.iter().all(|p| !blocked.contains(&(p.0 + off.0, p.1 + off.1))) {
                let path = pts.iter().map(|p| (p.0 + off.0, p.1 + off.1)).collect();
                self.pats.push(Pat { sig, path });
                self.selected = Some(self.pats.len() - 1);
                self.analyze();
                return;
            }
        }
    }

    fn search_results(&mut self) -> Vec<usize> {
        let q = self.search.trim().to_string();
        if self.search_cache.0 != q {
            let mut v: Vec<usize> = Vec::new();
            if let Some(db) = &self.db {
                // a signature like "qaq" finds the pattern directly
                if !q.is_empty() && q.chars().all(|c| "wedsaq".contains(c)) {
                    if let Some(&i) = db.by_sig.get(q.as_str()).and_then(|v| v.first()) {
                        v.push(i);
                    }
                }
                for i in db.search(&q) {
                    if !v.contains(&i) {
                        v.push(i);
                    }
                }
            }
            v.truncate(40);
            self.search_cache = (q, v);
        }
        self.search_cache.1.clone()
    }

    fn search_field(&mut self, ui: &mut egui::Ui) {
        let id = egui::Id::new("pattern-search");
        if ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::F)) {
            ui.memory_mut(|m| m.request_focus(id));
            self.search_open = true;
        }
        let resp = ui.add(egui::TextEdit::singleline(&mut self.search).id(id).hint_text("Search patterns   Ctrl+F").desired_width(230.0).margin(Margin::symmetric(8.0, 4.0)));
        self.search_rect = resp.rect;
        if resp.changed() || resp.gained_focus() {
            self.search_open = true;
        }
        if (resp.has_focus() || resp.lost_focus()) && ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.search.clear();
            self.search_open = false;
        }
        if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            if let Some(&e) = self.search_results().first() {
                self.add_entry(e, ui.ctx());
            }
            ui.memory_mut(|m| m.request_focus(id));
        }
    }

    fn add_entry(&mut self, e: usize, ctx: &egui::Context) {
        let Some(db) = &self.db else { return };
        let (sig, name) = (db.entries[e].sig.clone(), db.entries[e].name.clone());
        self.add_pattern(sig);
        self.toast(format!("Added {name}"), ctx);
    }

    fn search_popup(&mut self, ctx: &egui::Context) {
        let results = self.search_results();
        let Some(db) = &self.db else { return };
        let mut add = None;
        let mut docs = None;
        let area = egui::Area::new(egui::Id::new("search-popup")).order(egui::Order::Foreground).fixed_pos(self.search_rect.left_bottom() + Vec2::new(0.0, 8.0)).show(ctx, |ui| {
            egui::Frame::popup(ui.style()).fill(fx::PANEL).rounding(Rounding::same(10.0)).inner_margin(Margin::same(8.0)).show(ui, |ui| {
                ui.set_width(500.0);
                if results.is_empty() {
                    ui.label(RichText::new("No pattern found").color(MUTED));
                    return;
                }
                egui::ScrollArea::vertical().max_height(460.0).show(ui, |ui| {
                    for (k, &e) in results.iter().enumerate() {
                        let en = &db.entries[e];
                        let (_, c) = hover_row(ui, 100_000 + k, false, |ui| {
                            ui.horizontal(|ui| {
                                glyph_preview(ui, &en.sig, Vec2::new(48.0, 36.0));
                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new(&en.name).strong());
                                        ui.label(RichText::new(&en.modname).small().color(MUTED));
                                    });
                                    ui.label(RichText::new(format!("{}  ->  {}", or_dash(&en.input), or_dash(&en.output))).small().color(fx::lighten(MUTED, 0.25)));
                                });
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    if ui.add(egui::Button::new(RichText::new("docs").small()).frame(false)).on_hover_text("Open in Obsidian").clicked() {
                                        docs = Some(e);
                                    }
                                    ui.label(RichText::new(&en.sig).monospace().size(11.5).color(MUTED));
                                });
                            });
                        });
                        if c {
                            add = Some(e);
                        }
                    }
                });
                ui.add_space(2.0);
                ui.label(RichText::new("click or Enter: add to canvas    docs: open in Obsidian    Esc: close").small().color(MUTED));
            });
        });
        // clicking elsewhere closes the list
        let outside = ctx.input(|i| i.pointer.any_pressed() && i.pointer.interact_pos().is_some_and(|p| !area.response.rect.contains(p) && !self.search_rect.contains(p)));
        if outside {
            self.search_open = false;
        }
        if let Some(e) = docs {
            self.open_docs(e);
        } else if let Some(e) = add {
            self.add_entry(e, ctx);
        }
    }

    fn show_toast(&mut self, ctx: &egui::Context) {
        let now = ctx.input(|i| i.time);
        let Some((msg, t0)) = &mut self.toast else { return };
        if t0.is_nan() {
            *t0 = now;
        }
        if now - *t0 > 2.6 {
            self.toast = None;
            return;
        }
        let msg = msg.clone();
        egui::Area::new(egui::Id::new("toast")).order(egui::Order::Tooltip).anchor(Align2::CENTER_BOTTOM, Vec2::new(-180.0, -24.0)).interactable(false).show(ctx, |ui| {
            egui::Frame::popup(ui.style()).fill(Color32::from_rgb(36, 31, 54)).rounding(Rounding::same(8.0)).inner_margin(Margin::symmetric(12.0, 7.0)).show(ui, |ui| {
                ui.add(egui::Label::new(RichText::new(msg).color(Color32::WHITE)).extend());
            });
        });
    }
}