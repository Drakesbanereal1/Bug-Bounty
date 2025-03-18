extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to link the system Z3 shared library
    println!("cargo:rustc-link-lib=z3");

    // Specify the correct path to z3.h
    let z3_include_path = r"C:\Users\steve\Desktop\CIS class\Bug-Bounty\z3\include";

    let bindings = bindgen::Builder::default()
        .header(format!("{}/z3.h", z3_include_path)) // Corrected path
        .clang_arg(format!("-I{}", z3_include_path)) // Pass the include path
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
