//! hexpack: checks Hex Casting hexes against the Hexwoven Vault and packs the patterns
//! as compactly as possible (16:9 by default) onto the drawing grid.

mod check;
mod db;
mod fx;
mod geom;
mod gui;
mod literals;
mod pack;
mod program;
mod render;
mod types;

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

use check::{stack_text, Checker, Level};

const HELP: &str = "\
hexpack: check hexes and pack their patterns compactly

  hexpack                            open the app (also by double-click)
  hexpack ui <file.hex>              open the app with this file
  hexpack <file.hex> [options]       check + pack in the terminal
  hexpack search <text>              search patterns in the vault
  hexpack number <value>             drawing for Numerical Reflection

Options:
  --vault <folder>     vault folder (default: HEXPACK_VAULT, or a \"Hexwoven Vault\" folder
                       next to the program or in the current folder)
  --ratio <W:H>        aspect ratio, default 16:9
  --time <seconds>     time budget for packing, default 5
  --svg <file>         SVG output, default <file>.svg
  --check-only         do not pack
  --seed <number>      random seed (same seed = same result)
";

#[cfg(windows)]
fn utf8_console() {
    #[link(name = "kernel32")]
    extern "system" {
        fn SetConsoleOutputCP(id: u32) -> i32;
    }
    unsafe {
        SetConsoleOutputCP(65001);
    }
}
#[cfg(not(windows))]
fn utf8_console() {}

/// A double-click gives the program its own console window; close it.
#[cfg(windows)]
fn hide_own_console() {
    #[link(name = "kernel32")]
    extern "system" {
        fn GetConsoleProcessList(list: *mut u32, count: u32) -> u32;
        fn FreeConsole() -> i32;
    }
    unsafe {
        let mut buf = [0u32; 4];
        if GetConsoleProcessList(buf.as_mut_ptr(), 4) <= 1 {
            FreeConsole();
        }
    }
}
#[cfg(not(windows))]
fn hide_own_console() {}

/// Where the vault is looked for: `HEXPACK_VAULT`, then a `Hexwoven Vault` folder next to the
/// executable or up to three folders above it (so `target/release/hexpack` finds the one in the
/// repository), then in the current folder.
fn default_vault() -> PathBuf {
    if let Ok(v) = std::env::var("HEXPACK_VAULT") {
        return PathBuf::from(v);
    }
    let mut bases: Vec<PathBuf> = Vec::new();
    if let Some(dir) = std::env::current_exe().ok().and_then(|e| e.parent().map(|p| p.to_path_buf())) {
        bases.extend(dir.ancestors().take(4).map(|p| p.to_path_buf()));
    }
    if let Ok(cwd) = std::env::current_dir() {
        bases.push(cwd);
    }
    for base in &bases {
        let p = base.join("Hexwoven Vault");
        if p.join("02 Patterns").is_dir() {
            return p;
        }
    }
    PathBuf::from("Hexwoven Vault")
}

struct Opts {
    file: Option<String>,
    cmd: Option<(String, String)>,
    vault: PathBuf,
    ratio: (f64, f64),
    secs: f64,
    svg: Option<PathBuf>,
    check_only: bool,
    seed: u64,
}

fn parse_args() -> Result<Opts, String> {
    let mut o = Opts { file: None, cmd: None, vault: default_vault(), ratio: (16.0, 9.0), secs: 5.0, svg: None, check_only: false, seed: 0x9e37_79b9_7f4a_7c15 };
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    let val = |i: &mut usize, name: &str| -> Result<String, String> {
        *i += 1;
        args.get(*i).cloned().ok_or(format!("{name} needs a value"))
    };
    while i < args.len() {
        let a = args[i].as_str();
        match a {
            "-h" | "--help" | "help" => return Err(String::new()),
            "--vault" => o.vault = PathBuf::from(val(&mut i, a)?),
            "--ratio" | "--format" => {
                let v = val(&mut i, a)?;
                let (w, h) = v.split_once(':').ok_or("--ratio expects W:H, e.g. 16:9")?;
                o.ratio = (w.trim().parse().map_err(|_| "--ratio: width is not a number")?, h.trim().parse().map_err(|_| "--ratio: height is not a number")?);
            }
            "--time" | "--zeit" => o.secs = val(&mut i, a)?.parse().map_err(|_| "--time expects seconds")?,
            "--svg" => o.svg = Some(PathBuf::from(val(&mut i, a)?)),
            "--seed" => o.seed = val(&mut i, a)?.parse().map_err(|_| "--seed expects a number")?,
            "--check-only" | "--nur-pruefen" => o.check_only = true,
            "search" | "number" | "suche" | "zahl" if o.file.is_none() && o.cmd.is_none() => {
                let cmd = match a {
                    "suche" => "search",
                    "zahl" => "number",
                    c => c,
                };
                o.cmd = Some((cmd.to_string(), args[i + 1..].join(" ")));
                break;
            }
            _ if a.starts_with("--") => return Err(format!("unknown option {a}")),
            _ => o.file = Some(a.to_string()),
        }
        i += 1;
    }
    Ok(o)
}

fn main() -> ExitCode {
    if std::env::args().len() == 1 || std::env::args().nth(1).as_deref() == Some("ui") {
        hide_own_console();
        // hexpack ui [file.hex]  (vault via HEXPACK_VAULT)
        let preload = std::env::args().nth(2).and_then(|p| std::fs::read_to_string(p).ok()).map(|s| s.trim_start_matches('\u{feff}').to_string());
        return match gui::run(default_vault(), preload) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("could not open the window: {e}");
                ExitCode::FAILURE
            }
        };
    }
    utf8_console();
    let o = match parse_args() {
        Ok(o) => o,
        Err(e) => {
            if !e.is_empty() {
                eprintln!("error: {e}\n");
            }
            eprint!("{HELP}");
            return ExitCode::from(2);
        }
    };

    if let Some(("number", v)) = o.cmd.as_ref().map(|(a, b)| (a.as_str(), b.as_str())) {
        return match v.trim().replace(',', ".").parse::<f64>().ok().and_then(literals::number_sig) {
            Some(sig) => {
                println!("{sig}");
                ExitCode::SUCCESS
            }
            None => {
                eprintln!("no drawing found for \"{v}\"");
                ExitCode::FAILURE
            }
        };
    }

    let db = match db::Db::load(&o.vault) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("could not read the vault: {e}\npass its folder with --vault");
            return ExitCode::FAILURE;
        }
    };

    if let Some(("search", q)) = o.cmd.as_ref().map(|(a, b)| (a.as_str(), b.as_str())) {
        let hits = db.search(q);
        if hits.is_empty() {
            println!("nothing found for \"{q}\"");
        }
        for i in hits.into_iter().take(30) {
            let e = &db.entries[i];
            let io = format!("{} → {}", if e.input.is_empty() { "–" } else { &e.input }, if e.output.is_empty() { "–" } else { &e.output });
            println!("{:<36} {:<16} {:<18} {}", e.name, e.sig, e.modname, io);
        }
        return ExitCode::SUCCESS;
    }

    let Some(file) = o.file.clone() else {
        eprint!("{HELP}");
        return ExitCode::from(2);
    };
    let src = match std::fs::read_to_string(&file) {
        Ok(s) => s.trim_start_matches('\u{feff}').to_string(),
        Err(e) => {
            eprintln!("{file}: {e}");
            return ExitCode::FAILURE;
        }
    };

    println!("Vault: {} patterns loaded", db.entries.len());
    let prog = program::parse(&src, &db);

    // 1. Does it work?
    println!("\n━━ 1. Check ━━");
    let mut chk = Checker::new(&db, &prog);
    if prog.errors.is_empty() {
        chk.run();
        println!("{:>4}  {:<38} {}", "line", "pattern", "stack after (top = right)");
        for r in &chk.rows {
            let label = format!("{}{}", "  ".repeat(r.depth), r.label);
            let ln = if r.line == 0 { String::new() } else { r.line.to_string() };
            let stack = if r.header { String::new() } else { stack_text(&r.items, r.unknown) };
            println!("{:>4}  {:<38} {}", ln, label, stack);
        }
    }
    let mut all: Vec<(usize, Level, String)> = prog.errors.iter().map(|(l, t)| (*l, Level::Error, t.clone())).collect();
    all.extend(chk.msgs.iter().map(|m| (m.line, m.level, m.text.clone())));
    all.sort_by_key(|m| m.0);
    let errors = all.iter().filter(|m| m.1 == Level::Error).count();
    let warns = all.iter().filter(|m| m.1 == Level::Warn).count();
    if !all.is_empty() {
        println!();
        for (line, lvl, text) in &all {
            let tag = match lvl {
                Level::Error => "ERROR",
                Level::Warn => "warn ",
                Level::Info => "note ",
            };
            println!("  {tag}  line {line}: {text}");
        }
    }
    if !chk.costs.is_empty() {
        println!("\nCosts:");
        for c in &chk.costs {
            println!("  {c}");
        }
    }
    println!(
        "\n{}",
        match (errors, warns) {
            (0, 0) => "✔ No problems found.".to_string(),
            (0, w) => format!("✔ No errors, {w} warning(s)."),
            (e, w) => format!("✘ {e} error(s), {w} warning(s)."),
        }
    );
    if !prog.errors.is_empty() {
        println!("(The file has errors, so the stack was not simulated.)");
    }

    if o.check_only || prog.drawn.is_empty() {
        return if errors == 0 { ExitCode::SUCCESS } else { ExitCode::FAILURE };
    }

    // 2. Pack
    println!("\n━━ 2. Layout ({}:{}, no gap between patterns) ━━", o.ratio.0, o.ratio.1);
    let shapes: Vec<pack::Shape> = prog.drawn.iter().map(|d| pack::Shape::new(&d.sig)).collect();
    let (lay, stats) = pack::pack(&shapes, o.ratio, Duration::from_secs_f64(o.secs.max(0.05)), o.seed);
    let ps = render::placed(&shapes, &lay);
    let b = render::bounds(&ps);
    let cols = (b.1 - b.0) / 2 + 1;
    let rows = b.3 - b.2 + 1;
    println!("{}\n", render::ascii(&ps));
    println!("Size: {rows} rows × {cols} points ({} greedy runs, {} improvement steps)", stats.greedy_runs, stats.improve_steps);
    let real = ((b.1 - b.0) as f64 * 0.866) / ((b.3 - b.2).max(1) as f64 * 1.5);
    println!("Aspect: {:.2} : 1 (target {:.2} : 1)\n", real, o.ratio.0 / o.ratio.1);
    for (i, (pl, d)) in ps.iter().zip(&prog.drawn).enumerate() {
        let (row, col) = render::row_col(pl.path[0], b);
        let q = if d.quoted { " (quoted)" } else { "" };
        println!(
            "  {} {:>2}. {:<34} {:<12} start row {row:>2} point {col:>2}, first stroke {}{q}",
            render::tag(i),
            i + 1,
            d.label,
            d.sig,
            geom::DIR_NAMES[pl.rot]
        );
    }
    let svg_path = o.svg.clone().unwrap_or_else(|| PathBuf::from(&file).with_extension("svg"));
    let title = format!("hexpack: {}", file);
    match std::fs::write(&svg_path, render::svg(&ps, &prog.drawn, &title)) {
        Ok(()) => println!("\nImage saved: {}", svg_path.display()),
        Err(e) => eprintln!("could not write the SVG: {e}"),
    }
    if errors == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
