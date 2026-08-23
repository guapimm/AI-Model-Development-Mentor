import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Protocol, Settings } from "../types";

interface Preset {
  id: string;
  name: string;
  protocol: Protocol;
  baseUrl: string;
  models: string[];
  note?: string;
}

const GROUPS: { group: string; items: Preset[] }[] = [
  {
    group: "国内服务商",
    items: [
      { id: "deepseek", name: "DeepSeek", protocol: "openai", baseUrl: "https://api.deepseek.com/v1", models: ["deepseek-chat", "deepseek-reasoner"] },
      { id: "kimi", name: "Kimi（月之暗面）", protocol: "openai", baseUrl: "https://api.moonshot.cn/v1", models: ["moonshot-v1-8k", "moonshot-v1-32k", "moonshot-v1-128k"] },
      { id: "qwen", name: "通义千问", protocol: "openai", baseUrl: "https://dashscope.aliyuncs.com/compatible-mode/v1", models: ["qwen-turbo", "qwen-plus", "qwen-max"] },
      { id: "zhipu", name: "智谱 GLM", protocol: "openai", baseUrl: "https://open.bigmodel.cn/api/paas/v4", models: ["glm-4-flash", "glm-4-air", "glm-4-plus"] },
      { id: "doubao", name: "豆包·火山方舟", protocol: "openai", baseUrl: "https://ark.cn-beijing.volces.com/api/v3", models: [], note: "模型名请填你的推理接入点 ID（ep-xxx）" },
      { id: "siliconflow", name: "硅基流动", protocol: "openai", baseUrl: "https://api.siliconflow.cn/v1", models: ["deepseek-ai/DeepSeek-V3", "Qwen/Qwen2.5-72B-Instruct"] },
    ],
  },
  {
    group: "国际服务商",
    items: [
      { id: "openai", name: "OpenAI", protocol: "openai", baseUrl: "https://api.openai.com/v1", models: ["gpt-4o-mini", "gpt-4o"] },
      { id: "anthropic", name: "Anthropic Claude", protocol: "anthropic", baseUrl: "https://api.anthropic.com", models: ["claude-sonnet-4-20250514", "claude-3-5-haiku-latest"] },
      { id: "gemini", name: "Google Gemini", protocol: "gemini", baseUrl: "https://generativelanguage.googleapis.com/v1beta", models: ["gemini-2.0-flash", "gemini-1.5-pro"] },
      { id: "xai", name: "xAI Grok", protocol: "openai", baseUrl: "https://api.x.ai/v1", models: ["grok-3-mini"] },
      { id: "mistral", name: "Mistral", protocol: "openai", baseUrl: "https://api.mistral.ai/v1", models: ["mistral-large-latest"] },
      { id: "groq", name: "Groq", protocol: "openai", baseUrl: "https://api.groq.com/openai/v1", models: ["llama-3.3-70b-versatile"] },
      { id: "azure", name: "Azure OpenAI", protocol: "azure", baseUrl: "", models: [], note: "地址填 https://<资源名>.openai.azure.com" },
    ],
  },
  {
    group: "聚合网关",
    items: [
      { id: "openrouter", name: "OpenRouter（数百模型）", protocol: "openai", baseUrl: "https://openrouter.ai/api/v1", models: [] },
    ],
  },
  {
    group: "本地运行",
    items: [
      { id: "ollama", name: "Ollama", protocol: "openai", baseUrl: "http://localhost:11434/v1", models: [] },
      { id: "lmstudio", name: "LM Studio", protocol: "openai", baseUrl: "http://localhost:1234/v1", models: [] },
      { id: "vllm", name: "vLLM", protocol: "openai", baseUrl: "http://localhost:8000/v1", models: [] },
    ],
  },
];

const ALL_PRESETS = GROUPS.flatMap((g) => g.items);

const PROTOCOL_LABELS: Record<Protocol, string> = {
  openai: "OpenAI 兼容",
  anthropic: "Anthropic 原生",
  gemini: "Gemini 原生",
  azure: "Azure OpenAI",
};

export default function SettingsModal({
  initial,
  onClose,
  onSaved,
}: {
  initial: Settings;
  onClose: () => void;
  onSaved: (s: Settings) => void;
}) {
  const [presetId, setPresetId] = useState(
    () => ALL_PRESETS.find((p) => p.baseUrl === initial.baseUrl)?.id ?? "custom"
  );
  const [protocol, setProtocol] = useState<Protocol>(initial.protocol ?? "openai");
  const [baseUrl, setBaseUrl] = useState(initial.baseUrl);
  const [apiKey, setApiKey] = useState(initial.apiKey);
  const [model, setModel] = useState(initial.model);
  const [azureDeployment, setAzureDeployment] = useState(initial.azure_deployment ?? "");
  const [azureApiVersion, setAzureApiVersion] = useState(initial.azure_api_version ?? "");
  const [fetchedModels, setFetchedModels] = useState<string[]>([]);

  const [saving, setSaving] = useState(false);
  const [testing, setTesting] = useState(false);
  const [loadingModels, setLoadingModels] = useState(false);
  const [status, setStatus] = useState<{ ok: boolean; text: string } | null>(null);

  function currentSettings(): Settings {
    return {
      protocol,
      baseUrl: baseUrl.trim(),
      apiKey: apiKey.trim(),
      model: model.trim(),
      azure_deployment: azureDeployment.trim() || null,
      azure_api_version: azureApiVersion.trim() || null,
    };
  }

  function pickPreset(id: string) {
    setPresetId(id);
    setStatus(null);
    if (id === "custom") return;
    const p = ALL_PRESETS.find((x) => x.id === id);
    if (!p) return;
    setProtocol(p.protocol);
    setBaseUrl(p.baseUrl);
    setFetchedModels([]);
    if (p.models.length > 0) setModel(p.models[0]);
  }

  async function handleFetchModels() {
    setLoadingModels(true);
    setStatus(null);
    try {
      const models = await invoke<string[]>("list_ai_models", { settings: currentSettings() });
      setFetchedModels(models);
      setStatus({ ok: true, text: `获取到 ${models.length} 个模型，请在下拉框选择` });
    } catch (e) {
      setStatus({ ok: false, text: String(e) });
    } finally {
      setLoadingModels(false);
    }
  }

  async function handleTest() {
    setTesting(true);
    setStatus(null);
    try {
      await invoke("test_ai_connection", { settings: currentSettings() });
      setStatus({ ok: true, text: "✅ 连接成功，AI 服务工作正常" });
    } catch (e) {
      setStatus({ ok: false, text: `❌ ${String(e)}` });
    } finally {
      setTesting(false);
    }
  }

  async function handleSave() {
    if (!apiKey.trim() || !model.trim()) {
      setStatus({ ok: false, text: "API Key 和模型名称不能为空" });
      return;
    }
    setSaving(true);
    try {
      await invoke("update_settings", { settings: currentSettings() });
      onSaved(currentSettings());
      onClose();
    } catch (e) {
      setStatus({ ok: false, text: String(e) });
    } finally {
      setSaving(false);
    }
  }

  const modelOptions = fetchedModels.length > 0 ? fetchedModels : [];

  return (
    <div className="modal-mask" onClick={onClose}>
      <div className="modal modal-wide" onClick={(e) => e.stopPropagation()}>
        <h2>AI 服务设置</h2>

        <label className="field">
          <span>服务商预设</span>
          <select value={presetId} onChange={(e) => pickPreset(e.target.value)}>
            <option value="custom">自定义</option>
            {GROUPS.map((g) => (
              <optgroup key={g.group} label={g.group}>
                {g.items.map((p) => (
                  <option key={p.id} value={p.id}>{p.name}</option>
                ))}
              </optgroup>
            ))}
          </select>
        </label>

        <div className="protocol-chip">协议：{PROTOCOL_LABELS[protocol]}</div>

        <label className="field">
          <span>Base URL{protocol === "azure" ? "（Azure 资源地址，必填）" : ""}</span>
          <input
            value={baseUrl}
            onChange={(e) => setBaseUrl(e.target.value)}
            placeholder={
              protocol === "azure"
                ? "https://your-resource.openai.azure.com"
                : "https://api.deepseek.com/v1"
            }
          />
        </label>

        <label className="field">
          <span>API Key</span>
          <input
            type="password"
            value={apiKey}
            onChange={(e) => setApiKey(e.target.value)}
            placeholder="sk-..."
          />
        </label>

        <label className="field">
          <span>模型名称</span>
          <input
            value={model}
            onChange={(e) => setModel(e.target.value)}
            placeholder="deepseek-chat"
            list="model-options"
          />
          <datalist id="model-options">
            {[...new Set([...modelOptions, ...(ALL_PRESETS.find((p) => p.id === presetId)?.models ?? [])])].map((m) => (
              <option key={m} value={m} />
            ))}
          </datalist>
        </label>
        <button className="link-btn" onClick={handleFetchModels} disabled={loadingModels || testing}>
          {loadingModels ? "获取中..." : "🔄 从服务商获取模型列表"}
        </button>

        {protocol === "azure" && (
          <>
            <label className="field">
              <span>部署名称 Deployment（可选，默认同模型名）</span>
              <input
                value={azureDeployment}
                onChange={(e) => setAzureDeployment(e.target.value)}
                placeholder="my-deployment"
              />
            </label>
            <label className="field">
              <span>API 版本（可选）</span>
              <input
                value={azureApiVersion}
                onChange={(e) => setAzureApiVersion(e.target.value)}
                placeholder="2024-10-21"
              />
            </label>
          </>
        )}

        {status && (
          <div className={`modal-status ${status.ok ? "ok" : "err"}`}>{status.text}</div>
        )}

        {presetId !== "custom" &&
          (() => {
            const p = ALL_PRESETS.find((x) => x.id === presetId);
            return p?.note ? <p className="modal-hint">💡 {p.note}</p> : null;
          })()}

        <p className="modal-hint">
          Key 仅保存在本机应用数据目录，不会上传到任何服务器。
        </p>

        <div className="modal-actions">
          <button onClick={handleTest} disabled={testing || saving}>
            {testing ? "测试中..." : "🔌 测试连接"}
          </button>
          <button onClick={onClose}>取消</button>
          <button className="primary-btn" onClick={handleSave} disabled={saving || testing}>
            {saving ? "保存中..." : "保存"}
          </button>
        </div>
      </div>
    </div>
  );
}
