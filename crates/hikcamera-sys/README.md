# hikcamera-sys

Low-level, `bindgen`-generated FFI bindings for the [HikCamera MVS](https://www.hikrobotics.com/) industrial camera C SDK.

This crate is an implementation detail of the [`hikcamera`](https://docs.rs/hikcamera) safe wrapper. End users should prefer `hikcamera` over these raw bindings.

## Platform support

| OS | Arch | Status |
|---|---|---|
| Windows | x86_64 | ✅ Supported with SDK files from the `hikcamera-mvs` conda package |
| Windows | x86 | ⚠️ SDK files are staged, but the workspace currently targets x86_64 |
| Linux | x86_64 | 🚧 Future target, not packaged by this crate yet |
| macOS | any | ❌ Unsupported by the HikCamera MVS SDK |

## SDK files

The HikCamera MVS headers, import libraries, and runtime DLLs are provided by the `hikcamera-mvs` conda package.
pixi installs that package into `.pixi/envs/default`, and `build.rs` reads headers and import libraries from the active pixi environment.

## License

The Rust bindings in this crate are MIT-licensed. The HikCamera MVS SDK files remain under the HikCamera MVS SDK license, `LicenseRef-HikCamera-MVS`.
