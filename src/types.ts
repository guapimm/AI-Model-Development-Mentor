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
