use crate::scanner::{FileNode, ScanResult};
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

struct IdGen(usize);

impl IdGen {
    fn next(&mut self) -> String {
        self.0 += 1;
        format!("topic-{}", self.0)
    }
}

/// Max topics to keep the map usable in XMind.
const MAX_TOPICS: usize = 3000;

fn node_to_topic(
    node: &FileNode,
    summaries: &HashMap<String, String>,
    gen: &mut IdGen,
    budget: &mut usize,
) -> Value {
    let mut topic = Map::new();
    topic.insert("id".into(), json!(gen.next()));
    topic.insert("class".into(), json!("topic"));

    let title = if node.is_dir {
        format!("📁 {}", node.name)
    } else {
        match &node.language {
            Some(lang) => format!("📄 {} ({})", node.name, lang),
            None => format!("📄 {}", node.name),
        }
    };
    topic.insert("title".into(), json!(title));

    if !node.is_dir {
        if let Some(summary) = summaries.get(&node.relative_path) {
            topic.insert("notes".into(), json!({ "plain": { "content": summary } }));
        }
    }

    if node.is_dir && *budget > 0 {
        let mut children = Vec::new();
        for child in &node.children {
            if *budget == 0 {
                break;
            }
            *budget -= 1;
            children.push(node_to_topic(child, summaries, gen, budget));
        }
        if !children.is_empty() {
            topic.insert("children".into(), json!({ "attached": children }));
        }
    }

    Value::Object(topic)
}

pub fn export_xmind(
    scan: &ScanResult,
    out_path: &Path,
    summaries: &HashMap<String, String>,
) -> Result<(), String> {
    let mut gen = IdGen(0);
    let mut budget = MAX_TOPICS;

    let root_children: Vec<Value> = scan
        .tree
        .children
        .iter()
        .map(|child| {
            budget -= 1;
            node_to_topic(child, summaries, &mut gen, &mut budget)
        })
        .collect();

    let root_topic = json!({
        "id": gen.next(),
        "class": "topic",
        "title": format!("🦸 {}", scan.root_name),
        "children": { "attached": root_children },
    });

    let content = json!([{
        "id": "sheet-1",
        "class": "sheet",
        "title": scan.root_name,
        "rootTopic": root_topic,
    }]);

    let metadata = json!({
        "creator": { "name": "Code Superman", "version": "0.1.0" },
    });

    // XMind 2020+/ZEN 格式要求 manifest.json 列出包内文件条目，否则报"not a valid XMind File"。
    let manifest = json!({
        "file-entries": {
            "content.json": {},
            "metadata.json": {},
        },
    });

    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let file = std::fs::File::create(out_path).map_err(|e| e.to_string())?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default();

    zip.start_file("content.json", options)
        .map_err(|e| e.to_string())?;
    let content_str =
        serde_json::to_string_pretty(&content).map_err(|e| e.to_string())?;
    zip.write_all(content_str.as_bytes()).map_err(|e| e.to_string())?;

    zip.start_file("metadata.json", options)
        .map_err(|e| e.to_string())?;
    let meta_str = serde_json::to_string_pretty(&metadata).map_err(|e| e.to_string())?;
    zip.write_all(meta_str.as_bytes()).map_err(|e| e.to_string())?;

    zip.start_file("manifest.json", options)
        .map_err(|e| e.to_string())?;
    let manifest_str =
        serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())?;
    zip.write_all(manifest_str.as_bytes()).map_err(|e| e.to_string())?;

    zip.finish().map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::LangStat;

    #[test]
    fn test_export_xmind_creates_valid_zip() {
        let tree = FileNode {
            name: "demo".into(),
            relative_path: String::new(),
            is_dir: true,
            language: None,
            size: 0,
            children: vec![FileNode {
                name: "main.rs".into(),
                relative_path: "main.rs".into(),
                is_dir: false,
                language: Some("Rust".into()),
                size: 12,
                children: vec![],
            }],
        };
        let scan = ScanResult {
            root_name: "demo".into(),
            tree,
            total_files: 1,
            total_size: 12,
            truncated: false,
            languages: vec![LangStat { language: "Rust".into(), files: 1, bytes: 12 }],
        };

        let out = std::env::temp_dir().join("cs_test_out.xmind");
        export_xmind(&scan, &out, &HashMap::new()).expect("export should succeed");

        let f = std::fs::File::open(&out).unwrap();
        let mut archive = zip::ZipArchive::new(f).unwrap();
        assert_eq!(archive.len(), 3);
        let content = archive.by_name("content.json").unwrap();
        let text: String = std::io::Read::bytes(content).map(|b| b.unwrap() as char).collect();
        assert!(text.contains("main.rs"));
        assert!(text.contains("Rust"));

        let manifest = archive.by_name("manifest.json").unwrap();
        let mtext: String = std::io::Read::bytes(manifest).map(|b| b.unwrap() as char).collect();
        assert!(mtext.contains("content.json"));
        assert!(mtext.contains("file-entries"));

        std::fs::remove_file(&out).ok();
    }

    #[test]
    fn test_summary_attached_as_note() {
        let file = FileNode {
            name: "a.py".into(),
            relative_path: "src/a.py".into(),
            is_dir: false,
            language: Some("Python".into()),
            size: 5,
            children: vec![],
        };
        let scan = ScanResult {
            root_name: "p".into(),
            tree: FileNode {
                name: "p".into(),
                relative_path: String::new(),
                is_dir: true,
                language: None,
                size: 5,
                children: vec![file],
            },
            total_files: 1,
            total_size: 5,
            truncated: false,
            languages: vec![],
        };
        let mut summaries = HashMap::new();
        summaries.insert("src/a.py".to_string(), "入口文件".to_string());

        let out = std::env::temp_dir().join("cs_test_note.xmind");
        export_xmind(&scan, &out, &summaries).unwrap();

        let f = std::fs::File::open(&out).unwrap();
        let mut archive = zip::ZipArchive::new(f).unwrap();
        use std::io::Read;
        let mut text = String::new();
        archive.by_name("content.json").unwrap().read_to_string(&mut text).unwrap();
        assert!(text.contains("入口文件"));
        assert!(text.contains("notes"));
        std::fs::remove_file(&out).ok();
    }
}
