use std::{
    env, fs,io,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);


    fs::copy(r#"Detours\system.mak"#, out_dir.join("system.mak")).unwrap();

    let src_path = out_dir.join("src");
    if let Err(e) = fs::remove_dir_all(&src_path) {
        if e.kind() != io::ErrorKind::NotFound {
            panic!("{}",e);
        }
    }
    fs::create_dir_all(&src_path).unwrap();
    for entry in fs::read_dir(r#"Detours\src"#).unwrap() {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();
        assert!(metadata.is_file());
        fs::copy(entry.path(), src_path.join(entry.file_name())).unwrap();
    }


    let mut nmake = cc::windows_registry::find_tool(&target, "nmake.exe").unwrap();


    assert!(nmake.to_command()
        .current_dir(&src_path)
        .arg("INCD=include")
        .arg("LIBD=lib")
        .arg("BIND=bin")
        .arg("OBJD=obj")
        .status()
        .unwrap()
        .success());


    println!("cargo:rustc-link-search=native={}", src_path.join("lib").display());
    println!("cargo:rustc-link-lib=static=detours");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_args(&["-I", &format!(r#"{}\src\include"#, out_dir.display())])
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
