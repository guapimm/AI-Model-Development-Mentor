mod compact;
mod mcp;

use clap::{Parser, Subcommand};
use code_superman_core::{scanner, static_analysis, symbols, xmind};
use compact::Strength;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "code-superman",
    version,
    about = "Code Superman：代码库理解工具（MCP server + CLI）"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 以 stdio MCP server 模式运行
    Serve,
    /// 分析项目：语言/技术栈/入口点/度量/核心模块
    Analyze {
        /// 项目根目录
        path: String,
        /// 理解强度：brief | standard | detailed（默认 standard）
        #[arg(long)]
        detail: Option<String>,
        /// 同时导出 .xmind 到指定路径
        #[arg(long)]
        xmind: Option<String>,
    },
    /// 单文件符号大纲
    Symbols {
        /// 项目根目录
        path: String,
        /// 相对项目根的文件路径
        file: String,
    },
    /// 导出 XMind 架构思维导图
    Xmind {
        /// 项目根目录
        path: String,
        /// 输出文件路径（默认 <path>/architecture.xmind）
        #[arg(short, long)]
        out: Option<String>,
    },
}

async fn serve() {
    use rmcp::ServiceExt;
    let service = mcp::CodeSupermanServer;
    let transport = (tokio::io::stdin(), tokio::io::stdout());
    let server = service.serve(transport).await.expect("MCP server 启动失败");
    server.waiting().await.expect("MCP server 运行出错");
}

fn parse_strength(s: &Option<String>) -> Strength {
    match s {
        Some(v) => Strength::parse(v).unwrap_or_else(|| {
            eprintln!("错误：无效的强度 {}（可选 brief/standard/detailed）", v);
            std::process::exit(1);
        }),
        None => Strength::default(),
    }
}

fn run_analyze(path: &str, detail: Option<String>, xmind_out: Option<String>) -> Result<String, String> {
    let strength = parse_strength(&detail);
    let root = PathBuf::from(path);
    let scan = scanner::scan_project(&root)?;
    let report = static_analysis::run_static_analysis(&root)?;

    let dep_graph = if strength == Strength::Brief {
        None
    } else {
        Some(code_superman_core::depgraph::build_dependency_graph(&root)?)
    };

    let mut md = compact::render_analyze(&scan, &report, dep_graph.as_ref(), strength);

    if let Some(xp) = xmind_out {
        let out = PathBuf::from(xp);
        xmind::export_xmind(&scan, &out, &Default::default())?;
        md.push_str(&format!("\n---\n\n📦 已导出思维导图：{}\n", out.display()));
    }

    Ok(compact::truncate(md, strength.max_chars()))
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Serve => {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("无法创建 tokio 运行时")
                .block_on(serve());
        }
        Commands::Analyze {
            path,
            detail,
            xmind,
        } => run(|p| run_analyze(p, detail, xmind), &path),
        Commands::Symbols { path, file } => {
            let ext = file.rsplit('.').next().unwrap_or("").to_string();
            run(move |p| {
                let lang = scanner::extension_language(&ext)
                    .ok_or_else(|| format!("无法从扩展名 .{} 识别语言", ext))?
                    .to_string();
                let fs = symbols::parse_file(&PathBuf::from(p), &file, &lang)?;
                Ok(compact::truncate(
                    compact::render_symbols(&fs),
                    compact::Strength::Detailed.max_chars(),
                ))
            }, &path);
        }
        Commands::Xmind { path, out } => {
            run(|p| {
                let out_path = match &out {
                    Some(o) => PathBuf::from(o),
                    None => PathBuf::from(p).join("architecture.xmind"),
                };
                let scan = scanner::scan_project(&PathBuf::from(p))?;
                xmind::export_xmind(&scan, &out_path, &Default::default())?;
                Ok(format!("已导出到 {}", out_path.display()))
            }, &path);
        }
    }
}

fn run<F>(f: F, path: &str)
where
    F: FnOnce(&str) -> Result<String, String>,
{
    match f(path) {
        Ok(out) => println!("{}", out),
        Err(e) => {
            eprintln!("错误：{}", e);
            std::process::exit(1);
        }
    }
}
