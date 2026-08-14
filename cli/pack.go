package main

import (
	"archive/zip"
	"flag"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
)

func cmdPack(args []string) error {
	fs := flag.NewFlagSet("pack", flag.ContinueOnError)
	outFlag := fs.String("out", "skill", "输出目录")
	zipFlag := fs.Bool("zip", true, "同时生成 zip")
	if err := fs.Parse(args); err != nil {
		return err
	}
	out := *outFlag
	if err := os.MkdirAll(out, 0o755); err != nil {
		return err
	}
	skillMD := "# AI Model Mentor Skill\n\n提示词技能包：选择语言与模块后，按各语言 COMPATIBILITY.md 说明加载到你的 AI 工具。\n\n语言目录：zh-CN / en-US / ja-JP / ko-KR / es-ES / fr-FR / de-DE / pt-BR / ru-RU\n模块：agent（默认）/ security / style / workflow / complete\n"
	if err := os.WriteFile(filepath.Join(out, "SKILL.md"), []byte(skillMD), 0o644); err != nil {
		return err
	}
	entries, err := content.ReadDir("files")
	if err != nil {
		return err
	}
	for _, e := range entries {
		lang := e.Name()
		langDir := filepath.Join(out, lang)
		if err := os.MkdirAll(langDir, 0o755); err != nil {
			return err
		}
		mods, err := content.ReadDir("files/" + lang)
		if err != nil {
			return err
		}
		for _, m := range mods {
			data, err := content.ReadFile("files/" + lang + "/" + m.Name())
			if err != nil {
				return err
			}
			if err := os.WriteFile(filepath.Join(langDir, m.Name()), data, 0o644); err != nil {
				return err
			}
		}
	}
	if *zipFlag {
		zipPath := out + ".zip"
		if err := zipDir(out, zipPath); err != nil {
			return err
		}
		fmt.Println("✓ " + zipPath)
	}
	fmt.Println("✓ skill 包已生成到 " + out + "/")
	return nil
}

func zipDir(src, zipPath string) error {
	f, err := os.Create(zipPath)
	if err != nil {
		return err
	}
	defer f.Close()
	w := zip.NewWriter(f)
	defer w.Close()
	return filepath.Walk(src, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if info.IsDir() {
			return nil
		}
		rel, err := filepath.Rel(src, path)
		if err != nil {
			return err
		}
		rel = strings.ReplaceAll(rel, "\\", "/")
		zw, err := w.Create(rel)
		if err != nil {
			return err
		}
		sf, err := os.Open(path)
		if err != nil {
			return err
		}
		defer sf.Close()
		_, err = io.Copy(zw, sf)
		return err
	})
}
