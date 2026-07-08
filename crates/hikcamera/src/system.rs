use std::sync::Mutex;

use crate::{Devices, HikCameraError, Result, error::check, sys};

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
