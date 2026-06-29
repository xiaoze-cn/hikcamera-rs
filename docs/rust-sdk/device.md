源码文件：`crates/hikcamera/src/device.rs`

这个文件负责设备状态和设备信息整理。

对应 C SDK：

```c
MV_CC_EnumDevices
MV_CC_DEVICE_INFO_LIST
MV_CC_DEVICE_INFO
```

## 提供的结构体和枚举

- `Device`
  - 单个设备状态
  - 由 `Devices` 查找或遍历得到
  - 持有 SDK 入口生命周期，允许直接调用 `open()`
  - 内部保存原始 `MV_CC_DEVICE_INFO`
  - 内部保存整理后的 `DeviceInfo`

- `Devices`
  - 一次设备枚举的结果集合
  - 保留完整设备列表和设备信息
  - 提供列表访问入口，也提供常用设备选择入口
  - 查找不到或匹配到多台设备时返回显式错误
  - 常用选择入口会消费 `Devices`，直接返回选中的 `Device`

- `DeviceInfo`
  - 高层设备信息
  - 把不同传输层的设备信息统一整理成 Rust 字段

- `Transport`
  - 设备传输层类型
  - 对应 C SDK 的设备类型宏

- `IpConfig`
  - GigE IP 配置信息

- `UsbInfo`
  - USB 设备专用信息

## DeviceInfo

- 版本和分类
  - `major_version`：SDK 设备信息结构主版本
  - `minor_version`：SDK 设备信息结构次版本
  - `transport`：设备传输层类型
  - `device_type`：SDK 返回的原始产品类型信息

- 通用设备信息
  - `model`：设备型号
  - `serial`：设备序列号
  - `user_name`：用户自定义名称
  - `vendor`：厂商或制造商名称
  - `version`：设备版本
  - `family`：产品家族名称

- 接口标识
  - `device_id`：USB、CameraLink 或 GenTL 设备 ID
  - `interface_id`：采集卡或接口 ID
  - `mac`：设备 MAC 地址

- GigE 网络信息
  - `ip`：当前 IP 地址
  - `subnet`：当前子网掩码
  - `gateway`：当前网关
  - `net_export`：网口 IP 地址
  - `host_ip`：当前占用设备的主机 IP
  - `multicast_ip`：组播 IP 地址
  - `multicast_port`：组播端口
  - `ip_config`：GigE IP 配置能力和当前模式

- USB 信息
  - `usb`：USB 端点和 USB 标识信息

## Transport

- `GigE`：GigE Vision 设备
- `Usb`：USB3 Vision 设备
- `CameraLink`：CameraLink 设备
- `Ieee1394`：1394-a/b 设备
- `VirtualGigE`：虚拟 GigE Vision 设备
- `VirtualUsb`：虚拟 USB3 Vision 设备
- `GenTlGigE`：GenTL GigE 设备
- `GenTlCameraLink`：GenTL CameraLink 设备
- `GenTlCoaXPress`：GenTL CoaXPress 设备
- `GenTlXoF`：GenTL XoF 设备
- `GenTlVirtual`：GenTL 虚拟设备
- `Other(u32)`：未单独封装的传输层类型
- `Unknown`：SDK 返回的未知设备类型

## 提供的函数

## HikCamera 设备入口

- `HikCamera::devices()`
  - 枚举当前设备列表，返回 `Result<Devices>`
  - 当前枚举 `GigE + USB`
  - `DeviceInfo` 里保留了其他传输层的信息解析，供后续扩展专用枚举入口时复用
  - 底层调用 `MV_CC_EnumDevices`

## Devices

- `len()`
  - 返回枚举到的设备数量

- `is_empty()`
  - 判断当前设备列表是否为空

- `iter()`
  - 遍历设备列表
  - 用于打印设备信息、调试或自定义选择逻辑

- `as_slice()`
  - 以切片形式读取完整设备列表

- `get(index)`
  - 按索引读取设备
  - 返回 `Option<&Device>`

- `default()`
  - 默认选择枚举结果里的第一台设备
  - 如果没有设备，返回 `Error::NoDevice`

- `serial_number(value)`
  - 按序列号精确查找设备
  - 推荐作为固定设备的优先选择方式
  - 序列号通常比 IP 更稳定，也不依赖用户自定义名称
  - 如果找不到，返回 `Error::DeviceNotFound`
  - 如果匹配到多台，返回 `Error::MultipleDevices`

- `user_name(value)`
  - 按用户自定义名称精确查找设备
  - 适合现场给相机配置了可读名称的场景，例如 `left-camera`
  - 如果找不到，返回 `Error::DeviceNotFound`
  - 如果匹配到多台，返回 `Error::MultipleDevices`

- `ip(value)`
  - 按 GigE 当前 IP 精确查找设备
  - 主要用于 GigE 网络相机
  - 如果设备 IP 可能变化，优先考虑 `serial_number(value)`
  - 如果找不到，返回 `Error::DeviceNotFound`
  - 如果匹配到多台，返回 `Error::MultipleDevices`

- `mac(value)`
  - 按设备 MAC 地址精确查找设备
  - 主要用于 GigE 或带 MAC 信息的传输层设备
  - USB 设备通常没有 MAC 信息
  - 匹配时忽略大小写
  - 如果找不到，返回 `Error::DeviceNotFound`
  - 如果匹配到多台，返回 `Error::MultipleDevices`

- `IntoIterator`
  - `Devices` 可以被消费成设备迭代器
  - `&Devices` 可以按引用遍历

## Device

- `Device::info()`
  - 返回整理后的 `DeviceInfo`
  - 平时优先使用这个方法读取设备信息

- `Device::is_accessible()`
  - 检查设备当前是否可以被打开
  - 默认按独占权限检查
  - 底层调用 `MV_CC_IsDeviceAccessible`

- `Device::open()`
  - 打开当前设备
  - 返回 `Camera`
  - 底层先调用 `MV_CC_CreateHandle`
  - 然后调用 `MV_CC_OpenDevice`
  - 当前默认使用独占访问模式 `MV_ACCESS_Exclusive`
  - 如果打开失败，会调用 `MV_CC_DestroyHandle` 释放已经创建的句柄

## 常见写法

- 查看设备列表

```rust
let devices = hik.devices()?;
for device in devices.iter() {
    println!("{:?}", device.info());
}
```

- 按条件选择并打开设备

```rust
let camera = hik.devices()?.default()?.open()?;
let camera = hik.devices()?.serial_number("DA1234567")?.open()?;
let camera = hik.devices()?.user_name("left-camera")?.open()?;
let camera = hik.devices()?.ip("192.168.1.64")?.open()?;
let camera = hik.devices()?.mac("00:11:22:33:44:55")?.open()?;
```

- 先查看信息，再选择设备

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

