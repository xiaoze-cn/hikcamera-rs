#!/usr/bin/env bash
# Generate placeholder mdx files for every slug referenced by the sidebar.
# Real content gets filled in later steps.

set -eu

declare -A SLUGS=(
  # Guide (en)
  ["en/guide/quick-start"]="Quick start|Capture a frame in under five minutes with the default device."
  ["en/guide/sdk-lifecycle"]="SDK lifecycle|How HikCamera initializes and finalizes the underlying C SDK."
  ["en/guide/device-selection"]="Device selection|Enumerate visible cameras and pick one by serial, name, IP, or MAC."
  ["en/guide/camera-configuration"]="Camera configuration|Set exposure, gain, pixel format, trigger, ROI, and any GenICam node."
  ["en/guide/streaming"]="Streaming|Enter grabbing, pull frames with a timeout, and stop cleanly."
  ["en/guide/image-and-video"]="Image and video writers|Save frames as BMP/JPEG/PNG/TIFF and record clips to AVI."
  # Developer (en)
  ["en/developer/architecture"]="Architecture|Two-crate split, unsafe boundary, and layered lifecycle types."
  ["en/developer/error-model"]="Error model|How SDK status codes become a typed HikCameraError."
  ["en/developer/contributing"]="Contributing|Day-to-day just commands, formatting, language, and lefthook hooks."
  # Reference (en)
  ["en/reference/device-info-fields"]="Device info fields|Field reference for DeviceInfo across GigE, USB, and GenTL."
  ["en/reference/c-sdk/functions"]="C SDK functions|Functions exposed by MvCameraControl.h."
  ["en/reference/c-sdk/structs"]="C SDK structs|Structs and enums declared in CameraParams.h."
  ["en/reference/c-sdk/pixel-types"]="Pixel types|Pixel format constants from PixelType.h."
  ["en/reference/c-sdk/error-codes"]="C SDK error codes|MV_E_* status constants from MvErrorDefine.h."
  ["en/reference/c-sdk/isp-error-codes"]="ISP error codes|MV_ALG_* status constants from MvISPErrorDefine.h."
  ["en/reference/c-sdk/obsolete-interfaces"]="Obsolete C interfaces|Deprecated entry points kept for bindgen compatibility."
  ["en/reference/c-sdk/obsolete-params"]="Obsolete C params|Deprecated camera parameters kept for bindgen compatibility."
  # Guide (zh)
  ["zh/guide/installation"]="安装|使用 pixi + just 构建 workspace。"
  ["zh/guide/quick-start"]="快速开始|用默认设备在五分钟内拍一张照片。"
  ["zh/guide/sdk-lifecycle"]="SDK 生命周期|HikCamera 如何初始化和反初始化底层 C SDK。"
  ["zh/guide/device-selection"]="设备选择|枚举可见相机并按序列号、用户名、IP 或 MAC 选中。"
  ["zh/guide/camera-configuration"]="相机配置|设置曝光、增益、像素格式、触发模式、ROI 以及任意 GenICam 节点。"
  ["zh/guide/streaming"]="采集|进入 grabbing 模式、按超时取帧、干净停止。"
  ["zh/guide/image-and-video"]="图像与视频写入|把帧存成 BMP/JPEG/PNG/TIFF 或录制为 AVI。"
  # Developer (zh)
  ["zh/developer/architecture"]="架构|两个 crate 的划分、unsafe 边界、分层生命周期类型。"
  ["zh/developer/error-model"]="错误模型|SDK 状态码如何变成有类型的 HikCameraError。"
  ["zh/developer/contributing"]="贡献|日常 just 命令、格式化、语言策略和 lefthook 钩子。"
  # Reference (zh)
  ["zh/reference/device-info-fields"]="设备信息字段|DeviceInfo 字段参考（GigE/USB/GenTL）。"
  ["zh/reference/c-sdk/functions"]="C SDK 函数|MvCameraControl.h 暴露的函数。"
  ["zh/reference/c-sdk/structs"]="C SDK 结构体|CameraParams.h 中声明的结构体和枚举。"
  ["zh/reference/c-sdk/pixel-types"]="像素格式|PixelType.h 中的像素格式常量。"
  ["zh/reference/c-sdk/error-codes"]="C SDK 错误码|MvErrorDefine.h 中的 MV_E_* 状态常量。"
  ["zh/reference/c-sdk/isp-error-codes"]="ISP 错误码|MvISPErrorDefine.h 中的 MV_ALG_* 状态常量。"
  ["zh/reference/c-sdk/obsolete-interfaces"]="过时 C 接口|为 bindgen 兼容性保留的弃用入口。"
  ["zh/reference/c-sdk/obsolete-params"]="过时 C 参数|为 bindgen 兼容性保留的弃用相机参数。"
)

ROOT="$(dirname "$0")/../src/content/docs"

for slug in "${!SLUGS[@]}"; do
  entry="${SLUGS[$slug]}"
  title="${entry%%|*}"
  description="${entry#*|}"
  file="$ROOT/$slug.mdx"
  mkdir -p "$(dirname "$file")"
  if [ ! -f "$file" ]; then
    cat > "$file" <<EOF
---
title: $title
description: $description
---

Placeholder — content arrives in a later step.
EOF
    echo "created: $file"
  else
    echo "exists:  $file"
  fi
done
