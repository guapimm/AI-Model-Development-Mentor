use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    OpenAI,
    Anthropic,
    Gemini,
    Azure,
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::OpenAI
    }
}

impl Protocol {
    #[allow(dead_code)]
    pub fn label(&self) -> &'static str {
        match self {
            Protocol::OpenAI => "OpenAI 兼容",
            Protocol::Anthropic => "Anthropic 原生",
            Protocol::Gemini => "Gemini 原生",
            Protocol::Azure => "Azure OpenAI",
        }
    }

    /// Fallback base URL when the user leaves it empty.
    pub fn default_base_url(&self) -> &'static str {
        match self {
            Protocol::OpenAI => "https://api.openai.com/v1",
            Protocol::Anthropic => "https://api.anthropic.com",
            Protocol::Gemini => "https://generativelanguage.googleapis.com/v1beta",
            Protocol::Azure => "",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    #[serde(default)]
    pub protocol: Protocol,
    #[serde(default)]
    pub base_url: String,
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub model: String,
    // Azure-only optional fields.
    #[serde(default)]
    pub azure_deployment: Option<String>,
    #[serde(default)]
    pub azure_api_version: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            protocol: Protocol::OpenAI,
            base_url: Protocol::default_base_url(&Protocol::OpenAI).to_string(),
            api_key: String::new(),
            model: String::new(),
            azure_deployment: None,
            azure_api_version: None,
        }
    }
}

impl Settings {
    pub fn is_configured(&self) -> bool {
        !self.api_key.trim().is_empty() && !self.model.trim().is_empty()
    }

    /// Base URL with protocol fallback applied.
    pub fn effective_base_url(&self) -> String {
        let trimmed = self.base_url.trim().trim_end_matches('/');
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
        self.protocol.default_base_url().trim_end_matches('/').to_string()
    }
}

fn settings_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("无法定位应用数据目录: {e}"))?;
    Ok(dir.join("settings.json"))
}

/// Load settings; older config files (without protocol fields) migrate
/// seamlessly via serde defaults.
pub fn load_settings(app: &tauri::AppHandle) -> Settings {
    settings_path(app)
        .ok()
        .and_then(|p| fs::read_to_string(p).ok())
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_settings(app: &tauri::AppHandle, settings: &Settings) -> Result<(), String> {
    let path = settings_path(app)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_old_settings_migration() {
        // Old format: no protocol/provider fields at all.
        let old = r#"{"base_url":"https://api.deepseek.com/v1","api_key":"sk-x","model":"deepseek-chat"}"#;
        let s: Settings = serde_json::from_str(old).unwrap();
        assert_eq!(s.protocol, Protocol::OpenAI);
        assert!(s.is_configured());
    }

    #[test]
    fn test_effective_base_url_fallback() {
        let mut s = Settings::default();
        assert_eq!(s.effective_base_url(), "https://api.openai.com/v1");
        s.base_url = "https://api.moonshot.cn/v1/".to_string();
        assert_eq!(s.effective_base_url(), "https://api.moonshot.cn/v1");
        s.protocol = Protocol::Anthropic;
        s.base_url = String::new();
        assert_eq!(s.effective_base_url(), "https://api.anthropic.com");
    }
}
