# AGENTS.md

Guide for AI coding agents (and humans pairing with them) working on this repo.

## Project overview

`hikcamera-rs` is a safe Rust wrapper around the HikCamera MVS (industrial
camera) C SDK. It is Windows-only and links against the vendor DLLs shipped
under `crates/hikcamera-sys/lib/`.

The workspace has two crates:

| Crate | Role |
|-------|------|
| `hikcamera-sys` | Raw `bindgen` bindings against the C headers in `crates/hikcamera-sys/include/`. Re-exports as `hikcamera::sys`. |
| `hikcamera` | Safe high-level wrapper. Public API lives in `crates/hikcamera/src/`. |

Key source files in `crates/hikcamera/src/`:

- `lib.rs` — crate root, public re-exports
- `error.rs` — `HikCameraError` enum, `Status` newtype, SDK status-code → `StatusInfo` table
- `system.rs` — `HikCamera` SDK lifecycle (init/finalize refcount)
- `device.rs` — `Device` / `Devices` / `DeviceInfo` enumeration
- `camera.rs` — `Camera`, `Stream`, `Frame`, node map accessors, image/video writers

## Environment setup (do this once)

This repo needs `libclang` for `bindgen` and MSVC toolchain for linking. Both
are pulled in automatically via `pixi`:

```sh
just setup     # runs `pixi install`, copies libclang.dll, installs lefthook hooks
```

`just` itself sets `LIBCLANG_PATH` and prepends the pixi env to `PATH` for
every recipe — **always invoke cargo through `just`, not directly**. Running
`cargo` bare will fail with `Unable to find libclang`.

## Day-to-day commands

```sh
just check          # cargo check --workspace
just build          # cargo build --workspace
just test           # cargo test --workspace
just fmt-check      # rustfmt check (run before committing)
just pre-commit     # fmt-check + check (what lefthook runs)
just clean          # cargo clean
```

For anything not covered by a recipe, prefix the cargo invocation with the
`just` shell environment by running through `just` indirectly, or replicate
the env vars from the `justfile` header (`LIBCLANG_PATH`, `PATH`). When
adding a new common workflow, prefer adding a `just` recipe over documenting
a raw cargo command.

`just` recipes also work for ad-hoc single-test runs, e.g.:

```sh
just test --lib error     # cargo test --workspace --lib error
```

(extra args after the recipe name are forwarded to `cargo`).

## Conventions

### Language

- **All comments, doc comments, commit messages, and code-facing strings must
  be in English.** Chinese is acceptable only inside `docs/` (user-facing
  documentation that mirrors the upstream C SDK style) and in this `AGENTS.md`
  if needed for context.
- Code identifiers use American English spelling to match Rust style
  (`color`, not `colour`).

### Error handling

See `docs/rust-sdk/error.md` for the design rationale. Short version for
agents editing `error.rs`:

- SDK status codes flow through the `Status(i32)` newtype — never as raw
  `i32` in signatures.
- `check(code: i32) -> Result<()>` is the only entry point for converting an
  SDK return value into a `Result`.
- `HikCameraError::Sdk` is the only variant that carries an SDK code; wrapper
  errors are discriminated by enum variant.
- The SDK status → `StatusInfo` lookup uses a `const` slice table, not a
  giant `match`. **Add new SDK error codes by appending to the table**, not
  by writing more match arms.
- `Display` is derived via `thiserror`; avoid maintaining a parallel
  wrapper-level error info/message table.

### Dependencies

- Prefer `thiserror` for error enums.
- `derive_more` is welcome for trivial newtype / Deref / From / Display
  boilerplate **when the generated API is actually used**. Don't add a
  `derive` whose generated method/impl has no call site — that's just surface
  area. Before adding `#[derive(derive_more::X)]`, point to where the derived
  item gets called.
- `strum` is welcome for enums that benefit from `EnumIter`, `IntoStaticStr`,
  `VariantArray`, etc.
- Do **not** add a new dependency for a one-off `derive` that saves fewer than
  ~5 lines — judge each case.
- Don't `derive` constructors (`Constructor`, `new`) on types whose semantics
  are "internal lookup result" or "opaque handle" — exposing construction lets
  callers forge values that should only come from the wrapper.

### Formatting and style

- `rustfmt` with the project's default config (no custom `rustfmt.toml`).
  Run `just fmt-check` before declaring a change done.
- No `unsafe` in `crates/hikcamera/src/` outside of FFI calls into
  `crate::sys::*` — and those should already be wrapped by `check(...)` or a
  higher-level helper. New `unsafe` blocks need a `// SAFETY:` comment
  explaining the invariant.
- Public exports go through `lib.rs`. Avoid `pub use` deep in modules unless
  re-exporting from `lib.rs` would create a cycle.

## Things to watch out for

- **`bindgen` rebuild is slow** (parsing the full MSVC + SDK headers). Don't
  touch `crates/hikcamera-sys/wrapper.h`, `include/*.h`, or `build.rs` unless
  the task is specifically about FFI.
- **The `MV_*` constants are generated as `u32` by bindgen** (because the C
  `#define`s are `0x80000000`-style literals). The C SDK functions return
  `int` (signed). The conversion boundary is intentionally consolidated in
  `error::check` / `Status::OK` — don't sprinkle `as i32` / `as u32` casts
  elsewhere.
- **Examples in `crates/hikcamera/examples/` require a real camera** to run.
  Don't try to execute them in CI-like environments; `cargo build --examples`
  is enough to validate they compile.
- **No git operations unprompted.** Don't commit, branch, or push unless the
  user asks. Lefthook will run `fmt-check` + `check` on commit anyway.

## Where to look for context

- `docs/rust-sdk/*.md` — design notes for the safe wrapper, by module
- `docs/c-sdk/` — annotated C SDK reference
- `crates/hikcamera-sys/include/` — original HikCamera MVS headers (the
  source of truth for behavior, parameter ranges, and error codes)
