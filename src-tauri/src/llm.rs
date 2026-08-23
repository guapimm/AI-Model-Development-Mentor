use crate::settings::Settings;
use serde_json::{json, Value};

const MAX_COMPLETION_CHARS: usize = 4000;

pub struct ChatMessage {
    pub role: &'static str,
    pub content: String,
}

/// Minimal OpenAI-compatible chat completion client.
pub async fn chat(settings: &Settings, messages: &[ChatMessage]) -> Result<String, String> {
    if !settings.is_configured() {
        return Err("尚未配置 AI 服务：请先在设置中填写 API Key 和模型名称".to_string());
    }

    let base = settings.base_url.trim().trim_end_matches('/');
    let url = format!("{}/chat/completions", base);

    let payload = json!({
        "model": settings.model,
        "messages": messages.iter().map(|m| json!({
            "role": m.role,
            "content": m.content,
        })).collect::<Vec<_>>(),
        "temperature": 0.3,
        "stream": false,
    });

    let client = reqwest::Client::new();
    let resp = client
        .post(&url)
        .bearer_auth(&settings.api_key)
        .json(&payload)
        .timeout(std::time::Duration::from_secs(120))
        .send()
        .await
        .map_err(|e| format!("请求 AI 服务失败: {e}"))?;

    let status = resp.status();
    let body: Value = resp
        .json()
        .await
        .map_err(|e| format!("AI 响应解析失败 (HTTP {status}): {e}"))?;

    if !status.is_success() {
        let msg = body
            .get("error")
            .and_then(|e| e.get("message"))
            .and_then(|m| m.as_str())
            .unwrap_or("未知错误");
        return Err(format!("AI 服务返回错误 (HTTP {status}): {msg}"));
    }

    let content = body["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| format!("AI 响应格式异常: {body}"))?
        .to_string();

    Ok(content.chars().take(MAX_COMPLETION_CHARS).collect())
}
