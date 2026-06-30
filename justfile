set shell := ["sh", "-cu"]

root := justfile_directory()
host_os := os()
host_family := os_family()
windows_host := if host_family == "windows" { "true" } else { "false" }
path_sep := if windows_host == "true" { ";" } else { ":" }
pixi_bin := if windows_host == "true" { root + "\\.pixi\\envs\\default\\Library\\bin" } else { root + "/.pixi/envs/default/bin" }
libclang_dir := if windows_host == "true" { pixi_bin } else { root + "/.pixi/envs/default/lib" }
libclang_path := if windows_host == "true" { libclang_dir + "\\libclang.dll" } else if host_os == "macos" { libclang_dir + "/libclang.dylib" } else { libclang_dir + "/libclang.so" }
libclang_versioned_path := if windows_host == "true" { libclang_dir + "\\libclang-13.dll" } else { libclang_path }

export LIBCLANG_PATH := libclang_path
export PATH := pixi_bin + path_sep + libclang_dir + path_sep + env("PATH")
export LD_LIBRARY_PATH := libclang_dir + ":" + env("LD_LIBRARY_PATH", "")
export DYLD_LIBRARY_PATH := libclang_dir + ":" + env("DYLD_LIBRARY_PATH", "")

_default:
    just --list

check: setup
    cargo check --workspace

ra-check: ensure-libclang
    @cargo check --workspace --message-format=json --all-targets --keep-going --target-dir target/rust-analyzer

build: setup
    cargo build --workspace

build-examples: setup
    cargo build --workspace --examples

dll-deps: setup
    python scripts/check_dll_deps.py

dll-deps-md: setup
    python scripts/check_dll_deps.py --format markdown

test: setup
    cargo test --workspace

fmt-check:
    cargo fmt --all -- --check

pre-commit: fmt-check check

mod site

clean:
    cargo clean

setup:
    pixi install
    just ensure-libclang
    lefthook install

ensure-libclang:
    @if [ "{{ windows_host }}" = "true" ] && [ ! -f "{{ libclang_path }}" ] && [ -f "{{ libclang_versioned_path }}" ]; then cp "{{ libclang_versioned_path }}" "{{ libclang_path }}"; fi
    @test -f "{{ libclang_path }}" || { echo "error: libclang not found at {{ libclang_path }}; run 'pixi install' first" >&2; exit 1; }
