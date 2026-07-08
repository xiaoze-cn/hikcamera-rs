头文件：`conda-packages/hikcamera-mvs/sources/5.0.1-20260512/include/MvISPErrorDefine.h`

这个文件定义 ISP/算法处理相关错误码

它和 `MvErrorDefine.h` 不同，主要面向算法库、ISP、图像处理内部错误

## 成功和通用错误

- `MV_ALG_OK`：处理正确
- `MV_ALG_ERR`：不确定类型错误
- `MV_ALG_WARNING`：警告

## 能力集和内存错误

- `MV_ALG_E_ABILITY_ARG`：能力集中存在无效参数
- `MV_ALG_E_MEM_NULL`：内存地址为空
- `MV_ALG_E_MEM_ALIGN`：内存对齐不满足要求
- `MV_ALG_E_MEM_LACK`：内存空间大小不够
- `MV_ALG_E_MEM_SIZE_ALIGN`：内存空间大小不满足对齐要求
- `MV_ALG_E_MEM_ADDR_ALIGN`：内存地址不满足对齐要求
- `MV_ALG_E_OVER_MAX_MEM`：超过限定的最大内存
- `MV_ALG_E_MALLOC_MEM`：分配内存错误

## 图像错误

- `MV_ALG_E_IMG_FORMAT`：图像格式不正确或不支持
- `MV_ALG_E_IMG_SIZE`：图像宽高不正确或超出范围
- `MV_ALG_E_IMG_STEP`：图像宽高与 step 参数不匹配
- `MV_ALG_E_IMG_DATA_NULL`：图像数据地址为空
- `MV_ALG_E_IMAGE_CODEC`：图像解码器错误

## 参数错误

- `MV_ALG_E_CFG_TYPE`：设置或获取参数类型不正确
- `MV_ALG_E_CFG_SIZE`：设置或获取参数结构体大小不正确
- `MV_ALG_E_PRC_TYPE`：处理类型不正确
- `MV_ALG_E_PRC_SIZE`：处理输入/输出参数大小不正确
- `MV_ALG_E_FUNC_TYPE`：子处理类型不正确
- `MV_ALG_E_FUNC_SIZE`：子处理输入/输出参数大小不正确
- `MV_ALG_E_PARAM_INDEX`：index 参数不正确
- `MV_ALG_E_PARAM_VALUE`：value 参数不正确或超出范围
- `MV_ALG_E_PARAM_NUM`：param_num 参数不正确
- `MV_ALG_E_NULL_PTR`：函数参数指针为空
- `MV_ALG_E_BAD_ARG`：参数范围不正确
- `MV_ALG_E_DATA_SIZE`：数据大小不正确
- `MV_ALG_E_STEP`：数据 step 不正确

## 文件和运行环境错误

- `MV_ALG_E_TIME_OUT`：算法库超时
- `MV_ALG_E_LIB_VERSION`：算法版本号错误
- `MV_ALG_E_MODEL_VERSION`：模型版本号错误
- `MV_ALG_E_GPU_MEM_ALLOC`：GPU 内存分配错误
- `MV_ALG_E_FILE_NON_EXIST`：文件不存在
- `MV_ALG_E_NONE_STRING`：字符串为空
- `MV_ALG_E_FILE_OPEN`：打开文件错误
- `MV_ALG_E_FILE_READ`：文件读取错误
- `MV_ALG_E_FILE_WRITE`：文件写错误
- `MV_ALG_E_FILE_READ_SIZE`：文件读取大小错误
- `MV_ALG_E_FILE_TYPE`：文件类型错误
- `MV_ALG_E_MODEL_TYPE`：模型类型错误
- `MV_ALG_E_CPUID`：CPU 不支持优化代码中的指令集
- `MV_ALG_E_BIND_CORE_FAILED`：线程绑核失败

## 授权和回调错误

- `MV_ALG_E_CALL_BACK`：回调函数出错
- `MV_ALG_E_ENCRYPT`：加密错误
- `MV_ALG_E_EXPIRE`：算法库使用期限错误

## 噪声估计/降噪错误

前缀：`MV_ALG_E_DENOISE_*`

这一组主要描述噪声特性、噪声曲线、ROI、增益、噪声 profile 等参数错误：

- 图像格式错误
- 噪声特性类型错误
- 噪声特性个数错误
- 增益个数或增益值错误
- 噪声曲线柱数错误
- 噪声估计未初始化
- ROI 个数、原点或大小错误
- 噪声特性内存大小错误

## 去紫边错误

前缀：`MV_ALG_E_PFC_*`

主要错误：

- 去紫边 ROI 原点错误
- 去紫边 ROI 大小错误
- 去紫边滤波核尺寸错误
