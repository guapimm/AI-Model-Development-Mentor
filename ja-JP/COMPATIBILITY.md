# 各 AI ツールの読み込み説明（互換性ガイド）

`prompts/` ディレクトリ内のプロンプトの内容は、特定の AI ツールに**依存しません**。大規模言語モデル（LLM）ベースのコーディングツールであれば、どのツールでも利用できます。違いがあるのは**読み込み方法**だけです：メインファイル名、配置場所、読み込みコマンド。本ファイルは「読み込み説明ファイル」であり、新しいツールを追加する場合はここに 1 行追記するだけで済みます。

> ヒント：すべてのツールは `mentor` CLI でワンクリックインストールできます（ツールに応じて正しい場所へ自動的に書き込みます）。詳細は末尾を参照してください。

## クイック比較表

| ツール | メインファイル（エージェントの役割） | 配置場所 | 読み込み方法 | その他のモジュール（security/style/workflow） |
|------|---------------------|---------|---------|-----------------------------------|
| 小米 MIMO | `AGENTS.md` | プロジェクトルート | 手動：`/skill AGENTS.md` | 同様に `/skill security.md` などで 1 つずつ読み込む |
| Claude Code | `CLAUDE.md` または `AGENTS.md` | プロジェクトルート | 自動読み込み | メインファイル内で `@security.md` で参照するか、サブディレクトリに配置して必要に応じて読み込む |
| OpenAI Codex | `AGENTS.md` | プロジェクトルート | 自動読み込み | メインファイル内で `@security.md` で参照 |
| Cursor | `AGENTS.md` | `.cursor/rules/` | 自動読み込み（rules は glob で適用範囲のマッチングが可能） | 同名のファイルを同じディレクトリに配置 |
| Gemini CLI | `GEMINI.md` | プロジェクトルート | 自動読み込み | リネームしてまとめて配置するか、`@` で参照 |
| Google Jules | `JULES.md` | プロジェクトルート | 自動読み込み | 同上 |
| Aider | `CONVENTIONS.md` | プロジェクトルート | 自動読み込み | 内容をマージするか、ファイルを分けて参照 |
| Windsurf | `.windsurfrules` | プロジェクトルート | 自動読み込み | 同上 |
| GitHub Copilot Agent | `AGENTS.md` | プロジェクトルート | 自動読み込み | `@security.md` で参照 |

## 各ツールの詳細説明

### 小米 MIMO
1. `prompts/AGENTS.md` をプロジェクトルートにコピーする
2. MIMO のセッションで `/skill AGENTS.md` と入力し、メンター役割を読み込む
3. セキュリティ / スタイル / ワークフローが必要な場合は、`/skill security.md`、`/skill style.md`、`/skill workflow.md` を必要に応じて読み込む
4. 長期プロジェクト：`/dream` でルールを MEMORY.md に定着させる。接続が切れた場合は `mimo --continue` で復旧する

### Claude Code
1. `prompts/AGENTS.md` をコピー → `CLAUDE.md` にリネームする（または `AGENTS.md` のままでも可。新しいバージョンは自動認識する）
2. プロジェクトルートに配置すれば、毎回のセッションで自動読み込みされる
3. その他のモジュールは `CLAUDE.md` 内で `@security.md` と参照するか、直接追記してマージする
4. サブディレクトリ内の `CLAUDE.md` は、そのディレクトリに入ったときに必要に応じて読み込まれる

### OpenAI Codex
1. `prompts/AGENTS.md` をプロジェクトルートにコピーする（Codex はルートの `AGENTS.md` を自動読み込みする）
2. その他のモジュールは `AGENTS.md` 内で `@security.md` と参照する
3. 接続切れからの復旧は `codex --resume`（または `codex exec --resume`）

### Cursor
1. `prompts/AGENTS.md` を `.cursor/rules/` ディレクトリにコピーする（Agent が rules を自動読み込みする）
2. ファイル単位で適用範囲を指定したい場合は、`.mdc` 形式に変換して frontmatter の `globs` でマッチさせる
3. その他のモジュールも同名ファイルのまま `.cursor/rules/` に配置する

### Gemini CLI
1. `prompts/AGENTS.md` をコピー → `GEMINI.md` にリネームしてプロジェクトルートに配置すれば、自動読み込みされる
2. その他のモジュールは `GEMINI.md` にマージするか、必要に応じて `@` で参照する

### Google Jules
1. `prompts/AGENTS.md` をコピー → `JULES.md` にリネームしてプロジェクトルートに配置すれば、自動読み込みされる

### Aider
1. `prompts/AGENTS.md` をコピー → `CONVENTIONS.md` にリネームしてプロジェクトルートに配置すれば、編集セッションで自動読み込みされる

### Windsurf
1. `prompts/AGENTS.md` をコピー → `.windsurfrules` にリネームしてプロジェクトルートに配置すれば、自動読み込みされる

### GitHub Copilot Agent
1. `prompts/AGENTS.md` をプロジェクトルートにコピーすれば自動読み込みされる。その他のモジュールは `@security.md` で参照する

## mentor CLI でワンクリックインストール

```bash
mentor install          # インタラクティブ：言語選択 → モジュール選択（デフォルトは agent）→ ツールを自動識別 / 選択
mentor install --lang zh-CN --modules agent,security --cli claude-code
mentor add workflow     # モジュールを追加
mentor list             # インストール済みモジュールを確認
```

`mentor` は上表のルールに従って、各ツールが求めるファイル名と場所へ自動的に書き込みます（Claude Code → `CLAUDE.md`、Cursor → `.cursor/rules/`、その他 → `AGENTS.md` など）。

## 完全版プロンプト

モジュールごとに分割する必要がない場合は、`prompts/開発メンター完全版プロンプト.md`（4 モジュール統合版、一括読み込み用）をそのまま使用できます。
