use crate::scanner::{self, FileNode};
use crate::symbols::extract_symbols;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use tauri::ipc::Channel;

use crate::static_analysis::StaticProgress;

/// Skip files above this size for import parsing.
const MAX_PARSE_BYTES: usize = 300_000;
const MAX_EDGES: usize = 4000;
/// Keep the graph renderable.
const MAX_NODES: usize = 300;

#[derive(Serialize, Clone)]
pub struct DepGraphNode {
    pub id: String,
    pub language: String,
    #[serde(rename = "inDegree")]
    pub in_degree: usize,
    #[serde(rename = "outDegree")]
    pub out_degree: usize,
}

#[derive(Serialize, Clone)]
pub struct DepGraphEdge {
    pub from: String,
    pub to: String,
}

#[derive(Serialize)]
pub struct DepGraphData {
    pub nodes: Vec<DepGraphNode>,
    pub edges: Vec<DepGraphEdge>,
    #[serde(rename = "filesScanned")]
    pub files_scanned: usize,
    #[serde(rename = "edgesResolved")]
    pub edges_resolved: usize,
    pub truncated: bool,
}

struct FileIndex {
    /// every trailing-suffix path (no ext, lowercased) -> file indices
    suffix_map: HashMap<String, Vec<usize>>,
}

impl FileIndex {
    fn build(files: &[FileNode]) -> Self {
        let mut suffix_map: HashMap<String, Vec<usize>> = HashMap::new();

        for (i, f) in files.iter().enumerate() {
            let p = f.relative_path.to_lowercase();
            let key = strip_ext(&p);
            let segs: Vec<&str> = key.split('/').collect();
            // Register all trailing suffixes: "a/b/c" -> "c", "b/c", "a/b/c".
            for start in 0..segs.len() {
                let suffix = segs[start..].join("/");
                if suffix.len() > 2 {
                    suffix_map.entry(suffix).or_default().push(i);
                }
            }
        }

        Self { suffix_map }
    }

    fn lookup(&self, frag_lower_noext: &str) -> Vec<usize> {
        self.suffix_map
            .get(frag_lower_noext)
            .cloned()
            .unwrap_or_default()
    }
}

fn strip_ext(p: &str) -> String {
    match p.rsplit_once('.') {
        Some((base, ext)) if ext.len() <= 5 && !base.is_empty() => base.to_string(),
        _ => p.to_string(),
    }
}

fn dir_of(rel: &str) -> String {
    match rel.rsplit_once('/') {
        Some((dir, _)) => dir.to_string(),
        None => String::new(),
    }
}

/// Normalize a relative path (resolve ./ and ../ segments).
fn normalize_rel(base_dir: &str, target: &str) -> String {
    let joined = if base_dir.is_empty() {
        target.to_string()
    } else {
        format!("{}/{}", base_dir, target)
    };
    let mut out: Vec<&str> = Vec::new();
    for seg in joined.split('/') {
        match seg {
            "." | "" => {}
            ".." => {
                out.pop();
            }
            s => out.push(s),
        }
    }
    out.join("/")
}

fn quoted_strings(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'"' || bytes[i] == b'\'' {
            let quote = bytes[i];
            let start = i + 1;
            let mut j = start;
            while j < bytes.len() && bytes[j] != quote {
                j += 1;
            }
            if j < bytes.len() && j > start {
                out.push(text[start..j].to_string());
            }
            i = j + 1;
        } else {
            i += 1;
        }
    }
    out
}

/// Extract candidate module paths from a raw import statement.
fn import_targets(lang: &str, raw: &str) -> Vec<String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Vec::new();
    }

    // Quoted targets first (JS/TS, Go, C/C++ includes, Ruby require).
    let quotes = quoted_strings(raw);
    if !quotes.is_empty() {
        return quotes
            .into_iter()
            .filter(|q| !q.is_empty())
            .collect();
    }

    let mut body = raw.to_string();
    for prefix in ["use ", "using ", "import ", "from ", "#include ", "include ", "require "] {
        if let Some(rest) = body.strip_prefix(prefix) {
            body = rest.to_string();
            break;
        }
    }
    body = body
        .trim_start_matches("static ")
        .trim_end_matches(';')
        .trim()
        .to_string();

    match lang {
        "Python" => {
            // "from x.y import z" -> x/y ; "import a.b, c.d" -> a/b, c/d
            let mut outs = Vec::new();
            let source_part = match body.find(" import ") {
                Some(idx) => body[..idx].to_string(),
                None => body.clone(),
            };
            for part in source_part.split(',') {
                let t = part.trim().replace('.', "/");
                if !t.is_empty() {
                    outs.push(t);
                }
            }
            outs
        }
        "Rust" => {
            let b = body.split("::{").next().unwrap_or(&body);
            let b = b.split('{').next().unwrap_or(b);
            vec![b.replace("::", "/")]
        }
        "Java" | "C#" | "PHP" => {
            vec![body.replace('.', "/")]
        }
        _ => Vec::new(),
    }
}

/// Resolve one candidate to project-internal file indices.
fn resolve_candidate(
    candidate: &str,
    lang: &str,
    importer: &str,
    index: &FileIndex,
) -> Vec<usize> {
    let c = candidate.trim();
    if c.is_empty() || c.starts_with("http") || c.starts_with("node:") {
        return Vec::new();
    }

    // JS bare specifiers (npm packages) are external — skip unless relative.
    let is_js = matches!(lang, "TypeScript" | "JavaScript" | "TSX" | "JSX");
    let c = c.trim_end_matches('/');

    if is_js && !c.starts_with('.') {
        return Vec::new();
    }

    let importer_dir = dir_of(importer);

    let candidates: Vec<String> = if c.starts_with('.') {
        vec![normalize_rel(&importer_dir, c)]
    } else {
        match lang {
            "Rust" => {
                let mut list = Vec::new();
                let stripped = c.strip_prefix("crate/").map(|r| format!("src/{}", r));
                let base = stripped.unwrap_or_else(|| c.to_string());
                list.push(base.clone());
                // super/self relative forms
                if let Some(rest) = c.strip_prefix("super/") {
                    let parent = dir_of(&importer_dir);
                    list.push(normalize_rel(&parent, rest));
                }
                if let Some(rest) = c.strip_prefix("self/") {
                    list.push(normalize_rel(&importer_dir, rest));
                }
                list
            }
            "Python" | "Java" | "C#" | "PHP" | "Ruby" => {
                vec![c.to_string()]
            }
            "C" | "C++" => {
                vec![normalize_rel(&importer_dir, c)]
            }
            _ => vec![c.to_string()],
        }
    };

    let mut hits = Vec::new();
    for cand in &candidates {
        let lower = cand.to_lowercase();
        for idx in index.lookup(&lower) {
            if !hits.contains(&idx) {
                hits.push(idx);
                if hits.len() >= 3 {
                    return hits;
                }
            }
        }
    }

    // Fallback: imports often carry a trailing symbol name
    // (e.g. `use crate::models::user::User` -> drop "user").
    let mut lower = candidates[0].to_lowercase();
    for _ in 0..2 {
        match lower.rsplit_once('/') {
            Some((parent, _)) => {
                lower = parent.to_string();
                for idx in index.lookup(&lower) {
                    if !hits.contains(&idx) {
                        hits.push(idx);
                        if hits.len() >= 3 {
                            return hits;
                        }
                    }
                }
            }
            None => break,
        }
    }

    hits
}

pub fn build_dependency_graph(
    root: &Path,
    channel: Channel<StaticProgress>,
) -> Result<DepGraphData, String> {
    macro_rules! progress {
        ($pct:expr, $phase:expr) => {
            let _ = channel.send(StaticProgress {
                phase: $phase.to_string(),
                percent: $pct,
            });
        };
    }

    progress!(3, "扫描目录结构");
    let scan = scanner::scan_project(root)?;

    let mut files: Vec<FileNode> = Vec::new();
    collect_parsable_files(&scan.tree, &mut files);

    let total = files.len().max(1);
    progress!(8, format!("准备解析 {} 个文件的依赖", files.len()));

    let index = FileIndex::build(&files);

    // importer -> set of target indices
    let mut edge_set: HashSet<(usize, usize)> = HashSet::new();
    let mut last_percent: u8 = 8;

    for (i, f) in files.iter().enumerate() {
        let full = root.join(&f.relative_path);
        if let Ok(meta) = std::fs::metadata(&full) {
            if meta.len() as usize <= MAX_PARSE_BYTES {
                if let Ok(source) = std::fs::read_to_string(&full) {
                    let lang = f.language.clone().unwrap_or_default();
                    if let Some(fs) = extract_symbols(&f.relative_path, &lang, &source) {
                        let importer_idx =
                            files.iter().position(|x| x.relative_path == f.relative_path);
                        if let Some(from) = importer_idx {
                            for imp in &fs.imports {
                                for target in import_targets(&lang, imp) {
                                    for to in resolve_candidate(&target, &lang, &f.relative_path, &index)
                                    {
                                        if to != from {
                                            edge_set.insert((from, to));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Progress 8% -> 92%.
        let percent = (8 + ((i as u64 + 1) * 84 / total as u64)) as u8;
        if percent != last_percent {
            last_percent = percent;
            progress!(percent, format!("解析依赖 ({}/{})", i + 1, files.len()));
        }
    }

    progress!(94, "构建图谱");

    // Degree counting.
    let mut in_deg = vec![0usize; files.len()];
    let mut out_deg = vec![0usize; files.len()];
    for (from, to) in &edge_set {
        out_deg[*from] += 1;
        in_deg[*to] += 1;
    }

    // Keep top MAX_NODES by total degree so the render stays usable.
    let mut order: Vec<usize> = (0..files.len()).collect();
    order.sort_by(|a, b| {
        (in_deg[*b] + out_deg[*b]).cmp(&(in_deg[*a] + out_deg[*a]))
    });
    let keep: HashSet<usize> = order.into_iter().take(MAX_NODES).collect();

    let mut nodes: Vec<DepGraphNode> = Vec::new();
    for &i in &keep {
        nodes.push(DepGraphNode {
            id: files[i].relative_path.clone(),
            language: files[i].language.clone().unwrap_or_default(),
            in_degree: in_deg[i],
            out_degree: out_deg[i],
        });
    }

    let mut truncated = false;
    let mut edges: Vec<DepGraphEdge> = Vec::new();
    for (from, to) in &edge_set {
        if keep.contains(from) && keep.contains(to) {
            edges.push(DepGraphEdge {
                from: files[*from].relative_path.clone(),
                to: files[*to].relative_path.clone(),
            });
        }
    }
    if edges.len() > MAX_EDGES {
        edges.truncate(MAX_EDGES);
        truncated = true;
    }

    progress!(100, "完成");

    Ok(DepGraphData {
        nodes,
        edges,
        files_scanned: files.len(),
        edges_resolved: edge_set.len(),
        truncated,
    })
}

fn collect_parsable_files(node: &FileNode, out: &mut Vec<FileNode>) {
    if node.is_dir {
        for child in &node.children {
            collect_parsable_files(child, out);
        }
    } else if node.language.is_some() {
        out.push(node.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(path: &str, lang: &str) -> FileNode {
        let name = path.rsplit('/').next().unwrap_or(path).to_string();
        FileNode {
            name,
            relative_path: path.to_string(),
            is_dir: false,
            language: Some(lang.to_string()),
            size: 10,
            children: Vec::new(),
        }
    }

    #[test]
    fn test_import_targets_extraction() {
        assert_eq!(
            import_targets("Python", "from app.models import User"),
            vec!["app/models".to_string()]
        );
        assert_eq!(
            import_targets("Rust", "use crate::models::user::User;"),
            vec!["crate/models/user/User".to_string()]
        );
        assert_eq!(
            import_targets("Java", "import com.foo.Bar;"),
            vec!["com/foo/Bar".to_string()]
        );
        assert_eq!(
            import_targets("JavaScript", "import x from \"./utils/helper\""),
            vec!["./utils/helper".to_string()]
        );
    }

    #[test]
    fn test_resolution_and_graph() {
        let files = vec![
            node("src/utils/math.py", "Python"),
            node("src/app.py", "Python"),
            node("src/main.rs", "Rust"),
        ];
        let index = FileIndex::build(&files);

        // Python absolute import resolves via suffix map.
        let hits = resolve_candidate("utils/math", "Python", "src/app.py", &index);
        assert_eq!(hits, vec![0usize]);

        // Relative JS import resolves against importer dir.
        let js_files = vec![
            node("src/a/helper.ts", "TypeScript"),
            node("src/a/page.tsx", "TSX"),
        ];
        let js_index = FileIndex::build(&js_files);
        let hits = resolve_candidate("./helper", "TypeScript", "src/a/page.tsx", &js_index);
        assert_eq!(hits, vec![0usize]);

        // Rust crate:: path with trailing symbol falls back to parent segment.
        let rs_files = vec![node("src/main.rs", "Rust"), node("src/models/user.rs", "Rust")];
        let rs_index = FileIndex::build(&rs_files);
        let hits =
            resolve_candidate("crate/models/user/User", "Rust", "src/main.rs", &rs_index);
        assert_eq!(hits, vec![1usize]);
    }
}
