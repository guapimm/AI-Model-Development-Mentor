mod analysis;
mod llm;
mod scanner;
mod settings;
mod depgraph;
mod static_analysis;
mod symbols;
mod xmind;

use analysis::{ProjectAnalysis, Strength, SummarizeProgress};
use settings::Settings;
use std::path::PathBuf;
use tauri::ipc::Channel;
use tauri::AppHandle;

#[tauri::command]
fn scan_project(path: String) -> Result<scanner::ScanResult, String> {
    scanner::scan_project(&PathBuf::from(&path))
}

#[tauri::command]
fn get_settings(app: AppHandle) -> Settings {
    settings::load_settings(&app)
}

#[tauri::command]
fn update_settings(app: AppHandle, settings: Settings) -> Result<(), String> {
    settings::save_settings(&app, &settings)
}

#[tauri::command]
async fn ai_explain_file(
    app: AppHandle,
    root_path: String,
    relative_path: String,
    strength: Strength,
    unlimited_output: Option<bool>,
) -> Result<String, String> {
    let s = tauri::async_runtime::spawn_blocking(move || settings::load_settings(&app))
        .await
        .map_err(|e| e.to_string())?;
    analysis::explain_file(
        &s,
        &root_path,
        &relative_path,
        strength,
        unlimited_output.unwrap_or(false),
    )
    .await
}

#[tauri::command]
async fn ai_summarize_project(
    app: AppHandle,
    root_path: String,
    strength: Strength,
    full_scope: Option<bool>,
    unlimited_output: Option<bool>,
    channel: Channel<SummarizeProgress>,
) -> Result<ProjectAnalysis, String> {
    let s = tauri::async_runtime::spawn_blocking(move || settings::load_settings(&app))
        .await
        .map_err(|e| e.to_string())?;
    analysis::summarize_project(
        &s,
        &PathBuf::from(&root_path),
        strength,
        full_scope.unwrap_or(false),
        unlimited_output.unwrap_or(false),
        channel,
    )
    .await
}

#[tauri::command]
fn export_xmind(
    root_path: String,
    out_path: String,
    file_summaries: Option<Vec<analysis::FileSummary>>,
) -> Result<(), String> {
    let scan = scanner::scan_project(&PathBuf::from(&root_path))?;
    let summaries: std::collections::HashMap<String, String> = file_summaries
        .unwrap_or_default()
        .into_iter()
        .filter(|s| !s.summary.starts_with("⚠️"))
        .map(|s| (s.relative_path, s.summary))
        .collect();
    xmind::export_xmind(&scan, &PathBuf::from(&out_path), &summaries)
}

#[tauri::command]
fn analyze_static(
    root_path: String,
    channel: Channel<static_analysis::StaticProgress>,
) -> Result<static_analysis::StaticReport, String> {
    static_analysis::run_static_analysis(&PathBuf::from(&root_path), channel)
}

#[tauri::command]
fn get_file_symbols(
    root_path: String,
    relative_path: String,
    language: String,
) -> Result<symbols::FileSymbols, String> {
    symbols::parse_file(&PathBuf::from(&root_path), &relative_path, &language)
}

#[tauri::command]
fn get_dependency_graph(
    root_path: String,
    channel: Channel<static_analysis::StaticProgress>,
) -> Result<depgraph::DepGraphData, String> {
    depgraph::build_dependency_graph(&PathBuf::from(&root_path), channel)
}

#[tauri::command]
async fn list_ai_models(settings: Settings) -> Result<Vec<String>, String> {
    llm::list_models(&settings).await
}

#[tauri::command]
async fn test_ai_connection(settings: Settings) -> Result<(), String> {
    llm::test_connection(&settings).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            scan_project,
            get_settings,
            update_settings,
            ai_explain_file,
            ai_summarize_project,
            export_xmind,
            analyze_static,
            get_file_symbols,
            get_dependency_graph,
            list_ai_models,
            test_ai_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
