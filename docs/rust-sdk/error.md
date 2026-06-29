# error

源码文件：`crates/hikcamera/src/error.rs`

这个文件负责统一 Rust 高层 SDK 的错误类型。

不管错误来自海康 C SDK 返回码，还是来自 Rust 封装层自己的显式检查，都统一返回 `HikCameraError`。

对应 C SDK：

```c
MV_OK
MV_E_*
MV_ALG_*
MvErrorDefine.h
MvISPErrorDefine.h
```

## 提供的结构体和类型

- `Result<T>`
  - Rust 高层 SDK 统一使用的返回类型
  - 等价于 `std::result::Result<T, HikCameraError>`

- `HikCameraError`
  - Rust 高层 SDK 的统一错误类型
  - 使用 `thiserror` 派生 `Display` 和 `std::error::Error`
  - `Sdk { status }` 表示 C SDK 返回码错误
  - `NoDevice` 表示没有枚举到任何设备
  - `DeviceNotFound` 表示按指定条件没有找到设备
  - `MultipleDevices` 表示按指定条件匹配到多台设备
  - `NullHandle` 表示 C SDK 返回成功但没有给出有效相机句柄
  - `SdkStatePoisoned` 表示 SDK 初始化引用计数锁已经被 panic 污染
  - `InvalidString` 表示传给 C SDK 的路径、节点名或节点字符串包含 NUL 字节
  - `UnsupportedNode` 表示 `get_node()` / `set_node()` 暂不支持该 GenICam 节点类型
  - `NodeValueMismatch` 表示 `NodeValue::into_*()` 或 `as_*()` 取错类型
  - `NodeInputMismatch` 表示 `set_node()` 传入值类型和节点类型不匹配
  - `ValueOutOfRange` 表示封装层在调用 SDK 前发现数值超出可传范围
  - `RecordingInProgress` 表示同一个 `Stream` 上已经有视频写入器在录像
  - `EmptyFrame` 表示要保存、编码、转换或写入视频的帧没有图像数据
  - `EmptyVideo` 表示视频写入器没有写入任何帧就结束
  - `InvalidDuration` 表示传入的持续时间不合法
  - `InvalidFrameRate` 表示传入的视频帧率或目标采集帧率不合法
  - `InvalidRoi` 表示 ROI 宽度或高度为 0
  - 标记为 `non_exhaustive`，后续可以继续增加封装层错误类型

- `Status`
  - C SDK 返回状态码的强类型封装
  - `Status::info()` 返回 SDK 状态码名称和英文说明
  - SDK 错误使用 `MV_E_*`、`MV_ALG_*` 等 C SDK 名称

- `StatusInfo`
  - SDK 状态码翻译信息
  - 保存 SDK 状态码名称和英文说明
  - 仅用于 `Status`，Rust 封装层错误直接使用 `thiserror` 的 `Display` 文案

## 提供的函数

- `HikCameraError::code()`
  - SDK 错误返回 `Some(i32)`
  - Rust 封装层错误返回 `None`
  - 用于日志、调试，或者对照海康官方文档


## 内部错误转换

- `check(code)`
  - crate 内部使用
  - 把 C SDK 返回码转换成 Rust 的 `Result<()>`
  - `MV_OK` 返回 `Ok(())`
  - 非 `MV_OK` 返回 `Err(HikCameraError::Sdk { status })`

- wrapper 层错误直接构造对应的 `HikCameraError` enum variant，不再维护内部薄封装构造函数。

## Display 输出

`HikCameraError` 的 `Display` 和 `std::error::Error` 实现由 `thiserror` 派生，输出内容保持为面向日志和调试的英文信息。

- SDK 错误格式
  - `状态码名称 (十进制状态码, 十六进制状态码): 英文说明`

- 示例
  - `MV_E_LOAD_LIBRARY (-2147483636, 0x8000000C): Failed to load dynamic library`

- Rust 封装层错误
  - `NoDevice` 输出固定英文说明
  - `DeviceNotFound` 输出英文说明和选择条件
  - `MultipleDevices` 输出匹配数量和选择条件
  - node、字符串、数值范围、录像状态等错误会输出具体字段或节点信息

## 封装层显式错误

这些错误不是 C SDK 返回码，而是 Rust 封装层在调用 SDK 前后能明确判断的问题。它们的 `code()` 都返回 `None`。

- `NO_DEVICE`
  - 设备列表为空时返回
  - 例如 `devices.default()?`

- `DEVICE_NOT_FOUND`
  - 按序列号、用户名、IP、MAC 查找设备失败时返回

- `MULTIPLE_DEVICES`
  - 按某个选择条件匹配到多台设备时返回

- `NULL_HANDLE`
  - `MV_CC_CreateHandle` 返回成功，但输出 handle 仍为空时返回

- `SDK_STATE_POISONED`
  - SDK 初始化/释放引用计数锁被 panic 污染时返回
  - `HikCamera::new()` 会显式返回这个错误

- `INVALID_STRING`
  - 路径、节点名或节点字符串包含 C 字符串不能表示的 NUL 字节时返回
  - 避免在 `CString` 转换时 panic

- `UNSUPPORTED_NODE`
  - `get_node()` 读取了当前薄封装不支持的 GenICam 节点类型时返回
  - `set_node()` 写入 `Command` 节点时也会返回；命令节点应使用 `execute_node()`

- `NODE_VALUE_MISMATCH`
  - `NodeValue` 已经读取成功，但用户用错了取值方法时返回
  - 例如把 `NodeValue::Float` 调用 `into_int()`

- `NODE_INPUT_MISMATCH`
  - `set_node()` 的 Rust 输入类型和节点类型不匹配时返回
  - 例如浮点节点传入字符串

- `VALUE_OUT_OF_RANGE`
  - 超时时间、图像尺寸、编码缓存大小等在封装层转换成 SDK 参数前就已经越界时返回

- `RECORDING_IN_PROGRESS`
  - 同一个 `Stream` 已经有一个 `VideoWriter` 开始录像，又创建另一个视频写入器并写帧时返回

- `EMPTY_FRAME`
  - 保存图片、编码图片、图像处理或写入视频前发现 `Frame` 没有图像数据时返回

- `EMPTY_VIDEO`
  - `VideoWriter::finish()` 在没有写入任何帧时返回

- `INVALID_DURATION`
  - `Camera::take_video()` 的录制时长为 0 时返回

- `INVALID_FRAME_RATE`
  - `Camera::take_video()`、`Stream::save_video()`、`VideoOptions` 或 `Fps::Target` 的帧率不是有限正数时返回

- `INVALID_ROI`
  - `set_roi()` 的宽度或高度为 0 时返回

## 已覆盖的状态码

- 通用状态码
  - `MV_OK`：Success - 成功
  - `MV_E_HANDLE`：Invalid handle - 错误或无效的句柄
  - `MV_E_SUPPORT`：Unsupported function - 不支持的功能
  - `MV_E_BUFOVER`：Buffer overflow - 缓存已满
  - `MV_E_CALLORDER`：Invalid function call order - 函数调用顺序错误
  - `MV_E_PARAMETER`：Invalid parameter - 错误的参数
  - `MV_E_RESOURCE`：Resource allocation failed - 资源申请失败
  - `MV_E_NODATA`：Timeout or no data received - 超时或未收到数据
  - `MV_E_PRECONDITION`：Precondition failed or runtime environment changed - 前置条件有误或运行环境变化
  - `MV_E_VERSION`：Version mismatch - 版本不匹配
  - `MV_E_NOENOUGH_BUF`：Input buffer is too small - 传入缓存空间不足
  - `MV_E_ABNORMAL_IMAGE`：Abnormal or incomplete image - 异常图像或图像不完整
  - `MV_E_LOAD_LIBRARY`：Failed to load dynamic library - 动态库加载失败
  - `MV_E_NOOUTBUF`：No output buffer available - 没有可输出的缓存
  - `MV_E_ENCRYPT`：Encryption error - 加密错误
  - `MV_E_OPENFILE`：Failed to open file - 打开文件失败
  - `MV_E_DEV_OFFLINE`：Device is offline - 设备已掉线
  - `MV_E_DEV_BUSY`：Device is busy - 设备正忙
  - `MV_E_INTERNAL`：SDK internal error - SDK 内部错误
  - `MV_E_UNKNOW`：Unknown error - 未知错误
  - 其他 `MvErrorDefine.h` 中的通用错误码也按同样方式翻译

- GenICam 状态码
  - `MV_E_GC_GENERIC`：GenICam general error - GenICam 通用错误
  - `MV_E_GC_ARGUMENT`：GenICam invalid argument - GenICam 参数非法
  - `MV_E_GC_RANGE`：GenICam value out of range - GenICam 值超出范围
  - `MV_E_GC_ACCESS`：GenICam node access error - GenICam 节点访问条件错误
  - `MV_E_GC_TIMEOUT`：GenICam timeout - GenICam 超时
  - `MV_E_GC_NODE_NOT_FOUND`：GenICam node not found - GenICam 节点不存在
  - `MV_E_GC_NODE_VERIFY`：GenICam node validation failed - GenICam 节点校验失败
  - `MV_E_GC_FILE`：GenICam file error - GenICam 文件异常
  - `MV_E_GC_UNKNOW`：GenICam unknown error - GenICam 未知错误

- GigE 状态码
  - `MV_E_NOT_IMPLEMENTED`：GigE command is not supported by device - GigE 设备不支持该命令
  - `MV_E_INVALID_ADDRESS`：GigE target address does not exist - GigE 目标地址不存在
  - `MV_E_WRITE_PROTECT`：GigE target address is not writable - GigE 目标地址不可写
  - `MV_E_ACCESS_DENIED`：GigE access denied - GigE 设备无访问权限
  - `MV_E_BUSY`：GigE device is busy or network is disconnected - GigE 设备忙或网络断开
  - `MV_E_PACKET`：GigE packet data error - GigE 网络包数据错误
  - `MV_E_NETER`：GigE network error - GigE 网络错误
  - `MV_E_DRIVERATTACH`：GigE driver is not attached - GigE 驱动未绑定
  - `MV_E_TIMEOUT`：GigE timeout - GigE 超时
  - `MV_E_DEV_DISCONNECT`：GigE device disconnected - GigE 设备已断开
  - `MV_E_IP_CONFLICT`：Device IP conflict - 设备 IP 冲突

- USB 状态码
  - `MV_E_USB_READ`：USB read error - USB 读取错误
  - `MV_E_USB_WRITE`：USB write error - USB 写入错误
  - `MV_E_USB_DEVICE`：USB device exception - USB 设备异常
  - `MV_E_USB_GENICAM`：USB GenICam error - USB GenICam 相关错误
  - `MV_E_USB_BANDWIDTH`：USB bandwidth is insufficient - USB 带宽不足
  - `MV_E_USB_DRIVER`：USB driver mismatch or driver is not installed - USB 驱动不匹配或未安装
  - `MV_E_USB_UNKNOW`：USB unknown error - USB 未知错误

- 图像处理和渲染状态码
  - `MV_E_SUPPORT_PIXEL_FORMAT`：Unsupported pixel format - 不支持的像素格式
  - `MV_E_SUPPORT_IMAGE_TYPE`：Unsupported image type - 不支持的图像类型
  - `MV_E_NOENOUGH_INPUT_DATA`：Insufficient input image data - 输入图像数据不足
  - `MV_E_SR_NOT_INITIAL`：Render module is not initialized - 渲染模块未初始化
  - `MV_E_SR_SUPPORT_FUNCTION`：Render function is not supported - 渲染接口不支持
  - `MV_E_SR_LOAD_LIBRARY`：Render module failed to load dynamic library - 渲染模块动态库加载失败
  - `MV_E_SR_RUNTIME`：Render runtime error - 渲染运行环境错误

- 液态镜头状态码
  - `MV_E_LIQUIDLENS_CMD_NOT_SUPPORT`：Liquid lens command is not supported - 液态镜头不支持该命令
  - `MV_E_LIQUIDLENS_PERMISSION_DENIED`：Liquid lens permission denied - 液态镜头权限错误
  - `MV_E_LIQUIDLENS_DEVICE_BUSY`：Liquid lens device is busy - 液态镜头设备忙
  - `MV_E_LIQUIDLENS_COMMANDTIMEOUT`：Liquid lens command timeout - 液态镜头指令超时
  - `MV_E_LIQUIDLENS_OFFLINE`：Liquid lens is offline - 液态镜头离线
  - `MV_E_LIQUIDLENS_INIT_FAILED`：Liquid lens initialization failed - 液态镜头初始化失败
  - `MV_E_LIQUIDLENS_UNDEFINED_ERROR`：Liquid lens undefined error - 液态镜头未定义错误

- ISP 算法库状态码
  - `MV_ALG_ERR`：ISP algorithm unknown error - ISP 算法库未知错误
  - `MV_ALG_E_MEM_NULL`：ISP algorithm memory address is null - 内存地址为空
  - `MV_ALG_E_MEM_LACK`：ISP algorithm memory is insufficient - 内存空间不足
  - `MV_ALG_E_IMG_FORMAT`：ISP algorithm image format is invalid or unsupported - 图像格式不正确或不支持
  - `MV_ALG_E_IMG_SIZE`：ISP algorithm image size is invalid or out of range - 图像尺寸不正确或超出范围
  - `MV_ALG_E_PARAM_VALUE`：ISP algorithm parameter value is invalid or out of range - 参数值不正确或超出范围
  - `MV_ALG_E_TIME_OUT`：ISP algorithm timeout - 算法库超时
  - `MV_ALG_E_FILE_OPEN`：ISP algorithm failed to open file - 打开文件失败
  - `MV_ALG_E_FILE_READ`：ISP algorithm failed to read file - 读取文件失败
  - `MV_ALG_E_FILE_WRITE`：ISP algorithm failed to write file - 写入文件失败
  - `MV_ALG_E_MALLOC_MEM`：ISP algorithm memory allocation failed - 分配内存失败





