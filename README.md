# HikCamera

HikCamera MVS 相机 SDK 的 Rust 封装

本项目基于 HikCamera MVS 5.0.1 Build 20260512（工业相机 SDK 4.8.0.3 Build 20260512）开发

使用前需要安装 HikCamera MVS，并确保相机驱动和运行时环境可用

## 安装

```toml
[dependencies]
hikcamera = { git = "https://github.com/xiaoze-cn/hikcamera-rs.git" }
```

## 示例

```rust
use std::time::Duration;
use hikcamera::{HikCamera, Rotation};

const IMAGE_PATH: &str = "image.bmp";
const TIMEOUT: Duration = Duration::from_secs(1);

fn main() -> hikcamera::Result<()> {
    let hik = HikCamera::new()?;

    let devices = hik.devices()?;
    let device = devices.default()?;
    let mut camera = device.open()?;
    camera.set_exposure(8000.0)?;
    camera.set_gain(3.0)?;

    let mut stream = camera.stream()?;
    let frame = stream.take_frame(TIMEOUT)?;
    let frame = stream.rotate_frame(&frame, Rotation::Angle180)?;

    let mut output = stream.save_image(IMAGE_PATH)?;
    output.write_frame(&frame)?;
    output.finish()?;

    println!(
        "captured image: {}x{}, {} bytes",
        frame.info.width,
        frame.info.height,
        frame.data.len()
    );

    let camera = stream.stop()?;
    camera.close()?;

    Ok(())
}
```

## 项目结构

```text
HikCamera/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── AGENTS.md
├── justfile
├── pixi.toml
├── site/                # Starlight 文档站点
└── crates/
    ├── hikcamera/
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs
    │       ├── camera.rs
    │       ├── device.rs
    │       ├── error.rs
    │       └── system.rs
    └── hikcamera-sys/
        ├── Cargo.toml
        ├── build.rs
        ├── wrapper.h
        ├── include/       # 厂商 C 头文件
        └── lib/
            ├── win32/
            └── win64/
```

## 文档

在线站点（双语）由 `site/` 下的 Starlight 生成，本地预览：

```sh
just site install     # 首次
just site dev         # 本地开发
just site build       # 生成静态文件
```

主要分区：

- **使用指南**：安装、快速开始、SDK 生命周期、设备选择、相机配置、采集、图像与视频写入
- **开发者文档**：架构、错误模型、贡献
- **参考**：运行时依赖、设备信息字段、C SDK 函数 / 结构体 / 像素格式 / 错误码

## 协议

本项目中的 Rust 封装代码使用 MIT 协议，HikCamera MVS SDK 相关文件仍受其原始许可条款约束
