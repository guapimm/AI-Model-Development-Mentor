use crate::compact;
use code_superman_core::{depgraph, scanner, static_analysis, symbols, xmind};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{ErrorData, ServerHandler, tool, tool_handler, tool_router};
use schemars::JsonSchema;
use std::path::PathBuf;

const DEFAULT_TOP: usize = 20;

fn err(e: String) -> ErrorData {
    ErrorData::internal_error(e, None)
}

fn truncate_opt(s: String, max_chars: Option<u32>) -> String {
    compact::truncate(s, max_chars.unwrap_or(compact::DEFAULT_MAX_CHARS as u32) as usize)
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct AnalyzeParams {
    /// 项目根目录的绝对路径
    pub path: String,
    /// 最大文件表格行数（默认 20）
    #[serde(default)]
    pub top: Option<u32>,
    /// 输出字符上限，超出截断（默认 20000）
    #[serde(default)]
    pub max_chars: Option<u32>,
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct ScanParams {
    /// 项目根目录的绝对路径
    pub path: String,
    /// 目录树展开深度（默认 2）
    #[serde(default)]
    pub depth: Option<u32>,
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct SymbolsParams {
    /// 项目根目录的绝对路径
    pub path: String,
    /// 相对项目根的文件路径
    pub file: String,
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct DepGraphParams {
    /// 项目根目录的绝对路径
    pub path: String,
    /// 核心文件展示数量，按连接度排序（默认 30）
    #[serde(default)]
    pub top: Option<u32>,
    /// 输出字符上限（默认 20000）
    #[serde(default)]
    pub max_chars: Option<u32>,
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct XmindParams {
    /// 项目根目录的绝对路径
    pub path: String,
    /// 输出 .xmind 文件路径（默认 <path>/architecture.xmind）
    #[serde(default)]
    pub out: Option<String>,
}

#[derive(Clone, Default)]
pub struct CodeSupermanServer;

#[tool_router]
impl CodeSupermanServer {
    #[tool(
        description = "对代码库做静态分析：技术栈识别、入口点定位、行数/TODO 度量、超大文件警告。返回 Markdown 报告。"
    )]
    fn analyze_static(&self, Parameters(p): Parameters<AnalyzeParams>) -> Result<String, ErrorData> {
        let report = static_analysis::run_static_analysis(&PathBuf::from(&p.path)).map_err(err)?;
        Ok(truncate_opt(
            compact::render_static_report(&report, p.top.unwrap_or(DEFAULT_TOP as u32) as usize),
            p.max_chars,
        ))
    }

    #[tool(description = "扫描项目目录：语言占比统计与目录结构概览（尊重 .gitignore）。")]
    fn scan_project(&self, Parameters(p): Parameters<ScanParams>) -> Result<String, ErrorData> {
        let scan = scanner::scan_project(&PathBuf::from(&p.path)).map_err(err)?;
        Ok(compact::render_scan(&scan, p.depth.unwrap_or(2) as usize))
    }

    #[tool(
        description = "解析单个代码文件的符号大纲：函数/类/结构体清单及 import 列表（tree-sitter，支持 Rust/Python/TS/JS/Go/Java/C/C++/C#/PHP/Ruby）。"
    )]
    fn get_file_symbols(&self, Parameters(p): Parameters<SymbolsParams>) -> Result<String, ErrorData> {
        let rel = p.file.replace('\\', "/");
        let ext = rel.rsplit('.').next().unwrap_or("");
        let lang = scanner::extension_language(ext)
            .ok_or_else(|| format!("无法从扩展名 .{} 识别语言", ext))
            .map_err(err)?;
        let fs = symbols::parse_file(&PathBuf::from(&p.path), &rel, lang).map_err(err)?;
        Ok(compact::truncate(
            compact::render_symbols(&fs),
            compact::DEFAULT_MAX_CHARS,
        ))
    }

    #[tool(
        description = "解析项目 import 关系，构建文件级依赖图：核心文件（按入度/出度排序）+ 依赖边列表。用于找出项目核心模块。"
    )]
    fn get_dependency_graph(&self, Parameters(p): Parameters<DepGraphParams>) -> Result<String, ErrorData> {
        let g = depgraph::build_dependency_graph(&PathBuf::from(&p.path)).map_err(err)?;
        Ok(truncate_opt(
            compact::render_dep_graph(&g, p.top.unwrap_or(30) as usize),
            p.max_chars,
        ))
    }

    #[tool(description = "把项目架构导出为 .xmind 思维导图文件。")]
    fn export_xmind(&self, Parameters(p): Parameters<XmindParams>) -> Result<String, ErrorData> {
        let root = PathBuf::from(&p.path);
        let out = match &p.out {
            Some(o) => PathBuf::from(o),
            None => root.join("architecture.xmind"),
        };
        let scan = scanner::scan_project(&root).map_err(err)?;
        xmind::export_xmind(&scan, &out, &Default::default()).map_err(err)?;
        Ok(format!("已导出到 {}", out.display()))
    }
}

#[tool_handler(
    name = "code-superman",
    version = "0.2.0",
    instructions = "Code Superman：代码库静态理解工具。典型流程：先 scan_project 看语言构成，再 analyze_static 获取技术栈报告，get_dependency_graph 找核心文件，get_file_symbols 查看单个文件大纲。"
)]
impl ServerHandler for CodeSupermanServer {}
