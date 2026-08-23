use crate::compact::{self, Strength};
use code_superman_core::{depgraph, scanner, static_analysis, symbols, xmind};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{ErrorData, ServerHandler, tool, tool_handler, tool_router};
use schemars::JsonSchema;
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
    /// 指定此路径则同时导出 .xmind 思维导图（如 <path>/architecture.xmind）
    #[serde(default)]
    pub xmind_out: Option<String>,
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
}

#[derive(Clone, Default)]
pub struct CodeSupermanServer;

#[tool_router]
impl CodeSupermanServer {
    #[tool(
        description = "分析整个项目：语言占比、目录结构、技术栈识别、入口点定位、代码度量、核心模块（依赖图）。strength 控制详略：brief 最简，detailed 含全量依赖边。"
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
            xmind::export_xmind(&scan, &out, &Default::default()).map_err(err)?;
            md.push_str(&format!("\n---\n\n📦 已导出思维导图：{}\n", out.display()));
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
    instructions = "Code Superman：代码库理解工具。典型流程：先 analyze 看项目全貌与技术栈（brief 快览 / detailed 找核心模块），再 get_file_symbols 深入关键文件大纲。需要思维导图时在 analyze 里传 xmind_out，或单独调 export_xmind。"
)]
impl ServerHandler for CodeSupermanServer {}
