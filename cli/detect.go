package main

import (
	"os"
	"path/filepath"
)

func detectCLI(dir string) string {
	if _, err := os.Stat(filepath.Join(dir, ".mimocode")); err == nil {
		return "mimo"
	}
	if _, err := os.Stat(filepath.Join(dir, "CLAUDE.md")); err == nil {
		return "claude-code"
	}
	if _, err := os.Stat(filepath.Join(dir, ".cursor")); err == nil {
		return "cursor"
	}
	if _, err := os.Stat(filepath.Join(dir, ".codex")); err == nil {
		return "codex"
	}
	if _, err := os.Stat(filepath.Join(dir, "AGENTS.md")); err == nil {
		return "codex"
	}
	return ""
}
