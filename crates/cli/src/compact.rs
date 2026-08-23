use code_superman_core::depgraph::DepGraphData;
use code_superman_core::scanner::ScanResult;
use code_superman_core::static_analysis::StaticReport;
use code_superman_core::symbols::FileSymbols;

/// 理解强度：控制报告详略与输出上限。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Strength {
    Brief,
    #[default]
    Standard,
    Detailed,
}

impl Strength {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "brief" => Some(Self::Brief),
            "standard" => Some(Self::Standard),
            "detailed" => Some(Self::Detailed),
            _ => None,
        }
    }

    pub fn max_chars(self) -> usize {
        match self {
            Self::Brief => 3_000,
            Self::Standard => 20_000,
            Self::Detailed => 50_000,
        }
    }

    /// 度量表行数上限。
    fn metric_rows(self) -> usize {
        match self {
            Self::Brief => 0,
            Self::Standard => 20,
            Self::Detailed => 1000,
        }
    }

    /// 核心文件（依赖图节点）行数上限。
    fn core_rows(self) -> usize {
        match self {
            Self::Brief => 0,
            Self::Standard => 15,
            Self::Detailed => 300,
        }
    }

    /// 是否包含依赖边列表。
    fn include_edges(self) -> bool {
        self == Self::Detailed
    }

    /// 是否包含目录树。
    fn include_tree(self) -> bool {
        self != Self::Brief
    }
}

pub fn truncate(text: String, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text;
    }
    let cut: String = text.chars().take(max_chars).collect();
    format!(
        "{}\n\n---\n[输出已截断：原始输出约 {} 字符，超过上限 {}。可改用更高强度（detailed）或缩小分析范围。]",
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

/// 合并渲染：扫描 + 静态分析 +（standard/detailed）依赖图。
pub fn render_analyze(
    scan: &ScanResult,
    report: &StaticReport,
    dep_graph: Option<&DepGraphData>,
    strength: Strength,
) -> String {
    let mut out = String::new();
    out.push_str(&format!("# 项目分析：{}\n\n", scan.root_name));
    out.push_str(&format!(
        "- 文件总数：{}（{}）\n- 代码文件：{}\n- 总行数：{}\n- TODO/FIXME：{}\n\n",
        scan.total_files,
        human_size(scan.total_size),
        report.total_code_files,
        report.total_lines,
        report.total_todos,
    ));

    // 语言占比
    if !scan.languages.is_empty() {
        out.push_str("## 语言占比\n\n| 语言 | 文件数 | 大小 |\n|---|---|---|\n");
        for l in &scan.languages {
            out.push_str(&format!("| {} | {} | {} |\n", l.language, l.files, human_size(l.bytes)));
        }
        out.push('\n');
    }

    // 目录树（brief 不含）
    if strength.include_tree() {
        out.push_str("## 目录结构（深度 2）\n\n```\n");
        render_tree(&scan.tree, 0, 2, &mut out);
        out.push_str("```\n\n");
    }

    // 技术栈
    if !report.tech_stack.is_empty() {
        out.push_str("## 技术栈\n\n| 技术 | 类别 | 来源 |\n|---|---|---|\n");
        for t in &report.tech_stack {
            out.push_str(&format!("| {} | {} | {} |\n", t.name, t.category, t.source));
        }
        out.push('\n');
    }

    // 入口点
    if !report.entry_points.is_empty() {
        out.push_str("## 入口点\n\n");
        for e in &report.entry_points {
            out.push_str(&format!("- `{}` — {}\n", e.relative_path, e.reason));
        }
        out.push('\n');
    }

    // 度量（brief 不含）
    let rows = strength.metric_rows();
    if rows > 0 && !report.metrics.is_empty() {
        out.push_str(&format!(
            "## 最大文件（Top {}，按行数）\n\n| 文件 | 语言 | 行数 | 有效行 | TODO |\n|---|---|---|---|---|\n",
            rows.min(report.metrics.len())
        ));
        for m in report.metrics.iter().take(rows) {
            out.push_str(&format!(
                "| `{}` | {} | {} | {} | {} |\n",
                m.relative_path, m.language, m.lines, m.code_lines, m.todos
            ));
        }
        out.push('\n');
    }

    // 依赖图（standard/detailed）
    if let Some(g) = dep_graph {
        let core = strength.core_rows();
        if core > 0 {
            let mut nodes = g.nodes.clone();
            nodes.sort_by(|a, b| (b.in_degree + b.out_degree).cmp(&(a.in_degree + a.out_degree)));
            out.push_str(&format!(
                "## 核心文件（按连接度 Top {}）\n\n| 文件 | 语言 | 入度 | 出度 |\n|---|---|---|---|\n",
                core.min(nodes.len())
            ));
            for n in nodes.iter().take(core) {
                out.push_str(&format!(
                    "| `{}` | {} | {} | {} |\n",
                    n.id, n.language, n.in_degree, n.out_degree
                ));
            }
            out.push('\n');
        }
        if strength.include_edges() {
            out.push_str(&format!("## 依赖边（{} 条）\n\n```\n", g.edges.len()));
            for e in &g.edges {
                out.push_str(&format!("{} -> {}\n", e.from, e.to));
            }
            out.push_str("```\n");
        } else {
            out.push_str(&format!(
                "> 依赖边共 {} 条，detailed 强度可查看全量边列表。\n",
                g.edges_resolved
            ));
        }
    }

    // 警告
    if !report.warnings.is_empty() {
        out.push_str("## 警告\n\n");
        for w in &report.warnings {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strength_parse() {
        assert_eq!(Strength::parse("brief"), Some(Strength::Brief));
        assert_eq!(Strength::parse("DETAILED"), Some(Strength::Detailed));
        assert_eq!(Strength::parse("nope"), None);
        assert_eq!(Strength::default(), Strength::Standard);
    }

    #[test]
    fn test_strength_presets() {
        assert_eq!(Strength::Brief.max_chars(), 3000);
        assert_eq!(Strength::Detailed.max_chars(), 50000);
        assert!(!Strength::Brief.include_edges());
        assert!(Strength::Detailed.include_edges());
        assert!(!Strength::Brief.include_tree());
    }
}
