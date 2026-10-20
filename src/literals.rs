//! Patterns whose shape depends on a value: Numerical Reflection and Bookkeeper's Gambit.

use crate::geom;

/// Shortest valid signature for Numerical Reflection with value `n`.
/// After the prefix (`aqaa` positive, `dedd` negative): w +1, q +5, e +10, a ×2, d ÷2.
pub fn number_sig(n: f64) -> Option<String> {
    let (prefix, target) = if n < 0.0 { ("dedd", -n) } else { ("aqaa", n) };
    let int = target.fract() == 0.0;
    let mut budget: u64 = 3_000_000;
    let mut ops = String::new();
    for depth in 0..=30 {
        if dfs(prefix, &mut ops, 0.0, target, depth, int, &mut budget) {
            return Some(format!("{prefix}{ops}"));
        }
        if budget == 0 {
            break;
        }
    }
    None
}

fn dfs(prefix: &str, ops: &mut String, v: f64, target: f64, left: usize, int: bool, budget: &mut u64) -> bool {
    if (v - target).abs() < 1e-9 {
        return true;
    }
    if left == 0 || *budget == 0 {
        return false;
    }
    *budget -= 1;
    for c in ['e', 'q', 'w', 'a', 'd'] {
        let nv = match c {
            'w' => v + 1.0,
            'q' => v + 5.0,
            'e' => v + 10.0,
            'a' => v * 2.0,
            _ => v / 2.0,
        };
        if (c == 'a' || c == 'd') && v == 0.0 {
            continue;
        }
        if int && (c == 'd' || nv > target) {
            continue;
        }
        if nv > target * 2.0 + 10.0 {
            continue;
        }
        // upper bound: unreachable even with the biggest steps
        let mut best = nv;
        for _ in 1..left {
            best = (best + 10.0).max(best * 2.0);
        }
        if best < target {
            continue;
        }
        ops.push(c);
        if geom::trace(&format!("{prefix}{ops}"), 0).is_ok() && dfs(prefix, ops, nv, target, left - 1, int, budget) {
            return true;
        }
        ops.pop();
    }
    false
}

/// Value of a drawn Numerical Reflection, otherwise `None`.
pub fn decode_number(sig: &str) -> Option<f64> {
    let (neg, rest) = if let Some(r) = sig.strip_prefix("aqaa") {
        (false, r)
    } else {
        (true, sig.strip_prefix("dedd")?)
    };
    let mut v = 0.0;
    for c in rest.chars() {
        v = match c {
            'w' => v + 1.0,
            'q' => v + 5.0,
            'e' => v + 10.0,
            'a' => v * 2.0,
            'd' => v / 2.0,
            _ => return None,
        };
    }
    Some(if neg { -v } else { v })
}

/// Mask of a drawn Bookkeeper's Gambit, otherwise `None`.
pub fn decode_mask(sig: &str) -> Option<String> {
    let mut dirs = vec![0usize];
    for c in sig.chars() {
        let t = geom::turn_of(c)?;
        dirs.push((dirs.last().unwrap() + t) % 6);
    }
    // if it starts with a dip, the base direction is one step left of the first stroke
    let flat = if sig.starts_with('a') { 5 } else { 0 };
    let rel: Vec<usize> = dirs.iter().map(|d| (d + 6 - flat) % 6).collect();
    let mut mask = String::new();
    let mut i = 0;
    while i < rel.len() {
        match (rel[i], rel.get(i + 1)) {
            (0, _) => {
                mask.push('-');
                i += 1;
            }
            (1, Some(5)) => {
                mask.push('v');
                i += 2;
            }
            _ => return None,
        }
    }
    Some(mask)
}

/// Bookkeeper's Gambit from a mask like `v-v` (`-` keep, `v` remove).
/// As in game: keep = stroke in the base direction, remove = stroke up-right + down-left.
pub fn mask_sig(mask: &str) -> Result<String, String> {
    let mut dirs: Vec<usize> = Vec::new();
    for c in mask.chars() {
        match c {
            '-' => dirs.push(0),
            'v' | 'V' => {
                dirs.push(1);
                dirs.push(5);
            }
            _ => return Err(format!("a mask may only contain '-' and 'v', not '{c}'")),
        }
    }
    if dirs.is_empty() {
        return Err("empty mask".into());
    }
    Ok(dirs.windows(2).map(|w| geom::turn_char((w[1] + 6 - w[0]) % 6)).collect())
}
