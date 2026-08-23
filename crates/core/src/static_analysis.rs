use crate::scanner::{self, FileNode};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::path::Path;

/// Per-file read cap when counting metrics.
const METRIC_READ_CAP: u64 = 1_000_000;
/// Max rows kept in metrics table.
const MAX_METRICS: usize = 1000;
/// Lines threshold for "consider splitting" warnings.
const BIG_FILE_LINES: usize = 2000;

#[derive(Serialize, Clone)]
pub struct TechStackItem {
    pub name: String,
    pub category: String,
    /// Which manifest file it was found in.
    pub source: String,
}

#[derive(Serialize, Clone)]
pub struct FileMetric {
    #[serde(rename = "relativePath")]
    pub relative_path: String,
    pub language: String,
    pub lines: usize,
    #[serde(rename = "codeLines")]
    pub code_lines: usize,
    pub todos: usize,
}

#[derive(Serialize, Clone)]
pub struct EntryPoint {
    #[serde(rename = "relativePath")]
    pub relative_path: String,
    pub reason: String,
}

#[derive(Serialize)]
pub struct StaticReport {
    #[serde(rename = "rootName")]
    pub root_name: String,
    #[serde(rename = "techStack")]
    pub tech_stack: Vec<TechStackItem>,
    #[serde(rename = "entryPoints")]
    pub entry_points: Vec<EntryPoint>,
    pub metrics: Vec<FileMetric>,
    #[serde(rename = "totalCodeFiles")]
    pub total_code_files: usize,
    #[serde(rename = "totalLines")]
    pub total_lines: usize,
    #[serde(rename = "totalTodos")]
    pub total_todos: usize,
    pub warnings: Vec<String>,
}

struct Rule {
    pat: &'static str,
    name: &'static str,
    cat: &'static str,
}

macro_rules! rules {
    ($(($pat:expr, $name:expr, $cat:expr)),+ $(,)?) => {
        &[$(Rule { pat: $pat, name: $name, cat: $cat }),+]
    };
}

/// Dependency-name / keyword rules. For JSON manifests and .csproj we extract
/// dependency identifiers first; other manifests are matched on full lowercase
/// text (substring match, small false-positive risk accepted in v1).
static RULES: &[Rule] = rules!(
    ("react", "React", "前端框架"),
    ("vue", "Vue", "前端框架"),
    ("@angular/core", "Angular", "前端框架"),
    ("svelte", "Svelte", "前端框架"),
    ("next", "Next.js", "全栈框架"),
    ("nuxt", "Nuxt", "全栈框架"),
    ("astro", "Astro", "全栈框架"),
    ("remix", "Remix", "全栈框架"),
    ("solid-js", "SolidJS", "前端框架"),
    ("preact", "Preact", "前端框架"),
    ("jquery", "jQuery", "前端库"),
    ("tailwindcss", "Tailwind CSS", "样式"),
    ("element-plus", "Element Plus", "UI 组件库"),
    ("element-ui", "Element UI", "UI 组件库"),
    ("ant-design", "Ant Design", "UI 组件库"),
    ("antd", "Ant Design", "UI 组件库"),
    ("@mui/material", "MUI", "UI 组件库"),
    ("bootstrap", "Bootstrap", "样式"),
    ("redux", "Redux", "状态管理"),
    ("zustand", "Zustand", "状态管理"),
    ("pinia", "Pinia", "状态管理"),
    ("mobx", "MobX", "状态管理"),
    ("axios", "Axios", "网络库"),
    ("express", "Express", "后端框架"),
    ("koa", "Koa", "后端框架"),
    ("fastify", "Fastify", "后端框架"),
    ("@nestjs/core", "NestJS", "后端框架"),
    ("socket.io", "Socket.IO", "实时通信"),
    ("electron", "Electron", "桌面端"),
    ("prisma", "Prisma", "ORM/数据"),
    ("typeorm", "TypeORM", "ORM/数据"),
    ("sequelize", "Sequelize", "ORM/数据"),
    ("mongoose", "Mongoose", "ORM/数据"),
    ("knex", "Knex", "ORM/数据"),
    ("vite", "Vite", "构建工具"),
    ("webpack", "Webpack", "构建工具"),
    ("rollup", "Rollup", "构建工具"),
    ("esbuild", "esbuild", "构建工具"),
    ("typescript", "TypeScript", "语言/编译器"),
    ("jest", "Jest", "测试"),
    ("vitest", "Vitest", "测试"),
    ("mocha", "Mocha", "测试"),
    ("cypress", "Cypress", "测试"),
    ("playwright", "Playwright", "测试"),
    ("django", "Django", "后端框架"),
    ("flask", "Flask", "后端框架"),
    ("fastapi", "FastAPI", "后端框架"),
    ("tornado", "Tornado", "后端框架"),
    ("scrapy", "Scrapy", "爬虫"),
    ("sqlalchemy", "SQLAlchemy", "ORM/数据"),
    ("celery", "Celery", "任务队列"),
    ("numpy", "NumPy", "科学计算"),
    ("pandas", "Pandas", "数据处理"),
    ("torch", "PyTorch", "机器学习"),
    ("tensorflow", "TensorFlow", "机器学习"),
    ("scikit-learn", "scikit-learn", "机器学习"),
    ("pytest", "pytest", "测试"),
    ("uvicorn", "uvicorn", "服务器"),
    ("gunicorn", "gunicorn", "服务器"),
    ("pyside", "PySide", "桌面端"),
    ("pyqt", "PyQt", "桌面端"),
    ("tokio", "Tokio", "异步运行时"),
    ("actix-web", "Actix Web", "后端框架"),
    ("axum", "Axum", "后端框架"),
    ("rocket", "Rocket", "后端框架"),
    ("tauri", "Tauri", "桌面端"),
    ("sqlx", "SQLx", "ORM/数据"),
    ("diesel", "Diesel", "ORM/数据"),
    ("clap", "clap", "CLI 框架"),
    ("gin-gonic/gin", "Gin", "后端框架"),
    ("labstack/echo", "Echo", "后端框架"),
    ("gofiber", "Fiber", "后端框架"),
    ("beego", "Beego", "后端框架"),
    ("gorm.io", "GORM", "ORM/数据"),
    ("spf13/cobra", "Cobra", "CLI 框架"),
    ("stretchr/testify", "testify", "测试"),
    ("spring-boot", "Spring Boot", "后端框架"),
    ("spring-cloud", "Spring Cloud", "后端框架"),
    ("mybatis", "MyBatis", "ORM/数据"),
    ("hibernate", "Hibernate", "ORM/数据"),
    ("netty", "Netty", "网络库"),
    ("junit", "JUnit", "测试"),
    ("lombok", "Lombok", "开发工具"),
    ("laravel/framework", "Laravel", "后端框架"),
    ("symfony", "Symfony", "后端框架"),
    ("topthink/framework", "ThinkPHP", "后端框架"),
    ("phpunit", "PHPUnit", "测试"),
    ("rails", "Ruby on Rails", "后端框架"),
    ("sinatra", "Sinatra", "后端框架"),
    ("rspec", "RSpec", "测试"),
    ("flutter_sdk", "Flutter", "移动端"),
    ("microsoft.entityframeworkcore", "Entity Framework Core", "ORM/数据"),
    ("microsoft.aspnetcore", "ASP.NET Core", "后端框架"),
    ("xunit", "xUnit", "测试"),
    ("nunit", "NUnit", "测试"),
);

struct ManifestKind {
    file_name: &'static str,
    suffix: Option<&'static str>,
    json_deps: bool,
}

const MANIFESTS: &[ManifestKind] = &[
    ManifestKind { file_name: "package.json", suffix: None, json_deps: true },
    ManifestKind { file_name: "composer.json", suffix: None, json_deps: true },
    ManifestKind { file_name: "Cargo.toml", suffix: None, json_deps: false },
    ManifestKind { file_name: "go.mod", suffix: None, json_deps: false },
    ManifestKind { file_name: "requirements.txt", suffix: None, json_deps: false },
    ManifestKind { file_name: "pyproject.toml", suffix: None, json_deps: false },
    ManifestKind { file_name: "pom.xml", suffix: None, json_deps: false },
    ManifestKind { file_name: "build.gradle", suffix: None, json_deps: false },
    ManifestKind { file_name: "build.gradle.kts", suffix: None, json_deps: false },
    ManifestKind { file_name: "Gemfile", suffix: None, json_deps: false },
    ManifestKind { file_name: "pubspec.yaml", suffix: None, json_deps: false },
    ManifestKind { file_name: "", suffix: Some(".csproj"), json_deps: false },
];

fn collect_manifests(node: &FileNode, out: &mut Vec<(String, bool)>) {
    if !node.is_dir {
        for m in MANIFESTS {
            let matched = if let Some(suffix) = m.suffix {
                node.name.to_lowercase().ends_with(suffix)
            } else {
                node.name == m.file_name
            };
            if matched && node.size > 0 && node.size < 512_000 {
                out.push((node.relative_path.clone(), m.json_deps));
            }
        }
        return;
    }
    for child in &node.children {
        collect_manifests(child, out);
    }
}

/// Extract dependency identifiers from a JSON manifest (package.json etc).
fn json_dependency_names(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(text) {
        for key in ["dependencies", "devDependencies", "peerDependencies"] {
            if let Some(map) = v.get(key).and_then(|d| d.as_object()) {
                out.extend(map.keys().cloned());
            }
        }
    }
    out
}

/// Extract Include="..." values from .csproj style XML (lowercased).
fn csproj_includes(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let lower = text.to_lowercase();
    let mut rest = lower.as_str();
    while let Some(pos) = rest.find("include=\"") {
        let start = pos + "include=\"".len();
        if let Some(end) = rest[start..].find('"') {
            out.push(rest[start..start + end].to_string());
            rest = &rest[start + end..];
        } else {
            break;
        }
    }
    out
}

fn match_rules(haystack: &str, source: &str, out: &mut HashMap<String, TechStackItem>) {
    for rule in RULES {
        if haystack.contains(rule.pat) {
            out.entry(rule.name.to_string()).or_insert_with(|| TechStackItem {
                name: rule.name.to_string(),
                category: rule.cat.to_string(),
                source: source.to_string(),
            });
        }
    }
}

fn detect_tech_stack(manifests: &[(String, bool)], root: &Path) -> Vec<TechStackItem> {
    let mut map: HashMap<String, TechStackItem> = HashMap::new();

    for (rel, json_deps) in manifests {
        let Ok(text) = std::fs::read_to_string(root.join(rel)) else {
            continue;
        };

        if *json_deps {
            for dep in json_dependency_names(&text) {
                match_rules(&dep.to_lowercase(), rel, &mut map);
            }
        } else if rel.to_lowercase().ends_with(".csproj") {
            for inc in csproj_includes(&text) {
                match_rules(&inc, rel, &mut map);
            }
        } else {
            match_rules(&text.to_lowercase(), rel, &mut map);
        }
    }

    let mut items: Vec<TechStackItem> = map.into_values().collect();
    items.sort_by(|a, b| a.category.cmp(&b.category).then(a.name.cmp(&b.name)));
    items
}

fn count_file_metrics(path: &Path) -> Result<(usize, usize, usize), String> {
    let mut file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut buf = Vec::new();
    (&mut file)
        .take(METRIC_READ_CAP)
        .read_to_end(&mut buf)
        .map_err(|e| e.to_string())?;
    let text = String::from_utf8_lossy(&buf);
    let mut lines = 0usize;
    let mut code_lines = 0usize;
    let mut todos = 0usize;
    for line in text.lines() {
        lines += 1;
        if !line.trim().is_empty() {
            code_lines += 1;
        }
        let upper = line.trim().to_uppercase();
        if upper.contains("TODO") || upper.contains("FIXME") {
            todos += 1;
        }
    }
    Ok((lines, code_lines, todos))
}

fn collect_code_files(node: &FileNode, out: &mut Vec<FileNode>) {
    if node.is_dir {
        for child in &node.children {
            collect_code_files(child, out);
        }
    } else if node.language.is_some() {
        out.push(node.clone());
    }
}

fn detect_entry_points(files: &[FileNode]) -> Vec<EntryPoint> {
    let _seen: HashSet<()> = HashSet::new();
    let mut out = Vec::new();

    for f in files {
        let depth = f.relative_path.matches('/').count() + 1;
        let name = f.name.to_ascii_lowercase();

        let reason = if matches!(
            name.as_str(),
            "main.rs" | "main.go" | "main.py" | "main.c" | "main.cpp"
        ) {
            Some("程序入口 main")
        } else if name == "manage.py" {
            Some("Django 管理入口")
        } else if matches!(name.as_str(), "app.py" | "app.ts" | "app.js") && depth <= 2 {
            Some("应用入口 app")
        } else if depth <= 2
            && matches!(
                name.as_str(),
                "index.js" | "index.ts" | "index.tsx" | "index.jsx" | "index.vue" | "index.html"
            )
        {
            Some("模块入口 index")
        } else if depth <= 2 && matches!(name.as_str(), "app.tsx" | "app.jsx" | "app.vue") {
            Some("UI 根组件")
        } else {
            None
        };

        if let Some(reason) = reason {
            out.push(EntryPoint {
                relative_path: f.relative_path.clone(),
                reason: reason.to_string(),
            });
        }
    }
    out.sort_by_key(|a| a.relative_path.matches('/').count());
    out.truncate(30);
    out
}

pub fn run_static_analysis(root: &Path) -> Result<StaticReport, String> {
    let scan = scanner::scan_project(root)?;
    let root_name = scan.root_name.clone();

    let mut manifests = Vec::new();
    collect_manifests(&scan.tree, &mut manifests);

    let tech_stack = detect_tech_stack(&manifests, root);

    let mut files = Vec::new();
    collect_code_files(&scan.tree, &mut files);
    let total_code_files = files.len();

    let mut metrics: Vec<FileMetric> = Vec::new();
    let mut total_lines = 0usize;
    let mut total_todos = 0usize;

    for f in files.iter() {
        let full = root.join(&f.relative_path);
        if let Ok((lines, code_lines, todos)) = count_file_metrics(&full) {
            total_lines += lines;
            total_todos += todos;
            metrics.push(FileMetric {
                relative_path: f.relative_path.clone(),
                language: f.language.clone().unwrap_or_default(),
                lines,
                code_lines,
                todos,
            });
        }
    }

    metrics.sort_by(|a, b| b.lines.cmp(&a.lines));
    metrics.truncate(MAX_METRICS);

    let entry_points = detect_entry_points(&files);

    let mut warnings = Vec::new();
    for m in metrics.iter().take(20) {
        if m.lines >= BIG_FILE_LINES {
            warnings.push(format!(
                "`{}` 超过 {} 行，建议考虑拆分",
                m.relative_path, BIG_FILE_LINES
            ));
        }
    }
    if scan.truncated {
        warnings.push("项目文件过多，扫描结果被截断，统计可能不完整".to_string());
    }

    Ok(StaticReport {
        root_name,
        tech_stack,
        entry_points,
        metrics,
        total_code_files,
        total_lines,
        total_todos,
        warnings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(base: &Path, rel: &str, content: &str) {
        let p = base.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, content).unwrap();
    }

    #[test]
    fn test_json_dependency_extraction() {
        let deps = json_dependency_names(
            r#"{"name":"x","dependencies":{"react":"^18","axios":"^1"},"devDependencies":{"vitest":"^1"}}"#,
        );
        assert!(deps.contains(&"react".to_string()));
        assert!(deps.contains(&"vitest".to_string()));
    }

    #[test]
    fn test_csproj_include_extraction() {
        let incs = csproj_includes(
            r#"<Project><ItemGroup><PackageReference Include="Microsoft.EntityFrameworkCore" Version="8" /></ItemGroup></Project>"#,
        );
        assert_eq!(incs.len(), 1);
        assert!(incs[0].contains("entityframeworkcore"));
    }

    #[test]
    fn test_rule_matching() {
        let mut map = HashMap::new();
        match_rules("react-dom axios my-lib", "package.json", &mut map);
        assert!(map.contains_key("React"));
        assert!(map.contains_key("Axios"));
        assert!(!map.contains_key("Vue"));
    }

    #[test]
    fn test_tech_stack_and_entry_points() {
        let base = std::env::temp_dir().join(format!("cs_static_{}", std::process::id()));
        std::fs::create_dir_all(&base).unwrap();
        write(&base, "package.json", r#"{"dependencies":{"react":"^18","vue":"^3"}}"#);
        write(&base, "requirements.txt", "django==5.0\nrequests\n");
        write(
            &base,
            "src/main.rs",
            "// TODO: refactor\nfn main() {\n    println!(\"hi\");\n}\n",
        );

        let scan = scanner::scan_project(&base).unwrap();
        let mut manifests = Vec::new();
        collect_manifests(&scan.tree, &mut manifests);
        assert_eq!(manifests.len(), 2);

        let tech = detect_tech_stack(&manifests, &base);
        let names: Vec<&str> = tech.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"React"));
        assert!(names.contains(&"Vue"));
        assert!(names.contains(&"Django"));

        let mut files = Vec::new();
        collect_code_files(&scan.tree, &mut files);
        let entries = detect_entry_points(&files);
        assert!(entries.iter().any(|e| e.reason.contains("main")));

        let (lines, code_lines, todos) =
            count_file_metrics(&base.join("src").join("main.rs")).unwrap();
        assert_eq!((lines, code_lines, todos), (4, 4, 1));

        std::fs::remove_dir_all(&base).ok();
    }
}
