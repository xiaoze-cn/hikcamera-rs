源码文件：`crates/hikcamera/src/device.rs`

这个文件负责设备枚举结果、设备选择、设备信息整理和打开相机

## 提供的结构体和类型

- `Devices<'hik>`
  - 一次设备枚举的结果集合
  - 由 `HikCamera::devices()` 返回
  - 保留完整设备列表和整理后的设备信息
  - 查找不到或匹配到多台设备时返回显式错误
  - 常用选择入口会消费 `Devices`，直接返回选中的 `Device`
  - 方法：
    - `len(&self) -> usize`：返回设备数量
    - `is_empty(&self) -> bool`：判断设备列表是否为空
    - `iter(&self) -> impl Iterator<Item = &Device<'hik>>`：遍历设备列表
    - `as_slice(&self) -> &[Device<'hik>]`：以切片形式读取完整设备列表
    - `get(&self, index: usize) -> Option<&Device<'hik>>`：按索引读取设备
    - `default(self) -> Result<Device<'hik>>`：选择第一台设备，空列表返回 `HikCameraError::NoDevice`
    - `serial_number(self, value: &str) -> Result<Device<'hik>>`：按序列号精确查找设备
      - 固定设备优先用这个，比 IP 更稳定，也不依赖用户自定义名称
    - `user_name(self, value: &str) -> Result<Device<'hik>>`：按用户自定义名称精确查找设备
      - 适合现场给相机配置了可读名称的场景，例如 `left-camera`
    - `ip(self, value: &str) -> Result<Device<'hik>>`：按 GigE 当前 IP 精确查找设备
      - 如果设备 IP 可能变化，优先考虑 `serial_number(value)`
    - `mac(self, value: &str) -> Result<Device<'hik>>`：按 MAC 地址精确查找设备，匹配时忽略大小写
      - 主要用于 GigE 或带 MAC 信息的传输层设备，USB 设备通常没有 MAC 信息
    - `IntoIterator`
      - `Devices` 可以被消费成设备迭代器
      - `&Devices` 可以按引用遍历
  - 选择错误：
    - 找不到设备时返回 `HikCameraError::DeviceNotFound`
    - 匹配到多台设备时返回 `HikCameraError::MultipleDevices`

- `Device<'hik>`
  - 单个设备状态
  - 由 `Devices` 查找或遍历得到
  - 持有 SDK 入口生命周期，允许直接调用 `open()`
  - 内部保存原始 `MV_CC_DEVICE_INFO`
  - 内部保存整理后的 `DeviceInfo`
  - 方法：
    - `info(&self) -> &DeviceInfo`：返回整理后的设备信息
    - `is_accessible(&self) -> bool`：按独占权限检查设备当前是否可以打开
    - `open(self) -> Result<Camera<'hik>>`：创建 handle 并以独占模式打开设备
      - 打开失败时会销毁已经创建的 handle

- `DeviceInfo`
  - 高层设备信息
  - 把不同传输层的设备信息统一整理成 Rust 字段
  - 版本和分类：
    - `major_version`：SDK 设备信息结构主版本
    - `minor_version`：SDK 设备信息结构次版本
    - `transport`：设备传输层类型
    - `device_type`：SDK 返回的原始产品类型信息
  - 通用设备信息：
    - `model`：设备型号
    - `serial`：设备序列号
    - `user_name`：用户自定义名称
    - `vendor`：厂商或制造商名称
    - `version`：设备版本
    - `family`：产品家族名称
  - 接口标识：
    - `device_id`：USB、CameraLink 或 GenTL 设备 ID
    - `interface_id`：采集卡或接口 ID
    - `mac`：设备 MAC 地址
  - GigE 网络信息：
    - `ip`：当前 IP 地址
    - `subnet`：当前子网掩码
    - `gateway`：当前网关
    - `net_export`：网口 IP 地址
    - `host_ip`：当前占用设备的主机 IP
    - `multicast_ip`：组播 IP 地址
    - `multicast_port`：组播端口
    - `ip_config`：GigE IP 配置能力和当前模式
  - USB 信息：
    - `usb`：USB 端点和 USB 标识信息

- `Transport`
  - 设备传输层类型
  - 覆盖 GigE、USB、CameraLink、GenTL 等 C SDK 设备类型

- `IpConfig`
  - GigE IP 配置信息
  - 字段：`option`、`current`、`gen_tl_type`

- `UsbInfo`
  - USB 设备专用信息
  - 字段：`vendor_id`、`product_id`、`device_number`、`device_address`、`usb_version`、`control_in`、`control_out`、`stream`、`event`

## 设备入口

- `HikCamera::devices(&self) -> Result<Devices<'_>>`：枚举当前可见的 GigE / USB 设备
  - `DeviceInfo` 里保留了其他传输层的信息解析，供后续扩展专用枚举入口时复用

## 常见写法

- 查看设备列表：

```rust
let devices = hik.devices()?;
for device in devices.iter() {
    println!("{:?}", device.info());
}
```

- 按条件选择并打开设备：

```rust
let camera = hik.devices()?.default()?.open()?;
let camera = hik.devices()?.serial_number("DA1234567")?.open()?;
let camera = hik.devices()?.user_name("left-camera")?.open()?;
let camera = hik.devices()?.ip("192.168.1.64")?.open()?;
let camera = hik.devices()?.mac("00:11:22:33:44:55")?.open()?;
```

- 先查看信息，再选择设备：

```rust
let devices = hik.devices()?;

for device in &devices {
    let info = device.info();
    println!(
        "model={:?}, serial={:?}, name={:?}, ip={:?}, mac={:?}",
        info.model, info.serial, info.user_name, info.ip, info.mac
    );
}

let camera = devices.serial_number("DA1234567")?.open()?;
```
