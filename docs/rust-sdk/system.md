源码文件：`crates/hikcamera/src/system.rs`

这个文件负责 SDK 全局状态、SDK 版本和设备枚举入口。

对应 C SDK：

```c
MV_CC_Initialize
MV_CC_Finalize
MV_CC_GetSDKVersion
MV_CC_EnumDevices
```

## 提供的结构体

- `HikCamera`
  - Rust 高层 SDK 的入口状态
  - 创建时初始化海康 C SDK
  - 持有 SDK 已初始化这一层生命周期
  - 离开作用域时自动反初始化 SDK

- `HikVersion`
  - SDK 版本信息
  - 从 `MV_CC_GetSDKVersion` 的原始 `u32` 拆出来

## HikCamera

- `HikCamera::new()`
  - 初始化海康 C SDK
  - 成功后返回 `HikCamera`
  - 底层调用 `MV_CC_Initialize`
  - 多个 `HikCamera` 实例共享同一个 SDK 全局初始化计数
  - 只有第一个实例会真正初始化 SDK

- `version()`
  - 获取当前 SDK 版本
  - 返回 `HikVersion`
  - 底层调用 `MV_CC_GetSDKVersion`

- `devices()`
  - 枚举当前可见设备
  - 返回 `Result<Devices>`
  - 内部转到 `Devices::list()`
  - 底层调用 `MV_CC_EnumDevices`
  - 通过 `Devices` 继续查看设备信息、选择设备并打开相机

- `Drop`
  - `HikCamera` 离开作用域时自动调用
  - 会减少 SDK 全局初始化引用计数
  - 最后一个 `HikCamera` 离开作用域时才调用 `MV_CC_Finalize`

## HikVersion

- 字段
  - `major`：主版本
  - `minor`：次版本
  - `patch`：补丁版本
  - `build`：构建版本
  - `raw`：C SDK 返回的原始版本值

- `HikVersion::current()`
  - 直接读取当前 SDK 版本
  - 底层调用 `MV_CC_GetSDKVersion`

## 生命周期位置

- `HikCamera`
  - 对应 C SDK 的 `Initialize` 和 `Finalize`
  - 是最外层状态

- `Device`
  - 由 `HikCamera::devices()` 枚举出来
  - 只表示设备信息和设备可访问状态

- `Camera`
  - 由 `Device::open()` 打开
  - 对应 C SDK 的 `CreateHandle` 和 `OpenDevice`

- `Stream`
  - 由 `Camera::stream()` 进入
  - 对应 C SDK 的 `StartGrabbing`
  
