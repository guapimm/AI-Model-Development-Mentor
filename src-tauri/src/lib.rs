mod scanner;

use scanner::ScanResult;
use std::path::PathBuf;

#[tauri::command]
fn scan_project(path: String) -> Result<ScanResult, String> {
    scanner::scan_project(&PathBuf::from(&path))
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
        .invoke_handler(tauri::generate_handler![scan_project])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
