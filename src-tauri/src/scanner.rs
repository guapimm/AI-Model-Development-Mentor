use ignore::WalkBuilder;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

/// Hard cap to protect against pathological projects.
const MAX_FILES: usize = 50_000;
/// Per-directory display cap for very flat folders.
const MAX_CHILDREN_PER_DIR: usize = 2000;

/// Directories always skipped even when not covered by .gitignore.
const SKIP_DIRS: &[&str] = &[
    "node_modules",
    "__pycache__",
    ".venv",
    "venv",
    "vendor",
    "Pods",
    "DerivedData",
    ".gradle",
    ".idea",
    ".vs",
    ".svn",
    ".hg",
];

#[derive(Serialize, Clone)]
pub struct FileNode {
    pub name: String,
    #[serde(rename = "relativePath")]
    pub relative_path: String,
    pub is_dir: bool,
    pub language: Option<String>,
    pub size: u64,
    pub children: Vec<FileNode>,
}

#[derive(Serialize, Clone)]
pub struct LangStat {
    pub language: String,
    pub files: usize,
    pub bytes: u64,
}

#[derive(Serialize)]
pub struct ScanResult {
    pub root_name: String,
    pub tree: FileNode,
    pub total_files: usize,
    pub total_size: u64,
    pub truncated: bool,
    pub languages: Vec<LangStat>,
}

pub fn extension_language(ext: &str) -> Option<&'static str> {
    let lang = match ext.to_ascii_lowercase().as_str() {
        "rs" => "Rust",
        "js" | "mjs" | "cjs" => "JavaScript",
        "jsx" => "JSX",
        "ts" | "mts" | "cts" => "TypeScript",
        "tsx" => "TSX",
        "py" | "pyi" => "Python",
        "go" => "Go",
        "java" => "Java",
        "kt" | "kts" => "Kotlin",
        "swift" => "Swift",
        "c" | "h" => "C",
        "cc" | "cpp" | "cxx" | "hpp" | "hh" => "C++",
        "cs" => "C#",
        "rb" => "Ruby",
        "php" => "PHP",
        "m" | "mm" => "Objective-C",
        "scala" => "Scala",
        "sh" | "bash" | "zsh" => "Shell",
        "ps1" | "psm1" => "PowerShell",
        "bat" | "cmd" => "Batch",
        "lua" => "Lua",
        "pl" | "pm" => "Perl",
        "r" => "R",
        "dart" => "Dart",
        "vue" => "Vue",
        "svelte" => "Svelte",
        "astro" => "Astro",
        "html" | "htm" => "HTML",
        "css" => "CSS",
        "scss" | "sass" => "SCSS",
        "less" => "Less",
        "json" => "JSON",
        "yaml" | "yml" => "YAML",
        "toml" => "TOML",
        "xml" => "XML",
        "sql" => "SQL",
        "md" | "markdown" => "Markdown",
        "graphql" | "gql" => "GraphQL",
        "proto" => "Protobuf",
        "gradle" => "Gradle",
        "ipynb" => "Jupyter Notebook",
        "zig" => "Zig",
        "ex" | "exs" => "Elixir",
        "erl" => "Erlang",
        "hs" => "Haskell",
        "clj" | "cljs" | "cljc" => "Clojure",
        _ => return None,
    };
    Some(lang)
}

pub fn file_language(name: &str) -> Option<&'static str> {
    let lower = name.to_ascii_lowercase();
    if lower == "dockerfile" || lower.starts_with("dockerfile.") {
        return Some("Dockerfile");
    }
    if lower == "makefile" || lower.starts_with("makefile.")
        || lower == "gnumakefile" {
        return Some("Makefile");
    }
    Path::new(&lower)
        .extension()
        .and_then(|e| e.to_str())
        .and_then(extension_language)
}

fn finalize_children(children: &mut Vec<FileNode>) {
    children.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });
}

pub fn scan_project(root: &Path) -> Result<ScanResult, String> {
    if !root.is_dir() {
        return Err(format!("不是有效的文件夹: {}", root.display()));
    }

    let root_name = root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| root.to_string_lossy().to_string());

    let mut stats: HashMap<&'static str, (usize, u64)> = HashMap::new();

    // Synthetic root frame; stack[i] holds a node at depth i.
    let mut stack: Vec<FileNode> = vec![FileNode {
        name: root_name.clone(),
        relative_path: String::new(),
        is_dir: true,
        language: None,
        size: 0,
        children: Vec::new(),
    }];

    let mut count: usize = 0;
    let mut truncated = false;
    let mut overflow: Vec<FileNode> = Vec::new();
    // While inside a skipped subtree, entries deeper than this depth are ignored.
    let mut skip_deeper_than: usize = usize::MAX;

    WalkBuilder::new(root)
        .hidden(true) // skip dotfiles/dirs like .git; .gitignore rules still apply
        .build()
        .filter_map(|e| e.ok())
        .for_each(|entry| {
            let depth = entry.depth();
            if depth == 0 {
                return;
            }
            if depth > skip_deeper_than {
                return;
            }
            skip_deeper_than = usize::MAX;

            if count >= MAX_FILES {
                truncated = true;
                return;
            }

            let path = entry.path();
            let name = match path.file_name() {
                Some(n) => n.to_string_lossy().to_string(),
                None => return,
            };

            let ftype = match entry.file_type() {
                Some(t) => t,
                None => return, // broken symlink etc.
            };
            let is_dir = ftype.is_dir();

            if is_dir && SKIP_DIRS.contains(&name.as_str()) {
                skip_deeper_than = depth;
                return;
            }

            let size = if is_dir {
                0
            } else {
                entry.metadata().map(|m| m.len()).unwrap_or(0)
            };
            let lang = if is_dir { None } else { file_language(&name) };
            if let Some(l) = lang {
                let stat = stats.entry(l).or_insert((0, 0));
                stat.0 += 1;
                stat.1 += size;
            }

            let rel = path
                .strip_prefix(root)
                .map(|p| p.to_string_lossy().replace('\\', "/"))
                .unwrap_or_default();

            let node = FileNode {
                name,
                relative_path: rel,
                is_dir,
                language: lang.map(|s| s.to_string()),
                size,
                children: Vec::new(),
            };

            // Complete deeper frames, then attach this node.
            while stack.len() > depth {
                let done = stack.pop().unwrap();
                if let Some(parent) = stack.last_mut() {
                    if parent.children.len() >= MAX_CHILDREN_PER_DIR {
                        overflow.push(done);
                    } else {
                        parent.children.push(done);
                    }
                }
            }

            count += 1;
            if count >= MAX_FILES {
                truncated = true;
                return;
            }
            stack.push(node);
        });

    // Drain remaining frames bottom-up.
    while stack.len() > 1 {
        let done = stack.pop().unwrap();
        if let Some(parent) = stack.last_mut() {
            parent.children.push(done);
        }
    }

    if !overflow.is_empty() {
        truncated = true;
    }

    let mut tree = stack.remove(0);

    fn collapse(node: &mut FileNode) -> (usize, u64) {
        if node.is_dir {
            let mut files = 0usize;
            let mut bytes = 0u64;
            for child in &mut node.children {
                let (f, b) = collapse(child);
                files += f;
                bytes += b;
            }
            finalize_children(&mut node.children);
            node.size = bytes;
            (files, bytes)
        } else {
            (1, node.size)
        }
    }

    let (total_files, total_size) = collapse(&mut tree);

    let mut languages: Vec<LangStat> = stats
        .into_iter()
        .map(|(language, (files, bytes))| LangStat {
            language: language.to_string(),
            files,
            bytes,
        })
        .collect();
    languages.sort_by(|a, b| b.bytes.cmp(&a.bytes).then(b.files.cmp(&a.files)));

    Ok(ScanResult {
        root_name,
        tree,
        total_files,
        total_size,
        truncated,
        languages,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_language() {
        assert_eq!(extension_language("rs"), Some("Rust"));
        assert_eq!(extension_language("TSX"), Some("TSX"));
        assert_eq!(extension_language("unknownext"), None);
    }

    #[test]
    fn test_file_language_special_names() {
        assert_eq!(file_language("Dockerfile"), Some("Dockerfile"));
        assert_eq!(file_language("Makefile"), Some("Makefile"));
        assert_eq!(file_language("main.py"), Some("Python"));
    }

    #[test]
    fn test_scan_project_builds_tree() {
        let base = std::env::temp_dir().join(format!("cs_scan_test_{}", std::process::id()));
        let src = base.join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("main.rs"), "fn main() {}").unwrap();
        std::fs::write(base.join("README.md"), "# demo").unwrap();
        // node_modules must be skipped even without .gitignore
        let junk = base.join("node_modules").join("pkg");
        std::fs::create_dir_all(&junk).unwrap();
        std::fs::write(junk.join("index.js"), "//").unwrap();

        let result = scan_project(&base).expect("scan should succeed");
        assert_eq!(result.root_name, base.file_name().unwrap().to_string_lossy());
        assert_eq!(result.total_files, 2);
        assert!(!result.truncated);

        let langs: Vec<&str> = result.languages.iter().map(|l| l.language.as_str()).collect();
        assert!(langs.contains(&"Rust"));
        assert!(langs.contains(&"Markdown"));
        assert!(!langs.contains(&"JavaScript"), "node_modules should be skipped");

        std::fs::remove_dir_all(&base).ok();
    }

    #[test]
    fn test_scan_project_rejects_file() {
        let f = std::env::temp_dir().join("cs_not_a_dir.txt");
        std::fs::write(&f, "x").unwrap();
        assert!(scan_project(&f).is_err());
        std::fs::remove_file(&f).ok();
    }
}
