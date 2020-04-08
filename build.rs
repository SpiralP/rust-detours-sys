use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut nmake = cc::windows_registry::find(&target, "nmake.exe").unwrap();

    assert!(nmake.current_dir("Detours/src").status().unwrap().success());

    fs::copy(
        "Detours/lib.X64/detours.lib",
        Path::new(&out_dir).join("detours.lib"),
    )
    .unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=detours");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_arg("-IDetours/include")
        // The input header we would like to generate
        // bindings for.
        .header_contents(
            "bindgen.h",
            r#"
                #include <Windows.h>
                #include "detours.h"
            "#
        )
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_function("Detour.*")
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
