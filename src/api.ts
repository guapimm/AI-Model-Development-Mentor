import { invoke } from "@tauri-apps/api/core";
import { Channel } from "@tauri-apps/api/core";
import {
  DepGraphData,
  FileSymbols,
  Settings,
  StaticProgress,
  SummarizeProgress,
} from "./types";

export type Strength = "light" | "medium" | "deep";

export async function aiExplainFile(
  rootPath: string,
  relativePath: string,
  strength: Strength,
  unlimitedOutput: boolean
): Promise<string> {
  return invoke<string>("ai_explain_file", {
    rootPath,
    relativePath,
    strength,
    unlimitedOutput,
  });
}

export function aiSummarizeProject(
  rootPath: string,
  strength: Strength,
  fullScope: boolean,
  unlimitedOutput: boolean,
  onProgress: (p: SummarizeProgress) => void
): Promise<{ overview: string; fileSummaries: { relativePath: string; summary: string }[] }> {
  const channel = new Channel<SummarizeProgress>();
  channel.onmessage = onProgress;
  return invoke("ai_summarize_project", {
    rootPath,
    strength,
    fullScope,
    unlimitedOutput,
    channel,
  });
}

export function listAiModels(settings: Settings): Promise<string[]> {
  return invoke("list_ai_models", { settings });
}

export async function testAiConnection(settings: Settings): Promise<void> {
  await invoke("test_ai_connection", { settings });
}

export function analyzeStatic(
  rootPath: string,
  onProgress: (p: StaticProgress) => void
): Promise<StaticReportResult> {
  const channel = new Channel<StaticProgress>();
  channel.onmessage = onProgress;
  return invoke("analyze_static", { rootPath, channel });
}

export function getFileSymbols(
  rootPath: string,
  relativePath: string,
  language: string
): Promise<FileSymbols> {
  return invoke("get_file_symbols", { rootPath, relativePath, language });
}

export function getDependencyGraph(
  rootPath: string,
  onProgress: (p: StaticProgress) => void
): Promise<DepGraphData> {
  const channel = new Channel<StaticProgress>();
  channel.onmessage = onProgress;
  return invoke("get_dependency_graph", { rootPath, channel });
}

interface StaticReportResult {
  rootName: string;
  techStack: { name: string; category: string; source: string }[];
  entryPoints: { relativePath: string; reason: string }[];
  metrics: {
    relativePath: string;
    language: string;
    lines: number;
    codeLines: number;
    todos: number;
  }[];
  totalCodeFiles: number;
  totalLines: number;
  totalTodos: number;
  warnings: string[];
}

export function exportXmind(
  rootPath: string,
  outPath: string,
  fileSummaries?: { relativePath: string; summary: string }[]
): Promise<void> {
  return invoke("export_xmind", {
    rootPath,
    outPath,
    fileSummaries: fileSummaries ?? [],
  });
}
