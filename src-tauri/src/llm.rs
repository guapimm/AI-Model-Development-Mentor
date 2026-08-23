use crate::settings::{Protocol, Settings};
use serde_json::{json, Map, Value};

pub struct ChatMessage {
    pub role: &'static str,
    pub content: String,
}

pub async fn chat(
    settings: &Settings,
    messages: &[ChatMessage],
    max_tokens: Option<u16>,
) -> Result<String, String> {
    if !settings.is_configured() {
        return Err("尚未配置 AI 服务：请先在设置中填写 API Key 和模型名称".to_string());
    }

    match settings.protocol {
        Protocol::OpenAI => chat_openai_compatible(settings, messages, max_tokens).await,
        Protocol::Anthropic => chat_anthropic(settings, messages, max_tokens.unwrap_or(4096)).await,
        Protocol::Gemini => chat_gemini(settings, messages, max_tokens).await,
        Protocol::Azure => chat_azure(settings, messages, max_tokens).await,
    }
}

fn http_client() -> reqwest::Client {
    reqwest::Client::new()
}

async fn send_json(
    url: &str,
    headers: Vec<(&str, String)>,
    body: Value,
) -> Result<String, String> {
    let mut req = http_client()
        .post(url)
        .json(&body)
        .timeout(std::time::Duration::from_secs(180));
    for (k, v) in headers {
        req = req.header(k, v);
    }
    let resp = req
        .send()
        .await
        .map_err(|e| format!("请求 AI 服务失败: {e}"))?;

    let status = resp.status();
    let parsed: Value = resp
        .json()
        .await
        .map_err(|e| format!("AI 响应解析失败 (HTTP {status}): {e}"))?;

    if !status.is_success() {
        return Err(format!("AI 服务返回错误 (HTTP {status}): {}", error_message(&parsed)));
    }
    Ok(parsed.to_string())
}

fn error_message(body: &Value) -> String {
    body.get("error")
        .and_then(|e| e.get("message"))
        .and_then(|m| m.as_str())
        .or_else(|| body.get("message").and_then(|m| m.as_str()))
        .unwrap_or("未知错误")
        .to_string()
}

// ---------------- OpenAI-compatible ----------------

pub fn openai_body(model: &str, messages: &[ChatMessage], max_tokens: Option<u16>) -> Value {
    let mut obj = Map::new();
    obj.insert("model".into(), json!(model));
    obj.insert(
        "messages".into(),
        json!(messages
            .iter()
            .map(|m| json!({ "role": m.role, "content": m.content }))
            .collect::<Vec<_>>()),
    );
    obj.insert("temperature".into(), json!(0.3));
    obj.insert("stream".into(), json!(false));
    if let Some(mt) = max_tokens {
        obj.insert("max_tokens".into(), json!(mt));
    }
    Value::Object(obj)
}

pub fn parse_openai_response(body: &Value) -> Result<String, String> {
    body["choices"][0]["message"]["content"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("AI 响应格式异常: {body}"))
}

async fn chat_openai_compatible(
    settings: &Settings,
    messages: &[ChatMessage],
    max_tokens: Option<u16>,
) -> Result<String, String> {
    let url = format!("{}/chat/completions", settings.effective_base_url());
    let body = openai_body(&settings.model, messages, max_tokens);
    let raw = send_json(
        &url,
        vec![("Authorization", format!("Bearer {}", settings.api_key))],
        body,
    )
    .await?;
    let parsed: Value = serde_json::from_str(&raw).unwrap_or(Value::Null);
    parse_openai_response(&parsed)
}

// ---------------- Anthropic native ----------------

pub fn anthropic_body(system: &str, messages: &[ChatMessage], max_tokens: u16) -> Value {
    json!({
        "max_tokens": max_tokens,
        "system": system,
        "messages": messages
            .iter()
            .filter(|m| m.role != "system")
            .map(|m| json!({ "role": m.role, "content": m.content }))
            .collect::<Vec<_>>(),
    })
}

pub fn parse_anthropic_response(body: &Value) -> Result<String, String> {
    let blocks = body["content"]
        .as_array()
        .ok_or_else(|| format!("AI 响应格式异常: {body}"))?;
    let text: String = blocks
        .iter()
        .filter_map(|b| b.get("text").and_then(|t| t.as_str()))
        .collect::<Vec<_>>()
        .join("");
    if text.is_empty() {
        return Err(format!("AI 响应为空: {body}"));
    }
    Ok(text)
}

async fn chat_anthropic(settings: &Settings, messages: &[ChatMessage], max_tokens: u16) -> Result<String, String> {
    let url = format!("{}/v1/messages", settings.effective_base_url());
    let system = messages
        .iter()
        .find(|m| m.role == "system")
        .map(|m| m.content.clone())
        .unwrap_or_default();
    let body = anthropic_body(&system, messages, max_tokens);
    let raw = send_json(
        &url,
        vec![
            ("x-api-key", settings.api_key.clone()),
            ("anthropic-version", "2023-06-01".to_string()),
        ],
        body,
    )
    .await?;
    let parsed: Value = serde_json::from_str(&raw).unwrap_or(Value::Null);
    parse_anthropic_response(&parsed)
}

// ---------------- Gemini native ----------------

pub fn gemini_body(system: &str, messages: &[ChatMessage], max_tokens: Option<u16>) -> Value {
    let mut generation_config = Map::new();
    generation_config.insert("temperature".into(), json!(0.3));
    if let Some(mt) = max_tokens {
        generation_config.insert("maxOutputTokens".into(), json!(mt));
    }
    json!({
        "systemInstruction": { "parts": [{ "text": system }] },
        "contents": messages
            .iter()
            .filter(|m| m.role != "system")
            .map(|m| json!({ "role": "user", "parts": [{ "text": m.content }] }))
            .collect::<Vec<_>>(),
        "generationConfig": Value::Object(generation_config),
    })
}

pub fn parse_gemini_response(body: &Value) -> Result<String, String> {
    let parts = body["candidates"][0]["content"]["parts"]
        .as_array()
        .ok_or_else(|| format!("AI 响应格式异常: {body}"))?;
    let text: String = parts
        .iter()
        .filter_map(|p| p.get("text").and_then(|t| t.as_str()))
        .collect::<Vec<_>>()
        .join("");
    if text.is_empty() {
        return Err(format!("AI 响应为空: {body}"));
    }
    Ok(text)
}

async fn chat_gemini(settings: &Settings, messages: &[ChatMessage], max_tokens: Option<u16>) -> Result<String, String> {
    let base = settings.effective_base_url();
    let url = format!(
        "{}/models/{}:generateContent",
        base.trim_end_matches('/'),
        settings.model
    );
    let system = messages
        .iter()
        .find(|m| m.role == "system")
        .map(|m| m.content.clone())
        .unwrap_or_default();
    let body = gemini_body(&system, messages, max_tokens);
    let raw = send_json(&url, vec![], body.clone()).await?;
    // Gemini authenticates via ?key= query param; retry with it on error.
    let parsed: Value = serde_json::from_str(&raw).unwrap_or(Value::Null);

    if parsed.get("error").is_some() {
        let url_with_key = format!("{url}?key={}", settings.api_key);
        let raw2 = send_json(&url_with_key, vec![], body).await?;
        let parsed2: Value = serde_json::from_str(&raw2).unwrap_or(Value::Null);
        if parsed2.get("error").is_some() {
            return Err(error_message(&parsed2));
        }
        return parse_gemini_response(&parsed2);
    }
    parse_gemini_response(&parsed)
}

// ---------------- Azure OpenAI ----------------

async fn chat_azure(settings: &Settings, messages: &[ChatMessage], max_tokens: Option<u16>) -> Result<String, String> {
    let deployment = settings
        .azure_deployment
        .clone()
        .unwrap_or_else(|| settings.model.clone());
    let api_version = settings
        .azure_api_version
        .clone()
        .unwrap_or_else(|| "2024-10-21".to_string());
    let url = format!(
        "{}/openai/deployments/{}/chat/completions?api-version={}",
        settings.effective_base_url(),
        deployment,
        api_version
    );
    let mut body = openai_body(&settings.model, messages, max_tokens);
    // Azure ignores "model"; keep payload minimal.
    if let Some(obj) = body.as_object_mut() {
        obj.remove("model");
    }
    let raw = send_json(
        &url,
        vec![("api-key", settings.api_key.clone())],
        body,
    )
    .await?;
    let parsed: Value = serde_json::from_str(&raw).unwrap_or(Value::Null);
    parse_openai_response(&parsed)
}

// ---------------- Model listing & connection test ----------------

pub async fn list_models(settings: &Settings) -> Result<Vec<String>, String> {
    if settings.api_key.trim().is_empty() && settings.protocol != Protocol::OpenAI {
        return Err("请先填写 API Key".to_string());
    }

    let base = settings.effective_base_url();
    let (url, headers): (String, Vec<(&str, String)>) = match settings.protocol {
        Protocol::OpenAI | Protocol::Azure => (
            format!("{}/models", base),
            vec![(
                if settings.protocol == Protocol::Azure { "api-key" } else { "Authorization" },
                if settings.protocol == Protocol::Azure {
                    settings.api_key.clone()
                } else {
                    format!("Bearer {}", settings.api_key)
                },
            )],
        ),
        Protocol::Anthropic => (
            format!("{}/v1/models", base),
            vec![
                ("x-api-key", settings.api_key.clone()),
                ("anthropic-version", "2023-06-01".to_string()),
            ],
        ),
        Protocol::Gemini => (
            format!("{}/models?key={}", base, settings.api_key),
            vec![],
        ),
    };

    let mut req = http_client().get(&url).timeout(std::time::Duration::from_secs(30));
    for (k, v) in headers {
        req = req.header(k, v);
    }
    let resp = req
        .send()
        .await
        .map_err(|e| format!("获取模型列表失败: {e}"))?;
    let status = resp.status();
    let body: Value = resp
        .json()
        .await
        .map_err(|e| format!("响应解析失败 (HTTP {status}): {e}"))?;
    if !status.is_success() {
        return Err(format!("获取模型列表失败 (HTTP {status}): {}", error_message(&body)));
    }

    let models: Vec<String> = match settings.protocol {
        Protocol::Gemini => body["models"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|m| m.get("name").and_then(|n| n.as_str()))
                    .map(|n| n.trim_start_matches("models/").to_string())
                    .collect()
            })
            .unwrap_or_default(),
        _ => body["data"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|m| m.get("id").and_then(|i| i.as_str()))
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default(),
    };

    if models.is_empty() {
        return Err("服务商未返回模型列表，请手动输入模型名称".to_string());
    }
    Ok(models)
}

pub async fn test_connection(settings: &Settings) -> Result<(), String> {
    chat(
        settings,
        &[ChatMessage { role: "user", content: "请只回复两个字：正常".to_string() }],
        Some(32),
    )
    .await
    .map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn msgs() -> Vec<ChatMessage> {
        vec![
            ChatMessage { role: "system", content: "你是助手".into() },
            ChatMessage { role: "user", content: "解释这段代码".into() },
        ]
    }

    #[test]
    fn test_openai_body() {
        let b = openai_body("deepseek-chat", &msgs(), Some(1500));
        assert_eq!(b["model"], "deepseek-chat");
        assert_eq!(b["messages"][0]["role"], "system");
        assert_eq!(b["max_tokens"], 1500);
        let no_cap = openai_body("m", &msgs(), None);
        assert!(no_cap.get("max_tokens").is_none());
    }

    #[test]
    fn test_openai_response_parse() {
        let v = json!({"choices":[{"message":{"content":"你好"}}]});
        assert_eq!(parse_openai_response(&v).unwrap(), "你好");
        assert!(parse_openai_response(&json!({})).is_err());
    }

    #[test]
    fn test_anthropic_body_and_response() {
        let b = anthropic_body("sys", &msgs(), 4096);
        assert_eq!(b["system"], "sys");
        assert_eq!(b["max_tokens"], 4096);
        // system must not be duplicated into messages
        let arr = b["messages"].as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["role"], "user");

        let resp = json!({"content":[{"type":"text","text":"第一段"},{"type":"text","text":"第二段"}]});
        assert_eq!(parse_anthropic_response(&resp).unwrap(), "第一段第二段");
    }

    #[test]
    fn test_gemini_body_and_response() {
        let b = gemini_body("sys", &msgs(), Some(2000));
        assert_eq!(b["systemInstruction"]["parts"][0]["text"], "sys");
        assert_eq!(b["contents"][0]["role"], "user");
        assert_eq!(b["generationConfig"]["maxOutputTokens"], 2000);

        let resp = json!({"candidates":[{"content":{"parts":[{"text":"结果"}]}}]});
        assert_eq!(parse_gemini_response(&resp).unwrap(), "结果");
    }

    #[test]
    fn test_error_message_extraction() {
        let e = json!({"error":{"message":"invalid api key"}});
        assert_eq!(error_message(&e), "invalid api key");
    }
}
