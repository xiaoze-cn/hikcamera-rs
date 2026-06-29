import { useMemo, useState } from "react";
import { Boxes, Cpu, FileCode2, Link2, PackageCheck, Workflow } from "lucide-react";

const labels = {
  en: {
    title: "Interactive runtime dependency map",
    subtitle:
      "Switch architecture to see which DLLs are copied next to the final Rust binary and how they relate to headers, link-time artifacts, and runtime imports.",
    arch: "Architecture",
    direct: "Direct path",
    support: "Support DLL",
    system: "System DLL",
    compile: "Compile-time",
    link: "Link-time",
    runtime: "Runtime",
    kept: "kept",
    removed: "removed",
    nodes: {
      headers: "C headers",
      bindgen: "bindgen output",
      importLib: "MvCameraControl.lib",
      mainDll: "MvCameraControl.dll",
      genicam: "GenICam runtime",
      render: "MvRender.dll",
      isp: "MvISPControl.dll",
      ipp: "Intel IPP DLLs",
      crt: "Vendor CRT DLLs",
      system: "Windows system DLLs",
    },
    notes: {
      headers: "wrapper.h includes MvCameraControl.h, which pulls the SDK header set.",
      bindgen: "bindings.rs is generated at build time from the header entrypoint.",
      importLib: "The only native import library linked by rustc/MSVC.",
      mainDll: "The primary vendor runtime loaded by the final binary.",
      genicam: "GCBase, GenApi, NodeMapData, XmlParser, MathParser, and Log.",
      render: "Static import from MvCameraControl.dll, so it stays in both architectures.",
      isp: "Image/ISP support used by SDK APIs declared in MvCameraControl.h.",
      ipp: "ippi.dll and ippcore.dll; pulled by the ISP runtime.",
      crt: "libmmd.dll, msvcr100.dll, and pthreadVC2.dll.",
      system: "Provided by Windows, not stored in the repository.",
    },
    removedTitle: "Removed groups",
    removedIntro: "These are not in the kept runtime path for the current wrapper.",
  },
  zh: {
    title: "交互式运行时依赖图",
    subtitle:
      "切换架构可以查看最终 Rust 二进制旁边会复制哪些 DLL，以及它们和头文件、链接产物、运行时导入之间的关系。",
    arch: "架构",
    direct: "主路径",
    support: "支持 DLL",
    system: "系统 DLL",
    compile: "编译期",
    link: "链接期",
    runtime: "运行期",
    kept: "保留",
    removed: "删除",
    nodes: {
      headers: "C 头文件",
      bindgen: "bindgen 输出",
      importLib: "MvCameraControl.lib",
      mainDll: "MvCameraControl.dll",
      genicam: "GenICam 运行库",
      render: "MvRender.dll",
      isp: "MvISPControl.dll",
      ipp: "Intel IPP DLL",
      crt: "厂商 CRT DLL",
      system: "Windows 系统 DLL",
    },
    notes: {
      headers: "wrapper.h include MvCameraControl.h，后者汇入 SDK 头文件集合。",
      bindgen: "bindings.rs 在构建时从头文件入口生成。",
      importLib: "rustc/MSVC 链接的唯一原生导入库。",
      mainDll: "最终二进制加载的主厂商运行库。",
      genicam: "GCBase、GenApi、NodeMapData、XmlParser、MathParser 和 Log。",
      render: "MvCameraControl.dll 的静态导入，因此两个架构都保留。",
      isp: "MvCameraControl.h 中图像/ISP API 使用的支持库。",
      ipp: "ippi.dll 和 ippcore.dll，由 ISP 运行库拉起。",
      crt: "libmmd.dll、msvcr100.dll 和 pthreadVC2.dll。",
      system: "由 Windows 提供，不存放在仓库中。",
    },
    removedTitle: "已删除分组",
    removedIntro: "这些不在当前 wrapper 的保留运行时路径中。",
  },
};

const archFiles = {
  win64: [
    "CommonParameters.ini",
    "MvCameraControl.lib",
    "MvCameraControl.dll",
    "GCBase_MD_VC120_v3_0_MV.dll",
    "GenApi_MD_VC120_v3_0_MV.dll",
    "Log_MD_VC120_v3_0_MV.dll",
    "MathParser_MD_VC120_v3_0_MV.dll",
    "NodeMapData_MD_VC120_v3_0_MV.dll",
    "XmlParser_MD_VC120_v3_0_MV.dll",
    "MvRender.dll",
    "MvISPControl.dll",
    "ippi.dll",
    "ippcore.dll",
    "libmmd.dll",
    "msvcr100.dll",
    "pthreadVC2.dll",
  ],
  win32: [
    "CommonParameters.ini",
    "MvCameraControl.lib",
    "MvCameraControl.dll",
    "GCBase_MD_VC120_v3_0_MV.dll",
    "GenApi_MD_VC120_v3_0_MV.dll",
    "Log_MD_VC120_v3_0_MV.dll",
    "MathParser_MD_VC120_v3_0_MV.dll",
    "NodeMapData_MD_VC120_v3_0_MV.dll",
    "XmlParser_MD_VC120_v3_0_MV.dll",
    "MvRender.dll",
    "MvISPControl.dll",
    "ippi.dll",
    "ippcore.dll",
    "libmmd.dll",
    "msvcr100.dll",
    "pthreadVC2.dll",
  ],
};

const nodeOrder = [
  "headers",
  "bindgen",
  "importLib",
  "mainDll",
  "genicam",
  "render",
  "isp",
  "ipp",
  "crt",
  "system",
];

const nodeMeta = {
  headers: { stage: "compile", tone: "border-sky-400/60 bg-sky-500/10", icon: FileCode2 },
  bindgen: { stage: "compile", tone: "border-sky-400/60 bg-sky-500/10", icon: Workflow },
  importLib: { stage: "link", tone: "border-amber-400/60 bg-amber-500/10", icon: Link2 },
  mainDll: { stage: "runtime", tone: "border-emerald-400/60 bg-emerald-500/10", icon: PackageCheck },
  genicam: { stage: "runtime", tone: "border-violet-400/60 bg-violet-500/10", icon: Boxes },
  render: { stage: "runtime", tone: "border-violet-400/60 bg-violet-500/10", icon: Boxes },
  isp: { stage: "runtime", tone: "border-violet-400/60 bg-violet-500/10", icon: Boxes },
  ipp: { stage: "runtime", tone: "border-violet-400/60 bg-violet-500/10", icon: Cpu },
  crt: { stage: "runtime", tone: "border-violet-400/60 bg-violet-500/10", icon: Cpu },
  system: { stage: "runtime", tone: "border-zinc-400/40 bg-zinc-500/10", icon: Cpu },
};

const edges = [
  ["headers", "bindgen", "bindgen"],
  ["bindgen", "importLib", "build.rs"],
  ["importLib", "mainDll", "loader"],
  ["mainDll", "genicam", "import"],
  ["mainDll", "render", "import"],
  ["mainDll", "isp", "api support"],
  ["isp", "ipp", "import"],
  ["mainDll", "crt", "import"],
  ["mainDll", "system", "OS"],
];

const removedGroups = [
  ["GUI / wrapper", "MvCameraControlGUI.dll, MvCameraControlWrapper.dll, MvCameraPatch.dll"],
  ["DirectShow", "MvDSS.ax, MvDSS2.ax"],
  ["GenTL producers", "MvProducer*.cti, MvFGProducer*.cti"],
  ["Frame grabber stack", "MVFGControl.dll, MvLCProducer.dll, MvCamLVision.dll"],
  ["Standalone transports", "MvUsb3vTL.dll, MvSerial.dll, MvSerialCtrl.dll"],
  ["Media / codecs", "MediaProcess.dll, ThirdParty/*.dll, D3DCompiler_43.dll"],
];

export default function RuntimeDependencyGraph({ locale = "en" }) {
  const [arch, setArch] = useState("win64");
  const t = labels[locale] ?? labels.en;
  const files = archFiles[arch];
  const stages = useMemo(
    () => [
      [t.compile, ["headers", "bindgen"]],
      [t.link, ["importLib"]],
      [t.runtime, ["mainDll", "genicam", "render", "isp", "ipp", "crt", "system"]],
    ],
    [t],
  );

  return (
    <section className="my-8 overflow-hidden rounded-3xl border border-slate-700/40 bg-slate-950 text-slate-100 shadow-2xl shadow-slate-950/40">
      <div className="border-b border-slate-700/50 bg-gradient-to-br from-sky-500/20 via-slate-950 to-emerald-500/10 p-6">
        <div className="flex flex-col gap-4 md:flex-row md:items-start md:justify-between">
          <div>
            <p className="mb-2 text-xs font-bold uppercase tracking-[0.24em] text-sky-300">hikcamera-sys</p>
            <h2 className="m-0 text-2xl font-semibold text-white md:text-3xl">{t.title}</h2>
            <p className="mt-3 max-w-3xl text-sm leading-6 text-slate-300">{t.subtitle}</p>
          </div>
          <div className="rounded-2xl border border-slate-700/60 bg-slate-900/80 p-2">
            <p className="px-2 pb-2 text-xs font-semibold uppercase tracking-wider text-slate-400">{t.arch}</p>
            <div className="grid grid-cols-2 gap-2">
              {Object.keys(archFiles).map((item) => (
                <button
                  key={item}
                  type="button"
                  onClick={() => setArch(item)}
                  className={`rounded-xl px-4 py-2 text-sm font-semibold transition ${
                    arch === item
                      ? "bg-sky-400 text-slate-950 shadow-lg shadow-sky-500/30"
                      : "bg-slate-800 text-slate-300 hover:bg-slate-700"
                  }`}
                >
                  {item}
                </button>
              ))}
            </div>
          </div>
        </div>
      </div>

      <div className="grid gap-6 p-6 xl:grid-cols-[minmax(0,1fr)_22rem]">
        <div className="space-y-5">
          {stages.map(([stage, ids]) => (
            <div key={stage}>
              <h3 className="mb-3 text-sm font-bold uppercase tracking-[0.2em] text-slate-400">{stage}</h3>
              <div className="grid gap-3 md:grid-cols-2 xl:grid-cols-3">
                {ids.map((id) => {
                  const meta = nodeMeta[id];
                  const Icon = meta.icon;
                  return (
                    <article key={id} className={`rounded-2xl border p-4 ${meta.tone}`}>
                      <div className="mb-3 flex items-center gap-3">
                        <span className="rounded-xl bg-slate-950/60 p-2 text-sky-200">
                          <Icon size={18} />
                        </span>
                        <div>
                          <h4 className="m-0 text-base font-semibold text-white">{t.nodes[id]}</h4>
                          <p className="m-0 text-xs uppercase tracking-wider text-slate-400">{meta.stage}</p>
                        </div>
                      </div>
                      <p className="m-0 text-sm leading-6 text-slate-300">{t.notes[id]}</p>
                    </article>
                  );
                })}
              </div>
            </div>
          ))}

          <div className="rounded-2xl border border-slate-700/60 bg-slate-900/60 p-4">
            <h3 className="mb-3 text-sm font-bold uppercase tracking-[0.2em] text-slate-400">Dependency edges</h3>
            <div className="grid gap-2">
              {edges.map(([from, to, label]) => (
                <div key={`${from}-${to}`} className="grid gap-2 rounded-xl border border-slate-700/50 bg-slate-950/60 p-3 text-sm md:grid-cols-[1fr_auto_1fr] md:items-center">
                  <code className="overflow-hidden text-ellipsis whitespace-nowrap text-sky-200">{t.nodes[from]}</code>
                  <span className="rounded-full bg-slate-800 px-3 py-1 text-center text-xs font-bold uppercase tracking-wider text-emerald-300">{label}</span>
                  <code className="overflow-hidden text-ellipsis whitespace-nowrap text-emerald-200">{t.nodes[to]}</code>
                </div>
              ))}
            </div>
          </div>
        </div>

        <aside className="space-y-4">
          <div className="rounded-2xl border border-emerald-500/30 bg-emerald-500/10 p-4">
            <h3 className="mb-3 text-base font-semibold text-white">{arch} {t.kept}</h3>
            <div className="flex flex-wrap gap-2">
              {files.map((file) => (
                <span key={file} className="rounded-full border border-emerald-400/30 bg-slate-950/60 px-3 py-1 text-xs font-medium text-emerald-100">
                  {file}
                </span>
              ))}
            </div>
          </div>

          <div className="rounded-2xl border border-rose-500/30 bg-rose-500/10 p-4">
            <h3 className="mb-1 text-base font-semibold text-white">{t.removedTitle}</h3>
            <p className="mb-3 text-sm leading-6 text-slate-300">{t.removedIntro}</p>
            <div className="space-y-2">
              {removedGroups.map(([group, examples]) => (
                <div key={group} className="rounded-xl border border-rose-400/20 bg-slate-950/50 p-3">
                  <p className="m-0 text-sm font-semibold text-rose-100">{group}</p>
                  <p className="m-0 mt-1 text-xs leading-5 text-slate-400">{examples}</p>
                </div>
              ))}
            </div>
          </div>
        </aside>
      </div>
    </section>
  );
}
