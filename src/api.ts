import { invoke } from "@tauri-apps/api/core";
import { Channel } from "@tauri-apps/api/core";
import { SummarizeProgress } from "./types";

export async function aiExplainFile(rootPath: string, relativePath: string): Promise<string> {
  return invoke<string>("ai_explain_file", {
    rootPath,
    relativePath,
  });
}

export function aiSummarizeProject(
  rootPath: string,
  maxFiles: number,
  onProgress: (p: SummarizeProgress) => void
): Promise<{ overview: string; fileSummaries: { relativePath: string; summary: string }[] }> {
  const channel = new Channel<SummarizeProgress>();
  channel.onmessage = onProgress;
  return invoke("ai_summarize_project", {
    rootPath,
    maxFiles,
    channel,
  });
}
