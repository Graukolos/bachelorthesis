use bindgen::{Builder, MacroTypeVariation};
use std::{env, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    println!(
        "cargo:rustc-link-search={}",
        manifest_dir.join("circle").display()
    ); // for circle.ld
    println!(
        "cargo:rustc-link-search={}",
        manifest_dir.join("circle/lib").display()
    ); // for libcircle.a

    println!("cargo:rustc-link-lib=circle");

    //println!("cargo:rustc-link-arg=--section-start=.init=0x80000");
    //println!("cargo:rustc-link-arg=-Tcircle.ld");

    let bindings = Builder::default()
        .header("wrapper.hpp")
        .use_core()
        .clang_arg(format!(
            "-I{}",
            manifest_dir.join("circle/include").display()
        ))
        .default_macro_constant_type(MacroTypeVariation::Signed)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
