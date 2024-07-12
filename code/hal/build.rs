use std::{env, fs, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());

    fs::copy("link.x", out_dir.join("link.x")).unwrap();
    println!("cargo:rerun-if-changed=link.x");
}
