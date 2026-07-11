文档范围：`hikcamera-rs` 项目整体说明

这个项目是海康 MVS 工业相机 C SDK 的 Rust 封装

## 项目分层

- `hikcamera`
  - 高层安全封装 crate
  - 面向普通 Rust 用户
  - 提供 SDK 生命周期、设备枚举、相机打开、取流、实时显示、图像处理、图片保存、视频保存和统一错误类型

- `hikcamera-sys`
  - 底层 FFI crate
  - 面向封装层和需要直接访问 C SDK 的高级用户
  - 通过 `bindgen` 从海康 MVS 头文件生成原始绑定

- `hikcamera-mvs`
  - pixi/conda 包中的海康 MVS SDK 文件
  - 提供头文件、import library 和运行时 DLL
  - 仓库内源头位置：`conda-packages/hikcamera-mvs/sources/5.0.1-20260512/`

## 代码到文档的对应关系

- `crates/hikcamera/`
  - 文档：`site/content/hikcamera/index.md`
  - Rust 高层 SDK 的典型使用流程

- `crates/hikcamera/src/system.rs`
  - 文档：`site/content/hikcamera/system.md`
  - SDK 初始化、反初始化、版本读取和设备枚举入口

- `crates/hikcamera/src/error.rs`
  - 文档：`site/content/hikcamera/error.md`
  - `HikCameraError`、`Status`、SDK 返回码翻译和封装层错误

- `crates/hikcamera/src/device.rs`
  - 文档：`site/content/hikcamera/device.md`
  - 设备列表、设备选择、设备信息整理和打开相机

- `crates/hikcamera/src/camera.rs`
  - 文档：`site/content/hikcamera/camera.md`
  - 打开后的相机、采集流、帧数据、图像处理、图片写入和视频写入

- `crates/hikcamera/src/show.rs`
  - 文档：`docs/hikcamera/show.md`
  - 基于已有 `Stream` 的实时画面显示入口

- `crates/hikcamera-sys/`
  - 文档：`site/content/hikcamera-sys/index.md`
  - 原始 FFI 绑定、`bindgen` 构建流程和状态码生成

- `conda-packages/hikcamera-mvs/sources/5.0.1-20260512/include/`
  - 文档：`site/content/hikcamera-sys/c-sdk/`
  - 海康 MVS C SDK 头文件结构、函数、结构体、常量、像素格式和错误码

## 文档入口

- 先看 `site/content/index.md`
  - 理解项目分层和文档边界

- 使用高层 Rust SDK 时看 `site/content/hikcamera/`
  - `index.md`
  - `system.md`
  - `error.md`
  - `device.md`
  - `camera.md`

- 使用实时显示能力时看 `docs/hikcamera/show.md`

- 需要理解 FFI 或构建细节时看 `site/content/hikcamera-sys/index.md`

- 需要对照 C SDK 原始概念时看 `site/content/hikcamera-sys/c-sdk/`
  - `MvCameraControl.md`：C SDK 主入口函数
  - `CameraParams.md`：结构体、枚举和宏
  - `MvErrorDefine.md` / `MvISPErrorDefine.md`：错误码
  - `PixelType.md`：像素格式
  - `MvObsoleteInterfaces.md` / `ObsoleteCamParams.md`：旧接口和旧结构

## 设计边界

- 高层 API 优先放在 `hikcamera`
  - 用户默认不需要直接接触 C handle
  - SDK 返回码统一进入 `HikCameraError::Sdk`
  - 封装层主动检查的错误使用独立 enum variant

- 原始 SDK 能力保留在 `hikcamera-sys`
  - C 函数、结构体、宏和枚举尽量保持原始命名
  - 只对状态码做特殊处理，使它们以 `i32` 暴露

- C SDK 文档只做索引和解释
  - 不替代官方手册
  - 用来帮助理解 Rust 封装为什么这样组织
