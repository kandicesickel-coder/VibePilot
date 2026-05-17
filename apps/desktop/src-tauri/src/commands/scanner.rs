// src-tauri/src/commands/scanner.rs
// Project scanning commands — reads filesystem and generates project profile

use crate::storage::schema::ProjectScanResult;
use std::path::Path;
use std::process::Command;
use tracing::{info, warn};

const MAX_TREE_DEPTH: usize = 3;

/// Scan a local directory and generate a project profile
#[tauri::command]
pub async fn scan_project(path: String) -> Result<ProjectScanResult, String> {
    let path = Path::new(&path);
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

    // Detect languages by file extensions
    let languages = detect_languages(path);

    // Detect package managers
    let package_managers = detect_package_managers(path);

    // Detect test/build commands
    let test_commands = detect_test_commands(path, &package_managers);
    let build_commands = detect_build_commands(path, &package_managers);

    // Read AGENTS.md and CLAUDE.md
    let (has_agents_md, agents_md_content) = read_file(path, "AGENTS.md");
    let (has_claude_md, claude_md_content) = read_file(path, "CLAUDE.md");

    // Generate directory tree (limited depth)
    let directory_tree = generate_dir_tree(path, 0);

    info!("Scanned project '{}': {} languages, {} pkg managers", project_name, languages.len(), package_managers.len());

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
            let ext = entry.path().extension().and_then(|e| e.to_str()).unwrap_or("");
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
        if path.join("package.json").is_file() {
            if let Ok(content) = std::fs::read_to_string(path.join("package.json")) {
                if content.contains("\"test\"") { cmds.push("npm test".to_string()); }
                if content.contains("\"test:e2e\"") { cmds.push("npm run test:e2e".to_string()); }
            }
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
        if path.join("package.json").is_file() {
            if let Ok(content) = std::fs::read_to_string(path.join("package.json")) {
                if content.contains("\"build\"") { cmds.push("npm run build".to_string()); }
            }
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
            Err(e) => warn!("Failed to read {}: {}", path.display(), e),
        }
    }
    (false, None)
}

fn generate_dir_tree(dir: &Path, depth: usize) -> Vec<String> {
    let mut lines = Vec::new();
    if depth >= MAX_TREE_DEPTH { return lines; }

    let entries: Vec<_> = std::fs::read_dir(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            !name.starts_with('.') && name != "node_modules" && name != "target" && name != "__pycache__" && name != "dist" && name != "build"
        })
        .collect();

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