export interface FileNode {
  name: string;
  relativePath: string;
  isDir: boolean;
  language: string | null;
  size: number;
  children: FileNode[];
}

export interface LangStat {
  language: string;
  files: number;
  bytes: number;
}

export interface ScanResult {
  rootName: string;
  tree: FileNode;
  totalFiles: number;
  totalSize: number;
  truncated: boolean;
  languages: LangStat[];
}

export interface Settings {
  baseUrl: string;
  apiKey: string;
  model: string;
}

export interface SummarizeProgress {
  done: number;
  total: number;
  current: string;
  phase: string;
}

export interface FileSummary {
  relativePath: string;
  summary: string;
}

export interface ProjectAnalysis {
  overview: string;
  fileSummaries: FileSummary[];
}
