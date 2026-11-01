//! The modules to compile: the sample, or with the `all` feature every card.
use std::fmt::Write;

fn main() {
    println!("cargo:rerun-if-changed=sample.txt");
    println!("cargo:rerun-if-changed=build.rs");
    let root = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let stems: Vec<String> = if std::env::var_os("CARGO_FEATURE_ALL").is_some() {
        let mut all: Vec<String> = std::fs::read_dir(root.join("cards"))
            .unwrap()
            .map(|e| e.unwrap().path().file_stem().unwrap().to_string_lossy().to_string())
            .collect();
        all.sort();
        all
    } else {
        std::fs::read_to_string(root.join("sample.txt")).unwrap().lines().map(String::from).collect()
    };
    let mut out = String::new();
    for (i, stem) in stems.iter().enumerate() {
        let path = root.join("cards").join(format!("{stem}.rs"));
        writeln!(out, "#[path = {:?}]\nmod card_{i};", path.to_string_lossy()).unwrap();
    }
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    std::fs::write(std::path::Path::new(&out_dir).join("modules.rs"), out).unwrap();
}
