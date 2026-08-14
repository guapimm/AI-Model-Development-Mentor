// Generates embed_gen.rs in OUT_DIR with all prompt files embedded via include_str!.
// Content lives in ../cli/files (synced from the language dirs by cli/scripts/sync-embed.ps1).
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let files_root = Path::new(&manifest).join("../cli/files");
    let out = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out).join("embed_gen.rs");

    let mut items = Vec::new();
    for lang in fs::read_dir(&files_root).expect("cli/files missing - run cli/scripts/sync-embed.ps1 first") {
        let lang = lang.unwrap();
        if !lang.path().is_dir() {
            continue;
        }
        let lang_name = lang.file_name().to_string_lossy().to_string();
        for file in fs::read_dir(lang.path()).unwrap() {
            let file = file.unwrap();
            let name = file.file_name().to_string_lossy().to_string();
            let rel = format!("{}/{}", lang_name, name);
            let abs = files_root.join(&rel).canonicalize().unwrap();
            let abs_s = abs.to_string_lossy().replace('\\', "/");
            items.push(format!("(\"{}\", include_str!(r\"{}\")),", rel, abs_s));
        }
    }

    let mut langs: Vec<String> = fs::read_dir(&files_root)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    langs.sort();

    let code = format!(
        "pub const FILES: &[(&str, &str)] = &[\n{}\n];\n\npub const LANG_DIRS: &[&str] = &[{}];\n",
        items.join("\n"),
        langs.iter().map(|l| format!("\"{}\"", l)).collect::<Vec<_>>().join(", ")
    );
    fs::write(&dest, code).unwrap();
    println!("cargo:rerun-if-changed=../cli/files");
}
