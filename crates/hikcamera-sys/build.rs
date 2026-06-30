use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use target_lexicon::{Architecture, OperatingSystem, Triple};

const HIK_CAMERA_LIB: &str = "MvCameraControl";

const WATCHED_ENV_VARS: &[&str] = &["LIBCLANG_PATH"];

fn main() {
    for name in WATCHED_ENV_VARS {
        println!("cargo:rerun-if-env-changed={name}");
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let include_dir = manifest_dir.join("include");
    let header = manifest_dir.join("wrapper.h");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed={}", header.display());

    let target = Triple::from_str(&env::var("TARGET").expect("Cargo did not set TARGET"))
        .expect("failed to parse TARGET triple");
    let lib_dir = match (&target.operating_system, &target.architecture) {
        (OperatingSystem::Windows, Architecture::X86_64) => manifest_dir.join("lib").join("win64"),
        (OperatingSystem::Windows, Architecture::X86_32(_)) => {
            manifest_dir.join("lib").join("win32")
        }
        _ => panic!("unsupported target for hikcamera-sys: {target}"),
    };
    let link_lib = match target.operating_system {
        OperatingSystem::Windows => HIK_CAMERA_LIB,
        _ => panic!("unsupported target OS for hikcamera-sys: {target}"),
    };

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib={link_lib}");

    let bindings = bindgen::Builder::default()
        .header(header.to_string_lossy())
        .clang_arg(format!("-I{}", include_dir.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to generate HikCamera MVS bindings");

    let generated_bindings = out_dir.join("bindings.rs");
    let review_bindings = manifest_dir.join("src").join("bindings.rs");

    bindings
        .write_to_file(&generated_bindings)
        .expect("failed to write HikCamera MVS bindings");
    bindings
        .write_to_file(&review_bindings)
        .expect("failed to write review copy of HikCamera MVS bindings");
}
