extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut bindings = bindgen::Builder::default()
    // The input header we would like to generate
    // bindings for.
        .header("wrapper.h");
    // Add the path to the Distrib inc. directory
    let build_include_files = vec!("sr_linux", "sr_unix_gnp", "sr_unix_gnp", "sr_unix_cm", "sr_unix", "sr_port_cm", "sr_port");
    for path in build_include_files {
        bindings = bindings.clang_arg(format!("-I/home/chathaway/programming/YDB/{}", path));
    }
    bindings = bindings.clang_arg("-I/home/chathaway/programming/ydb-ng-bridge");
    let builder = bindings
    // Finish the builder and generate the bindings.
        .generate()
    // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    builder
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
