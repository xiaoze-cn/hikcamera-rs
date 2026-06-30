use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;

const HIK_CAMERA_LIB: &str = "MvCameraControl";

fn main() {
    println!("cargo:rerun-if-env-changed=LIBCLANG_PATH");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let include_dir = manifest_dir.join("include");
    let header = manifest_dir.join("wrapper.h");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed={}", header.display());
    println!("cargo:rerun-if-changed={}", include_dir.display());

    // Cargo sets CARGO_CFG_TARGET_OS / CARGO_CFG_TARGET_ARCH for build scripts
    // automatically; no need to pull in `target-lexicon` just to parse the
    // triple.
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS unset");
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH unset");

    let lib_subdir = match (target_os.as_str(), target_arch.as_str()) {
        ("windows", "x86_64") => "win64",
        ("windows", "x86") => "win32",
        _ => panic!("unsupported target for hikcamera-sys: {target_os}-{target_arch}"),
    };
    let lib_dir = manifest_dir.join("lib").join(lib_subdir);

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib={HIK_CAMERA_LIB}");

    // 1. Generate status-code constants ourselves. The SDK declares them as
    //    unsigned `0x80000000`-style literals, which bindgen cannot emit as
    //    `i32` (literal out of range). We re-encode each value as
    //    `0x80000000_u32 as i32` so the generated constant keeps its original
    //    hex form while having the same bit pattern as the SDK's `int` return.
    //    See `src/lib.rs` for the design rationale.
    let status_codes_out = out_dir.join("status_codes.rs");
    let status_codes_src = generate_status_codes(&include_dir);

    // 2. Blocklist the two status-code headers so bindgen doesn't double-
    //    emit their `#define`s. The path is regex-matched against what clang
    //    reports (which includes the `-I` prefix); `.*<filename>` is the most
    //    stable form across build environments.
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

    // 3. Stable-path review copies under `target/generated/` (OUT_DIR has a
    //    per-build hash suffix; humans need a stable path to diff across SDK
    //    upgrades).
    let generated_dir = target_dir(&manifest_dir).join("generated");
    write_generated(&generated_dir.join("bindings.rs"), bindings.to_string());
    write_generated(&generated_dir.join("status_codes.rs"), status_codes_src);
}

/// Resolve the workspace `target/` directory. Prefers the `CARGO_TARGET_DIR`
/// env var; falls back to `<crate>/../../target`.
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

/// Parse `MvErrorDefine.h` + `MvISPErrorDefine.h` and return a Rust source
/// string that re-declares every status-code macro (`MV_OK`, `MV_E_*`,
/// `MV_ALG_*`) as an `i32` constant, matching the SDK's `c_int` return type.
///
/// Each `0x80000000`-style value is rewritten as `<hex>_u32 as i32` so the
/// original hex form is preserved in the source while the Rust literal stays
/// in range. The C compiler does the equivalent implicit truncation when the
/// SDK compares `int` against these macros.
fn generate_status_codes(include_dir: &Path) -> String {
    let sources = [
        include_dir.join("MvErrorDefine.h"),
        include_dir.join("MvISPErrorDefine.h"),
    ];

    // Status-code family prefixes (MV_OK, MV_E_*, MV_ALG_*). Other MV_*
    // macros (MV_ACCESS_*, pixel formats, ...) are enum-parameter values
    // that bindgen emits as `u32` to match unsigned SDK function parameters.
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

    // Dedup: MvErrorDefine.h re-includes MvISPErrorDefine.h. Keep first
    // occurrence so the MV_ALG_* constants stay grouped with their source.
    let mut seen = std::collections::HashSet::new();
    entries.retain(|(name, _)| seen.insert(name.clone()));

    // Cheap SDK-format drift detector: if the regex stops matching (header
    // changed style, new prefix family we don't recognise, regex itself
    // broken), the entry count collapses. Fail loud here so the wrapper
    // doesn't silently ship with a partial status-code table. The threshold
    // is intentionally loose — bump it after a real SDK upgrade that grows
    // the table.
    const MIN_EXPECTED_STATUS_CODES: usize = 200;
    assert!(
        entries.len() >= MIN_EXPECTED_STATUS_CODES,
        "found only {} SDK status codes across MvErrorDefine.h + MvISPErrorDefine.h; \
         expected at least {}. Either the vendored SDK shrank dramatically or \
         the parsing regex in build.rs no longer matches the header format.",
        entries.len(),
        MIN_EXPECTED_STATUS_CODES,
    );

    let mut out = String::new();
    out.push_str("// @generated by `build.rs`. Do not edit by hand.\n");
    out.push_str("//\n");
    out.push_str("// HikCamera MVS status codes as `i32` constants, matching the\n");
    out.push_str("// SDK's `c_int` return type. Parsed from:\n");
    out.push_str("//   - include/MvErrorDefine.h\n");
    out.push_str("//   - include/MvISPErrorDefine.h\n");
    out.push_str("//\n");
    out.push_str("// Each unsigned `0x80000000`-style literal from the C header is\n");
    out.push_str("// re-encoded as `<hex>_u32 as i32` so the bit pattern matches the\n");
    out.push_str("// SDK's `int` return value while keeping the hex form readable.\n");
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
