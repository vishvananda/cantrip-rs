fn main() {
    println!("cargo:rerun-if-env-changed=MTGISH_SOURCE_ROOT");
    let root=std::env::var("MTGISH_SOURCE_ROOT").expect("Set MTGISH_SOURCE_ROOT to the pinned fixture checkout; see README.md");
    println!("cargo:rustc-env=MTGISH_SOURCE_ROOT={root}");
}
