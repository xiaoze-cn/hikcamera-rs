源码文件：`crates/hikcamera/src/system.rs`

这个文件负责 SDK 全局状态、SDK 版本读取和设备枚举入口

## 提供的结构体和类型

- `HikCamera`
  - Rust 高层 SDK 的入口状态
  - 包住海康 C SDK 的全局初始化生命周期
  - 多实例共享初始化引用计数
  - 方法：
    - `HikCamera::new() -> Result<Self>`：初始化海康 C SDK，首个实例调用 `MV_CC_Initialize`，后续实例只增加引用计数
      - 引用计数锁被污染时返回 `HikCameraError::SdkStatePoisoned`
    - `version(&self) -> HikVersion`：读取当前 SDK 版本
    - `devices(&self) -> Result<Devices<'_>>`：枚举当前可见的 GigE / USB 设备
    - `Drop`
      - 减少 SDK 全局初始化引用计数
      - 最后一个实例离开作用域时调用 `MV_CC_Finalize`

- `HikVersion`
  - SDK 版本信息
  - 从 `MV_CC_GetSDKVersion` 的原始 `u32` 拆出来
  - 字段：
    - `major`：主版本
    - `minor`：次版本
    - `patch`：补丁版本
    - `build`：构建版本
    - `raw`：C SDK 返回的原始版本值
  - 方法：
    - `HikVersion::current() -> Self`：直接读取当前 SDK 版本

## 生命周期关系

- `HikCamera`
  - 最外层 SDK 状态
  - 对应 C SDK 的初始化和反初始化阶段
  - 是 `Devices`、`Device`、`Camera` 生命周期的来源

- `Devices<'hik>`
  - 由 `HikCamera::devices()` 枚举得到
  - 借用 `HikCamera` 的生命周期
  - 保存一次枚举得到的设备集合

- `Device<'hik>`
  - 由 `Devices` 查找或遍历得到
  - 只表示设备信息和设备可访问状态
  - 调用 `open()` 后进入 `Camera`

- `Camera<'hik>`
  - 由 `Device::open()` 打开
  - 对应 C SDK 的 `CreateHandle` 和 `OpenDevice`
  - 可以设置参数、拍照、录像，或进入 `Stream`

- `Stream<'hik>`
  - 由 `Camera::stream()` 进入
  - 对应 C SDK 的 `StartGrabbing`
  - 调用 `stop()` 后停止采集并取回 `Camera`
