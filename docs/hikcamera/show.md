`show` 用来把相机实时画面显示到一个轻量窗口，适合在开发和现场调试时确认画面、焦距、曝光和采集链路。

它不是海康 MVS 客户端的替代品，也不提供参数调节面板。曝光、增益、触发、ROI、像素格式等相机参数仍然通过 `hikcamera::Camera` 设置。

## 安装

```toml
[dependencies]
hikcamera = { git = "https://github.com/xiaoze-cn/hikcamera-rs.git" }
```

## 示例

```rust
use hikcamera::{HikCamera, ShowExt, ShowOptions};

fn main() -> hikcamera::ShowResult<()> {
    let hik = HikCamera::new()?;
    let mut camera = hik.devices()?.default()?.open()?;

    camera.set_exposure(20_000.0)?;

    let stream = camera.stream()?;
    let stream = stream
        .show_with(ShowOptions::new().window_size(1000, 750))?
        .run()?;

    let camera = stream.stop()?;
    camera.close()?;

    Ok(())
}
```

## 使用流程

- 先打开相机并设置需要的参数
  - 例如曝光、增益、触发模式、ROI

- 再调用 `camera.stream()?` 进入采集流

- 在 `Stream` 上调用 `show()` 或 `show_with(...)`
  - `show()` 使用默认窗口参数
  - `show_with(...)` 可以设置窗口大小、标题和取帧超时

- 调用 `run()` 后进入显示窗口
  - 窗口显示期间会持续从这条 `Stream` 取帧
  - 按 `Esc` 或关闭窗口会退出显示

- 显示结束后会返回原来的 `Stream`
  - 之后继续调用 `stream.stop()?`
  - 最后调用 `camera.close()?`

## 窗口参数

- `ShowOptions::new()`
  - 创建默认窗口参数

- `window_size(width, height)`
  - 设置窗口初始大小
  - 默认大小为 `1000 x 750`

- `title(title)`
  - 设置窗口标题
  - 默认标题为 `HikCamera Show`

- `timeout(timeout)`
  - 设置每次取帧的超时时间
  - 默认超时时间为 1 秒

## 行为说明

- `show` 基于已有 `Stream` 工作，不会单独再启动另一条相机流
- `run()` 会阻塞当前线程，直到窗口退出
- 同一条 `Stream` 在显示期间不能同时被其它代码取帧
- 当前显示实现支持 Windows；其它平台会返回不支持错误
- 运行真实相机时仍需要可用的海康 MVS runtime
