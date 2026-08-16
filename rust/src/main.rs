// mentor - AI Model Mentor CLI (Rust, zero dependencies).
// Mirror of the Go binary (cli/). Embeds all language/module prompt files.
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

mod zip;

include!(concat!(env!("OUT_DIR"), "/embed_gen.rs"));

const VERSION: &str = "0.1.0";

const SNAPSHOT_TEMPLATE: &str = r#"# 项目快照（断点续传）

> 由 AI 导师在每次对话结束时更新，用于断点续传与上下文恢复。控制在 200 行以内。

## 技术栈版本

| 技术 | 版本 |
|------|------|
| （待填） | |

## 数据库表清单

| 表名 | 用途 |
|------|------|
| | |

## 已完成的 API 接口

| 方法 | 路径 | 说明 |
|------|------|------|
| | | |

## 当前进度与待办

- 当前阶段：
- 下一步：
- 待确认事项：

## 续传暗号

（此处填入本次对话结束时的续传暗号）
"#;

struct Lang {
    code: &'static str,
    name: &'static str,
}

const LANGUAGES: &[Lang] = &[
    Lang { code: "zh-CN", name: "中文" },
    Lang { code: "en-US", name: "English" },
    Lang { code: "ja-JP", name: "日本語" },
    Lang { code: "ko-KR", name: "한국어" },
    Lang { code: "es-ES", name: "Español" },
    Lang { code: "fr-FR", name: "Français" },
    Lang { code: "de-DE", name: "Deutsch" },
    Lang { code: "pt-BR", name: "Português" },
    Lang { code: "ru-RU", name: "Русский" },
];

struct Mod {
    id: &'static str,
    file: &'static str,
    desc: &'static str,
}

const MODULES: &[Mod] = &[
    Mod { id: "agent", file: "agent.md", desc: "导师角色（默认必选）" },
    Mod { id: "security", file: "security.md", desc: "安全规范" },
    Mod { id: "style", file: "style.md", desc: "交互风格" },
    Mod { id: "workflow", file: "workflow.md", desc: "开发工作流" },
    Mod { id: "complete", file: "complete.md", desc: "完整版合并提示词" },
];

struct Cli {
    id: &'static str,
    name: &'static str,
    agent_file: &'static str,
    dir: &'static str,
}

const CLIS: &[Cli] = &[
    Cli { id: "mimo", name: "小米 MIMO", agent_file: "AGENTS.md", dir: "" },
    Cli { id: "claude-code", name: "Claude Code", agent_file: "CLAUDE.md", dir: "" },
    Cli { id: "codex", name: "OpenAI Codex", agent_file: "AGENTS.md", dir: "" },
    Cli { id: "cursor", name: "Cursor", agent_file: "AGENTS.md", dir: ".cursor/rules" },
    Cli { id: "other", name: "其他（自定义）", agent_file: "AGENTS.md", dir: "" },
];

struct Args {
    lang: Option<String>,
    modules: Option<String>,
    cli: Option<String>,
    dir: String,
    out: Option<String>,
    name: Option<String>,
    goal: Option<String>,
    show: bool,
    positional: Vec<String>,
}

fn parse_args(argv: &[String]) -> Args {
    let mut out = Args {
        lang: None,
        modules: None,
        cli: None,
        dir: ".".to_string(),
        out: None,
        name: None,
        goal: None,
        show: false,
        positional: Vec::new(),
    };
    let mut i = 0;
    while i < argv.len() {
        let a = &argv[i];
        match a.as_str() {
            "--lang" => {
                out.lang = argv.get(i + 1).cloned();
                i += 1;
            }
            "--modules" => {
                out.modules = argv.get(i + 1).cloned();
                i += 1;
            }
            "--cli" => {
                out.cli = argv.get(i + 1).cloned();
                i += 1;
            }
            "--dir" => {
                out.dir = argv.get(i + 1).cloned().unwrap_or_else(|| ".".to_string());
                i += 1;
            }
            "--out" => {
                out.out = argv.get(i + 1).cloned();
                i += 1;
            }
            "--name" => {
                out.name = argv.get(i + 1).cloned();
                i += 1;
            }
            "--goal" => {
                out.goal = argv.get(i + 1).cloned();
                i += 1;
            }
            "--show" => out.show = true,
            _ => out.positional.push(a.clone()),
        }
        i += 1;
    }
    out
}

fn read_line(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line.trim().to_string())
}

fn pick_lang(flag: &Option<String>) -> io::Result<&'static Lang> {
    if let Some(f) = flag {
        return LANGUAGES
            .iter()
            .find(|l| l.code == f)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, format!("未知语言: {}", f)));
    }
    println!("🌍 选择语言:");
    for (i, l) in LANGUAGES.iter().enumerate() {
        println!("  {}. {} ({})", i + 1, l.name, l.code);
    }
    loop {
        let input = read_line("> ")?;
        if let Ok(n) = input.parse::<usize>() {
            if n >= 1 && n <= LANGUAGES.len() {
                return Ok(&LANGUAGES[n - 1]);
            }
        }
        println!("请输入 1-{} 之间的数字", LANGUAGES.len());
    }
}

fn pick_modules(flag: &Option<String>) -> io::Result<Vec<&'static Mod>> {
    let ids: Vec<String> = match flag {
        Some(f) => f.split(',').map(|s| s.trim().to_string()).collect(),
        None => {
            println!("\n📦 选择模块（多选逗号分隔，如 1,2,4；回车默认只装 agent）:");
            for (i, m) in MODULES.iter().enumerate() {
                println!("  {}. {} — {}", i + 1, m.id, m.desc);
            }
            let input = read_line("> ")?;
            if input.trim().is_empty() {
                vec!["agent".to_string()]
            } else {
                input.split(',').map(|s| s.trim().to_string()).collect()
            }
        }
    };
    let mut out: Vec<&'static Mod> = Vec::new();
    for id in ids {
        let hit = if let Ok(n) = id.parse::<usize>() {
            if n >= 1 && n <= MODULES.len() {
                Some(&MODULES[n - 1])
            } else {
                None
            }
        } else {
            MODULES.iter().find(|m| m.id == id)
        };
        let m = hit.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, format!("未知模块: {}", id)))?;
        if !out.iter().any(|x| x.id == m.id) {
            out.push(m);
        }
    }
    Ok(out)
}

fn detect_cli(dir: &str) -> Option<&'static str> {
    let probe = |p: &str| Path::new(p).exists();
    if probe(&format!("{}/.mimocode", dir)) {
        return Some("mimo");
    }
    if probe(&format!("{}/CLAUDE.md", dir)) {
        return Some("claude-code");
    }
    if probe(&format!("{}/.cursor", dir)) {
        return Some("cursor");
    }
    if probe(&format!("{}/.codex", dir)) {
        return Some("codex");
    }
    if probe(&format!("{}/AGENTS.md", dir)) {
        return Some("codex");
    }
    None
}

fn pick_cli(flag: &Option<String>, dir: &str) -> io::Result<&'static Cli> {
    if let Some(f) = flag {
        return CLIS
            .iter()
            .find(|c| c.id == f)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, format!("未知工具: {}", f)));
    }
    if let Some(d) = detect_cli(dir) {
        if let Some(c) = CLIS.iter().find(|c| c.id == d) {
            println!("\n🖥️ 检测到工具: {}", c.name);
            return Ok(c);
        }
    }
    println!("\n🖥️ 选择目标工具:");
    for (i, c) in CLIS.iter().enumerate() {
        println!("  {}. {}", i + 1, c.name);
    }
    loop {
        let input = read_line("> ")?;
        if let Ok(n) = input.parse::<usize>() {
            if n >= 1 && n <= CLIS.len() {
                return Ok(&CLIS[n - 1]);
            }
        }
        println!("请输入 1-{} 之间的数字", CLIS.len());
    }
}

fn target_name(id: &str, cli: &Cli) -> String {
    if id == "agent" {
        cli.agent_file.to_string()
    } else if id == "complete" {
        "complete-mentor-prompt.md".to_string()
    } else {
        format!("{}.md", id)
    }
}

fn embed_bytes(lang: &str, file: &str) -> Option<&'static str> {
    let key = format!("{}/{}", lang, file);
    FILES.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
}

fn install_files(lang: &Lang, mods: &[&Mod], cli: &Cli, dir: &str) -> io::Result<()> {
    for m in mods {
        let content = embed_bytes(lang.code, m.file)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, format!("内嵌内容缺失 {}/{}", lang.code, m.file)))?;
        let name = target_name(m.id, cli);
        let target = Path::new(dir).join(cli.dir).join(&name);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&target, content)?;
        println!("✓ {}  ({} {})", target.display(), lang.code, m.id);
    }
    println!("完成。按 COMPATIBILITY.md 的说明启动对应工具即可。");
    Ok(())
}

fn cmd_install(argv: &[String]) -> io::Result<()> {
    let a = parse_args(argv);
    let lang = pick_lang(&a.lang)?;
    let mods = pick_modules(&a.modules)?;
    let cli = pick_cli(&a.cli, &a.dir)?;
    install_files(lang, &mods, cli, &a.dir)
}

fn cmd_add(argv: &[String]) -> io::Result<()> {
    let a = parse_args(argv);
    if a.positional.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "用法: mentor add <模块>... [--lang zh-CN] [--dir .]"));
    }
    let lang = pick_lang(&a.lang)?;
    let mut mods = Vec::new();
    for id in &a.positional {
        let m = MODULES
            .iter()
            .find(|m| m.id == id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, format!("未知模块: {}", id)))?;
        mods.push(m);
    }
    let cli = pick_cli(&a.cli, &a.dir)?;
    install_files(lang, &mods, cli, &a.dir)
}

fn cmd_remove(argv: &[String]) -> io::Result<()> {
    let a = parse_args(argv);
    if a.positional.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "用法: mentor remove <模块>... [--dir .]"));
    }
    for id in &a.positional {
        let names: Vec<String> = if id == "agent" {
            vec!["AGENTS.md".to_string(), "CLAUDE.md".to_string()]
        } else {
            vec![target_name(id, &CLIS[0])]
        };
        for n in &names {
            for p in [
                Path::new(&a.dir).join(n),
                Path::new(&a.dir).join(".cursor/rules").join(n),
            ] {
                if let Ok(()) = fs::remove_file(&p) {
                    println!("✗ 已移除 {}", p.display());
                }
            }
        }
    }
    Ok(())
}

fn cmd_list(argv: &[String]) -> io::Result<()> {
    let a = parse_args(argv);
    let names = ["AGENTS.md", "CLAUDE.md", "security.md", "style.md", "workflow.md", "complete-mentor-prompt.md"];
    let mut found = false;
    for n in names {
        for p in [Path::new(&a.dir).join(n), Path::new(&a.dir).join(".cursor/rules").join(n)] {
            if p.exists() {
                println!("✓ {}", p.display());
                found = true;
            }
        }
    }
    if !found {
        println!("（未检测到已安装的导师模块）");
    }
    Ok(())
}

fn cmd_detect(argv: &[String]) -> io::Result<()> {
    let a = parse_args(argv);
    match detect_cli(&a.dir) {
        Some(d) => {
            let c = CLIS.iter().find(|c| c.id == d);
            println!("检测到: {}", c.map(|c| c.name).unwrap_or(d));
        }
        None => println!("未检测到已知工具（可手动指定: mimo / claude-code / codex / cursor / other）"),
    }
    Ok(())
}

fn cmd_pack(argv: &[String]) -> io::Result<()> {
    let a = parse_args(argv);
    let out = a
        .out
        .clone()
        .unwrap_or_else(|| if a.dir == "." { "skill".to_string() } else { a.dir.clone() });
    fs::create_dir_all(&out)?;
    let skill_md = "# AI Model Mentor Skill\n\n提示词技能包：选择语言与模块后，按各语言 COMPATIBILITY.md 说明加载到你的 AI 工具。\n\n语言目录：zh-CN / en-US / ja-JP / ko-KR / es-ES / fr-FR / de-DE / pt-BR / ru-RU\n模块：agent（默认）/ security / style / workflow / complete\n";
    fs::write(Path::new(&out).join("SKILL.md"), skill_md)?;
    for lang in LANG_DIRS {
        let lang_dir = Path::new(&out).join(lang);
        fs::create_dir_all(&lang_dir)?;
        for (key, content) in FILES {
            if let Some(rel) = key.strip_prefix(lang).and_then(|r| r.strip_prefix('/')) {
                fs::write(lang_dir.join(rel), content)?;
            }
        }
    }
    let zip_path = format!("{}.zip", out);
    zip::write_stored_zip(Path::new(&out), Path::new(&zip_path))?;
    println!("✓ {}", zip_path);
    println!("✓ skill 包已生成到 {}/", out);
    Ok(())
}

fn cmd_init(argv: &[String]) -> io::Result<()> {
    let a = parse_args(argv);
    let name = match a.name {
        Some(n) => n,
        None => read_line("📛 项目名称（如：记账小助手）> ")?,
    };
    let goal = match a.goal {
        Some(g) => g,
        None => read_line("🎯 项目核心目标（一句话）> ")?,
    };

    fs::create_dir_all(&a.dir)?;
    let req = format!("# 项目需求说明书：{}\n\n## 核心目标\n{}\n\n## 用户角色\n（待补充，请与导师确认）\n\n## 核心操作流程\n（待补充）\n\n## 必须存储的数据\n（待补充）\n", name, goal);
    fs::write(Path::new(&a.dir).join("REQUIREMENTS.md"), req)?;
    println!("✓ {}", Path::new(&a.dir).join("REQUIREMENTS.md").display());

    let env = "# 环境变量示例：复制为 .env 并填入真实值，切勿把 .env 提交到仓库\n# 数据库连接串\nDATABASE_URL=\n# 密钥（示例）\nSECRET_KEY=\n";
    fs::write(Path::new(&a.dir).join(".env.example"), env)?;
    println!("✓ {}", Path::new(&a.dir).join(".env.example").display());

    let docs_dir = Path::new(&a.dir).join("docs");
    fs::create_dir_all(&docs_dir)?;
    fs::write(docs_dir.join("SNAPSHOT.md"), SNAPSHOT_TEMPLATE)?;
    println!("✓ {}", docs_dir.join("SNAPSHOT.md").display());

    println!("\n下一步：运行 `mentor install` 安装导师提示词，然后在你的 AI 工具中把本目录作为项目启动。");
    Ok(())
}

fn cmd_snapshot(argv: &[String]) -> io::Result<()> {
    let a = parse_args(argv);
    let p = Path::new(&a.dir).join("docs").join("SNAPSHOT.md");
    if a.show {
        if !p.exists() {
            println!("（未找到 {}，可先运行 `mentor snapshot` 生成）", p.display());
            return Ok(());
        }
        let b = fs::read_to_string(&p)?;
        print!("{}", b);
        return Ok(());
    }
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent)?;
    }
    if p.exists() {
        println!("已存在快照，不覆盖。查看请用 `mentor snapshot --show`");
        return Ok(());
    }
    fs::write(&p, SNAPSHOT_TEMPLATE)?;
    println!("✓ {}", p.display());
    Ok(())
}

fn usage() {
    println!(
        "AI 模型导师 mentor v{}

用法:
  mentor init               初始化项目：生成需求说明书 + .env.example + 文档骨架
  mentor init --name foo --goal \"做记账\" --dir ./proj
  mentor install             交互式安装向导（选语言 → 选模块 → 选工具）
  mentor install --lang zh-CN --modules agent,security --cli claude-code --dir ./proj
  mentor add <模块>...        追加模块，如: mentor add security --lang zh-CN
  mentor remove <模块>...     移除模块
  mentor list                列出当前项目已安装的模块
  mentor detect              检测项目使用的 AI 工具
  mentor snapshot            生成/查看项目快照（断点续传），--show 查看现有快照
  mentor pack                生成兼容 skill 目录 + zip
  mentor version             版本号
  mentor help                帮助

模块: agent(默认), security, style, workflow, complete
语言: zh-CN en-US ja-JP ko-KR es-ES fr-FR de-DE pt-BR ru-RU
工具: mimo claude-code codex cursor other",
        VERSION
    );
}

fn main() {
    let argv: Vec<String> = env::args().skip(1).collect();
    let result: io::Result<()> = if argv.is_empty() {
        usage();
        Ok(())
    } else {
        match argv[0].as_str() {
            "init" => cmd_init(&argv[1..]),
            "install" => cmd_install(&argv[1..]),
            "add" => cmd_add(&argv[1..]),
            "remove" => cmd_remove(&argv[1..]),
            "list" => cmd_list(&argv[1..]),
            "detect" => cmd_detect(&argv[1..]),
            "pack" => cmd_pack(&argv[1..]),
            "snapshot" => cmd_snapshot(&argv[1..]),
            "version" | "-v" | "--version" => {
                println!("mentor {}", VERSION);
                Ok(())
            }
            "help" | "-h" | "--help" => {
                usage();
                Ok(())
            }
            other => {
                eprintln!("未知命令: {}\n", other);
                usage();
                Ok(())
            }
        }
    };
    if let Err(e) = result {
        eprintln!("错误: {}", e);
        std::process::exit(1);
    }
}
