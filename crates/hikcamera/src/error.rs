use crate::sys;

use derive_more::From;
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, HikCameraError>;

pub type Error = HikCameraError;

/// Raw status code returned by the HikCamera MV SDK.
///
/// The C API returns `int`. `MV_OK` (0) means success; failures are
/// `0x80000000`-style literals that read as negative values when stored in
/// a signed `i32`. This newtype keeps raw SDK status codes visually and
/// type-distinct from arbitrary `i32` values flowing through the wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, From)]
pub struct Status(pub i32);

impl Status {
    /// SDK success status (`MV_OK`).
    pub const OK: Self = Self(sys::MV_OK as i32);

    #[inline]
    pub const fn raw(self) -> i32 {
        self.0
    }

    /// Reinterpret as `u32` (useful for `0x80000000`-style hex formatting).
    #[inline]
    pub const fn as_u32(self) -> u32 {
        self.0 as u32
    }

    #[inline]
    pub const fn is_ok(self) -> bool {
        self.0 == Self::OK.0
    }

    pub fn info(self) -> StatusInfo {
        sdk_error_info(self)
    }

    pub(crate) fn into_result(self) -> Result<()> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(HikCameraError::Sdk { status: self })
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let info = sdk_error_info(*self);
        write!(f, "{} ({}, {:#010X})", info.name, self.0, self.as_u32())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, ThisError)]
#[non_exhaustive]
pub enum HikCameraError {
    #[error("{status}: {}", status.info().message)]
    Sdk { status: Status },
    #[error("No HikCamera device was found")]
    NoDevice,
    #[error("no HikCamera device matched selector `{selector}`")]
    DeviceNotFound { selector: String },
    #[error("{count} HikCamera devices matched selector `{selector}`")]
    MultipleDevices { selector: String, count: usize },
    #[error("The HikCamera SDK returned a null handle")]
    NullHandle,
    #[error("The HikCamera SDK reference counter is poisoned")]
    SdkStatePoisoned,
    #[error("{field} contains an interior NUL byte")]
    InvalidString { field: &'static str },
    #[error("node `{key}` has unsupported type `{kind}`")]
    UnsupportedNode { key: String, kind: &'static str },
    #[error("expected node value `{expected}`, got `{actual}`")]
    NodeValueMismatch {
        expected: &'static str,
        actual: &'static str,
    },
    #[error("node `{key}` expects input `{expected}`, got `{actual}`")]
    NodeInputMismatch {
        key: String,
        expected: &'static str,
        actual: &'static str,
    },
    #[error("{field} is out of range")]
    ValueOutOfRange { field: &'static str },
    #[error("A video recording is already active on this stream")]
    RecordingInProgress,
    #[error("Frame has no image data")]
    EmptyFrame,
    #[error("Video output has no frames")]
    EmptyVideo,
    #[error("{field} must be greater than zero")]
    InvalidDuration { field: &'static str },
    #[error("{field} must be finite and greater than zero")]
    InvalidFrameRate { field: &'static str },
    #[error("ROI width and height must be greater than zero")]
    InvalidRoi,
}

/// Stable, human-readable description of a raw SDK [`Status`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusInfo {
    /// Stable SDK identifier, e.g. `"MV_E_NODATA"`.
    pub name: &'static str,
    /// Short human-readable SDK status description.
    pub message: &'static str,
}

pub(crate) fn check(code: i32) -> Result<()> {
    Status::from(code).into_result()
}

impl HikCameraError {
    /// Raw SDK status code. Returns `Some` only for [`HikCameraError::Sdk`];
    /// wrapper-level errors return `None`.
    pub fn code(&self) -> Option<i32> {
        match self {
            Self::Sdk { status } => Some(status.raw()),
            _ => None,
        }
    }
}

/// Lookup table mapping SDK status codes to `StatusInfo` (linear scan;
/// error paths are not hot, so no hashing required).
const SDK_ERRORS: &[(i32, StatusInfo)] = &[
    (
        sys::MV_OK as i32,
        StatusInfo {
            name: "MV_OK",
            message: "Success",
        },
    ),
    (
        sys::MV_E_HANDLE as i32,
        StatusInfo {
            name: "MV_E_HANDLE",
            message: "Invalid handle",
        },
    ),
    (
        sys::MV_E_SUPPORT as i32,
        StatusInfo {
            name: "MV_E_SUPPORT",
            message: "Unsupported function",
        },
    ),
    (
        sys::MV_E_BUFOVER as i32,
        StatusInfo {
            name: "MV_E_BUFOVER",
            message: "Buffer overflow",
        },
    ),
    (
        sys::MV_E_CALLORDER as i32,
        StatusInfo {
            name: "MV_E_CALLORDER",
            message: "Invalid function call order",
        },
    ),
    (
        sys::MV_E_PARAMETER as i32,
        StatusInfo {
            name: "MV_E_PARAMETER",
            message: "Invalid parameter",
        },
    ),
    (
        sys::MV_E_RESOURCE as i32,
        StatusInfo {
            name: "MV_E_RESOURCE",
            message: "Resource allocation failed",
        },
    ),
    (
        sys::MV_E_NODATA as i32,
        StatusInfo {
            name: "MV_E_NODATA",
            message: "Timeout or no data received",
        },
    ),
    (
        sys::MV_E_PRECONDITION as i32,
        StatusInfo {
            name: "MV_E_PRECONDITION",
            message: "Precondition failed or runtime environment changed",
        },
    ),
    (
        sys::MV_E_VERSION as i32,
        StatusInfo {
            name: "MV_E_VERSION",
            message: "Version mismatch",
        },
    ),
    (
        sys::MV_E_NOENOUGH_BUF as i32,
        StatusInfo {
            name: "MV_E_NOENOUGH_BUF",
            message: "Input buffer is too small",
        },
    ),
    (
        sys::MV_E_ABNORMAL_IMAGE as i32,
        StatusInfo {
            name: "MV_E_ABNORMAL_IMAGE",
            message: "Abnormal or incomplete image",
        },
    ),
    (
        sys::MV_E_LOAD_LIBRARY as i32,
        StatusInfo {
            name: "MV_E_LOAD_LIBRARY",
            message: "Failed to load dynamic library",
        },
    ),
    (
        sys::MV_E_NOOUTBUF as i32,
        StatusInfo {
            name: "MV_E_NOOUTBUF",
            message: "No output buffer available",
        },
    ),
    (
        sys::MV_E_ENCRYPT as i32,
        StatusInfo {
            name: "MV_E_ENCRYPT",
            message: "Encryption error",
        },
    ),
    (
        sys::MV_E_OPENFILE as i32,
        StatusInfo {
            name: "MV_E_OPENFILE",
            message: "Failed to open file",
        },
    ),
    (
        sys::MV_E_BUF_IN_USE as i32,
        StatusInfo {
            name: "MV_E_BUF_IN_USE",
            message: "Buffer is already in use",
        },
    ),
    (
        sys::MV_E_BUF_INVALID as i32,
        StatusInfo {
            name: "MV_E_BUF_INVALID",
            message: "Invalid buffer address",
        },
    ),
    (
        sys::MV_E_NOALIGN_BUF as i32,
        StatusInfo {
            name: "MV_E_NOALIGN_BUF",
            message: "Buffer alignment error",
        },
    ),
    (
        sys::MV_E_NOENOUGH_BUF_NUM as i32,
        StatusInfo {
            name: "MV_E_NOENOUGH_BUF_NUM",
            message: "Insufficient buffer count",
        },
    ),
    (
        sys::MV_E_PORT_IN_USE as i32,
        StatusInfo {
            name: "MV_E_PORT_IN_USE",
            message: "Port is in use",
        },
    ),
    (
        sys::MV_E_IMAGE_DECODEC as i32,
        StatusInfo {
            name: "MV_E_IMAGE_DECODEC",
            message: "Image decoding error",
        },
    ),
    (
        sys::MV_E_UINT32_LIMIT as i32,
        StatusInfo {
            name: "MV_E_UINT32_LIMIT",
            message: "Image size exceeds unsigned int limit",
        },
    ),
    (
        sys::MV_E_IMAGE_HEIGHT as i32,
        StatusInfo {
            name: "MV_E_IMAGE_HEIGHT",
            message: "Abnormal image height",
        },
    ),
    (
        sys::MV_E_NOENOUGH_DDR as i32,
        StatusInfo {
            name: "MV_E_NOENOUGH_DDR",
            message: "Insufficient frame grabber DDR buffer",
        },
    ),
    (
        sys::MV_E_NOENOUGH_STREAM as i32,
        StatusInfo {
            name: "MV_E_NOENOUGH_STREAM",
            message: "Insufficient frame grabber stream channels",
        },
    ),
    (
        sys::MV_E_NORESPONSE as i32,
        StatusInfo {
            name: "MV_E_NORESPONSE",
            message: "Device does not respond",
        },
    ),
    (
        sys::MV_E_WRITEFILE as i32,
        StatusInfo {
            name: "MV_E_WRITEFILE",
            message: "Failed to write file",
        },
    ),
    (
        sys::MV_E_READFILE as i32,
        StatusInfo {
            name: "MV_E_READFILE",
            message: "Failed to read file",
        },
    ),
    (
        sys::MV_E_FILELENGTH as i32,
        StatusInfo {
            name: "MV_E_FILELENGTH",
            message: "Invalid file length",
        },
    ),
    (
        sys::MV_E_RESOURCE_EVENT as i32,
        StatusInfo {
            name: "MV_E_RESOURCE_EVENT",
            message: "Failed to create event resource",
        },
    ),
    (
        sys::MV_E_RESOURCE_THREAD as i32,
        StatusInfo {
            name: "MV_E_RESOURCE_THREAD",
            message: "Failed to create thread resource",
        },
    ),
    (
        sys::MV_E_DEV_OFFLINE as i32,
        StatusInfo {
            name: "MV_E_DEV_OFFLINE",
            message: "Device is offline",
        },
    ),
    (
        sys::MV_E_DEV_SUPPORT as i32,
        StatusInfo {
            name: "MV_E_DEV_SUPPORT",
            message: "Device is not supported",
        },
    ),
    (
        sys::MV_E_PLATFORM_SUPPORT as i32,
        StatusInfo {
            name: "MV_E_PLATFORM_SUPPORT",
            message: "Platform is not implemented",
        },
    ),
    (
        sys::MV_E_SERIAL_BUFFER_FULL as i32,
        StatusInfo {
            name: "MV_E_SERIAL_BUFFER_FULL",
            message: "Device serial buffer is full",
        },
    ),
    (
        sys::MV_E_CHANNEL_INDEX as i32,
        StatusInfo {
            name: "MV_E_CHANNEL_INDEX",
            message: "Invalid stream channel index",
        },
    ),
    (
        sys::MV_E_PARAMETER_RANGE as i32,
        StatusInfo {
            name: "MV_E_PARAMETER_RANGE",
            message: "Parameter is out of range",
        },
    ),
    (
        sys::MV_E_RESOURCE_IO as i32,
        StatusInfo {
            name: "MV_E_RESOURCE_IO",
            message: "IO resource error",
        },
    ),
    (
        sys::MV_E_IMAGE_INFO_INVALID as i32,
        StatusInfo {
            name: "MV_E_IMAGE_INFO_INVALID",
            message: "Invalid image information",
        },
    ),
    (
        sys::MV_E_RESOURCE_IN_USE as i32,
        StatusInfo {
            name: "MV_E_RESOURCE_IN_USE",
            message: "Requested resource is in use",
        },
    ),
    (
        sys::MV_E_DEV_NOT_IMPLEMENTED as i32,
        StatusInfo {
            name: "MV_E_DEV_NOT_IMPLEMENTED",
            message: "Command is not implemented by device",
        },
    ),
    (
        sys::MV_E_DEV_INVALID_PARAMETER as i32,
        StatusInfo {
            name: "MV_E_DEV_INVALID_PARAMETER",
            message: "Device command parameter is invalid or out of range",
        },
    ),
    (
        sys::MV_E_DEV_INVALID_ADDRESS as i32,
        StatusInfo {
            name: "MV_E_DEV_INVALID_ADDRESS",
            message: "Register address does not exist",
        },
    ),
    (
        sys::MV_E_DEV_WRITE_PROTECT as i32,
        StatusInfo {
            name: "MV_E_DEV_WRITE_PROTECT",
            message: "Attempted to write a read-only register",
        },
    ),
    (
        sys::MV_E_DEV_BAD_ALIGNMENT as i32,
        StatusInfo {
            name: "MV_E_DEV_BAD_ALIGNMENT",
            message: "Register address alignment error",
        },
    ),
    (
        sys::MV_E_DEV_ACCESS_DENIED as i32,
        StatusInfo {
            name: "MV_E_DEV_ACCESS_DENIED",
            message: "Register access denied",
        },
    ),
    (
        sys::MV_E_DEV_BUSY as i32,
        StatusInfo {
            name: "MV_E_DEV_BUSY",
            message: "Device is busy",
        },
    ),
    (
        sys::MV_E_DEV_MSG_TIMEOUT as i32,
        StatusInfo {
            name: "MV_E_DEV_MSG_TIMEOUT",
            message: "Device response timeout",
        },
    ),
    (
        sys::MV_E_DEV_INVALID_HEADER as i32,
        StatusInfo {
            name: "MV_E_DEV_INVALID_HEADER",
            message: "Invalid command header received",
        },
    ),
    (
        sys::MV_E_DEV_UNKNOWN as i32,
        StatusInfo {
            name: "MV_E_DEV_UNKNOWN",
            message: "Unknown error returned by device",
        },
    ),
    (
        sys::MV_E_DEV_INVALID_PARAMS as i32,
        StatusInfo {
            name: "MV_E_DEV_INVALID_PARAMS",
            message: "Device returned invalid parameters",
        },
    ),
    (
        sys::MV_E_DEV_WRONG_CONFIG as i32,
        StatusInfo {
            name: "MV_E_DEV_WRONG_CONFIG",
            message: "Device configuration does not allow this command",
        },
    ),
    (
        sys::MV_E_DEV_CRC as i32,
        StatusInfo {
            name: "MV_E_DEV_CRC",
            message: "CRC error",
        },
    ),
    (
        sys::MV_E_INTERNAL as i32,
        StatusInfo {
            name: "MV_E_INTERNAL",
            message: "SDK internal error",
        },
    ),
    (
        sys::MV_E_UNKNOW as i32,
        StatusInfo {
            name: "MV_E_UNKNOW",
            message: "Unknown error",
        },
    ),
    (
        sys::MV_E_GC_GENERIC as i32,
        StatusInfo {
            name: "MV_E_GC_GENERIC",
            message: "GenICam general error",
        },
    ),
    (
        sys::MV_E_GC_ARGUMENT as i32,
        StatusInfo {
            name: "MV_E_GC_ARGUMENT",
            message: "GenICam invalid argument",
        },
    ),
    (
        sys::MV_E_GC_RANGE as i32,
        StatusInfo {
            name: "MV_E_GC_RANGE",
            message: "GenICam value out of range",
        },
    ),
    (
        sys::MV_E_GC_PROPERTY as i32,
        StatusInfo {
            name: "MV_E_GC_PROPERTY",
            message: "GenICam property error",
        },
    ),
    (
        sys::MV_E_GC_RUNTIME as i32,
        StatusInfo {
            name: "MV_E_GC_RUNTIME",
            message: "GenICam runtime error",
        },
    ),
    (
        sys::MV_E_GC_LOGICAL as i32,
        StatusInfo {
            name: "MV_E_GC_LOGICAL",
            message: "GenICam logical error",
        },
    ),
    (
        sys::MV_E_GC_ACCESS as i32,
        StatusInfo {
            name: "MV_E_GC_ACCESS",
            message: "GenICam node access error",
        },
    ),
    (
        sys::MV_E_GC_TIMEOUT as i32,
        StatusInfo {
            name: "MV_E_GC_TIMEOUT",
            message: "GenICam timeout",
        },
    ),
    (
        sys::MV_E_GC_DYNAMICCAST as i32,
        StatusInfo {
            name: "MV_E_GC_DYNAMICCAST",
            message: "GenICam dynamic cast error",
        },
    ),
    (
        sys::MV_E_GC_NODE_NOT_FOUND as i32,
        StatusInfo {
            name: "MV_E_GC_NODE_NOT_FOUND",
            message: "GenICam node not found",
        },
    ),
    (
        sys::MV_E_GC_NODE_VERIFY as i32,
        StatusInfo {
            name: "MV_E_GC_NODE_VERIFY",
            message: "GenICam node validation failed",
        },
    ),
    (
        sys::MV_E_GC_FILE as i32,
        StatusInfo {
            name: "MV_E_GC_FILE",
            message: "GenICam file error",
        },
    ),
    (
        sys::MV_E_GC_URL_DESC as i32,
        StatusInfo {
            name: "MV_E_GC_URL_DESC",
            message: "GenICam device XML URL error",
        },
    ),
    (
        sys::MV_E_GC_UNKNOW as i32,
        StatusInfo {
            name: "MV_E_GC_UNKNOW",
            message: "GenICam unknown error",
        },
    ),
    (
        sys::MV_E_NOT_IMPLEMENTED as i32,
        StatusInfo {
            name: "MV_E_NOT_IMPLEMENTED",
            message: "GigE command is not supported by device",
        },
    ),
    (
        sys::MV_E_INVALID_ADDRESS as i32,
        StatusInfo {
            name: "MV_E_INVALID_ADDRESS",
            message: "GigE target address does not exist",
        },
    ),
    (
        sys::MV_E_WRITE_PROTECT as i32,
        StatusInfo {
            name: "MV_E_WRITE_PROTECT",
            message: "GigE target address is not writable",
        },
    ),
    (
        sys::MV_E_ACCESS_DENIED as i32,
        StatusInfo {
            name: "MV_E_ACCESS_DENIED",
            message: "GigE access denied",
        },
    ),
    (
        sys::MV_E_BUSY as i32,
        StatusInfo {
            name: "MV_E_BUSY",
            message: "GigE device is busy or network is disconnected",
        },
    ),
    (
        sys::MV_E_PACKET as i32,
        StatusInfo {
            name: "MV_E_PACKET",
            message: "GigE packet data error",
        },
    ),
    (
        sys::MV_E_NETER as i32,
        StatusInfo {
            name: "MV_E_NETER",
            message: "GigE network error",
        },
    ),
    (
        sys::MV_E_DRIVERATTACH as i32,
        StatusInfo {
            name: "MV_E_DRIVERATTACH",
            message: "GigE driver is not attached",
        },
    ),
    (
        sys::MV_E_PACKET_ID_MISMATCH as i32,
        StatusInfo {
            name: "MV_E_PACKET_ID_MISMATCH",
            message: "GigE packet ID mismatch",
        },
    ),
    (
        sys::MV_E_IMAGE_BUFFER_OVERFLOW as i32,
        StatusInfo {
            name: "MV_E_IMAGE_BUFFER_OVERFLOW",
            message: "GigE image buffer overflow",
        },
    ),
    (
        sys::MV_E_NO_BUFFER_FOR_USE as i32,
        StatusInfo {
            name: "MV_E_NO_BUFFER_FOR_USE",
            message: "GigE no buffer available",
        },
    ),
    (
        sys::MV_E_XML_INFO_PACKET_ERR as i32,
        StatusInfo {
            name: "MV_E_XML_INFO_PACKET_ERR",
            message: "GigE XML information packet parse error",
        },
    ),
    (
        sys::MV_E_TIMEOUT as i32,
        StatusInfo {
            name: "MV_E_TIMEOUT",
            message: "GigE timeout",
        },
    ),
    (
        sys::MV_E_NET_TRANSMISSION_TYPE_ERR as i32,
        StatusInfo {
            name: "MV_E_NET_TRANSMISSION_TYPE_ERR",
            message: "GigE transmission type parameter error",
        },
    ),
    (
        sys::MV_E_SUPPORT_MODIFY_DEVICE_IP as i32,
        StatusInfo {
            name: "MV_E_SUPPORT_MODIFY_DEVICE_IP",
            message: "Device IP mode cannot be modified in static IP mode",
        },
    ),
    (
        sys::MV_E_KEY_VERIFICATION as i32,
        StatusInfo {
            name: "MV_E_KEY_VERIFICATION",
            message: "GigE key verification error",
        },
    ),
    (
        sys::MV_E_VALUE_NOT_EXPECTED as i32,
        StatusInfo {
            name: "MV_E_VALUE_NOT_EXPECTED",
            message: "GigE unexpected value",
        },
    ),
    (
        sys::MV_E_DEV_DISCONNECT as i32,
        StatusInfo {
            name: "MV_E_DEV_DISCONNECT",
            message: "GigE device disconnected",
        },
    ),
    (
        sys::MV_E_UDP_INIT as i32,
        StatusInfo {
            name: "MV_E_UDP_INIT",
            message: "UDP initialization failed",
        },
    ),
    (
        sys::MV_E_UDP_SEND_DATA as i32,
        StatusInfo {
            name: "MV_E_UDP_SEND_DATA",
            message: "UDP send failed",
        },
    ),
    (
        sys::MV_E_UDP_RECV_DATA as i32,
        StatusInfo {
            name: "MV_E_UDP_RECV_DATA",
            message: "UDP receive failed",
        },
    ),
    (
        sys::MV_E_UDP_CONNECT as i32,
        StatusInfo {
            name: "MV_E_UDP_CONNECT",
            message: "UDP connection failed",
        },
    ),
    (
        sys::MV_E_UDP_RESET_CONNECT as i32,
        StatusInfo {
            name: "MV_E_UDP_RESET_CONNECT",
            message: "UDP reset connection failed",
        },
    ),
    (
        sys::MV_E_MULTICAST_ADD_DEVICE as i32,
        StatusInfo {
            name: "MV_E_MULTICAST_ADD_DEVICE",
            message: "Failed to add multicast device",
        },
    ),
    (
        sys::MV_E_MULTICAST_IP_INVALID as i32,
        StatusInfo {
            name: "MV_E_MULTICAST_IP_INVALID",
            message: "Invalid multicast IP address",
        },
    ),
    (
        sys::MV_E_IP_CONFLICT as i32,
        StatusInfo {
            name: "MV_E_IP_CONFLICT",
            message: "Device IP conflict",
        },
    ),
    (
        sys::MV_E_USB_READ as i32,
        StatusInfo {
            name: "MV_E_USB_READ",
            message: "USB read error",
        },
    ),
    (
        sys::MV_E_USB_WRITE as i32,
        StatusInfo {
            name: "MV_E_USB_WRITE",
            message: "USB write error",
        },
    ),
    (
        sys::MV_E_USB_DEVICE as i32,
        StatusInfo {
            name: "MV_E_USB_DEVICE",
            message: "USB device exception",
        },
    ),
    (
        sys::MV_E_USB_GENICAM as i32,
        StatusInfo {
            name: "MV_E_USB_GENICAM",
            message: "USB GenICam error",
        },
    ),
    (
        sys::MV_E_USB_BANDWIDTH as i32,
        StatusInfo {
            name: "MV_E_USB_BANDWIDTH",
            message: "USB bandwidth is insufficient",
        },
    ),
    (
        sys::MV_E_USB_DRIVER as i32,
        StatusInfo {
            name: "MV_E_USB_DRIVER",
            message: "USB driver mismatch or driver is not installed",
        },
    ),
    (
        sys::MV_E_USB_UNKNOW as i32,
        StatusInfo {
            name: "MV_E_USB_UNKNOW",
            message: "USB unknown error",
        },
    ),
    (
        sys::MV_E_UPG_FILE_MISMATCH as i32,
        StatusInfo {
            name: "MV_E_UPG_FILE_MISMATCH",
            message: "Firmware file mismatch",
        },
    ),
    (
        sys::MV_E_UPG_LANGUSGE_MISMATCH as i32,
        StatusInfo {
            name: "MV_E_UPG_LANGUSGE_MISMATCH",
            message: "Firmware language mismatch",
        },
    ),
    (
        sys::MV_E_UPG_CONFLICT as i32,
        StatusInfo {
            name: "MV_E_UPG_CONFLICT",
            message: "Upgrade conflict",
        },
    ),
    (
        sys::MV_E_UPG_INNER_ERR as i32,
        StatusInfo {
            name: "MV_E_UPG_INNER_ERR",
            message: "Device internal error during upgrade",
        },
    ),
    (
        sys::MV_E_UPG_UNKNOW as i32,
        StatusInfo {
            name: "MV_E_UPG_UNKNOW",
            message: "Unknown upgrade error",
        },
    ),
    (
        sys::MV_E_SUPPORT_PIXEL_FORMAT as i32,
        StatusInfo {
            name: "MV_E_SUPPORT_PIXEL_FORMAT",
            message: "Unsupported pixel format",
        },
    ),
    (
        sys::MV_E_SUPPORT_IMAGE_TYPE as i32,
        StatusInfo {
            name: "MV_E_SUPPORT_IMAGE_TYPE",
            message: "Unsupported image type",
        },
    ),
    (
        sys::MV_E_NOENOUGH_INPUT_DATA as i32,
        StatusInfo {
            name: "MV_E_NOENOUGH_INPUT_DATA",
            message: "Insufficient input image data",
        },
    ),
    (
        sys::MV_E_SR_NOT_INITIAL as i32,
        StatusInfo {
            name: "MV_E_SR_NOT_INITIAL",
            message: "Render module is not initialized",
        },
    ),
    (
        sys::MV_E_SR_SUPPORT_FUNCTION as i32,
        StatusInfo {
            name: "MV_E_SR_SUPPORT_FUNCTION",
            message: "Render function is not supported",
        },
    ),
    (
        sys::MV_E_SR_SUPPORT_ENGINE as i32,
        StatusInfo {
            name: "MV_E_SR_SUPPORT_ENGINE",
            message: "Render engine is not supported",
        },
    ),
    (
        sys::MV_E_SR_SUPPORT_PIXELTYPE as i32,
        StatusInfo {
            name: "MV_E_SR_SUPPORT_PIXELTYPE",
            message: "Render pixel format is not supported",
        },
    ),
    (
        sys::MV_E_SR_SUPPORT_TEXTURESIZE as i32,
        StatusInfo {
            name: "MV_E_SR_SUPPORT_TEXTURESIZE",
            message: "Render texture size is not supported",
        },
    ),
    (
        sys::MV_E_SR_SUPPORT_WND as i32,
        StatusInfo {
            name: "MV_E_SR_SUPPORT_WND",
            message: "Render window is not supported",
        },
    ),
    (
        sys::MV_E_SR_SUPPORT_EFFECT as i32,
        StatusInfo {
            name: "MV_E_SR_SUPPORT_EFFECT",
            message: "Render effect is not supported",
        },
    ),
    (
        sys::MV_E_SR_SUPPORT_VIEWTYPE as i32,
        StatusInfo {
            name: "MV_E_SR_SUPPORT_VIEWTYPE",
            message: "Render view transformation is not supported",
        },
    ),
    (
        sys::MV_E_SR_SUPPORT_STATE as i32,
        StatusInfo {
            name: "MV_E_SR_SUPPORT_STATE",
            message: "Render state is not supported",
        },
    ),
    (
        sys::MV_E_SR_SUBPORT as i32,
        StatusInfo {
            name: "MV_E_SR_SUBPORT",
            message: "Invalid render port",
        },
    ),
    (
        sys::MV_E_SR_PORT_USING as i32,
        StatusInfo {
            name: "MV_E_SR_PORT_USING",
            message: "Render port is in use",
        },
    ),
    (
        sys::MV_E_SR_D3D_RESOURCE as i32,
        StatusInfo {
            name: "MV_E_SR_D3D_RESOURCE",
            message: "Failed to create D3D resource",
        },
    ),
    (
        sys::MV_E_SR_SWAPCHAIN as i32,
        StatusInfo {
            name: "MV_E_SR_SWAPCHAIN",
            message: "Swap chain error",
        },
    ),
    (
        sys::MV_E_SR_SHADER as i32,
        StatusInfo {
            name: "MV_E_SR_SHADER",
            message: "Shader error",
        },
    ),
    (
        sys::MV_E_SR_FONT as i32,
        StatusInfo {
            name: "MV_E_SR_FONT",
            message: "Font rendering error",
        },
    ),
    (
        sys::MV_E_SR_LOAD_LIBRARY as i32,
        StatusInfo {
            name: "MV_E_SR_LOAD_LIBRARY",
            message: "Render module failed to load dynamic library",
        },
    ),
    (
        sys::MV_E_SR_OPENGL_RESOURCE as i32,
        StatusInfo {
            name: "MV_E_SR_OPENGL_RESOURCE",
            message: "Failed to create OpenGL resource",
        },
    ),
    (
        sys::MV_E_SR_CONTEXT as i32,
        StatusInfo {
            name: "MV_E_SR_CONTEXT",
            message: "Render context operation failed",
        },
    ),
    (
        sys::MV_E_SR_PRESENT as i32,
        StatusInfo {
            name: "MV_E_SR_PRESENT",
            message: "Present operation failed",
        },
    ),
    (
        sys::MV_E_SR_INVALID_RECT as i32,
        StatusInfo {
            name: "MV_E_SR_INVALID_RECT",
            message: "Invalid rectangle",
        },
    ),
    (
        sys::MV_E_SR_INVALID_FLOAT as i32,
        StatusInfo {
            name: "MV_E_SR_INVALID_FLOAT",
            message: "Invalid normalized float value",
        },
    ),
    (
        sys::MV_E_SR_INVALID_COLOR as i32,
        StatusInfo {
            name: "MV_E_SR_INVALID_COLOR",
            message: "Invalid color",
        },
    ),
    (
        sys::MV_E_SR_INVALID_POINT as i32,
        StatusInfo {
            name: "MV_E_SR_INVALID_POINT",
            message: "Invalid point",
        },
    ),
    (
        sys::MV_E_SR_RUNTIME as i32,
        StatusInfo {
            name: "MV_E_SR_RUNTIME",
            message: "Render runtime error",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_CMD_NOT_SUPPORT as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_CMD_NOT_SUPPORT",
            message: "Liquid lens command is not supported",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_REGISTER_NOT_EXIST as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_REGISTER_NOT_EXIST",
            message: "Liquid lens register does not exist",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_PERMISSION_DENIED as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_PERMISSION_DENIED",
            message: "Liquid lens permission denied",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_CHECKSUM_ERROR as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_CHECKSUM_ERROR",
            message: "Liquid lens checksum error",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_PACKET_FORMAT_ERROR as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_PACKET_FORMAT_ERROR",
            message: "Liquid lens packet format error",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_DATA_FOAMAT_ERROR as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_DATA_FOAMAT_ERROR",
            message: "Liquid lens data field format error",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_DATA_OUT_RANGE as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_DATA_OUT_RANGE",
            message: "Liquid lens parameter out of range",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_WRITE_DATA_LENGTH_ERROR as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_WRITE_DATA_LENGTH_ERROR",
            message: "Liquid lens write length does not match register length",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_DEVICE_BUSY as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_DEVICE_BUSY",
            message: "Liquid lens device is busy",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_DATA_INCORRECT_ORDER as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_DATA_INCORRECT_ORDER",
            message: "Liquid lens command order error",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_RUN_COND_NOT_MET as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_RUN_COND_NOT_MET",
            message: "Liquid lens run condition is not met",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_COMMANDTIMEOUT as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_COMMANDTIMEOUT",
            message: "Liquid lens command timeout",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_OFFLINE as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_OFFLINE",
            message: "Liquid lens is offline",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_AF_IMAGE_ABNORMAL as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_AF_IMAGE_ABNORMAL",
            message: "Liquid lens autofocus image is abnormal",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_ACK_DATA_LENGTH_ERROR as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_ACK_DATA_LENGTH_ERROR",
            message: "Liquid lens ACK data length error",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_TRIGGER_MODE_NOT_OPEN as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_TRIGGER_MODE_NOT_OPEN",
            message: "Liquid lens trigger mode is not enabled",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_NOT_SOFT_TRIGGER_MODE as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_NOT_SOFT_TRIGGER_MODE",
            message: "Liquid lens is not in soft trigger mode",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_DEVICE_NOT_GRABBING as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_DEVICE_NOT_GRABBING",
            message: "Liquid lens device is not grabbing",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_STRATEGY_NOT_ONEBYONE as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_STRATEGY_NOT_ONEBYONE",
            message: "Liquid lens stream strategy is not supported",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_AF_IMAGE_LOST as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_AF_IMAGE_LOST",
            message: "Liquid lens autofocus image is lost or count is abnormal",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_AF_NOT_CONVERGED as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_AF_NOT_CONVERGED",
            message: "Liquid lens autofocus did not converge",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_SERIAL_PORT_PARAMS_FAIL as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_SERIAL_PORT_PARAMS_FAIL",
            message: "Liquid lens serial port parameter configuration failed",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_INIT_FAILED as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_INIT_FAILED",
            message: "Liquid lens initialization failed",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_TASK_EXECUTING as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_TASK_EXECUTING",
            message: "Liquid lens task is already executing",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_AF_SHARPNESS_CALC_FAILED as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_AF_SHARPNESS_CALC_FAILED",
            message: "Liquid lens autofocus sharpness calculation failed",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_AF_FRAME_RATE_LOW as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_AF_FRAME_RATE_LOW",
            message: "Liquid lens autofocus frame rate is too low",
        },
    ),
    (
        sys::MV_E_LIQUIDLENS_UNDEFINED_ERROR as i32,
        StatusInfo {
            name: "MV_E_LIQUIDLENS_UNDEFINED_ERROR",
            message: "Liquid lens undefined error",
        },
    ),
    (
        sys::MV_ALG_ERR as i32,
        StatusInfo {
            name: "MV_ALG_ERR",
            message: "ISP algorithm unknown error",
        },
    ),
    (
        sys::MV_ALG_E_ABILITY_ARG as i32,
        StatusInfo {
            name: "MV_ALG_E_ABILITY_ARG",
            message: "ISP algorithm ability argument is invalid",
        },
    ),
    (
        sys::MV_ALG_E_MEM_NULL as i32,
        StatusInfo {
            name: "MV_ALG_E_MEM_NULL",
            message: "ISP algorithm memory address is null",
        },
    ),
    (
        sys::MV_ALG_E_MEM_ALIGN as i32,
        StatusInfo {
            name: "MV_ALG_E_MEM_ALIGN",
            message: "ISP algorithm memory alignment error",
        },
    ),
    (
        sys::MV_ALG_E_MEM_LACK as i32,
        StatusInfo {
            name: "MV_ALG_E_MEM_LACK",
            message: "ISP algorithm memory is insufficient",
        },
    ),
    (
        sys::MV_ALG_E_MEM_SIZE_ALIGN as i32,
        StatusInfo {
            name: "MV_ALG_E_MEM_SIZE_ALIGN",
            message: "ISP algorithm memory size alignment error",
        },
    ),
    (
        sys::MV_ALG_E_MEM_ADDR_ALIGN as i32,
        StatusInfo {
            name: "MV_ALG_E_MEM_ADDR_ALIGN",
            message: "ISP algorithm memory address alignment error",
        },
    ),
    (
        sys::MV_ALG_E_IMG_FORMAT as i32,
        StatusInfo {
            name: "MV_ALG_E_IMG_FORMAT",
            message: "ISP algorithm image format is invalid or unsupported",
        },
    ),
    (
        sys::MV_ALG_E_IMG_SIZE as i32,
        StatusInfo {
            name: "MV_ALG_E_IMG_SIZE",
            message: "ISP algorithm image size is invalid or out of range",
        },
    ),
    (
        sys::MV_ALG_E_IMG_STEP as i32,
        StatusInfo {
            name: "MV_ALG_E_IMG_STEP",
            message: "ISP algorithm image size does not match step",
        },
    ),
    (
        sys::MV_ALG_E_IMG_DATA_NULL as i32,
        StatusInfo {
            name: "MV_ALG_E_IMG_DATA_NULL",
            message: "ISP algorithm image data address is null",
        },
    ),
    (
        sys::MV_ALG_E_CFG_TYPE as i32,
        StatusInfo {
            name: "MV_ALG_E_CFG_TYPE",
            message: "ISP algorithm config type is invalid",
        },
    ),
    (
        sys::MV_ALG_E_CFG_SIZE as i32,
        StatusInfo {
            name: "MV_ALG_E_CFG_SIZE",
            message: "ISP algorithm config size is invalid",
        },
    ),
    (
        sys::MV_ALG_E_PRC_TYPE as i32,
        StatusInfo {
            name: "MV_ALG_E_PRC_TYPE",
            message: "ISP algorithm process type is invalid",
        },
    ),
    (
        sys::MV_ALG_E_PRC_SIZE as i32,
        StatusInfo {
            name: "MV_ALG_E_PRC_SIZE",
            message: "ISP algorithm process size is invalid",
        },
    ),
    (
        sys::MV_ALG_E_FUNC_TYPE as i32,
        StatusInfo {
            name: "MV_ALG_E_FUNC_TYPE",
            message: "ISP algorithm sub-process type is invalid",
        },
    ),
    (
        sys::MV_ALG_E_FUNC_SIZE as i32,
        StatusInfo {
            name: "MV_ALG_E_FUNC_SIZE",
            message: "ISP algorithm sub-process size is invalid",
        },
    ),
    (
        sys::MV_ALG_E_PARAM_INDEX as i32,
        StatusInfo {
            name: "MV_ALG_E_PARAM_INDEX",
            message: "ISP algorithm parameter index is invalid",
        },
    ),
    (
        sys::MV_ALG_E_PARAM_VALUE as i32,
        StatusInfo {
            name: "MV_ALG_E_PARAM_VALUE",
            message: "ISP algorithm parameter value is invalid or out of range",
        },
    ),
    (
        sys::MV_ALG_E_PARAM_NUM as i32,
        StatusInfo {
            name: "MV_ALG_E_PARAM_NUM",
            message: "ISP algorithm parameter count is invalid",
        },
    ),
    (
        sys::MV_ALG_E_NULL_PTR as i32,
        StatusInfo {
            name: "MV_ALG_E_NULL_PTR",
            message: "ISP algorithm pointer parameter is null",
        },
    ),
    (
        sys::MV_ALG_E_OVER_MAX_MEM as i32,
        StatusInfo {
            name: "MV_ALG_E_OVER_MAX_MEM",
            message: "ISP algorithm maximum memory limit exceeded",
        },
    ),
    (
        sys::MV_ALG_E_CALL_BACK as i32,
        StatusInfo {
            name: "MV_ALG_E_CALL_BACK",
            message: "ISP algorithm callback error",
        },
    ),
    (
        sys::MV_ALG_E_ENCRYPT as i32,
        StatusInfo {
            name: "MV_ALG_E_ENCRYPT",
            message: "ISP algorithm encryption error",
        },
    ),
    (
        sys::MV_ALG_E_EXPIRE as i32,
        StatusInfo {
            name: "MV_ALG_E_EXPIRE",
            message: "ISP algorithm license expired",
        },
    ),
    (
        sys::MV_ALG_E_BAD_ARG as i32,
        StatusInfo {
            name: "MV_ALG_E_BAD_ARG",
            message: "ISP algorithm argument range is invalid",
        },
    ),
    (
        sys::MV_ALG_E_DATA_SIZE as i32,
        StatusInfo {
            name: "MV_ALG_E_DATA_SIZE",
            message: "ISP algorithm data size is invalid",
        },
    ),
    (
        sys::MV_ALG_E_STEP as i32,
        StatusInfo {
            name: "MV_ALG_E_STEP",
            message: "ISP algorithm data step is invalid",
        },
    ),
    (
        sys::MV_ALG_E_CPUID as i32,
        StatusInfo {
            name: "MV_ALG_E_CPUID",
            message: "CPU does not support required instruction set",
        },
    ),
    (
        sys::MV_ALG_WARNING as i32,
        StatusInfo {
            name: "MV_ALG_WARNING",
            message: "ISP algorithm warning",
        },
    ),
    (
        sys::MV_ALG_E_TIME_OUT as i32,
        StatusInfo {
            name: "MV_ALG_E_TIME_OUT",
            message: "ISP algorithm timeout",
        },
    ),
    (
        sys::MV_ALG_E_LIB_VERSION as i32,
        StatusInfo {
            name: "MV_ALG_E_LIB_VERSION",
            message: "ISP algorithm library version error",
        },
    ),
    (
        sys::MV_ALG_E_MODEL_VERSION as i32,
        StatusInfo {
            name: "MV_ALG_E_MODEL_VERSION",
            message: "ISP algorithm model version error",
        },
    ),
    (
        sys::MV_ALG_E_GPU_MEM_ALLOC as i32,
        StatusInfo {
            name: "MV_ALG_E_GPU_MEM_ALLOC",
            message: "ISP algorithm GPU memory allocation failed",
        },
    ),
    (
        sys::MV_ALG_E_FILE_NON_EXIST as i32,
        StatusInfo {
            name: "MV_ALG_E_FILE_NON_EXIST",
            message: "ISP algorithm file does not exist",
        },
    ),
    (
        sys::MV_ALG_E_NONE_STRING as i32,
        StatusInfo {
            name: "MV_ALG_E_NONE_STRING",
            message: "ISP algorithm string is empty",
        },
    ),
    (
        sys::MV_ALG_E_IMAGE_CODEC as i32,
        StatusInfo {
            name: "MV_ALG_E_IMAGE_CODEC",
            message: "ISP algorithm image codec error",
        },
    ),
    (
        sys::MV_ALG_E_FILE_OPEN as i32,
        StatusInfo {
            name: "MV_ALG_E_FILE_OPEN",
            message: "ISP algorithm failed to open file",
        },
    ),
    (
        sys::MV_ALG_E_FILE_READ as i32,
        StatusInfo {
            name: "MV_ALG_E_FILE_READ",
            message: "ISP algorithm failed to read file",
        },
    ),
    (
        sys::MV_ALG_E_FILE_WRITE as i32,
        StatusInfo {
            name: "MV_ALG_E_FILE_WRITE",
            message: "ISP algorithm failed to write file",
        },
    ),
    (
        sys::MV_ALG_E_FILE_READ_SIZE as i32,
        StatusInfo {
            name: "MV_ALG_E_FILE_READ_SIZE",
            message: "ISP algorithm file read size error",
        },
    ),
    (
        sys::MV_ALG_E_FILE_TYPE as i32,
        StatusInfo {
            name: "MV_ALG_E_FILE_TYPE",
            message: "ISP algorithm file type error",
        },
    ),
    (
        sys::MV_ALG_E_MODEL_TYPE as i32,
        StatusInfo {
            name: "MV_ALG_E_MODEL_TYPE",
            message: "ISP algorithm model type error",
        },
    ),
    (
        sys::MV_ALG_E_MALLOC_MEM as i32,
        StatusInfo {
            name: "MV_ALG_E_MALLOC_MEM",
            message: "ISP algorithm memory allocation failed",
        },
    ),
    (
        sys::MV_ALG_E_BIND_CORE_FAILED as i32,
        StatusInfo {
            name: "MV_ALG_E_BIND_CORE_FAILED",
            message: "ISP algorithm thread core binding failed",
        },
    ),
    (
        sys::MV_ALG_E_DENOISE_NE_IMG_FORMAT as i32,
        StatusInfo {
            name: "MV_ALG_E_DENOISE_NE_IMG_FORMAT",
            message: "Denoise noise-estimation image format error",
        },
    ),
    (
        sys::MV_ALG_E_DENOISE_NE_FEATURE_TYPE as i32,
        StatusInfo {
            name: "MV_ALG_E_DENOISE_NE_FEATURE_TYPE",
            message: "Denoise noise-estimation feature type error",
        },
    ),
    (
        sys::MV_ALG_E_DENOISE_NE_PROFILE_NUM as i32,
        StatusInfo {
            name: "MV_ALG_E_DENOISE_NE_PROFILE_NUM",
            message: "Denoise noise-estimation profile count error",
        },
    ),
    (
        sys::MV_ALG_E_DENOISE_NE_GAIN_NUM as i32,
        StatusInfo {
            name: "MV_ALG_E_DENOISE_NE_GAIN_NUM",
            message: "Denoise noise-estimation gain count error",
        },
    ),
    (
        sys::MV_ALG_E_DENOISE_NE_GAIN_VAL as i32,
        StatusInfo {
            name: "MV_ALG_E_DENOISE_NE_GAIN_VAL",
            message: "Denoise noise-estimation gain value error",
        },
    ),
    (
        sys::MV_ALG_E_DENOISE_NE_BIN_NUM as i32,
        StatusInfo {
            name: "MV_ALG_E_DENOISE_NE_BIN_NUM",
            message: "Denoise noise-estimation bin count error",
        },
    ),
    (
        sys::MV_ALG_E_DENOISE_NE_INIT_GAIN as i32,
        StatusInfo {
            name: "MV_ALG_E_DENOISE_NE_INIT_GAIN",
            message: "Denoise noise-estimation initial gain error",
        },
    ),
    (
        sys::MV_ALG_E_DENOISE_NE_NOT_INIT as i32,
        StatusInfo {
            name: "MV_ALG_E_DENOISE_NE_NOT_INIT",
            message: "Denoise noise estimation is not initialized",
        },
    ),
    (
        sys::MV_ALG_E_DENOISE_COLOR_MODE as i32,
        StatusInfo {
            name: "MV_ALG_E_DENOISE_COLOR_MODE",
            message: "Denoise color mode error",
        },
    ),
    (
        sys::MV_ALG_E_DENOISE_ROI_NUM as i32,
        StatusInfo {
            name: "MV_ALG_E_DENOISE_ROI_NUM",
            message: "Denoise ROI count error",
        },
    ),
    (
        sys::MV_ALG_E_DENOISE_ROI_ORI_PT as i32,
        StatusInfo {
            name: "MV_ALG_E_DENOISE_ROI_ORI_PT",
            message: "Denoise ROI origin error",
        },
    ),
    (
        sys::MV_ALG_E_DENOISE_ROI_SIZE as i32,
        StatusInfo {
            name: "MV_ALG_E_DENOISE_ROI_SIZE",
            message: "Denoise ROI size error",
        },
    ),
    (
        sys::MV_ALG_E_DENOISE_GAIN_NOT_EXIST as i32,
        StatusInfo {
            name: "MV_ALG_E_DENOISE_GAIN_NOT_EXIST",
            message: "Denoise camera gain does not exist",
        },
    ),
    (
        sys::MV_ALG_E_DENOISE_GAIN_BEYOND_RANGE as i32,
        StatusInfo {
            name: "MV_ALG_E_DENOISE_GAIN_BEYOND_RANGE",
            message: "Denoise camera gain is out of range",
        },
    ),
    (
        sys::MV_ALG_E_DENOISE_NP_BUF_SIZE as i32,
        StatusInfo {
            name: "MV_ALG_E_DENOISE_NP_BUF_SIZE",
            message: "Denoise noise profile buffer size error",
        },
    ),
    (
        sys::MV_ALG_E_PFC_ROI_PT as i32,
        StatusInfo {
            name: "MV_ALG_E_PFC_ROI_PT",
            message: "Purple fringe correction ROI origin error",
        },
    ),
    (
        sys::MV_ALG_E_PFC_ROI_SIZE as i32,
        StatusInfo {
            name: "MV_ALG_E_PFC_ROI_SIZE",
            message: "Purple fringe correction ROI size error",
        },
    ),
    (
        sys::MV_ALG_E_PFC_KERNEL_SIZE as i32,
        StatusInfo {
            name: "MV_ALG_E_PFC_KERNEL_SIZE",
            message: "Purple fringe correction kernel size error",
        },
    ),
];

/// Fallback `UNKNOWN` info for status codes not present in `SDK_ERRORS`,
/// precomputed so we never construct it on the fly.
const UNKNOWN_INFO: StatusInfo = StatusInfo {
    name: "UNKNOWN",
    message: "Unknown status code",
};

fn sdk_error_info(status: Status) -> StatusInfo {
    SDK_ERRORS
        .iter()
        .find(|(code, _)| *code == status.0)
        .map(|(_, info)| *info)
        .unwrap_or(UNKNOWN_INFO)
}

#[cfg(test)]
#[path = "error_tests.rs"]
mod tests;
