//! Output of a packed layout: ASCII for the terminal and SVG for the browser.

use std::fmt::Write as _;

use crate::geom::{self, Pt, DIR_NAMES};
use crate::pack::{Layout, Shape};
use crate::program::Drawn;

pub struct Placed {
    pub path: Vec<Pt>,
    pub rot: usize,
}

pub fn placed(shapes: &[Shape], lay: &Layout) -> Vec<Placed> {
    shapes
        .iter()
        .enumerate()
        .map(|(i, s)| Placed {
            path: s.path[lay.rot[i]].iter().map(|p| (p.0 + lay.off[i].0, p.1 + lay.off[i].1)).collect(),
            rot: lay.rot[i],
        })
        .collect()
}

pub fn bounds(ps: &[Placed]) -> (i32, i32, i32, i32) {
    let mut b = (i32::MAX, i32::MIN, i32::MAX, i32::MIN);
    for p in ps.iter().flat_map(|p| p.path.iter()) {
        let x = geom::screen_x(*p);
        b = (b.0.min(x), b.1.max(x), b.2.min(p.1), b.3.max(p.1));
    }
    b
}

/// Marker at the start point in the ASCII picture.
pub fn tag(i: usize) -> char {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    T.get(i).map(|&c| c as char).unwrap_or('*')
}

/// Row and point number within the row (both from 1, top left).
pub fn row_col(p: Pt, b: (i32, i32, i32, i32)) -> (i32, i32) {
    let x = geom::screen_x(p);
    let r = p.1;
    let first = if (b.0 - r).rem_euclid(2) == 0 { b.0 } else { b.0 + 1 };
    (r - b.2 + 1, (x - first) / 2 + 1)
}

pub fn ascii(ps: &[Placed]) -> String {
    let b = bounds(ps);
    let w = ((b.1 - b.0) * 2 + 1) as usize;
    let h = ((b.3 - b.2) * 2 + 1) as usize;
    let mut cv = vec![vec![' '; w]; h];
    let pos = |p: Pt| (((geom::screen_x(p) - b.0) * 2) as usize, ((p.1 - b.2) * 2) as usize);
    for r in b.2..=b.3 {
        for x in b.0..=b.1 {
            if (x - r).rem_euclid(2) == 0 {
                let (c, rr) = (((x - b.0) * 2) as usize, ((r - b.2) * 2) as usize);
                cv[rr][c] = '·';
            }
        }
    }
    for pl in ps {
        for win in pl.path.windows(2) {
            let (a, z) = (pos(win[0]), pos(win[1]));
            let (c0, r0) = (a.0.min(z.0), a.1.min(z.1));
            if a.1 == z.1 {
                for c in c0 + 1..c0 + 4 {
                    cv[r0][c] = '-';
                }
            } else {
                let top = if a.1 < z.1 { a } else { z };
                let bot = if a.1 < z.1 { z } else { a };
                let ch = if bot.0 > top.0 { '\\' } else { '/' };
                cv[top.1 + 1][(top.0 + bot.0) / 2] = ch;
            }
        }
        for p in &pl.path {
            let (c, r) = pos(*p);
            cv[r][c] = 'o';
        }
    }
    for (i, pl) in ps.iter().enumerate() {
        let (c, r) = pos(pl.path[0]);
        cv[r][c] = tag(i);
    }
    cv.into_iter().map(|row| row.into_iter().collect::<String>().trim_end().to_string()).collect::<Vec<_>>().join("\n")
}

const COLORS: [&str; 8] = ["#7F77DD", "#1D9E75", "#D85A30", "#378ADD", "#BA7517", "#D4537E", "#639922", "#5F5E5A"];

pub fn svg(ps: &[Placed], drawn: &[Drawn], title: &str) -> String {
    let b = bounds(ps);
    let s = 34.0;
    let pad = 40.0;
    let sq = 0.866_025_403_784_438_6;
    let px = |p: Pt| ((geom::screen_x(p) - b.0) as f64 * sq * s + pad, (p.1 - b.2) as f64 * 1.5 * s + pad);
    let gw = (b.1 - b.0) as f64 * sq * s + 2.0 * pad;
    let gh = (b.3 - b.2) as f64 * 1.5 * s + 2.0 * pad;
    let legend_h = 22.0 * ps.len() as f64 + 40.0;
    let w = gw.max(640.0);
    let h = gh + legend_h;
    let mut o = String::new();
    let _ = writeln!(o, r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w:.0}" height="{h:.0}" viewBox="0 0 {w:.0} {h:.0}" font-family="Segoe UI, sans-serif">"##);
    let _ = writeln!(o, r##"<title>{}</title><rect width="100%" height="100%" fill="#fbfaf7"/>"##, esc(title));
    for r in b.2..=b.3 {
        for x in b.0..=b.1 {
            if (x - r).rem_euclid(2) == 0 {
                let (cx, cy) = (((x - b.0) as f64) * sq * s + pad, (r - b.2) as f64 * 1.5 * s + pad);
                let _ = writeln!(o, r##"<circle cx="{cx:.1}" cy="{cy:.1}" r="2" fill="#b4b2a9"/>"##);
            }
        }
    }
    for (i, pl) in ps.iter().enumerate() {
        let col = COLORS[i % COLORS.len()];
        let pts: Vec<String> = pl.path.iter().map(|p| {
            let (x, y) = px(*p);
            format!("{x:.1},{y:.1}")
        }).collect();
        let dash = if drawn[i].quoted { r##" stroke-dasharray="7 5""## } else { "" };
        let _ = writeln!(o, r##"<polyline points="{}" fill="none" stroke="{col}" stroke-width="5" stroke-linejoin="round" stroke-linecap="round"{dash}/>"##, pts.join(" "));
    }
    for (i, pl) in ps.iter().enumerate() {
        let (x0, y0) = px(pl.path[0]);
        let (x1, y1) = px(pl.path[1]);
        let (dx, dy) = (x1 - x0, y1 - y0);
        let l = (dx * dx + dy * dy).sqrt();
        let (ux, uy) = (dx / l, dy / l);
        let (tx, ty) = (x0 + ux * l * 0.62, y0 + uy * l * 0.62);
        let a = (tx - ux * 9.0 - uy * 6.0, ty - uy * 9.0 + ux * 6.0);
        let c = (tx - ux * 9.0 + uy * 6.0, ty - uy * 9.0 - ux * 6.0);
        let _ = writeln!(o, r##"<polygon points="{tx:.1},{ty:.1} {:.1},{:.1} {:.1},{:.1}" fill="#c0306a"/>"##, a.0, a.1, c.0, c.1);
        let _ = writeln!(o, r##"<circle cx="{x0:.1}" cy="{y0:.1}" r="10" fill="#c0306a"/><text x="{x0:.1}" y="{:.1}" text-anchor="middle" font-size="11" font-weight="600" fill="#fff">{}</text>"##, y0 + 4.0, i + 1);
    }
    let mut y = gh + 20.0;
    for (i, (pl, d)) in ps.iter().zip(drawn).enumerate() {
        let (row, col) = row_col(pl.path[0], b);
        let col_c = COLORS[i % COLORS.len()];
        let _ = writeln!(
            o,
            r##"<rect x="{pad}" y="{:.1}" width="12" height="12" rx="3" fill="{col_c}"/><text x="{:.1}" y="{:.1}" font-size="13" fill="#2c2c2a">{}. {}  ·  {}  ·  start row {row}, point {col}, first stroke {}</text>"##,
            y - 10.0,
            pad + 20.0,
            y,
            i + 1,
            esc(&d.label),
            d.sig,
            DIR_NAMES[pl.rot]
        );
        y += 22.0;
    }
    o.push_str("</svg>\n");
    o
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}
