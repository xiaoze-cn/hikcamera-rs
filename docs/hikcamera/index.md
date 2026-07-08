源码位置：`crates/hikcamera/`

这个 crate 是用户主要使用的 Rust 高层 SDK，负责把 `hikcamera-sys` 暴露的 C SDK 能力整理成更符合 Rust 使用习惯的接口

## 使用流程

- 初始化 SDK
  - `HikCamera::new() -> Result<Self>`
  - 创建高层 SDK 入口
  - 首个实例初始化海康 C SDK，后续实例复用同一份全局状态

- 枚举设备
  - `HikCamera::devices() -> Result<Devices<'_>>`
  - 返回当前可见的 GigE / USB 设备列表
  - 常用选择方式：
    - `devices.default() -> Result<Device<'_>>`：选择第一台设备
    - `devices.serial_number(value) -> Result<Device<'_>>`：按序列号选择
    - `devices.user_name(value) -> Result<Device<'_>>`：按用户自定义名称选择
    - `devices.ip(value) -> Result<Device<'_>>`：按 IP 选择
    - `devices.mac(value) -> Result<Device<'_>>`：按 MAC 选择

- 打开相机
  - `Device::open() -> Result<Camera<'_>>`
  - 创建 C SDK handle 并打开设备
  - 成功后进入相机控制阶段

- 采集图像
  - `Camera::stream() -> Result<Stream<'_>>`
  - 开始取流
  - `Stream::take_frame(timeout) -> Result<Frame>`：获取一帧图像
  - `Stream::stop() -> Result<Camera<'_>>`：停止取流并取回相机

- 关闭相机
  - `Camera::close() -> Result<()>`
  - 显式关闭设备 handle
  - 如果提前进入 `Stream`，需要先 `stop()` 回到 `Camera`

## 最小示例

```rust
use std::path::Path;
use std::time::Duration;

use hikcamera::HikCamera;

fn main() -> hikcamera::Result<()> {
    let hik = HikCamera::new()?;

    let devices = hik.devices()?;
    let device = devices.default()?;
    let camera = device.open()?;

    let mut stream = camera.stream()?;
    let frame = stream.take_frame(Duration::from_secs(1))?;

    let mut image = stream.save_image(Path::new("image.bmp"))?;
    image.write_frame(&frame)?;
    image.finish()?;

    let camera = stream.stop()?;
    camera.close()?;

    Ok(())
}
```

## 常用文档

- `system.md`
  - SDK 初始化、版本读取和设备枚举入口

- `device.md`
  - 设备列表、设备选择、设备信息和打开相机

- `camera.md`
  - 相机控制、取流、帧数据、图像处理、图片保存和视频保存

- `error.md`
  - `HikCameraError`、`Status`、SDK 返回码和封装层错误
