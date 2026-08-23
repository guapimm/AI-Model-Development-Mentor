use crate::compact::{self, Strength};
use code_superman_core::xmind::{XmindInput};
use code_superman_core::{depgraph, scanner, static_analysis, symbols, xmind};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{ErrorData, ServerHandler, tool, tool_handler, tool_router};
use schemars::JsonSchema;
use std::collections::HashMap;
use std::path::PathBuf;

fn err(e: String) -> ErrorData {
    ErrorData::internal_error(e, None)
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct AnalyzeParams {
    /// 项目根目录的绝对路径
    pub path: String,
    /// 理解强度：brief=语言/技术栈/入口点；standard=加度量Top20与核心文件（默认）；detailed=全量依赖边
    #[serde(default)]
    pub strength: Option<String>,
    /// 指定此路径则同时导出 .xmind 思维导图（含总览/入口点/核心模块/目录树分支）
    #[serde(default)]
    pub xmind_out: Option<String>,
    /// 相对路径 -> 该文件职责一句话摘要，写入思维导图对应节点备注（可选；建议先分析再总结传入）
    #[serde(default)]
    pub file_summaries: Option<HashMap<String, String>>,
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct SymbolsParams {
    /// 项目根目录的绝对路径
    pub path: String,
    /// 相对项目根的文件路径
    pub file: String,
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct XmindParams {
    /// 项目根目录的绝对路径
    pub path: String,
    /// 输出 .xmind 文件路径（默认 <path>/architecture.xmind）
    #[serde(default)]
    pub out: Option<String>,
    /// 相对路径 -> 该文件职责一句话摘要，写入思维导图对应节点备注（可选）
    #[serde(default)]
    pub file_summaries: Option<HashMap<String, String>>,
}

fn do_export_xmind(
    root: &PathBuf,
    out: &PathBuf,
    report: &static_analysis::StaticReport,
    dep_graph: Option<&depgraph::DepGraphData>,
    summaries: &HashMap<String, String>,
) -> Result<(), ErrorData> {
    let scan = scanner::scan_project(root).map_err(err)?;
    let input = XmindInput {
        root,
        scan: &scan,
        report,
        dep_graph,
        summaries,
    };
    xmind::export_xmind(&input, out).map_err(err)
}

#[derive(Clone, Default)]
pub struct CodeSupermanServer;

#[tool_router]
impl CodeSupermanServer {
#[tool(
    description = "分析整个项目：语言占比、目录结构、技术栈识别、入口点定位、Token 用量估算、代码度量、核心模块（依赖图）。strength 控制详略：brief 最简（含 Token 估算，速度快），detailed 含全量依赖边。"
)]
fn analyze(&self, Parameters(p): Parameters<AnalyzeParams>) -> Result<String, ErrorData> {
        let root = PathBuf::from(&p.path);
        let strength = match &p.strength {
            Some(s) => Strength::parse(s)
                .ok_or_else(|| format!("无效的 strength：{}（可选 brief/standard/detailed）", s))
                .map_err(err)?,
            None => Strength::default(),
        };

        let scan = scanner::scan_project(&root).map_err(err)?;
        let report = static_analysis::run_static_analysis(&root).map_err(err)?;

        let dep_graph = if strength == Strength::Brief {
            None
        } else {
            Some(depgraph::build_dependency_graph(&root).map_err(err)?)
        };

        let mut md = compact::render_analyze(&scan, &report, dep_graph.as_ref(), strength);

        if let Some(xmind_path) = &p.xmind_out {
            let out = PathBuf::from(xmind_path);
            let empty = HashMap::new();
            let summaries = p.file_summaries.as_ref().unwrap_or(&empty);
            do_export_xmind(&root, &out, &report, dep_graph.as_ref(), summaries)?;
            md.push_str(&format!(
                "\n---\n\n📦 已导出架构思维导图（含总览/技术栈/入口点/核心模块/文件职责备注）：{}\n",
                out.display()
            ));
        }

        Ok(compact::truncate(md, strength.max_chars()))
    }

    #[tool(
        description = "解析单个代码文件的符号大纲：函数/类/结构体清单及 import 列表（tree-sitter，支持 Rust/Python/TS/JS/Go/Java/C/C++/C#/PHP/Ruby）。"
    )]
    fn get_file_symbols(
        &self,
        Parameters(p): Parameters<SymbolsParams>,
    ) -> Result<String, ErrorData> {
        let rel = p.file.replace('\\', "/");
        let ext = rel.rsplit('.').next().unwrap_or("");
        let lang = scanner::extension_language(ext)
            .ok_or_else(|| format!("无法从扩展名 .{} 识别语言", ext))
            .map_err(err)?;
        let fs = symbols::parse_file(&PathBuf::from(&p.path), &rel, lang).map_err(err)?;
        Ok(compact::truncate(
            compact::render_symbols(&fs),
            compact::Strength::Detailed.max_chars(),
        ))
    }

    #[tool(
        description = "把项目架构导出为 .xmind 思维导图：包含总览（技术栈/语言占比/Token估算）、入口点、核心模块、警告与完整目录树（节点标注语言/行数/被依赖数/符号构成）。导出前应先理解各关键文件职责，经 file_summaries 传入摘要，否则导图缺少「每个文件在做什么」的说明。"
    )]
    fn export_xmind(&self, Parameters(p): Parameters<XmindParams>) -> Result<String, ErrorData> {
        let root = PathBuf::from(&p.path);
        let out = match &p.out {
            Some(o) => PathBuf::from(o),
            None => root.join("architecture.xmind"),
        };
        let report = static_analysis::run_static_analysis(&root).map_err(err)?;
        let dep_graph = depgraph::build_dependency_graph(&root).map_err(err)?;
        let empty = HashMap::new();
        let summaries = p.file_summaries.as_ref().unwrap_or(&empty);
        do_export_xmind(&root, &out, &report, Some(&dep_graph), summaries)?;
        Ok(format!(
            "已导出架构思维导图到 {}（可传 file_summaries 为文件节点附加职责说明）",
            out.display()
        ))
    }
}

#[tool_handler(
    name = "code-superman",
    version = "0.2.0",
    instructions = "Code Superman：代码库理解工具。\n\n【两段式流程】当用户未指定强度时：先以 strength=brief 调用 analyze 获取 Token 估算与核心文件清单（秒级），然后用提问能力向用户展示估算结果并让其选择强度——选项应包含成本信息：⚡简要(报告~750 tokens)、⚖️标准(~5K tokens，精读核心Top15约X K)、🔬详尽(~12.5K tokens，全量依赖边)，其中 X 取自 brief 报告的核心 Top15 估算值。用户已明确指定强度时跳过询问直接分析。\n\n【导出思维导图】用户需要 xmind 时，必须先理解各关键文件的职责，为至少核心文件各写一句「这个文件在做什么」摘要，经 file_summaries 参数传入（会显示在导图节点备注中）；未传摘要则导图缺少职责说明。是否导出 xmind 也应在弹窗中一并询问用户。"
)]
impl ServerHandler for CodeSupermanServer {}
