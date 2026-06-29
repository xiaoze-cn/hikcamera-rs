# MvCameraControl

头文件：`crates/hikcamera-sys/includes/MvCameraControl.h`

这是海康 MVS C SDK 的主入口头文件，`hikcamera-sys/MvCamera.h` 也是通过包含它来暴露 SDK

它本身继续包含：

```c
#include "MvErrorDefine.h"
#include "CameraParams.h"
#include "MvObsoleteInterfaces.h"
```

所以实际使用 C SDK 时，大多数函数从这里找，结构体、常量、像素格式和错误码再跳到对应头文件

## 文件提供的能力

`MvCameraControl.h` 按官方注释分成 14 个部分：

- 回调函数定义
- SDK 初始化和版本信息
- 相机枚举、打开、关闭和取流
- 采集卡枚举、打开、关闭
- 相机/采集卡属性万能配置、寄存器、XML、文件访问
- 相机/采集卡升级
- 异常回调和事件接口
- GigE Vision 专用接口
- CameraLink 专用接口
- USB3 Vision 专用接口
- GenTL 相关接口
- 图像保存、格式转换、ISP、录像、图像重构
- 支持串口通信的设备接口
- 支持液态镜头的设备接口

## 回调函数定义

提供图像、异常、事件、液态镜头相关的回调函数类型

- `MvImageCallback`
  - 旧图像回调，主要给废弃接口使用

- `MvImageCallbackEx`
  - 图像回调，返回图像数据和 `MV_FRAME_OUT_INFO_EX`

- `MvImageCallbackEx2`
  - 扩展图像回调，配合 `bAutoFree` 控制缓存释放

- `MvExceptionCallback`
  - 设备异常回调

- `MvEventCallback`
  - 相机事件回调

- `MvStreamExceptionCallback`
  - 流异常回调

- `MvLiquidLensMsgCallback`
  - 液态镜头消息回调

- `MvLiquidLensExceptionCallback`
  - 液态镜头异常回调

## SDK 初始化

- `MV_CC_Initialize()`
  - 初始化 SDK 全局运行环境

- `MV_CC_Finalize()`
  - 反初始化 SDK，释放全局资源

- `MV_CC_GetSDKVersion()`
  - 获取 SDK 版本号

典型流程是程序启动时初始化，结束前反初始化

## 相机控制和取流

### 枚举和打开

- `MV_CC_EnumDevices(nTLayerType, pstDevList)`
  - 按传输层枚举设备

- `MV_CC_EnumDevicesEx(nTLayerType, pstDevList, strManufacturerName)`
  - 按传输层和制造商枚举设备

- `MV_CC_EnumDevicesEx2(nTLayerType, pstDevList, strManufacturerName, enSortMethod)`
  - 枚举设备并排序

- `MV_CC_EnumDevicesByInterface(handle, pstDevList)`
  - 通过采集卡接口枚举其下设备

- `MV_CC_IsDeviceAccessible(pstDevInfo, nAccessMode)`
  - 检查设备是否能用指定权限打开

- `MV_CC_CreateHandle(handle, pstDevInfo)`
  - 根据 `MV_CC_DEVICE_INFO` 创建设备句柄

- `MV_CC_OpenDevice(handle, nAccessMode, nSwitchoverKey)`
  - 打开设备

- `MV_CC_IsDeviceConnected(handle)`
  - 判断设备是否仍连接

- `MV_CC_GetDeviceInfo(handle, pstDevInfo)`
  - 获取当前设备信息

- `MV_CC_GetAllMatchInfo(handle, pstInfo)`
  - 获取匹配信息，例如网络流量、丢包、USB 接收字节数

- `MV_CC_CloseDevice(handle)`
  - 关闭设备连接

- `MV_CC_DestroyHandle(handle)`
  - 销毁设备句柄

### 取流

- `MV_CC_RegisterImageCallBackEx(handle, cbOutput, pUser)`
  - 注册图像数据回调

- `MV_CC_RegisterImageCallBackEx2(handle, cbOutput, pUser, bAutoFree)`
  - 注册图像数据回调，并控制回调后是否自动释放缓存

- `MV_CC_StartGrabbing(handle)`
  - 开始采集

- `MV_CC_StopGrabbing(handle)`
  - 停止采集

- `MV_CC_GetImageBuffer(handle, pstFrame, nMsec)`
  - 主动获取一帧 SDK 管理的图像缓存

- `MV_CC_FreeImageBuffer(handle, pstFrame)`
  - 释放 `MV_CC_GetImageBuffer` 取得的缓存权限

- `MV_CC_GetOneFrameTimeout(handle, pData, nDataSize, pstFrameInfo, nMsec)`
  - 主动获取一帧到用户分配的缓存

- `MV_CC_ClearImageBuffer(handle)`
  - 清空内部图像缓存

- `MV_CC_GetValidImageNum(handle, pnValidImageNum)`
  - 获取当前可输出的有效图像数量

### 取流缓存

- `MV_CC_SetImageNodeNum(handle, nNum)`
  - 设置 SDK 内部图像缓存节点数

- `MV_CC_SetGrabStrategy(handle, enGrabStrategy)`
  - 设置取流策略

- `MV_CC_SetOutputQueueSize(handle, nOutputQueueSize)`
  - 设置 LatestImages 策略下的输出队列大小

- `MV_CC_AllocAlignedBuffer(nBufSize, nAlignment)`
  - 申请对齐内存

- `MV_CC_FreeAlignedBuffer(pBuffer)`
  - 释放对齐内存

- `MV_CC_GetPayloadSize(handle, pnPayloadSize, pnAlignment)`
  - 获取 payload 大小和对齐要求

- `MV_CC_RegisterBuffer(handle, pBuffer, nBufSize, pUser)`
  - 注册外部缓存给 SDK 使用

- `MV_CC_UnRegisterBuffer(handle, pBuffer)`
  - 取消注册外部缓存

### 显示和绘制

- `MV_CC_DisplayOneFrameEx(handle, hWnd, pstDisplayInfo)`
  - 显示一帧图像

- `MV_CC_DisplayOneFrameEx2(handle, hWnd, pstImage, enRenderMode)`
  - 用 `MV_CC_IMAGE` 显示一帧图像

- `MV_CC_DrawRect(handle, pRectInfo)`
  - 画矩形辅助线

- `MV_CC_DrawCircle(handle, pCircleInfo)`
  - 画圆形辅助线

- `MV_CC_DrawLines(handle, pLinesInfo)`
  - 画线段辅助线

## 采集卡配置

这些函数管理 frame grabber/interface，不是普通相机 handle

- `MV_CC_EnumInterfaces(nTLayerType, pInterfaceInfoList)`
  - 枚举采集卡接口

- `MV_CC_CreateInterface(handle, pInterfaceInfo)`
  - 根据采集卡信息创建接口句柄

- `MV_CC_CreateInterfaceByID(handle, pInterfaceID)`
  - 根据接口 ID 创建接口句柄

- `MV_CC_OpenInterface(handle, pReserved)`
  - 打开采集卡接口

- `MV_CC_CloseInterface(handle)`
  - 关闭采集卡接口

- `MV_CC_DestroyInterface(handle)`
  - 销毁采集卡接口句柄

## 属性、寄存器、XML、文件

### 万能属性接口

这一组直接通过 GenICam 节点名读写属性

- `MV_CC_GetIntValueEx(handle, strKey, pstIntValue)`
- `MV_CC_SetIntValueEx(handle, strKey, nValue)`
- `MV_CC_GetEnumValue(handle, strKey, pstEnumValue)`
- `MV_CC_GetEnumValueEx(handle, strKey, pstEnumValue)`
- `MV_CC_SetEnumValue(handle, strKey, nValue)`
- `MV_CC_GetEnumEntrySymbolic(handle, strKey, pstEnumEntry)`
- `MV_CC_SetEnumValueByString(handle, strKey, strValue)`
- `MV_CC_GetFloatValue(handle, strKey, pstFloatValue)`
- `MV_CC_SetFloatValue(handle, strKey, fValue)`
- `MV_CC_GetBoolValue(handle, strKey, pbValue)`
- `MV_CC_SetBoolValue(handle, strKey, bValue)`
- `MV_CC_GetStringValue(handle, strKey, pstStringValue)`
- `MV_CC_SetStringValue(handle, strKey, strValue)`
- `MV_CC_SetCommandValue(handle, strKey)`

常见节点名：

```text
Width
Height
ExposureTime
Gain
PixelFormat
TriggerMode
TriggerSource
TriggerSoftware
DeviceUserID
```

### 节点类型和参数文件

- `MV_XML_GetNodeAccessMode(handle, strName, penAccessMode)`
  - 获取节点访问模式

- `MV_XML_GetNodeInterfaceType(handle, strName, penInterfaceType)`
  - 获取节点类型

- `MV_CC_FeatureLoad(handle, strFileName)`
  - 导入设备属性

- `MV_CC_FeatureLoadEx(handle, strFileName, pstNodeErrorList)`
  - 导入设备属性并返回失败节点列表

- `MV_CC_FeatureSave(handle, strFileName)`
  - 保存设备属性

### 寄存器和 XML

- `MV_CC_ReadMemory(handle, pBuffer, nAddress, nLength)`
  - 读设备寄存器/内存

- `MV_CC_WriteMemory(handle, pBuffer, nAddress, nLength)`
  - 写设备寄存器/内存

- `MV_CC_InvalidateNodes(handle)`
  - 使节点缓存失效

- `MV_XML_GetGenICamXML(handle, pData, nDataSize, pnDataLen)`
  - 获取设备 GenICam XML

### 相机文件

- `MV_CC_FileAccessRead(handle, pstFileAccess)`
- `MV_CC_FileAccessReadEx(handle, pstFileAccessEx)`
- `MV_CC_FileAccessWrite(handle, pstFileAccess)`
- `MV_CC_FileAccessWriteEx(handle, pstFileAccessEx)`
- `MV_CC_GetFileAccessProgress(handle, pstFileAccessProgress)`

## 升级

- `MV_CC_LocalUpgrade(handle, strFilePathName)`
  - 本地升级相机或采集卡

- `MV_CC_GetUpgradeProcess(handle, pnProcess)`
  - 获取升级进度

## 异常和事件

- `MV_CC_RegisterExceptionCallBack(handle, cbException, pUser)`
  - 注册设备异常回调

- `MV_CC_RegisterAllEventCallBack(handle, cbEvent, pUser)`
  - 注册所有事件回调

- `MV_CC_RegisterEventCallBackEx(handle, strEventName, cbEvent, pUser)`
  - 注册指定事件回调

- `MV_CC_RegisterStreamExceptionCallBack(handle, cbStreamException, pUser)`
  - 注册流异常回调

- `MV_CC_EventNotificationOn(handle, strEventName)`
  - 打开指定事件通知

- `MV_CC_EventNotificationOff(handle, strEventName)`
  - 关闭指定事件通知

## 专用接口

- `MV_GIGE_SetEnumDevTimeout(nMilTimeout)`
  - 设置 GigE 枚举超时

- `MV_GIGE_ForceIpEx(handle, nIP, nSubNetMask, nDefaultGateWay)`
  - 强制设置 IP、子网掩码、网关

- `MV_GIGE_SetIpConfig(handle, nType)`
  - 设置 IP 获取方式

- `MV_GIGE_SetNetTransMode(handle, nType)`
  - 设置网络传输模式

- `MV_GIGE_GetNetTransInfo(handle, pstInfo)`
  - 获取网络传输信息

- `MV_GIGE_SetDiscoveryMode(nMode)`
  - 设置枚举回复包类型

- `MV_GIGE_SetGvspTimeout(handle, nMillisec)`
- `MV_GIGE_GetGvspTimeout(handle, pnMillisec)`
- `MV_GIGE_SetGvcpTimeout(handle, nMillisec)`
- `MV_GIGE_GetGvcpTimeout(handle, pnMillisec)`
- `MV_GIGE_SetRetryGvcpTimes(handle, nRetryGvcpTimes)`
- `MV_GIGE_GetRetryGvcpTimes(handle, pnRetryGvcpTimes)`

- `MV_CC_GetOptimalPacketSize(handle)`
  - 获取最佳 packet size

- `MV_GIGE_SetResend(handle, bEnable, nMaxResendPercent, nResendTimeout)`
  - 设置丢包重发

- `MV_GIGE_SetResendMaxRetryTimes(handle, nRetryTimes)`
- `MV_GIGE_GetResendMaxRetryTimes(handle, pnRetryTimes)`
- `MV_GIGE_SetResendTimeInterval(handle, nMillisec)`
- `MV_GIGE_GetResendTimeInterval(handle, pnMillisec)`

- `MV_GIGE_SetTransmissionType(handle, pstTransmissionType)`
  - 设置单播/组播等传输类型

- `MV_GIGE_IssueActionCommand(pstActionCmdInfo, pstActionCmdResults)`
  - 发送 Action Command

- `MV_GIGE_GetMulticastStatus(pstDevInfo, pbStatus)`
  - 获取组播状态

## 专用接口

- `MV_CAML_GetSerialPortList(pstSerialPortList)`
  - 获取串口列表

- `MV_CAML_SetEnumSerialPorts(pstSerialPortList)`
  - 设置枚举串口列表

- `MV_CAML_SetDeviceBaudrate(handle, nBaudrate)`
  - 设置设备波特率

- `MV_CAML_GetDeviceBaudrate(handle, pnCurrentBaudrate)`
  - 获取设备波特率

- `MV_CAML_GetSupportBaudrates(handle, pnBaudrateAblity)`
  - 获取支持的波特率

- `MV_CAML_SetGenCPTimeOut(handle, nMillisec)`
  - 设置串口操作等待时长

## USB3 Vision 专用接口

- `MV_USB_SetTransferSize(handle, nTransferSize)`
  - 设置传输包大小

- `MV_USB_GetTransferSize(handle, pnTransferSize)`
  - 获取传输包大小

- `MV_USB_SetTransferWays(handle, nTransferWays)`
  - 设置传输通道个数

- `MV_USB_GetTransferWays(handle, pnTransferWays)`
  - 获取传输通道个数

- `MV_USB_SetEventNodeNum(handle, nEventNodeNum)`
  - 设置事件缓存节点数

- `MV_USB_SetSyncTimeOut(handle, nMills)`
  - 设置同步读写超时

- `MV_USB_GetSyncTimeOut(handle, pnMills)`
  - 获取同步读写超时

## GenTL

- `MV_CC_EnumInterfacesByGenTL(pstIFList, strGenTLPath)`
  - 通过 GenTL CTI 文件枚举接口

- `MV_CC_UnloadGenTLLibrary(pGenTLPath)`
  - 卸载 CTI 库

- `MV_CC_EnumDevicesByGenTL(pstIFInfo, pstDevList)`
  - 通过 GenTL interface 枚举设备

- `MV_CC_CreateHandleByGenTL(handle, pstDevInfo)`
  - 根据 GenTL 设备信息创建设备句柄

## 图像处理

- `MV_CC_SaveImageEx3(handle, pstSaveParam)`
  - 保存图片到内存，支持 BMP/JPEG 等

- `MV_CC_SaveImageToFileEx(handle, pstSaveFileParam)`
  - 保存图片到文件，支持 BMP/JPEG/PNG/TIFF

- `MV_CC_SaveImageToFileEx2(handle, pstImage, pSaveImageParam, pcImagePath)`
  - 按 `MV_CC_IMAGE` 保存图片，支持超大图 PNG/TIFF

- `MV_CC_RotateImage(handle, pstRotateParam)`
  - 旋转图像

- `MV_CC_FlipImage(handle, pstFlipParam)`
  - 翻转图像

- `MV_CC_ConvertPixelTypeEx(handle, pstCvtParam)`
  - 像素格式转换

- `MV_CC_SetBayerCvtQuality(handle, nBayerCvtQuality)`
  - 设置 Bayer 插值质量

- `MV_CC_SetBayerFilterEnable(handle, bFilterEnable)`
  - 设置 Bayer 插值平滑

- `MV_CC_SetBayerGammaValue(handle, fBayerGammaValue)`
  - 设置 Bayer Gamma

- `MV_CC_SetGammaValue(handle, enSrcPixelType, fGammaValue)`
  - 设置 Mono8/Bayer Gamma

- `MV_CC_SetBayerGammaParam(handle, pstGammaParam)`
  - 设置 Bayer Gamma 参数

- `MV_CC_SetBayerCCMParam(handle, pstCCMParam)`
- `MV_CC_SetBayerCCMParamEx(handle, pstCCMParam)`
  - 设置颜色校正矩阵

- `MV_CC_ImageContrast(handle, pstContrastParam)`
  - 图像对比度

- `MV_CC_PurpleFringing(handle, pstPurpleFringingParam)`
  - 去紫边

- `MV_CC_SetISPConfig(handle, pstParam)`
  - 设置 ISP 配置，Windows 平台

- `MV_CC_ISPProcess(handle, pstInputImage, pstOutputImage)`
  - ISP 算法处理，Windows 平台

- `MV_CC_HB_Decode(handle, pstDecodeParam)`
  - HB 无损压缩解码

- `MV_CC_StartRecord(handle, pstRecordParam)`
  - 开始录像

- `MV_CC_InputOneFrame(handle, pstInputFrameInfo)`
  - 输入一帧录像数据，不支持 HB/JPEG 直接输入

- `MV_CC_InputOneFrameEx(handle, pstInputFrameInfo)`
  - 输入一帧录像数据，支持 HB/JPEG 内部解码

- `MV_CC_StopRecord(handle)`
  - 停止录像

- `MV_CC_ReconstructImage(handle, pstReconstructParam)`
  - 图像重构，用于线阵相机分时曝光/分时频闪等场景

## 串口通信设备

- `MV_CC_SerialPort_Open(handle)`
  - 打开串口

- `MV_CC_SerialPort_Write(handle, pBuffer, nLength, pnWriteLen)`
  - 写串口数据，一次最大 512 字节

- `MV_CC_SerialPort_Read(handle, pBuffer, nLength, pnReadLen, nMsec)`
  - 读串口数据

- `MV_CC_SerialPort_ClearBuffer(handle)`
  - 清空串口接收缓存

- `MV_CC_SerialPort_Close(handle)`
  - 关闭串口

## 液态镜头

- `MV_CC_LiquidLens_Open(handle, pstLiquidLensInfo)`
  - 打开液态镜头

- `MV_CC_LiquidLens_Close(handle)`
  - 关闭液态镜头

- `MV_CC_LiquidLens_SetFocalPower(handle, nFocalPower, nTriggerEnable)`
  - 设置光焦度

- `MV_CC_LiquidLens_GetFocalPower(handle, pnFocalPower)`
  - 获取光焦度

- `MV_CC_LiquidLens_AutoFocus(handle, pstAutoFocusParam, nOutputImage, pnResultFocalPower)`
  - 自动对焦

- `MV_CC_LiquidLens_FocalPowerRangeScan(handle, pstRangeScanParam, cbLiquidLensMsg, pUser)`
  - 光焦度区间扫描

- `MV_CC_LiquidLens_MultiFocalPowerScan(handle, pstMultiFPScanParam, cbLiquidLensMsg, pUser)`
  - 多焦度扫描

- `MV_CC_LiquidLens_SetTriggerDelayTime(handle, nDelayTimeMs)`
- `MV_CC_LiquidLens_GetTriggerDelayTime(handle, pnDelayTimeMs)`
  - 设置/获取触发延时

- `MV_CC_LiquidLens_SaveUserSet(handle, enUserSet)`
- `MV_CC_LiquidLens_GetCurrentUserSet(handle, penUserSet)`
- `MV_CC_LiquidLens_LoadUserSet(handle, enUserSet)`
- `MV_CC_LiquidLens_SetDefaultUserSet(handle, enUserSet)`
- `MV_CC_LiquidLens_GetDefaultUserSet(handle, penUserSet)`
  - 用户集保存、加载和默认用户集配置

- `MV_CC_LiquidLens_SetTempDetectInterval(handle, nIntervalSeconds)`
- `MV_CC_LiquidLens_GetTempDetectInterval(handle, pnIntervalSeconds)`
  - 温度检测间隔

- `MV_CC_LiquidLens_RegisterExceptionCallBack(handle, cbLiquidLensException, pUser)`
  - 注册液态镜头异常回调
