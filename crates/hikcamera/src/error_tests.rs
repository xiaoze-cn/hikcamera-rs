use super::*;

#[test]
fn ok_code() {
    assert!(check(sys::MV_OK).is_ok());
}

#[test]
fn status_ok() {
    assert!(Status::OK.is_ok());
    assert!(Status(0).is_ok());
    assert!(!Status(sys::MV_E_NODATA).is_ok());
}

#[test]
fn status_display() {
    let s = Status(sys::MV_E_LOAD_LIBRARY);
    assert_eq!(s.to_string(), "MV_E_LOAD_LIBRARY (-2147483636, 0x8000000C)");
}

#[test]
fn sdk_error() {
    let status = Status(sys::MV_E_NODATA);
    let error = HikCameraError::Sdk { status };

    assert_eq!(status.info().name, "MV_E_NODATA");
    assert_eq!(status.info().message, "Timeout or no data received");
    assert_eq!(error.code(), Some(sys::MV_E_NODATA));
    assert_eq!(check(sys::MV_E_NODATA), Err(error));
}

#[test]
fn unknown_sdk_error() {
    let status = Status(123);
    let error = HikCameraError::Sdk { status };

    assert_eq!(status.info().name, "UNKNOWN");
    assert_eq!(status.info().message, "Unknown status code");
    assert_eq!(error.code(), Some(123));
}

#[test]
fn display_sdk_error() {
    let error = HikCameraError::Sdk {
        status: Status(sys::MV_E_LOAD_LIBRARY),
    };

    assert_eq!(
        error.to_string(),
        "MV_E_LOAD_LIBRARY (-2147483636, 0x8000000C): Failed to load dynamic library"
    );
}

#[test]
fn wrapper_error_code() {
    let error = HikCameraError::NoDevice;

    assert_eq!(error.code(), None);
}

#[test]
fn wrapper_error_display() {
    let error = HikCameraError::NodeInputMismatch {
        key: "ExposureTime".to_owned(),
        expected: "Float",
        actual: "EnumSymbol",
    };

    assert_eq!(error.code(), None);
    assert_eq!(
        error.to_string(),
        "node `ExposureTime` expects input `Float`, got `EnumSymbol`"
    );
}

#[test]
fn wrapper_error_display_details() {
    assert_eq!(
        HikCameraError::InvalidString { field: "path" }.to_string(),
        "path contains an interior NUL byte"
    );
    assert_eq!(
        HikCameraError::NullHandle.to_string(),
        "The HikCamera SDK returned a null handle"
    );
    assert_eq!(
        HikCameraError::SdkStatePoisoned.to_string(),
        "The HikCamera SDK reference counter is poisoned"
    );
    assert_eq!(
        HikCameraError::UnsupportedNode {
            key: "Root".to_owned(),
            kind: "Category",
        }
        .to_string(),
        "node `Root` has unsupported type `Category`"
    );
    assert_eq!(
        HikCameraError::NodeValueMismatch {
            expected: "Float",
            actual: "Int",
        }
        .to_string(),
        "expected node value `Float`, got `Int`"
    );
    assert_eq!(
        HikCameraError::ValueOutOfRange {
            field: "frame dimension",
        }
        .to_string(),
        "frame dimension is out of range"
    );
    assert_eq!(
        HikCameraError::RecordingInProgress.to_string(),
        "A video recording is already active on this stream"
    );
    assert_eq!(
        HikCameraError::EmptyFrame.to_string(),
        "Frame has no image data"
    );
    assert_eq!(
        HikCameraError::EmptyVideo.to_string(),
        "Video output has no frames"
    );
    assert_eq!(
        HikCameraError::InvalidDuration {
            field: "video duration",
        }
        .to_string(),
        "video duration must be greater than zero"
    );
    assert_eq!(
        HikCameraError::InvalidFrameRate {
            field: "video frame rate",
        }
        .to_string(),
        "video frame rate must be finite and greater than zero"
    );
    assert_eq!(
        HikCameraError::InvalidRoi.to_string(),
        "ROI width and height must be greater than zero"
    );
}
