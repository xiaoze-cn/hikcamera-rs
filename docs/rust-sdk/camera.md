# camera

源代码：`crates/hikcamera/src/camera.rs`

这一层封装打开后的相机、采集流、帧数据、图片写入和视频写入。

核心设计是：

```text
Camera  设置参数，进入采集流，也提供高层拍照/录像入口
Stream  已经 StartGrabbing 的采集状态，负责取 Frame
Frame   一帧图像数据，可以处理，也可以写入图片或视频
Writer  保存目标，负责把 Frame 写出去
```

## 对应 C SDK

```c
MV_CC_StartGrabbing
MV_CC_GetImageBuffer
MV_CC_FreeImageBuffer
MV_CC_StopGrabbing
MV_CC_SaveImageToFileEx2
MV_CC_SaveImageEx3
MV_CC_ConvertPixelTypeEx
MV_CC_RotateImage
MV_CC_FlipImage
MV_CC_ImageContrast
MV_CC_HB_Decode
MV_CC_StartRecord
MV_CC_InputOneFrameEx
MV_CC_StopRecord
```

## 类型

- `Camera`
  - 已打开的相机
  - 用于设置曝光、增益、触发、像素格式等参数
  - 可以调用 `stream()` 进入采集流
  - 可以调用高层 `take_image()` / `take_video()`
  - `raw_handle()` 是底层逃生口

- `Stream`
  - 已开始采集的相机流
  - 由 `Camera::stream()` 创建
  - 负责 `take_frame()`
  - 负责创建 `ImageWriter` / `VideoWriter`
  - `raw_handle()` 是底层逃生口

- `Frame`
  - 从 `Stream` 取得的一帧图像
  - 包含 `FrameInfo` 和原始图像数据
  - `Image` 是 `Frame` 的兼容别名

- `FrameInfo`
  - 一帧图像的元信息
  - 对应 C SDK 的 `MV_FRAME_OUT_INFO_EX`
  - `ImageInfo` 是 `FrameInfo` 的兼容别名

- `ImageWriter`
  - 图片输出器
  - 由 `save_image()` / `save_image_with()` 创建
  - `write_frame()` 把一帧保存成图片

- `VideoWriter`
  - 视频输出器
  - 由 `save_video()` / `save_video_with()` 创建
  - 第一次 `write_frame()` 时根据该帧宽高和像素格式启动底层录像
  - 后续 `write_frame()` 继续写入帧
  - `finish()` 停止录像并返回 `Video`

- `Video`
  - 录像结果信息
  - 包含路径、帧数、帧率、宽高、像素格式和耗时

- `IntValue`
  - 整数节点读取结果
  - 包含当前值、最大值、最小值和步进

- `FloatValue`
  - 浮点节点读取结果
  - 包含当前值、最大值和最小值

- `EnumValue`
  - 枚举节点读取结果
  - 包含当前枚举值和支持值列表

- `StringValue`
  - 字符串节点读取结果
  - 包含当前字符串和最大长度

- `NodeType`
  - GenICam 节点类型
  - 由 `node_type(key)` 返回

- `NodeValue`
  - `get_node(key)` 返回的统一节点值
  - 目前支持 `Int`、`Float`、`Enum`、`Bool`、`String`

- `NodeInput`
  - `set_node(key, value)` 接收的统一节点输入
  - Rust 的整数、浮点、布尔可以自动转入
  - `&str` / `String` 默认作为枚举 symbolic 文本转入

- `EnumSymbol`
  - 枚举节点的 symbolic 文本输入
  - `&str` / `String` 会自动按这个语义处理

- `NodeString`
  - 字符串节点输入
  - 写入字符串节点时使用 `NodeString::new(value)`

## 高层 API

高层 API 面向最常见的用户任务：拍一张图、录一段视频。

这一路径会直接完成“采集并保存”，适合不需要在保存前处理图像的场景。

```rust
use std::time::Duration;

use hikcamera::HikCamera;

fn main() -> hikcamera::Result<()> {
    let hik = HikCamera::new()?;
    let mut camera = hik.devices()?.default()?.open()?;

    camera.set_exposure(8000.0)?;
    camera.set_gain(3.0)?;

    let image = camera.take_image("capture.bmp")?;
    let video = camera.take_video("capture.avi", Duration::from_secs(10), 30.0)?;

    println!("image: {}x{}", image.info.width, image.info.height);
    println!("video frames: {}", video.frame_count);

    Ok(())
}
```

- `Camera::take_image(path)`
  - 内部临时进入采集流
  - 取一帧 `Frame`
  - 保存到图片文件
  - 返回该 `Frame`

- `Camera::take_video(path, duration, frame_rate)`
  - 内部临时进入采集流
  - 在给定时长内连续取帧并写入视频
  - `frame_rate` 是视频文件帧率，单位是 fps
  - 返回 `Video`

## 相机参数

这些方法属于 `Camera`，一般在进入 `stream()` 之前调用。

- `get_exposure()`
  - 读取曝光时间节点 `ExposureTime`
  - 返回 `FloatValue`

- `set_exposure(value)`
  - 设置曝光时间节点 `ExposureTime`

- `get_exposure_auto()`
  - 读取自动曝光节点 `ExposureAuto`
  - 返回 `EnumValue`

- `set_exposure_auto(mode)`
  - 设置自动曝光模式
  - `AutoMode::Off`、`Once`、`Continuous`

- `get_gain()`
  - 读取增益节点 `Gain`
  - 返回 `FloatValue`

- `set_gain(value)`
  - 设置增益节点 `Gain`

- `get_gain_auto()`
  - 读取自动增益节点 `GainAuto`
  - 返回 `EnumValue`

- `set_gain_auto(mode)`
  - 设置自动增益模式
  - `AutoMode::Off`、`Once`、`Continuous`

- `get_roi()` / `set_roi(roi)`
  - 读取或设置图像 ROI
  - 包含宽度、高度、水平偏移和垂直偏移
  - 对应节点 `Width`、`Height`、`OffsetX`、`OffsetY`

- `get_trigger()` / `set_trigger(trigger)`
  - 读取或设置触发方式
  - `Trigger::Off` 关闭触发模式
  - `Trigger::Software` 使用软件触发
  - `Trigger::source("Line0")` 使用指定硬件输入线触发
  - 对应节点 `TriggerMode` 和 `TriggerSource`

- 软件触发执行命令不单独封装
  - 使用 `execute_node("TriggerSoftware")`
  - 一般配合 `set_trigger(Trigger::Software)` 使用

- `get_fps()` / `set_fps(fps)`
  - 读取或设置采集帧率控制
  - `Fps::Target(value)` 启用帧率控制，并设置目标采集帧率
  - `Fps::Free` 关闭帧率控制，让相机根据曝光、触发、带宽和设备能力运行
  - 对应节点 `AcquisitionFrameRateEnable` 和 `AcquisitionFrameRate`

示例：

```rust
camera.set_exposure_auto(AutoMode::Off)?;
camera.set_exposure(8000.0)?;
camera.set_gain(3.0)?;
camera.set_roi(Roi {
    width: 1920,
    height: 1080,
    offset_x: 0,
    offset_y: 0,
})?;
camera.set_trigger(Trigger::Off)?;
camera.set_fps(Fps::Target(30.0))?;
```

## 通用节点 API

通用节点 API 用于访问暂时没有专门封装的方法。

- `node_type(key)`
  - 返回指定 GenICam 节点的类型
  - 底层调用 `MV_XML_GetNodeInterfaceType`

- `get_node(key)`
  - 按节点类型读取值
  - 返回 `NodeValue`
  - 支持整数、浮点、枚举、布尔、字符串节点

- `set_node(key, value)`
  - 按节点类型写入值
  - 对枚举节点可以写入数值，也可以写入 symbolic 文本
  - `&str` / `String` 默认表示枚举 symbolic 文本，例如 `"BayerRG8"`、`"Off"`
  - 写入字符串节点时使用 `NodeString::new(value)`，避免和枚举 symbolic 文本混淆

- `convert_enum_to_text(key, value)`
  - 把枚举数值转换成 symbolic 文本
  - 例如把 `PixelFormat` 的当前数值转换为 `"Mono8"` 这类文本

- `execute_node(key)`
  - 执行 command 节点
  - 例如 `execute_node("TriggerSoftware")`

示例：

```rust
camera.set_node("ExposureTime", 8000.0)?;
camera.set_node("Gain", 3.0)?;
camera.set_node("PixelFormat", "BayerRG8")?;
camera.set_node("TriggerMode", "Off")?;
camera.set_node("DeviceUserID", NodeString::new("left-camera"))?;

let exposure = camera.get_node("ExposureTime")?.into_float()?;
println!("exposure: {}", exposure.current);
```

`NodeValue` 可以用这些方法取出具体类型：

- `as_int()` / `into_int()`
- `as_float()` / `into_float()`
- `as_enum()` / `into_enum()`
- `as_bool()` / `into_bool()`
- `as_string()` / `into_string()`

## 采集流配置

这些方法配置 SDK 取流缓存和取图策略。`Camera::set_stream()` 应在 `stream()` 之前调用。

- `Camera::set_stream(options)`
  - 设置 SDK 内部图像缓存节点数、取流策略和输出队列大小
  - `StreamOptions::image_nodes` 对应 `MV_CC_SetImageNodeNum`
  - `StreamOptions::strategy` 对应 `MV_CC_SetGrabStrategy`
  - `StreamOptions::queue_size` 对应 `MV_CC_SetOutputQueueSize`
  - `GrabStrategy::OneByOne`：从旧到新逐帧获取
  - `GrabStrategy::LatestOnly`：只取最新一帧
  - `GrabStrategy::LatestImages`：取最新的多帧队列
  - `GrabStrategy::Upcoming`：等待下一帧

```rust
camera.set_stream(StreamOptions {
    image_nodes: 4,
    strategy: GrabStrategy::LatestOnly,
    queue_size: None,
})?;
```

- `Stream::clear_buffer()`
  - 清空 SDK 内部图像缓存
  - 底层调用 `MV_CC_ClearImageBuffer`

- `Stream::get_image_count()`
  - 查询当前可输出的有效图像数量
  - 底层调用 `MV_CC_GetValidImageNum`

## 高级缓存接口

- `get_buffer_info()`
  - 获取外部缓存需要的大小和对齐信息
  - 返回 `BufferInfo { size, alignment }`
  - 底层仍调用 `MV_CC_GetPayloadSize`

- `register_buffer(buffer, size)`
  - 注册用户分配的外部缓存
  - `unsafe`
  - 底层调用 `MV_CC_RegisterBuffer`

- `unregister_buffer(buffer)`
  - 取消注册外部缓存
  - `unsafe`
  - 底层调用 `MV_CC_UnRegisterBuffer`

## 显式 API

显式 API 把流程拆成三步：

```text
take_frame -> 可选处理 -> 写入存储
```

显式 API 的主要价值是把 `Frame` 暴露出来，让用户可以在保存前插入像素格式转换、旋转、翻转、对比度调整、算法处理、筛帧、显示、推流或多设备同步等逻辑。

拍照：

```rust
use std::time::Duration;

let mut stream = camera.stream()?;
let timeout = Duration::from_secs(1);

let frame = stream.take_frame(timeout)?;
let frame = stream.convert_frame(&frame, frame.info.pixel_type)?;

let mut image = stream.save_image("capture.bmp")?;
image.write_frame(&frame)?;
image.finish()?;

let camera = stream.stop()?;
```

录像：

```rust
use std::time::{Duration, Instant};

let mut stream = camera.stream()?;
let timeout = Duration::from_secs(1);

let mut video = stream.save_video("capture.avi", 30.0)?;
let started = Instant::now();

while started.elapsed() < Duration::from_secs(10) {
    let frame = stream.take_frame(timeout)?;
    // 这里可以做图像处理、过滤、显示、推流或同步其它设备
    video.write_frame(&frame)?;
}

let video = video.finish()?;
let camera = stream.stop()?;
```

`save_video()` 返回的是视频写入器，不是马上保存完成。  
视频文件按帧流式写入，不会先把完整视频缓存到内存。

## Frame 处理

Frame 处理函数只挂在 `Stream` 上。它们都接收已有 `Frame`，返回新的 `Frame`：

- `convert_frame(frame, pixel_type)`
  - 转换像素格式
  - 底层调用 `MV_CC_ConvertPixelTypeEx`
  - 常用于把 Bayer、Mono、RGB/BGR 等格式转成后续算法或保存需要的格式

- `rotate_frame(frame, rotation)`
  - 旋转图像
  - 支持 90、180、270 度
  - 底层调用 `MV_CC_RotateImage`

- `reflect_frame(frame, direction)`
  - 翻转图像
  - 支持垂直翻转和水平翻转
  - 底层调用 `MV_CC_FlipImage`

- `contrast_frame(frame, factor)`
  - 调整图像对比度
  - `factor` 是对比度系数
  - 底层调用 `MV_CC_ImageContrast`

- `hb_decode(frame)`
  - 解码 HB 无损压缩帧
  - 自动估算输出缓存大小
  - 底层调用 `MV_CC_HB_Decode`

- `hb_decode_with(frame, buffer_size)`
  - 解码 HB 无损压缩帧
  - 使用用户指定的输出缓存大小
  - 适合自动估算不够或用户想控制缓存的场景
  - 底层调用 `MV_CC_HB_Decode`

示例：

```rust
let frame = stream.take_frame(Duration::from_secs(1))?;
let frame = stream.rotate_frame(&frame, Rotation::Angle90)?;
let frame = stream.contrast_frame(&frame, 20)?;

stream.save_image("processed.bmp")?.write_frame(&frame)?;
```

## 图像处理配置

这些方法设置 SDK 内部图像处理参数，影响后续的像素转换、保存或相关处理。

- `set_bayer_conversion(options)`
  - 设置 Bayer 转 RGB/BGR 时的转换策略
  - `BayerInterpolation::Fast`：快速
  - `BayerInterpolation::Balanced`：均衡
  - `BayerInterpolation::Optimal`：最优
  - `BayerInterpolation::OptimalPlus`：增强最优
  - `BayerOptions.smoothing` 开启或关闭 Bayer 插值平滑
  - 底层依次调用 `MV_CC_SetBayerCvtQuality` 和 `MV_CC_SetBayerFilterEnable`

```rust
camera.set_bayer_conversion(BayerOptions {
    interpolation: BayerInterpolation::Optimal,
    smoothing: true,
})?;
```

- `set_gamma(value)`
  - 按当前 `PixelFormat` 设置 Gamma 值
  - 内部先读取当前像素格式，再调用 `MV_CC_SetGammaValue`
  - 自定义 Gamma 曲线和 `MV_CC_SetBayerGammaParam` 暂不放在高层 API

- `set_bayer_ccm(options)`
  - 设置 Bayer CCM 颜色校正矩阵
  - `CcmOptions` 包含是否启用、3x3 矩阵和缩放系数
  - 底层调用 `MV_CC_SetBayerCCMParamEx`

## 保存图片

- `Stream::save_image(path)`
  - 使用默认 `SaveOptions`
  - 默认保存 BMP

- `Stream::save_image_with(path, options)`
  - 指定图片格式、质量、Bayer 插值方式和字节序

- `ImageWriter::write_frame(frame)`
  - 保存传入的这一帧

- `ImageWriter::finish()`
  - 结束图片输出
  - 当前实现中图片没有持续会话，主要用于和视频输出保持一致

```rust
let frame = stream.take_frame(Duration::from_secs(1))?;

let options = SaveOptions::new(ImageFormat::Png);
let mut image = stream.save_image_with("capture.png", options)?;
image.write_frame(&frame)?;
image.finish()?;
```

## 保存视频

- `Stream::save_video(path, frame_rate)`
  - 创建视频写入器
  - 默认 AVI，默认码率 `8 * 1024`

- `Stream::save_video_with(options)`
  - 使用完整 `VideoOptions`

- `VideoWriter::write_frame(frame)`
  - 第一次调用时启动底层 `MV_CC_StartRecord`
  - 每次调用写入一帧

- `VideoWriter::finish()`
  - 调用 `MV_CC_StopRecord`
  - 返回 `Video`

```rust
let mut video = stream.save_video_with(
    VideoOptions::new("capture.avi", 30.0).bit_rate(16 * 1024),
)?;

for _ in 0..300 {
    let frame = stream.take_frame(Duration::from_secs(1))?;
    video.write_frame(&frame)?;
}

let video = video.finish()?;
```

## Options

- `SaveOptions`
  - `format`：图片格式
  - `quality`：JPEG 质量
  - `method`：Bayer 插值方式
  - `endian`：字节序

- `ImageFormat`
  - `Bmp`
  - `Jpeg`
  - `Png`
  - `Tiff`

- `VideoOptions`
  - `path`：视频文件路径
  - `format`：视频格式
  - `frame_rate`：视频文件帧率，单位 fps
  - `bit_rate`：码率

- `VideoFormat`
  - `Avi`

## 底层逃生口

- `Camera::raw_handle()`
- `Stream::raw_handle()`

这些方法返回 C SDK 原始 handle，用于调用当前 Rust 封装还没有覆盖的接口。
