头文件：`conda-packages/hikcamera-mvs/sources/5.0.1-20260512/include/PixelType.h`

这个文件定义 `enum MvGvspPixelType`

它描述相机输出、图像保存、像素格式转换、ISP、录像等接口使用的像素格式

## 编码规则

像素格式值由几类信息组合而成：

- 像素类别：Mono，Color，Custom
- bit count：例如 8、10、12、16、24、32、48、64、96 bit
- Pixel ID：低位的具体格式编号

## Undefined

- `PixelType_Gvsp_Undefined`：未定义像素格式

## Mono

单通道灰度格式：

- `PixelType_Gvsp_Mono1p`
- `PixelType_Gvsp_Mono2p`
- `PixelType_Gvsp_Mono4p`
- `PixelType_Gvsp_Mono8`
- `PixelType_Gvsp_Mono8_Signed`
- `PixelType_Gvsp_Mono10`
- `PixelType_Gvsp_Mono10_Packed`
- `PixelType_Gvsp_Mono12`
- `PixelType_Gvsp_Mono12_Packed`
- `PixelType_Gvsp_Mono14`
- `PixelType_Gvsp_Mono16`

常用的是 `Mono8`、`Mono10`、`Mono12`、`Mono16`

## Bayer

Bayer 原始彩色阵列：

- `PixelType_Gvsp_BayerGR8`
- `PixelType_Gvsp_BayerRG8`
- `PixelType_Gvsp_BayerGB8`
- `PixelType_Gvsp_BayerBG8`
- `PixelType_Gvsp_BayerRBGG8`
- `PixelType_Gvsp_BayerBRGG8`
- `PixelType_Gvsp_BayerGR10`
- `PixelType_Gvsp_BayerRG10`
- `PixelType_Gvsp_BayerGB10`
- `PixelType_Gvsp_BayerBG10`
- `PixelType_Gvsp_BayerGR12`
- `PixelType_Gvsp_BayerRG12`
- `PixelType_Gvsp_BayerGB12`
- `PixelType_Gvsp_BayerBG12`
- `PixelType_Gvsp_BayerGR10_Packed`
- `PixelType_Gvsp_BayerRG10_Packed`
- `PixelType_Gvsp_BayerGB10_Packed`
- `PixelType_Gvsp_BayerBG10_Packed`
- `PixelType_Gvsp_BayerGR12_Packed`
- `PixelType_Gvsp_BayerRG12_Packed`
- `PixelType_Gvsp_BayerGB12_Packed`
- `PixelType_Gvsp_BayerBG12_Packed`
- `PixelType_Gvsp_BayerGR16`
- `PixelType_Gvsp_BayerRG16`
- `PixelType_Gvsp_BayerGB16`
- `PixelType_Gvsp_BayerBG16`

Bayer 格式通常需要通过 `MV_CC_ConvertPixelTypeEx` 转成 RGB/BGR

## RGB / BGR / RGBA / BGRA

打包彩色格式：

- `PixelType_Gvsp_RGB8_Packed`
- `PixelType_Gvsp_BGR8_Packed`
- `PixelType_Gvsp_RGBA8_Packed`
- `PixelType_Gvsp_BGRA8_Packed`
- `PixelType_Gvsp_RGB10_Packed`
- `PixelType_Gvsp_BGR10_Packed`
- `PixelType_Gvsp_RGB12_Packed`
- `PixelType_Gvsp_BGR12_Packed`
- `PixelType_Gvsp_RGB16_Packed`
- `PixelType_Gvsp_BGR16_Packed`
- `PixelType_Gvsp_RGBA16_Packed`
- `PixelType_Gvsp_BGRA16_Packed`
- `PixelType_Gvsp_RGB10V1_Packed`
- `PixelType_Gvsp_RGB10V2_Packed`
- `PixelType_Gvsp_RGB12V1_Packed`
- `PixelType_Gvsp_RGB565_Packed`
- `PixelType_Gvsp_BGR565_Packed`

## YUV / YCbCr

- `PixelType_Gvsp_YUV411_Packed`
- `PixelType_Gvsp_YUV422_Packed`
- `PixelType_Gvsp_YUV422_YUYV_Packed`
- `PixelType_Gvsp_YUV444_Packed`
- `PixelType_Gvsp_YCBCR8_CBYCR`
- `PixelType_Gvsp_YCBCR422_8`
- `PixelType_Gvsp_YCBCR422_8_CBYCRY`
- `PixelType_Gvsp_YCBCR411_8_CBYYCRYY`
- `PixelType_Gvsp_YCBCR601_8_CBYCR`
- `PixelType_Gvsp_YCBCR601_422_8`
- `PixelType_Gvsp_YCBCR601_422_8_CBYCRY`
- `PixelType_Gvsp_YCBCR601_411_8_CBYYCRYY`
- `PixelType_Gvsp_YCBCR709_8_CBYCR`
- `PixelType_Gvsp_YCBCR709_422_8`
- `PixelType_Gvsp_YCBCR709_422_8_CBYCRY`
- `PixelType_Gvsp_YCBCR709_411_8_CBYYCRYY`
- `PixelType_Gvsp_YUV420SP_NV12`
- `PixelType_Gvsp_YUV420SP_NV21`

## Planar

- `PixelType_Gvsp_RGB8_Planar`
- `PixelType_Gvsp_RGB10_Planar`
- `PixelType_Gvsp_RGB12_Planar`
- `PixelType_Gvsp_RGB16_Planar`

Planar 表示不同颜色通道分平面存储

## 压缩和 3D

- `PixelType_Gvsp_Jpeg`：JPEG 压缩图像
- `PixelType_Gvsp_Coord3D_ABC32f`
- `PixelType_Gvsp_Coord3D_ABC32f_Planar`
- `PixelType_Gvsp_Coord3D_AB32f`
- `PixelType_Gvsp_Coord3D_AB32`
- `PixelType_Gvsp_Coord3D_AC32f_64`
- `PixelType_Gvsp_Coord3D_AC32f_Planar`
- `PixelType_Gvsp_Coord3D_AC32`
- `PixelType_Gvsp_Coord3D_A32f`
- `PixelType_Gvsp_Coord3D_A32`
- `PixelType_Gvsp_Coord3D_C32f`
- `PixelType_Gvsp_Coord3D_C32`
- `PixelType_Gvsp_Coord3D_ABC16`
- `PixelType_Gvsp_Coord3D_C16`

3D 格式用于深度、点云、坐标类设备

## Float

- `PixelType_Gvsp_Float32`：32 位浮点格式

## HB 无损压缩

HB 格式是海康的无损压缩相关像素格式，通常配合 `MV_CC_HB_Decode`

- `PixelType_Gvsp_HB_Mono8`
- `PixelType_Gvsp_HB_Mono10`
- `PixelType_Gvsp_HB_Mono10_Packed`
- `PixelType_Gvsp_HB_Mono12`
- `PixelType_Gvsp_HB_Mono12_Packed`
- `PixelType_Gvsp_HB_Mono16`
- `PixelType_Gvsp_HB_BayerGR8`
- `PixelType_Gvsp_HB_BayerRG8`
- `PixelType_Gvsp_HB_BayerGB8`
- `PixelType_Gvsp_HB_BayerBG8`
- `PixelType_Gvsp_HB_BayerGR10`
- `PixelType_Gvsp_HB_BayerRG10`
- `PixelType_Gvsp_HB_BayerGB10`
- `PixelType_Gvsp_HB_BayerBG10`
- `PixelType_Gvsp_HB_BayerGR12`
- `PixelType_Gvsp_HB_BayerRG12`
- `PixelType_Gvsp_HB_BayerGB12`
- `PixelType_Gvsp_HB_BayerBG12`
- `PixelType_Gvsp_HB_YUV422_Packed`
- `PixelType_Gvsp_HB_YUV422_YUYV_Packed`
- `PixelType_Gvsp_HB_RGB8_Packed`
- `PixelType_Gvsp_HB_BGR8_Packed`
- `PixelType_Gvsp_HB_RGBA8_Packed`
- `PixelType_Gvsp_HB_BGRA8_Packed`
- `PixelType_Gvsp_HB_RGB16_Packed`
- `PixelType_Gvsp_HB_BGR16_Packed`
- `PixelType_Gvsp_HB_RGBA16_Packed`
- `PixelType_Gvsp_HB_BGRA16_Packed`
