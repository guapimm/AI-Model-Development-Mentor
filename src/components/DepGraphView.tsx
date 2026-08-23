import { useEffect, useRef, useState } from "react";
import { Graph } from "@antv/g6";
import { DepGraphData } from "../types";
import { langColor } from "../utils";

export default function DepGraphView({
  data,
  onSelectFile,
}: {
  data: DepGraphData;
  onSelectFile: (path: string) => void;
}) {
  const containerRef = useRef<HTMLDivElement | null>(null);
  const graphRef = useRef<Graph | null>(null);
  const [selectedNode, setSelectedNode] = useState<string | null>(null);

  useEffect(() => {
    if (!containerRef.current) return;

    const graph = new Graph({
      container: containerRef.current,
      animation: false,
      autoFit: "view",
      padding: 20,
      data: {
        nodes: data.nodes.map((n) => ({ id: n.id, data: { ...n } })),
        edges: data.edges.map((e, i) => ({
          id: `e${i}`,
          source: e.from,
          target: e.to,
        })),
      },
      node: {
        style: {
          size: (d: any) => 8 + Math.min(18, ((d.data?.inDegree ?? 0) + (d.data?.outDegree ?? 0)) * 1.5),
          fill: (d: any) => langColor(d.data?.language ?? null),
          stroke: "#14171c",
          lineWidth: 1,
          labelText: (d: any) => d.id.split("/").pop() ?? d.id,
          labelPlacement: "right",
          labelFontSize: 10,
          labelFill: "#8b93a1",
          labelBackground: true,
          labelBackgroundFill: "rgba(20,23,28,0.8)",
          labelBackgroundRadius: 3,
          labelPadding: [1, 4],
        },
      },
      edge: {
        style: {
          stroke: "#333c4d",
          strokeWidth: 1,
          endArrowSize: 5,
          endArrowFill: "#333c4d",
        },
      },
      layout: {
        type: "d3-force",
        link: { distance: 60, strength: 0.2 },
        manyBody: { strength: -120 },
      },
      behaviors: ["drag-canvas", "zoom-canvas", "drag-element"],
    }) as unknown as Graph;

    graph.on("node:click", (evt: any) => {
      const id = evt?.target?.id ?? evt?.target?._nodeData?.id;
      if (typeof id === "string") {
        setSelectedNode(id);
        onSelectFile(id);
      }
    });

    graph.render().catch(() => {});
    graphRef.current = graph;

    return () => {
      graph.destroy();
      graphRef.current = null;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [data]);

  const nodeInfo = selectedNode
    ? data.nodes.find((n) => n.id === selectedNode)
    : null;

  return (
    <div className="dep-graph-wrap">
      <div className="dep-graph-meta">
        <span>{data.filesScanned} 个文件 · 解析出 {data.edgesResolved} 条依赖</span>
        <span>节点按连接度着色大小 · 拖拽/滚轮缩放 · 点击节点定位文件</span>
      </div>
      {data.truncated && (
        <p className="dim-note">⚠️ 依赖过多，仅展示前 4000 条边，图谱可能不完整。</p>
      )}
      {nodeInfo && (
        <p className="dim-note">
          当前选中：<b className="mono">{nodeInfo.id}</b>（{nodeInfo.language}）·
          被依赖 {nodeInfo.inDegree} 次 · 依赖 {nodeInfo.outDegree} 个文件
        </p>
      )}
      <div ref={containerRef} className="dep-graph-canvas" />
    </div>
  );
}
