# MvObsoleteInterfaces

头文件：`crates/hikcamera-sys/includes/MvObsoleteInterfaces.h`

这个文件放旧接口

这些接口仍然在 SDK 头文件里，也会被 bindgen 看见，但新封装优先走 `MvCameraControl.h` 里的新版函数

## 旧图像和 XML 接口

- `MV_CC_GetImageInfo(handle, pstInfo)`
  - 获取图像基础信息

- `MV_CC_GetTlProxy(handle)`
  - 获取 transport layer proxy

- `MV_XML_GetRootNode(handle, pstNode)`
  - 获取 XML 根节点

- `MV_XML_GetChildren(handle, pstNode, pstNodesList)`
  - 获取 XML 子节点

- `MV_XML_GetNodeFeature(handle, pstNode, pstFeature)`
  - 获取 XML 节点特征

- `MV_XML_UpdateNodeFeature(handle, enType, pstFeature)`
  - 更新 XML 节点特征

- `MV_XML_RegisterUpdateCallBack(handle, cbUpdate, pUser)`
  - 注册 XML 更新回调

## 旧取流接口

- `MV_CC_GetOneFrame(handle, pData, nDataSize, pFrameInfo)`
  - 旧版主动取图
  - 新接口优先用 `MV_CC_GetOneFrameTimeout` 或 `MV_CC_GetImageBuffer`

- `MV_CC_GetOneFrameEx(handle, pData, nDataSize, pFrameInfo)`
  - 旧版扩展主动取图
  - 新接口优先用 `MV_CC_GetOneFrameTimeout` 或 `MV_CC_GetImageBuffer`

- `MV_CC_RegisterImageCallBack(handle, cbOutput, pUser)`
  - 旧版图像回调
  - 新接口优先用 `MV_CC_RegisterImageCallBackEx` 或 `MV_CC_RegisterImageCallBackEx2`

## 旧保存、转换和显示接口

- `MV_CC_SaveImage(pSaveParam)`
  - 旧版保存图像

- `MV_CC_SaveImageEx(pSaveParam)`
  - 旧版扩展保存图像

- `MV_CC_SaveImageEx2(handle, pstSaveParam)`
  - 旧版保存图像
  - 新接口优先用 `MV_CC_SaveImageEx3`

- `MV_CC_SaveImageToFile(handle, pstSaveFileParam)`
  - 旧版保存图像到文件
  - 新接口优先用 `MV_CC_SaveImageToFileEx` 或 `MV_CC_SaveImageToFileEx2`

- `MV_CC_ConvertPixelType(handle, pstCvtParam)`
  - 旧版像素格式转换
  - 新接口优先用 `MV_CC_ConvertPixelTypeEx`

- `MV_CC_Display(handle, hWnd)`
  - 旧版显示接口

- `MV_CC_DisplayOneFrame(handle, pstDisplayInfo)`
  - 旧版显示一帧
  - 新接口优先用 `MV_CC_DisplayOneFrameEx` 或 `MV_CC_DisplayOneFrameEx2`

- `MV_CC_OpenParamsGUI(handle)`
  - 打开参数 GUI

- `MV_CC_SavePointCloudData(handle, pstPointDataParam)`
  - 保存点云数据

## 旧图像算法接口

- `MV_CC_BayerNoiseEstimate(handle, pstNoiseEstimateParam)`
  - Bayer 噪声估计

- `MV_CC_BayerSpatialDenoise(handle, pstSpatialDenoiseParam)`
  - Bayer 空域降噪

- `MV_CC_SetBayerCLUTParam(handle, pstCLUTParam)`
  - 设置 Bayer CLUT 参数

- `MV_CC_ImageSharpen(handle, pstSharpenParam)`
  - 图像锐化

- `MV_CC_ColorCorrect(handle, pstColorCorrectParam)`
  - 颜色校正

- `MV_CC_NoiseEstimate(handle, pstNoiseEstimateParam)`
  - 噪声估计

- `MV_CC_SpatialDenoise(handle, pstSpatialDenoiseParam)`
  - 空域降噪

- `MV_CC_LSCCalib(handle, pstLSCCalibParam)`
  - LSC 标定

- `MV_CC_LSCCorrect(handle, pstLSCCorrectParam)`
  - LSC 校正

## 旧具体参数 getter/setter

这些接口把常见 GenICam 节点包装成单独函数

现在更推荐用万能属性接口：

```text
MV_CC_GetIntValueEx
MV_CC_SetIntValueEx
MV_CC_GetFloatValue
MV_CC_SetFloatValue
MV_CC_GetEnumValueEx
MV_CC_SetEnumValue
MV_CC_SetEnumValueByString
MV_CC_GetStringValue
MV_CC_SetStringValue
MV_CC_SetCommandValue
```

旧接口包括：

- `MV_CC_GetIntValue` / `MV_CC_SetIntValue`
- `MV_CC_GetWidth` / `MV_CC_SetWidth`
- `MV_CC_GetHeight` / `MV_CC_SetHeight`
- `MV_CC_GetAOIoffsetX` / `MV_CC_SetAOIoffsetX`
- `MV_CC_GetAOIoffsetY` / `MV_CC_SetAOIoffsetY`
- `MV_CC_GetAutoExposureTimeLower` / `MV_CC_SetAutoExposureTimeLower`
- `MV_CC_GetAutoExposureTimeUpper` / `MV_CC_SetAutoExposureTimeUpper`
- `MV_CC_GetBrightness` / `MV_CC_SetBrightness`
- `MV_CC_GetFrameRate` / `MV_CC_SetFrameRate`
- `MV_CC_GetGain` / `MV_CC_SetGain`
- `MV_CC_GetExposureTime` / `MV_CC_SetExposureTime`
- `MV_CC_GetPixelFormat` / `MV_CC_SetPixelFormat`
- `MV_CC_GetAcquisitionMode` / `MV_CC_SetAcquisitionMode`
- `MV_CC_GetGainMode` / `MV_CC_SetGainMode`
- `MV_CC_GetExposureAutoMode` / `MV_CC_SetExposureAutoMode`
- `MV_CC_GetTriggerMode` / `MV_CC_SetTriggerMode`
- `MV_CC_GetTriggerDelay` / `MV_CC_SetTriggerDelay`
- `MV_CC_GetTriggerSource` / `MV_CC_SetTriggerSource`
- `MV_CC_TriggerSoftwareExecute`
- `MV_CC_GetGammaSelector` / `MV_CC_SetGammaSelector`
- `MV_CC_GetGamma` / `MV_CC_SetGamma`
- `MV_CC_GetSharpness` / `MV_CC_SetSharpness`
- `MV_CC_GetHue` / `MV_CC_SetHue`
- `MV_CC_GetSaturation` / `MV_CC_SetSaturation`
- `MV_CC_GetBalanceWhiteAuto` / `MV_CC_SetBalanceWhiteAuto`
- `MV_CC_GetBalanceRatioRed` / `MV_CC_SetBalanceRatioRed`
- `MV_CC_GetBalanceRatioGreen` / `MV_CC_SetBalanceRatioGreen`
- `MV_CC_GetBalanceRatioBlue` / `MV_CC_SetBalanceRatioBlue`
- `MV_CC_GetFrameSpecInfoAbility` / `MV_CC_SetFrameSpecInfoAbility`
- `MV_CC_GetDeviceUserID` / `MV_CC_SetDeviceUserID`
- `MV_CC_GetBurstFrameCount` / `MV_CC_SetBurstFrameCount`
- `MV_CC_GetAcquisitionLineRate` / `MV_CC_SetAcquisitionLineRate`
- `MV_CC_GetHeartBeatTimeout` / `MV_CC_SetHeartBeatTimeout`

## 旧传输层接口

GigE：

- `MV_GIGE_ForceIp(handle, nIP)`
  - 新接口优先用 `MV_GIGE_ForceIpEx`

- `MV_GIGE_GetGevSCPSPacketSize` / `MV_GIGE_SetGevSCPSPacketSize`
- `MV_GIGE_GetGevSCPD` / `MV_GIGE_SetGevSCPD`
- `MV_GIGE_GetGevSCDA` / `MV_GIGE_SetGevSCDA`
- `MV_GIGE_GetGevSCSP` / `MV_GIGE_SetGevSCSP`

CameraLink：

- `MV_CAML_SetDeviceBauderate`
  - 新接口拼写是 `MV_CAML_SetDeviceBaudrate`

- `MV_CAML_GetDeviceBauderate`
  - 新接口拼写是 `MV_CAML_GetDeviceBaudrate`

- `MV_CAML_GetSupportBauderates`
  - 新接口拼写是 `MV_CAML_GetSupportBaudrates`

USB：

- `MV_USB_RegisterStreamExceptionCallBack`
  - 新接口优先用统一的 `MV_CC_RegisterStreamExceptionCallBack`

## 旧辅助接口

- `MV_CC_SetSDKLogPath(strSDKLogPath)`
  - 设置 SDK 日志路径

- `MV_CC_EnumerateTls()`
  - 枚举传输层

- `MV_CC_CreateHandleWithoutLog(handle, pstDevInfo)`
  - 创建不带日志的 handle

- `MV_CC_RegisterImageCallBackForRGB(handle, cbOutput, pUser)`
  - 注册 RGB 图像回调

- `MV_CC_RegisterImageCallBackForBGR(handle, cbOutput, pUser)`
  - 注册 BGR 图像回调

- `MV_CC_GetImageForRGB(handle, pData, nDataSize, pstFrameInfo, nMsec)`
  - 主动获取 RGB 图像

- `MV_CC_GetImageForBGR(handle, pData, nDataSize, pstFrameInfo, nMsec)`
  - 主动获取 BGR 图像
