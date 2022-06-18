/// Build script for jpeg-turbo

use bindgen;
use cmake;
use std::path;

fn main() {
    // build libjpeg-turbo with cmake
    let outdir = cmake::build("libjpeg-turbo");
    
    // tell cargo where to find our static library
    println!("cargo:rustc-link-search-native={}/lib", outdir.display());

    // tell cargo which library to link against
    println!("cargo:rustc-link-lib=jpeg");

    // generate bindings for libjpeg-turbo
    let outdir = std::env::var("OUT_DIR").expect("missing OUT_DIR variable");
    let include_dir_arg = format!("-I{}/include", outdir);
    let infile = format!("{}/include/turbojpeg.h", &outdir);

    let bindings = bindgen::builder()
        .header(infile)
        .clang_arg(include_dir_arg)
        .generate()
        .expect("Failed to generate rust bindings to libjpeg-turbo");

    let outdir : path::PathBuf = outdir.into();
    let outfile = outdir.join("bindings.rs");

    bindings.write_to_file(outfile).expect("Failed to write bindings");
}