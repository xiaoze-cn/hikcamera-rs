use crate::sys;

use derive_more::From;

pub type Result<T> = std::result::Result<T, HikCameraError>;

/// Raw status code returned by the HikCamera MV SDK
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, From)]
pub struct Status(pub i32);

impl Status {
    /// SDK success status
    pub const OK: Self = Self(sys::MV_OK);

    /// Return the raw SDK status code
    #[inline]
    pub const fn raw(self) -> i32 {
        self.0
    }

    /// Return the same bits as an unsigned value for hex formatting
    #[inline]
    pub const fn as_u32(self) -> u32 {
        self.0 as u32
    }

    /// Check whether this status is `MV_OK`
    #[inline]
    pub const fn is_ok(self) -> bool {
        self.0 == Self::OK.0
    }

    /// Look up the human-readable status information
    pub fn info(self) -> StatusInfo {
        sdk_errors()
            .iter()
            .find(|(code, _)| *code == self.0)
            .map(|(_, info)| *info)
            .unwrap_or(UNKNOWN_STATUS_INFO)
    }

    /// Convert this SDK status into a wrapper result
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
        let info = self.info();
        write!(f, "{} ({}, {:#010X})", info.name, self.0, self.as_u32())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
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

/// Human-readable SDK status information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusInfo {
    /// SDK status identifier
    pub name: &'static str,
    /// SDK status message
    pub message: &'static str,
}

/// Convert an SDK return value into a `Result`
pub(crate) fn check(code: i32) -> Result<()> {
    Status(code).into_result()
}

impl HikCameraError {
    /// Return the SDK status code when this is an SDK error
    pub fn code(&self) -> Option<i32> {
        match self {
            Self::Sdk { status } => Some(status.raw()),
            _ => None,
        }
    }
}

macro_rules! status_info {
    ($code:ident, $message:literal) => {
        (
            sys::$code,
            StatusInfo {
                name: stringify!($code),
                message: $message,
            },
        )
    };
}

/// Look up SDK status descriptions with a small linear table
fn sdk_errors() -> &'static [(i32, StatusInfo)] {
    &[
        status_info!(MV_OK, "Success"),
        status_info!(MV_E_HANDLE, "Invalid handle"),
        status_info!(MV_E_SUPPORT, "Unsupported function"),
        status_info!(MV_E_BUFOVER, "Buffer overflow"),
        status_info!(MV_E_CALLORDER, "Invalid function call order"),
        status_info!(MV_E_PARAMETER, "Invalid parameter"),
        status_info!(MV_E_RESOURCE, "Resource allocation failed"),
        status_info!(MV_E_NODATA, "Timeout or no data received"),
        status_info!(
            MV_E_PRECONDITION,
            "Precondition failed or runtime environment changed"
        ),
        status_info!(MV_E_VERSION, "Version mismatch"),
        status_info!(MV_E_NOENOUGH_BUF, "Input buffer is too small"),
        status_info!(MV_E_ABNORMAL_IMAGE, "Abnormal or incomplete image"),
        status_info!(MV_E_LOAD_LIBRARY, "Failed to load dynamic library"),
        status_info!(MV_E_NOOUTBUF, "No output buffer available"),
        status_info!(MV_E_ENCRYPT, "Encryption error"),
        status_info!(MV_E_OPENFILE, "Failed to open file"),
        status_info!(MV_E_BUF_IN_USE, "Buffer is already in use"),
        status_info!(MV_E_BUF_INVALID, "Invalid buffer address"),
        status_info!(MV_E_NOALIGN_BUF, "Buffer alignment error"),
        status_info!(MV_E_NOENOUGH_BUF_NUM, "Insufficient buffer count"),
        status_info!(MV_E_PORT_IN_USE, "Port is in use"),
        status_info!(MV_E_IMAGE_DECODEC, "Image decoding error"),
        status_info!(MV_E_UINT32_LIMIT, "Image size exceeds unsigned int limit"),
        status_info!(MV_E_IMAGE_HEIGHT, "Abnormal image height"),
        status_info!(MV_E_NOENOUGH_DDR, "Insufficient frame grabber DDR buffer"),
        status_info!(
            MV_E_NOENOUGH_STREAM,
            "Insufficient frame grabber stream channels"
        ),
        status_info!(MV_E_NORESPONSE, "Device does not respond"),
        status_info!(MV_E_WRITEFILE, "Failed to write file"),
        status_info!(MV_E_READFILE, "Failed to read file"),
        status_info!(MV_E_FILELENGTH, "Invalid file length"),
        status_info!(MV_E_RESOURCE_EVENT, "Failed to create event resource"),
        status_info!(MV_E_RESOURCE_THREAD, "Failed to create thread resource"),
        status_info!(MV_E_DEV_OFFLINE, "Device is offline"),
        status_info!(MV_E_DEV_SUPPORT, "Device is not supported"),
        status_info!(MV_E_PLATFORM_SUPPORT, "Platform is not implemented"),
        status_info!(MV_E_SERIAL_BUFFER_FULL, "Device serial buffer is full"),
        status_info!(MV_E_CHANNEL_INDEX, "Invalid stream channel index"),
        status_info!(MV_E_PARAMETER_RANGE, "Parameter is out of range"),
        status_info!(MV_E_RESOURCE_IO, "IO resource error"),
        status_info!(MV_E_IMAGE_INFO_INVALID, "Invalid image information"),
        status_info!(MV_E_RESOURCE_IN_USE, "Requested resource is in use"),
        status_info!(
            MV_E_DEV_NOT_IMPLEMENTED,
            "Command is not implemented by device"
        ),
        status_info!(
            MV_E_DEV_INVALID_PARAMETER,
            "Device command parameter is invalid or out of range"
        ),
        status_info!(MV_E_DEV_INVALID_ADDRESS, "Register address does not exist"),
        status_info!(
            MV_E_DEV_WRITE_PROTECT,
            "Attempted to write a read-only register"
        ),
        status_info!(MV_E_DEV_BAD_ALIGNMENT, "Register address alignment error"),
        status_info!(MV_E_DEV_ACCESS_DENIED, "Register access denied"),
        status_info!(MV_E_DEV_BUSY, "Device is busy"),
        status_info!(MV_E_DEV_MSG_TIMEOUT, "Device response timeout"),
        status_info!(MV_E_DEV_INVALID_HEADER, "Invalid command header received"),
        status_info!(MV_E_DEV_UNKNOWN, "Unknown error returned by device"),
        status_info!(
            MV_E_DEV_INVALID_PARAMS,
            "Device returned invalid parameters"
        ),
        status_info!(
            MV_E_DEV_WRONG_CONFIG,
            "Device configuration does not allow this command"
        ),
        status_info!(MV_E_DEV_CRC, "CRC error"),
        status_info!(MV_E_INTERNAL, "SDK internal error"),
        status_info!(MV_E_UNKNOW, "Unknown error"),
        status_info!(MV_E_GC_GENERIC, "GenICam general error"),
        status_info!(MV_E_GC_ARGUMENT, "GenICam invalid argument"),
        status_info!(MV_E_GC_RANGE, "GenICam value out of range"),
        status_info!(MV_E_GC_PROPERTY, "GenICam property error"),
        status_info!(MV_E_GC_RUNTIME, "GenICam runtime error"),
        status_info!(MV_E_GC_LOGICAL, "GenICam logical error"),
        status_info!(MV_E_GC_ACCESS, "GenICam node access error"),
        status_info!(MV_E_GC_TIMEOUT, "GenICam timeout"),
        status_info!(MV_E_GC_DYNAMICCAST, "GenICam dynamic cast error"),
        status_info!(MV_E_GC_NODE_NOT_FOUND, "GenICam node not found"),
        status_info!(MV_E_GC_NODE_VERIFY, "GenICam node validation failed"),
        status_info!(MV_E_GC_FILE, "GenICam file error"),
        status_info!(MV_E_GC_URL_DESC, "GenICam device XML URL error"),
        status_info!(MV_E_GC_UNKNOW, "GenICam unknown error"),
        status_info!(
            MV_E_NOT_IMPLEMENTED,
            "GigE command is not supported by device"
        ),
        status_info!(MV_E_INVALID_ADDRESS, "GigE target address does not exist"),
        status_info!(MV_E_WRITE_PROTECT, "GigE target address is not writable"),
        status_info!(MV_E_ACCESS_DENIED, "GigE access denied"),
        status_info!(MV_E_BUSY, "GigE device is busy or network is disconnected"),
        status_info!(MV_E_PACKET, "GigE packet data error"),
        status_info!(MV_E_NETER, "GigE network error"),
        status_info!(MV_E_DRIVERATTACH, "GigE driver is not attached"),
        status_info!(MV_E_PACKET_ID_MISMATCH, "GigE packet ID mismatch"),
        status_info!(MV_E_IMAGE_BUFFER_OVERFLOW, "GigE image buffer overflow"),
        status_info!(MV_E_NO_BUFFER_FOR_USE, "GigE no buffer available"),
        status_info!(
            MV_E_XML_INFO_PACKET_ERR,
            "GigE XML information packet parse error"
        ),
        status_info!(MV_E_TIMEOUT, "GigE timeout"),
        status_info!(
            MV_E_NET_TRANSMISSION_TYPE_ERR,
            "GigE transmission type parameter error"
        ),
        status_info!(
            MV_E_SUPPORT_MODIFY_DEVICE_IP,
            "Device IP mode cannot be modified in static IP mode"
        ),
        status_info!(MV_E_KEY_VERIFICATION, "GigE key verification error"),
        status_info!(MV_E_VALUE_NOT_EXPECTED, "GigE unexpected value"),
        status_info!(MV_E_DEV_DISCONNECT, "GigE device disconnected"),
        status_info!(MV_E_UDP_INIT, "UDP initialization failed"),
        status_info!(MV_E_UDP_SEND_DATA, "UDP send failed"),
        status_info!(MV_E_UDP_RECV_DATA, "UDP receive failed"),
        status_info!(MV_E_UDP_CONNECT, "UDP connection failed"),
        status_info!(MV_E_UDP_RESET_CONNECT, "UDP reset connection failed"),
        status_info!(MV_E_MULTICAST_ADD_DEVICE, "Failed to add multicast device"),
        status_info!(MV_E_MULTICAST_IP_INVALID, "Invalid multicast IP address"),
        status_info!(MV_E_IP_CONFLICT, "Device IP conflict"),
        status_info!(MV_E_USB_READ, "USB read error"),
        status_info!(MV_E_USB_WRITE, "USB write error"),
        status_info!(MV_E_USB_DEVICE, "USB device exception"),
        status_info!(MV_E_USB_GENICAM, "USB GenICam error"),
        status_info!(MV_E_USB_BANDWIDTH, "USB bandwidth is insufficient"),
        status_info!(
            MV_E_USB_DRIVER,
            "USB driver mismatch or driver is not installed"
        ),
        status_info!(MV_E_USB_UNKNOW, "USB unknown error"),
        status_info!(MV_E_UPG_FILE_MISMATCH, "Firmware file mismatch"),
        status_info!(MV_E_UPG_LANGUSGE_MISMATCH, "Firmware language mismatch"),
        status_info!(MV_E_UPG_CONFLICT, "Upgrade conflict"),
        status_info!(MV_E_UPG_INNER_ERR, "Device internal error during upgrade"),
        status_info!(MV_E_UPG_UNKNOW, "Unknown upgrade error"),
        status_info!(MV_E_SUPPORT_PIXEL_FORMAT, "Unsupported pixel format"),
        status_info!(MV_E_SUPPORT_IMAGE_TYPE, "Unsupported image type"),
        status_info!(MV_E_NOENOUGH_INPUT_DATA, "Insufficient input image data"),
        status_info!(MV_E_SR_NOT_INITIAL, "Render module is not initialized"),
        status_info!(MV_E_SR_SUPPORT_FUNCTION, "Render function is not supported"),
        status_info!(MV_E_SR_SUPPORT_ENGINE, "Render engine is not supported"),
        status_info!(
            MV_E_SR_SUPPORT_PIXELTYPE,
            "Render pixel format is not supported"
        ),
        status_info!(
            MV_E_SR_SUPPORT_TEXTURESIZE,
            "Render texture size is not supported"
        ),
        status_info!(MV_E_SR_SUPPORT_WND, "Render window is not supported"),
        status_info!(MV_E_SR_SUPPORT_EFFECT, "Render effect is not supported"),
        status_info!(
            MV_E_SR_SUPPORT_VIEWTYPE,
            "Render view transformation is not supported"
        ),
        status_info!(MV_E_SR_SUPPORT_STATE, "Render state is not supported"),
        status_info!(MV_E_SR_SUBPORT, "Invalid render port"),
        status_info!(MV_E_SR_PORT_USING, "Render port is in use"),
        status_info!(MV_E_SR_D3D_RESOURCE, "Failed to create D3D resource"),
        status_info!(MV_E_SR_SWAPCHAIN, "Swap chain error"),
        status_info!(MV_E_SR_SHADER, "Shader error"),
        status_info!(MV_E_SR_FONT, "Font rendering error"),
        status_info!(
            MV_E_SR_LOAD_LIBRARY,
            "Render module failed to load dynamic library"
        ),
        status_info!(MV_E_SR_OPENGL_RESOURCE, "Failed to create OpenGL resource"),
        status_info!(MV_E_SR_CONTEXT, "Render context operation failed"),
        status_info!(MV_E_SR_PRESENT, "Present operation failed"),
        status_info!(MV_E_SR_INVALID_RECT, "Invalid rectangle"),
        status_info!(MV_E_SR_INVALID_FLOAT, "Invalid normalized float value"),
        status_info!(MV_E_SR_INVALID_COLOR, "Invalid color"),
        status_info!(MV_E_SR_INVALID_POINT, "Invalid point"),
        status_info!(MV_E_SR_RUNTIME, "Render runtime error"),
        status_info!(
            MV_E_LIQUIDLENS_CMD_NOT_SUPPORT,
            "Liquid lens command is not supported"
        ),
        status_info!(
            MV_E_LIQUIDLENS_REGISTER_NOT_EXIST,
            "Liquid lens register does not exist"
        ),
        status_info!(
            MV_E_LIQUIDLENS_PERMISSION_DENIED,
            "Liquid lens permission denied"
        ),
        status_info!(MV_E_LIQUIDLENS_CHECKSUM_ERROR, "Liquid lens checksum error"),
        status_info!(
            MV_E_LIQUIDLENS_PACKET_FORMAT_ERROR,
            "Liquid lens packet format error"
        ),
        status_info!(
            MV_E_LIQUIDLENS_DATA_FOAMAT_ERROR,
            "Liquid lens data field format error"
        ),
        status_info!(
            MV_E_LIQUIDLENS_DATA_OUT_RANGE,
            "Liquid lens parameter out of range"
        ),
        status_info!(
            MV_E_LIQUIDLENS_WRITE_DATA_LENGTH_ERROR,
            "Liquid lens write length does not match register length"
        ),
        status_info!(MV_E_LIQUIDLENS_DEVICE_BUSY, "Liquid lens device is busy"),
        status_info!(
            MV_E_LIQUIDLENS_DATA_INCORRECT_ORDER,
            "Liquid lens command order error"
        ),
        status_info!(
            MV_E_LIQUIDLENS_RUN_COND_NOT_MET,
            "Liquid lens run condition is not met"
        ),
        status_info!(
            MV_E_LIQUIDLENS_COMMANDTIMEOUT,
            "Liquid lens command timeout"
        ),
        status_info!(MV_E_LIQUIDLENS_OFFLINE, "Liquid lens is offline"),
        status_info!(
            MV_E_LIQUIDLENS_AF_IMAGE_ABNORMAL,
            "Liquid lens autofocus image is abnormal"
        ),
        status_info!(
            MV_E_LIQUIDLENS_ACK_DATA_LENGTH_ERROR,
            "Liquid lens ACK data length error"
        ),
        status_info!(
            MV_E_LIQUIDLENS_TRIGGER_MODE_NOT_OPEN,
            "Liquid lens trigger mode is not enabled"
        ),
        status_info!(
            MV_E_LIQUIDLENS_NOT_SOFT_TRIGGER_MODE,
            "Liquid lens is not in soft trigger mode"
        ),
        status_info!(
            MV_E_LIQUIDLENS_DEVICE_NOT_GRABBING,
            "Liquid lens device is not grabbing"
        ),
        status_info!(
            MV_E_LIQUIDLENS_STRATEGY_NOT_ONEBYONE,
            "Liquid lens stream strategy is not supported"
        ),
        status_info!(
            MV_E_LIQUIDLENS_AF_IMAGE_LOST,
            "Liquid lens autofocus image is lost or count is abnormal"
        ),
        status_info!(
            MV_E_LIQUIDLENS_AF_NOT_CONVERGED,
            "Liquid lens autofocus did not converge"
        ),
        status_info!(
            MV_E_LIQUIDLENS_SERIAL_PORT_PARAMS_FAIL,
            "Liquid lens serial port parameter configuration failed"
        ),
        status_info!(
            MV_E_LIQUIDLENS_INIT_FAILED,
            "Liquid lens initialization failed"
        ),
        status_info!(
            MV_E_LIQUIDLENS_TASK_EXECUTING,
            "Liquid lens task is already executing"
        ),
        status_info!(
            MV_E_LIQUIDLENS_AF_SHARPNESS_CALC_FAILED,
            "Liquid lens autofocus sharpness calculation failed"
        ),
        status_info!(
            MV_E_LIQUIDLENS_AF_FRAME_RATE_LOW,
            "Liquid lens autofocus frame rate is too low"
        ),
        status_info!(
            MV_E_LIQUIDLENS_UNDEFINED_ERROR,
            "Liquid lens undefined error"
        ),
        status_info!(MV_ALG_ERR, "ISP algorithm unknown error"),
        status_info!(
            MV_ALG_E_ABILITY_ARG,
            "ISP algorithm ability argument is invalid"
        ),
        status_info!(MV_ALG_E_MEM_NULL, "ISP algorithm memory address is null"),
        status_info!(MV_ALG_E_MEM_ALIGN, "ISP algorithm memory alignment error"),
        status_info!(MV_ALG_E_MEM_LACK, "ISP algorithm memory is insufficient"),
        status_info!(
            MV_ALG_E_MEM_SIZE_ALIGN,
            "ISP algorithm memory size alignment error"
        ),
        status_info!(
            MV_ALG_E_MEM_ADDR_ALIGN,
            "ISP algorithm memory address alignment error"
        ),
        status_info!(
            MV_ALG_E_IMG_FORMAT,
            "ISP algorithm image format is invalid or unsupported"
        ),
        status_info!(
            MV_ALG_E_IMG_SIZE,
            "ISP algorithm image size is invalid or out of range"
        ),
        status_info!(
            MV_ALG_E_IMG_STEP,
            "ISP algorithm image size does not match step"
        ),
        status_info!(
            MV_ALG_E_IMG_DATA_NULL,
            "ISP algorithm image data address is null"
        ),
        status_info!(MV_ALG_E_CFG_TYPE, "ISP algorithm config type is invalid"),
        status_info!(MV_ALG_E_CFG_SIZE, "ISP algorithm config size is invalid"),
        status_info!(MV_ALG_E_PRC_TYPE, "ISP algorithm process type is invalid"),
        status_info!(MV_ALG_E_PRC_SIZE, "ISP algorithm process size is invalid"),
        status_info!(
            MV_ALG_E_FUNC_TYPE,
            "ISP algorithm sub-process type is invalid"
        ),
        status_info!(
            MV_ALG_E_FUNC_SIZE,
            "ISP algorithm sub-process size is invalid"
        ),
        status_info!(
            MV_ALG_E_PARAM_INDEX,
            "ISP algorithm parameter index is invalid"
        ),
        status_info!(
            MV_ALG_E_PARAM_VALUE,
            "ISP algorithm parameter value is invalid or out of range"
        ),
        status_info!(
            MV_ALG_E_PARAM_NUM,
            "ISP algorithm parameter count is invalid"
        ),
        status_info!(MV_ALG_E_NULL_PTR, "ISP algorithm pointer parameter is null"),
        status_info!(
            MV_ALG_E_OVER_MAX_MEM,
            "ISP algorithm maximum memory limit exceeded"
        ),
        status_info!(MV_ALG_E_CALL_BACK, "ISP algorithm callback error"),
        status_info!(MV_ALG_E_ENCRYPT, "ISP algorithm encryption error"),
        status_info!(MV_ALG_E_EXPIRE, "ISP algorithm license expired"),
        status_info!(MV_ALG_E_BAD_ARG, "ISP algorithm argument range is invalid"),
        status_info!(MV_ALG_E_DATA_SIZE, "ISP algorithm data size is invalid"),
        status_info!(MV_ALG_E_STEP, "ISP algorithm data step is invalid"),
        status_info!(
            MV_ALG_E_CPUID,
            "CPU does not support required instruction set"
        ),
        status_info!(MV_ALG_WARNING, "ISP algorithm warning"),
        status_info!(MV_ALG_E_TIME_OUT, "ISP algorithm timeout"),
        status_info!(MV_ALG_E_LIB_VERSION, "ISP algorithm library version error"),
        status_info!(MV_ALG_E_MODEL_VERSION, "ISP algorithm model version error"),
        status_info!(
            MV_ALG_E_GPU_MEM_ALLOC,
            "ISP algorithm GPU memory allocation failed"
        ),
        status_info!(MV_ALG_E_FILE_NON_EXIST, "ISP algorithm file does not exist"),
        status_info!(MV_ALG_E_NONE_STRING, "ISP algorithm string is empty"),
        status_info!(MV_ALG_E_IMAGE_CODEC, "ISP algorithm image codec error"),
        status_info!(MV_ALG_E_FILE_OPEN, "ISP algorithm failed to open file"),
        status_info!(MV_ALG_E_FILE_READ, "ISP algorithm failed to read file"),
        status_info!(MV_ALG_E_FILE_WRITE, "ISP algorithm failed to write file"),
        status_info!(
            MV_ALG_E_FILE_READ_SIZE,
            "ISP algorithm file read size error"
        ),
        status_info!(MV_ALG_E_FILE_TYPE, "ISP algorithm file type error"),
        status_info!(MV_ALG_E_MODEL_TYPE, "ISP algorithm model type error"),
        status_info!(
            MV_ALG_E_MALLOC_MEM,
            "ISP algorithm memory allocation failed"
        ),
        status_info!(
            MV_ALG_E_BIND_CORE_FAILED,
            "ISP algorithm thread core binding failed"
        ),
        status_info!(
            MV_ALG_E_DENOISE_NE_IMG_FORMAT,
            "Denoise noise-estimation image format error"
        ),
        status_info!(
            MV_ALG_E_DENOISE_NE_FEATURE_TYPE,
            "Denoise noise-estimation feature type error"
        ),
        status_info!(
            MV_ALG_E_DENOISE_NE_PROFILE_NUM,
            "Denoise noise-estimation profile count error"
        ),
        status_info!(
            MV_ALG_E_DENOISE_NE_GAIN_NUM,
            "Denoise noise-estimation gain count error"
        ),
        status_info!(
            MV_ALG_E_DENOISE_NE_GAIN_VAL,
            "Denoise noise-estimation gain value error"
        ),
        status_info!(
            MV_ALG_E_DENOISE_NE_BIN_NUM,
            "Denoise noise-estimation bin count error"
        ),
        status_info!(
            MV_ALG_E_DENOISE_NE_INIT_GAIN,
            "Denoise noise-estimation initial gain error"
        ),
        status_info!(
            MV_ALG_E_DENOISE_NE_NOT_INIT,
            "Denoise noise estimation is not initialized"
        ),
        status_info!(MV_ALG_E_DENOISE_COLOR_MODE, "Denoise color mode error"),
        status_info!(MV_ALG_E_DENOISE_ROI_NUM, "Denoise ROI count error"),
        status_info!(MV_ALG_E_DENOISE_ROI_ORI_PT, "Denoise ROI origin error"),
        status_info!(MV_ALG_E_DENOISE_ROI_SIZE, "Denoise ROI size error"),
        status_info!(
            MV_ALG_E_DENOISE_GAIN_NOT_EXIST,
            "Denoise camera gain does not exist"
        ),
        status_info!(
            MV_ALG_E_DENOISE_GAIN_BEYOND_RANGE,
            "Denoise camera gain is out of range"
        ),
        status_info!(
            MV_ALG_E_DENOISE_NP_BUF_SIZE,
            "Denoise noise profile buffer size error"
        ),
        status_info!(
            MV_ALG_E_PFC_ROI_PT,
            "Purple fringe correction ROI origin error"
        ),
        status_info!(
            MV_ALG_E_PFC_ROI_SIZE,
            "Purple fringe correction ROI size error"
        ),
        status_info!(
            MV_ALG_E_PFC_KERNEL_SIZE,
            "Purple fringe correction kernel size error"
        ),
    ][..]
}

const UNKNOWN_STATUS_INFO: StatusInfo = StatusInfo {
    name: "UNKNOWN",
    message: "Unknown status code",
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_status() {
        assert_eq!(check(sys::MV_OK), Ok(()));

        let status = Status(sys::MV_E_NODATA);
        let error = HikCameraError::Sdk { status };

        assert_eq!(status.info().name, "MV_E_NODATA");
        assert_eq!(status.info().message, "Timeout or no data received");
        assert_eq!(error.code(), Some(sys::MV_E_NODATA));
        assert_eq!(check(sys::MV_E_NODATA), Err(error));
    }

    #[test]
    fn status_info() {
        assert!(Status::OK.is_ok());
        assert!(!Status(sys::MV_E_NODATA).is_ok());
        assert_eq!(
            Status(sys::MV_E_LOAD_LIBRARY).to_string(),
            "MV_E_LOAD_LIBRARY (-2147483636, 0x8000000C)"
        );

        let unknown = Status(123).info();
        assert_eq!(unknown.name, "UNKNOWN");
        assert_eq!(unknown.message, "Unknown status code");
    }

    #[test]
    fn sdk_error() {
        let error = HikCameraError::Sdk {
            status: Status(sys::MV_E_LOAD_LIBRARY),
        };

        assert_eq!(error.code(), Some(sys::MV_E_LOAD_LIBRARY));
        assert_eq!(
            error.to_string(),
            "MV_E_LOAD_LIBRARY (-2147483636, 0x8000000C): Failed to load dynamic library"
        );
    }

    #[test]
    fn wrapper_error() {
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
}
