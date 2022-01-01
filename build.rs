extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    /* 1. Build git submodule checkout of libh264bitstream */
    // Build the project in the path `foo` and installs it in `$OUT_DIR`
    let dst = autotools::Config::new("libh264bitstream")
        .reconf("-i")
        .build();

    println!("built in {}", dst.as_path().display());
    let libh264bitstream_include_dir = dst.as_path().join("include");
    let libh264bitstream_lib_dir = dst.as_path().join("lib");

    // Simply link the library without using pkg-config
    println!(
        "cargo:rustc-link-search=native={}",
        libh264bitstream_lib_dir.display()
    );
    println!("cargo:rustc-link-lib=static=h264bitstream");

    /* 2. Generate Bindings */

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg(format!("-I{}", libh264bitstream_include_dir.display()))
        .whitelist_function("h264_new")
        .whitelist_function("h264_free")
        .whitelist_function("find_nal_unit")
        .whitelist_function("read_nal_unit")
        .whitelist_function("write_nal_unit")
        .whitelist_function("rbsp_to_nal")
        .whitelist_function("nal_to_rbsp")
        .whitelist_function("debug_nal")
        .whitelist_type("h264_stream_t")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
