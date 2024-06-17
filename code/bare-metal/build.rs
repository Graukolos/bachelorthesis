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
        manifest_dir.join("circle/lib/usb").display()
    ); // for libusb.a
    println!(
        "cargo:rustc-link-search={}",
        manifest_dir.join("circle/lib/input").display()
    ); // for libinput.a
    println!(
        "cargo:rustc-link-search={}",
        manifest_dir.join("circle/lib/fs/fat").display()
    ); // for libfatfs.a
    println!(
        "cargo:rustc-link-search={}",
        manifest_dir.join("circle/lib/fs").display()
    ); // for libfs.a
    println!(
        "cargo:rustc-link-search={}",
        manifest_dir.join("circle/lib").display()
    ); // for libcircle.a

    println!("cargo:rustc-link-lib=usb");
    println!("cargo:rustc-link-lib=input");
    println!("cargo:rustc-link-lib=fatfs");
    println!("cargo:rustc-link-lib=fs");
    println!("cargo:rustc-link-lib=circle");

    let bindings = Builder::default()
        .header("wrapper.hpp")
        .use_core()
        .clang_arg(format!(
            "-I{}",
            manifest_dir.join("circle/include").display()
        ))
        .vtable_generation(true)
        .default_macro_constant_type(MacroTypeVariation::Signed)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
