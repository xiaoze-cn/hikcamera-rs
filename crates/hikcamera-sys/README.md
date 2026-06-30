# hikcamera-sys

Low-level, `bindgen`-generated FFI bindings for the
[HikCamera MVS](https://www.hikrobotics.com/) industrial camera C SDK.

This crate is intended as an implementation detail of the
[`hikcamera`](https://docs.rs/hikcamera) safe wrapper and is Windows-only.
End users should prefer `hikcamera` over using these raw bindings directly.

## Platform support

| OS | Arch | Status |
|---|---|---|
| Windows | x86_64 | ✅ Supported (vendored import lib + conda runtime DLL) |
| Windows | x86 | ⚠️ Bindings compile; runtime DLL not packaged |
| Linux / macOS | any | ❌ Unsupported by upstream SDK |

## What's vendored

- `include/` — original HikCamera MVS C headers (`MvCameraControl.h` and friends).
- `lib/win{32,64}/MvCameraControl.lib` — MSVC import libraries.

The matching runtime DLLs (`MvCameraControl.dll` and its dependencies) are
distributed via the `hikcamera-mvs-runtime` conda package; see
[`conda-packages/`](../conda-packages) in the workspace root.

## License

The Rust bindings in this crate are MIT-licensed. The vendored HikCamera MVS
headers and import library remain under the HikCamera MVS SDK license — see
`LicenseRef-HikCamera-MVS`.
