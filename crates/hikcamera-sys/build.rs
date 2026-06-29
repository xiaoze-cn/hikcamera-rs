use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    watch_env();

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let include_dir = manifest_dir.join("include");
    let header = manifest_dir.join("wrapper.h");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed={}", header.display());
    println!(
        "cargo:rerun-if-changed={}",
        include_dir.join("MvCameraControl.h").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        include_dir.join("CameraParams.h").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        include_dir.join("MvErrorDefine.h").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        include_dir.join("MvISPErrorDefine.h").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        include_dir.join("MvObsoleteInterfaces.h").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        include_dir.join("ObsoleteCamParams.h").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        include_dir.join("PixelType.h").display()
    );

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

    let include_args = clang_dirs()
        .into_iter()
        .filter(|path| path.exists())
        .map(|path| format!("-I{}", path.display()))
        .collect::<Vec<_>>();

    let bindings = bindgen::Builder::default()
        .header(header.to_string_lossy())
        .clang_arg(format!("-I{}", include_dir.display()))
        .clang_args(include_args)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to generate HikCamera MVS bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("failed to write HikCamera MVS bindings");

    copy_runtime(&runtime_dll, &out_dir);
}

fn watch_env() {
    for name in [
        "INCLUDE",
        "VCToolsInstallDir",
        "VCINSTALLDIR",
        "VSINSTALLDIR",
        "WindowsSdkDir",
        "WindowsSDKVersion",
    ] {
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
    copy_runtime_dir(&runtime_dir.join("ThirdParty"), profile_dir);
}

fn is_runtime_file(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "dll" | "cti" | "ini" | "ax"
            )
        })
}

fn clang_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Some(include) = env::var_os("INCLUDE") {
        dirs.extend(env::split_paths(&include));
    }

    if let Some(vctools_install_dir) = env::var_os("VCToolsInstallDir") {
        dirs.push(PathBuf::from(vctools_install_dir).join("include"));
    }

    if let Some(vc_install_dir) = env::var_os("VCINSTALLDIR") {
        let tools_dir = PathBuf::from(vc_install_dir).join("Tools").join("MSVC");
        if let Some(msvc_dir) = msvc_dir(&tools_dir) {
            dirs.push(msvc_dir.join("include"));
        }
    }

    for vs_install_dir in vs_dirs() {
        let tools_dir = vs_install_dir.join("VC").join("Tools").join("MSVC");
        if let Some(msvc_dir) = msvc_dir(&tools_dir) {
            dirs.push(msvc_dir.join("include"));
        }
    }

    dirs.extend(sdk_dirs());
    dirs.sort();
    dirs.dedup();
    dirs
}

fn vs_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Some(vs_install_dir) = env::var_os("VSINSTALLDIR") {
        dirs.push(PathBuf::from(vs_install_dir));
    }

    if let Some(vswhere) = vswhere_path() {
        if let Ok(output) = Command::new(vswhere)
            .args(["-latest", "-products", "*", "-property", "installationPath"])
            .output()
        {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_owned();
                let path = PathBuf::from(path);
                if path.join("VC").join("Tools").join("MSVC").exists() {
                    dirs.push(path);
                }
            }
        }
    }

    dirs
}

fn vswhere_path() -> Option<PathBuf> {
    if let Ok(output) = Command::new("vswhere").arg("-?").output() {
        if output.status.success() {
            return Some(PathBuf::from("vswhere"));
        }
    }

    ["ProgramFiles(x86)", "ProgramFiles"]
        .into_iter()
        .filter_map(env::var_os)
        .map(PathBuf::from)
        .map(|path| {
            path.join("Microsoft Visual Studio")
                .join("Installer")
                .join("vswhere.exe")
        })
        .find(|path| path.exists())
}

fn sdk_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Some(windows_sdk_dir) = env::var_os("WindowsSdkDir") {
        let sdk_dir = PathBuf::from(windows_sdk_dir).join("Include");
        if let Some(version) = env::var_os("WindowsSDKVersion") {
            dirs.extend(sdk_parts(sdk_dir.join(version)));
        } else if let Some(version_dir) = latest_dir(&sdk_dir) {
            dirs.extend(sdk_parts(version_dir));
        }
    }

    for root_var in ["ProgramFiles(x86)", "ProgramFiles"] {
        if let Some(root) = env::var_os(root_var) {
            let sdk_dir = PathBuf::from(root)
                .join("Windows Kits")
                .join("10")
                .join("Include");
            if let Some(version_dir) = latest_dir(&sdk_dir) {
                dirs.extend(sdk_parts(version_dir));
            }
        }
    }

    dirs
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

fn sdk_parts(version_dir: PathBuf) -> Vec<PathBuf> {
    ["ucrt", "shared", "um", "winrt"]
        .into_iter()
        .map(|name| version_dir.join(name))
        .collect()
}

fn latest_dir(parent: &Path) -> Option<PathBuf> {
    let mut children = std::fs::read_dir(parent)
        .ok()?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();

    children.sort();
    children.pop()
}

fn msvc_dir(parent: &Path) -> Option<PathBuf> {
    let mut children = std::fs::read_dir(parent)
        .ok()?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.join("include").join("stdint.h").exists())
        .collect::<Vec<_>>();

    children.sort();
    children.pop()
}
