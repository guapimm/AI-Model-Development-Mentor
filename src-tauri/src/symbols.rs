use serde::Serialize;
use std::path::Path;
use tree_sitter::{Node, Parser, Tree};

/// Files larger than this are skipped for symbol parsing.
const MAX_PARSE_BYTES: usize = 300_000;
const MAX_SYMBOLS: usize = 500;
const MAX_IMPORTS: usize = 80;

#[derive(Serialize, Clone)]
pub struct Symbol {
    pub kind: String,
    pub name: String,
    #[serde(rename = "startLine")]
    pub start_line: usize,
    #[serde(rename = "endLine")]
    pub end_line: usize,
    pub signature: String,
}

#[derive(Serialize, Clone)]
pub struct FileSymbols {
    #[serde(rename = "relativePath")]
    pub relative_path: String,
    pub language: String,
    #[serde(rename = "supportedParse")]
    pub supported_parse: bool,
    pub symbols: Vec<Symbol>,
    pub imports: Vec<String>,
}

struct LangCfg {
    /// node kind -> display label
    symbol_kinds: &'static [(&'static str, &'static str)],
    import_kinds: &'static [&'static str],
}

fn cfg_for(language: &str) -> Option<LangCfg> {
    let cfg = match language {
        "Rust" => LangCfg {
            symbol_kinds: &[
                ("function_item", "函数"),
                ("struct_item", "结构体"),
                ("enum_item", "枚举"),
                ("trait_item", "Trait"),
                ("impl_item", "实现块"),
            ],
            import_kinds: &["use_declaration"],
        },
        "Python" => LangCfg {
            symbol_kinds: &[("function_definition", "函数"), ("class_definition", "类")],
            import_kinds: &["import_statement", "import_from_statement"],
        },
        "TypeScript" | "JavaScript" | "TSX" | "JSX" => LangCfg {
            symbol_kinds: &[
                ("function_declaration", "函数"),
                ("method_definition", "方法"),
                ("class_declaration", "类"),
                ("interface_declaration", "接口"),
                ("type_alias_declaration", "类型别名"),
                ("enum_declaration", "枚举"),
                ("abstract_class_declaration", "抽象类"),
            ],
            import_kinds: &["import_statement"],
        },
        "Go" => LangCfg {
            symbol_kinds: &[
                ("function_declaration", "函数"),
                ("method_declaration", "方法"),
                ("type_spec", "类型"),
            ],
            import_kinds: &["import_declaration"],
        },
        "Java" => LangCfg {
            symbol_kinds: &[
                ("method_declaration", "方法"),
                ("constructor_declaration", "构造器"),
                ("class_declaration", "类"),
                ("interface_declaration", "接口"),
                ("enum_declaration", "枚举"),
                ("record_declaration", "Record"),
            ],
            import_kinds: &["import_declaration"],
        },
        "C" => LangCfg {
            symbol_kinds: &[
                ("function_definition", "函数"),
                ("struct_specifier", "结构体"),
                ("enum_specifier", "枚举"),
                ("type_definition", "类型定义"),
            ],
            import_kinds: &["preproc_include"],
        },
        "C++" => LangCfg {
            symbol_kinds: &[
                ("function_definition", "函数"),
                ("class_specifier", "类"),
                ("struct_specifier", "结构体"),
                ("enum_specifier", "枚举"),
            ],
            import_kinds: &["preproc_include", "using_declaration"],
        },
        "C#" => LangCfg {
            symbol_kinds: &[
                ("method_declaration", "方法"),
                ("constructor_declaration", "构造器"),
                ("class_declaration", "类"),
                ("interface_declaration", "接口"),
                ("struct_declaration", "结构体"),
                ("enum_declaration", "枚举"),
                ("record_declaration", "Record"),
            ],
            import_kinds: &["using_directive"],
        },
        "PHP" => LangCfg {
            symbol_kinds: &[
                ("function_definition", "函数"),
                ("method_declaration", "方法"),
                ("class_declaration", "类"),
                ("interface_declaration", "接口"),
                ("trait_declaration", "Trait"),
            ],
            import_kinds: &["namespace_use_declaration", "include_expression"],
        },
        "Ruby" => LangCfg {
            symbol_kinds: &[
                ("method", "方法"),
                ("singleton_method", "类方法"),
                ("class", "类"),
                ("module", "模块"),
            ],
            import_kinds: &["call"],
        },
        _ => return None,
    };
    Some(cfg)
}

fn grammar_for(parser: &mut Parser, language: &str) -> bool {
    let lang = match language {
        "Rust" => tree_sitter_rust::LANGUAGE,
        "Python" => tree_sitter_python::LANGUAGE,
        "TypeScript" => tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
        "TSX" => tree_sitter_typescript::LANGUAGE_TSX,
        "JavaScript" | "JSX" => tree_sitter_javascript::LANGUAGE,
        "Go" => tree_sitter_go::LANGUAGE,
        "Java" => tree_sitter_java::LANGUAGE,
        "C" => tree_sitter_c::LANGUAGE,
        "C++" => tree_sitter_cpp::LANGUAGE,
        "C#" => tree_sitter_c_sharp::LANGUAGE,
        "PHP" => tree_sitter_php::LANGUAGE_PHP,
        "Ruby" => tree_sitter_ruby::LANGUAGE,
        _ => return false,
    };
    parser.set_language(&lang.into()).is_ok()
}

fn first_line(text: &str) -> String {
    let line = text.lines().next().unwrap_or("").trim();
    let mut s: String = line.chars().take(140).collect();
    if line.chars().count() > 140 {
        s.push('…');
    }
    s
}

/// Extract a human-readable name for a symbol node, handling languages
/// where the declaration name isn't in the standard "name" field.
fn node_display_name(node: Node, source: &str) -> Option<String> {
    // Standard path: most grammars expose a "name" field.
    if let Some(name_node) = node.child_by_field_name("name") {
        return Some(
            name_node
                .utf8_text(source.as_bytes())
                .ok()?
                .to_string(),
        );
    }

    match node.kind() {
        // Rust impl blocks: name comes from the implemented type.
        "impl_item" => node
            .child_by_field_name("type")
            .and_then(|t| t.utf8_text(source.as_bytes()).ok())
            .map(|s| format!("impl {}", s)),
        // C/C++ functions: declarator -> function_declarator -> declarator.
        "function_definition" => find_descendant(node, "function_declarator")
            .and_then(|fd| fd.child_by_field_name("declarator"))
            .and_then(|d| d.utf8_text(source.as_bytes()).ok())
            .map(|s| s.to_string()),
        _ => None,
    }
}

/// Go type_spec: refine the label based on the declared body kind.
fn go_type_label(node: Node) -> &'static str {
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            match cursor.node().kind() {
                "struct_type" => return "结构体",
                "interface_type" => return "接口",
                _ => {}
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    "类型"
}

fn find_descendant<'a>(root: Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        if n.kind() == kind {
            return Some(n);
        }
        let mut child = n.walk();
        let mut children: Vec<Node<'a>> = Vec::new();
        if child.goto_first_child() {
            loop {
                children.push(child.node());
                if !child.goto_next_sibling() {
                    break;
                }
            }
        }
        for c in children.into_iter().rev() {
            stack.push(c);
        }
    }
    None
}

pub fn extract_symbols(
    relative_path: &str,
    language: &str,
    source: &str,
) -> Option<FileSymbols> {
    let cfg = cfg_for(language)?;

    let mut parser = Parser::new();
    if !grammar_for(&mut parser, language) {
        return None;
    }
    let tree: Tree = parser.parse(source, None)?;

    let mut symbols: Vec<Symbol> = Vec::new();
    let mut imports: Vec<String> = Vec::new();

    let mut cursor = tree.walk();
    'walk: loop {
        let node = cursor.node();

        // Imports
        if cfg.import_kinds.contains(&node.kind()) && imports.len() < MAX_IMPORTS {
            let text = node.utf8_text(source.as_bytes()).unwrap_or("");
            let line = first_line(text);
            // Ruby: only capture require/load calls.
            if language != "Ruby" || line.starts_with("require") || line.starts_with("load") {
                if !line.is_empty() && !imports.contains(&line) {
                    imports.push(line);
                }
            }
        }

        // Symbols
        if let Some((_, base_label)) = cfg.symbol_kinds.iter().find(|(k, _)| *k == node.kind()) {
            if symbols.len() < MAX_SYMBOLS {
                let label: &str = if node.kind() == "type_spec" {
                    go_type_label(node)
                } else {
                    base_label
                };
                if let Some(name) = node_display_name(node, source) {
                    symbols.push(Symbol {
                        kind: label.to_string(),
                        name,
                        start_line: node.start_position().row + 1,
                        end_line: node.end_position().row + 1,
                        signature: first_line(
                            node.utf8_text(source.as_bytes()).unwrap_or(""),
                        ),
                    });
                }
            }
        }

        // Generic DFS traversal.
        if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                continue 'walk;
            }
            if !cursor.goto_parent() {
                break 'walk;
            }
        }
    }

    // Sort by start line for outline display.
    symbols.sort_by_key(|s| s.start_line);

    Some(FileSymbols {
        relative_path: relative_path.to_string(),
        language: language.to_string(),
        supported_parse: true,
        symbols,
        imports,
    })
}

pub fn parse_file(root: &Path, relative_path: &str, language: &str) -> Result<FileSymbols, String> {
    let full = root.join(relative_path);
    let meta = std::fs::metadata(&full).map_err(|e| e.to_string())?;
    if meta.len() as usize > MAX_PARSE_BYTES {
        return Err(format!("文件过大（{} KB），跳过符号解析", meta.len() / 1024));
    }
    let source = std::fs::read_to_string(&full).map_err(|e| e.to_string())?;
    extract_symbols(relative_path, language, &source)
        .ok_or_else(|| format!("暂不支持该语言的符号解析: {language}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_symbols() {
        let src = r#"
use std::collections::HashMap;

struct Point { x: i32, y: i32 }

trait Shape { fn area(&self) -> f64; }

impl Shape for Point {
    fn area(&self) -> f64 { 0.0 }
}

fn main() {
    println!("hi");
}
"#;
        let fs = extract_symbols("a.rs", "Rust", src).unwrap();
        assert!(fs.supported_parse);
        let names: Vec<&str> = fs.symbols.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"Point"));
        assert!(names.contains(&"Shape"));
        assert!(names.iter().any(|n| n.starts_with("impl ")));
        assert!(names.contains(&"main"));
        assert!(fs.imports.iter().any(|i| i.contains("HashMap")));
    }

    #[test]
    fn test_typescript_symbols() {
        let src = r#"
import { useState } from "react";

export interface User { id: number; }

export class Service {
    fetch(): void {}
}

function helper(): number { return 1; }

export type Alias = string;
"#;
        let fs = extract_symbols("a.ts", "TypeScript", src).unwrap();
        let names: Vec<&str> = fs.symbols.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"User"));
        assert!(names.contains(&"Service"));
        assert!(names.contains(&"helper"));
        assert!(names.contains(&"Alias"));
        assert!(fs.imports.iter().any(|i| i.contains("react")));
    }

    #[test]
    fn test_python_symbols() {
        let src = "from os import path\n\nclass Foo:\n    def bar(self):\n        pass\n\ndef main():\n    pass\n";
        let fs = extract_symbols("a.py", "Python", src).unwrap();
        let names: Vec<&str> = fs.symbols.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"Foo"));
        assert!(names.contains(&"bar"));
        assert!(names.contains(&"main"));
        assert!(fs.imports.len() >= 1);
    }

    #[test]
    fn test_go_and_c() {
        let go_src = "package main\n\nimport \"fmt\"\n\ntype Server struct { port int }\n\nfunc main() { fmt.Println() }\n";
        let fs = extract_symbols("a.go", "Go", go_src).unwrap();
        let names: Vec<&str> = fs.symbols.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"Server"));
        assert!(names.contains(&"main"));

        let c_src = "#include <stdio.h>\n\nint add(int a, int b) {\n    return a + b;\n}\n";
        let fc = extract_symbols("a.c", "C", c_src).unwrap();
        let namesc: Vec<&str> = fc.symbols.iter().map(|s| s.name.as_str()).collect();
        assert!(namesc.contains(&"add"));
        assert!(fc.imports.iter().any(|i| i.contains("stdio.h")));
    }

    #[test]
    fn test_unsupported_language() {
        assert!(extract_symbols("a.txt", "Markdown", "# hi").is_none());
    }
}
