//! Hex Casting style visuals: glowing, shimmering lines between the nodes.

use eframe::egui::{self, Color32, Painter, Pos2, Shape, Stroke, Vec2};

pub const BG: Color32 = Color32::from_rgb(13, 11, 21);
pub const PANEL: Color32 = Color32::from_rgb(22, 19, 33);
pub const ACCENT: Color32 = Color32::from_rgb(179, 142, 243);
pub const PINK: Color32 = Color32::from_rgb(252, 119, 190);

/// Colour gradient (start, end) of a pattern.
pub type Grad = (Color32, Color32);

pub const MOD_GRAD: Grad = (ACCENT, PINK);
pub const QUOTED_GRAD: Grad = (Color32::from_rgb(110, 170, 255), Color32::from_rgb(190, 150, 255));
pub const ERROR_GRAD: Grad = (Color32::from_rgb(255, 90, 80), Color32::from_rgb(255, 170, 90));
pub const DRAW_GRAD: Grad = (Color32::from_rgb(240, 230, 255), ACCENT);

pub fn lerp(a: Color32, b: Color32, t: f32) -> Color32 {
    let f = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Color32::from_rgba_premultiplied(f(a.r(), b.r()), f(a.g(), b.g()), f(a.b(), b.b()), f(a.a(), b.a()))
}

pub fn lighten(c: Color32, t: f32) -> Color32 {
    lerp(c, Color32::WHITE, t)
}

fn hash(mut x: u32) -> f32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb_352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846c_a68b);
    x ^= x >> 16;
    x as f32 / u32::MAX as f32 * 2.0 - 1.0
}

/// When a pattern passes a node a second time, the later line bends just around it
/// (as in Hex Casting), so you can see which line came first and where it turns.
fn route(nodes: &[Pos2], scale: f32) -> Vec<Pos2> {
    let off = scale * 0.2;
    let mut out = Vec::with_capacity(nodes.len() + 4);
    for (i, &p) in nodes.iter().enumerate() {
        let seen = nodes[..i].iter().any(|q| (*q - p).length() < 0.5);
        if !seen {
            out.push(p);
            continue;
        }
        let din = (p - nodes[i - 1]).normalized();
        match nodes.get(i + 1) {
            Some(&next) => {
                let dout = (next - p).normalized();
                // into the turn; when going straight: step aside
                let mut side = dout - din;
                if side.length() < 0.2 {
                    side = Vec2::new(-din.y, din.x);
                }
                let side = side.normalized() * off * 0.6;
                out.push(p - din * off + side);
                out.push(p + dout * off + side);
            }
            None => out.push(p - din * off),
        }
    }
    out
}

/// Gentle shimmer: every edge sways with two slow sine waves across the line.
/// The offset is zero at both ends of each edge, so the nodes stay fixed.
fn zap(pts: &[Pos2], seed: u32, time: f64, amp: f32) -> Vec<Pos2> {
    use std::f32::consts::PI;
    let t = time as f32;
    let mut out = vec![pts[0]];
    for (si, w) in pts.windows(2).enumerate() {
        let (a, c) = (w[0], w[1]);
        let d = c - a;
        let len = d.length();
        if len < 1.0 {
            out.push(c);
            continue;
        }
        let n = Vec2::new(-d.y, d.x) / len;
        let sub = ((len / 5.0).ceil() as usize).clamp(2, 32);
        let k = seed.wrapping_mul(0x9e37_79b9).wrapping_add(si as u32 * 0x85eb_ca6b);
        let (p1, p2) = (hash(k) * PI, hash(k ^ 0x5bd1_e995) * PI);
        let (s1, s2) = (2.0 + hash(k.wrapping_add(7)) * 0.7, 3.0 + hash(k.wrapping_add(13)) * 0.8);
        for j in 1..sub {
            let u = j as f32 / sub as f32;
            let env = (PI * u).sin();
            let o = 0.65 * (PI * u * 2.0 + p1 + t * s1).sin() + 0.35 * (PI * u * 3.0 + p2 - t * s2).sin();
            out.push(a + d * u + n * o * amp * env);
        }
        out.push(c);
    }
    out
}

/// Draws a pattern as a glowing line: glow, gradient, bright core, node dots.
pub fn zappy(painter: &Painter, nodes: &[Pos2], seed: u32, time: f64, scale: f32, width: f32, grad: Grad, dim: f32) {
    if nodes.len() < 2 {
        return;
    }
    let z = zap(&route(nodes, scale), seed, time, scale * 0.045);
    let mid = lerp(grad.0, grad.1, 0.5);
    painter.add(Shape::line(z.clone(), Stroke::new(width * 4.2, mid.gamma_multiply(0.06 * dim))));
    painter.add(Shape::line(z.clone(), Stroke::new(width * 2.2, mid.gamma_multiply(0.16 * dim))));
    let n = z.len() - 1;
    for i in 0..n {
        let col = lerp(grad.0, grad.1, i as f32 / n as f32).gamma_multiply(dim);
        painter.line_segment([z[i], z[i + 1]], Stroke::new(width, col));
        painter.circle_filled(z[i + 1], width * 0.5, col);
    }
    painter.add(Shape::line(z, Stroke::new((width * 0.3).max(1.0), Color32::from_white_alpha((150.0 * dim) as u8))));
    let m = nodes.len() - 1;
    for (i, p) in nodes.iter().enumerate() {
        if nodes[..i].iter().any(|q| (*q - *p).length() < 0.5) {
            continue;
        }
        let col = lighten(lerp(grad.0, grad.1, i as f32 / m.max(1) as f32), 0.35).gamma_multiply(dim);
        painter.circle_filled(*p, width * 1.2, col.gamma_multiply(0.3));
        painter.circle_filled(*p, width * 0.7, col);
    }
}

/// Shimmering beam from the last node to the mouse pointer (while drawing).
pub fn beam(painter: &Painter, from: Pos2, to: Pos2, time: f64, scale: f32, width: f32) {
    if (to - from).length() < 2.0 {
        return;
    }
    let z = zap(&[from, to], 7, time * 1.5, scale * 0.04);
    painter.add(Shape::line(z.clone(), Stroke::new(width * 2.0, ACCENT.gamma_multiply(0.15))));
    painter.add(Shape::line(z, Stroke::new(width * 0.6, lighten(ACCENT, 0.4).gamma_multiply(0.8))));
}
/// Dark purple theme for the whole app.
pub fn apply_theme(ctx: &egui::Context) {
    let mut v = egui::Visuals::dark();
    v.panel_fill = PANEL;
    v.window_fill = PANEL;
    v.extreme_bg_color = BG;
    v.faint_bg_color = Color32::from_rgb(30, 26, 44);
    v.selection.bg_fill = Color32::from_rgb(92, 66, 150);
    v.selection.stroke = Stroke::new(1.0_f32, lighten(ACCENT, 0.3));
    v.hyperlink_color = ACCENT;
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, Color32::from_rgb(48, 42, 68));
    v.widgets.inactive.weak_bg_fill = Color32::from_rgb(38, 33, 56);
    v.widgets.inactive.bg_fill = Color32::from_rgb(38, 33, 56);
    v.widgets.hovered.weak_bg_fill = Color32::from_rgb(58, 48, 88);
    v.widgets.hovered.bg_fill = Color32::from_rgb(58, 48, 88);
    v.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, ACCENT);
    v.widgets.active.weak_bg_fill = Color32::from_rgb(92, 66, 150);
    v.widgets.active.bg_fill = Color32::from_rgb(92, 66, 150);
    v.slider_trailing_fill = true;
    for w in [&mut v.widgets.inactive, &mut v.widgets.hovered, &mut v.widgets.active, &mut v.widgets.open] {
        w.rounding = egui::Rounding::same(6.0);
    }
    v.window_rounding = egui::Rounding::same(10.0);
    ctx.options_mut(|o| o.theme_preference = egui::ThemePreference::Dark);
    ctx.set_visuals_of(egui::Theme::Dark, v);

    let mut style = egui::Style { visuals: ctx.style_of(egui::Theme::Dark).visuals.clone(), ..(*ctx.style_of(egui::Theme::Dark)).clone() };
    style.spacing.item_spacing = Vec2::new(8.0, 6.0);
    style.spacing.button_padding = Vec2::new(10.0, 5.0);
    for (ts, size) in [
        (egui::TextStyle::Heading, 22.0),
        (egui::TextStyle::Body, 14.5),
        (egui::TextStyle::Button, 14.5),
        (egui::TextStyle::Monospace, 13.0),
        (egui::TextStyle::Small, 11.5),
    ] {
        if let Some(f) = style.text_styles.get_mut(&ts) {
            f.size = size;
        }
    }
    ctx.set_style_of(egui::Theme::Dark, style);
}
