//! Hex grid in axial coordinates (q, r).
//! Screen: x = 2q + r (half column widths), y = r (rows).

use std::collections::HashSet;

pub type Pt = (i32, i32);

/// Directions clockwise, starting with east.
pub const DIRS: [Pt; 6] = [(1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)];
pub const DIR_NAMES: [&str; 6] = ["east", "south-east", "south-west", "west", "north-west", "north-east"];

/// w = straight, e = right, d = sharp right, s = back, a = sharp left, q = left
pub fn turn_of(c: char) -> Option<usize> {
    "wedsaq".find(c)
}

pub fn turn_char(t: usize) -> char {
    b"wedsaq"[t % 6] as char
}

/// Traces a pattern from (0,0) with start direction `dir`.
/// Fails if an edge is used twice: the game does not accept that.
pub fn trace(sig: &str, dir: usize) -> Result<Vec<Pt>, String> {
    let mut d = dir % 6;
    let mut p: Pt = (0, 0);
    let mut pts = vec![p];
    let mut edges: HashSet<(Pt, Pt)> = HashSet::new();
    let mut step = |d: usize, p: &mut Pt, pts: &mut Vec<Pt>| -> bool {
        let n = (p.0 + DIRS[d].0, p.1 + DIRS[d].1);
        let e = if *p < n { (*p, n) } else { (n, *p) };
        *p = n;
        pts.push(n);
        edges.insert(e)
    };
    step(d, &mut p, &mut pts);
    for (i, c) in sig.chars().enumerate() {
        let t = turn_of(c).ok_or_else(|| format!("unknown character '{c}' in the signature"))?;
        d = (d + t) % 6;
        if !step(d, &mut p, &mut pts) {
            return Err(format!("stroke {} ('{c}') reuses an edge that was already drawn", i + 2));
        }
    }
    Ok(pts)
}

pub fn screen_x(p: Pt) -> i32 {
    2 * p.0 + p.1
}
