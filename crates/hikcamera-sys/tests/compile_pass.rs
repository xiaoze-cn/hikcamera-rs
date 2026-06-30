//! Smoke test: ensure the generated bindings still reference the SDK symbols
//! the safe wrapper depends on.
//!
//! If a future `bindgen` upgrade silently renames a symbol (e.g. by changing
//! mangling or adding a version suffix), the `hikcamera` crate would fail to
//! compile with a confusing `unresolved import` error. This test makes that
//! failure local to `hikcamera-sys` and names the missing symbol directly.
//!
//! The same goes for the status-code constants emitted by `build.rs`'s
//! `generate_status_codes()` — if the regex ever stops matching, this test
//! catches it here instead of in the wrapper. The `let _: i32 = ...` bindings
//! also assert that those constants have the **correct type** (`i32`); if
//! `build.rs` regresses and they come out as `u32`, this file fails to
//! compile.
//!
//! The lists below mirror the entry points and status codes the safe wrapper
//! actually uses; update them when the wrapper starts/stops using an SDK
//! symbol. Note: this test does **not** link or call the functions — it only
//! asserts the symbols exist as FFI items, so it runs on any host without the
//! vendor DLL being loadable.

#![allow(dead_code)]

use hikcamera_sys::{
    MV_ALG_ERR, MV_ALG_OK, MV_CC_ClearImageBuffer, MV_CC_CreateHandle, MV_CC_DestroyHandle,
    MV_CC_Finalize, MV_CC_FreeImageBuffer, MV_CC_GetImageBuffer, MV_CC_GetSDKVersion,
    MV_CC_Initialize, MV_CC_InputOneFrameEx, MV_CC_OpenDevice, MV_CC_SaveImageToFile,
    MV_CC_SetCommandValue, MV_CC_StartGrabbing, MV_CC_StartRecord, MV_CC_StopGrabbing,
    MV_CC_StopRecord, MV_E_HANDLE, MV_E_LOAD_LIBRARY, MV_E_NODATA, MV_FRAME_OUT, MV_OK,
    MvGvspPixelType,
};

// Touch each function symbol as a raw pointer to force name resolution
// without generating a call site (which would require linking the DLL).
#[test]
fn ffi_function_symbols_resolve() {
    let _ = MV_CC_Initialize as *const ();
    let _ = MV_CC_Finalize as *const ();
    let _ = MV_CC_GetSDKVersion as *const ();
    let _ = MV_CC_CreateHandle as *const ();
    let _ = MV_CC_OpenDevice as *const ();
    let _ = MV_CC_DestroyHandle as *const ();
    let _ = MV_CC_StartGrabbing as *const ();
    let _ = MV_CC_StopGrabbing as *const ();
    let _ = MV_CC_GetImageBuffer as *const ();
    let _ = MV_CC_FreeImageBuffer as *const ();
    let _ = MV_CC_ClearImageBuffer as *const ();
    let _ = MV_CC_SetCommandValue as *const ();
    let _ = MV_CC_SaveImageToFile as *const ();
    let _ = MV_CC_StartRecord as *const ();
    let _ = MV_CC_StopRecord as *const ();
    let _ = MV_CC_InputOneFrameEx as *const ();
}

// Touch key types via null pointers — proves the type names still resolve.
#[test]
fn ffi_type_symbols_resolve() {
    let _: *const MV_FRAME_OUT = std::ptr::null();
    let _: MvGvspPixelType = MvGvspPixelType::default();
}

// Status-code constants must exist *and* be `i32`. The `let _: i32 = ...`
// bindings fail to compile if `build.rs` regresses and emits them as `u32`
// (which would silently break every comparison in the safe wrapper).
#[test]
fn status_code_constants_are_i32() {
    let _: i32 = MV_OK;
    let _: i32 = MV_E_HANDLE;
    let _: i32 = MV_E_NODATA;
    let _: i32 = MV_E_LOAD_LIBRARY;
    let _: i32 = MV_ALG_OK;
    let _: i32 = MV_ALG_ERR;
}
