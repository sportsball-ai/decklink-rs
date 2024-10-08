extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings;

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=c++");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");

        cc::Build::new()
            .flag("-stdlib=libc++")
            .include("vendor/Mac/include")
            .file("src/lib.cpp")
            .file("vendor/Mac/include/DeckLinkAPIDispatch_v10_8.cpp")
            .compile("decklink");

        let sdk_root = std::process::Command::new("xcrun")
            .args(["--sdk", "macosx", "--show-sdk-path"])
            .stderr(std::process::Stdio::inherit())
            .output()
            .expect("unable to get sdk path")
            .stdout;
        let sdk_root = String::from_utf8_lossy(&sdk_root).trim().to_string();

        bindings = bindgen::Builder::default()
            .clang_arg("-x")
            .clang_arg("objective-c++")
            .clang_arg(format!("-isysroot{}", sdk_root))
            .clang_arg("-Ivendor/Mac/include")
    }

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=stdc++");

        cc::Build::new()
            .include("vendor/Linux/include")
            .file("src/lib.cpp")
            .file("vendor/Linux/include/DeckLinkAPIDispatch_v10_8.cpp")
            .compile("decklink");

        bindings = bindgen::Builder::default().clang_arg("-Ivendor/Linux/include")
    }

    let bindings = bindings
        .header("src/lib.hpp")
        .allowlist_function("buffer_.+")
        .allowlist_function("unknown_.+")
        .allowlist_function(".*decklink_.+")
        .allowlist_type("_BMD.+")
        .layout_tests(false)
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("unable to write bindings");
}
