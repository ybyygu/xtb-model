// [[file:xtb.note::0b9b361d][0b9b361d]]
use bindgen;

use std::path::PathBuf;

fn main() {
    //	gcc -o test main.c -I include/xtb build/libxtb.a -lgfortran -lm -lblas -llapack
    println!("cargo:rustc-link-search=lib");
    println!("cargo:rustc-link-lib=static=xtb");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=blas");
    println!("cargo:rustc-link-lib=lapack");
    println!("cargo:rustc-link-lib=gfortran");

    let bindings = bindgen::Builder::default()
        .header("include/xtb.h")
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
// 0b9b361d ends here
