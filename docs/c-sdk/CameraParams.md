# CameraParams

头文件：`crates/hikrobot-sys/includes/CameraParams.h`

这个文件不提供函数，主要提供 `MvCameraControl.h` 函数所需的结构体、枚举和宏定义

它包含：

```c
#include "PixelType.h"
```

## 设备枚举和设备信息

- `MV_SORT_METHOD`
  - 设备枚举排序方式
  - 按序列号、用户自定义名、当前 IP 升序/降序排序

- `MV_GIGE_DEVICE_INFO`
  - GigE 设备信息
  - 包含 IP 配置、当前 IP、子网掩码、网关、制造商、型号、序列号、用户自定义名、占用主机 IP、组播信息

- `MV_USB3_DEVICE_INFO`
  - USB3 Vision 设备信息
  - 包含端点、Vendor/Product ID、设备 GUID、型号、家族名、序列号、用户自定义名、USB 协议、设备地址

- `MV_CamL_DEV_INFO`
  - CameraLink 设备信息
  - 包含串口号、型号、家族名、版本、制造商、序列号

- `MV_CXP_DEVICE_INFO`
  - CoaXPress 设备信息

- `MV_CML_DEVICE_INFO`
  - 采集卡下 CameraLink 相机信息

- `MV_XOF_DEVICE_INFO`
  - XoFLink 设备信息

- `MV_GENTL_VIR_DEVICE_INFO`
  - GenTL 虚拟设备信息

- `MV_CC_DEVICE_INFO`
  - 单个设备信息
  - 关键字段：版本、MAC、`nTLayerType`、设备类型信息、`SpecialInfo`
  - `SpecialInfo` 是 union，根据传输层类型读取不同设备信息结构

- `MV_CC_DEVICE_INFO_LIST`
  - 设备列表
  - `nDeviceNum` 表示设备数量
  - `pDeviceInfo` 最多 `MV_MAX_DEVICE_NUM` 个

## 设备类型宏

传输层类型：

- `MV_UNKNOW_DEVICE`
- `MV_GIGE_DEVICE`
- `MV_1394_DEVICE`
- `MV_USB_DEVICE`
- `MV_CAMERALINK_DEVICE`
- `MV_VIR_GIGE_DEVICE`
- `MV_VIR_USB_DEVICE`
- `MV_GENTL_GIGE_DEVICE`
- `MV_GENTL_CAMERALINK_DEVICE`
- `MV_GENTL_CXP_DEVICE`
- `MV_GENTL_XOF_DEVICE`
- `MV_GENTL_VIR_DEVICE`

最大数量：

- `MV_MAX_TLS_NUM`
  - 最多传输层实例数

- `MV_MAX_DEVICE_NUM`
  - 最多设备数，当前为 256

## 采集卡信息

采集卡类型宏：

- `MV_GIGE_INTERFACE`
- `MV_CAMERALINK_INTERFACE`
- `MV_CXP_INTERFACE`
- `MV_XOF_INTERFACE`
- `MV_VIR_INTERFACE`
- `MV_LC_INTERFACE`

结构：

- `MV_INTERFACE_INFO`
  - 单个采集卡接口信息

- `MV_INTERFACE_INFO_LIST`
  - 采集卡接口列表
  - 最多 `MV_MAX_INTERFACE_NUM` 个

- `MV_GENTL_IF_INFO`
  - GenTL interface 信息

- `MV_GENTL_IF_INFO_LIST`
  - GenTL interface 列表

- `MV_GENTL_DEV_INFO`
  - GenTL device 信息

- `MV_GENTL_DEV_INFO_LIST`
  - GenTL device 列表

## 访问权限

- `MV_ACCESS_Exclusive`
  - 独占权限

- `MV_ACCESS_ExclusiveWithSwitch`
  - 可抢占后独占打开

- `MV_ACCESS_Control`
  - 控制权限

- `MV_ACCESS_ControlWithSwitch`
  - 可抢占后控制打开

- `MV_ACCESS_ControlSwitchEnable`
  - 以可被抢占的控制权限打开

- `MV_ACCESS_ControlSwitchEnableWithKey`
  - 带 key 抢占控制打开

- `MV_ACCESS_Monitor`
  - 监控只读模式

## 图像帧结构

- `MV_CHUNK_DATA_CONTENT`
  - chunk data 内容

- `MV_CC_IMAGE`
  - 通用图像结构
  - 图像处理、保存、ISP 等接口会用到

- `MV_FRAME_EXTRA_INFO_TYPE`
  - 帧附加信息类型
  - 包含无附加信息、子图、多部分

- `MV_GIGE_ZONE_INFO`
  - GigE 多区域信息

- `MV_GIGE_MULTI_PART_INFO`
  - GigE 多部分图像信息

- `MV_FRAME_OUT_INFO_EX`
  - 扩展帧信息
  - 包含宽高、像素格式、帧号、时间戳、数据长度、chunk、trigger index、I/O 状态、多区域/多部分信息等

- `MV_FRAME_OUT`
  - 主动取流返回的一帧
  - 包含图像数据指针和 `MV_FRAME_OUT_INFO_EX`

- `MV_GRAB_STRATEGY`
  - 取流策略
  - 用于 `MV_CC_SetGrabStrategy`

## 传输和匹配信息

- `MV_NETTRANS_INFO`
  - GigE 网络传输信息

- `MV_ALL_MATCH_INFO`
  - 通用匹配信息容器

- `MV_MATCH_INFO_NET_DETECT`
  - 网络检测信息

- `MV_MATCH_INFO_USB_DETECT`
  - USB 检测信息

- `MV_MATCH_TYPE_NET_DETECT`
  - 网络检测类型

- `MV_MATCH_TYPE_USB_DETECT`
  - USB 检测类型

## 显示和图像处理参数

- `MV_DISPLAY_FRAME_INFO_EX`
  - 显示一帧参数

- `MV_SAVE_IAMGE_TYPE`
  - 保存图片类型

- `MV_SAVE_IMAGE_PARAM_EX3`
  - 保存图像到内存参数

- `MV_SAVE_IMAGE_TO_FILE_PARAM_EX`
  - 保存图像到文件参数

- `MV_CC_SAVE_IMAGE_PARAM`
  - 保存图像参数

- `MV_IMG_ROTATION_ANGLE`
  - 图像旋转角度

- `MV_CC_ROTATE_IMAGE_PARAM`
  - 图像旋转参数

- `MV_IMG_FLIP_TYPE`
  - 图像翻转类型

- `MV_CC_FLIP_IMAGE_PARAM`
  - 图像翻转参数

- `MV_CC_PIXEL_CONVERT_PARAM_EX`
  - 像素格式转换参数

- `MV_CC_GAMMA_TYPE`
  - Gamma 类型

- `MV_CC_GAMMA_PARAM`
  - Gamma 参数

- `MV_CC_CCM_PARAM`
  - CCM 参数

- `MV_CC_CCM_PARAM_EX`
  - 扩展 CCM 参数

- `MV_CC_CONTRAST_PARAM`
  - 对比度参数

- `MV_CC_FRAME_SPEC_INFO`
  - 帧水印/特殊信息

- `MV_CC_PURPLE_FRINGING_PARAM`
  - 去紫边参数

- `MV_CC_ISP_CONFIG_PARAM`
  - ISP 配置参数

- `MV_CC_HB_DECODE_PARAM`
  - HB 无损解码参数

- `MV_RECORD_FORMAT_TYPE`
  - 录像格式

- `MV_CC_RECORD_PARAM`
  - 录像参数

- `MV_CC_INPUT_FRAME_INFO`
  - 输入一帧录像数据

- `MV_CC_INPUT_FRAME_INFO_EX`
  - 扩展输入一帧录像数据

- `MV_RECONSTRUCT_IMAGE_PARAM`
  - 图像重构参数

- `MV_OUTPUT_IMAGE_INFO`
  - 图像重构输出图像信息

## 常用相机枚举值

这些主要给旧接口或直接设置枚举节点时使用：

- `MV_CAM_ACQUISITION_MODE`
  - 采集模式

- `MV_CAM_GAIN_MODE`
  - 增益模式

- `MV_CAM_EXPOSURE_MODE`
  - 曝光模式

- `MV_CAM_EXPOSURE_AUTO_MODE`
  - 自动曝光模式

- `MV_CAM_TRIGGER_MODE`
  - 触发模式

- `MV_CAM_GAMMA_SELECTOR`
  - Gamma selector

- `MV_CAM_BALANCEWHITE_AUTO`
  - 自动白平衡

- `MV_CAM_TRIGGER_SOURCE`
  - 触发源

## GigE 和 CameraLink 常量

IP 配置：

- `MV_IP_CFG_STATIC`
- `MV_IP_CFG_DHCP`
- `MV_IP_CFG_LLA`

网络传输：

- `MV_NET_TRANS_DRIVER`
- `MV_NET_TRANS_SOCKET`

CameraLink 波特率：

- `MV_CAML_BAUDRATE_9600`
- `MV_CAML_BAUDRATE_19200`
- `MV_CAML_BAUDRATE_38400`
- `MV_CAML_BAUDRATE_57600`
- `MV_CAML_BAUDRATE_115200`
- `MV_CAML_BAUDRATE_230400`
- `MV_CAML_BAUDRATE_460800`
- `MV_CAML_BAUDRATE_921600`
- `MV_CAML_BAUDRATE_AUTOMAX`

## 事件、文件和 Action Command

- `MV_CC_STREAM_EXCEPTION_TYPE`
  - 流异常类型

- `MV_CC_STREAM_EXCEPTION_INFO`
  - 流异常信息

- `MV_EVENT_OUT_INFO`
  - 事件输出信息

- `MV_CC_FILE_ACCESS`
  - 相机文件访问

- `MV_CC_FILE_ACCESS_EX`
  - 扩展相机文件访问

- `MV_CC_FILE_ACCESS_PROGRESS`
  - 文件访问进度

- `MV_GIGE_TRANSMISSION_TYPE`
  - GigE 传输类型

- `MV_TRANSMISSION_TYPE`
  - 传输类型参数

- `MV_ACTION_CMD_INFO`
  - Action Command 参数

- `MV_ACTION_CMD_RESULT`
  - Action Command 单个结果

- `MV_ACTION_CMD_RESULT_LIST`
  - Action Command 结果列表

## GenICam 节点值结构

- `MV_XML_InterfaceType`
  - XML 节点类型

- `MV_XML_AccessMode`
  - XML 节点访问模式

- `MVCC_NODE_NAME`
  - 节点名称

- `MVCC_NODE_NAME_LIST`
  - 节点名称列表

- `MVCC_NODE_ERR_TYPE`
  - 节点错误类型

- `MVCC_NODE_ERROR`
  - 单个节点错误

- `MVCC_NODE_ERROR_LIST`
  - 节点错误列表

- `MVCC_ENUMVALUE`
  - enum 节点值

- `MVCC_ENUMVALUE_EX`
  - 扩展 enum 节点值，最多 256 个支持值

- `MVCC_ENUMENTRY`
  - enum 条目符号

- `MVCC_INTVALUE`
  - integer 节点值

- `MVCC_INTVALUE_EX`
  - 扩展 integer 节点值，支持 64 位值

- `MVCC_FLOATVALUE`
  - float 节点值

- `MVCC_STRINGVALUE`
  - string 节点值

## 绘图辅助结构

- `MVCC_COLORF`
  - 颜色

- `MVCC_POINTF`
  - 点坐标

- `MVCC_RECT_INFO`
  - 矩形辅助线

- `MVCC_CIRCLE_INFO`
  - 圆形辅助线

- `MVCC_LINES_INFO`
  - 线段辅助线

## 串口和液态镜头

- `MV_CAML_SERIAL_PORT`
  - CameraLink 串口

- `MV_CAML_SERIAL_PORT_LIST`
  - CameraLink 串口列表

- `MV_CC_LIQUIDLENS_MSG_TYPE`
  - 液态镜头消息类型

- `MV_CC_LIQUIDLENS_EXCEPTION_TYPE`
  - 液态镜头异常类型

- `MV_CC_LIQUIDLENS_MSG`
  - 液态镜头消息

- `MV_CC_LIQUIDLENS_EXCEPTION_MSG`
  - 液态镜头异常消息

- `MV_CC_LIQUIDLENS_INFO`
  - 液态镜头基本信息

- `MV_CC_LIQUIDLENS_RANGE_SCAN_PARAM`
  - 光焦度区间扫描参数

- `MV_CC_LIQUIDLENS_AUTOFOCUS_PARAM`
  - 自动对焦参数

- `MV_CC_LIQUIDLENS_MULTI_FP_SCAN_PARAM`
  - 多焦度扫描参数

- `MV_CC_LIQUIDLENS_USERSET`
  - 液态镜头用户集
