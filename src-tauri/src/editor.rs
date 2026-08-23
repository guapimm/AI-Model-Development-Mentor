use serde::Serialize;
use std::path::PathBuf;

/// Files above this size open read-only / refuse to load into the editor.
pub const MAX_EDIT_BYTES: u64 = 2_000_000;

#[derive(Serialize)]
pub struct FileContent {
    pub content: String,
    #[serde(rename = "truncated")]
    pub truncated: bool,
}

fn resolve_in_root(root: &str, relative_path: &str) -> Result<PathBuf, String> {
    let clean = relative_path.replace('\\', "/");
    if clean.split('/').any(|seg| seg == "..") {
        return Err("路径越界：不允许包含 ..".to_string());
    }
    let root_abs = std::fs::canonicalize(root).map_err(|e| e.to_string())?;
    let full = root_abs.join(&clean);

    // Target may not exist yet (new file); canonicalize the deepest existing ancestor.
    let mut probe = full.clone();
    while !probe.exists() {
        match probe.parent() {
            Some(p) => probe = p.to_path_buf(),
            None => return Err("无效路径".to_string()),
        }
    }
    let probe_abs = std::fs::canonicalize(&probe).map_err(|e| e.to_string())?;
    if !probe_abs.starts_with(&root_abs) {
        return Err("路径越界：不允许访问项目外的文件".to_string());
    }
    Ok(full)
}

pub fn read_file(root: &str, relative_path: &str) -> Result<FileContent, String> {
    let path = resolve_in_root(root, relative_path)?;
    let meta = std::fs::metadata(&path).map_err(|e| e.to_string())?;
    if meta.len() > MAX_EDIT_BYTES {
        return Err(format!(
            "文件过大（{} KB），编辑器仅支持 {} KB 以内的文件",
            meta.len() / 1024,
            MAX_EDIT_BYTES / 1024
        ));
    }
    let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
    let content = String::from_utf8_lossy(&bytes).to_string();
    Ok(FileContent { content, truncated: false })
}

pub fn save_file(root: &str, relative_path: &str, content: &str) -> Result<(), String> {
    let path = resolve_in_root(root, relative_path)?;
    if content.len() as u64 > MAX_EDIT_BYTES {
        return Err(format!("内容超过大小上限（{} KB）", MAX_EDIT_BYTES / 1024));
    }
    std::fs::write(path, content).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_and_save_roundtrip() {
        let base = std::env::temp_dir().join(format!("cs_edit_{}", std::process::id()));
        std::fs::create_dir_all(base.join("src")).unwrap();
        std::fs::write(base.join("src").join("a.txt"), "hello").unwrap();

        let f = read_file(base.to_str().unwrap(), "src/a.txt").unwrap();
        assert_eq!(f.content, "hello");
        assert!(!f.truncated);

        save_file(base.to_str().unwrap(), "src/a.txt", "world!").unwrap();
        let f2 = read_file(base.to_str().unwrap(), "src/a.txt").unwrap();
        assert_eq!(f2.content, "world!");

        std::fs::remove_dir_all(&base).ok();
    }

    #[test]
    fn test_path_traversal_blocked() {
        let base = std::env::temp_dir().join(format!("cs_edit_t_{}", std::process::id()));
        std::fs::create_dir_all(&base).unwrap();
        assert!(read_file(base.to_str().unwrap(), "../outside.txt").is_err());
        assert!(save_file(base.to_str().unwrap(), "../../x.txt", "x").is_err());
        std::fs::remove_dir_all(&base).ok();
    }
}
