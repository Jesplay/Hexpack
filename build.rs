//! Windows with the GNU toolchain only: the minimal MinGW shipped with rustup lacks a few
//! import libraries. The GNU linker can link directly against the system DLLs, so we put
//! copies of them on a search path. On Linux and with MSVC this does nothing.

use std::env;
use std::path::PathBuf;

const MISSING: [&str; 6] = ["shlwapi", "uxtheme", "dwmapi", "imm32", "propsys", "uiautomationcore"];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if !env::var("TARGET").unwrap_or_default().ends_with("windows-gnu") {
        return;
    }
    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("syslibs");
    let _ = std::fs::create_dir_all(&out);
    let sys = PathBuf::from(env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".into())).join("System32");
    for dll in MISSING {
        let src = sys.join(format!("{dll}.dll"));
        let dst = out.join(format!("{dll}.dll"));
        if src.exists() && !dst.exists() {
            let _ = std::fs::copy(&src, &dst);
        }
    }
    println!("cargo:rustc-link-search=native={}", out.display());
}
