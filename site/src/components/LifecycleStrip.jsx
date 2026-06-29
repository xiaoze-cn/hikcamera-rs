const labels = {
  en: {
    title: "Lifecycle at a glance",
    stages: [
      {
        step: "HikCamera",
        kind: "Global SDK init",
        description:
          "Initializes MV_CC_Initialize on first instance, shares a refcount, finalizes on last drop.",
      },
      {
        step: "Devices",
        kind: "Enumeration",
        description:
          "MV_CC_EnumDevices returns MV_CC_DEVICE_INFO_LIST; helper accessors select by serial / user name / IP / MAC.",
      },
      {
        step: "Device",
        kind: "Selected handle",
        description:
          "Borrows the HikCamera lifetime, holds the raw MV_CC_DEVICE_INFO, and exposes open() to construct a Camera.",
      },
      {
        step: "Camera",
        kind: "Open device",
        description:
          "Wraps CreateHandle + OpenDevice, owns the parameter node map, and configures exposure / gain / pixel format.",
      },
      {
        step: "Stream",
        kind: "Grabbing",
        description:
          "Wraps StartGrabbing, returns Frame via GetImageBuffer / FreeImageBuffer, owns video recording state.",
      },
      {
        step: "Frame",
        kind: "Image data",
        description:
          "Single image buffer with frame info; convertible, rotatable, flippable, and writable to disk.",
      },
    ],
  },
  zh: {
    title: "生命周期一览",
    stages: [
      {
        step: "HikCamera",
        kind: "全局 SDK 初始化",
        description:
          "首个实例调用 MV_CC_Initialize，多实例共享引用计数，最后一个 drop 时调用 MV_CC_Finalize。",
      },
      {
        step: "Devices",
        kind: "枚举集合",
        description:
          "MV_CC_EnumDevices 返回 MV_CC_DEVICE_INFO_LIST，提供按序列号 / 用户名 / IP / MAC 选择的便捷方法。",
      },
      {
        step: "Device",
        kind: "选中设备",
        description:
          "借用 HikCamera 生命周期，保存原始 MV_CC_DEVICE_INFO，提供 open() 构造 Camera。",
      },
      {
        step: "Camera",
        kind: "已打开设备",
        description:
          "包装 CreateHandle + OpenDevice，持有参数节点表，配置曝光 / 增益 / 像素格式等。",
      },
      {
        step: "Stream",
        kind: "采集流",
        description:
          "包装 StartGrabbing，通过 GetImageBuffer / FreeImageBuffer 取帧，并管理录像状态。",
      },
      {
        step: "Frame",
        kind: "图像帧",
        description: "单帧图像数据与信息，可转换、旋转、翻转或写入磁盘。",
      },
    ],
  },
};

export default function LifecycleStrip({ locale = "en" }) {
  const t = labels[locale] ?? labels.en;
  return (
    <section className="my-8 overflow-hidden rounded-3xl border border-slate-700/40 bg-slate-950 text-slate-100 shadow-2xl shadow-slate-950/40">
      <div className="border-b border-slate-700/50 bg-gradient-to-br from-sky-500/20 via-slate-950 to-emerald-500/10 p-6">
        <p className="mb-2 text-xs font-bold uppercase tracking-[0.24em] text-sky-300">
          hikcamera
        </p>
        <h2 className="m-0 text-2xl font-semibold text-white md:text-3xl">{t.title}</h2>
      </div>
      <ol className="grid gap-3 p-6 md:grid-cols-2 xl:grid-cols-3">
        {t.stages.map((stage, index) => (
          <li
            key={stage.step}
            className="rounded-2xl border border-slate-700/60 bg-slate-900/60 p-4"
          >
            <div className="mb-3 flex items-center gap-3">
              <span className="grid h-9 w-9 place-items-center rounded-full bg-sky-400 text-base font-bold text-slate-950 shadow-lg shadow-sky-500/30">
                {index + 1}
              </span>
              <div>
                <p className="m-0 text-base font-semibold text-white">
                  <code className="text-sky-200">{stage.step}</code>
                </p>
                <p className="m-0 text-xs uppercase tracking-wider text-slate-400">
                  {stage.kind}
                </p>
              </div>
            </div>
            <p className="m-0 text-sm leading-6 text-slate-300">{stage.description}</p>
          </li>
        ))}
      </ol>
    </section>
  );
}
