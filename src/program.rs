//! A hex as a sequence of patterns, either from a text file or from drawn signatures.
//!
//! Text format, one pattern per line:
//! ```text
//! # comment
//! stack: vec, num            (optional: what is already on the stack)
//! Mind's Reflection
//! Numerical Reflection: 10   (or short: num: 10)
//! Bookkeeper's Gambit: v-    (or short: mask: v-)
//! sig: qaq                   (raw angles)
//! Introspection              (or {)
//!   Place Block
//! Retrospection              (or })
//! Thoth's Gambit
//! ```

use crate::db::{norm, Db};
use crate::geom;
use crate::literals;
use crate::types::{parse_slots, Slot, Ty};

pub enum Kind {
    Op(usize),
    Number(f64),
    Mask(String),
    Unknown,
    Block(usize),
    Escaped,
}

pub struct Step {
    pub line: usize,
    pub label: String,
    pub kind: Kind,
}

/// A pattern that actually gets drawn on the grid.
pub struct Drawn {
    pub label: String,
    pub sig: String,
    pub quoted: bool,
    pub known: bool,
}

pub struct Program {
    pub steps: Vec<Step>,
    pub blocks: Vec<Vec<Step>>,
    pub drawn: Vec<Drawn>,
    pub init: Vec<Ty>,
    pub errors: Vec<(usize, String)>,
}

/// One entry before assembling the program.
pub enum Item {
    Open(String),
    Close(String),
    Escape(String),
    Pat { label: String, sig: String, kind: Kind },
    Stack(Vec<Ty>),
}

fn after_colon<'a>(line: &'a str, keys: &[&str]) -> Option<&'a str> {
    let (k, v) = line.split_once(':')?;
    let k = norm(k);
    keys.iter().any(|x| *x == k).then_some(v.trim())
}

fn fmt_num(v: f64) -> String {
    if v.fract() == 0.0 {
        format!("{}", v as i64)
    } else {
        format!("{v}")
    }
}

/// Parses a hex text file.
pub fn parse(src: &str, db: &Db) -> Program {
    let sig_of = |name: &str, fallback: &str| db.lookup(name).map(|i| db.entries[i].sig.clone()).unwrap_or_else(|_| fallback.to_string());
    let mut items = Vec::new();
    for (ln0, raw) in src.lines().enumerate() {
        let line_no = ln0 + 1;
        let text = raw.split('#').next().unwrap().trim();
        if text.is_empty() {
            continue;
        }
        let n = norm(text);
        let item: Result<Item, String> = if let Some(v) = after_colon(text, &["stack", "start"]) {
            Ok(Item::Stack(
                parse_slots(v)
                    .into_iter()
                    .map(|s| match s {
                        Slot::One(t) => t,
                        Slot::Many => Ty::Any,
                    })
                    .collect(),
            ))
        } else if n == "introspection" || n == "{" {
            Ok(Item::Open(sig_of("Introspection", "qqq")))
        } else if n == "retrospection" || n == "}" {
            Ok(Item::Close(sig_of("Retrospection", "eee")))
        } else if n == "consideration" {
            Ok(Item::Escape(sig_of("Consideration", "qqqaw")))
        } else if let Some(v) = after_colon(text, &["numerical reflection", "num", "zahl"]) {
            match v.replace(',', ".").parse::<f64>() {
                Ok(x) => match literals::number_sig(x) {
                    Some(sig) => Ok(Item::Pat { label: format!("Numerical Reflection: {}", fmt_num(x)), sig, kind: Kind::Number(x) }),
                    None => Err(format!("no drawing found for the number {v}")),
                },
                Err(_) => Err(format!("\"{v}\" is not a number")),
            }
        } else if let Some(v) = after_colon(text, &["bookkeeper's gambit", "mask", "maske"]) {
            literals::mask_sig(v).map(|sig| Item::Pat { label: format!("Bookkeeper's Gambit: {v}"), sig, kind: Kind::Mask(v.to_string()) })
        } else if let Some(v) = after_colon(text, &["sig", "signatur"]) {
            Ok(recognize(&v.split_whitespace().next().unwrap_or("").to_lowercase(), db))
        } else {
            db.lookup(text).map(|i| Item::Pat { label: db.entries[i].name.clone(), sig: db.entries[i].sig.clone(), kind: Kind::Op(i) })
        };
        items.push((line_no, item));
    }
    build(items)
}

/// Builds a program from drawn signatures. Line number = pattern number.
pub fn from_sigs(sigs: &[String], db: &Db) -> Program {
    build(sigs.iter().enumerate().map(|(i, s)| (i + 1, Ok(recognize(s, db)))).collect())
}

/// Recognises a signature: number, Bookkeeper mask, parentheses or a normal pattern.
pub fn recognize(sig: &str, db: &Db) -> Item {
    let hit = db.by_sig.get(sig).and_then(|v| v.first().copied());
    let op = hit.map(|i| db.entries[i].op_id.as_str()).unwrap_or("");
    if hit.is_none() || op == "hexcasting:number" {
        if let Some(v) = literals::decode_number(sig) {
            return Item::Pat { label: format!("Numerical Reflection: {}", fmt_num(v)), sig: sig.into(), kind: Kind::Number(v) };
        }
    }
    match op {
        "hexcasting:open_paren" => return Item::Open(sig.into()),
        "hexcasting:close_paren" => return Item::Close(sig.into()),
        "hexcasting:escape" => return Item::Escape(sig.into()),
        _ => {}
    }
    if hit.is_none() || op == "hexcasting:mask" {
        if let Some(m) = literals::decode_mask(sig) {
            return Item::Pat { label: format!("Bookkeeper's Gambit: {m}"), sig: sig.into(), kind: Kind::Mask(m) };
        }
    }
    match hit {
        Some(i) => Item::Pat { label: db.entries[i].name.clone(), sig: sig.into(), kind: Kind::Op(i) },
        None => Item::Pat { label: "unknown".into(), sig: sig.into(), kind: Kind::Unknown },
    }
}

struct Frame {
    line: usize,
    steps: Vec<Step>,
}

fn build(items: Vec<(usize, Result<Item, String>)>) -> Program {
    let mut p = Program { steps: Vec::new(), blocks: Vec::new(), drawn: Vec::new(), init: Vec::new(), errors: Vec::new() };
    let mut frames: Vec<Frame> = Vec::new();
    let mut escape_next = false;
    let mut seen_pattern = false;

    for (line_no, item) in items {
        let item = match item {
            Ok(i) => i,
            Err(e) => {
                p.errors.push((line_no, e));
                continue;
            }
        };
        if let Item::Stack(ts) = item {
            if seen_pattern {
                p.errors.push((line_no, "`stack:` must come before the first pattern".into()));
            }
            p.init.extend(ts);
            continue;
        }
        seen_pattern = true;
        let quoted = !frames.is_empty() || escape_next;

        // a Consideration escapes the next pattern, whatever it is
        if escape_next {
            escape_next = false;
            let (label, sig) = match item {
                Item::Open(s) => ("Introspection".to_string(), s),
                Item::Close(s) => ("Retrospection".to_string(), s),
                Item::Escape(s) => ("Consideration".to_string(), s),
                Item::Pat { label, sig, .. } => (label, sig),
                Item::Stack(_) => unreachable!(),
            };
            p.drawn.push(Drawn { label: label.clone(), sig, quoted, known: true });
            push(&mut p.steps, &mut frames, Step { line: line_no, label: format!("Consideration + {label}"), kind: Kind::Escaped });
            continue;
        }

        match item {
            Item::Open(sig) => {
                p.drawn.push(Drawn { label: "Introspection".into(), sig, quoted, known: true });
                frames.push(Frame { line: line_no, steps: Vec::new() });
            }
            Item::Close(sig) => {
                p.drawn.push(Drawn { label: "Retrospection".into(), sig, quoted: frames.len() > 1, known: true });
                match frames.pop() {
                    Some(f) => close(&mut p, &mut frames, f, line_no),
                    None => p.errors.push((line_no, "Retrospection without a matching Introspection".into())),
                }
            }
            Item::Escape(sig) => {
                p.drawn.push(Drawn { label: "Consideration".into(), sig, quoted, known: true });
                escape_next = true;
            }
            Item::Pat { label, sig, kind } => {
                if let Err(e) = geom::trace(&sig, 0) {
                    p.errors.push((line_no, format!("{label}: signature {sig} cannot be drawn: {e}")));
                }
                let known = !matches!(kind, Kind::Unknown);
                p.drawn.push(Drawn { label: label.clone(), sig, quoted, known });
                push(&mut p.steps, &mut frames, Step { line: line_no, label, kind });
            }
            Item::Stack(_) => unreachable!(),
        }
    }
    // close open parentheses anyway, so there is something to see while drawing
    while let Some(f) = frames.pop() {
        p.errors.push((f.line, "Introspection is never closed by a Retrospection".into()));
        let end = f.line;
        close(&mut p, &mut frames, f, end);
    }
    if escape_next {
        p.errors.push((p.drawn.len(), "Consideration at the end has nothing to escape".into()));
    }
    p
}

fn close(p: &mut Program, frames: &mut Vec<Frame>, f: Frame, end: usize) {
    let id = p.blocks.len();
    p.blocks.push(f.steps);
    let step = Step { line: f.line, label: format!("Introspection…Retrospection ({}–{})", f.line, end), kind: Kind::Block(id) };
    push(&mut p.steps, frames, step);
}

fn push(top: &mut Vec<Step>, frames: &mut [Frame], step: Step) {
    match frames.last_mut() {
        Some(f) => f.steps.push(step),
        None => top.push(step),
    }
}
