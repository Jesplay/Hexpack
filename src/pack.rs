//! Packs patterns without shared points into the smallest rectangle with aspect ratio W:H.
//! Every pattern may be rotated in all 6 directions (only the sequence of angles matters).
//! Method: greedy placement with restarts, then ruin & recreate until the time is up.

use std::time::{Duration, Instant};

use crate::geom::{self, Pt, DIRS};

pub struct Shape {
    /// point sequence per rotation (start direction 0..6)
    pub path: Vec<Vec<Pt>>,
    /// occupied points per rotation, without duplicates
    pub uniq: Vec<Vec<Pt>>,
    /// (xmin, xmax, rmin, rmax) per rotation, x = 2q + r
    bounds: Vec<(i32, i32, i32, i32)>,
}

impl Shape {
    pub fn new(sig: &str) -> Shape {
        let mut path = Vec::new();
        let mut uniq = Vec::new();
        let mut bounds = Vec::new();
        for rot in 0..6 {
            // place undrawable signatures anyway, the checker reports them
            let p = geom::trace(sig, rot).unwrap_or_else(|_| trace_loose(sig, rot));
            let mut u = p.clone();
            u.sort();
            u.dedup();
            bounds.push(bbox(&u));
            path.push(p);
            uniq.push(u);
        }
        Shape { path, uniq, bounds }
    }
}

fn trace_loose(sig: &str, dir: usize) -> Vec<Pt> {
    let mut d = dir;
    let mut p = (0, 0);
    let mut out = vec![p];
    p = (p.0 + DIRS[d].0, p.1 + DIRS[d].1);
    out.push(p);
    for c in sig.chars() {
        d = (d + geom::turn_of(c).unwrap_or(0)) % 6;
        p = (p.0 + DIRS[d].0, p.1 + DIRS[d].1);
        out.push(p);
    }
    out
}

fn bbox(pts: &[Pt]) -> (i32, i32, i32, i32) {
    let mut b = (i32::MAX, i32::MIN, i32::MAX, i32::MIN);
    for &p in pts {
        let x = geom::screen_x(p);
        b.0 = b.0.min(x);
        b.1 = b.1.max(x);
        b.2 = b.2.min(p.1);
        b.3 = b.3.max(p.1);
    }
    b
}

#[derive(Clone)]
pub struct Layout {
    pub rot: Vec<usize>,
    pub off: Vec<Pt>,
    pub key: Key,
}

/// Score, smaller is better.
#[derive(Clone, Copy, Debug)]
pub struct Key {
    /// size of the smallest W:H rectangle around everything
    pub fit: f64,
    /// area of the bounding box
    pub area: f64,
}

impl Key {
    fn better(&self, o: &Key) -> bool {
        if (self.fit - o.fit).abs() > 1e-9 {
            return self.fit < o.fit;
        }
        self.area < o.area - 1e-9
    }
    fn not_worse(&self, o: &Key) -> bool {
        !o.better(self)
    }
}

const SQ3_2: f64 = 0.866_025_403_784_438_6;
const G: i32 = 384;
const GO: i32 = G / 2;

struct State<'a> {
    shapes: &'a [Shape],
    ratio: (f64, f64),
    grid: Vec<u16>,
    rot: Vec<usize>,
    off: Vec<Pt>,
    placed: Vec<bool>,
}

fn cell(p: Pt) -> Option<usize> {
    let (q, r) = (p.0 + GO, p.1 + GO);
    (q >= 0 && r >= 0 && q < G && r < G).then(|| (r * G + q) as usize)
}

impl<'a> State<'a> {
    fn new(shapes: &'a [Shape], ratio: (f64, f64)) -> Self {
        let n = shapes.len();
        State { shapes, ratio, grid: vec![0; (G * G) as usize], rot: vec![0; n], off: vec![(0, 0); n], placed: vec![false; n] }
    }

    fn key_of(&self, b: (i32, i32, i32, i32)) -> Key {
        let w = (b.1 - b.0) as f64 * SQ3_2;
        let h = (b.3 - b.2) as f64 * 1.5;
        Key { fit: (w / self.ratio.0).max(h / self.ratio.1), area: (w + 1.0) * (h + 1.0) }
    }

    fn bounds(&self) -> Option<(i32, i32, i32, i32)> {
        let mut b: Option<(i32, i32, i32, i32)> = None;
        for i in 0..self.shapes.len() {
            if !self.placed[i] {
                continue;
            }
            let sb = self.shapes[i].bounds[self.rot[i]];
            let (oq, or) = self.off[i];
            let ox = 2 * oq + or;
            let s = (sb.0 + ox, sb.1 + ox, sb.2 + or, sb.3 + or);
            b = Some(match b {
                None => s,
                Some(b) => (b.0.min(s.0), b.1.max(s.1), b.2.min(s.2), b.3.max(s.3)),
            });
        }
        b
    }

    fn free(&self, i: usize, rot: usize, off: Pt) -> bool {
        self.shapes[i].uniq[rot].iter().all(|p| cell((p.0 + off.0, p.1 + off.1)).is_some_and(|c| self.grid[c] == 0))
    }

    fn contacts(&self, i: usize, rot: usize, off: Pt) -> i32 {
        let mut n = 0;
        for p in &self.shapes[i].uniq[rot] {
            for d in DIRS {
                if let Some(c) = cell((p.0 + off.0 + d.0, p.1 + off.1 + d.1)) {
                    if self.grid[c] != 0 && self.grid[c] as usize != i + 1 {
                        n += 1;
                    }
                }
            }
        }
        n
    }

    fn put(&mut self, i: usize, rot: usize, off: Pt) {
        for p in &self.shapes[i].uniq[rot] {
            let c = cell((p.0 + off.0, p.1 + off.1)).unwrap();
            self.grid[c] = (i + 1) as u16;
        }
        self.rot[i] = rot;
        self.off[i] = off;
        self.placed[i] = true;
    }

    fn take(&mut self, i: usize) {
        for p in &self.shapes[i].uniq[self.rot[i]] {
            if let Some(c) = cell((p.0 + self.off[i].0, p.1 + self.off[i].1)) {
                self.grid[c] = 0;
            }
        }
        self.placed[i] = false;
    }

    /// Puts pattern `i` at the best free spot.
    fn place_best(&mut self, i: usize, rng: &mut Rng) {
        let cur = self.bounds();
        let mut best: Option<(Key, i32, u64, usize, Pt)> = None;
        for rot in 0..6 {
            let sb = self.shapes[i].bounds[rot];
            let (lo_x, hi_x, lo_r, hi_r) = match cur {
                // start point (0,0) relative to the shape: the shape lies around the origin
                None => (0, 0, 0, 0),
                Some(b) => (b.0 - sb.1 - 2, b.1 - sb.0 + 2, b.2 - sb.3 - 1, b.3 - sb.2 + 1),
            };
            for or in lo_r..=hi_r {
                let q_lo = (lo_x - or).div_euclid(2);
                let q_hi = (hi_x - or).div_euclid(2) + 1;
                for oq in q_lo..=q_hi {
                    let off = (oq, or);
                    if cur.is_some() && !self.free(i, rot, off) {
                        continue;
                    }
                    let ox = 2 * oq + or;
                    let s = (sb.0 + ox, sb.1 + ox, sb.2 + or, sb.3 + or);
                    let nb = match cur {
                        None => s,
                        Some(b) => (b.0.min(s.0), b.1.max(s.1), b.2.min(s.2), b.3.max(s.3)),
                    };
                    let key = self.key_of(nb);
                    if best.as_ref().is_some_and(|b| b.0.better(&key)) {
                        continue;
                    }
                    // same size: most neighbours wins, otherwise random
                    let cont = if cur.is_some() { self.contacts(i, rot, off) } else { 0 };
                    let tie = rng.next();
                    let take = match &best {
                        None => true,
                        Some((bk, bc, bt, _, _)) => key.better(bk) || cont > *bc || (cont == *bc && tie < *bt),
                    };
                    if take {
                        best = Some((key, cont, tie, rot, off));
                    }
                }
            }
        }
        let (_, _, _, rot, off) = best.expect("kein Platz im Raster gefunden");
        self.put(i, rot, off);
    }

    fn key(&self) -> Key {
        self.key_of(self.bounds().unwrap_or((0, 0, 0, 0)))
    }

    fn snapshot(&self) -> Layout {
        Layout { rot: self.rot.clone(), off: self.off.clone(), key: self.key() }
    }
}

pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed.max(1))
    }
    pub fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = self.below(i + 1);
            v.swap(i, j);
        }
    }
}

pub struct Stats {
    pub greedy_runs: u64,
    pub improve_steps: u64,
}

pub fn pack(shapes: &[Shape], ratio: (f64, f64), budget: Duration, seed: u64) -> (Layout, Stats) {
    let n = shapes.len();
    let start = Instant::now();
    let mut rng = Rng::new(seed);
    let mut stats = Stats { greedy_runs: 0, improve_steps: 0 };
    let size = |i: usize| shapes[i].uniq[0].len();

    // phase 1: greedy, with different orders
    let mut best: Option<Layout> = None;
    let phase1 = budget.mul_f64(0.3);
    loop {
        let mut order: Vec<usize> = (0..n).collect();
        if stats.greedy_runs == 0 {
            order.sort_by_key(|&i| std::cmp::Reverse(size(i)));
        } else {
            let noise: Vec<f64> = (0..n).map(|i| size(i) as f64 * (0.5 + rng.unit())).collect();
            order.sort_by(|&a, &b| noise[b].partial_cmp(&noise[a]).unwrap());
        }
        let mut st = State::new(shapes, ratio);
        for &i in &order {
            st.place_best(i, &mut rng);
        }
        stats.greedy_runs += 1;
        let lay = st.snapshot();
        if best.as_ref().is_none_or(|b| lay.key.better(&b.key)) {
            best = Some(lay);
        }
        if start.elapsed() >= phase1 || n == 0 {
            break;
        }
    }
    let mut best = best.unwrap();
    if n < 2 {
        return (best, stats);
    }

    // phase 2: ruin & recreate from the best solution
    let mut st = State::new(shapes, ratio);
    for i in 0..n {
        st.put(i, best.rot[i], best.off[i]);
    }
    let mut cur_key = st.key();
    while start.elapsed() < budget {
        stats.improve_steps += 1;
        let k = 1 + rng.below(n.min(6));
        let mut pick: Vec<usize> = Vec::new();
        // prefer something on the border, otherwise the box never shrinks
        if let Some(b) = st.bounds() {
            let edge: Vec<usize> = (0..n)
                .filter(|&i| {
                    let sb = shapes[i].bounds[st.rot[i]];
                    let (oq, or) = st.off[i];
                    let ox = 2 * oq + or;
                    sb.0 + ox == b.0 || sb.1 + ox == b.1 || sb.2 + or == b.2 || sb.3 + or == b.3
                })
                .collect();
            if !edge.is_empty() {
                pick.push(edge[rng.below(edge.len())]);
            }
        }
        while pick.len() < k {
            let i = rng.below(n);
            if !pick.contains(&i) {
                pick.push(i);
            }
        }
        let old: Vec<(usize, usize, Pt)> = pick.iter().map(|&i| (i, st.rot[i], st.off[i])).collect();
        for &i in &pick {
            st.take(i);
        }
        if rng.below(2) == 0 {
            rng.shuffle(&mut pick);
        } else {
            pick.sort_by_key(|&i| std::cmp::Reverse(size(i)));
        }
        for &i in &pick {
            st.place_best(i, &mut rng);
        }
        let key = st.key();
        let accept = key.not_worse(&cur_key) || rng.unit() < 0.01;
        if accept {
            cur_key = key;
            if key.better(&best.key) {
                best = st.snapshot();
            }
        } else {
            for &i in &pick {
                st.take(i);
            }
            for &(i, rot, off) in &old {
                st.put(i, rot, off);
            }
        }
    }
    (best, stats)
}
