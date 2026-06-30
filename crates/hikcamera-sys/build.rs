use std::env;
use std::path::{Path, PathBuf};

const WATCHED_ENV_VARS: &[&str] = &["LIBCLANG_PATH"];

const RUNTIME_EXTENSIONS: &[&str] = &["dll", "cti", "ini", "ax"];

fn main() {
    watch_env();

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let include_dir = manifest_dir.join("include");
    let header = manifest_dir.join("wrapper.h");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    emit_rerun_if_changed(&header);

    let lib_dir = lib_dir(&manifest_dir);
    let import_lib = lib_dir.join("MvCameraControl.lib");
    let runtime_dll = lib_dir.join("MvCameraControl.dll");

    assert!(
        import_lib.exists(),
        "missing HikCamera import library: {}",
        import_lib.display()
    );
    assert!(
        runtime_dll.exists(),
        "missing HikCamera runtime DLL: {}",
        runtime_dll.display()
    );

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=MvCameraControl");

    let bindings = bindgen::Builder::default()
        .header(header.to_string_lossy())
        .clang_arg(format!("-I{}", include_dir.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to generate HikCamera MVS bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("failed to write HikCamera MVS bindings");

    copy_runtime(&runtime_dll, &out_dir);
}

fn emit_rerun_if_changed(path: &Path) {
    println!("cargo:rerun-if-changed={}", path.display());
}

fn watch_env() {
    for name in WATCHED_ENV_VARS {
        println!("cargo:rerun-if-env-changed={name}");
    }
}

fn lib_dir(manifest_dir: &Path) -> PathBuf {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    assert!(
        target_os == "windows",
        "hikcamera-sys currently supports Windows targets only"
    );

    let target_pointer_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap();
    match target_pointer_width.as_str() {
        "64" => manifest_dir.join("lib").join("win64"),
        "32" => manifest_dir.join("lib").join("win32"),
        _ => panic!("unsupported target pointer width: {target_pointer_width}"),
    }
}

fn copy_runtime(runtime_dll: &Path, out_dir: &Path) {
    let Some(profile_dir) = out_dir.ancestors().find(|path| {
        path.file_name()
            .is_some_and(|name| name == "debug" || name == "release")
    }) else {
        println!(
            "cargo:warning=failed to locate Cargo profile directory from OUT_DIR={}",
            out_dir.display()
        );
        return;
    };

    let runtime_dir = runtime_dll.parent().unwrap();
    copy_runtime_dir(runtime_dir, profile_dir);
}

fn is_runtime_file(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            RUNTIME_EXTENSIONS.contains(&extension.to_ascii_lowercase().as_str())
        })
}

fn copy_runtime_dir(source_dir: &Path, profile_dir: &Path) {
    if !source_dir.exists() {
        return;
    }

    for entry in std::fs::read_dir(source_dir)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", source_dir.display()))
        .filter_map(Result::ok)
    {
        let source = entry.path();
        if source.is_file() && is_runtime_file(&source) {
            let target = profile_dir.join(source.file_name().unwrap());
            std::fs::copy(&source, &target).unwrap_or_else(|err| {
                panic!(
                    "failed to copy {} to {}: {err}",
                    source.display(),
                    target.display()
                )
            });
        }
    }
}
