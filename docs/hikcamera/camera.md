源码文件：`crates/hikcamera/src/camera.rs`

这个文件负责打开后的相机、采集流、帧数据、图像处理、图片写入和视频写入

## 提供的结构体、类型和函数

- `Camera<'hik>`
  - 已打开的相机
  - 由 `Device::open()` 创建
  - 用于设置曝光、增益、触发、像素格式等参数
  - 可以直接拍照或录像
  - 可以进入显式采集流 `Stream`
  - 生命周期：
    - `Camera` 持有底层 C SDK handle
    - `Drop` 时会兜底关闭设备并销毁 handle
    - 示例代码里仍建议显式调用 `close()`
  - 高层拍摄方法：
    - `take_image<P: Into<PathBuf>>(&mut self, path: P) -> Result<Frame>`：临时进入采集流，取一帧并保存为图片
    - `take_video<P: Into<PathBuf>>(&mut self, path: P, duration: Duration, frame_rate: f32) -> Result<Video>`：临时进入采集流，在给定时长内连续取帧并写入视频
      - `duration` 为 0 时返回 `HikCameraError::InvalidDuration`
      - `frame_rate` 必须是有限正数，否则返回 `HikCameraError::InvalidFrameRate`
  - 采集流方法：
    - `stream(self) -> Result<Stream<'hik>>`：显式进入采集流，消费当前 `Camera`
      - 如果需要继续配置相机，先调用 `Stream::stop()` 取回 `Camera`
    - `set_stream(&mut self, options: StreamOptions) -> Result<()>`：设置 SDK 内部图像缓存节点数、取流策略和输出队列大小
      - 应在 `stream()` 之前调用
  - 生命周期方法：
    - `close(self) -> Result<()>`：显式关闭相机并销毁底层句柄
    - `raw_handle(&self) -> *mut c_void`：返回底层 C SDK handle，用于调用尚未封装的接口
  - 曝光和增益：
    - `get_exposure(&self) -> Result<FloatValue>`：读取 `ExposureTime`
    - `set_exposure(&mut self, value: f32) -> Result<()>`：设置 `ExposureTime`
    - `get_exposure_auto(&self) -> Result<EnumValue>`：读取 `ExposureAuto`
    - `set_exposure_auto(&mut self, mode: AutoMode) -> Result<()>`：设置自动曝光模式
    - `get_gain(&self) -> Result<FloatValue>`：读取 `Gain`
    - `set_gain(&mut self, value: f32) -> Result<()>`：设置 `Gain`
    - `get_gain_auto(&self) -> Result<EnumValue>`：读取 `GainAuto`
    - `set_gain_auto(&mut self, mode: AutoMode) -> Result<()>`：设置自动增益模式
  - ROI、触发和帧率：
    - `get_roi(&self) -> Result<Roi>`：读取 `Width`、`Height`、`OffsetX`、`OffsetY`
    - `set_roi(&mut self, roi: Roi) -> Result<()>`：设置图像 ROI
      - `width` 或 `height` 为 0 时返回 `HikCameraError::InvalidRoi`
    - `get_trigger(&self) -> Result<Trigger>`：读取 `TriggerMode` 和 `TriggerSource`
    - `set_trigger(&mut self, trigger: Trigger) -> Result<()>`：设置触发方式
      - `Trigger::Off` 关闭触发模式
      - `Trigger::Software` 使用软件触发
      - `Trigger::source("Line0")` 使用指定硬件输入线触发
    - `get_fps(&self) -> Result<Fps>`：读取采集帧率控制状态
    - `set_fps(&mut self, fps: Fps) -> Result<()>`：设置采集帧率控制
      - `Fps::Target(value)` 启用目标帧率
      - `Fps::Free` 关闭帧率控制
      - `value` 必须是有限正数，否则返回 `HikCameraError::InvalidFrameRate`
  - 通用节点方法：
    - `node_type(&self, key: &str) -> Result<NodeType>`：读取指定 GenICam 节点类型
    - `get_node(&self, key: &str) -> Result<NodeValue>`：按节点类型读取整数、浮点、枚举、布尔或字符串值
      - 当前不支持的节点类型返回 `HikCameraError::UnsupportedNode`
    - `set_node<V: Into<NodeInput>>(&mut self, key: &str, value: V) -> Result<()>`：按节点类型写入值
      - `&str` / `String` 默认表示枚举 symbolic 文本
      - 写入字符串节点时使用 `NodeString::new(value)`
      - 输入类型和节点类型不匹配时返回 `HikCameraError::NodeInputMismatch`
    - `convert_enum_to_text(&self, key: &str, value: u32) -> Result<String>`：把枚举数值转换成 symbolic 文本，例如 `"Mono8"`
    - `execute_node(&mut self, key: &str) -> Result<()>`：执行 command 节点，例如 `TriggerSoftware`
  - 图像处理配置：
    - `set_bayer_conversion(&mut self, options: BayerOptions) -> Result<()>`：设置 Bayer 转 RGB/BGR 的转换质量和平滑开关
    - `set_gamma(&mut self, value: f32) -> Result<()>`：按当前 `PixelFormat` 设置 Gamma 值
    - `set_bayer_ccm(&mut self, options: CcmOptions) -> Result<()>`：设置 Bayer CCM 颜色校正矩阵
  - 外部缓存方法：
    - `get_buffer_info(&self) -> Result<BufferInfo>`：获取外部缓存需要的大小和对齐信息
    - `unsafe fn register_buffer(&mut self, buffer: *mut c_void, size: u64) -> Result<()>`：注册用户分配的外部缓存
    - `unsafe fn unregister_buffer(&mut self, buffer: *mut c_void) -> Result<()>`：取消注册外部缓存

- `Stream<'hik>`
  - 已开始采集的相机流
  - 由 `Camera::stream()` 创建
  - 负责取帧、清理缓存、保存图片、保存视频和帧处理
  - 生命周期：
    - `Stream` 持有和 `Camera` 同一个底层 handle
    - `Drop` 时会兜底停止采集
    - 调用 `stop()` 可以显式停止采集并取回 `Camera`
  - 取帧和流状态：
    - `take_frame(&mut self, timeout: Duration) -> Result<Frame>`：在指定超时时间内取一帧，并复制图像数据到 `Frame.data`
      - 超时时间转换越界时返回 `HikCameraError::ValueOutOfRange`
    - `clear_buffer(&mut self) -> Result<()>`：清空 SDK 内部图像缓存
    - `get_image_count(&self) -> Result<u32>`：查询当前可输出的有效图像数量
    - `stop(self) -> Result<Camera<'hik>>`：停止采集并取回原来的 `Camera`
      - 如果仍有录像状态，会先停止录像
    - `raw_handle(&self) -> *mut c_void`：返回底层 C SDK handle
  - 图片写入：
    - `save_image<P: Into<PathBuf>>(&self, path: P) -> Result<ImageWriter>`：使用默认 `SaveOptions` 创建图片输出器
    - `save_image_with<P: Into<PathBuf>>(&self, path: P, options: SaveOptions) -> Result<ImageWriter>`：使用指定参数创建图片输出器
    - `encode_frame(&self, frame: &Frame, options: SaveOptions) -> Result<Vec<u8>>`：把一帧编码成图片字节
  - 视频写入：
    - `save_video<P: Into<PathBuf>>(&self, path: P, frame_rate: f32) -> Result<VideoWriter>`：用默认 AVI 参数创建视频写入器
    - `save_video_with(&self, options: VideoOptions) -> Result<VideoWriter>`：使用完整 `VideoOptions` 创建视频写入器
  - Frame 处理：
    - `convert_frame(&self, frame: &Frame, pixel_type: u32) -> Result<Frame>`：转换像素格式
    - `rotate_frame(&self, frame: &Frame, rotation: Rotation) -> Result<Frame>`：旋转图像
    - `reflect_frame(&self, frame: &Frame, direction: ReflectionDirection) -> Result<Frame>`：翻转图像
    - `contrast_frame(&self, frame: &Frame, factor: u32) -> Result<Frame>`：调整图像对比度
    - `hb_decode(&self, frame: &Frame) -> Result<Frame>`：解码 HB 无损压缩帧，自动估算输出缓存大小
    - `hb_decode_with(&self, frame: &Frame, buffer_size: u32) -> Result<Frame>`：解码 HB 无损压缩帧，使用指定输出缓存大小

- `Frame`
  - 从 `Stream` 取得的一帧图像，`Image` 是它的兼容别名
  - 字段：
    - `info: FrameInfo`：一帧图像的元信息
    - `data: Vec<u8>`：原始图像数据，保存、编码、处理或写入视频前不能为空

- `FrameInfo`
  - 一帧图像的元信息，对应 C SDK 的 `MV_FRAME_OUT_INFO_EX`
  - `ImageInfo` 是它的兼容别名
  - 基础图像字段：
    - `width`：图像宽度
    - `height`：图像高度
    - `pixel_type`：SDK 像素格式值
    - `frame_len`：图像数据长度
    - `frame_num`：帧号
  - 时间戳字段：
    - `device_timestamp`：设备时间戳
    - `host_timestamp`：主机时间戳
    - `second_count`：秒计数
    - `cycle_count`：周期计数
    - `cycle_offset`：周期偏移
  - 成像参数字段：
    - `gain`：当前帧增益
    - `exposure_time`：当前帧曝光时间
    - `average_brightness`：平均亮度
    - `red` / `green` / `blue`：白平衡或颜色统计相关值
  - 触发和 IO 字段：
    - `frame_counter`：帧计数器
    - `trigger_index`：触发序号
    - `input`：输入状态
    - `output`：输出状态
  - ROI 和 chunk 字段：
    - `offset_x` / `offset_y`：图像偏移
    - `chunk_width` / `chunk_height`：chunk 中的图像尺寸
    - `unparsed_chunk_num`：未解析 chunk 数量
  - 其它诊断字段：
    - `lost_packet`：丢包数量
    - `extra_type`：扩展类型
    - `sub_image_num`：子图数量
    - `first_encoder_count` / `last_encoder_count`：编码器计数
    - `last_frame_flag`：最后一帧标记

- `ImageWriter`
  - 图片输出器
  - 由 `Stream::save_image()` 或 `Stream::save_image_with()` 创建
  - 方法：
    - `write_frame(&mut self, frame: &Frame) -> Result<()>`：保存传入的这一帧
      - 空帧返回 `HikCameraError::EmptyFrame`
    - `finish(self) -> Result<()>`：结束图片输出
    - `path(&self) -> &Path`：返回图片输出路径

- `VideoWriter`
  - 视频输出器
  - 由 `Stream::save_video()` 或 `Stream::save_video_with()` 创建
  - 第一次 `write_frame()` 时根据该帧宽高和像素格式启动底层录像
  - 后续 `write_frame()` 继续写入帧
  - 同一个 `Stream` 同时只能有一个正在写帧的 `VideoWriter`
  - 方法：
    - `write_frame(&mut self, frame: &Frame) -> Result<()>`：写入一帧，首次调用时启动底层录像
      - 空帧返回 `HikCameraError::EmptyFrame`
      - 录像冲突返回 `HikCameraError::RecordingInProgress`
    - `finish(self) -> Result<Video>`：停止录像并返回 `Video`
      - 没有写入任何帧时返回 `HikCameraError::EmptyVideo`
    - `path(&self) -> &Path`：返回视频输出路径

- `Video`
  - 录像结果信息
  - 字段：
    - `path`：视频输出路径
    - `frame_count`：写入帧数
    - `frame_rate`：视频文件帧率
    - `width` / `height`：视频帧尺寸
    - `pixel_type`：视频帧像素格式
    - `elapsed`：录像耗时

- 节点读取结果类型
  - `IntValue`：整数节点读取结果，包含 `current`、`max`、`min`、`increment`
  - `FloatValue`：浮点节点读取结果，包含 `current`、`max`、`min`
  - `EnumValue`：枚举节点读取结果，包含 `current` 和 `supported`
  - `StringValue`：字符串节点读取结果，包含 `current` 和 `max_length`

- `NodeType`
  - GenICam 节点类型
  - 由 `Camera::node_type(key)` 返回
  - 覆盖常用 GenICam 节点接口类型，未单独封装的类型保留为 `Other`

- `NodeValue`
  - `Camera::get_node(key)` 返回的统一节点值
  - 覆盖整数、浮点、枚举、布尔和字符串节点值
  - 取值方法：
    - `as_int(&self) -> Result<&IntValue>` / `into_int(self) -> Result<IntValue>`：读取整数值
    - `as_float(&self) -> Result<&FloatValue>` / `into_float(self) -> Result<FloatValue>`：读取浮点值
    - `as_enum(&self) -> Result<&EnumValue>` / `into_enum(self) -> Result<EnumValue>`：读取枚举值
    - `as_bool(&self) -> Result<bool>` / `into_bool(self) -> Result<bool>`：读取布尔值
    - `as_string(&self) -> Result<&StringValue>` / `into_string(self) -> Result<StringValue>`：读取字符串值
    - 类型不匹配时返回 `HikCameraError::NodeValueMismatch`

- `NodeInput`
  - `Camera::set_node(key, value)` 接收的统一节点输入
  - 覆盖整数、浮点、布尔、枚举 symbolic 文本和字符串节点输入
  - 自动转换：
    - `i32` / `i64` / `u32`：转成 `NodeInput::Int`
    - `f32` / `f64`：转成 `NodeInput::Float`
    - `bool`：转成 `NodeInput::Bool`
    - `&str` / `String`：转成 `NodeInput::EnumSymbol`
    - `EnumSymbol`：转成 `NodeInput::EnumSymbol`
    - `NodeString`：转成 `NodeInput::String`

- `EnumSymbol`：枚举节点 symbolic 文本输入，`new(value) -> Self`

- `NodeString`：字符串节点输入，`new(value) -> Self`

- `BufferInfo`：外部缓存尺寸和对齐信息，字段 `size`、`alignment`

- `Roi`：图像区域设置，字段 `width`、`height`、`offset_x`、`offset_y`

- `Fps`：采集帧率控制

- `Rotation`：图像旋转角度

- `ReflectionDirection`：图像翻转方向

- `BayerInterpolation`：Bayer 转换质量

- `BayerOptions`：Bayer 转换配置，字段 `interpolation`、`smoothing`，默认 `Optimal + smoothing`

- `CcmOptions`：Bayer CCM 颜色校正参数，字段 `enabled`、`matrix`、`scale`

- `GrabStrategy`：SDK 取流策略

- `StreamOptions`：SDK 取流缓存配置，字段 `image_nodes`、`strategy`、`queue_size`

- `ImageFormat`：图片保存格式

- `SaveOptions`：图片保存参数，字段 `format`、`quality`、`method`、`endian`
  - 方法：
    - `SaveOptions::new(format: ImageFormat) -> Self`：创建指定格式的默认保存参数
    - `SaveOptions::default() -> Self`：默认使用 BMP

- `VideoFormat`：视频保存格式

- `VideoOptions`：视频保存参数，字段 `path`、`format`、`frame_rate`、`bit_rate`
  - 方法：
    - `VideoOptions::new<P: Into<PathBuf>>(path: P, frame_rate: f32) -> Self`：创建默认 AVI 参数
    - `format(self, format: VideoFormat) -> Self`：修改视频格式
    - `bit_rate(self, bit_rate: u32) -> Self`：修改码率

- `AutoMode`：自动曝光、自动增益模式

- `Trigger`：触发模式，表示关闭触发、软件触发或指定硬件触发源
  - 方法：
    - `Trigger::source(value: impl Into<String>) -> Self`：构造 `Trigger::Source`

## 常见写法

- 高层拍照和录像：

```rust
use std::time::Duration;

use hikcamera::{AutoMode, HikCamera};

fn main() -> hikcamera::Result<()> {
    let hik = HikCamera::new()?;
    let mut camera = hik.devices()?.default()?.open()?;

    camera.set_exposure_auto(AutoMode::Off)?;
    camera.set_exposure(8000.0)?;
    camera.set_gain(3.0)?;

    let image = camera.take_image("capture.bmp")?;
    let video = camera.take_video("capture.avi", Duration::from_secs(10), 30.0)?;

    println!("image: {}x{}", image.info.width, image.info.height);
    println!("video frames: {}", video.frame_count);

    camera.close()?;
    Ok(())
}
```

- 显式采集、处理并保存图片：

```rust
use std::time::Duration;

use hikcamera::{ImageFormat, Rotation, SaveOptions};

let mut stream = camera.stream()?;
let frame = stream.take_frame(Duration::from_secs(1))?;
let frame = stream.rotate_frame(&frame, Rotation::Angle90)?;

let mut image = stream.save_image_with("processed.png", SaveOptions::new(ImageFormat::Png))?;
image.write_frame(&frame)?;
image.finish()?;

let camera = stream.stop()?;
```

- 显式录像：

```rust
use std::time::{Duration, Instant};

use hikcamera::VideoOptions;

let mut stream = camera.stream()?;
let mut video = stream.save_video_with(
    VideoOptions::new("capture.avi", 30.0).bit_rate(16 * 1024),
)?;
let started = Instant::now();

while started.elapsed() < Duration::from_secs(10) {
    let frame = stream.take_frame(Duration::from_secs(1))?;
    video.write_frame(&frame)?;
}

let video = video.finish()?;
let camera = stream.stop()?;
```

- 通用节点访问：

```rust
use hikcamera::NodeString;

camera.set_node("ExposureTime", 8000.0)?;
camera.set_node("Gain", 3.0)?;
camera.set_node("PixelFormat", "BayerRG8")?;
camera.set_node("TriggerMode", "Off")?;
camera.set_node("DeviceUserID", NodeString::new("left-camera"))?;

let exposure = camera.get_node("ExposureTime")?.into_float()?;
println!("exposure: {}", exposure.current);
```
