import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'

import { navigation, navigationItems } from './navigation.js'

const md = lines => lines.join('\n')

const readDoc = path =>
  readFileSync(resolve(process.cwd(), '..', 'docs', path), 'utf8').trim()

const docPage = (title, source) => md([`# ${title}`, '', source])

const hikcameraDocs = {
  index: readDoc('hikcamera/index.md'),
  system: readDoc('hikcamera/system.md'),
  device: readDoc('hikcamera/device.md'),
  camera: readDoc('hikcamera/camera.md'),
  error: readDoc('hikcamera/error.md')
}

const hikcameraSysDocs = {
  index: readDoc('hikcamera-sys/index.md')
}

const slugify = value =>
  value
    .trim()
    .toLowerCase()
    .replace(/[^\p{Letter}\p{Number}\s-]/gu, '')
    .replace(/\s+/g, '-')

const tocFrom = markdown =>
  markdown
    .split('\n')
    .map(line => line.match(/^(#{2,3})\s+(.+)$/))
    .filter(Boolean)
    .map(match => ({
      id: slugify(match[2]),
      title: match[2],
      depth: match[1].length
    }))

const body = {
  '': {
    zh: md([
      '# 项目介绍',
      '',
      '`hikcamera-rs` 是海康机器人 MVS SDK 的 Rust 封装，工作区分成两个层次：普通用户主要使用高层 crate `hikcamera`，需要直接碰 C SDK 时再使用底层 crate `hikcamera-sys`。',
      '',
      '## 定位',
      '',
      '`hikcamera` 负责把底层 C SDK 能力整理成更符合 Rust 使用习惯的接口。它隐藏初始化、设备枚举、handle 打开关闭、错误码转换等重复工作，让应用代码围绕 `HikCamera`、`Device`、`Camera` 和 `Stream` 组织。',
      '',
      '## 主要流程',
      '',
      '- 创建 SDK 入口：`HikCamera::new()`',
      '- 枚举设备：`hik.devices()?`',
      '- 选择设备：`default()`、`serial_number(...)`、`user_name(...)`、`ip(...)` 或 `mac(...)`',
      '- 打开相机：`Device::open()`',
      '- 取流和取帧：`Camera::stream()`、`Stream::take_frame(...)`',
      '- 保存图片或视频后，显式 `stop()` 和 `close()`',
      '',
      '## 文档边界',
      '',
      '这份站点面向 Rust 使用者。`hikcamera` 页面讲高层 API 和推荐写法；`hikcamera-sys` 页面只讲 FFI crate 的构建、绑定和边界，不把 C SDK 手册展开进正文。'
    ]),
    en: md([
      '# Introduction',
      '',
      '`hikcamera-rs` is a Rust wrapper around the Hikrobot MVS SDK. The workspace has two layers: most users work with the high-level `hikcamera` crate, while `hikcamera-sys` stays available for direct FFI access.',
      '',
      '## Role',
      '',
      '`hikcamera` turns the raw C SDK surface into Rust-shaped APIs. It keeps initialization, device enumeration, handle ownership, and status-code conversion out of application code.',
      '',
      '## Main Flow',
      '',
      '- Create the SDK entry with `HikCamera::new()`',
      '- Enumerate devices with `hik.devices()?`',
      '- Select by default device, serial number, user name, IP, or MAC address',
      '- Open the selected device with `Device::open()`',
      '- Start grabbing with `Camera::stream()` and read frames with `Stream::take_frame(...)`',
      '- Save images or video, then explicitly `stop()` and `close()`',
      '',
      '## Documentation Boundary',
      '',
      'This site is written for Rust users. The `hikcamera` pages describe the high-level API and common usage patterns. The `hikcamera-sys` page explains the FFI crate boundary without expanding the C SDK reference.'
    ])
  },
  'quick-start': {
    zh: md([
      '# 快速开始',
      '',
      '最小路径是：初始化 SDK、枚举设备、打开相机、进入取流、取一帧并保存。',
      '',
      '## 最小示例',
      '',
      '```rust',
      'use std::path::Path;',
      'use std::time::Duration;',
      '',
      'use hikcamera::HikCamera;',
      '',
      'fn main() -> hikcamera::Result<()> {',
      '    let hik = HikCamera::new()?;',
      '',
      '    let devices = hik.devices()?;',
      '    let device = devices.default()?;',
      '    let camera = device.open()?;',
      '',
      '    let mut stream = camera.stream()?;',
      '    let frame = stream.take_frame(Duration::from_secs(1))?;',
      '',
      '    let mut image = stream.save_image(Path::new("image.bmp"))?;',
      '    image.write_frame(&frame)?;',
      '    image.finish()?;',
      '',
      '    let camera = stream.stop()?;',
      '    camera.close()?;',
      '',
      '    Ok(())',
      '}',
      '```',
      '',
      '## 选择设备',
      '',
      '开发阶段可以先用 `devices.default()?` 打开第一台设备。固定现场设备时，更推荐用 `serial_number(...)`，它比 IP 更稳定，也不依赖用户自定义名称。',
      '',
      '```rust',
      'let camera = hik.devices()?.serial_number("DA1234567")?.open()?;',
      '```',
      '',
      '## 取流以后',
      '',
      '`Camera::stream()` 会消费当前 `Camera` 并返回 `Stream`。如果后续还要继续设置相机参数，需要先调用 `Stream::stop()` 取回 `Camera`。'
    ]),
    en: md([
      '# Quick Start',
      '',
      'The smallest path is: initialize the SDK, enumerate devices, open one camera, start grabbing, read one frame, and save it.',
      '',
      '## Minimal Example',
      '',
      '```rust',
      'use std::path::Path;',
      'use std::time::Duration;',
      '',
      'use hikcamera::HikCamera;',
      '',
      'fn main() -> hikcamera::Result<()> {',
      '    let hik = HikCamera::new()?;',
      '',
      '    let devices = hik.devices()?;',
      '    let device = devices.default()?;',
      '    let camera = device.open()?;',
      '',
      '    let mut stream = camera.stream()?;',
      '    let frame = stream.take_frame(Duration::from_secs(1))?;',
      '',
      '    let mut image = stream.save_image(Path::new("image.bmp"))?;',
      '    image.write_frame(&frame)?;',
      '    image.finish()?;',
      '',
      '    let camera = stream.stop()?;',
      '    camera.close()?;',
      '',
      '    Ok(())',
      '}',
      '```',
      '',
      '## Device Selection',
      '',
      'During early development, `devices.default()?` is often enough. For a fixed deployment, prefer `serial_number(...)`; it is more stable than IP and does not depend on a configured user name.',
      '',
      '```rust',
      'let camera = hik.devices()?.serial_number("DA1234567")?.open()?;',
      '```',
      '',
      '## After Streaming',
      '',
      '`Camera::stream()` consumes the current `Camera` and returns a `Stream`. If you need to configure the camera again, call `Stream::stop()` first to get the `Camera` back.'
    ])
  },
  'hikcamera/lifecycle': {
    zh: docPage('生命周期', hikcameraDocs.system),
    en: docPage('Lifecycle', hikcameraDocs.system)
  },
  'hikcamera/devices': {
    zh: docPage('设备信息', hikcameraDocs.device),
    en: docPage('Devices', hikcameraDocs.device)
  },
  'hikcamera/camera': {
    zh: docPage('相机控制', hikcameraDocs.camera),
    en: docPage('Camera', hikcameraDocs.camera)
  },
  'hikcamera/errors': {
    zh: docPage('错误处理', hikcameraDocs.error),
    en: docPage('Error Handling', hikcameraDocs.error)
  },
  'hikcamera-sys': {
    zh: docPage('hikcamera-sys', hikcameraSysDocs.index),
    en: docPage('hikcamera-sys', hikcameraSysDocs.index)
  },
  'design-philosophy': {
    zh: md([
      '# 设计哲学'
    ]),
    en: md([
      '# Design Philosophy'
    ])
  }
}

const pageFor = item => {
  const markdown = body[item.slug] ?? {
    zh: `# ${item.label.zh}\n\n这一页先预留。`,
    en: `# ${item.label.en}\n\nThis page is reserved for now.`
  }

  return {
    slug: item.slug,
    title: item.label.en,
    titleZh: item.label.zh,
    markdown,
    toc: {
      zh: tocFrom(markdown.zh),
      en: tocFrom(markdown.en)
    }
  }
}

const pages = navigationItems.map(pageFor)

export { navigation, pages }
