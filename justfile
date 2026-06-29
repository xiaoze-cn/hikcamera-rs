set shell := ["sh", "-cu"]

root := justfile_directory()
libclang_dir := root + "\\.pixi\\envs\\default\\Library\\bin"
libclang_dll := libclang_dir + "\\libclang.dll"
libclang_versioned_dll := libclang_dir + "\\libclang-13.dll"

export LIBCLANG_PATH := libclang_dll
export PATH := libclang_dir + ";" + env_var("PATH")

_default:
    just --list

check: setup
    cargo check --workspace

build: setup
    cargo build --workspace

fmt-check:
    cargo fmt --all -- --check

pre-commit: fmt-check check

clean:
    cargo clean

setup:
    pixi install
    test -f "{{ libclang_dll }}" || cp "{{ libclang_versioned_dll }}" "{{ libclang_dll }}"
    lefthook install
