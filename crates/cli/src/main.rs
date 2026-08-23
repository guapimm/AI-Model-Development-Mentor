mod compact;
mod mcp;

use clap::{Parser, Subcommand};
use code_superman_core::{depgraph, scanner, static_analysis, symbols, xmind};
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
    /// 扫描项目：语言占比 + 目录结构
    Scan {
        /// 项目根目录
        path: String,
        /// 目录树展开深度
        #[arg(long, default_value_t = 2)]
        depth: usize,
    },
    /// 静态分析：技术栈、入口点、度量、警告
    Analyze {
        /// 项目根目录
        path: String,
        /// 最大文件表格行数
        #[arg(long, default_value_t = 20)]
        top: usize,
        /// 输出字符上限
        #[arg(long, default_value_t = 20000)]
        max_chars: usize,
    },
    /// 单文件符号大纲
    Symbols {
        /// 项目根目录
        path: String,
        /// 相对项目根的文件路径
        file: String,
    },
    /// 依赖关系图
    Deps {
        /// 项目根目录
        path: String,
        /// 核心文件展示数量
        #[arg(long, default_value_t = 30)]
        top: usize,
        /// 输出字符上限
        #[arg(long, default_value_t = 20000)]
        max_chars: usize,
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
        Commands::Scan { path, depth } => {
            run(|p| {
                let scan = scanner::scan_project(&PathBuf::from(p))?;
                Ok(compact::render_scan(&scan, depth))
            }, &path);
        }
        Commands::Analyze {
            path,
            top,
            max_chars,
        } => {
            run(
                |p| {
                    let report = static_analysis::run_static_analysis(&PathBuf::from(p))?;
                    Ok(compact::truncate(
                        compact::render_static_report(&report, top),
                        max_chars,
                    ))
                },
                &path,
            );
        }
        Commands::Symbols { path, file } => {
            let ext = file.rsplit('.').next().unwrap_or("").to_string();
            run(move |p| {
                let lang = scanner::extension_language(&ext)
                    .ok_or_else(|| format!("无法从扩展名 .{} 识别语言", ext))?
                    .to_string();
                let fs = symbols::parse_file(&PathBuf::from(p), &file, &lang)?;
                Ok(compact::render_symbols(&fs))
            }, &path);
        }
        Commands::Deps {
            path,
            top,
            max_chars,
        } => {
            run(|p| {
                let g = depgraph::build_dependency_graph(&PathBuf::from(p))?;
                Ok(compact::truncate(
                    compact::render_dep_graph(&g, top),
                    max_chars,
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

async fn serve() {
    use rmcp::ServiceExt;
    let service = mcp::CodeSupermanServer;
    let transport = (tokio::io::stdin(), tokio::io::stdout());
    let server = service.serve(transport).await.expect("MCP server 启动失败");
    server.waiting().await.expect("MCP server 运行出错");
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
