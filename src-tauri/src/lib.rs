mod analysis;
mod llm;
mod scanner;
mod settings;
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
) -> Result<String, String> {
    let s = tauri::async_runtime::spawn_blocking(move || settings::load_settings(&app))
        .await
        .map_err(|e| e.to_string())?;
    analysis::explain_file(&s, &root_path, &relative_path, strength).await
}

#[tauri::command]
async fn ai_summarize_project(
    app: AppHandle,
    root_path: String,
    strength: Strength,
    channel: Channel<SummarizeProgress>,
) -> Result<ProjectAnalysis, String> {
    let s = tauri::async_runtime::spawn_blocking(move || settings::load_settings(&app))
        .await
        .map_err(|e| e.to_string())?;
    analysis::summarize_project(&s, &PathBuf::from(&root_path), strength, channel).await
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
            export_xmind
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
