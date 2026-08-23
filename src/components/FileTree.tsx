import { useState } from "react";
import { FileNode } from "../types";
import { formatBytes, langColor } from "../utils";

function TreeItem({
  node,
  depth,
  onSelect,
  selectedPath,
}: {
  node: FileNode;
  depth: number;
  onSelect: (node: FileNode) => void;
  selectedPath: string | null;
}) {
  const [open, setOpen] = useState(depth < 1);

  return (
    <div>
      <div
        className={`tree-row ${selectedPath === node.relativePath ? "selected" : ""}`}
        style={{ paddingLeft: `${depth * 14 + 8}px` }}
        onClick={() => {
          if (node.isDir) setOpen((o) => !o);
          onSelect(node);
        }}
      >
        <span className="tree-icon">{node.isDir ? (open ? "📂" : "📁") : "📄"}</span>
        <span className="tree-name" title={node.relativePath}>{node.name}</span>
        {node.language && (
          <>
            <span className="lang-dot" style={{ background: langColor(node.language) }} />
            <span className="tree-lang">{node.language}</span>
          </>
        )}
        {!node.isDir && (
          <span className="tree-size">{formatBytes(node.size)}</span>
        )}
      </div>
      {node.isDir && open && (
        <div>
          {node.children.map((child) => (
            <TreeItem
              key={child.relativePath}
              node={child}
              depth={depth + 1}
              onSelect={onSelect}
              selectedPath={selectedPath}
            />
          ))}
        </div>
      )}
    </div>
  );
}

export default function FileTree({
  root,
  onSelect,
  selectedPath,
}: {
  root: FileNode;
  onSelect: (node: FileNode) => void;
  selectedPath: string | null;
}) {
  return (
    <div className="file-tree">
      {root.children.map((child) => (
        <TreeItem
          key={child.relativePath}
          node={child}
          depth={0}
          onSelect={onSelect}
          selectedPath={selectedPath}
        />
      ))}
    </div>
  );
}
