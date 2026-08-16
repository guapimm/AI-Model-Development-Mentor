package main

import (
	"fmt"
	"os"
)

const version = "0.1.0"

type langInfo struct {
	code string
	name string
}

var languages = []langInfo{
	{"zh-CN", "中文"},
	{"en-US", "English"},
	{"ja-JP", "日本語"},
	{"ko-KR", "한국어"},
	{"es-ES", "Español"},
	{"fr-FR", "Français"},
	{"de-DE", "Deutsch"},
	{"pt-BR", "Português"},
	{"ru-RU", "Русский"},
}

type modInfo struct {
	id   string
	file string
	desc string
}

var modules = []modInfo{
	{"agent", "agent.md", "导师角色（默认必选）"},
	{"security", "security.md", "安全规范"},
	{"style", "style.md", "交互风格"},
	{"workflow", "workflow.md", "开发工作流"},
	{"complete", "complete.md", "完整版合并提示词"},
}

type cliInfo struct {
	id        string
	name      string
	agentFile string
	dir       string
}

var clis = []cliInfo{
	{"mimo", "小米 MIMO", "AGENTS.md", ""},
	{"claude-code", "Claude Code", "CLAUDE.md", ""},
	{"codex", "OpenAI Codex", "AGENTS.md", ""},
	{"cursor", "Cursor", "AGENTS.md", ".cursor/rules"},
	{"other", "其他（自定义）", "AGENTS.md", ""},
}

func main() {
	args := os.Args[1:]
	if len(args) == 0 {
		usage()
		return
	}
	var err error
	switch args[0] {
	case "init":
		err = cmdInit(args[1:])
	case "install":
		err = cmdInstall(args[1:])
	case "add":
		err = cmdAdd(args[1:])
	case "remove":
		err = cmdRemove(args[1:])
	case "list":
		err = cmdList(args[1:])
	case "detect":
		err = cmdDetect(args[1:])
	case "pack":
		err = cmdPack(args[1:])
	case "snapshot":
		err = cmdSnapshot(args[1:])
	case "version", "-v", "--version":
		fmt.Println("mentor " + version)
	case "help", "-h", "--help":
		usage()
	default:
		fmt.Fprintf(os.Stderr, "未知命令: %s\n\n", args[0])
		usage()
	}
	if err != nil {
		fmt.Fprintln(os.Stderr, "错误:", err)
		os.Exit(1)
	}
}

func usage() {
  fmt.Println(`AI 模型导师 mentor CLI v` + version + `

用法:
  mentor init               初始化项目：生成需求说明书 + .env.example + 文档骨架
  mentor init --name foo --goal "做记账" --dir ./proj
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
工具: mimo claude-code codex cursor other`)
}
