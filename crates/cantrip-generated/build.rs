//! The modules to compile: the sample, with the `all` feature every card, or with `proposals` the planned cards.
//!
//! With `proposals` the cards are the ones planned.txt lists, from cards/. Set `CANTRIP_PROPOSALS` to a directory, as
//! mtgish's tools/proposal_check.py writes it, to compile its cards/ instead (from its list.txt if it has one) with its
//! stubs.rs as the module `proposals`.
use std::fmt::Write;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=sample.txt");
    println!("cargo:rerun-if-changed=planned.txt");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CANTRIP_PROPOSALS");
    let root = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let lines = |path: PathBuf| -> Vec<String> {
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display())).lines().map(String::from).collect()
    };
    let proposals = std::env::var_os("CARGO_FEATURE_PROPOSALS").is_some();
    let check = std::env::var_os("CANTRIP_PROPOSALS").filter(|_| proposals).map(PathBuf::from);
    let mut cards = root.join("cards");
    let mut out = String::new();
    let stems: Vec<String> = if std::env::var_os("CARGO_FEATURE_ALL").is_some() {
        let mut all: Vec<String> = std::fs::read_dir(root.join("cards"))
            .unwrap()
            .map(|e| e.unwrap().path().file_stem().unwrap().to_string_lossy().to_string())
            .collect();
        all.sort();
        all
    } else if let Some(dir) = &check {
        cards = dir.join("cards");
        let stubs = dir.join("stubs.rs");
        println!("cargo:rerun-if-changed={}", dir.join("list.txt").display());
        writeln!(out, "#[path = {:?}]\nmod proposals;", stubs.to_string_lossy()).unwrap();
        let list = dir.join("list.txt");
        lines(if list.exists() { list } else { root.join("planned.txt") })
    } else if proposals {
        lines(root.join("planned.txt"))
    } else {
        lines(root.join("sample.txt"))
    };
    for (i, stem) in stems.iter().enumerate() {
        let path = cards.join(format!("{stem}.rs"));
        writeln!(out, "#[path = {:?}]\nmod card_{i};", path.to_string_lossy()).unwrap();
    }
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    std::fs::write(std::path::Path::new(&out_dir).join("modules.rs"), out).unwrap();
}
