use code_superman_core::depgraph::DepGraphData;
use code_superman_core::scanner::ScanResult;
use code_superman_core::static_analysis::StaticReport;
use code_superman_core::symbols::FileSymbols;

/// Default output cap (chars) before truncation.
pub const DEFAULT_MAX_CHARS: usize = 20_000;

pub fn truncate(text: String, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text;
    }
    let cut: String = text.chars().take(max_chars).collect();
    format!(
        "{}\n\n---\n[输出已截断：原始输出约 {} 字符，超过上限 {}。请用更窄的参数（如 --max-nodes、--top）缩小范围后重试。]",
        cut,
        text.chars().count(),
        max_chars
    )
}

fn human_size(bytes: u64) -> String {
    if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

pub fn render_scan(scan: &ScanResult, depth: usize) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "# 项目扫描：{}\n\n- 文件总数：{}（{}）\n- 截断：{}\n\n## 语言占比\n\n| 语言 | 文件数 | 大小 |\n|---|---|---|\n",
        scan.root_name,
        scan.total_files,
        human_size(scan.total_size),
        if scan.truncated { "是（文件过多）" } else { "否" },
    ));
    for l in &scan.languages {
        out.push_str(&format!("| {} | {} | {} |\n", l.language, l.files, human_size(l.bytes)));
    }
    if depth > 0 {
        out.push_str("\n## 目录结构\n\n```\n");
        render_tree(&scan.tree, 0, depth, &mut out);
        out.push_str("```\n");
    }
    out
}

fn render_tree(node: &code_superman_core::scanner::FileNode, level: usize, max_depth: usize, out: &mut String) {
    if level > max_depth {
        return;
    }
    let indent = "  ".repeat(level);
    if node.is_dir {
        out.push_str(&format!("{}{}/\n", indent, node.name));
        for c in node.children.iter().take(50) {
            render_tree(c, level + 1, max_depth, out);
        }
        if node.children.len() > 50 {
            out.push_str(&format!("{}… 共 {} 项\n", indent, node.children.len()));
        }
    } else {
        let lang = node.language.as_deref().unwrap_or("-");
        out.push_str(&format!("{}{} [{}] {}\n", indent, node.name, lang, human_size(node.size)));
    }
}

pub fn render_static_report(r: &StaticReport, top: usize) -> String {
    let mut out = String::new();
    out.push_str(&format!("# 静态分析报告：{}\n\n", r.root_name));
    out.push_str(&format!(
        "- 代码文件总数：{}\n- 总行数：{}\n- TODO/FIXME 总数：{}\n\n",
        r.total_code_files, r.total_lines, r.total_todos
    ));

    if !r.tech_stack.is_empty() {
        out.push_str("## 技术栈\n\n| 技术 | 类别 | 来源 |\n|---|---|---|\n");
        for t in &r.tech_stack {
            out.push_str(&format!("| {} | {} | {} |\n", t.name, t.category, t.source));
        }
        out.push('\n');
    }

    if !r.entry_points.is_empty() {
        out.push_str("## 入口点\n\n");
        for e in &r.entry_points {
            out.push_str(&format!("- `{}` — {}\n", e.relative_path, e.reason));
        }
        out.push('\n');
    }

    if !r.metrics.is_empty() {
        out.push_str(&format!(
            "## 最大文件（Top {}，按行数）\n\n| 文件 | 语言 | 行数 | 有效行 | TODO |\n|---|---|---|---|---|\n",
            top.min(r.metrics.len())
        ));
        for m in r.metrics.iter().take(top) {
            out.push_str(&format!(
                "| `{}` | {} | {} | {} | {} |\n",
                m.relative_path, m.language, m.lines, m.code_lines, m.todos
            ));
        }
        out.push('\n');
    }

    if !r.warnings.is_empty() {
        out.push_str("## 警告\n\n");
        for w in &r.warnings {
            out.push_str(&format!("- ⚠️ {}\n", w));
        }
    }
    out
}

pub fn render_symbols(fs: &FileSymbols) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "# 符号大纲：{}（{}）\n\n",
        fs.relative_path, fs.language
    ));
    if !fs.supported_parse {
        out.push_str("> 该语言暂不支持 tree-sitter 解析。\n");
        return out;
    }
    if fs.symbols.is_empty() && fs.imports.is_empty() {
        out.push_str("（未发现符号或导入）\n");
        return out;
    }
    if !fs.imports.is_empty() {
        out.push_str(&format!("## 导入（{}）\n\n", fs.imports.len()));
        for i in &fs.imports {
            out.push_str(&format!("- `{}`\n", i));
        }
        out.push('\n');
    }
    if !fs.symbols.is_empty() {
        out.push_str(&format!("## 符号（{}）\n\n| 类型 | 名称 | 行范围 | 签名 |\n|---|---|---|---|\n", fs.symbols.len()));
        for s in &fs.symbols {
            let sig = if s.signature.chars().count() > 80 {
                let t: String = s.signature.chars().take(80).collect();
                format!("{}…", t)
            } else {
                s.signature.clone()
            };
            out.push_str(&format!(
                "| {} | `{}` | {}-{} | `{}` |\n",
                s.kind,
                s.name,
                s.start_line,
                s.end_line,
                sig.replace('|', "\\|")
            ));
        }
    }
    out
}

pub fn render_dep_graph(g: &DepGraphData, top: usize) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "# 依赖关系图\n\n- 扫描文件：{}\n- 解析出的依赖边总数：{}\n- 展示节点：{}（按连接度取 top）\n- 边截断：{}\n\n",
        g.files_scanned, g.edges_resolved, g.nodes.len(), g.truncated
    ));

    let mut nodes = g.nodes.clone();
    nodes.sort_by(|a, b| (b.in_degree + b.out_degree).cmp(&(a.in_degree + a.out_degree)));

    out.push_str(&format!("## 核心文件（按连接度 Top {}）\n\n| 文件 | 语言 | 入度 | 出度 |\n|---|---|---|---|\n", top.min(nodes.len())));
    for n in nodes.iter().take(top) {
        out.push_str(&format!(
            "| `{}` | {} | {} | {} |\n",
            n.id, n.language, n.in_degree, n.out_degree
        ));
    }

    out.push_str(&format!("\n## 依赖边（{} 条）\n\n```\n", g.edges.len()));
    for e in &g.edges {
        out.push_str(&format!("{} -> {}\n", e.from, e.to));
    }
    out.push_str("```\n");
    out
}
