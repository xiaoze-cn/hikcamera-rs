use std::ptr::{self, NonNull};
use std::sync::Mutex;

use crate::{Camera, Device, Devices, HikCameraError, Result, error::check, sys};

static SDK_REF_COUNT: Mutex<usize> = Mutex::new(0);

#[derive(Debug)]
pub struct HikCamera {
    _private: (),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HikVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub build: u8,
    pub raw: u32,
}

impl HikCamera {
    pub fn new() -> Result<Self> {
        let mut users = SDK_REF_COUNT
            .lock()
            .map_err(|_| HikCameraError::SdkStatePoisoned)?;

        if *users == 0 {
            check(unsafe { sys::MV_CC_Initialize() })?;
        }

        *users += 1;
        Ok(Self { _private: () })
    }

    pub fn version(&self) -> HikVersion {
        HikVersion::current()
    }

    pub fn devices(&self) -> Result<Devices<'_>> {
        Devices::list(self)
    }

    pub(crate) fn open_device<'hik>(&'hik self, device: &Device<'hik>) -> Result<Camera<'hik>> {
        let mut handle = ptr::null_mut();
        check(unsafe { sys::MV_CC_CreateHandle(&mut handle, device.raw()) })?;

        let Some(handle) = NonNull::new(handle.cast()) else {
            return Err(HikCameraError::NullHandle);
        };

        if let Err(error) =
            check(unsafe { sys::MV_CC_OpenDevice(handle.as_ptr(), sys::MV_ACCESS_Exclusive, 0) })
        {
            unsafe {
                sys::MV_CC_DestroyHandle(handle.as_ptr());
            }
            return Err(error);
        }

        Ok(Camera::from_handle(handle))
    }
}

impl Drop for HikCamera {
    fn drop(&mut self) {
        let Ok(mut users) = SDK_REF_COUNT.lock() else {
            return;
        };

        *users = users.saturating_sub(1);
        if *users == 0 {
            unsafe {
                let _ = sys::MV_CC_Finalize();
            }
        }
    }
}

impl HikVersion {
    pub fn current() -> Self {
        Self::from_raw(unsafe { sys::MV_CC_GetSDKVersion() })
    }

    fn from_raw(raw: u32) -> Self {
        Self {
            major: ((raw >> 24) & 0xFF) as u8,
            minor: ((raw >> 16) & 0xFF) as u8,
            patch: ((raw >> 8) & 0xFF) as u8,
            build: (raw & 0xFF) as u8,
            raw,
        }
    }
}
