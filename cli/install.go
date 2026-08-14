package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

func readLine(prompt string) (string, error) {
	fmt.Print(prompt)
	r := bufio.NewReader(os.Stdin)
	line, err := r.ReadString('\n')
	if err != nil && line == "" {
		return "", err
	}
	return strings.TrimSpace(line), nil
}

func pickLang(flagLang string) (langInfo, error) {
	if flagLang != "" {
		for _, l := range languages {
			if l.code == flagLang {
				return l, nil
			}
		}
		return langInfo{}, fmt.Errorf("未知语言: %s", flagLang)
	}
	fmt.Println("🌍 选择语言:")
	for i, l := range languages {
		fmt.Printf("  %d. %s (%s)\n", i+1, l.name, l.code)
	}
	for {
		in, err := readLine("> ")
		if err != nil {
			return langInfo{}, err
		}
		n, err := strconv.Atoi(in)
		if err == nil && n >= 1 && n <= len(languages) {
			return languages[n-1], nil
		}
		fmt.Println("请输入 1-" + strconv.Itoa(len(languages)) + " 之间的数字")
	}
}

func pickModules(flagMods string) ([]modInfo, error) {
	var ids []string
	if flagMods != "" {
		for _, m := range strings.Split(flagMods, ",") {
			ids = append(ids, strings.TrimSpace(m))
		}
	} else {
		fmt.Println("\n📦 选择模块（多选逗号分隔，如 1,2,4；回车默认只装 agent）:")
		for i, m := range modules {
			fmt.Printf("  %d. %s — %s\n", i+1, m.id, m.desc)
		}
		in, err := readLine("> ")
		if err != nil {
			return nil, err
		}
		if strings.TrimSpace(in) == "" {
			ids = []string{"agent"}
		} else {
			for _, part := range strings.Split(in, ",") {
				part = strings.TrimSpace(part)
				n, err := strconv.Atoi(part)
				if err == nil && n >= 1 && n <= len(modules) {
					ids = append(ids, modules[n-1].id)
				} else {
					ids = append(ids, part)
				}
			}
		}
	}
	var out []modInfo
	for _, id := range ids {
		found := false
		for _, m := range modules {
			if m.id == id {
				out = append(out, m)
				found = true
				break
			}
		}
		if !found {
			return nil, fmt.Errorf("未知模块: %s", id)
		}
	}
	return out, nil
}

func pickCLI(flagCLI, dir string) (cliInfo, error) {
	if flagCLI != "" {
		for _, c := range clis {
			if c.id == flagCLI {
				return c, nil
			}
		}
		return cliInfo{}, fmt.Errorf("未知工具: %s", flagCLI)
	}
	if d := detectCLI(dir); d != "" {
		for _, c := range clis {
			if c.id == d {
				fmt.Printf("\n🖥️ 检测到工具: %s\n", c.name)
				return c, nil
			}
		}
	}
	fmt.Println("\n🖥️ 选择目标工具:")
	for i, c := range clis {
		fmt.Printf("  %d. %s\n", i+1, c.name)
	}
	for {
		in, err := readLine("> ")
		if err != nil {
			return cliInfo{}, err
		}
		n, err := strconv.Atoi(in)
		if err == nil && n >= 1 && n <= len(clis) {
			return clis[n-1], nil
		}
		fmt.Println("请输入 1-" + strconv.Itoa(len(clis)) + " 之间的数字")
	}
}

func cmdInstall(args []string) error {
	fs := flag.NewFlagSet("install", flag.ContinueOnError)
	langFlag := fs.String("lang", "", "语言代码")
	modsFlag := fs.String("modules", "", "模块列表（逗号分隔）")
	cliFlag := fs.String("cli", "", "目标工具")
	dirFlag := fs.String("dir", ".", "安装目录")
	if err := fs.Parse(args); err != nil {
		return err
	}
	lang, err := pickLang(*langFlag)
	if err != nil {
		return err
	}
	mods, err := pickModules(*modsFlag)
	if err != nil {
		return err
	}
	cli, err := pickCLI(*cliFlag, *dirFlag)
	if err != nil {
		return err
	}
	return installFiles(lang, mods, cli, *dirFlag)
}

func installFiles(l langInfo, mods []modInfo, c cliInfo, dir string) error {
	for _, m := range mods {
		data, err := content.ReadFile("files/" + l.code + "/" + m.file)
		if err != nil {
			return fmt.Errorf("内嵌内容缺失 %s/%s: %w", l.code, m.file, err)
		}
		name := fileBase(m.id)
		if m.id == "agent" {
			name = c.agentFile
		}
		target := filepath.Join(dir, c.dir, name)
		if err := os.MkdirAll(filepath.Dir(target), 0o755); err != nil {
			return err
		}
		if err := os.WriteFile(target, data, 0o644); err != nil {
			return err
		}
		fmt.Printf("✓ %s  (%s %s)\n", target, l.code, m.id)
	}
	fmt.Println("完成。按 COMPATIBILITY.md 的说明启动对应工具即可。")
	return nil
}

func fileBase(id string) string {
	if id == "complete" {
		return "complete-mentor-prompt.md"
	}
	return id + ".md"
}

func cmdAdd(args []string) error {
	fs := flag.NewFlagSet("add", flag.ContinueOnError)
	langFlag := fs.String("lang", "zh-CN", "语言代码")
	cliFlag := fs.String("cli", "", "目标工具（默认自动检测）")
	dirFlag := fs.String("dir", ".", "目录")
	if err := fs.Parse(args); err != nil {
		return err
	}
	if fs.NArg() == 0 {
		return fmt.Errorf("用法: mentor add <模块>... [--lang zh-CN] [--dir .]")
	}
	lang, err := pickLang(*langFlag)
	if err != nil {
		return err
	}
	var mods []modInfo
	for _, id := range fs.Args() {
		found := false
		for _, m := range modules {
			if m.id == id {
				mods = append(mods, m)
				found = true
				break
			}
		}
		if !found {
			return fmt.Errorf("未知模块: %s", id)
		}
	}
	cli, err := pickCLI(*cliFlag, *dirFlag)
	if err != nil {
		return err
	}
	return installFiles(lang, mods, cli, *dirFlag)
}

func cmdRemove(args []string) error {
	fs := flag.NewFlagSet("remove", flag.ContinueOnError)
	dirFlag := fs.String("dir", ".", "目录")
	if err := fs.Parse(args); err != nil {
		return err
	}
	if fs.NArg() == 0 {
		return fmt.Errorf("用法: mentor remove <模块>... [--dir .]")
	}
	for _, id := range fs.Args() {
		var names []string
		if id == "agent" {
			names = []string{"AGENTS.md", "CLAUDE.md"}
		} else {
			names = []string{fileBase(id)}
		}
		for _, n := range names {
			for _, p := range []string{filepath.Join(*dirFlag, n), filepath.Join(*dirFlag, ".cursor", "rules", n)} {
				if err := os.Remove(p); err == nil {
					fmt.Printf("✗ 已移除 %s\n", p)
				}
			}
		}
	}
	return nil
}

func cmdList(args []string) error {
	fs := flag.NewFlagSet("list", flag.ContinueOnError)
	dirFlag := fs.String("dir", ".", "目录")
	if err := fs.Parse(args); err != nil {
		return err
	}
	names := []string{"AGENTS.md", "CLAUDE.md", "security.md", "style.md", "workflow.md", "complete-mentor-prompt.md"}
	found := false
	for _, n := range names {
		for _, p := range []string{filepath.Join(*dirFlag, n), filepath.Join(*dirFlag, ".cursor", "rules", n)} {
			if _, err := os.Stat(p); err == nil {
				fmt.Println("✓ " + p)
				found = true
			}
		}
	}
	if !found {
		fmt.Println("（未检测到已安装的导师模块）")
	}
	return nil
}

func cmdDetect(args []string) error {
	fs := flag.NewFlagSet("detect", flag.ContinueOnError)
	dirFlag := fs.String("dir", ".", "目录")
	if err := fs.Parse(args); err != nil {
		return err
	}
	d := detectCLI(*dirFlag)
	if d == "" {
		fmt.Println("未检测到已知工具（可手动指定: mimo / claude-code / codex / cursor / other）")
		return nil
	}
	for _, c := range clis {
		if c.id == d {
			fmt.Println("检测到: " + c.name)
			return nil
		}
	}
	return nil
}
