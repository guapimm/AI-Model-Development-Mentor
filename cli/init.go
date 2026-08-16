package main

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"
)

func cmdInit(args []string) error {
	fs := flag.NewFlagSet("init", flag.ContinueOnError)
	nameFlag := fs.String("name", "", "项目名称")
	goalFlag := fs.String("goal", "", "项目核心目标（一句话）")
	dirFlag := fs.String("dir", ".", "初始化目录")
	if err := fs.Parse(args); err != nil {
		return err
	}

	name := *nameFlag
	goal := *goalFlag

	if name == "" {
		n, err := readLine("📛 项目名称（如：记账小助手）> ")
		if err != nil {
			return err
		}
		name = n
	}
	if goal == "" {
		g, err := readLine("🎯 项目核心目标（一句话）> ")
		if err != nil {
			return err
		}
		goal = g
	}

	dir := *dirFlag
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return err
	}

	req := fmt.Sprintf("# 项目需求说明书：%s\n\n## 核心目标\n%s\n\n## 用户角色\n（待补充，请与导师确认）\n\n## 核心操作流程\n（待补充）\n\n## 必须存储的数据\n（待补充）\n", name, goal)
	if err := os.WriteFile(filepath.Join(dir, "REQUIREMENTS.md"), []byte(req), 0o644); err != nil {
		return err
	}
	fmt.Println("✓ " + filepath.Join(dir, "REQUIREMENTS.md"))

	env := "# 环境变量示例：复制为 .env 并填入真实值，切勿把 .env 提交到仓库\n# 数据库连接串\nDATABASE_URL=\n# 密钥（示例）\nSECRET_KEY=\n"
	if err := os.WriteFile(filepath.Join(dir, ".env.example"), []byte(env), 0o644); err != nil {
		return err
	}
	fmt.Println("✓ " + filepath.Join(dir, ".env.example"))

	docsDir := filepath.Join(dir, "docs")
	if err := os.MkdirAll(docsDir, 0o755); err != nil {
		return err
	}
	if err := os.WriteFile(filepath.Join(docsDir, "SNAPSHOT.md"), []byte(snapshotTemplate()), 0o644); err != nil {
		return err
	}
	fmt.Println("✓ " + filepath.Join(docsDir, "SNAPSHOT.md"))

	fmt.Println("\n下一步：运行 `mentor install` 安装导师提示词，然后在你的 AI 工具中把本目录作为项目启动。")
	return nil
}
