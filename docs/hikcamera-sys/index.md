源码位置：`crates/hikcamera-sys/`

这个 crate 负责把海康 MVS C SDK 接入 Rust 编译流程，导出原始 FFI 绑定和类型正确的 SDK 状态码常量

## 构建入口

- `build.rs`
  - `hikcamera-sys` 的主要构建逻辑，比 `src/lib.rs` 更早执行
  - 负责把 pixi 环境里的海康 MVS SDK 接入 Cargo 构建流程
  - 主要工作：
    - 定位 SDK 安装目录
    - 检查当前编译目标
    - 检查头文件和 import library 是否存在
    - 设置 Cargo 链接参数
    - 调用 bindgen 生成原始 FFI 绑定
    - 额外生成 `i32` 类型的 SDK 状态码常量
  - SDK 路径查找顺序：
    - `CONDA_PREFIX/Library`
      - pixi 激活环境后优先使用
    - `PIXI_PROJECT_ROOT/.pixi/envs/<PIXI_ENVIRONMENT_NAME>/Library`
      - pixi 通过环境变量告诉构建脚本项目根目录和环境名时使用
    - 工作区 `.pixi/envs/default/Library`
      - 没有显式环境变量时的默认回退路径
  - 必须找到：
    - `include/hikcamera-mvs`：海康 MVS 头文件目录
    - `lib/MvCameraControl.lib`：Windows import library
  - 只支持 `windows-x86_64`
    - 其他平台会在构建阶段直接失败

- 链接设置
  - `cargo:rustc-link-search`：指向 pixi 环境里的 SDK `lib` 目录
  - `cargo:rustc-link-lib=dylib=MvCameraControl`：声明链接 `MvCameraControl`
  - 运行时 DLL 由 pixi 环境提供，不在这个 crate 里复制

## bindgen 绑定

- `wrapper.h`
  - bindgen 的入口头文件
  - 先用预处理条件检查 Windows 平台
  - 只包含 `MvCameraControl.h`，不手动展开其它 SDK 头文件

- `MvCameraControl.h`
  - 海康 MVS C SDK 的主入口头文件
  - 继续包含 SDK 内部需要的其它头文件
    - `CameraParams.h`
    - `MvErrorDefine.h`
    - `MvObsoleteInterfaces.h`
    - 其它由 C SDK 自己组织的依赖头文件
  - bindgen 从这个入口看到 C 函数、结构体、枚举和普通宏常量
  - 这样生成结果更接近 C SDK 自己的组织方式

- `bindings.rs`
  - bindgen 生成的原始 FFI 绑定
  - 保留 C SDK 的命名风格
  - 保留 C SDK 的 `unsafe` 调用边界
  - 编译时写入 `OUT_DIR/bindings.rs`
  - 同时复制一份到 `target/generated/bindings.rs`
    - 这份稳定路径用于人工查看和 SDK 升级 diff

## 状态码生成

- 背景
  - 海康 C SDK 函数返回 `int`
  - 错误码宏常见写法是 `0x80000000` 这类十六进制字面量
  - bindgen 容易把这些宏生成为 `u32`
  - Rust 高层封装检查 SDK 返回值时使用 `i32`
  - 如果状态码常量是 `u32`，高层就需要反复做类型转换
  - 所以状态码在 `hikcamera-sys` 里被统一生成为 `i32`

- 特殊处理
  - `build.rs` blocklist：
    - `MvErrorDefine.h`
    - `MvISPErrorDefine.h`
  - 这两个头文件里的状态码不交给 bindgen 直接生成
  - `build.rs` 自己解析并生成 `status_codes.rs`
  - 其它 SDK 类型和普通宏仍然由 bindgen 生成

- 解析范围
  - `MV_OK`：SDK 成功状态
  - `MV_E_*`：MVS SDK 通用错误、GenICam 错误、GigE 错误、USB 错误、图像处理错误等
  - `MV_ALG_*`：ISP 算法库错误

- 生成结果
  - 十进制值直接生成 `i32`
  - 十六进制错误值生成 `<hex>_u32 as i32`
    - 保留原始 bit pattern
    - 对外类型仍然是 `i32`
  - 至少需要解析到 200 个状态码
    - 低于这个数量说明 SDK 头文件格式可能变化，构建会直接失败

- 输出位置
  - 编译使用：`OUT_DIR/status_codes.rs`
  - 人工查看：`target/generated/status_codes.rs`

## 对外导出

- `src/lib.rs`
  - include `status_codes.rs`
  - include `bindings.rs`
  - 放开 C SDK 命名风格相关 lint
    - `non_camel_case_types`
    - `non_snake_case`
    - `non_upper_case_globals`
  - 导出的内容分成两类：
    - 类型为 `i32` 的 SDK 状态码常量
    - bindgen 生成的原始 FFI 绑定

- 使用方式
  - 普通用户优先使用 `hikcamera`
  - 需要直接调用 C SDK 函数时再使用 `hikcamera-sys`
  - `hikcamera-sys` 暴露的是底层能力：
    - 函数调用仍然遵循 C SDK 规则
    - 指针、buffer、handle 生命周期由调用方负责
    - SDK 返回码需要调用方自行检查，或交给高层 `hikcamera` 封装处理
