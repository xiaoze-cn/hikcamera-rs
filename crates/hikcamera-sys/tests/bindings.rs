//! Smoke tests for the generated public binding surface

#![allow(dead_code)]

use hikcamera_sys::{
    MV_ALG_ERR, MV_ALG_OK, MV_CC_ClearImageBuffer, MV_CC_CreateHandle, MV_CC_DestroyHandle,
    MV_CC_Finalize, MV_CC_FreeImageBuffer, MV_CC_GetImageBuffer, MV_CC_GetSDKVersion,
    MV_CC_Initialize, MV_CC_InputOneFrameEx, MV_CC_OpenDevice, MV_CC_SaveImageToFile,
    MV_CC_SetCommandValue, MV_CC_StartGrabbing, MV_CC_StartRecord, MV_CC_StopGrabbing,
    MV_CC_StopRecord, MV_E_HANDLE, MV_E_LOAD_LIBRARY, MV_E_NODATA, MV_FRAME_OUT, MV_OK,
    MvGvspPixelType,
};

#[test]
fn ffi_functions() {
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

#[test]
fn ffi_types() {
    let _: *const MV_FRAME_OUT = std::ptr::null();
    let _: MvGvspPixelType = MvGvspPixelType::default();
}

#[test]
fn status_codes() {
    let _: i32 = MV_OK;
    let _: i32 = MV_E_HANDLE;
    let _: i32 = MV_E_NODATA;
    let _: i32 = MV_E_LOAD_LIBRARY;
    let _: i32 = MV_ALG_OK;
    let _: i32 = MV_ALG_ERR;
}
