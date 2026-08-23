import { invoke } from "@tauri-apps/api/core";
import { Channel } from "@tauri-apps/api/core";
import { SummarizeProgress } from "./types";

export type Strength = "light" | "medium" | "deep";

export async function aiExplainFile(
  rootPath: string,
  relativePath: string,
  strength: Strength
): Promise<string> {
  return invoke<string>("ai_explain_file", {
    rootPath,
    relativePath,
    strength,
  });
}

export function aiSummarizeProject(
  rootPath: string,
  strength: Strength,
  onProgress: (p: SummarizeProgress) => void
): Promise<{ overview: string; fileSummaries: { relativePath: string; summary: string }[] }> {
  const channel = new Channel<SummarizeProgress>();
  channel.onmessage = onProgress;
  return invoke("ai_summarize_project", {
    rootPath,
    strength,
    channel,
  });
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
