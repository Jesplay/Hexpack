//! Runs the hex on a stack of types and reports what does not fit together.

use std::collections::HashMap;

use crate::db::Db;
use crate::program::{Kind, Program, Step};
use crate::types::{fit, join, parse_slots, subst, Fit, Slot, Ty};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Level {
    Info,
    Warn,
    Error,
}

/// One line of the stack trace.
pub struct Row {
    pub line: usize,
    pub depth: usize,
    pub label: String,
    /// stack afterwards, bottom first (shortened to the topmost entries)
    pub items: Vec<String>,
    /// there may be unknown iotas below the ones shown
    pub unknown: bool,
    /// sub-heading (loop body etc.), not a pattern
    pub header: bool,
}

/// The stack as one line of text (for the terminal).
pub fn stack_text(items: &[String], unknown: bool) -> String {
    let floor = if unknown { "?? " } else { "" };
    if items.is_empty() {
        format!("{floor}(empty)")
    } else {
        format!("{floor}{}", items.join(" | "))
    }
}

pub struct Msg {
    pub line: usize,
    pub level: Level,
    pub text: String,
}

#[derive(Clone)]
struct Stack {
    items: Vec<Ty>,
    /// there may be unknown iotas below the known ones
    unknown_floor: bool,
}

pub struct Checker<'a> {
    db: &'a Db,
    prog: &'a Program,
    pub msgs: Vec<Msg>,
    pub rows: Vec<Row>,
    pub final_items: Vec<String>,
    pub final_unknown: bool,
    pub costs: Vec<String>,
    halted: bool,
}

const MAX_SHOWN: usize = 6;

fn show(st: &Stack) -> Vec<String> {
    let mut parts: Vec<String> = st.items.iter().map(|t| t.to_string()).collect();
    if parts.len() > MAX_SHOWN {
        let cut = parts.len() - MAX_SHOWN;
        parts.drain(..cut);
        parts.insert(0, format!("+{cut}"));
    }
    parts
}

fn int_of(t: &Ty) -> Option<i64> {
    match t {
        Ty::Num(Some(v)) if v.fract() == 0.0 => Some(*v as i64),
        _ => None,
    }
}

impl<'a> Checker<'a> {
    pub fn new(db: &'a Db, prog: &'a Program) -> Self {
        Checker { db, prog, msgs: Vec::new(), rows: Vec::new(), final_items: Vec::new(), final_unknown: false, costs: Vec::new(), halted: false }
    }

    pub fn run(&mut self) {
        let mut st = Stack { items: self.prog.init.clone(), unknown_floor: false };
        self.rows.push(Row { line: 0, depth: 0, label: "Start".into(), items: show(&st), unknown: st.unknown_floor, header: false });
        let prog = self.prog;
        self.block(&prog.steps, &mut st, 0, false);
        self.final_items = show(&st);
        self.final_unknown = st.unknown_floor;
    }

    fn msg(&mut self, line: usize, level: Level, text: impl Into<String>) {
        let text = text.into();
        if !self.msgs.iter().any(|m| m.line == line && m.text == text) {
            self.msgs.push(Msg { line, level, text });
        }
    }

    fn pop(&mut self, st: &mut Stack, step: &Step) -> Ty {
        match st.items.pop() {
            Some(t) => t,
            None => {
                if !st.unknown_floor {
                    self.msg(step.line, Level::Error, format!("{}: the stack is too short", step.label));
                }
                Ty::Any
            }
        }
    }

    fn forget(st: &mut Stack) {
        st.items.clear();
        st.unknown_floor = true;
    }

    fn block(&mut self, steps: &[Step], st: &mut Stack, depth: usize, in_loop: bool) {
        for step in steps {
            if self.halted {
                return;
            }
            self.step(step, st, depth, in_loop);
            self.rows.push(Row { line: step.line, depth, label: step.label.clone(), items: show(st), unknown: st.unknown_floor, header: false });
        }
    }

    fn step(&mut self, step: &Step, st: &mut Stack, depth: usize, in_loop: bool) {
        match &step.kind {
            Kind::Number(v) => st.items.push(Ty::Num(Some(*v))),
            Kind::Block(id) => st.items.push(Ty::Code(*id)),
            Kind::Escaped => st.items.push(Ty::Pattern),
            Kind::Unknown => {
                self.msg(step.line, Level::Warn, format!("{}: pattern not in the vault, stack unchecked from here", step.label));
                Self::forget(st);
            }
            Kind::Mask(mask) => {
                let mut taken = Vec::new();
                for _ in 0..mask.chars().count() {
                    taken.push(self.pop(st, step));
                }
                taken.reverse();
                for (t, c) in taken.into_iter().zip(mask.chars()) {
                    if c == '-' {
                        st.items.push(t);
                    }
                }
            }
            Kind::Op(i) => self.op(*i, step, st, depth, in_loop),
        }
    }

    fn op(&mut self, idx: usize, step: &Step, st: &mut Stack, depth: usize, in_loop: bool) {
        let db = self.db;
        let e = &db.entries[idx];
        if e.per_world {
            self.msg(step.line, Level::Warn, format!("{}: great spell, its angles differ per world, the vault's may be wrong", e.name));
        }
        if e.enlightenment {
            self.msg(step.line, Level::Info, format!("{}: needs enlightenment", e.name));
        }
        for j in self.db.sig_conflicts(idx) {
            let o = &self.db.entries[j];
            self.msg(step.line, Level::Warn, format!("{}: \"{}\" ({}) has the same angles, only one wins in game", e.name, o.name, o.modname));
        }
        if !e.cost.is_empty() {
            let where_ = if in_loop { " (per iteration)" } else { "" };
            self.costs.push(format!("#{}  {}: {}{}", step.line, e.name, e.cost, where_));
        }

        let id = e.op_id.strip_prefix("hexcasting:").unwrap_or(&e.op_id);
        let n = st.items.len();
        macro_rules! need {
            ($k:expr) => {
                if n < $k {
                    if !st.unknown_floor {
                        self.msg(step.line, Level::Error, format!("{}: needs {} iotas, the stack has {}", e.name, $k, n));
                    }
                    Self::forget(st);
                    return;
                }
            };
        }
        match id {
            "swap" => {
                need!(2);
                st.items.swap(n - 1, n - 2);
            }
            "rotate" => {
                need!(3);
                let t = st.items.remove(n - 3);
                st.items.push(t);
            }
            "rotate_reverse" => {
                need!(3);
                let t = st.items.pop().unwrap();
                st.items.insert(n - 3, t);
            }
            "over" => {
                need!(2);
                st.items.push(st.items[n - 2].clone());
            }
            "tuck" => {
                need!(2);
                let t = st.items[n - 1].clone();
                st.items.insert(n - 2, t);
            }
            "duplicate" => {
                need!(1);
                st.items.push(st.items[n - 1].clone());
            }
            "2dup" => {
                need!(2);
                st.items.push(st.items[n - 2].clone());
                st.items.push(st.items[n - 2].clone());
            }
            "duplicate_n" => {
                let cnt = self.pop(st, step);
                let x = self.pop(st, step);
                match int_of(&cnt) {
                    Some(k) if k >= 0 => {
                        for _ in 0..k {
                            st.items.push(x.clone());
                        }
                    }
                    Some(k) => self.msg(step.line, Level::Error, format!("{}: count {k} is negative", e.name)),
                    None => {
                        self.msg(step.line, Level::Info, format!("{}: count only known at runtime, stack unchecked after", e.name));
                        Self::forget(st);
                    }
                }
            }
            "last_n_list" => {
                let cnt = self.pop(st, step);
                match int_of(&cnt) {
                    Some(k) if k >= 0 => {
                        let k = k as usize;
                        if st.items.len() < k && !st.unknown_floor {
                            self.msg(step.line, Level::Error, format!("{}: wants to pack {k} iotas, the stack has {}", e.name, st.items.len()));
                        }
                        let take = k.min(st.items.len());
                        let items: Vec<Ty> = st.items.split_off(st.items.len() - take);
                        let unknown = take < k;
                        let el = if unknown { Ty::Any } else { join(&items) };
                        st.items.push(Ty::List(Box::new(el)));
                    }
                    _ => {
                        self.msg(step.line, Level::Info, format!("{}: count only known at runtime, stack unchecked after", e.name));
                        Self::forget(st);
                        st.items.push(Ty::List(Box::new(Ty::Any)));
                    }
                }
            }
            "stack_len" => {
                let v = if st.unknown_floor { None } else { Some(st.items.len() as f64) };
                st.items.push(Ty::Num(v));
            }
            "splat" => {
                let _ = self.pop(st, step);
                Self::forget(st);
            }
            "fisherman" | "fisherman/copy" => {
                let k = self.pop(st, step);
                match int_of(&k) {
                    Some(k) if k >= 1 && (k as usize) <= st.items.len() => {
                        let at = st.items.len() - k as usize;
                        let t = if id == "fisherman" { st.items.remove(at) } else { st.items[at].clone() };
                        st.items.push(t);
                    }
                    Some(k) if !st.unknown_floor => {
                        self.msg(step.line, Level::Error, format!("{}: depth {k} does not exist on the stack", e.name));
                        Self::forget(st);
                    }
                    _ => Self::forget(st),
                }
            }
            "add" | "sub" | "mul" | "div" | "modulo" | "pow" => {
                let b = self.pop(st, step);
                let a = self.pop(st, step);
                st.items.push(self.arith(id, &a, &b, step));
            }
            "floor" | "ceil" | "coerce_axial" => {
                let a = self.pop(st, step);
                let r = match a {
                    Ty::Num(Some(v)) => Ty::Num(Some(match id {
                        "floor" => v.floor(),
                        "ceil" => v.ceil(),
                        _ => v.signum(),
                    })),
                    Ty::Num(None) | Ty::Vec => a,
                    Ty::Any | Ty::Var(_) | Ty::Union(_) => Ty::Union(vec![Ty::Num(None), Ty::Vec]),
                    other => {
                        self.msg(step.line, Level::Error, format!("{}: needs num or vec, got {other}", e.name));
                        Ty::Any
                    }
                };
                st.items.push(r);
            }
            "for_each" => {
                let list = self.pop(st, step);
                let code = self.pop(st, step);
                if matches!(list, Ty::Code(_)) && matches!(code, Ty::List(_)) {
                    self.msg(step.line, Level::Error, format!("{}: code and list are swapped, a Jester's Gambit is missing before it", e.name));
                    st.items.push(Ty::List(Box::new(Ty::Any)));
                    return;
                }
                let el = match &list {
                    Ty::List(e) => (**e).clone(),
                    Ty::Code(_) => Ty::Pattern,
                    Ty::Any => Ty::Any,
                    other => {
                        self.msg(step.line, Level::Error, format!("{}: expects a list on top, got {other} (wrong order?)", e.name));
                        Ty::Any
                    }
                };
                let out = match code {
                    Ty::Code(b) => {
                        let mut inner = st.clone();
                        inner.items.push(el);
                        let prog = self.prog;
                        let body = &prog.blocks[b];
                        self.rows.push(Row { line: 0, depth: depth + 1, label: "for each element".into(), items: Vec::new(), unknown: false, header: true });
                        self.block(body, &mut inner, depth + 1, true);
                        if inner.unknown_floor {
                            Ty::Any
                        } else {
                            join(&inner.items)
                        }
                    }
                    Ty::Pattern | Ty::Any => Ty::Any,
                    other => {
                        self.msg(step.line, Level::Error, format!("{}: expects code below the list, got {other}", e.name));
                        Ty::Any
                    }
                };
                st.items.push(Ty::List(Box::new(out)));
            }
            "eval" => {
                let code = self.pop(st, step);
                match code {
                    Ty::Code(b) => {
                        let prog = self.prog;
                        let body = &prog.blocks[b];
                        self.rows.push(Row { line: 0, depth: depth + 1, label: "Hermes runs".into(), items: Vec::new(), unknown: false, header: true });
                        self.block(body, st, depth + 1, in_loop);
                    }
                    _ => Self::forget(st),
                }
            }
            id if id.starts_with("const/vec/") => st.items.push(Ty::Vec),
            "halt" => self.halted = true,
            "open_paren" | "close_paren" | "escape" => {}
            _ => self.generic(idx, step, st),
        }
    }

    fn arith(&mut self, id: &str, a: &Ty, b: &Ty, step: &Step) -> Ty {
        use Ty::*;
        let num_or_vec = |t: &Ty| matches!(t, Num(_) | Vec | Any | Var(_)) || matches!(t, Union(v) if v.iter().all(|x| matches!(x, Num(_) | Vec | Null)));
        if !num_or_vec(a) || !num_or_vec(b) {
            // adding lists etc. exists too, we do not check that in detail
            if matches!((a, b), (List(_), List(_))) || matches!((a, b), (Code(_), _) | (_, Code(_))) {
                return a.clone();
            }
            self.msg(step.line, Level::Warn, format!("{}: math on {a} and {b} is unusual", step.label));
            return Any;
        }
        match (a, b) {
            (Num(Some(x)), Num(Some(y))) => {
                let v = match id {
                    "add" => x + y,
                    "sub" => x - y,
                    "mul" => x * y,
                    "div" => {
                        if *y == 0.0 {
                            self.msg(step.line, Level::Error, format!("{}: division by zero", step.label));
                            return Num(None);
                        }
                        x / y
                    }
                    "modulo" => x.rem_euclid(*y),
                    _ => x.powf(*y),
                };
                Num(Some(v))
            }
            (Num(_), Num(_)) => Num(None),
            (Vec, Vec) => match id {
                "mul" => Num(None),
                _ => Vec,
            },
            (Vec, Num(_)) | (Num(_), Vec) => {
                if id == "div" && matches!(b, Num(Some(y)) if *y == 0.0) {
                    self.msg(step.line, Level::Error, format!("{}: division by zero", step.label));
                }
                if matches!((id, a, b), ("add" | "sub", _, _)) {
                    self.msg(step.line, Level::Warn, format!("{}: mixing vec and num in +/- is a mishap in game", step.label));
                }
                Vec
            }
            _ => Union(vec![Num(None), Vec]),
        }
    }

    fn generic(&mut self, idx: usize, step: &Step, st: &mut Stack) {
        let db = self.db;
        let e = &db.entries[idx];
        if e.input.is_empty() && e.output.is_empty() && e.cost.is_empty() {
            self.msg(step.line, Level::Info, format!("{}: the vault lists no inputs/outputs, stack unchecked from here", e.name));
            Self::forget(st);
            return;
        }
        let ins = parse_slots(&e.input);
        let outs = parse_slots(&e.output);
        let mut vars: HashMap<char, Ty> = HashMap::new();
        if ins.iter().any(|s| matches!(s, Slot::Many)) {
            // check the fixed iotas on top, everything below is unknown
            for s in ins.iter().rev() {
                match s {
                    Slot::Many => break,
                    Slot::One(want) => {
                        let have = self.pop(st, step);
                        self.check_fit(&have, want, step, &mut vars);
                    }
                }
            }
            Self::forget(st);
        } else {
            let mut got = Vec::new();
            for s in ins.iter().rev() {
                if let Slot::One(want) = s {
                    got.push((self.pop(st, step), want.clone()));
                }
            }
            for (have, want) in got.iter().rev() {
                self.check_fit(have, want, step, &mut vars);
            }
        }
        for s in &outs {
            match s {
                Slot::One(t) => st.items.push(subst(t, &vars)),
                Slot::Many => Self::forget(st),
            }
        }
    }

    fn check_fit(&mut self, have: &Ty, want: &Ty, step: &Step, vars: &mut HashMap<char, Ty>) {
        match fit(have, want, vars) {
            Fit::Yes => {}
            Fit::Maybe => self.msg(step.line, Level::Warn, format!("{}: expects {want}, got {have}, might fit", step.label)),
            Fit::No => self.msg(step.line, Level::Error, format!("{}: expects {want}, got {have}", step.label)),
        }
    }
}
