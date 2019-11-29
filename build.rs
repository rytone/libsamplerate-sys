extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut dst = cmake::build("libsamplerate");

    dst.push("lib");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=samplerate");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
