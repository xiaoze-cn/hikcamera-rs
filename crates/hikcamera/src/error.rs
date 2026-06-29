use std::fmt;

use crate::sys;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    Sdk {
        code: i32,
    },
    NoDevice,
    DeviceNotFound {
        selector: String,
    },
    MultipleDevices {
        selector: String,
        count: usize,
    },
    NullHandle,
    SdkStatePoisoned,
    InvalidString {
        field: &'static str,
    },
    UnsupportedNode {
        key: String,
        kind: &'static str,
    },
    NodeValueMismatch {
        expected: &'static str,
        actual: &'static str,
    },
    NodeInputMismatch {
        key: String,
        expected: &'static str,
        actual: &'static str,
    },
    ValueOutOfRange {
        field: &'static str,
    },
    RecordingInProgress,
    EmptyFrame,
    EmptyVideo,
    InvalidDuration {
        field: &'static str,
    },
    InvalidFrameRate {
        field: &'static str,
    },
    InvalidRoi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorInfo {
    pub name: &'static str,
    pub message: &'static str,
}

pub(crate) fn check(code: i32) -> Result<()> {
    if code == sys::MV_OK as i32 {
        Ok(())
    } else {
        Err(Error::sdk(code))
    }
}

impl Error {
    pub(crate) fn sdk(code: i32) -> Self {
        Self::Sdk { code }
    }

    pub(crate) fn device_not_found(selector: impl Into<String>) -> Self {
        Self::DeviceNotFound {
            selector: selector.into(),
        }
    }

    pub(crate) fn multiple_devices(selector: impl Into<String>, count: usize) -> Self {
        Self::MultipleDevices {
            selector: selector.into(),
            count,
        }
    }

    pub(crate) fn null_handle() -> Self {
        Self::NullHandle
    }

    pub(crate) fn sdk_state_poisoned() -> Self {
        Self::SdkStatePoisoned
    }

    pub(crate) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    pub(crate) fn unsupported_node(key: impl Into<String>, kind: &'static str) -> Self {
        Self::UnsupportedNode {
            key: key.into(),
            kind,
        }
    }

    pub(crate) fn node_value_mismatch(expected: &'static str, actual: &'static str) -> Self {
        Self::NodeValueMismatch { expected, actual }
    }

    pub(crate) fn node_input_mismatch(
        key: impl Into<String>,
        expected: &'static str,
        actual: &'static str,
    ) -> Self {
        Self::NodeInputMismatch {
            key: key.into(),
            expected,
            actual,
        }
    }

    pub(crate) fn value_out_of_range(field: &'static str) -> Self {
        Self::ValueOutOfRange { field }
    }

    pub(crate) fn recording_in_progress() -> Self {
        Self::RecordingInProgress
    }

    pub(crate) fn empty_frame() -> Self {
        Self::EmptyFrame
    }

    pub(crate) fn empty_video() -> Self {
        Self::EmptyVideo
    }

    pub(crate) fn invalid_duration(field: &'static str) -> Self {
        Self::InvalidDuration { field }
    }

    pub(crate) fn invalid_frame_rate(field: &'static str) -> Self {
        Self::InvalidFrameRate { field }
    }

    pub(crate) fn invalid_roi() -> Self {
        Self::InvalidRoi
    }

    pub fn code(&self) -> Option<i32> {
        match self {
            Self::Sdk { code } => Some(*code),
            _ => None,
        }
    }

    pub fn info(&self) -> ErrorInfo {
        match self {
            Self::Sdk { code } => sdk_error_info(*code as u32),
            Self::NoDevice => info("NO_DEVICE", "No HikCamera device was found"),
            Self::DeviceNotFound { .. } => info(
                "DEVICE_NOT_FOUND",
                "No HikCamera device matched the selector",
            ),
            Self::MultipleDevices { .. } => info(
                "MULTIPLE_DEVICES",
                "Multiple HikCamera devices matched the selector",
            ),
            Self::NullHandle => info("NULL_HANDLE", "The HikCamera SDK returned a null handle"),
            Self::SdkStatePoisoned => info(
                "SDK_STATE_POISONED",
                "The HikCamera SDK reference counter is poisoned",
            ),
            Self::InvalidString { .. } => {
                info("INVALID_STRING", "String contains an interior NUL byte")
            }
            Self::UnsupportedNode { .. } => info(
                "UNSUPPORTED_NODE",
                "The GenICam node type is not supported by this helper",
            ),
            Self::NodeValueMismatch { .. } => info(
                "NODE_VALUE_MISMATCH",
                "The GenICam node value was read as a different type",
            ),
            Self::NodeInputMismatch { .. } => info(
                "NODE_INPUT_MISMATCH",
                "The GenICam node input type does not match the node type",
            ),
            Self::ValueOutOfRange { .. } => info(
                "VALUE_OUT_OF_RANGE",
                "Value is out of range for this wrapper API",
            ),
            Self::RecordingInProgress => info(
                "RECORDING_IN_PROGRESS",
                "A video recording is already active on this stream",
            ),
            Self::EmptyFrame => info("EMPTY_FRAME", "Frame has no image data"),
            Self::EmptyVideo => info("EMPTY_VIDEO", "Video output has no frames"),
            Self::InvalidDuration { .. } => info(
                "INVALID_DURATION",
                "Duration must be finite and greater than zero",
            ),
            Self::InvalidFrameRate { .. } => info(
                "INVALID_FRAME_RATE",
                "Frame rate must be finite and greater than zero",
            ),
            Self::InvalidRoi => info(
                "INVALID_ROI",
                "ROI width and height must be greater than zero",
            ),
        }
    }

    pub fn name(&self) -> &'static str {
        self.info().name
    }

    pub fn message(&self) -> &'static str {
        self.info().message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sdk { code } => write!(
                f,
                "{} ({}, 0x{:08X}): {}",
                self.name(),
                code,
                *code as u32,
                self.message()
            ),
            Self::NoDevice => write!(f, "{}", self.message()),
            Self::DeviceNotFound { selector } => {
                write!(f, "no HikCamera device matched selector `{selector}`")
            }
            Self::MultipleDevices { selector, count } => {
                write!(f, "{count} HikCamera devices matched selector `{selector}`")
            }
            Self::NullHandle => write!(f, "HikCamera SDK returned a null handle"),
            Self::SdkStatePoisoned => write!(f, "HikCamera SDK reference counter is poisoned"),
            Self::InvalidString { field } => {
                write!(f, "{field} contains an interior NUL byte")
            }
            Self::UnsupportedNode { key, kind } => {
                write!(f, "node `{key}` has unsupported type `{kind}`")
            }
            Self::NodeValueMismatch { expected, actual } => {
                write!(f, "expected node value `{expected}`, got `{actual}`")
            }
            Self::NodeInputMismatch {
                key,
                expected,
                actual,
            } => write!(f, "node `{key}` expects input `{expected}`, got `{actual}`"),
            Self::ValueOutOfRange { field } => {
                write!(f, "{field} is out of range")
            }
            Self::RecordingInProgress => write!(f, "a video recording is already active"),
            Self::EmptyFrame => write!(f, "frame has no image data"),
            Self::EmptyVideo => write!(f, "video output has no frames"),
            Self::InvalidDuration { field } => {
                write!(f, "{field} must be greater than zero")
            }
            Self::InvalidFrameRate { field } => {
                write!(f, "{field} must be finite and greater than zero")
            }
            Self::InvalidRoi => write!(f, "ROI width and height must be greater than zero"),
        }
    }
}

impl std::error::Error for Error {}

fn sdk_error_info(code: u32) -> ErrorInfo {
    match code {
        sys::MV_OK => info("MV_OK", "Success"),

        sys::MV_E_HANDLE => info("MV_E_HANDLE", "Invalid handle"),
        sys::MV_E_SUPPORT => info("MV_E_SUPPORT", "Unsupported function"),
        sys::MV_E_BUFOVER => info("MV_E_BUFOVER", "Buffer overflow"),
        sys::MV_E_CALLORDER => info("MV_E_CALLORDER", "Invalid function call order"),
        sys::MV_E_PARAMETER => info("MV_E_PARAMETER", "Invalid parameter"),
        sys::MV_E_RESOURCE => info("MV_E_RESOURCE", "Resource allocation failed"),
        sys::MV_E_NODATA => info("MV_E_NODATA", "Timeout or no data received"),
        sys::MV_E_PRECONDITION => info(
            "MV_E_PRECONDITION",
            "Precondition failed or runtime environment changed",
        ),
        sys::MV_E_VERSION => info("MV_E_VERSION", "Version mismatch"),
        sys::MV_E_NOENOUGH_BUF => info("MV_E_NOENOUGH_BUF", "Input buffer is too small"),
        sys::MV_E_ABNORMAL_IMAGE => info("MV_E_ABNORMAL_IMAGE", "Abnormal or incomplete image"),
        sys::MV_E_LOAD_LIBRARY => info("MV_E_LOAD_LIBRARY", "Failed to load dynamic library"),
        sys::MV_E_NOOUTBUF => info("MV_E_NOOUTBUF", "No output buffer available"),
        sys::MV_E_ENCRYPT => info("MV_E_ENCRYPT", "Encryption error"),
        sys::MV_E_OPENFILE => info("MV_E_OPENFILE", "Failed to open file"),
        sys::MV_E_BUF_IN_USE => info("MV_E_BUF_IN_USE", "Buffer is already in use"),
        sys::MV_E_BUF_INVALID => info("MV_E_BUF_INVALID", "Invalid buffer address"),
        sys::MV_E_NOALIGN_BUF => info("MV_E_NOALIGN_BUF", "Buffer alignment error"),
        sys::MV_E_NOENOUGH_BUF_NUM => info("MV_E_NOENOUGH_BUF_NUM", "Insufficient buffer count"),
        sys::MV_E_PORT_IN_USE => info("MV_E_PORT_IN_USE", "Port is in use"),
        sys::MV_E_IMAGE_DECODEC => info("MV_E_IMAGE_DECODEC", "Image decoding error"),
        sys::MV_E_UINT32_LIMIT => {
            info("MV_E_UINT32_LIMIT", "Image size exceeds unsigned int limit")
        }
        sys::MV_E_IMAGE_HEIGHT => info("MV_E_IMAGE_HEIGHT", "Abnormal image height"),
        sys::MV_E_NOENOUGH_DDR => {
            info("MV_E_NOENOUGH_DDR", "Insufficient frame grabber DDR buffer")
        }
        sys::MV_E_NOENOUGH_STREAM => info(
            "MV_E_NOENOUGH_STREAM",
            "Insufficient frame grabber stream channels",
        ),
        sys::MV_E_NORESPONSE => info("MV_E_NORESPONSE", "Device does not respond"),
        sys::MV_E_WRITEFILE => info("MV_E_WRITEFILE", "Failed to write file"),
        sys::MV_E_READFILE => info("MV_E_READFILE", "Failed to read file"),
        sys::MV_E_FILELENGTH => info("MV_E_FILELENGTH", "Invalid file length"),
        sys::MV_E_RESOURCE_EVENT => info("MV_E_RESOURCE_EVENT", "Failed to create event resource"),
        sys::MV_E_RESOURCE_THREAD => {
            info("MV_E_RESOURCE_THREAD", "Failed to create thread resource")
        }
        sys::MV_E_DEV_OFFLINE => info("MV_E_DEV_OFFLINE", "Device is offline"),
        sys::MV_E_DEV_SUPPORT => info("MV_E_DEV_SUPPORT", "Device is not supported"),
        sys::MV_E_PLATFORM_SUPPORT => info("MV_E_PLATFORM_SUPPORT", "Platform is not implemented"),
        sys::MV_E_SERIAL_BUFFER_FULL => {
            info("MV_E_SERIAL_BUFFER_FULL", "Device serial buffer is full")
        }
        sys::MV_E_CHANNEL_INDEX => info("MV_E_CHANNEL_INDEX", "Invalid stream channel index"),
        sys::MV_E_PARAMETER_RANGE => info("MV_E_PARAMETER_RANGE", "Parameter is out of range"),
        sys::MV_E_RESOURCE_IO => info("MV_E_RESOURCE_IO", "IO resource error"),
        sys::MV_E_IMAGE_INFO_INVALID => {
            info("MV_E_IMAGE_INFO_INVALID", "Invalid image information")
        }
        sys::MV_E_RESOURCE_IN_USE => info("MV_E_RESOURCE_IN_USE", "Requested resource is in use"),
        sys::MV_E_DEV_NOT_IMPLEMENTED => info(
            "MV_E_DEV_NOT_IMPLEMENTED",
            "Command is not implemented by device",
        ),
        sys::MV_E_DEV_INVALID_PARAMETER => info(
            "MV_E_DEV_INVALID_PARAMETER",
            "Device command parameter is invalid or out of range",
        ),
        sys::MV_E_DEV_INVALID_ADDRESS => info(
            "MV_E_DEV_INVALID_ADDRESS",
            "Register address does not exist",
        ),
        sys::MV_E_DEV_WRITE_PROTECT => info(
            "MV_E_DEV_WRITE_PROTECT",
            "Attempted to write a read-only register",
        ),
        sys::MV_E_DEV_BAD_ALIGNMENT => {
            info("MV_E_DEV_BAD_ALIGNMENT", "Register address alignment error")
        }
        sys::MV_E_DEV_ACCESS_DENIED => info("MV_E_DEV_ACCESS_DENIED", "Register access denied"),
        sys::MV_E_DEV_BUSY => info("MV_E_DEV_BUSY", "Device is busy"),
        sys::MV_E_DEV_MSG_TIMEOUT => info("MV_E_DEV_MSG_TIMEOUT", "Device response timeout"),
        sys::MV_E_DEV_INVALID_HEADER => {
            info("MV_E_DEV_INVALID_HEADER", "Invalid command header received")
        }
        sys::MV_E_DEV_UNKNOWN => info("MV_E_DEV_UNKNOWN", "Unknown error returned by device"),
        sys::MV_E_DEV_INVALID_PARAMS => info(
            "MV_E_DEV_INVALID_PARAMS",
            "Device returned invalid parameters",
        ),
        sys::MV_E_DEV_WRONG_CONFIG => info(
            "MV_E_DEV_WRONG_CONFIG",
            "Device configuration does not allow this command",
        ),
        sys::MV_E_DEV_CRC => info("MV_E_DEV_CRC", "CRC error"),
        sys::MV_E_INTERNAL => info("MV_E_INTERNAL", "SDK internal error"),
        sys::MV_E_UNKNOW => info("MV_E_UNKNOW", "Unknown error"),

        sys::MV_E_GC_GENERIC => info("MV_E_GC_GENERIC", "GenICam general error"),
        sys::MV_E_GC_ARGUMENT => info("MV_E_GC_ARGUMENT", "GenICam invalid argument"),
        sys::MV_E_GC_RANGE => info("MV_E_GC_RANGE", "GenICam value out of range"),
        sys::MV_E_GC_PROPERTY => info("MV_E_GC_PROPERTY", "GenICam property error"),
        sys::MV_E_GC_RUNTIME => info("MV_E_GC_RUNTIME", "GenICam runtime error"),
        sys::MV_E_GC_LOGICAL => info("MV_E_GC_LOGICAL", "GenICam logical error"),
        sys::MV_E_GC_ACCESS => info("MV_E_GC_ACCESS", "GenICam node access error"),
        sys::MV_E_GC_TIMEOUT => info("MV_E_GC_TIMEOUT", "GenICam timeout"),
        sys::MV_E_GC_DYNAMICCAST => info("MV_E_GC_DYNAMICCAST", "GenICam dynamic cast error"),
        sys::MV_E_GC_NODE_NOT_FOUND => info("MV_E_GC_NODE_NOT_FOUND", "GenICam node not found"),
        sys::MV_E_GC_NODE_VERIFY => info("MV_E_GC_NODE_VERIFY", "GenICam node validation failed"),
        sys::MV_E_GC_FILE => info("MV_E_GC_FILE", "GenICam file error"),
        sys::MV_E_GC_URL_DESC => info("MV_E_GC_URL_DESC", "GenICam device XML URL error"),
        sys::MV_E_GC_UNKNOW => info("MV_E_GC_UNKNOW", "GenICam unknown error"),

        sys::MV_E_NOT_IMPLEMENTED => info(
            "MV_E_NOT_IMPLEMENTED",
            "GigE command is not supported by device",
        ),
        sys::MV_E_INVALID_ADDRESS => {
            info("MV_E_INVALID_ADDRESS", "GigE target address does not exist")
        }
        sys::MV_E_WRITE_PROTECT => {
            info("MV_E_WRITE_PROTECT", "GigE target address is not writable")
        }
        sys::MV_E_ACCESS_DENIED => info("MV_E_ACCESS_DENIED", "GigE access denied"),
        sys::MV_E_BUSY => info(
            "MV_E_BUSY",
            "GigE device is busy or network is disconnected",
        ),
        sys::MV_E_PACKET => info("MV_E_PACKET", "GigE packet data error"),
        sys::MV_E_NETER => info("MV_E_NETER", "GigE network error"),
        sys::MV_E_DRIVERATTACH => info("MV_E_DRIVERATTACH", "GigE driver is not attached"),
        sys::MV_E_PACKET_ID_MISMATCH => info("MV_E_PACKET_ID_MISMATCH", "GigE packet ID mismatch"),
        sys::MV_E_IMAGE_BUFFER_OVERFLOW => {
            info("MV_E_IMAGE_BUFFER_OVERFLOW", "GigE image buffer overflow")
        }
        sys::MV_E_NO_BUFFER_FOR_USE => info("MV_E_NO_BUFFER_FOR_USE", "GigE no buffer available"),
        sys::MV_E_XML_INFO_PACKET_ERR => info(
            "MV_E_XML_INFO_PACKET_ERR",
            "GigE XML information packet parse error",
        ),
        sys::MV_E_TIMEOUT => info("MV_E_TIMEOUT", "GigE timeout"),
        sys::MV_E_NET_TRANSMISSION_TYPE_ERR => info(
            "MV_E_NET_TRANSMISSION_TYPE_ERR",
            "GigE transmission type parameter error",
        ),
        sys::MV_E_SUPPORT_MODIFY_DEVICE_IP => info(
            "MV_E_SUPPORT_MODIFY_DEVICE_IP",
            "Device IP mode cannot be modified in static IP mode",
        ),
        sys::MV_E_KEY_VERIFICATION => info("MV_E_KEY_VERIFICATION", "GigE key verification error"),
        sys::MV_E_VALUE_NOT_EXPECTED => info("MV_E_VALUE_NOT_EXPECTED", "GigE unexpected value"),
        sys::MV_E_DEV_DISCONNECT => info("MV_E_DEV_DISCONNECT", "GigE device disconnected"),
        sys::MV_E_UDP_INIT => info("MV_E_UDP_INIT", "UDP initialization failed"),
        sys::MV_E_UDP_SEND_DATA => info("MV_E_UDP_SEND_DATA", "UDP send failed"),
        sys::MV_E_UDP_RECV_DATA => info("MV_E_UDP_RECV_DATA", "UDP receive failed"),
        sys::MV_E_UDP_CONNECT => info("MV_E_UDP_CONNECT", "UDP connection failed"),
        sys::MV_E_UDP_RESET_CONNECT => {
            info("MV_E_UDP_RESET_CONNECT", "UDP reset connection failed")
        }
        sys::MV_E_MULTICAST_ADD_DEVICE => info(
            "MV_E_MULTICAST_ADD_DEVICE",
            "Failed to add multicast device",
        ),
        sys::MV_E_MULTICAST_IP_INVALID => {
            info("MV_E_MULTICAST_IP_INVALID", "Invalid multicast IP address")
        }
        sys::MV_E_IP_CONFLICT => info("MV_E_IP_CONFLICT", "Device IP conflict"),

        sys::MV_E_USB_READ => info("MV_E_USB_READ", "USB read error"),
        sys::MV_E_USB_WRITE => info("MV_E_USB_WRITE", "USB write error"),
        sys::MV_E_USB_DEVICE => info("MV_E_USB_DEVICE", "USB device exception"),
        sys::MV_E_USB_GENICAM => info("MV_E_USB_GENICAM", "USB GenICam error"),
        sys::MV_E_USB_BANDWIDTH => info("MV_E_USB_BANDWIDTH", "USB bandwidth is insufficient"),
        sys::MV_E_USB_DRIVER => info(
            "MV_E_USB_DRIVER",
            "USB driver mismatch or driver is not installed",
        ),
        sys::MV_E_USB_UNKNOW => info("MV_E_USB_UNKNOW", "USB unknown error"),

        sys::MV_E_UPG_FILE_MISMATCH => info("MV_E_UPG_FILE_MISMATCH", "Firmware file mismatch"),
        sys::MV_E_UPG_LANGUSGE_MISMATCH => {
            info("MV_E_UPG_LANGUSGE_MISMATCH", "Firmware language mismatch")
        }
        sys::MV_E_UPG_CONFLICT => info("MV_E_UPG_CONFLICT", "Upgrade conflict"),
        sys::MV_E_UPG_INNER_ERR => {
            info("MV_E_UPG_INNER_ERR", "Device internal error during upgrade")
        }
        sys::MV_E_UPG_UNKNOW => info("MV_E_UPG_UNKNOW", "Unknown upgrade error"),

        sys::MV_E_SUPPORT_PIXEL_FORMAT => {
            info("MV_E_SUPPORT_PIXEL_FORMAT", "Unsupported pixel format")
        }
        sys::MV_E_SUPPORT_IMAGE_TYPE => info("MV_E_SUPPORT_IMAGE_TYPE", "Unsupported image type"),
        sys::MV_E_NOENOUGH_INPUT_DATA => {
            info("MV_E_NOENOUGH_INPUT_DATA", "Insufficient input image data")
        }
        sys::MV_E_SR_NOT_INITIAL => info("MV_E_SR_NOT_INITIAL", "Render module is not initialized"),
        sys::MV_E_SR_SUPPORT_FUNCTION => info(
            "MV_E_SR_SUPPORT_FUNCTION",
            "Render function is not supported",
        ),
        sys::MV_E_SR_SUPPORT_ENGINE => {
            info("MV_E_SR_SUPPORT_ENGINE", "Render engine is not supported")
        }
        sys::MV_E_SR_SUPPORT_PIXELTYPE => info(
            "MV_E_SR_SUPPORT_PIXELTYPE",
            "Render pixel format is not supported",
        ),
        sys::MV_E_SR_SUPPORT_TEXTURESIZE => info(
            "MV_E_SR_SUPPORT_TEXTURESIZE",
            "Render texture size is not supported",
        ),
        sys::MV_E_SR_SUPPORT_WND => info("MV_E_SR_SUPPORT_WND", "Render window is not supported"),
        sys::MV_E_SR_SUPPORT_EFFECT => {
            info("MV_E_SR_SUPPORT_EFFECT", "Render effect is not supported")
        }
        sys::MV_E_SR_SUPPORT_VIEWTYPE => info(
            "MV_E_SR_SUPPORT_VIEWTYPE",
            "Render view transformation is not supported",
        ),
        sys::MV_E_SR_SUPPORT_STATE => {
            info("MV_E_SR_SUPPORT_STATE", "Render state is not supported")
        }
        sys::MV_E_SR_SUBPORT => info("MV_E_SR_SUBPORT", "Invalid render port"),
        sys::MV_E_SR_PORT_USING => info("MV_E_SR_PORT_USING", "Render port is in use"),
        sys::MV_E_SR_D3D_RESOURCE => info("MV_E_SR_D3D_RESOURCE", "Failed to create D3D resource"),
        sys::MV_E_SR_SWAPCHAIN => info("MV_E_SR_SWAPCHAIN", "Swap chain error"),
        sys::MV_E_SR_SHADER => info("MV_E_SR_SHADER", "Shader error"),
        sys::MV_E_SR_FONT => info("MV_E_SR_FONT", "Font rendering error"),
        sys::MV_E_SR_LOAD_LIBRARY => info(
            "MV_E_SR_LOAD_LIBRARY",
            "Render module failed to load dynamic library",
        ),
        sys::MV_E_SR_OPENGL_RESOURCE => info(
            "MV_E_SR_OPENGL_RESOURCE",
            "Failed to create OpenGL resource",
        ),
        sys::MV_E_SR_CONTEXT => info("MV_E_SR_CONTEXT", "Render context operation failed"),
        sys::MV_E_SR_PRESENT => info("MV_E_SR_PRESENT", "Present operation failed"),
        sys::MV_E_SR_INVALID_RECT => info("MV_E_SR_INVALID_RECT", "Invalid rectangle"),
        sys::MV_E_SR_INVALID_FLOAT => {
            info("MV_E_SR_INVALID_FLOAT", "Invalid normalized float value")
        }
        sys::MV_E_SR_INVALID_COLOR => info("MV_E_SR_INVALID_COLOR", "Invalid color"),
        sys::MV_E_SR_INVALID_POINT => info("MV_E_SR_INVALID_POINT", "Invalid point"),
        sys::MV_E_SR_RUNTIME => info("MV_E_SR_RUNTIME", "Render runtime error"),

        sys::MV_E_LIQUIDLENS_CMD_NOT_SUPPORT => info(
            "MV_E_LIQUIDLENS_CMD_NOT_SUPPORT",
            "Liquid lens command is not supported",
        ),
        sys::MV_E_LIQUIDLENS_REGISTER_NOT_EXIST => info(
            "MV_E_LIQUIDLENS_REGISTER_NOT_EXIST",
            "Liquid lens register does not exist",
        ),
        sys::MV_E_LIQUIDLENS_PERMISSION_DENIED => info(
            "MV_E_LIQUIDLENS_PERMISSION_DENIED",
            "Liquid lens permission denied",
        ),
        sys::MV_E_LIQUIDLENS_CHECKSUM_ERROR => info(
            "MV_E_LIQUIDLENS_CHECKSUM_ERROR",
            "Liquid lens checksum error",
        ),
        sys::MV_E_LIQUIDLENS_PACKET_FORMAT_ERROR => info(
            "MV_E_LIQUIDLENS_PACKET_FORMAT_ERROR",
            "Liquid lens packet format error",
        ),
        sys::MV_E_LIQUIDLENS_DATA_FOAMAT_ERROR => info(
            "MV_E_LIQUIDLENS_DATA_FOAMAT_ERROR",
            "Liquid lens data field format error",
        ),
        sys::MV_E_LIQUIDLENS_DATA_OUT_RANGE => info(
            "MV_E_LIQUIDLENS_DATA_OUT_RANGE",
            "Liquid lens parameter out of range",
        ),
        sys::MV_E_LIQUIDLENS_WRITE_DATA_LENGTH_ERROR => info(
            "MV_E_LIQUIDLENS_WRITE_DATA_LENGTH_ERROR",
            "Liquid lens write length does not match register length",
        ),
        sys::MV_E_LIQUIDLENS_DEVICE_BUSY => {
            info("MV_E_LIQUIDLENS_DEVICE_BUSY", "Liquid lens device is busy")
        }
        sys::MV_E_LIQUIDLENS_DATA_INCORRECT_ORDER => info(
            "MV_E_LIQUIDLENS_DATA_INCORRECT_ORDER",
            "Liquid lens command order error",
        ),
        sys::MV_E_LIQUIDLENS_RUN_COND_NOT_MET => info(
            "MV_E_LIQUIDLENS_RUN_COND_NOT_MET",
            "Liquid lens run condition is not met",
        ),
        sys::MV_E_LIQUIDLENS_COMMANDTIMEOUT => info(
            "MV_E_LIQUIDLENS_COMMANDTIMEOUT",
            "Liquid lens command timeout",
        ),
        sys::MV_E_LIQUIDLENS_OFFLINE => info("MV_E_LIQUIDLENS_OFFLINE", "Liquid lens is offline"),
        sys::MV_E_LIQUIDLENS_AF_IMAGE_ABNORMAL => info(
            "MV_E_LIQUIDLENS_AF_IMAGE_ABNORMAL",
            "Liquid lens autofocus image is abnormal",
        ),
        sys::MV_E_LIQUIDLENS_ACK_DATA_LENGTH_ERROR => info(
            "MV_E_LIQUIDLENS_ACK_DATA_LENGTH_ERROR",
            "Liquid lens ACK data length error",
        ),
        sys::MV_E_LIQUIDLENS_TRIGGER_MODE_NOT_OPEN => info(
            "MV_E_LIQUIDLENS_TRIGGER_MODE_NOT_OPEN",
            "Liquid lens trigger mode is not enabled",
        ),
        sys::MV_E_LIQUIDLENS_NOT_SOFT_TRIGGER_MODE => info(
            "MV_E_LIQUIDLENS_NOT_SOFT_TRIGGER_MODE",
            "Liquid lens is not in soft trigger mode",
        ),
        sys::MV_E_LIQUIDLENS_DEVICE_NOT_GRABBING => info(
            "MV_E_LIQUIDLENS_DEVICE_NOT_GRABBING",
            "Liquid lens device is not grabbing",
        ),
        sys::MV_E_LIQUIDLENS_STRATEGY_NOT_ONEBYONE => info(
            "MV_E_LIQUIDLENS_STRATEGY_NOT_ONEBYONE",
            "Liquid lens stream strategy is not supported",
        ),
        sys::MV_E_LIQUIDLENS_AF_IMAGE_LOST => info(
            "MV_E_LIQUIDLENS_AF_IMAGE_LOST",
            "Liquid lens autofocus image is lost or count is abnormal",
        ),
        sys::MV_E_LIQUIDLENS_AF_NOT_CONVERGED => info(
            "MV_E_LIQUIDLENS_AF_NOT_CONVERGED",
            "Liquid lens autofocus did not converge",
        ),
        sys::MV_E_LIQUIDLENS_SERIAL_PORT_PARAMS_FAIL => info(
            "MV_E_LIQUIDLENS_SERIAL_PORT_PARAMS_FAIL",
            "Liquid lens serial port parameter configuration failed",
        ),
        sys::MV_E_LIQUIDLENS_INIT_FAILED => info(
            "MV_E_LIQUIDLENS_INIT_FAILED",
            "Liquid lens initialization failed",
        ),
        sys::MV_E_LIQUIDLENS_TASK_EXECUTING => info(
            "MV_E_LIQUIDLENS_TASK_EXECUTING",
            "Liquid lens task is already executing",
        ),
        sys::MV_E_LIQUIDLENS_AF_SHARPNESS_CALC_FAILED => info(
            "MV_E_LIQUIDLENS_AF_SHARPNESS_CALC_FAILED",
            "Liquid lens autofocus sharpness calculation failed",
        ),
        sys::MV_E_LIQUIDLENS_AF_FRAME_RATE_LOW => info(
            "MV_E_LIQUIDLENS_AF_FRAME_RATE_LOW",
            "Liquid lens autofocus frame rate is too low",
        ),
        sys::MV_E_LIQUIDLENS_UNDEFINED_ERROR => info(
            "MV_E_LIQUIDLENS_UNDEFINED_ERROR",
            "Liquid lens undefined error",
        ),

        sys::MV_ALG_ERR => info("MV_ALG_ERR", "ISP algorithm unknown error"),
        sys::MV_ALG_E_ABILITY_ARG => info(
            "MV_ALG_E_ABILITY_ARG",
            "ISP algorithm ability argument is invalid",
        ),
        sys::MV_ALG_E_MEM_NULL => info("MV_ALG_E_MEM_NULL", "ISP algorithm memory address is null"),
        sys::MV_ALG_E_MEM_ALIGN => {
            info("MV_ALG_E_MEM_ALIGN", "ISP algorithm memory alignment error")
        }
        sys::MV_ALG_E_MEM_LACK => info("MV_ALG_E_MEM_LACK", "ISP algorithm memory is insufficient"),
        sys::MV_ALG_E_MEM_SIZE_ALIGN => info(
            "MV_ALG_E_MEM_SIZE_ALIGN",
            "ISP algorithm memory size alignment error",
        ),
        sys::MV_ALG_E_MEM_ADDR_ALIGN => info(
            "MV_ALG_E_MEM_ADDR_ALIGN",
            "ISP algorithm memory address alignment error",
        ),
        sys::MV_ALG_E_IMG_FORMAT => info(
            "MV_ALG_E_IMG_FORMAT",
            "ISP algorithm image format is invalid or unsupported",
        ),
        sys::MV_ALG_E_IMG_SIZE => info(
            "MV_ALG_E_IMG_SIZE",
            "ISP algorithm image size is invalid or out of range",
        ),
        sys::MV_ALG_E_IMG_STEP => info(
            "MV_ALG_E_IMG_STEP",
            "ISP algorithm image size does not match step",
        ),
        sys::MV_ALG_E_IMG_DATA_NULL => info(
            "MV_ALG_E_IMG_DATA_NULL",
            "ISP algorithm image data address is null",
        ),
        sys::MV_ALG_E_CFG_TYPE => info("MV_ALG_E_CFG_TYPE", "ISP algorithm config type is invalid"),
        sys::MV_ALG_E_CFG_SIZE => info("MV_ALG_E_CFG_SIZE", "ISP algorithm config size is invalid"),
        sys::MV_ALG_E_PRC_TYPE => {
            info("MV_ALG_E_PRC_TYPE", "ISP algorithm process type is invalid")
        }
        sys::MV_ALG_E_PRC_SIZE => {
            info("MV_ALG_E_PRC_SIZE", "ISP algorithm process size is invalid")
        }
        sys::MV_ALG_E_FUNC_TYPE => info(
            "MV_ALG_E_FUNC_TYPE",
            "ISP algorithm sub-process type is invalid",
        ),
        sys::MV_ALG_E_FUNC_SIZE => info(
            "MV_ALG_E_FUNC_SIZE",
            "ISP algorithm sub-process size is invalid",
        ),
        sys::MV_ALG_E_PARAM_INDEX => info(
            "MV_ALG_E_PARAM_INDEX",
            "ISP algorithm parameter index is invalid",
        ),
        sys::MV_ALG_E_PARAM_VALUE => info(
            "MV_ALG_E_PARAM_VALUE",
            "ISP algorithm parameter value is invalid or out of range",
        ),
        sys::MV_ALG_E_PARAM_NUM => info(
            "MV_ALG_E_PARAM_NUM",
            "ISP algorithm parameter count is invalid",
        ),
        sys::MV_ALG_E_NULL_PTR => info(
            "MV_ALG_E_NULL_PTR",
            "ISP algorithm pointer parameter is null",
        ),
        sys::MV_ALG_E_OVER_MAX_MEM => info(
            "MV_ALG_E_OVER_MAX_MEM",
            "ISP algorithm maximum memory limit exceeded",
        ),
        sys::MV_ALG_E_CALL_BACK => info("MV_ALG_E_CALL_BACK", "ISP algorithm callback error"),
        sys::MV_ALG_E_ENCRYPT => info("MV_ALG_E_ENCRYPT", "ISP algorithm encryption error"),
        sys::MV_ALG_E_EXPIRE => info("MV_ALG_E_EXPIRE", "ISP algorithm license expired"),
        sys::MV_ALG_E_BAD_ARG => info(
            "MV_ALG_E_BAD_ARG",
            "ISP algorithm argument range is invalid",
        ),
        sys::MV_ALG_E_DATA_SIZE => info("MV_ALG_E_DATA_SIZE", "ISP algorithm data size is invalid"),
        sys::MV_ALG_E_STEP => info("MV_ALG_E_STEP", "ISP algorithm data step is invalid"),
        sys::MV_ALG_E_CPUID => info(
            "MV_ALG_E_CPUID",
            "CPU does not support required instruction set",
        ),
        sys::MV_ALG_WARNING => info("MV_ALG_WARNING", "ISP algorithm warning"),
        sys::MV_ALG_E_TIME_OUT => info("MV_ALG_E_TIME_OUT", "ISP algorithm timeout"),
        sys::MV_ALG_E_LIB_VERSION => info(
            "MV_ALG_E_LIB_VERSION",
            "ISP algorithm library version error",
        ),
        sys::MV_ALG_E_MODEL_VERSION => info(
            "MV_ALG_E_MODEL_VERSION",
            "ISP algorithm model version error",
        ),
        sys::MV_ALG_E_GPU_MEM_ALLOC => info(
            "MV_ALG_E_GPU_MEM_ALLOC",
            "ISP algorithm GPU memory allocation failed",
        ),
        sys::MV_ALG_E_FILE_NON_EXIST => info(
            "MV_ALG_E_FILE_NON_EXIST",
            "ISP algorithm file does not exist",
        ),
        sys::MV_ALG_E_NONE_STRING => info("MV_ALG_E_NONE_STRING", "ISP algorithm string is empty"),
        sys::MV_ALG_E_IMAGE_CODEC => {
            info("MV_ALG_E_IMAGE_CODEC", "ISP algorithm image codec error")
        }
        sys::MV_ALG_E_FILE_OPEN => info("MV_ALG_E_FILE_OPEN", "ISP algorithm failed to open file"),
        sys::MV_ALG_E_FILE_READ => info("MV_ALG_E_FILE_READ", "ISP algorithm failed to read file"),
        sys::MV_ALG_E_FILE_WRITE => {
            info("MV_ALG_E_FILE_WRITE", "ISP algorithm failed to write file")
        }
        sys::MV_ALG_E_FILE_READ_SIZE => info(
            "MV_ALG_E_FILE_READ_SIZE",
            "ISP algorithm file read size error",
        ),
        sys::MV_ALG_E_FILE_TYPE => info("MV_ALG_E_FILE_TYPE", "ISP algorithm file type error"),
        sys::MV_ALG_E_MODEL_TYPE => info("MV_ALG_E_MODEL_TYPE", "ISP algorithm model type error"),
        sys::MV_ALG_E_MALLOC_MEM => info(
            "MV_ALG_E_MALLOC_MEM",
            "ISP algorithm memory allocation failed",
        ),
        sys::MV_ALG_E_BIND_CORE_FAILED => info(
            "MV_ALG_E_BIND_CORE_FAILED",
            "ISP algorithm thread core binding failed",
        ),
        sys::MV_ALG_E_DENOISE_NE_IMG_FORMAT => info(
            "MV_ALG_E_DENOISE_NE_IMG_FORMAT",
            "Denoise noise-estimation image format error",
        ),
        sys::MV_ALG_E_DENOISE_NE_FEATURE_TYPE => info(
            "MV_ALG_E_DENOISE_NE_FEATURE_TYPE",
            "Denoise noise-estimation feature type error",
        ),
        sys::MV_ALG_E_DENOISE_NE_PROFILE_NUM => info(
            "MV_ALG_E_DENOISE_NE_PROFILE_NUM",
            "Denoise noise-estimation profile count error",
        ),
        sys::MV_ALG_E_DENOISE_NE_GAIN_NUM => info(
            "MV_ALG_E_DENOISE_NE_GAIN_NUM",
            "Denoise noise-estimation gain count error",
        ),
        sys::MV_ALG_E_DENOISE_NE_GAIN_VAL => info(
            "MV_ALG_E_DENOISE_NE_GAIN_VAL",
            "Denoise noise-estimation gain value error",
        ),
        sys::MV_ALG_E_DENOISE_NE_BIN_NUM => info(
            "MV_ALG_E_DENOISE_NE_BIN_NUM",
            "Denoise noise-estimation bin count error",
        ),
        sys::MV_ALG_E_DENOISE_NE_INIT_GAIN => info(
            "MV_ALG_E_DENOISE_NE_INIT_GAIN",
            "Denoise noise-estimation initial gain error",
        ),
        sys::MV_ALG_E_DENOISE_NE_NOT_INIT => info(
            "MV_ALG_E_DENOISE_NE_NOT_INIT",
            "Denoise noise estimation is not initialized",
        ),
        sys::MV_ALG_E_DENOISE_COLOR_MODE => {
            info("MV_ALG_E_DENOISE_COLOR_MODE", "Denoise color mode error")
        }
        sys::MV_ALG_E_DENOISE_ROI_NUM => {
            info("MV_ALG_E_DENOISE_ROI_NUM", "Denoise ROI count error")
        }
        sys::MV_ALG_E_DENOISE_ROI_ORI_PT => {
            info("MV_ALG_E_DENOISE_ROI_ORI_PT", "Denoise ROI origin error")
        }
        sys::MV_ALG_E_DENOISE_ROI_SIZE => {
            info("MV_ALG_E_DENOISE_ROI_SIZE", "Denoise ROI size error")
        }
        sys::MV_ALG_E_DENOISE_GAIN_NOT_EXIST => info(
            "MV_ALG_E_DENOISE_GAIN_NOT_EXIST",
            "Denoise camera gain does not exist",
        ),
        sys::MV_ALG_E_DENOISE_GAIN_BEYOND_RANGE => info(
            "MV_ALG_E_DENOISE_GAIN_BEYOND_RANGE",
            "Denoise camera gain is out of range",
        ),
        sys::MV_ALG_E_DENOISE_NP_BUF_SIZE => info(
            "MV_ALG_E_DENOISE_NP_BUF_SIZE",
            "Denoise noise profile buffer size error",
        ),
        sys::MV_ALG_E_PFC_ROI_PT => info(
            "MV_ALG_E_PFC_ROI_PT",
            "Purple fringe correction ROI origin error",
        ),
        sys::MV_ALG_E_PFC_ROI_SIZE => info(
            "MV_ALG_E_PFC_ROI_SIZE",
            "Purple fringe correction ROI size error",
        ),
        sys::MV_ALG_E_PFC_KERNEL_SIZE => info(
            "MV_ALG_E_PFC_KERNEL_SIZE",
            "Purple fringe correction kernel size error",
        ),

        _ => info("UNKNOWN", "Unknown status code"),
    }
}

fn info(name: &'static str, message: &'static str) -> ErrorInfo {
    ErrorInfo { name, message }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_code() {
        assert!(check(sys::MV_OK as i32).is_ok());
    }

    #[test]
    fn sdk_error() {
        let error = Error::sdk(sys::MV_E_NODATA as i32);

        assert_eq!(error.code(), Some(sys::MV_E_NODATA as i32));
        assert_eq!(error.name(), "MV_E_NODATA");
        assert_eq!(error.message(), "Timeout or no data received");
        assert_eq!(check(sys::MV_E_NODATA as i32), Err(error));
    }

    #[test]
    fn unknown_sdk_error() {
        let error = Error::sdk(123);

        assert_eq!(error.code(), Some(123));
        assert_eq!(error.name(), "UNKNOWN");
        assert_eq!(error.message(), "Unknown status code");
    }

    #[test]
    fn display_sdk_error() {
        let error = Error::sdk(sys::MV_E_LOAD_LIBRARY as i32);

        assert_eq!(
            error.to_string(),
            "MV_E_LOAD_LIBRARY (-2147483636, 0x8000000C): Failed to load dynamic library"
        );
    }

    #[test]
    fn custom_error_info() {
        let error = Error::NoDevice;

        assert_eq!(error.code(), None);
        assert_eq!(error.name(), "NO_DEVICE");
        assert_eq!(error.message(), "No HikCamera device was found");
    }

    #[test]
    fn wrapper_error_info() {
        let error = Error::node_input_mismatch("ExposureTime", "Float", "EnumSymbol");

        assert_eq!(error.code(), None);
        assert_eq!(error.name(), "NODE_INPUT_MISMATCH");
        assert_eq!(
            error.to_string(),
            "node `ExposureTime` expects input `Float`, got `EnumSymbol`"
        );
    }

    #[test]
    fn wrapper_error_display_details() {
        assert_eq!(
            Error::invalid_string("path").to_string(),
            "path contains an interior NUL byte"
        );
        assert_eq!(
            Error::null_handle().to_string(),
            "HikCamera SDK returned a null handle"
        );
        assert_eq!(
            Error::sdk_state_poisoned().to_string(),
            "HikCamera SDK reference counter is poisoned"
        );
        assert_eq!(
            Error::unsupported_node("Root", "Category").to_string(),
            "node `Root` has unsupported type `Category`"
        );
        assert_eq!(
            Error::node_value_mismatch("Float", "Int").to_string(),
            "expected node value `Float`, got `Int`"
        );
        assert_eq!(
            Error::value_out_of_range("frame dimension").to_string(),
            "frame dimension is out of range"
        );
        assert_eq!(
            Error::recording_in_progress().to_string(),
            "a video recording is already active"
        );
        assert_eq!(Error::empty_frame().to_string(), "frame has no image data");
        assert_eq!(
            Error::empty_video().to_string(),
            "video output has no frames"
        );
        assert_eq!(
            Error::invalid_duration("video duration").to_string(),
            "video duration must be greater than zero"
        );
        assert_eq!(
            Error::invalid_frame_rate("video frame rate").to_string(),
            "video frame rate must be finite and greater than zero"
        );
        assert_eq!(
            Error::invalid_roi().to_string(),
            "ROI width and height must be greater than zero"
        );
    }
}
