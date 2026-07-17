# HikCamera

HikCamera MVS 相机 SDK 的 Rust 封装

本项目基于 HikCamera MVS 5.0.1 Build 20260512（工业相机 SDK 4.8.0.3 Build 20260512）开发

使用前需要安装 HikCamera MVS，并确保相机驱动和运行时环境可用

## 安装

```toml
[dependencies]
hikcamera = { git = "https://github.com/xiaoze-cn/HikCamera-rs.git" }
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
hikcamera-rs/
├── .pixi/
├── conda-packages/
│   └── hikcamera-mvs/
│       ├── recipe.yaml
│       └── sources/
├── crates/
│   ├── hikcamera-sys/
│   │   ├── Cargo.toml
│   │   ├── build.rs
│   │   ├── tests/
│   │   │   └── bindings.rs
│   │   └── wrapper.h
│   └── hikcamera/
│       ├── Cargo.toml
│       ├── examples/
│       │   ├── camera.rs
│       │   ├── device.rs
│       │   ├── show.rs
│       │   └── system.rs
│       └── src/
│           ├── lib.rs
│           ├── camera.rs
│           ├── device.rs
│           ├── error.rs
│           ├── show.rs
│           └── system.rs
├── docs/
│   ├── hikcamera/
│   ├── hikcamera-sys/
│   └── index.md
├── scripts/
│   ├── check.py
│   └── check.toml
├── site/
│   ├── src/
│   ├── public/
│   └── justfile
├── target/
├── .gitignore
├── .gitattributes
├── README.md
├── LICENSE
├── Cargo.toml
├── pixi.toml
├── justfile
└── lefthook.yml
```

## Conda 包发布位置

`conda-packages/` 是 HikCamera MVS Conda 包的唯一源头，其中的 recipe 与 SDK 源文件从当前仓库读取。构建后的 `.conda` 包应放入下列 Channel 仓库的对应平台目录，并更新 `repodata.json`：

```text
C:\Projects\Integrations\HikCamera\conda-packages
    -> C:\Projects\Distributions\Conda Channel\win-64
    -> https://github.com/xiaoze-cn/Conda-Packages
```

如果 Channel 仓库目录变更，应同步更新其 `README.md` 和 `index.html`；本项目内的构建和检查配置均使用相对路径，无需修改。

## 协议

本项目中的 Rust 封装代码使用 MIT 协议，HikCamera MVS SDK 相关文件仍受其原始许可条款约束
