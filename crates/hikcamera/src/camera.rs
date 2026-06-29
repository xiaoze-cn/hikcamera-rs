use std::cell::Cell;
use std::ffi::{CStr, CString, c_void};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};
use std::ptr::NonNull;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::{Error, HikCamera, Result, error::check, sys};

#[derive(Debug)]
pub struct Camera<'hik> {
    inner: Option<Rc<CameraInner>>,
    _hik: PhantomData<&'hik HikCamera>,
}

#[derive(Debug)]
pub struct Stream<'hik> {
    inner: Option<Rc<CameraInner>>,
    recording: Rc<Cell<bool>>,
    _hik: PhantomData<&'hik HikCamera>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub info: FrameInfo,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FrameInfo {
    pub width: u32,
    pub height: u32,
    pub pixel_type: u32,
    pub frame_len: u64,
    pub frame_num: u32,
    pub device_timestamp: u64,
    pub host_timestamp: i64,
    pub second_count: u32,
    pub cycle_count: u32,
    pub cycle_offset: u32,
    pub gain: f32,
    pub exposure_time: f32,
    pub average_brightness: u32,
    pub red: u32,
    pub green: u32,
    pub blue: u32,
    pub frame_counter: u32,
    pub trigger_index: u32,
    pub input: u32,
    pub output: u32,
    pub offset_x: u32,
    pub offset_y: u32,
    pub chunk_width: u32,
    pub chunk_height: u32,
    pub lost_packet: u32,
    pub unparsed_chunk_num: u32,
    pub extra_type: u32,
    pub sub_image_num: u32,
    pub first_encoder_count: u32,
    pub last_encoder_count: u32,
    pub last_frame_flag: u32,
}

pub type Image = Frame;
pub type ImageInfo = FrameInfo;

#[derive(Debug)]
pub struct ImageWriter {
    inner: Rc<CameraInner>,
    path: PathBuf,
    options: SaveOptions,
}

#[derive(Debug)]
pub struct VideoWriter {
    inner: Rc<CameraInner>,
    options: VideoOptions,
    recording: Rc<Cell<bool>>,
    started: bool,
    frame_count: u64,
    info: Option<FrameInfo>,
    started_at: Instant,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Video {
    pub path: PathBuf,
    pub frame_count: u64,
    pub frame_rate: f32,
    pub width: u32,
    pub height: u32,
    pub pixel_type: u32,
    pub elapsed: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntValue {
    pub current: i64,
    pub max: i64,
    pub min: i64,
    pub increment: i64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloatValue {
    pub current: f32,
    pub max: f32,
    pub min: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumValue {
    pub current: u32,
    pub supported: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringValue {
    pub current: String,
    pub max_length: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Value,
    Base,
    Integer,
    Bool,
    Command,
    Float,
    String,
    Register,
    Category,
    Enum,
    EnumEntry,
    Port,
    Other(sys::MV_XML_InterfaceType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeValue {
    Int(IntValue),
    Float(FloatValue),
    Enum(EnumValue),
    Bool(bool),
    String(StringValue),
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeInput {
    Int(i64),
    Float(f32),
    Bool(bool),
    EnumSymbol(String),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumSymbol(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeString(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferInfo {
    pub size: u64,
    pub alignment: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Roi {
    pub width: u32,
    pub height: u32,
    pub offset_x: u32,
    pub offset_y: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Fps {
    Target(f32),
    Free,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
    Angle90,
    Angle180,
    Angle270,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflectionDirection {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BayerInterpolation {
    Fast,
    Balanced,
    Optimal,
    OptimalPlus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BayerOptions {
    pub interpolation: BayerInterpolation,
    pub smoothing: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CcmOptions {
    pub enabled: bool,
    pub matrix: [i32; 9],
    pub scale: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrabStrategy {
    OneByOne,
    LatestOnly,
    LatestImages,
    Upcoming,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamOptions {
    pub image_nodes: u32,
    pub strategy: GrabStrategy,
    pub queue_size: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Bmp,
    Jpeg,
    Png,
    Tiff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SaveOptions {
    pub format: ImageFormat,
    pub quality: u32,
    pub method: i32,
    pub endian: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VideoOptions {
    pub path: PathBuf,
    pub format: VideoFormat,
    pub frame_rate: f32,
    pub bit_rate: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoFormat {
    Avi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoMode {
    Off,
    Once,
    Continuous,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Trigger {
    Off,
    Software,
    Source(String),
}

#[derive(Debug)]
struct CameraInner {
    handle: NonNull<c_void>,
    closed: Cell<bool>,
}

impl<'hik> Camera<'hik> {
    pub(crate) fn from_handle(handle: NonNull<c_void>) -> Self {
        Self {
            inner: Some(Rc::new(CameraInner {
                handle,
                closed: Cell::new(false),
            })),
            _hik: PhantomData,
        }
    }

    pub fn stream(mut self) -> Result<Stream<'hik>> {
        check(unsafe { sys::MV_CC_StartGrabbing(self.handle()) })?;

        let inner = self.take_inner();
        Ok(Stream::new(inner))
    }

    pub fn close(mut self) -> Result<()> {
        let inner = self.take_inner();
        inner.close()
    }

    pub fn raw_handle(&self) -> *mut c_void {
        self.handle()
    }

    pub fn take_image<P: Into<PathBuf>>(&mut self, path: P) -> Result<Frame> {
        let mut stream = self.start_stream()?;
        let frame = stream.take_frame(Duration::from_secs(1))?;
        stream.save_image(path)?.write_frame(&frame)?;
        stream.stop_grabbing()?;
        Ok(frame)
    }

    pub fn take_video<P: Into<PathBuf>>(
        &mut self,
        path: P,
        duration: Duration,
        frame_rate: f32,
    ) -> Result<Video> {
        validate_duration("video duration", duration)?;
        validate_frame_rate("video frame rate", frame_rate)?;

        let mut stream = self.start_stream()?;
        let mut output = stream.save_video(path, frame_rate)?;
        let timeout = Duration::from_secs(1);
        let started = Instant::now();

        while started.elapsed() < duration {
            let frame = stream.take_frame(timeout)?;
            output.write_frame(&frame)?;
        }

        let video = output.finish()?;
        stream.stop_grabbing()?;
        Ok(video)
    }

    pub fn get_exposure(&self) -> Result<FloatValue> {
        self.get_node("ExposureTime")?.into_float()
    }

    pub fn set_exposure(&mut self, value: f32) -> Result<()> {
        self.set_node("ExposureTime", value)
    }

    pub fn get_exposure_auto(&self) -> Result<EnumValue> {
        self.get_node("ExposureAuto")?.into_enum()
    }

    pub fn set_exposure_auto(&mut self, mode: AutoMode) -> Result<()> {
        self.set_node("ExposureAuto", mode.symbol())
    }

    pub fn get_gain(&self) -> Result<FloatValue> {
        self.get_node("Gain")?.into_float()
    }

    pub fn set_gain(&mut self, value: f32) -> Result<()> {
        self.set_node("Gain", value)
    }

    pub fn get_gain_auto(&self) -> Result<EnumValue> {
        self.get_node("GainAuto")?.into_enum()
    }

    pub fn set_gain_auto(&mut self, mode: AutoMode) -> Result<()> {
        self.set_node("GainAuto", mode.symbol())
    }

    pub fn get_roi(&self) -> Result<Roi> {
        Ok(Roi {
            width: self.get_node("Width")?.into_int()?.current as u32,
            height: self.get_node("Height")?.into_int()?.current as u32,
            offset_x: self.get_node("OffsetX")?.into_int()?.current as u32,
            offset_y: self.get_node("OffsetY")?.into_int()?.current as u32,
        })
    }

    pub fn set_roi(&mut self, roi: Roi) -> Result<()> {
        validate_roi(roi)?;

        self.set_node("Width", roi.width)?;
        self.set_node("Height", roi.height)?;
        self.set_node("OffsetX", roi.offset_x)?;
        self.set_node("OffsetY", roi.offset_y)
    }

    pub fn get_trigger(&self) -> Result<Trigger> {
        let mode = self.get_node("TriggerMode")?.into_enum()?.current;
        let mode = self.convert_enum_to_text("TriggerMode", mode)?;
        if mode == "Off" {
            return Ok(Trigger::Off);
        }

        let source = self.get_node("TriggerSource")?.into_enum()?.current;
        let source = self.convert_enum_to_text("TriggerSource", source)?;
        if source == "Software" {
            Ok(Trigger::Software)
        } else {
            Ok(Trigger::Source(source))
        }
    }

    pub fn set_trigger(&mut self, trigger: Trigger) -> Result<()> {
        match trigger {
            Trigger::Off => self.set_node("TriggerMode", "Off"),
            Trigger::Software => {
                self.set_node("TriggerMode", "On")?;
                self.set_node("TriggerSource", "Software")
            }
            Trigger::Source(source) => {
                self.set_node("TriggerMode", "On")?;
                self.set_node("TriggerSource", source)
            }
        }
    }

    pub fn get_fps(&self) -> Result<Fps> {
        let enabled = self.get_node("AcquisitionFrameRateEnable")?.into_bool()?;
        if enabled {
            Ok(Fps::Target(
                self.get_node("AcquisitionFrameRate")?.into_float()?.current,
            ))
        } else {
            Ok(Fps::Free)
        }
    }

    pub fn set_fps(&mut self, fps: Fps) -> Result<()> {
        match fps {
            Fps::Target(value) => {
                validate_frame_rate("target fps", value)?;
                self.set_node("AcquisitionFrameRateEnable", true)?;
                self.set_node("AcquisitionFrameRate", value)
            }
            Fps::Free => self.set_node("AcquisitionFrameRateEnable", false),
        }
    }

    pub fn node_type(&self, key: &str) -> Result<NodeType> {
        node_type(self.handle(), key)
    }

    pub fn get_node(&self, key: &str) -> Result<NodeValue> {
        match self.node_type(key)? {
            NodeType::Integer => get_int_node(self.handle(), key).map(NodeValue::Int),
            NodeType::Bool => get_bool_node(self.handle(), key).map(NodeValue::Bool),
            NodeType::Float => get_float_node(self.handle(), key).map(NodeValue::Float),
            NodeType::String => get_string_node(self.handle(), key).map(NodeValue::String),
            NodeType::Enum => get_enum_node(self.handle(), key).map(NodeValue::Enum),
            kind => Err(Error::unsupported_node(key, kind.name())),
        }
    }

    pub fn set_node<V: Into<NodeInput>>(&mut self, key: &str, value: V) -> Result<()> {
        let kind = self.node_type(key)?;
        let value = value.into();

        match (kind, value) {
            (NodeType::Integer, NodeInput::Int(value)) => set_int_node(self.handle(), key, value),
            (NodeType::Float, NodeInput::Float(value)) => set_float_node(self.handle(), key, value),
            (NodeType::Bool, NodeInput::Bool(value)) => set_bool_node(self.handle(), key, value),
            (NodeType::String, NodeInput::String(value)) => {
                set_string_node(self.handle(), key, value.as_str())
            }
            (NodeType::Enum, NodeInput::Int(value)) if value >= 0 && value <= u32::MAX as i64 => {
                set_enum_node(self.handle(), key, value as u32)
            }
            (NodeType::Enum, NodeInput::EnumSymbol(value)) => {
                select_enum_with_text(self.handle(), key, value.as_str())
            }
            (NodeType::Command, _) => Err(Error::unsupported_node(key, kind.name())),
            (kind, value) => Err(Error::node_input_mismatch(
                key,
                kind.input_name(),
                value.name(),
            )),
        }
    }

    pub fn convert_enum_to_text(&self, key: &str, value: u32) -> Result<String> {
        convert_enum_to_text(self.handle(), key, value)
    }

    pub fn execute_node(&mut self, key: &str) -> Result<()> {
        let key = key_string(key, "node key")?;
        check(unsafe { sys::MV_CC_SetCommandValue(self.handle(), key.as_ptr()) })
    }

    pub fn set_stream(&mut self, options: StreamOptions) -> Result<()> {
        check(unsafe { sys::MV_CC_SetImageNodeNum(self.handle(), options.image_nodes) })?;
        check(unsafe { sys::MV_CC_SetGrabStrategy(self.handle(), options.strategy.raw()) })?;

        if let Some(queue_size) = options.queue_size {
            check(unsafe { sys::MV_CC_SetOutputQueueSize(self.handle(), queue_size) })?;
        }

        Ok(())
    }

    pub fn set_bayer_conversion(&mut self, options: BayerOptions) -> Result<()> {
        check(unsafe {
            sys::MV_CC_SetBayerCvtQuality(self.handle(), options.interpolation.raw())
        })?;
        check(unsafe {
            sys::MV_CC_SetBayerFilterEnable(self.handle(), bool_value(options.smoothing))
        })
    }

    pub fn set_gamma(&mut self, value: f32) -> Result<()> {
        let pixel_type = self.get_node("PixelFormat")?.into_enum()?.current;
        check(unsafe {
            sys::MV_CC_SetGammaValue(self.handle(), pixel_type as sys::MvGvspPixelType, value)
        })
    }

    pub fn set_bayer_ccm(&mut self, options: CcmOptions) -> Result<()> {
        let mut param = sys::MV_CC_CCM_PARAM_EX {
            bCCMEnable: bool_value(options.enabled),
            nCCMat: options.matrix,
            nCCMScale: options.scale,
            nRes: [0; 8],
        };

        check(unsafe { sys::MV_CC_SetBayerCCMParamEx(self.handle(), &mut param) })
    }

    pub fn get_buffer_info(&self) -> Result<BufferInfo> {
        let mut size = 0;
        let mut alignment = 0;
        check(unsafe { sys::MV_CC_GetPayloadSize(self.handle(), &mut size, &mut alignment) })?;
        Ok(BufferInfo { size, alignment })
    }

    pub unsafe fn register_buffer(&mut self, buffer: *mut c_void, size: u64) -> Result<()> {
        check(unsafe {
            sys::MV_CC_RegisterBuffer(self.handle(), buffer, size, std::ptr::null_mut())
        })
    }

    pub unsafe fn unregister_buffer(&mut self, buffer: *mut c_void) -> Result<()> {
        check(unsafe { sys::MV_CC_UnRegisterBuffer(self.handle(), buffer) })
    }

    fn take_inner(&mut self) -> Rc<CameraInner> {
        self.inner
            .take()
            .expect("camera inner state should be present")
    }

    fn inner(&self) -> Rc<CameraInner> {
        self.inner
            .as_ref()
            .expect("camera inner state should be present")
            .clone()
    }

    fn handle(&self) -> *mut c_void {
        self.inner
            .as_ref()
            .expect("camera inner state should be present")
            .handle
            .as_ptr()
    }

    fn start_stream(&self) -> Result<Stream<'hik>> {
        check(unsafe { sys::MV_CC_StartGrabbing(self.handle()) })?;
        Ok(Stream::new(self.inner()))
    }
}

impl Drop for Camera<'_> {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            let _ = inner.close();
        }
    }
}

impl<'hik> Stream<'hik> {
    fn new(inner: Rc<CameraInner>) -> Self {
        Self {
            inner: Some(inner),
            recording: Rc::new(Cell::new(false)),
            _hik: PhantomData,
        }
    }

    pub fn take_frame(&mut self, timeout: Duration) -> Result<Frame> {
        let mut frame = MaybeUninit::<sys::MV_FRAME_OUT>::zeroed();
        let timeout_ms = timeout_ms(timeout)?;

        check(unsafe { sys::MV_CC_GetImageBuffer(self.handle(), frame.as_mut_ptr(), timeout_ms) })?;

        let mut frame = unsafe { frame.assume_init() };
        let info = FrameInfo::from_raw(&frame.stFrameInfo);
        let data = if frame.pBufAddr.is_null() || info.frame_len == 0 {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(frame.pBufAddr, info.frame_len as usize).to_vec() }
        };

        check(unsafe { sys::MV_CC_FreeImageBuffer(self.handle(), &mut frame) })?;

        Ok(Frame { info, data })
    }

    pub fn raw_handle(&self) -> *mut c_void {
        self.handle()
    }

    pub fn clear_buffer(&mut self) -> Result<()> {
        check(unsafe { sys::MV_CC_ClearImageBuffer(self.handle()) })
    }

    pub fn get_image_count(&self) -> Result<u32> {
        let mut count = 0;
        check(unsafe { sys::MV_CC_GetValidImageNum(self.handle(), &mut count) })?;
        Ok(count)
    }

    pub fn save_image<P: Into<PathBuf>>(&self, path: P) -> Result<ImageWriter> {
        Ok(ImageWriter::new(
            self.inner(),
            path.into(),
            SaveOptions::default(),
        ))
    }

    pub fn save_image_with<P: Into<PathBuf>>(
        &self,
        path: P,
        options: SaveOptions,
    ) -> Result<ImageWriter> {
        Ok(ImageWriter::new(self.inner(), path.into(), options))
    }

    pub fn save_video<P: Into<PathBuf>>(&self, path: P, frame_rate: f32) -> Result<VideoWriter> {
        VideoWriter::new(
            self.inner(),
            self.recording.clone(),
            VideoOptions::new(path, frame_rate),
        )
    }

    pub fn save_video_with(&self, options: VideoOptions) -> Result<VideoWriter> {
        VideoWriter::new(self.inner(), self.recording.clone(), options)
    }

    pub fn encode_frame(&self, frame: &Frame, options: SaveOptions) -> Result<Vec<u8>> {
        encode_image(self.handle(), frame, options)
    }

    pub fn convert_frame(&self, frame: &Frame, pixel_type: u32) -> Result<Frame> {
        convert_image(self.handle(), frame, pixel_type)
    }

    pub fn rotate_frame(&self, frame: &Frame, rotation: Rotation) -> Result<Frame> {
        rotate_image(self.handle(), frame, rotation)
    }

    pub fn reflect_frame(&self, frame: &Frame, direction: ReflectionDirection) -> Result<Frame> {
        reflect_image(self.handle(), frame, direction)
    }

    pub fn contrast_frame(&self, frame: &Frame, factor: u32) -> Result<Frame> {
        contrast_image(self.handle(), frame, factor)
    }

    pub fn hb_decode(&self, frame: &Frame) -> Result<Frame> {
        hb_decode(self.handle(), frame, decode_size(frame)?)
    }

    pub fn hb_decode_with(&self, frame: &Frame, buffer_size: u32) -> Result<Frame> {
        hb_decode(self.handle(), frame, buffer_size)
    }

    pub fn stop(mut self) -> Result<Camera<'hik>> {
        let inner = self.stop_inner()?;
        Ok(Camera {
            inner: Some(inner),
            _hik: PhantomData,
        })
    }

    fn stop_grabbing(&mut self) -> Result<()> {
        let _ = self.stop_inner()?;
        Ok(())
    }

    fn stop_inner(&mut self) -> Result<Rc<CameraInner>> {
        if self.recording.get() {
            check(unsafe { sys::MV_CC_StopRecord(self.handle()) })?;
            self.recording.set(false);
        }

        check(unsafe { sys::MV_CC_StopGrabbing(self.handle()) })?;
        Ok(self.take_inner())
    }

    fn take_inner(&mut self) -> Rc<CameraInner> {
        self.inner
            .take()
            .expect("stream inner state should be present")
    }

    fn inner(&self) -> Rc<CameraInner> {
        self.inner
            .as_ref()
            .expect("stream inner state should be present")
            .clone()
    }

    fn handle(&self) -> *mut c_void {
        self.inner
            .as_ref()
            .expect("stream inner state should be present")
            .handle
            .as_ptr()
    }
}

impl Drop for Stream<'_> {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            unsafe {
                if self.recording.get() {
                    sys::MV_CC_StopRecord(inner.handle.as_ptr());
                    self.recording.set(false);
                }
                sys::MV_CC_StopGrabbing(inner.handle.as_ptr());
            }
        }
    }
}

impl ImageWriter {
    fn new(inner: Rc<CameraInner>, path: PathBuf, options: SaveOptions) -> Self {
        Self {
            inner,
            path,
            options,
        }
    }

    pub fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        save_image(self.handle(), frame, &self.path, self.options)
    }

    pub fn finish(self) -> Result<()> {
        Ok(())
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn handle(&self) -> *mut c_void {
        self.inner.handle.as_ptr()
    }
}

impl VideoWriter {
    fn new(
        inner: Rc<CameraInner>,
        recording: Rc<Cell<bool>>,
        options: VideoOptions,
    ) -> Result<Self> {
        validate_frame_rate("video frame rate", options.frame_rate)?;

        Ok(Self {
            inner,
            options,
            recording,
            started: false,
            frame_count: 0,
            info: None,
            started_at: Instant::now(),
        })
    }

    pub fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        if !self.started {
            self.start(frame)?;
        }

        let mut raw = sys::MV_CC_INPUT_FRAME_INFO_EX {
            enPixelType: frame.info.pixel_type as sys::MvGvspPixelType,
            nWidth: frame.info.width,
            nHeight: frame.info.height,
            pData: frame.data.as_ptr() as *mut _,
            nDataLen: frame.data.len() as u64,
            nRes: [0; 8],
        };

        check(unsafe { sys::MV_CC_InputOneFrameEx(self.handle(), &mut raw) })?;
        self.frame_count += 1;
        Ok(())
    }

    pub fn finish(mut self) -> Result<Video> {
        if self.frame_count == 0 {
            return Err(Error::empty_video());
        }

        self.stop()?;
        let info = self.info.clone();

        Ok(Video {
            path: self.options.path.clone(),
            frame_count: self.frame_count,
            frame_rate: self.options.frame_rate,
            width: info.as_ref().map(|info| info.width).unwrap_or_default(),
            height: info.as_ref().map(|info| info.height).unwrap_or_default(),
            pixel_type: info
                .as_ref()
                .map(|info| info.pixel_type)
                .unwrap_or_default(),
            elapsed: self.started_at.elapsed(),
        })
    }

    pub fn path(&self) -> &Path {
        &self.options.path
    }

    fn start(&mut self, frame: &Frame) -> Result<()> {
        if self.recording.get() {
            return Err(Error::recording_in_progress());
        }
        validate_frame(frame)?;

        let path = path_string(&self.options.path)?;
        let mut param = sys::MV_CC_RECORD_PARAM {
            enPixelType: frame.info.pixel_type as sys::MvGvspPixelType,
            nWidth: u16_value(frame.info.width)?,
            nHeight: u16_value(frame.info.height)?,
            fFrameRate: self.options.frame_rate,
            nBitRate: self.options.bit_rate,
            enRecordFmtType: self.options.format.raw(),
            strFilePath: path.as_ptr() as *mut _,
            nRes: [0; 8],
        };

        check(unsafe { sys::MV_CC_StartRecord(self.handle(), &mut param) })?;
        self.recording.set(true);
        self.started = true;
        self.info = Some(frame.info.clone());
        self.started_at = Instant::now();
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        if self.started {
            check(unsafe { sys::MV_CC_StopRecord(self.handle()) })?;
            self.recording.set(false);
            self.started = false;
        }

        Ok(())
    }

    fn handle(&self) -> *mut c_void {
        self.inner.handle.as_ptr()
    }
}

impl Drop for VideoWriter {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

impl CameraInner {
    fn close(&self) -> Result<()> {
        if self.closed.replace(true) {
            return Ok(());
        }

        let close = check(unsafe { sys::MV_CC_CloseDevice(self.handle.as_ptr()) });
        let destroy = check(unsafe { sys::MV_CC_DestroyHandle(self.handle.as_ptr()) });

        close?;
        destroy?;
        Ok(())
    }
}

impl Drop for CameraInner {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

impl GrabStrategy {
    fn raw(self) -> sys::MV_GRAB_STRATEGY {
        match self {
            Self::OneByOne => sys::_MV_GRAB_STRATEGY__MV_GrabStrategy_OneByOne,
            Self::LatestOnly => sys::_MV_GRAB_STRATEGY__MV_GrabStrategy_LatestImagesOnly,
            Self::LatestImages => sys::_MV_GRAB_STRATEGY__MV_GrabStrategy_LatestImages,
            Self::Upcoming => sys::_MV_GRAB_STRATEGY__MV_GrabStrategy_UpcomingImage,
        }
    }
}

impl ImageFormat {
    fn raw(self) -> sys::MV_SAVE_IAMGE_TYPE {
        match self {
            Self::Bmp => sys::MV_SAVE_IAMGE_TYPE_MV_Image_Bmp,
            Self::Jpeg => sys::MV_SAVE_IAMGE_TYPE_MV_Image_Jpeg,
            Self::Png => sys::MV_SAVE_IAMGE_TYPE_MV_Image_Png,
            Self::Tiff => sys::MV_SAVE_IAMGE_TYPE_MV_Image_Tif,
        }
    }
}

impl Rotation {
    fn raw(self) -> sys::MV_IMG_ROTATION_ANGLE {
        match self {
            Self::Angle90 => sys::_MV_IMG_ROTATION_ANGLE__MV_IMAGE_ROTATE_90,
            Self::Angle180 => sys::_MV_IMG_ROTATION_ANGLE__MV_IMAGE_ROTATE_180,
            Self::Angle270 => sys::_MV_IMG_ROTATION_ANGLE__MV_IMAGE_ROTATE_270,
        }
    }
}

impl ReflectionDirection {
    fn raw(self) -> sys::MV_IMG_FLIP_TYPE {
        match self {
            Self::Vertical => sys::_MV_IMG_FLIP_TYPE__MV_FLIP_VERTICAL,
            Self::Horizontal => sys::_MV_IMG_FLIP_TYPE__MV_FLIP_HORIZONTAL,
        }
    }
}

impl BayerInterpolation {
    fn raw(self) -> u32 {
        match self {
            Self::Fast => 0,
            Self::Balanced => 1,
            Self::Optimal => 2,
            Self::OptimalPlus => 3,
        }
    }
}

impl Default for BayerOptions {
    fn default() -> Self {
        Self {
            interpolation: BayerInterpolation::Balanced,
            smoothing: false,
        }
    }
}

impl Default for CcmOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            matrix: [0; 9],
            scale: 1024,
        }
    }
}

impl VideoFormat {
    fn raw(self) -> sys::MV_RECORD_FORMAT_TYPE {
        match self {
            Self::Avi => sys::_MV_RECORD_FORMAT_TYPE__MV_FormatType_AVI,
        }
    }
}

impl VideoOptions {
    pub fn new<P: Into<PathBuf>>(path: P, frame_rate: f32) -> Self {
        Self {
            path: path.into(),
            format: VideoFormat::Avi,
            frame_rate,
            bit_rate: 8 * 1024,
        }
    }

    pub fn format(mut self, format: VideoFormat) -> Self {
        self.format = format;
        self
    }

    pub fn bit_rate(mut self, bit_rate: u32) -> Self {
        self.bit_rate = bit_rate;
        self
    }
}

impl SaveOptions {
    pub fn new(format: ImageFormat) -> Self {
        Self {
            format,
            quality: 90,
            method: 1,
            endian: 0,
        }
    }
}

impl AutoMode {
    fn symbol(self) -> &'static str {
        match self {
            Self::Off => "Off",
            Self::Once => "Once",
            Self::Continuous => "Continuous",
        }
    }
}

impl Trigger {
    pub fn source(value: impl Into<String>) -> Self {
        Self::Source(value.into())
    }
}

impl EnumSymbol {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl NodeString {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl Default for SaveOptions {
    fn default() -> Self {
        Self::new(ImageFormat::Bmp)
    }
}

impl FrameInfo {
    fn from_raw(raw: &sys::MV_FRAME_OUT_INFO_EX) -> Self {
        let width = if raw.nExtendWidth > 0 {
            raw.nExtendWidth
        } else {
            raw.nWidth as u32
        };
        let height = if raw.nExtendHeight > 0 {
            raw.nExtendHeight
        } else {
            raw.nHeight as u32
        };
        let frame_len = if raw.nFrameLenEx > 0 {
            raw.nFrameLenEx
        } else {
            raw.nFrameLen as u64
        };

        Self {
            width,
            height,
            pixel_type: raw.enPixelType as u32,
            frame_len,
            frame_num: raw.nFrameNum,
            device_timestamp: ((raw.nDevTimeStampHigh as u64) << 32) | raw.nDevTimeStampLow as u64,
            host_timestamp: raw.nHostTimeStamp,
            second_count: raw.nSecondCount,
            cycle_count: raw.nCycleCount,
            cycle_offset: raw.nCycleOffset,
            gain: raw.fGain,
            exposure_time: raw.fExposureTime,
            average_brightness: raw.nAverageBrightness,
            red: raw.nRed,
            green: raw.nGreen,
            blue: raw.nBlue,
            frame_counter: raw.nFrameCounter,
            trigger_index: raw.nTriggerIndex,
            input: raw.nInput,
            output: raw.nOutput,
            offset_x: raw.nOffsetX as u32,
            offset_y: raw.nOffsetY as u32,
            chunk_width: raw.nChunkWidth as u32,
            chunk_height: raw.nChunkHeight as u32,
            lost_packet: raw.nLostPacket,
            unparsed_chunk_num: raw.nUnparsedChunkNum,
            extra_type: raw.nExtraType,
            sub_image_num: raw.nSubImageNum,
            first_encoder_count: raw.nFirstLineEncoderCount,
            last_encoder_count: raw.nLastLineEncoderCount,
            last_frame_flag: raw.nLastFrameFlag,
        }
    }
}

impl NodeType {
    fn from_raw(raw: sys::MV_XML_InterfaceType) -> Self {
        match raw {
            sys::MV_XML_InterfaceType_IFT_IValue => Self::Value,
            sys::MV_XML_InterfaceType_IFT_IBase => Self::Base,
            sys::MV_XML_InterfaceType_IFT_IInteger => Self::Integer,
            sys::MV_XML_InterfaceType_IFT_IBoolean => Self::Bool,
            sys::MV_XML_InterfaceType_IFT_ICommand => Self::Command,
            sys::MV_XML_InterfaceType_IFT_IFloat => Self::Float,
            sys::MV_XML_InterfaceType_IFT_IString => Self::String,
            sys::MV_XML_InterfaceType_IFT_IRegister => Self::Register,
            sys::MV_XML_InterfaceType_IFT_ICategory => Self::Category,
            sys::MV_XML_InterfaceType_IFT_IEnumeration => Self::Enum,
            sys::MV_XML_InterfaceType_IFT_IEnumEntry => Self::EnumEntry,
            sys::MV_XML_InterfaceType_IFT_IPort => Self::Port,
            other => Self::Other(other),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Value => "Value",
            Self::Base => "Base",
            Self::Integer => "Integer",
            Self::Bool => "Bool",
            Self::Command => "Command",
            Self::Float => "Float",
            Self::String => "String",
            Self::Register => "Register",
            Self::Category => "Category",
            Self::Enum => "Enum",
            Self::EnumEntry => "EnumEntry",
            Self::Port => "Port",
            Self::Other(_) => "Other",
        }
    }

    fn input_name(&self) -> &'static str {
        match self {
            Self::Integer => "Int",
            Self::Bool => "Bool",
            Self::Float => "Float",
            Self::String => "NodeString",
            Self::Enum => "Int or EnumSymbol",
            Self::Command => "execute_node",
            _ => self.name(),
        }
    }
}

impl NodeValue {
    pub fn as_int(&self) -> Result<&IntValue> {
        match self {
            Self::Int(value) => Ok(value),
            other => Err(Error::node_value_mismatch("Int", other.name())),
        }
    }

    pub fn into_int(self) -> Result<IntValue> {
        match self {
            Self::Int(value) => Ok(value),
            other => Err(Error::node_value_mismatch("Int", other.name())),
        }
    }

    pub fn as_float(&self) -> Result<&FloatValue> {
        match self {
            Self::Float(value) => Ok(value),
            other => Err(Error::node_value_mismatch("Float", other.name())),
        }
    }

    pub fn into_float(self) -> Result<FloatValue> {
        match self {
            Self::Float(value) => Ok(value),
            other => Err(Error::node_value_mismatch("Float", other.name())),
        }
    }

    pub fn as_enum(&self) -> Result<&EnumValue> {
        match self {
            Self::Enum(value) => Ok(value),
            other => Err(Error::node_value_mismatch("Enum", other.name())),
        }
    }

    pub fn into_enum(self) -> Result<EnumValue> {
        match self {
            Self::Enum(value) => Ok(value),
            other => Err(Error::node_value_mismatch("Enum", other.name())),
        }
    }

    pub fn as_bool(&self) -> Result<bool> {
        match self {
            Self::Bool(value) => Ok(*value),
            other => Err(Error::node_value_mismatch("Bool", other.name())),
        }
    }

    pub fn into_bool(self) -> Result<bool> {
        match self {
            Self::Bool(value) => Ok(value),
            other => Err(Error::node_value_mismatch("Bool", other.name())),
        }
    }

    pub fn as_string(&self) -> Result<&StringValue> {
        match self {
            Self::String(value) => Ok(value),
            other => Err(Error::node_value_mismatch("String", other.name())),
        }
    }

    pub fn into_string(self) -> Result<StringValue> {
        match self {
            Self::String(value) => Ok(value),
            other => Err(Error::node_value_mismatch("String", other.name())),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Int(_) => "Int",
            Self::Float(_) => "Float",
            Self::Enum(_) => "Enum",
            Self::Bool(_) => "Bool",
            Self::String(_) => "String",
        }
    }
}

impl NodeInput {
    fn name(&self) -> &'static str {
        match self {
            Self::Int(_) => "Int",
            Self::Float(_) => "Float",
            Self::Bool(_) => "Bool",
            Self::EnumSymbol(_) => "EnumSymbol",
            Self::String(_) => "NodeString",
        }
    }
}

impl From<i32> for NodeInput {
    fn from(value: i32) -> Self {
        Self::Int(value as i64)
    }
}

impl From<i64> for NodeInput {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<u32> for NodeInput {
    fn from(value: u32) -> Self {
        Self::Int(value as i64)
    }
}

impl From<f32> for NodeInput {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<f64> for NodeInput {
    fn from(value: f64) -> Self {
        Self::Float(value as f32)
    }
}

impl From<bool> for NodeInput {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<&str> for NodeInput {
    fn from(value: &str) -> Self {
        Self::EnumSymbol(value.to_owned())
    }
}

impl From<String> for NodeInput {
    fn from(value: String) -> Self {
        Self::EnumSymbol(value)
    }
}

impl From<EnumSymbol> for NodeInput {
    fn from(value: EnumSymbol) -> Self {
        Self::EnumSymbol(value.0)
    }
}

impl From<NodeString> for NodeInput {
    fn from(value: NodeString) -> Self {
        Self::String(value.0)
    }
}

fn node_type(handle: *mut c_void, key: &str) -> Result<NodeType> {
    let key = key_string(key, "node key")?;
    let mut kind = sys::MV_XML_InterfaceType_IFT_IValue;
    check(unsafe { sys::MV_XML_GetNodeInterfaceType(handle, key.as_ptr(), &mut kind) })?;
    Ok(NodeType::from_raw(kind))
}

fn get_int_node(handle: *mut c_void, key: &str) -> Result<IntValue> {
    let key = key_string(key, "node key")?;
    let mut value = MaybeUninit::<sys::MVCC_INTVALUE_EX>::zeroed();

    check(unsafe { sys::MV_CC_GetIntValueEx(handle, key.as_ptr(), value.as_mut_ptr()) })?;

    let value = unsafe { value.assume_init() };
    Ok(IntValue {
        current: value.nCurValue,
        max: value.nMax,
        min: value.nMin,
        increment: value.nInc,
    })
}

fn set_int_node(handle: *mut c_void, key: &str, value: i64) -> Result<()> {
    let key = key_string(key, "node key")?;
    check(unsafe { sys::MV_CC_SetIntValueEx(handle, key.as_ptr(), value) })
}

fn get_float_node(handle: *mut c_void, key: &str) -> Result<FloatValue> {
    let key = key_string(key, "node key")?;
    let mut value = MaybeUninit::<sys::MVCC_FLOATVALUE>::zeroed();

    check(unsafe { sys::MV_CC_GetFloatValue(handle, key.as_ptr(), value.as_mut_ptr()) })?;

    let value = unsafe { value.assume_init() };
    Ok(FloatValue {
        current: value.fCurValue,
        max: value.fMax,
        min: value.fMin,
    })
}

fn set_float_node(handle: *mut c_void, key: &str, value: f32) -> Result<()> {
    let key = key_string(key, "node key")?;
    check(unsafe { sys::MV_CC_SetFloatValue(handle, key.as_ptr(), value) })
}

fn get_enum_node(handle: *mut c_void, key: &str) -> Result<EnumValue> {
    let key = key_string(key, "node key")?;
    let mut value = MaybeUninit::<sys::MVCC_ENUMVALUE_EX>::zeroed();

    check(unsafe { sys::MV_CC_GetEnumValueEx(handle, key.as_ptr(), value.as_mut_ptr()) })?;

    let value = unsafe { value.assume_init() };
    let count = value.nSupportedNum.min(value.nSupportValue.len() as u32) as usize;

    Ok(EnumValue {
        current: value.nCurValue,
        supported: value.nSupportValue[..count].to_vec(),
    })
}

fn set_enum_node(handle: *mut c_void, key: &str, value: u32) -> Result<()> {
    let key = key_string(key, "node key")?;
    check(unsafe { sys::MV_CC_SetEnumValue(handle, key.as_ptr(), value) })
}

fn select_enum_with_text(handle: *mut c_void, key: &str, text: &str) -> Result<()> {
    let key = key_string(key, "node key")?;
    let text = key_string(text, "node text")?;
    check(unsafe { sys::MV_CC_SetEnumValueByString(handle, key.as_ptr(), text.as_ptr()) })
}

fn convert_enum_to_text(handle: *mut c_void, key: &str, value: u32) -> Result<String> {
    let key = key_string(key, "node key")?;
    let entry = MaybeUninit::<sys::MVCC_ENUMENTRY>::zeroed();
    let mut entry = unsafe { entry.assume_init() };
    entry.nValue = value;

    check(unsafe { sys::MV_CC_GetEnumEntrySymbolic(handle, key.as_ptr(), &mut entry) })?;

    Ok(c_text(&entry.chSymbolic))
}

fn get_bool_node(handle: *mut c_void, key: &str) -> Result<bool> {
    let key = key_string(key, "node key")?;
    let mut value: sys::bool_ = 0;

    check(unsafe { sys::MV_CC_GetBoolValue(handle, key.as_ptr(), &mut value) })?;

    Ok(value != 0)
}

fn set_bool_node(handle: *mut c_void, key: &str, value: bool) -> Result<()> {
    let key = key_string(key, "node key")?;
    check(unsafe { sys::MV_CC_SetBoolValue(handle, key.as_ptr(), bool_value(value)) })
}

fn get_string_node(handle: *mut c_void, key: &str) -> Result<StringValue> {
    let key = key_string(key, "node key")?;
    let mut value = MaybeUninit::<sys::MVCC_STRINGVALUE>::zeroed();

    check(unsafe { sys::MV_CC_GetStringValue(handle, key.as_ptr(), value.as_mut_ptr()) })?;

    let value = unsafe { value.assume_init() };
    Ok(StringValue {
        current: c_text(&value.chCurValue),
        max_length: value.nMaxLength,
    })
}

fn set_string_node(handle: *mut c_void, key: &str, value: &str) -> Result<()> {
    let key = key_string(key, "node key")?;
    let value = key_string(value, "node text")?;
    check(unsafe { sys::MV_CC_SetStringValue(handle, key.as_ptr(), value.as_ptr()) })
}

fn save_image(handle: *mut c_void, image: &Image, path: &Path, options: SaveOptions) -> Result<()> {
    validate_frame(image)?;

    let mut raw = raw_image(image);
    let mut param = sys::MV_CC_SAVE_IMAGE_PARAM {
        enImageType: options.format.raw(),
        nQuality: options.quality,
        iMethodValue: options.method,
        nEndian: options.endian,
        nReserved: [0; 7],
    };
    let path = path_string(path)?;

    check(unsafe { sys::MV_CC_SaveImageToFileEx2(handle, &mut raw, &mut param, path.as_ptr()) })
}

fn encode_image(handle: *mut c_void, image: &Image, options: SaveOptions) -> Result<Vec<u8>> {
    validate_frame(image)?;

    let data_len = len_u32(image.data.len())?;
    let buffer_size = encode_size(image)?;
    let mut output = vec![0; buffer_size as usize];

    let mut param = sys::MV_SAVE_IMAGE_PARAM_EX3 {
        pData: image.data.as_ptr() as *mut _,
        nDataLen: data_len,
        enPixelType: image.info.pixel_type as sys::MvGvspPixelType,
        nWidth: image.info.width,
        nHeight: image.info.height,
        pImageBuffer: output.as_mut_ptr(),
        nImageLen: 0,
        nBufferSize: buffer_size,
        enImageType: options.format.raw(),
        nJpgQuality: options.quality,
        iMethodValue: options.method as u32,
        nReserved: [0; 3],
    };

    check(unsafe { sys::MV_CC_SaveImageEx3(handle, &mut param) })?;
    output.truncate(param.nImageLen as usize);
    Ok(output)
}

fn convert_image(handle: *mut c_void, image: &Image, pixel_type: u32) -> Result<Image> {
    validate_frame(image)?;

    let data_len = len_u32(image.data.len())?;
    let buffer_size = pixel_size(image.info.width, image.info.height, pixel_type)?;
    let mut output = vec![0; buffer_size as usize];

    let mut param = sys::MV_CC_PIXEL_CONVERT_PARAM_EX {
        nWidth: image.info.width,
        nHeight: image.info.height,
        enSrcPixelType: image.info.pixel_type as sys::MvGvspPixelType,
        pSrcData: image.data.as_ptr() as *mut _,
        nSrcDataLen: data_len,
        enDstPixelType: pixel_type as sys::MvGvspPixelType,
        pDstBuffer: output.as_mut_ptr(),
        nDstLen: 0,
        nDstBufferSize: buffer_size,
        nRes: [0; 4],
    };

    check(unsafe { sys::MV_CC_ConvertPixelTypeEx(handle, &mut param) })?;
    output.truncate(param.nDstLen as usize);

    let mut info = image.info.clone();
    info.pixel_type = pixel_type;
    info.frame_len = param.nDstLen as u64;

    Ok(Image { info, data: output })
}

fn rotate_image(handle: *mut c_void, image: &Image, rotation: Rotation) -> Result<Image> {
    validate_frame(image)?;

    let data_len = len_u32(image.data.len())?;
    let buffer_size = data_len;
    let mut output = vec![0; buffer_size as usize];

    let mut param = sys::MV_CC_ROTATE_IMAGE_PARAM {
        enPixelType: image.info.pixel_type as sys::MvGvspPixelType,
        nWidth: image.info.width,
        nHeight: image.info.height,
        pSrcData: image.data.as_ptr() as *mut _,
        nSrcDataLen: data_len,
        pDstBuf: output.as_mut_ptr(),
        nDstBufLen: 0,
        nDstBufSize: buffer_size,
        enRotationAngle: rotation.raw(),
        nRes: [0; 8],
    };

    check(unsafe { sys::MV_CC_RotateImage(handle, &mut param) })?;
    output.truncate(param.nDstBufLen as usize);

    let mut info = image.info.clone();
    if matches!(rotation, Rotation::Angle90 | Rotation::Angle270) {
        std::mem::swap(&mut info.width, &mut info.height);
    }
    info.frame_len = param.nDstBufLen as u64;

    Ok(Image { info, data: output })
}

fn reflect_image(
    handle: *mut c_void,
    image: &Image,
    direction: ReflectionDirection,
) -> Result<Image> {
    validate_frame(image)?;

    let data_len = len_u32(image.data.len())?;
    let buffer_size = data_len;
    let mut output = vec![0; buffer_size as usize];

    let mut param = sys::MV_CC_FLIP_IMAGE_PARAM {
        enPixelType: image.info.pixel_type as sys::MvGvspPixelType,
        nWidth: image.info.width,
        nHeight: image.info.height,
        pSrcData: image.data.as_ptr() as *mut _,
        nSrcDataLen: data_len,
        pDstBuf: output.as_mut_ptr(),
        nDstBufLen: 0,
        nDstBufSize: buffer_size,
        enFlipType: direction.raw(),
        nRes: [0; 8],
    };

    check(unsafe { sys::MV_CC_FlipImage(handle, &mut param) })?;
    output.truncate(param.nDstBufLen as usize);

    let mut info = image.info.clone();
    info.frame_len = param.nDstBufLen as u64;

    Ok(Image { info, data: output })
}

fn contrast_image(handle: *mut c_void, image: &Image, factor: u32) -> Result<Image> {
    validate_frame(image)?;

    let data_len = len_u32(image.data.len())?;
    let buffer_size = data_len;
    let mut output = vec![0; buffer_size as usize];

    let mut param = sys::MV_CC_CONTRAST_PARAM {
        nWidth: image.info.width,
        nHeight: image.info.height,
        pSrcBuf: image.data.as_ptr() as *mut _,
        nSrcBufLen: data_len,
        enPixelType: image.info.pixel_type as sys::MvGvspPixelType,
        pDstBuf: output.as_mut_ptr(),
        nDstBufSize: buffer_size,
        nDstBufLen: 0,
        nContrastFactor: factor,
        nRes: [0; 8],
    };

    check(unsafe { sys::MV_CC_ImageContrast(handle, &mut param) })?;
    output.truncate(param.nDstBufLen as usize);

    let mut info = image.info.clone();
    info.frame_len = param.nDstBufLen as u64;

    Ok(Image { info, data: output })
}

fn hb_decode(handle: *mut c_void, image: &Image, buffer_size: u32) -> Result<Image> {
    validate_frame(image)?;

    let data_len = len_u32(image.data.len())?;
    let mut output = vec![0; buffer_size as usize];

    let mut param = sys::MV_CC_HB_DECODE_PARAM {
        pSrcBuf: image.data.as_ptr() as *mut _,
        nSrcLen: data_len,
        nWidth: 0,
        nHeight: 0,
        pDstBuf: output.as_mut_ptr(),
        nDstBufSize: buffer_size,
        nDstBufLen: 0,
        enDstPixelType: 0,
        stFrameSpecInfo: unsafe { MaybeUninit::zeroed().assume_init() },
        nRes: [0; 8],
    };

    check(unsafe { sys::MV_CC_HB_Decode(handle, &mut param) })?;
    output.truncate(param.nDstBufLen as usize);

    let mut info = image.info.clone();
    info.width = param.nWidth;
    info.height = param.nHeight;
    info.pixel_type = param.enDstPixelType as u32;
    info.frame_len = param.nDstBufLen as u64;
    info.second_count = param.stFrameSpecInfo.nSecondCount;
    info.cycle_count = param.stFrameSpecInfo.nCycleCount;
    info.cycle_offset = param.stFrameSpecInfo.nCycleOffset;
    info.gain = param.stFrameSpecInfo.fGain;
    info.exposure_time = param.stFrameSpecInfo.fExposureTime;
    info.average_brightness = param.stFrameSpecInfo.nAverageBrightness;
    info.red = param.stFrameSpecInfo.nRed;
    info.green = param.stFrameSpecInfo.nGreen;
    info.blue = param.stFrameSpecInfo.nBlue;
    info.frame_counter = param.stFrameSpecInfo.nFrameCounter;
    info.trigger_index = param.stFrameSpecInfo.nTriggerIndex;
    info.input = param.stFrameSpecInfo.nInput;
    info.output = param.stFrameSpecInfo.nOutput;
    info.offset_x = param.stFrameSpecInfo.nOffsetX as u32;
    info.offset_y = param.stFrameSpecInfo.nOffsetY as u32;
    info.chunk_width = param.stFrameSpecInfo.nFrameWidth as u32;
    info.chunk_height = param.stFrameSpecInfo.nFrameHeight as u32;

    Ok(Image { info, data: output })
}

fn raw_image(image: &Image) -> sys::MV_CC_IMAGE {
    sys::MV_CC_IMAGE {
        nWidth: image.info.width,
        nHeight: image.info.height,
        enPixelType: image.info.pixel_type as sys::MvGvspPixelType,
        pImageBuf: image.data.as_ptr() as *mut _,
        nImageBufSize: image.data.len() as u64,
        nImageLen: image.info.frame_len,
        nReserved: [0; 4],
    }
}

fn encode_size(image: &Image) -> Result<u32> {
    let raw_size = pixel_size(image.info.width, image.info.height, image.info.pixel_type)
        .unwrap_or_else(|_| image.data.len().min(u32::MAX as usize) as u32);
    let source_size = len_u32(image.data.len())?;
    raw_size
        .max(source_size)
        .checked_mul(4)
        .and_then(|value| value.checked_add(4096))
        .ok_or_else(|| uint32_error().into())
}

fn decode_size(image: &Image) -> Result<u32> {
    let raw_size = pixel_size(image.info.width, image.info.height, image.info.pixel_type)
        .unwrap_or_else(|_| image.data.len().min(u32::MAX as usize) as u32);

    raw_size
        .max(len_u32(image.data.len())?)
        .checked_mul(4)
        .ok_or_else(|| uint32_error().into())
}

fn pixel_size(width: u32, height: u32, pixel_type: u32) -> Result<u32> {
    let bits = (pixel_type >> 16) & 0xff;
    if bits == 0 {
        return Err(uint32_error().into());
    }

    let pixels = (width as u64)
        .checked_mul(height as u64)
        .ok_or_else(uint32_error)?;
    let bits = pixels.checked_mul(bits as u64).ok_or_else(uint32_error)?;
    let bytes = bits.checked_add(7).ok_or_else(uint32_error)? / 8;

    if bytes > u32::MAX as u64 {
        Err(uint32_error().into())
    } else {
        Ok(bytes as u32)
    }
}

fn u16_value(value: u32) -> Result<u16> {
    if value > u16::MAX as u32 {
        Err(Error::value_out_of_range("frame dimension"))
    } else {
        Ok(value as u16)
    }
}

fn validate_frame(frame: &Frame) -> Result<()> {
    if frame.data.is_empty() || frame.info.frame_len == 0 {
        Err(Error::empty_frame())
    } else {
        Ok(())
    }
}

fn validate_duration(field: &'static str, duration: Duration) -> Result<()> {
    if duration.is_zero() {
        Err(Error::invalid_duration(field))
    } else {
        Ok(())
    }
}

fn validate_frame_rate(field: &'static str, frame_rate: f32) -> Result<()> {
    if frame_rate.is_finite() && frame_rate > 0.0 {
        Ok(())
    } else {
        Err(Error::invalid_frame_rate(field))
    }
}

fn validate_roi(roi: Roi) -> Result<()> {
    if roi.width == 0 || roi.height == 0 {
        Err(Error::invalid_roi())
    } else {
        Ok(())
    }
}

fn bool_value(value: bool) -> sys::bool_ {
    if value { 1 } else { 0 }
}

fn timeout_ms(timeout: Duration) -> Result<u32> {
    let millis = timeout.as_millis();
    if millis > u32::MAX as u128 {
        Err(uint32_error().into())
    } else {
        Ok(millis as u32)
    }
}

fn len_u32(value: usize) -> Result<u32> {
    if value > u32::MAX as usize {
        Err(uint32_error().into())
    } else {
        Ok(value as u32)
    }
}

fn uint32_error() -> Error {
    Error::value_out_of_range("u32 value")
}

fn path_string(path: &Path) -> Result<CString> {
    CString::new(path.to_string_lossy().as_bytes()).map_err(|_| Error::invalid_string("path"))
}

fn key_string(value: &str, field: &'static str) -> Result<CString> {
    CString::new(value).map_err(|_| Error::invalid_string(field))
}

fn c_text(bytes: &[std::os::raw::c_char]) -> String {
    unsafe { CStr::from_ptr(bytes.as_ptr()) }
        .to_string_lossy()
        .trim()
        .to_owned()
}
