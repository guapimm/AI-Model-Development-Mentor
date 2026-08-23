import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Settings } from "../types";

export default function SettingsModal({
  initial,
  onClose,
  onSaved,
}: {
  initial: Settings | null;
  onClose: () => void;
  onSaved: (s: Settings) => void;
}) {
  const [baseUrl, setBaseUrl] = useState(initial?.baseUrl ?? "");
  const [apiKey, setApiKey] = useState(initial?.apiKey ?? "");
  const [model, setModel] = useState(initial?.model ?? "");
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function handleSave() {
    if (!apiKey.trim() || !model.trim()) {
      setError("API Key 和模型名称不能为空");
      return;
    }
    setSaving(true);
    setError(null);
    const s: Settings = {
      baseUrl: baseUrl.trim(),
      apiKey: apiKey.trim(),
      model: model.trim(),
    };
    try {
      await invoke("update_settings", { settings: s });
      onSaved(s);
      onClose();
    } catch (e) {
      setError(String(e));
    } finally {
      setSaving(false);
    }
  }

  return (
    <div className="modal-mask" onClick={onClose}>
      <div className="modal" onClick={(e) => e.stopPropagation()}>
        <h2>AI 服务设置</h2>
        <p className="modal-hint">
          兼容 OpenAI 接口协议。常见组合：
          DeepSeek（https://api.deepseek.com/v1 / deepseek-chat）、
          Kimi（https://api.moonshot.cn/v1 / moonshot-v1-8k）、
          本地 Ollama（http://localhost:11434/v1 / 模型名）。
        </p>

        <label className="field">
          <span>Base URL</span>
          <input
            value={baseUrl}
            onChange={(e) => setBaseUrl(e.target.value)}
            placeholder="https://api.deepseek.com/v1"
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
          />
        </label>

        {error && <div className="modal-error">{error}</div>}

        <p className="modal-hint">
          Key 仅保存在本机应用数据目录，不会上传到任何服务器。
        </p>

        <div className="modal-actions">
          <button onClick={onClose}>取消</button>
          <button className="primary-btn" onClick={handleSave} disabled={saving}>
            {saving ? "保存中..." : "保存"}
          </button>
        </div>
      </div>
    </div>
  );
}
