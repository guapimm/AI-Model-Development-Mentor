use crate::depgraph::DepGraphData;
use crate::scanner::{FileNode, ScanResult};
use crate::static_analysis::StaticReport;
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

fn topic(id: &mut IdGen, title: String) -> Map<String, Value> {
    let mut t = Map::new();
    t.insert("id".into(), json!(id.next()));
    t.insert("class".into(), json!("topic"));
    t.insert("title".into(), json!(title));
    t
}

fn attach(map: &mut Map<String, Value>, children: Vec<Value>) {
    if !children.is_empty() {
        map.insert("children".into(), json!({ "attached": children }));
    }
}

fn est_tokens(chars: usize) -> String {
    let k = chars / 4 / 1000;
    if k >= 1 {
        format!("~{}K tokens", k)
    } else {
        format!("~{} tokens", chars / 4)
    }
}

/// XMind 导出的全部输入。
pub struct XmindInput<'a> {
    pub scan: &'a ScanResult,
    pub report: &'a StaticReport,
    pub dep_graph: Option<&'a DepGraphData>,
    /// 相对路径 -> 该文件职责摘要（通常由调用方大模型撰写），写入节点备注。
    pub summaries: &'a HashMap<String, String>,
}

fn overview_branch(input: &XmindInput, gen: &mut IdGen) -> Value {
    let mut root = topic(gen, "📊 总览".into());
    let mut children = Vec::new();

    // 统计
    let mut stat = topic(
        gen,
        format!(
            "📈 统计：{} 个文件 / {} 代码文件 / {} 行 / TODO {}",
            input.scan.total_files,
            input.report.total_code_files,
            input.report.total_lines,
            input.report.total_todos,
        ),
    );
    let core_chars: usize = match input.dep_graph {
        Some(g) => {
            let mut nodes = g.nodes.clone();
            nodes.sort_by(|a, b| (b.in_degree + b.out_degree).cmp(&(a.in_degree + a.out_degree)));
            nodes
                .iter()
                .take(15)
                .filter_map(|n| input.report.metrics.iter().find(|m| m.relative_path == n.id))
                .map(|m| m.chars)
                .sum()
        }
        None => 0,
    };
    if core_chars > 0 {
        stat.insert(
            "title".into(),
            json!(format!(
                "📈 统计：{} 个文件 / {} 代码文件 / {} 行 / TODO {} / 核心 Top15 {}",
                input.scan.total_files,
                input.report.total_code_files,
                input.report.total_lines,
                input.report.total_todos,
                est_tokens(core_chars),
            )),
        );
        children.push(Value::Object(stat));
    } else {
        children.push(Value::Object(stat));
    }

    let tok = topic(
        gen,
        format!(
            "🔢 Token 估算（粗估，4字符/token）：全部代码 {}",
            est_tokens(input.report.total_chars)
        ),
    );
    children.push(Value::Object(tok));

    // 语言占比
    let mut langs = topic(gen, "🗣️ 语言占比".into());
    attach(
        &mut langs,
        input
            .scan
            .languages
            .iter()
            .map(|l| {
                Value::Object(topic(
                    gen,
                    format!("{}：{} 个文件", l.language, l.files),
                ))
            })
            .collect(),
    );
    children.push(Value::Object(langs));

    // 技术栈
    if !input.report.tech_stack.is_empty() {
        let mut tech = topic(gen, "🧰 技术栈".into());
        attach(
            &mut tech,
            input
                .report
                .tech_stack
                .iter()
                .map(|t| {
                    Value::Object(topic(
                        gen,
                        format!("{}（{}）", t.name, t.category),
                    ))
                })
                .collect(),
        );
        children.push(Value::Object(tech));
    }

    attach(&mut root, children);
    Value::Object(root)
}

fn entry_points_branch(input: &XmindInput, gen: &mut IdGen) -> Option<Value> {
    if input.report.entry_points.is_empty() {
        return None;
    }
    let mut root = topic(gen, "🚪 入口点".into());
    attach(
        &mut root,
        input
            .report
            .entry_points
            .iter()
            .map(|e| Value::Object(topic(gen, format!("{} — {}", e.relative_path, e.reason))))
            .collect(),
    );
    Some(Value::Object(root))
}

fn core_modules_branch(input: &XmindInput, gen: &mut IdGen) -> Option<Value> {
    let g = input.dep_graph?;
    let mut nodes = g.nodes.clone();
    nodes.sort_by(|a, b| (b.in_degree + b.out_degree).cmp(&(a.in_degree + a.out_degree)));

    let mut root = topic(gen, "⭐ 核心模块（按连接度 Top 15）".into());
    attach(
        &mut root,
        nodes
            .iter()
            .take(15)
            .map(|n| {
                Value::Object(topic(
                    gen,
                    format!("{} [入度{} 出度{}]", n.id, n.in_degree, n.out_degree),
                ))
            })
            .collect(),
    );
    Some(Value::Object(root))
}

fn warnings_branch(input: &XmindInput, gen: &mut IdGen) -> Option<Value> {
    if input.report.warnings.is_empty() {
        return None;
    }
    let mut root = topic(gen, "⚠️ 警告".into());
    attach(
        &mut root,
        input
            .report
            .warnings
            .iter()
            .map(|w| Value::Object(topic(gen, w.clone())))
            .collect(),
    );
    Some(Value::Object(root))
}

/// 文件节点的静态标注：语言 · 行数 · TODO · 入度。
fn file_annotation(node: &FileNode, input: &XmindInput) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(lang) = &node.language {
        parts.push(lang.to_string());
    }
    if let Some(m) = input
        .report
        .metrics
        .iter()
        .find(|m| m.relative_path == node.relative_path)
    {
        parts.push(format!("{}行", m.lines));
        if m.todos > 0 {
            parts.push(format!("TODO{}", m.todos));
        }
    }
    if let Some(g) = input.dep_graph {
        if let Some(n) = g.nodes.iter().find(|n| n.id == node.relative_path) {
            if n.in_degree > 0 || n.out_degree > 0 {
                parts.push(format!("入度{}", n.in_degree));
            }
        }
    }
    if parts.is_empty() {
        String::new()
    } else {
        format!(" [{}]", parts.join(" · "))
    }
}

fn tree_branch(input: &XmindInput, gen: &mut IdGen, budget: &mut usize) -> Value {
    fn node_to_topic(
        node: &FileNode,
        input: &XmindInput,
        gen: &mut IdGen,
        budget: &mut usize,
    ) -> Value {
        *budget = budget.saturating_sub(1);
        let mut t = if node.is_dir {
            topic(gen, format!("📁 {}", node.name))
        } else {
            let ann = file_annotation(node, input);
            topic(gen, format!("📄 {}{}", node.name, ann))
        };

        if !node.is_dir {
            if let Some(summary) = input.summaries.get(&node.relative_path) {
                t.insert("notes".into(), json!({ "plain": { "content": summary } }));
            }
        }

        if node.is_dir && *budget > 0 {
            let mut children = Vec::new();
            for child in &node.children {
                if *budget == 0 {
                    break;
                }
                children.push(node_to_topic(child, input, gen, budget));
            }
            attach(&mut t, children);
        }

        Value::Object(t)
    }

    let mut root = topic(gen, "📁 目录结构".into());
    let mut children = Vec::new();
    for child in &input.scan.tree.children {
        if *budget == 0 {
            break;
        }
        children.push(node_to_topic(child, input, gen, budget));
    }
    attach(&mut root, children);
    Value::Object(root)
}

pub fn export_xmind(input: &XmindInput, out_path: &Path) -> Result<(), String> {
    let mut gen = IdGen(0);

    let mut branch_children: Vec<Value> = vec![overview_branch(input, &mut gen)];
    for optional in [
        entry_points_branch(input, &mut gen),
        core_modules_branch(input, &mut gen),
        warnings_branch(input, &mut gen),
    ]
    .into_iter()
    .flatten()
    {
        branch_children.push(optional);
    }
    let mut budget = MAX_TOPICS.saturating_sub(branch_children.len() * 50);
    let tree = tree_branch(input, &mut gen, &mut budget);
    branch_children.push(tree);

    let root_topic = json!({
        "id": gen.next(),
        "class": "topic",
        "title": format!("🦸 {}", input.scan.root_name),
        "children": { "attached": branch_children },
    });

    let content = json!([{
        "id": "sheet-1",
        "class": "sheet",
        "title": input.scan.root_name,
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
    use crate::depgraph::{DepGraphData, DepGraphNode};
    use crate::scanner::LangStat;
    use crate::static_analysis::{FileMetric, TechStackItem};

    fn fixture() -> (ScanResult, StaticReport, DepGraphData) {
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
        let report = StaticReport {
            root_name: "demo".into(),
            tech_stack: vec![TechStackItem {
                name: "clap".into(),
                category: "CLI 框架".into(),
                source: "Cargo.toml".into(),
            }],
            entry_points: vec![crate::static_analysis::EntryPoint {
                relative_path: "main.rs".into(),
                reason: "程序入口 main".into(),
            }],
            metrics: vec![FileMetric {
                relative_path: "main.rs".into(),
                language: "Rust".into(),
                lines: 10,
                code_lines: 8,
                todos: 1,
                chars: 400,
            }],
            total_code_files: 1,
            total_lines: 10,
            total_todos: 1,
            total_chars: 400,
            warnings: vec![],
        };
        let graph = DepGraphData {
            nodes: vec![DepGraphNode {
                id: "main.rs".into(),
                language: "Rust".into(),
                in_degree: 2,
                out_degree: 0,
            }],
            edges: vec![],
            files_scanned: 1,
            edges_resolved: 0,
            truncated: false,
        };
        (scan, report, graph)
    }

    #[test]
    fn test_export_contains_architecture_branches_and_manifest() {
        let (scan, report, graph) = fixture();
        let input = XmindInput {
            scan: &scan,
            report: &report,
            dep_graph: Some(&graph),
            summaries: &HashMap::new(),
        };
        let out = std::env::temp_dir().join("cs_test_arch.xmind");
        export_xmind(&input, &out).expect("export should succeed");

        let f = std::fs::File::open(&out).unwrap();
        let mut archive = zip::ZipArchive::new(f).unwrap();
        assert_eq!(archive.len(), 3);
        use std::io::Read;
        let mut text = String::new();
        archive.by_name("content.json").unwrap().read_to_string(&mut text).unwrap();

        assert!(text.contains("📊 总览"));
        assert!(text.contains("🚪 入口点"));
        assert!(text.contains("⭐ 核心模块"));
        assert!(text.contains("Token 估算"));
        assert!(text.contains("clap"));
        assert!(text.contains("[Rust · 10行 · TODO1 · 入度2]"));

        let manifest = archive.by_name("manifest.json").unwrap();
        let mtext: String = std::io::Read::bytes(manifest).map(|b| b.unwrap() as char).collect();
        assert!(mtext.contains("file-entries"));

        std::fs::remove_file(&out).ok();
    }

    #[test]
    fn test_summary_attached_as_note() {
        let (scan, report, graph) = fixture();
        let mut summaries = HashMap::new();
        summaries.insert("main.rs".to_string(), "程序入口".to_string());
        let input = XmindInput {
            scan: &scan,
            report: &report,
            dep_graph: Some(&graph),
            summaries: &summaries,
        };
        let out = std::env::temp_dir().join("cs_test_note.xmind");
        export_xmind(&input, &out).unwrap();

        let f = std::fs::File::open(&out).unwrap();
        let mut archive = zip::ZipArchive::new(f).unwrap();
        use std::io::Read;
        let mut text = String::new();
        archive.by_name("content.json").unwrap().read_to_string(&mut text).unwrap();
        assert!(text.contains("程序入口"));
        assert!(text.contains("notes"));
        std::fs::remove_file(&out).ok();
    }
}
