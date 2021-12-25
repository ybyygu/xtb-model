// [[file:xtb.note::0b9b361d][0b9b361d]]
use bindgen;
use cc;

use std::env;
use std::path::PathBuf;

fn main() {
    //	gcc -o test main.c -I include/xtb build/libxtb.a -lgfortran -lm -lblas -llapack
    cc::Build::new()
        .cpp(false)
        .include("include")
        .flag("-lm")
        .flag("-lblas")
        .flag("-llapack")
        .flag("-lgfortran")
        // .flag("-Lbuild")
        // .flag("-lxtb")
        .flag("build/libxtb.a")
        // local wrapper for exposing only API what I need
        .file("wrapper.c")
        .compile("libxtbmodel.a");

    println!("cargo:rustc-link-search=build");
    println!("cargo:rustc-link-lib=static=xtb");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=blas");
    println!("cargo:rustc-link-lib=lapack");
    println!("cargo:rustc-link-lib=gfortran");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
// 0b9b361d ends here
