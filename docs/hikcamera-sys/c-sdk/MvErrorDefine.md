头文件：`conda-packages/hikcamera-mvs/sources/5.0.1-20260512/include/MvErrorDefine.h`

这个文件定义 MVS C SDK 主错误码

大多数 `MV_CC_*`、`MV_GIGE_*`、`MV_USB_*`、`MV_CAML_*` 函数成功时返回 `MV_OK`，失败时返回这里的错误码

## 成功

- `MV_OK`：成功，无错误

## 通用错误

- `MV_E_HANDLE`：错误或无效句柄
- `MV_E_SUPPORT`：不支持的功能
- `MV_E_BUFOVER`：缓存已满
- `MV_E_CALLORDER`：函数调用顺序错误
- `MV_E_PARAMETER`：参数错误
- `MV_E_RESOURCE`：资源申请失败，常见是内存不足
- `MV_E_NODATA`：超时，未收到数据
- `MV_E_PRECONDITION`：前置条件错误，或运行环境已经变化
- `MV_E_VERSION`：版本不匹配
- `MV_E_NOENOUGH_BUF`：输入缓存太小
- `MV_E_ABNORMAL_IMAGE`：异常图像，可能是丢包导致图像不完整
- `MV_E_LOAD_LIBRARY`：动态库加载失败
- `MV_E_NOOUTBUF`：没有可输出的缓存
- `MV_E_ENCRYPT`：加密错误
- `MV_E_OPENFILE`：打开文件错误
- `MV_E_BUF_IN_USE`：缓存地址已使用
- `MV_E_BUF_INVALID`：无效缓存地址
- `MV_E_NOALIGN_BUF`：缓存对齐异常
- `MV_E_NOENOUGH_BUF_NUM`：缓存个数不足
- `MV_E_PORT_IN_USE`：串口被占用
- `MV_E_DEV_OFFLINE`：设备掉线
- `MV_E_DEV_SUPPORT`：设备不支持
- `MV_E_PLATFORM_SUPPORT`：当前平台未实现
- `MV_E_RESOURCE_IN_USE`：资源已被占用

## 设备返回错误

- `MV_E_DEV_NOT_IMPLEMENTED`：命令在设备中未实现
- `MV_E_DEV_INVALID_PARAMETER`：命令参数无效或超出范围
- `MV_E_DEV_INVALID_ADDRESS`：访问不存在的寄存器地址
- `MV_E_DEV_WRITE_PROTECT`：尝试写只读寄存器
- `MV_E_DEV_BAD_ALIGNMENT`：寄存器地址未按底层技术对齐
- `MV_E_DEV_ACCESS_DENIED`：访问无权限
- `MV_E_DEV_BUSY`：设备忙
- `MV_E_DEV_MSG_TIMEOUT`：设备应答超时
- `MV_E_DEV_UNKNOWN`：设备返回未知错误码

## GenICam 错误

- `MV_E_GC_GENERIC`：GenICam 通用错误
- `MV_E_GC_ARGUMENT`：参数非法
- `MV_E_GC_RANGE`：值超出范围
- `MV_E_GC_PROPERTY`：属性错误
- `MV_E_GC_RUNTIME`：运行时错误
- `MV_E_GC_LOGICAL`：逻辑错误
- `MV_E_GC_ACCESS`：访问错误
- `MV_E_GC_TIMEOUT`：超时
- `MV_E_GC_DYNAMICCAST`：类型转换错误
- `MV_E_GC_UNKNOW`：GenICam 未知错误
- `MV_E_GC_NODE_NOT_FOUND`：节点不存在
- `MV_E_GC_NODE_VERIFY`：节点校验失败
- `MV_E_GC_FILE`：GenICam 文件异常
- `MV_E_GC_URL_DESC`：设备 XML URL 描述异常

## 网络错误

- `MV_E_NOT_IMPLEMENTED`：命令不被设备支持
- `MV_E_INVALID_ADDRESS`：访问目标地址不存在
- `MV_E_WRITE_PROTECT`：目标地址不可写
- `MV_E_ACCESS_DENIED`：无权限
- `MV_E_BUSY`：设备忙或网络断开
- `MV_E_PACKET`：网络包数据错误
- `MV_E_NETER`：网络相关错误
- `MV_E_DRIVERATTACH`：GigE 驱动未绑定
- `MV_E_PACKET_ID_MISMATCH`：包 ID 不匹配
- `MV_E_IMAGE_BUFFER_OVERFLOW`：图像缓冲区溢出
- `MV_E_NO_BUFFER_FOR_USE`：没有可用缓冲区
- `MV_E_XML_INFO_PACKET_ERR`：XML 信息包解析错误
- `MV_E_TIMEOUT`：网络超时
- `MV_E_NET_TRANSMISSION_TYPE_ERR`：网络传输类型参数错误
- `MV_E_IP_CONFLICT`：设备 IP 冲突

## USB 错误

- `MV_E_USB_READ`：USB 读错误
- `MV_E_USB_WRITE`：USB 写错误
- `MV_E_USB_DEVICE`：USB 设备异常
- `MV_E_USB_GENICAM`：USB GenICam 相关错误
- `MV_E_USB_BANDWIDTH`：USB 带宽不足
- `MV_E_USB_DRIVER`：USB 驱动不匹配或未安装
- `MV_E_USB_UNKNOW`：USB 未知错误

## 升级错误

- `MV_E_UPG_FILE_MISMATCH`：固件不匹配
- `MV_E_UPG_LANGUSGE_MISMATCH`：固件语言不匹配
- `MV_E_UPG_CONFLICT`：升级冲突
- `MV_E_UPG_INNER_ERR`：升级时设备内部错误
- `MV_E_UPG_UNKNOW`：升级未知错误

## 图像处理错误

- `MV_E_SUPPORT_PIXEL_FORMAT`：不支持的像素格式
- `MV_E_SUPPORT_IMAGE_TYPE`：不支持的图像类型
- `MV_E_NOENOUGH_INPUT_DATA`：输入图像数据不足

## 显示渲染错误

- `MV_E_SR_NOT_INITIAL`：未初始化
- `MV_E_SR_SUPPORT_FUNCTION`：不支持接口
- `MV_E_SR_SUPPORT_ENGINE`：不支持渲染引擎
- `MV_E_SR_SUPPORT_PIXELTYPE`：不支持像素格式
- `MV_E_SR_SUPPORT_TEXTURESIZE`：不支持纹理大小
- `MV_E_SR_SUPPORT_WND`：不支持显示窗口
- `MV_E_SR_RUNTIME`：运行环境错误

## 液态镜头错误

- `MV_E_LIQUIDLENS_CMD_NOT_SUPPORT`：不支持命令
- `MV_E_LIQUIDLENS_REGISTER_NOT_EXIST`：寄存器不存在
- `MV_E_LIQUIDLENS_PERMISSION_DENIED`：权限错误
- `MV_E_LIQUIDLENS_CHECKSUM_ERROR`：checksum 校验失败
- `MV_E_LIQUIDLENS_DEVICE_BUSY`：设备忙
- `MV_E_LIQUIDLENS_COMMANDTIMEOUT`：镜头指令超时
- `MV_E_LIQUIDLENS_OFFLINE`：镜头离线
- `MV_E_LIQUIDLENS_NOT_SOFT_TRIGGER_MODE`：不是软触发模式
- `MV_E_LIQUIDLENS_DEVICE_NOT_GRABBING`：设备未处于取流状态
- `MV_E_LIQUIDLENS_AF_NOT_CONVERGED`：自动对焦未收敛
- `MV_E_LIQUIDLENS_UNDEFINED_ERROR`：未定义错误
