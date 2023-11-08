extern crate bindgen;
extern crate cmake;

use cmake::Config;
use std::{env, path::PathBuf};

fn main() {
    // Run cmake to build nng
    let dst = Config::new("libiwasm")
        .generator("Unix Makefiles")
        .define("CMAKE_BUILD_TYPE", "Release")
        .no_build_target(true)
        .build();
    // Check output of `cargo build --verbose`, should see something like:
    // -L native=/path/runng/target/debug/build/runng-sys-abc1234/out
    // That contains output from cmake
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("build").display()
    );
    println!("cargo:rustc-link-lib=libiwasm");
    println!("cargo:rustc-link-lib=vmlib");

    let bindings = bindgen::Builder::default()
        .header("wasm-micro-runtime/core/iwasm/include/wasm_export.h")
        // This is needed if use `#include <nng.h>` instead of `#include "path/nng.h"`
        //.clang_arg("-Inng/src/")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
