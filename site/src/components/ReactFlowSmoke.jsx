import "@xyflow/react/dist/style.css";
import { Background, ReactFlow } from "@xyflow/react";

const nodes = [
  {
    id: "source",
    position: { x: 40, y: 80 },
    data: { label: "source" },
    sourcePosition: "right",
  },
  {
    id: "target",
    position: { x: 360, y: 80 },
    data: { label: "target" },
    targetPosition: "left",
  },
];

const edges = [
  {
    id: "source-target",
    source: "source",
    target: "target",
    type: "smoothstep",
    style: { stroke: "#ef4444", strokeWidth: 8 },
  },
];

export default function ReactFlowSmoke() {
  return (
    <div
      style={{
        width: 720,
        height: 320,
        border: "1px solid #cbd5e1",
        borderRadius: 12,
        background: "white",
      }}
    >
      <ReactFlow
        nodes={nodes}
        edges={edges}
        fitView
        fitViewOptions={{ padding: 0.2 }}
        proOptions={{ hideAttribution: true }}
      >
        <Background />
      </ReactFlow>
    </div>
  );
}
