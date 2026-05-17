// src-tauri/src/commands/scanner.rs
// Project scanning commands — reads filesystem and generates project profile

use serde::{Deserialize, Serialize};
use std::path::Path;

const MAX_TREE_DEPTH: usize = 3;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectScanResult {
    pub name: String,
    pub path: String,
    pub languages: Vec<String>,
    pub package_managers: Vec<String>,
    pub test_commands: Vec<String>,
    pub build_commands: Vec<String>,
    pub has_agents_md: bool,
    pub has_claude_md: bool,
    #[serde(default)]
    pub agents_md_content: Option<String>,
    #[serde(default)]
    pub claude_md_content: Option<String>,
    pub directory_tree: Vec<String>,
}

/// Scan a local directory and generate a project profile
pub fn scan_project(path_str: &str) -> Result<ProjectScanResult, String> {
    let path = Path::new(path_str);
    if !path.exists() {
        return Err(format!("Path does not exist: {}", path.display()));
    }
    if !path.is_dir() {
        return Err(format!("Path is not a directory: {}", path.display()));
    }

    let project_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let languages = detect_languages(path);
    let package_managers = detect_package_managers(path);
    let test_commands = detect_test_commands(path, &package_managers);
    let build_commands = detect_build_commands(path, &package_managers);

    let (has_agents_md, agents_md_content) = read_file(path, "AGENTS.md");
    let (has_claude_md, claude_md_content) = read_file(path, "CLAUDE.md");
    let directory_tree = generate_dir_tree(path, 0);

    println!("Scanned project '{}': {} languages, {} pkg managers",
        project_name, languages.len(), package_managers.len());

    Ok(ProjectScanResult {
        name: project_name,
        path: path.to_string_lossy().to_string(),
        languages,
        package_managers,
        test_commands,
        build_commands,
        has_agents_md,
        has_claude_md,
        agents_md_content,
        claude_md_content,
        directory_tree,
    })
}

fn detect_languages(path: &Path) -> Vec<String> {
    let mut languages = Vec::new();
    let mut seen = std::collections::HashSet::new();

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let lang = match ext {
                "rs" => "Rust",
                "ts" | "tsx" => "TypeScript",
                "js" | "jsx" | "mjs" => "JavaScript",
                "py" => "Python",
                "go" => "Go",
                "java" => "Java",
                "kt" => "Kotlin",
                "swift" => "Swift",
                "cs" => "C#",
                "cpp" | "cc" | "cxx" => "C++",
                "c" | "h" => "C",
                "rb" => "Ruby",
                "php" => "PHP",
                "ex" | "exs" => "Elixir",
                "tf" => "Terraform",
                "sh" | "bash" => "Shell",
                "css" | "scss" => "CSS",
                "html" => "HTML",
                "yaml" | "yml" => "YAML",
                "json" => "JSON",
                "toml" => "TOML",
                "md" => "Markdown",
                _ => continue,
            };
            if !seen.contains(lang) {
                seen.insert(lang.to_string());
                languages.push(lang.to_string());
            }
        }
    }

    languages
}

fn detect_package_managers(path: &Path) -> Vec<String> {
    let mut managers = Vec::new();
    if path.join("package.json").exists() { managers.push("npm".to_string()); }
    if path.join("pnpm-lock.yaml").exists() { managers.push("pnpm".to_string()); }
    if path.join("yarn.lock").exists() { managers.push("yarn".to_string()); }
    if path.join("Cargo.toml").exists() { managers.push("cargo".to_string()); }
    if path.join("go.mod").exists() { managers.push("go".to_string()); }
    if path.join("pom.xml").exists() { managers.push("maven".to_string()); }
    if path.join("build.gradle").exists() { managers.push("gradle".to_string()); }
    if path.join("Gemfile").exists() { managers.push("bundler".to_string()); }
    if path.join("requirements.txt").exists() || path.join("pyproject.toml").exists() { managers.push("pip".to_string()); }
    managers
}

fn detect_test_commands(path: &Path, managers: &[String]) -> Vec<String> {
    let mut cmds = Vec::new();
    if managers.contains(&"npm".to_string()) {
        if let Ok(content) = std::fs::read_to_string(path.join("package.json")) {
            if content.contains("\"test\"") { cmds.push("npm test".to_string()); }
            if content.contains("\"test:e2e\"") { cmds.push("npm run test:e2e".to_string()); }
        }
    }
    if managers.contains(&"cargo".to_string()) {
        if path.join("Cargo.toml").is_file() { cmds.push("cargo test".to_string()); }
    }
    if managers.contains(&"go".to_string()) {
        if path.join("go.mod").is_file() { cmds.push("go test ./...".to_string()); }
    }
    cmds
}

fn detect_build_commands(path: &Path, managers: &[String]) -> Vec<String> {
    let mut cmds = Vec::new();
    if managers.contains(&"npm".to_string()) {
        if let Ok(content) = std::fs::read_to_string(path.join("package.json")) {
            if content.contains("\"build\"") { cmds.push("npm run build".to_string()); }
        }
    }
    if managers.contains(&"cargo".to_string()) {
        cmds.push("cargo build".to_string());
    }
    if managers.contains(&"go".to_string()) {
        cmds.push("go build ./...".to_string());
    }
    cmds
}

fn read_file(dir: &Path, filename: &str) -> (bool, Option<String>) {
    let path = dir.join(filename);
    if path.is_file() {
        match std::fs::read_to_string(&path) {
            Ok(content) => return (true, Some(content)),
            Err(e) => eprintln!("Failed to read {}: {}", path.display(), e),
        }
    }
    (false, None)
}

fn generate_dir_tree(dir: &Path, depth: usize) -> Vec<String> {
    let mut lines = Vec::new();
    if depth >= MAX_TREE_DEPTH { return lines; }

    let entries: Vec<_> = match std::fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .filter(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                !name.starts_with('.') && name != "node_modules" && name != "target"
                    && name != "__pycache__" && name != "dist" && name != "build"
            })
            .collect(),
        Err(_) => return lines,
    };

    for entry in entries.iter().take(20) {
        let name = entry.file_name().to_string_lossy().to_string();
        let prefix = "  ".repeat(depth);
        if entry.path().is_dir() {
            lines.push(format!("{}+{}/", prefix, name));
            let sub = generate_dir_tree(&entry.path(), depth + 1);
            lines.extend(sub);
        } else {
            lines.push(format!("{}{}", prefix, name));
        }
    }

    lines
}