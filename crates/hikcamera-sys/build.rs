use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;

const HIK_CAMERA_LIB: &str = "MvCameraControl";

fn main() {
    println!("cargo:rerun-if-env-changed=LIBCLANG_PATH");
    println!("cargo:rerun-if-env-changed=CONDA_PREFIX");
    println!("cargo:rerun-if-env-changed=PIXI_PROJECT_ROOT");
    println!("cargo:rerun-if-env-changed=PIXI_ENVIRONMENT_NAME");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let sdk_dir = sdk_library_dir(&manifest_dir);
    let include_dir = sdk_dir.join("include").join("hikcamera-mvs");
    let header = manifest_dir.join("wrapper.h");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed={}", header.display());
    println!("cargo:rerun-if-changed={}", sdk_dir.display());

    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS unset");
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH unset");
    match (target_os.as_str(), target_arch.as_str()) {
        ("windows", "x86_64") => {}
        _ => panic!("unsupported target for hikcamera-sys: {target_os}-{target_arch}"),
    }
    let lib_dir = sdk_dir.join("lib");
    require_dir("HikCamera MVS headers", &include_dir);
    require_dir("HikCamera MVS import libraries", &lib_dir);
    require_file(
        "HikCamera MVS import library",
        &lib_dir.join(format!("{HIK_CAMERA_LIB}.lib")),
    );

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib={HIK_CAMERA_LIB}");

    // Bindgen cannot emit the SDK's unsigned hex error literals as `i32`
    let status_codes_out = out_dir.join("status_codes.rs");
    let status_codes_src = generate_status_codes(&include_dir);

    // Blocklist status-code headers because `status_codes.rs` owns them
    let bindings = bindgen::Builder::default()
        .header(header.to_string_lossy())
        .clang_arg(format!("-I{}", include_dir.display()))
        .blocklist_file(r".*MvErrorDefine\.h")
        .blocklist_file(r".*MvISPErrorDefine\.h")
        .generate()
        .expect("failed to generate HikCamera MVS bindings");

    fs::write(&status_codes_out, &status_codes_src).expect("failed to write status_codes.rs");
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("failed to write HikCamera MVS bindings");

    // Keep review copies at stable paths for SDK upgrade diffs
    let generated_dir = target_dir(&manifest_dir).join("generated");
    write_generated(&generated_dir.join("bindings.rs"), bindings.to_string());
    write_generated(&generated_dir.join("status_codes.rs"), status_codes_src);
}

fn sdk_library_dir(manifest_dir: &Path) -> PathBuf {
    if let Some(prefix) = env::var_os("CONDA_PREFIX") {
        return PathBuf::from(prefix).join("Library");
    }

    let project_root = env::var_os("PIXI_PROJECT_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| workspace_root(manifest_dir));
    let environment_name = env::var_os("PIXI_ENVIRONMENT_NAME").unwrap_or_else(|| "default".into());

    project_root
        .join(".pixi")
        .join("envs")
        .join(environment_name)
        .join("Library")
}

fn workspace_root(manifest_dir: &Path) -> PathBuf {
    manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .map(Path::to_path_buf)
        .unwrap_or_else(|| manifest_dir.to_path_buf())
}

fn require_dir(label: &str, path: &Path) {
    assert!(
        path.is_dir(),
        "{label} directory not found: {}",
        path.display()
    );
}

fn require_file(label: &str, path: &Path) {
    assert!(path.is_file(), "{label} not found: {}", path.display());
}

fn target_dir(manifest_dir: &Path) -> PathBuf {
    if let Some(dir) = env::var_os("CARGO_TARGET_DIR") {
        return PathBuf::from(dir);
    }
    manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("target"))
        .unwrap_or_else(|| PathBuf::from("target"))
}

fn write_generated(path: &Path, contents: String) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, contents);
}

/// Generate SDK status-code constants as `i32`, matching C API return values
fn generate_status_codes(include_dir: &Path) -> String {
    let sources = [
        include_dir.join("MvErrorDefine.h"),
        include_dir.join("MvISPErrorDefine.h"),
    ];

    // Other MV_* macros are enum-parameter values and should stay in bindgen
    let define_re = Regex::new(
        r"^\s*#define\s+(MV_OK|MV_E_[A-Z0-9_]+|MV_ALG_[A-Z0-9_]+)\s+(0x[0-9A-Fa-f]+|\d+)",
    )
    .expect("invalid status-code regex");

    let mut entries: Vec<(String, String)> = Vec::new();
    for source in sources {
        let text = fs::read_to_string(&source)
            .unwrap_or_else(|e| panic!("failed to read {}: {e}", source.display()));
        for line in text.lines() {
            if let Some(caps) = define_re.captures(line) {
                let name = caps.get(1).unwrap().as_str().to_owned();
                let value = caps.get(2).unwrap().as_str().to_owned();
                entries.push((name, value));
            }
        }
    }

    // MvErrorDefine.h re-includes MvISPErrorDefine.h
    let mut seen = std::collections::HashSet::new();
    entries.retain(|(name, _)| seen.insert(name.clone()));

    // Fail loud if SDK header format changes and the regex stops matching
    const MIN_EXPECTED_STATUS_CODES: usize = 200;
    assert!(
        entries.len() >= MIN_EXPECTED_STATUS_CODES,
        concat!(
            "SDK status-code parsing found {} entries, expected at least {}. ",
            "The vendored SDK may have changed, or build.rs no longer matches the header format."
        ),
        entries.len(),
        MIN_EXPECTED_STATUS_CODES,
    );

    let mut out = String::new();
    out.push_str("// @generated by build.rs. Do not edit by hand.\n");
    out.push_str("// Source: hikcamera-mvs package headers under the active pixi environment.\n");
    out.push_str("// Status-code constants use i32 to match the HikCamera C API return type.\n");
    out.push_str("// Hex error literals use <hex>_u32 as i32 to preserve bit patterns.\n");
    out.push_str("\n");

    for (name, value) in &entries {
        let rendered = if value.starts_with("0x") || value.starts_with("0X") {
            format!("{value}_u32 as i32")
        } else {
            value.clone()
        };
        out.push_str(&format!("pub const {name}: i32 = {rendered};\n"));
    }

    out
}
