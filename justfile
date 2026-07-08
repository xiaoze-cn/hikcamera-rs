set shell := ["sh", "-cu"]

root := justfile_directory()

host_os := os()
host_family := os_family()
system := host_os

separator := if system == "windows" { ";" } else { ":" }
joiner := if system == "windows" { "\\" } else { "/" }

pixi_env := root + "/.pixi/envs/default"
pixi_bin := if system == "windows" { root + "\\.pixi\\envs\\default\\Library\\bin" } else { pixi_env + "/bin" }

libclang_dir := if system == "windows" { pixi_bin } else { pixi_env + "/lib" }
libclang_file := if system == "windows" { "libclang.dll" } else if system == "macos" { "libclang.dylib" } else { "libclang.so" }
libclang_path := libclang_dir + joiner + libclang_file

export LIBCLANG_PATH := libclang_path
export PATH := pixi_bin + separator + libclang_dir + separator + env("PATH")
export LD_LIBRARY_PATH := libclang_dir + ":" + env("LD_LIBRARY_PATH", "")
export DYLD_LIBRARY_PATH := libclang_dir + ":" + env("DYLD_LIBRARY_PATH", "")

_default:
    @just --unsorted --list

[group('Setup')]
setup:
    pixi install
    just _libclang
    lefthook install

[group('Rust')]
check: _libclang
    cargo check --workspace

[group('Rust')]
build: _libclang
    cargo build --workspace

[group('Rust')]
test: _libclang
    cargo test --workspace

[group('Rust')]
clean:
    cargo clean

[group('Examples')]
build-examples: _libclang
    cargo build --workspace --examples

[group('Environment')]
env-check: _libclang
    python scripts/check_env.py runtime

[group('Verify')]
fmt-check:
    cargo fmt --all -- --check

[group('Verify')]
pre-commit: fmt-check check

[group('Verify')]
ra-check: _libclang
    @cargo check --workspace --message-format=json --all-targets --keep-going --target-dir target/rust-analyzer

[group: 'Site']
mod site

_libclang:
    @if [ "{{ system }}" = "windows" ] && [ ! -f "{{ libclang_path }}" ] && [ -f "{{ libclang_dir }}{{ joiner }}libclang-13.dll" ]; then cp "{{ libclang_dir }}{{ joiner }}libclang-13.dll" "{{ libclang_path }}"; fi
    @test -f "{{ libclang_path }}" || { echo "error: libclang not found at {{ libclang_path }}; run 'pixi install' first" >&2; exit 1; }
