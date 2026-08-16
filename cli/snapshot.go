package main

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"
)

func snapshotTemplate() string {
	return `# 项目快照（断点续传）

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
`
}

func cmdSnapshot(args []string) error {
	fs := flag.NewFlagSet("snapshot", flag.ContinueOnError)
	dirFlag := fs.String("dir", ".", "目录")
	showFlag := fs.Bool("show", false, "显示现有快照")
	if err := fs.Parse(args); err != nil {
		return err
	}
	p := filepath.Join(*dirFlag, "docs", "SNAPSHOT.md")
	if *showFlag {
		if _, err := os.Stat(p); err != nil {
			fmt.Println("（未找到 " + p + "，可先运行 `mentor snapshot` 生成）")
			return nil
		}
		b, err := os.ReadFile(p)
		if err != nil {
			return err
		}
		fmt.Print(string(b))
		return nil
	}
	if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
		return err
	}
	if _, err := os.Stat(p); err == nil {
		fmt.Println("已存在快照，不覆盖。查看请用 `mentor snapshot --show`")
		return nil
	}
	if err := os.WriteFile(p, []byte(snapshotTemplate()), 0o644); err != nil {
		return err
	}
	fmt.Println("✓ " + p)
	return nil
}
