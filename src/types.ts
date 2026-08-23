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

export interface StaticProgress {
  phase: string;
  percent: number;
}

export interface TechStackItem {
  name: string;
  category: string;
  source: string;
}

export interface FileMetric {
  relativePath: string;
  language: string;
  lines: number;
  codeLines: number;
  todos: number;
}

export interface EntryPoint {
  relativePath: string;
  reason: string;
}

export interface StaticReport {
  rootName: string;
  techStack: TechStackItem[];
  entryPoints: EntryPoint[];
  metrics: FileMetric[];
  totalCodeFiles: number;
  totalLines: number;
  totalTodos: number;
  warnings: string[];
}

export interface SymbolInfo {
  kind: string;
  name: string;
  startLine: number;
  endLine: number;
  signature: string;
}

export interface FileSymbols {
  relativePath: string;
  language: string;
  supportedParse: boolean;
  symbols: SymbolInfo[];
  imports: string[];
}

export interface DepGraphNode {
  id: string;
  language: string;
  inDegree: number;
  outDegree: number;
}

export interface DepGraphEdge {
  from: string;
  to: string;
}

export interface DepGraphData {
  nodes: DepGraphNode[];
  edges: DepGraphEdge[];
  filesScanned: number;
  edgesResolved: number;
  truncated: boolean;
}
