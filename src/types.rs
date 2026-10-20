//! Rough iota types, parsed from the free-text `input`/`output` fields in the vault.

use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Ty {
    Any,
    /// placeholder like `a` in `a, b -> a`
    Var(char),
    Num(Option<f64>),
    Vec,
    Entity,
    Bool,
    Null,
    Pattern,
    List(Box<Ty>),
    /// list from Introspection…Retrospection (index into `Program::blocks`)
    Code(usize),
    Named(String),
    Union(Vec<Ty>),
}

pub enum Slot {
    One(Ty),
    /// any number of iotas (`many`)
    Many,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Fit {
    Yes,
    Maybe,
    No,
}

fn split_top(s: &str, sep: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    let mut rest = s;
    while let Some(c) = rest.chars().next() {
        if depth == 0 && rest.starts_with(sep) {
            out.push(std::mem::take(&mut cur));
            rest = &rest[sep.len()..];
            continue;
        }
        match c {
            '[' | '(' | '{' | '<' => depth += 1,
            ']' | ')' | '}' | '>' => depth = (depth - 1).max(0),
            _ => {}
        }
        cur.push(c);
        rest = &rest[c.len_utf8()..];
    }
    out.push(cur);
    out
}

pub fn parse_slots(s: &str) -> Vec<Slot> {
    let s = s.trim();
    if s.is_empty() || s.eq_ignore_ascii_case("none") || s.eq_ignore_ascii_case("nothing") {
        return Vec::new();
    }
    split_top(s, ",")
        .into_iter()
        .map(|p| {
            let p = p.trim().to_lowercase();
            if p == "many" || p == "..." || p == "stack" {
                Slot::Many
            } else {
                Slot::One(parse_ty(&p))
            }
        })
        .collect()
}

fn strip_wrap(s: &str, open: char, close: char) -> Option<&str> {
    let inner = s.strip_prefix(open)?.strip_suffix(close)?;
    // only if the brackets really belong together
    let mut d = 0;
    for c in inner.chars() {
        if c == open {
            d += 1
        } else if c == close {
            d -= 1;
            if d < 0 {
                return None;
            }
        }
    }
    (d == 0).then_some(inner)
}

pub fn parse_ty(s: &str) -> Ty {
    let s = s.trim();
    if let Some(inner) = strip_wrap(s, '(', ')') {
        return parse_ty(inner);
    }
    if let Some(base) = s.strip_suffix('?') {
        return union(vec![parse_ty(base), Ty::Null]);
    }
    for sep in ["|", " or ", "/"] {
        let parts = split_top(s, sep);
        if parts.len() > 1 {
            return union(parts.iter().map(|p| parse_ty(p)).collect());
        }
    }
    if let Some(inner) = strip_wrap(s, '[', ']') {
        return Ty::List(Box::new(if inner.trim().is_empty() { Ty::Any } else { parse_ty(inner) }));
    }
    if let Some(rest) = s.strip_prefix("list of ") {
        return Ty::List(Box::new(parse_ty(rest)));
    }
    word(s)
}

fn word(w: &str) -> Ty {
    let w = w.trim();
    if w.len() == 1 && w.chars().all(|c| c.is_ascii_alphabetic()) {
        return Ty::Var(w.chars().next().unwrap());
    }
    match w {
        "any" | "iota" | "any iota" | "anything" | "varies" | "" => Ty::Any,
        "num" | "number" | "numbers" | "int" | "long" | "index" | "double" | "float" | "positive num" | "int ≥ 0"
        | "slot num" | "indices" => Ty::Num(None),
        "vec" | "vector" | "vectors" | "vecs" | "pos" | "position" => Ty::Vec,
        "bool" | "boolean" => Ty::Bool,
        "null" | "none" | "nothing" => Ty::Null,
        "pattern" | "patterns" => Ty::Pattern,
        "evaluatable" | "executable" => Ty::Union(vec![Ty::Pattern, Ty::List(Box::new(Ty::Any))]),
        "list" | "lists" => Ty::List(Box::new(Ty::Any)),
        w if w.contains("entit") || matches!(w, "player" | "villager" | "wisp" | "item frame") => Ty::Entity,
        w => Ty::Named(w.to_string()),
    }
}

fn union(mut v: Vec<Ty>) -> Ty {
    let mut flat = Vec::new();
    for t in v.drain(..) {
        match t {
            Ty::Union(inner) => flat.extend(inner),
            t => flat.push(t),
        }
    }
    let mut out: Vec<Ty> = Vec::new();
    for t in flat {
        if !out.contains(&t) {
            out.push(t);
        }
    }
    if out.contains(&Ty::Any) {
        return Ty::Any;
    }
    if out.len() == 1 {
        out.pop().unwrap()
    } else {
        Ty::Union(out)
    }
}

/// Common type of several values (for list elements).
pub fn join(ts: &[Ty]) -> Ty {
    if ts.is_empty() {
        return Ty::Any;
    }
    let ts: Vec<Ty> = ts.iter().map(|t| if let Ty::Num(_) = t { Ty::Num(None) } else { t.clone() }).collect();
    union(ts)
}

pub fn fit(have: &Ty, want: &Ty, vars: &mut HashMap<char, Ty>) -> Fit {
    use Ty::*;
    match (have, want) {
        (_, Any) => Fit::Yes,
        (_, Var(c)) => match vars.get(c).cloned() {
            Some(b) => fit(have, &b, vars),
            None => {
                vars.insert(*c, have.clone());
                Fit::Yes
            }
        },
        (Any, _) | (Var(_), _) => Fit::Yes,
        (_, Union(ws)) => {
            let fits: std::vec::Vec<Fit> = ws.iter().map(|w| fit(have, w, vars)).collect();
            if fits.contains(&Fit::Yes) {
                Fit::Yes
            } else if fits.contains(&Fit::Maybe) {
                Fit::Maybe
            } else {
                Fit::No
            }
        }
        (Union(hs), _) => {
            // null cases are bad luck at runtime, not wrong wiring
            let hs: std::vec::Vec<&Ty> = hs.iter().filter(|h| **h != Null).collect();
            if hs.is_empty() {
                return fit(&Null, want, vars);
            }
            let fits: std::vec::Vec<Fit> = hs.iter().map(|h| fit(h, want, vars)).collect();
            if fits.iter().all(|f| *f == Fit::Yes) {
                Fit::Yes
            } else if fits.iter().all(|f| *f == Fit::No) {
                Fit::No
            } else {
                Fit::Maybe
            }
        }
        (Num(_), Num(_)) => Fit::Yes,
        (Code(_), List(e)) => fit(&Pattern, e, vars),
        (List(a), List(b)) => match fit(a, b, vars) {
            Fit::No => Fit::Maybe,
            f => f,
        },
        (Named(a), Named(b)) => {
            if a == b {
                Fit::Yes
            } else {
                Fit::Maybe
            }
        }
        (Named(_), _) | (_, Named(_)) => Fit::Maybe,
        (a, b) if std::mem::discriminant(a) == std::mem::discriminant(b) => Fit::Yes,
        _ => Fit::No,
    }
}

pub fn subst(t: &Ty, vars: &HashMap<char, Ty>) -> Ty {
    match t {
        Ty::Var(c) => vars.get(c).cloned().unwrap_or(Ty::Any),
        Ty::List(e) => Ty::List(Box::new(subst(e, vars))),
        Ty::Union(v) => union(v.iter().map(|x| subst(x, vars)).collect()),
        t => t.clone(),
    }
}

fn fmt_num(v: f64) -> String {
    if v.fract() == 0.0 && v.abs() < 1e15 {
        format!("{}", v as i64)
    } else {
        format!("{v:.3}")
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Any => write!(f, "?"),
            Ty::Var(c) => write!(f, "{c}"),
            Ty::Num(Some(v)) => write!(f, "{}", fmt_num(*v)),
            Ty::Num(None) => write!(f, "num"),
            Ty::Vec => write!(f, "vec"),
            Ty::Entity => write!(f, "entity"),
            Ty::Bool => write!(f, "bool"),
            Ty::Null => write!(f, "null"),
            Ty::Pattern => write!(f, "pattern"),
            Ty::List(e) => write!(f, "[{e}]"),
            Ty::Code(_) => write!(f, "[code]"),
            Ty::Named(n) => write!(f, "{n}"),
            Ty::Union(v) => {
                let parts: Vec<String> = v.iter().map(|t| t.to_string()).collect();
                write!(f, "{}", parts.join("|"))
            }
        }
    }
}
