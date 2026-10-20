//! Reads all patterns from the Obsidian vault (`02 Patterns/**/*.md`, YAML front matter).

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};


pub struct Entry {
    pub name: String,
    pub modname: String,
    pub op_id: String,
    pub sig: String,
    pub input: String,
    pub output: String,
    pub cost: String,
    pub per_world: bool,
    pub enlightenment: bool,
    /// vault file the entry comes from
    pub file: PathBuf,
}

pub struct Db {
    pub entries: Vec<Entry>,
    by_name: HashMap<String, Vec<usize>>,
    pub by_sig: HashMap<String, Vec<usize>>,
}

/// Normalises spelling: lower case, straight apostrophes, plain minus.
pub fn norm(s: &str) -> String {
    let s: String = s
        .chars()
        .map(|c| match c {
            '’' | '‘' | '`' | '´' => '\'',
            '−' | '–' | '—' => '-',
            c => c,
        })
        .collect();
    s.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase()
}

fn loose(s: &str) -> String {
    norm(s).chars().filter(|c| c.is_alphanumeric() || *c == '+' || *c == '-').collect()
}

fn unquote(v: &str) -> String {
    let v = v.trim();
    if v.len() >= 2 && v.starts_with('"') && v.ends_with('"') {
        v[1..v.len() - 1].replace("\\\"", "\"").replace("\\\\", "\\")
    } else {
        v.to_string()
    }
}

fn parse_list(v: &str) -> Vec<String> {
    let v = v.trim().trim_start_matches('[').trim_end_matches(']');
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut inq = false;
    let mut esc = false;
    for c in v.chars() {
        if esc {
            cur.push(c);
            esc = false;
        } else if c == '\\' && inq {
            esc = true;
        } else if c == '"' {
            if inq {
                out.push(std::mem::take(&mut cur));
            }
            inq = !inq;
        } else if inq {
            cur.push(c);
        }
    }
    out
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            walk(&p, out);
        } else if p.extension().is_some_and(|x| x == "md") {
            out.push(p);
        }
    }
}

impl Db {
    pub fn load(vault: &Path) -> Result<Db, String> {
        let dir = vault.join("02 Patterns");
        if !dir.is_dir() {
            return Err(format!("folder not found: {}", dir.display()));
        }
        let mut files = Vec::new();
        walk(&dir, &mut files);
        files.sort();
        let mut db = Db { entries: Vec::new(), by_name: HashMap::new(), by_sig: HashMap::new() };
        for f in files {
            let Ok(text) = fs::read_to_string(&f) else { continue };
            let text = text.replace("\r\n", "\n");
            let Some(rest) = text.strip_prefix("---\n") else { continue };
            let Some(end) = rest.find("\n---") else { continue };
            let mut kv: HashMap<&str, &str> = HashMap::new();
            for line in rest[..end].lines() {
                if let Some((k, v)) = line.split_once(':') {
                    kv.insert(k.trim(), v.trim());
                }
            }
            let Some(sig_raw) = kv.get("signature") else { continue };
            let sig = unquote(sig_raw);
            let get = |k: &str| kv.get(k).map(|v| unquote(v)).unwrap_or_default();
            let aliases = kv.get("aliases").map(|v| parse_list(v)).unwrap_or_default();
            let stem = f.file_stem().unwrap().to_string_lossy().to_string();
            let name = aliases.first().cloned().unwrap_or_else(|| stem.clone());
            let idx = db.entries.len();
            db.entries.push(Entry {
                name: name.clone(),
                modname: get("mod"),
                op_id: get("op_id"),
                sig: sig.clone(),
                input: get("input"),
                output: get("output"),
                cost: get("cost"),
                per_world: get("per_world") == "true",
                enlightenment: get("needs_enlightenment") == "true",
                file: f.clone(),
            });
            let mut names = aliases;
            names.push(stem);
            names.push(name);
            names.sort();
            names.dedup();
            for n in names {
                for k in [norm(&n), loose(&n)] {
                    let v = db.by_name.entry(k).or_default();
                    if !v.contains(&idx) {
                        v.push(idx);
                    }
                }
            }
            db.by_sig.entry(sig).or_default().push(idx);
        }
        // real patterns (with op_id) before book examples, Hex Casting first
        let entries = &db.entries;
        for v in db.by_sig.values_mut() {
            v.sort_by_key(|&i| (entries[i].op_id.is_empty(), entries[i].modname != "Hex Casting", i));
        }
        Ok(db)
    }

    /// Looks a pattern up by name. `Name @ Mod` picks the mod when a name is ambiguous.
    pub fn lookup(&self, query: &str) -> Result<usize, String> {
        let (name, modq) = match query.split_once('@') {
            Some((n, m)) => (n.trim(), Some(norm(m))),
            None => (query.trim(), None),
        };
        let hits = self
            .by_name
            .get(&norm(name))
            .or_else(|| self.by_name.get(&loose(name)))
            .cloned()
            .unwrap_or_default();
        let hits: Vec<usize> = match &modq {
            Some(m) => hits.into_iter().filter(|&i| norm(&self.entries[i].modname) == *m).collect(),
            None => hits,
        };
        match hits.len() {
            0 => {
                let sugg = self.search(name).into_iter().take(4).map(|i| self.entries[i].name.clone()).collect::<Vec<_>>();
                if sugg.is_empty() {
                    Err(format!("unknown pattern \"{name}\""))
                } else {
                    Err(format!("unknown pattern \"{name}\" (did you mean: {}?)", sugg.join(", ")))
                }
            }
            1 => Ok(hits[0]),
            _ => {
                // same signature = really the same pattern, prefer Hex Casting
                let first_sig = &self.entries[hits[0]].sig;
                if hits.iter().all(|&i| &self.entries[i].sig == first_sig) {
                    let best = hits.iter().copied().find(|&i| self.entries[i].modname == "Hex Casting").unwrap_or(hits[0]);
                    Ok(best)
                } else {
                    let opts = hits.iter().map(|&i| format!("{} @ {}", self.entries[i].name, self.entries[i].modname)).collect::<Vec<_>>();
                    Err(format!("\"{name}\" is ambiguous, write e.g.: {}", opts.join(" | ")))
                }
            }
        }
    }

    /// Searches by name. Order: name starts with the query, then a word starts with it,
    /// then contains it anywhere. Each signature appears only once.
    pub fn search(&self, query: &str) -> Vec<usize> {
        let q = loose(query);
        if q.is_empty() {
            return Vec::new();
        }
        let words: Vec<String> = norm(query).split(' ').map(loose).filter(|w| !w.is_empty()).collect();
        let mut scored: Vec<(u8, bool, usize, usize)> = Vec::new();
        for (i, e) in self.entries.iter().enumerate() {
            let name = loose(&e.name);
            let name_words: Vec<String> = norm(&e.name).split(' ').map(loose).collect();
            let starts = |w: &str| name_words.iter().any(|nw| nw.starts_with(w));
            let score = if name.starts_with(&q) {
                0
            } else if starts(&q) || (words.len() > 1 && words.iter().all(|w| starts(w))) {
                1
            } else if name.contains(&q) {
                2
            } else if words.iter().all(|w| name.contains(w.as_str())) {
                3
            } else {
                continue;
            };
            scored.push((score, e.op_id.is_empty(), e.name.len(), i));
        }
        scored.sort();
        let mut seen = std::collections::HashSet::new();
        scored.into_iter().map(|s| s.3).filter(|&i| seen.insert(self.entries[i].sig.clone())).collect()
    }
    /// Other patterns with the same signature but a different function.
    pub fn sig_conflicts(&self, idx: usize) -> Vec<usize> {
        let e = &self.entries[idx];
        self.by_sig
            .get(&e.sig)
            .map(|v| {
                v.iter()
                    .copied()
                    .filter(|&j| j != idx && !self.entries[j].op_id.is_empty() && self.entries[j].op_id != e.op_id)
                    .collect()
            })
            .unwrap_or_default()
    }

}
