# ObsoleteCamParams

头文件：`crates/hikcamera-sys/includes/ObsoleteCamParams.h`

这个文件为 `MvObsoleteInterfaces.h` 里的旧接口提供配套结构体、枚举和宏

新代码优先使用 `CameraParams.h` 中的新结构体

## 旧帧信息和保存参数

- `MV_FRAME_OUT_INFO`
  - 旧版帧信息
  - 新接口优先用 `MV_FRAME_OUT_INFO_EX`

- `MV_SAVE_IMAGE_PARAM`
  - 旧版保存图像参数

- `MV_SAVE_IMAGE_PARAM_EX`
  - 旧版扩展保存图像参数

- `MV_SAVE_IMG_TO_FILE_PARAM`
  - 旧版保存图像到文件参数

- `MV_CC_PIXEL_CONVERT_PARAM`
  - 旧版像素格式转换参数
  - 新接口优先用 `MV_CC_PIXEL_CONVERT_PARAM_EX`

- `MV_DISPLAY_FRAME_INFO`
  - 旧版显示帧参数
  - 新接口优先用 `MV_DISPLAY_FRAME_INFO_EX`

- `MV_IMAGE_BASIC_INFO`
  - 旧版图像基础信息

## 旧图像算法参数

- `MV_CC_BAYER_NOISE_FEATURE_TYPE`
  - Bayer 噪声特性类型

- `MV_CC_BAYER_NOISE_PROFILE_INFO`
  - Bayer 噪声 profile 信息

- `MV_CC_BAYER_NOISE_ESTIMATE_PARAM`
  - Bayer 噪声估计参数

- `MV_CC_BAYER_SPATIAL_DENOISE_PARAM`
  - Bayer 空域降噪参数

- `MV_CC_CLUT_PARAM`
  - CLUT 参数

- `MV_CC_SHARPEN_PARAM`
  - 锐化参数

- `MV_CC_COLOR_CORRECT_PARAM`
  - 颜色校正参数

- `MV_CC_RECT_I`
  - 整数矩形

- `MV_CC_NOISE_ESTIMATE_PARAM`
  - 噪声估计参数

- `MV_CC_SPATIAL_DENOISE_PARAM`
  - 空域降噪参数

- `MV_CC_LSC_CALIB_PARAM`
  - LSC 标定参数

- `MV_CC_LSC_CORRECT_PARAM`
  - LSC 校正参数

## 旧 XML 节点结构

旧 XML 遍历接口使用这些结构

- `MV_XML_Visibility`
  - XML 节点可见性

- `MV_XML_NODE_FEATURE`
  - XML 节点基础信息

- `MV_XML_NODES_LIST`
  - XML 节点列表

- `MV_XML_FEATURE_Value`
  - XML value 节点

- `MV_XML_FEATURE_Base`
  - XML base 节点

- `MV_XML_FEATURE_Integer`
  - XML integer 节点

- `MV_XML_FEATURE_Boolean`
  - XML boolean 节点

- `MV_XML_FEATURE_Command`
  - XML command 节点

- `MV_XML_FEATURE_Float`
  - XML float 节点

- `MV_XML_FEATURE_String`
  - XML string 节点

- `MV_XML_FEATURE_Register`
  - XML register 节点

- `MV_XML_FEATURE_Category`
  - XML category 节点

- `MV_XML_FEATURE_EnumEntry`
  - XML enum entry 节点

- `MV_XML_FEATURE_Enumeration`
  - XML enumeration 节点

- `MV_XML_FEATURE_Port`
  - XML port 节点

- `MV_XML_CAMERA_FEATURE`
  - XML 相机节点联合结构

## 旧点云保存参数

- `MV_SAVE_POINT_CLOUD_FILE_TYPE`
  - 点云输出文件类型

- `MV_SAVE_POINT_CLOUD_PARAM`
  - 点云保存参数

## 旧宏

- `MV_MAX_XML_NODE_NUM_C`
  - XML 节点最大数量

- `MV_MAX_XML_NODE_STRLEN_C`
  - XML 节点名长度

- `MV_MAX_XML_STRVALUE_STRLEN_C`
  - XML 字符串值长度

- `MV_MAX_XML_DISC_STRLEN_C`
  - XML 描述长度

- `MV_MAX_XML_ENTRY_NUM`
  - XML enum entry 最大数量

- `MV_MAX_XML_PARENTS_NUM`
  - XML 父节点最大数量

- `MV_MAX_XML_SYMBOLIC_STRLEN_C`
  - XML symbolic 字符串长度
