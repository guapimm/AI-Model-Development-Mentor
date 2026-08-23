use crate::llm::{chat, ChatMessage};
use crate::scanner::{self, FileNode};
use crate::settings::Settings;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::path::{Path, PathBuf};
use tauri::ipc::Channel;

/// Max source bytes sent per file to the LLM.
const MAX_FILE_BYTES: u64 = 60_000;
/// Default cap on files summarized in one project run.
const DEFAULT_MAX_FILES: usize = 30;

#[derive(Clone, Serialize)]
pub struct SummarizeProgress {
    pub done: usize,
    pub total: usize,
    pub current: String,
    pub phase: String,
}

#[derive(Serialize)]
pub struct ProjectAnalysis {
    pub overview: String,
    #[serde(rename = "fileSummaries")]
    pub file_summaries: Vec<FileSummary>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileSummary {
    #[serde(rename = "relativePath")]
    pub relative_path: String,
    pub summary: String,
}

fn system_prompt() -> ChatMessage {
    ChatMessage {
        role: "system",
        content: "你是资深软件工程师，正在帮助一位对代码库不熟悉的用户理解项目。\
                  用简体中文回答，语言简洁、准确、面向初学者。不要编造代码中不存在的内容。"
            .to_string(),
    }
}

fn read_source(path: &Path) -> Result<(String, bool), String> {
    let mut file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut buf = vec![0u8; MAX_FILE_BYTES as usize + 1];
    let n = file.read(&mut buf).map_err(|e| e.to_string())?;
    let truncated = n as u64 > MAX_FILE_BYTES;
    buf.truncate(n.min(MAX_FILE_BYTES as usize));
    let text = String::from_utf8_lossy(&buf).to_string();
    Ok((text, truncated))
}

fn collect_code_files(node: &FileNode, out: &mut Vec<FileNode>) {
    if node.is_dir {
        for child in &node.children {
            collect_code_files(child, out);
        }
    } else if node.language.is_some() && node.size > 0 && node.name != "Cargo.lock" {
        out.push(node.clone());
    }
}

fn pick_files(tree: &FileNode, max_files: usize) -> Vec<FileNode> {
    let mut files = Vec::new();
    collect_code_files(tree, &mut files);
    // Heuristic: larger files tend to be more central; also prefer code over configs.
    files.sort_by(|a, b| b.size.cmp(&a.size));
    files.truncate(max_files);
    files
}

const FILE_PROMPT: &str = "\
请分析以下代码文件，输出：
1. 一句话说明该文件的职责
2. 主要的函数/类/组件及其作用（逐条列出）
3. 能推断出的与其他模块的关系

文件路径: {path}
语言: {lang}
{trunc}
```
{content}
```";

pub async fn explain_file(
    settings: &Settings,
    root: &str,
    relative_path: &str,
) -> Result<String, String> {
    let full: PathBuf = Path::new(root).join(relative_path);
    if !full.is_file() {
        return Err(format!("文件不存在: {relative_path}"));
    }
    let lang = scanner::file_language(
        &full
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default(),
    )
    .unwrap_or("未知");
    let (content, truncated) = read_source(&full)?;
    let trunc_note = if truncated { "\n(注意: 文件过长，仅展示前一部分)" } else { "" };

    let user_msg = FILE_PROMPT
        .replace("{path}", relative_path)
        .replace("{lang}", lang)
        .replace("{trunc}", trunc_note)
        .replace("{content}", &content);

    chat(
        &settings,
        &[system_prompt(), ChatMessage { role: "user", content: user_msg }],
    )
    .await
}

pub async fn summarize_project(
    settings: &Settings,
    root: &Path,
    max_files: usize,
    channel: Channel<SummarizeProgress>,
) -> Result<ProjectAnalysis, String> {
    let scan = scanner::scan_project(root)?;

    let files = pick_files(&scan.tree, if max_files == 0 { DEFAULT_MAX_FILES } else { max_files });
    let total = files.len();

    let mut summaries: Vec<FileSummary> = Vec::new();

    for (i, f) in files.iter().enumerate() {
        let _ = channel.send(SummarizeProgress {
            done: i,
            total,
            current: f.relative_path.clone(),
            phase: format!("正在理解文件 ({}/{})", i + 1, total),
        });

        let full: PathBuf = root.join(&f.relative_path);
        let Ok((content, truncated)) = read_source(&full) else {
            continue;
        };
        let trunc_note = if truncated { "\n(注意: 文件过长，仅展示前一部分)" } else { "" };
        let user_msg = FILE_PROMPT
            .replace("{path}", &f.relative_path)
            .replace("{lang}", f.language.as_deref().unwrap_or("未知"))
            .replace("{trunc}", trunc_note)
            .replace("{content}", &content);

        match chat(&settings, &[system_prompt(), ChatMessage { role: "user", content: user_msg }]).await {
            Ok(summary) => summaries.push(FileSummary {
                relative_path: f.relative_path.clone(),
                summary,
            }),
            Err(e) => {
                // Abort the run on auth/config errors; tolerate per-file hiccups.
                if e.contains("HTTP 401") || e.contains("HTTP 403") || e.contains("尚未配置") {
                    return Err(e);
                }
                summaries.push(FileSummary {
                    relative_path: f.relative_path.clone(),
                    summary: format!("⚠️ 分析失败: {e}"),
                });
            }
        }
    }

    // Architecture overview via map-reduce.
    let _ = channel.send(SummarizeProgress {
        done: total,
        total,
        current: "生成项目架构总览".to_string(),
        phase: "正在生成项目架构总览".to_string(),
    });

    let lang_stats: Vec<String> = scan
        .languages
        .iter()
        .take(8)
        .map(|l| format!("{} ({} 个文件)", l.language, l.files))
        .collect();

    let mut digest = String::new();
    for s in &summaries {
        // Keep each summary short in the digest to fit context.
        let brief: String = s.summary.chars().take(400).collect();
        digest.push_str(&format!("### {}\n{}\n\n", s.relative_path, brief));
    }

    let overview_prompt = format!(
        "以下是一个项目的整体信息和各文件的分析摘要，请为这个项目写一份架构总览，包括：\
1. 项目是做什么的（推断）\n2. 使用的技术栈\n3. 目录/模块结构及职责划分\n4. 核心数据流或工作流程\n\
5. 建议从哪些文件开始阅读\n\n项目名称: {}\n技术栈统计: {}\n\n各文件摘要:\n{}",
        scan.root_name,
        lang_stats.join(", "),
        digest
    );

    let overview = chat(
        &settings,
        &[system_prompt(), ChatMessage { role: "user", content: overview_prompt }],
    )
    .await
    .unwrap_or_else(|e| format!("⚠️ 架构总览生成失败: {e}"));

    Ok(ProjectAnalysis { overview, file_summaries: summaries })
}
