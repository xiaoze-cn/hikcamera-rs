import { useMemo, useState } from "react";
import "@xyflow/react/dist/style.css";
import dagre from "dagre";
import {
  Background,
  Controls,
  Handle,
  MarkerType,
  Position,
  ReactFlow,
} from "@xyflow/react";
import {
  Boxes,
  Cpu,
  FileCode2,
  Link2,
  PackageCheck,
  Workflow,
} from "lucide-react";

const locales = {
  en: {
    title: "Runtime dependency graph",
    subtitle:
      "Switch architecture to inspect the vendored DLL closure. Solid arrows are PE imports discovered from DLL import tables; the dashed ISP arrow is a conservative companion path kept for SDK image/ISP APIs.",
    arch: "Architecture",
    build: "Build-time path",
    kept: "kept files",
    removed: "removed groups",
    direct: "PE import",
    companion: "kept companion",
    system: "Windows system DLLs",
    local: "Vendored DLL",
    entry: "Runtime entry",
    nodes: {
      headers: "Headers",
      bindgen: "Bindings",
      importLib: "Import lib",
    },
    notes: {
      headers:
        "wrapper.h includes MvCameraControl.h, which pulls the SDK header set.",
      bindgen: "bindings.rs is generated at build time into Cargo's OUT_DIR.",
      importLib: "The native import library linked by rustc/MSVC.",
    },
    groups: [
      [
        "GUI / wrapper",
        "MvCameraControlGUI.dll, MvCameraControlWrapper.dll, MvCameraPatch.dll",
      ],
      ["DirectShow", "MvDSS.ax, MvDSS2.ax"],
      ["GenTL producers", "MvProducer*.cti, MvFGProducer*.cti"],
      [
        "Frame grabber stack",
        "MVFGControl.dll, MvLCProducer.dll, MvCamLVision.dll",
      ],
      [
        "Standalone transports",
        "MvUsb3vTL.dll, MvSerial.dll, MvSerialCtrl.dll",
      ],
      [
        "Media / codecs",
        "MediaProcess.dll, ThirdParty/*.dll, D3DCompiler_43.dll",
      ],
    ],
  },
  zh: {
    title: "运行时依赖图",
    subtitle:
      "切换架构可以查看随包 DLL 闭包。实线箭头来自 DLL import table；虚线 ISP 箭头表示为 SDK 图像/ISP API 保守保留的 companion 路径。",
    arch: "架构",
    build: "构建期路径",
    kept: "保留文件",
    removed: "已删除分组",
    direct: "PE import",
    companion: "保守保留",
    system: "Windows 系统 DLL",
    local: "随包 DLL",
    entry: "运行时入口",
    nodes: {
      headers: "头文件",
      bindgen: "绑定输出",
      importLib: "导入库",
    },
    notes: {
      headers: "wrapper.h include MvCameraControl.h，后者汇入 SDK 头文件集合。",
      bindgen: "bindings.rs 在构建时生成到 Cargo 的 OUT_DIR。",
      importLib: "rustc/MSVC 链接的原生导入库。",
    },
    groups: [
      [
        "GUI 和 wrapper helper",
        "MvCameraControlGUI.dll, MvCameraControlWrapper.dll, MvCameraPatch.dll",
      ],
      ["DirectShow filter", "MvDSS.ax, MvDSS2.ax"],
      ["GenTL producer plugin", "MvProducer*.cti, MvFGProducer*.cti"],
      ["采集卡控制栈", "MVFGControl.dll, MvLCProducer.dll, MvCamLVision.dll"],
      [
        "独立传输层/运行时 helper",
        "MvUsb3vTL.dll, MvSerial.dll, MvSerialCtrl.dll",
      ],
      ["媒体/编解码", "MediaProcess.dll, ThirdParty/*.dll, D3DCompiler_43.dll"],
    ],
  },
};

const archFiles = {
  win64: [
    "CommonParameters.ini",
    "MvCameraControl.lib",
    "MvCameraControl.dll",
    "MvISPControl.dll",
    "MvRender.dll",
    "GCBase_MD_VC120_v3_0_MV.dll",
    "GenApi_MD_VC120_v3_0_MV.dll",
    "Log_MD_VC120_v3_0_MV.dll",
    "MathParser_MD_VC120_v3_0_MV.dll",
    "NodeMapData_MD_VC120_v3_0_MV.dll",
    "XmlParser_MD_VC120_v3_0_MV.dll",
    "ippi.dll",
    "ippcore.dll",
    "libmmd.dll",
    "msvcp120.dll",
    "msvcr100.dll",
    "msvcr120.dll",
    "pthreadVC2.dll",
  ],
  win32: [
    "CommonParameters.ini",
    "MvCameraControl.lib",
    "MvCameraControl.dll",
    "MvISPControl.dll",
    "MvRender.dll",
    "GCBase_MD_VC120_v3_0_MV.dll",
    "GenApi_MD_VC120_v3_0_MV.dll",
    "Log_MD_VC120_v3_0_MV.dll",
    "MathParser_MD_VC120_v3_0_MV.dll",
    "NodeMapData_MD_VC120_v3_0_MV.dll",
    "XmlParser_MD_VC120_v3_0_MV.dll",
    "ippi.dll",
    "ippcore.dll",
    "libmmd.dll",
    "msvcp120.dll",
    "msvcr100.dll",
    "msvcr120.dll",
    "pthreadVC2.dll",
  ],
};

const buildNodes = ["headers", "bindgen", "importLib"];
const buildMeta = {
  headers: {
    stage: "compile",
    icon: FileCode2,
    tone: "border-sky-200 bg-sky-50 dark:border-sky-800 dark:bg-sky-950/40",
  },
  bindgen: {
    stage: "compile",
    icon: Workflow,
    tone: "border-sky-200 bg-sky-50 dark:border-sky-800 dark:bg-sky-950/40",
  },
  importLib: {
    stage: "link",
    icon: Link2,
    tone: "border-amber-200 bg-amber-50 dark:border-amber-800 dark:bg-amber-950/40",
  },
};

const NODE_WIDTH = 220;
const NODE_HEIGHT = 76;

const flowNodes = [
  node("MvCameraControl.dll", "entry", 0, 250),
  node("MvISPControl.dll", "local", 0, 450),
  node("GCBase_MD_VC120_v3_0_MV.dll", "local", 440, 0),
  node("GenApi_MD_VC120_v3_0_MV.dll", "local", 440, 110),
  node("MvRender.dll", "local", 440, 220),
  node("pthreadVC2.dll", "local", 440, 330),
  node("libmmd.dll", "local", 440, 440),
  node("msvcr120.dll", "local", 440, 550),
  node("ippi.dll", "local", 440, 660),
  node("MathParser_MD_VC120_v3_0_MV.dll", "local", 880, 0),
  node("XmlParser_MD_VC120_v3_0_MV.dll", "local", 880, 110),
  node("Log_MD_VC120_v3_0_MV.dll", "local", 880, 220),
  node("NodeMapData_MD_VC120_v3_0_MV.dll", "local", 880, 330),
  node("msvcp120.dll", "local", 880, 440),
  node("msvcr100.dll", "local", 880, 550),
  node("ippcore.dll", "local", 880, 660),
  node("Windows system DLLs", "system", 1320, 330),
];

const flowEdges = [
  edge("MvCameraControl.dll", "GCBase_MD_VC120_v3_0_MV.dll"),
  edge("MvCameraControl.dll", "GenApi_MD_VC120_v3_0_MV.dll"),
  edge("MvCameraControl.dll", "MvRender.dll"),
  edge("MvCameraControl.dll", "pthreadVC2.dll"),
  edge("MvCameraControl.dll", "libmmd.dll"),
  edge("MvCameraControl.dll", "msvcr120.dll"),
  edge("MvCameraControl.dll", "Windows system DLLs"),
  edge("MvCameraControl.dll", "MvISPControl.dll", "companion"),
  edge("MvISPControl.dll", "pthreadVC2.dll"),
  edge("MvISPControl.dll", "ippi.dll"),
  edge("MvISPControl.dll", "libmmd.dll"),
  edge("MvISPControl.dll", "msvcr120.dll"),
  edge("MvISPControl.dll", "Windows system DLLs"),
  edge("GenApi_MD_VC120_v3_0_MV.dll", "MathParser_MD_VC120_v3_0_MV.dll"),
  edge("GenApi_MD_VC120_v3_0_MV.dll", "XmlParser_MD_VC120_v3_0_MV.dll"),
  edge("GenApi_MD_VC120_v3_0_MV.dll", "Log_MD_VC120_v3_0_MV.dll"),
  edge("GenApi_MD_VC120_v3_0_MV.dll", "NodeMapData_MD_VC120_v3_0_MV.dll"),
  edge("GenApi_MD_VC120_v3_0_MV.dll", "GCBase_MD_VC120_v3_0_MV.dll"),
  edge("GenApi_MD_VC120_v3_0_MV.dll", "msvcp120.dll"),
  edge("GenApi_MD_VC120_v3_0_MV.dll", "msvcr120.dll"),
  edge("XmlParser_MD_VC120_v3_0_MV.dll", "NodeMapData_MD_VC120_v3_0_MV.dll"),
  edge("XmlParser_MD_VC120_v3_0_MV.dll", "GCBase_MD_VC120_v3_0_MV.dll"),
  edge("XmlParser_MD_VC120_v3_0_MV.dll", "msvcp120.dll"),
  edge("XmlParser_MD_VC120_v3_0_MV.dll", "msvcr120.dll"),
  edge("Log_MD_VC120_v3_0_MV.dll", "GCBase_MD_VC120_v3_0_MV.dll"),
  edge("Log_MD_VC120_v3_0_MV.dll", "msvcp120.dll"),
  edge("Log_MD_VC120_v3_0_MV.dll", "msvcr120.dll"),
  edge("NodeMapData_MD_VC120_v3_0_MV.dll", "GCBase_MD_VC120_v3_0_MV.dll"),
  edge("NodeMapData_MD_VC120_v3_0_MV.dll", "msvcp120.dll"),
  edge("NodeMapData_MD_VC120_v3_0_MV.dll", "msvcr120.dll"),
  edge("GCBase_MD_VC120_v3_0_MV.dll", "msvcp120.dll"),
  edge("GCBase_MD_VC120_v3_0_MV.dll", "msvcr120.dll"),
  edge("MathParser_MD_VC120_v3_0_MV.dll", "msvcp120.dll"),
  edge("MathParser_MD_VC120_v3_0_MV.dll", "msvcr120.dll"),
  edge("ippi.dll", "ippcore.dll"),
  edge("msvcp120.dll", "msvcr120.dll"),
  edge("pthreadVC2.dll", "msvcr100.dll"),
  edge("MvRender.dll", "Windows system DLLs"),
  edge("ippi.dll", "Windows system DLLs"),
];

const nodeTypes = { dll: DllNode };

export default function RuntimeDependencyGraph({ locale = "en" }) {
  const [arch, setArch] = useState("win64");
  const t = locales[locale] ?? locales.en;
  const files = archFiles[arch];
  const nodes = useMemo(() => getLayoutedNodes(flowNodes, flowEdges), []);
  const edges = useMemo(() => flowEdges, []);

  return (
    <section className="runtime-deps my-8 overflow-hidden rounded-3xl border border-slate-200 bg-white text-slate-950 shadow-xl dark:border-slate-800 dark:bg-slate-950 dark:text-slate-50">
      <div className="border-b border-slate-200 bg-slate-50 p-6 dark:border-slate-800 dark:bg-slate-900">
        <div className="flex flex-col gap-4 md:flex-row md:items-start md:justify-between">
          <div>
            <p className="mb-2 text-xs font-bold uppercase tracking-[0.24em] text-sky-600 dark:text-sky-300">
              hikcamera-sys
            </p>
            <h2 className="m-0 text-2xl font-semibold md:text-3xl">
              {t.title}
            </h2>
            <p className="mt-3 max-w-3xl text-sm leading-6 text-slate-600 dark:text-slate-300">
              {t.subtitle}
            </p>
          </div>
          <div className="rounded-2xl border border-slate-200 bg-white p-2 dark:border-slate-700 dark:bg-slate-800">
            <p className="px-2 pb-2 text-xs font-semibold uppercase tracking-wider text-slate-500 dark:text-slate-400">
              {t.arch}
            </p>
            <div className="grid grid-cols-2 gap-2">
              {Object.keys(archFiles).map((item) => (
                <button
                  key={item}
                  type="button"
                  onClick={() => setArch(item)}
                  className={`rounded-xl px-4 py-2 text-sm font-semibold transition ${
                    arch === item
                      ? "bg-sky-500 text-white shadow-sm dark:bg-sky-400 dark:text-slate-950"
                      : "bg-slate-100 text-slate-700 hover:bg-slate-200 dark:bg-slate-700 dark:text-slate-200 dark:hover:bg-slate-600"
                  }`}
                >
                  {item}
                </button>
              ))}
            </div>
          </div>
        </div>
      </div>

      <div className="grid gap-6 p-6 2xl:grid-cols-[minmax(0,1fr)_22rem]">
        <div className="space-y-6">
          <div>
            <h3 className="mb-3 text-sm font-bold uppercase tracking-[0.2em] text-slate-500 dark:text-slate-400">
              {t.build}
            </h3>
            <div className="grid gap-3 lg:grid-cols-3">
              {buildNodes.map((id) => {
                const meta = buildMeta[id];
                const Icon = meta.icon;
                return (
                  <article
                    key={id}
                    className={`min-w-0 rounded-2xl border p-4 ${meta.tone}`}
                  >
                    <div className="mb-3 flex items-start gap-3">
                      <span className="rounded-xl bg-white p-2 text-sky-600 dark:bg-slate-900 dark:text-sky-300">
                        <Icon size={18} />
                      </span>
                      <div>
                        <h4 className="m-0 text-base font-semibold leading-tight">
                          {t.nodes[id]}
                        </h4>
                        <p className="m-0 text-xs uppercase tracking-wider text-slate-500 dark:text-slate-400">
                          {meta.stage}
                        </p>
                      </div>
                    </div>
                    <p className="m-0 text-sm leading-6 text-slate-600 dark:text-slate-300">
                      {t.notes[id]}
                    </p>
                  </article>
                );
              })}
            </div>
          </div>

          <div>
            <h3 className="mb-3 text-sm font-bold uppercase tracking-[0.2em] text-slate-500 dark:text-slate-400">
              {t.runtime}
            </h3>
            <div className="h-[820px] overflow-hidden rounded-2xl border border-slate-200 bg-slate-50 dark:border-slate-800 dark:bg-slate-900">
              <ReactFlow
                defaultNodes={nodes}
                defaultEdges={edges}
                nodeTypes={nodeTypes}
                fitView
                fitViewOptions={{
                  padding: 0.16,
                  maxZoom: 0.95,
                  nodes: [
                    { id: "MvCameraControl.dll" },
                    { id: "MvISPControl.dll" },
                    { id: "GenApi_MD_VC120_v3_0_MV.dll" },
                    { id: "MvRender.dll" },
                    { id: "pthreadVC2.dll" },
                    { id: "libmmd.dll" },
                    { id: "msvcr120.dll" },
                  ],
                }}
                minZoom={0.25}
                maxZoom={1.5}
                nodesDraggable={false}
                elementsSelectable={false}
                proOptions={{ hideAttribution: true }}
              >
                <Background
                  color="currentColor"
                  gap={22}
                  className="text-slate-200 dark:text-slate-700"
                />
                <Controls showInteractive={false} />
              </ReactFlow>
            </div>
            <div className="mt-3 flex flex-wrap gap-2 text-xs">
              <Legend
                className="border-emerald-500 text-emerald-700 dark:text-emerald-300"
                label={t.entry}
              />
              <Legend
                className="border-violet-500 text-violet-700 dark:text-violet-300"
                label={t.local}
              />
              <Legend
                className="border-slate-400 text-slate-600 dark:border-slate-500 dark:text-slate-300"
                label={t.system}
              />
              <Legend
                className="border-sky-500 text-sky-700 dark:text-sky-300"
                label={t.direct}
              />
              <Legend
                className="border-amber-500 text-amber-700 dark:text-amber-300"
                label={t.companion}
              />
            </div>
          </div>
        </div>

        <aside className="space-y-4">
          <div className="rounded-2xl border border-emerald-200 bg-emerald-50 p-4 dark:border-emerald-800 dark:bg-emerald-950/40">
            <h3 className="mb-3 text-base font-semibold">
              {arch} {t.kept}
            </h3>
            <div className="flex flex-wrap gap-2">
              {files.map((file) => (
                <span
                  key={file}
                  className="rounded-full border border-emerald-300 bg-white px-3 py-1 text-xs font-medium text-emerald-700 dark:border-emerald-700 dark:bg-slate-950 dark:text-emerald-300"
                >
                  {file}
                </span>
              ))}
            </div>
          </div>

          <div className="rounded-2xl border border-slate-200 bg-slate-50 p-4 dark:border-slate-800 dark:bg-slate-900">
            <h3 className="mb-3 text-base font-semibold">{t.removed}</h3>
            <div className="space-y-3">
              {t.groups.map(([name, examples]) => (
                <div
                  key={name}
                  className="rounded-xl border border-slate-200 bg-white p-3 dark:border-slate-700 dark:bg-slate-950"
                >
                  <p className="m-0 text-sm font-semibold">{name}</p>
                  <p className="m-0 mt-1 text-xs leading-5 text-slate-600 dark:text-slate-400">
                    {examples}
                  </p>
                </div>
              ))}
            </div>
          </div>
        </aside>
      </div>
    </section>
  );
}

function DllNode({ data }) {
  const Icon =
    data.kind === "entry" ? PackageCheck : data.kind === "system" ? Cpu : Boxes;
  const tone =
    data.kind === "entry"
      ? "border-emerald-300 bg-emerald-50 text-emerald-950 dark:border-emerald-700 dark:bg-emerald-950 dark:text-emerald-50"
      : data.kind === "system"
        ? "border-slate-300 bg-slate-100 text-slate-950 dark:border-slate-600 dark:bg-slate-800 dark:text-slate-50"
        : "border-violet-300 bg-violet-50 text-violet-950 dark:border-violet-700 dark:bg-violet-950 dark:text-violet-50";

  return (
    <div
      className={`h-[76px] w-[220px] rounded-xl border p-3 shadow-lg ${tone}`}
    >
      <Handle type="target" position={Position.Top} />
      <div className="flex min-w-0 items-center gap-2">
        <Icon size={16} />
        <code className="overflow-hidden text-ellipsis whitespace-nowrap bg-transparent p-0 text-xs">
          {data.label}
        </code>
      </div>
      <div className="mt-2 text-[0.65rem] uppercase tracking-widest text-slate-500 dark:text-slate-400">
        {data.kind}
      </div>
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
}

function Legend({ className, label }) {
  return (
    <span
      className={`rounded-full border px-2.5 py-1 font-semibold ${className}`}
    >
      {label}
    </span>
  );
}

function node(id, kind, x = 0, y = 0) {
  return {
    id,
    type: "dll",
    position: { x, y },
    sourcePosition: Position.Bottom,
    targetPosition: Position.Top,
    data: { label: id, kind },
  };
}

function edge(source, target, kind = "direct") {
  const companion = kind === "companion";
  return {
    id: `${source}->${target}-${kind}`,
    source,
    target,
    type: "smoothstep",
    markerEnd: {
      type: MarkerType.ArrowClosed,
      color: companion ? "#f59e0b" : "#0ea5e9",
    },
    style: {
      stroke: companion ? "#f59e0b" : "#0ea5e9",
      strokeWidth: companion ? 4.5 : 4,
      opacity: 1,
      strokeDasharray: companion ? "8 7" : undefined,
    },
  };
}

function getLayoutedNodes(nodes, edges) {
  const graph = new dagre.graphlib.Graph();
  graph.setDefaultEdgeLabel(() => ({}));
  graph.setGraph({
    rankdir: "TB",
    nodesep: 28,
    ranksep: 54,
    marginx: 24,
    marginy: 24,
  });

  for (const item of nodes) {
    graph.setNode(item.id, { width: NODE_WIDTH, height: NODE_HEIGHT });
  }

  for (const item of edges) {
    graph.setEdge(item.source, item.target);
  }

  dagre.layout(graph);

  return nodes.map((item) => {
    const layout = graph.node(item.id);
    return {
      ...item,
      position: {
        x: layout.x - NODE_WIDTH / 2,
        y: layout.y - NODE_HEIGHT / 2,
      },
    };
  });
}
