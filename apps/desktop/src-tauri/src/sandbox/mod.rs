// src-tauri/src/sandbox/mod.rs
// Command whitelist, permission model, and audit logging

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandWhitelist {
    pub allowed_commands: HashSet<String>,
    pub allowed_paths: HashSet<String>,
}

impl Default for CommandWhitelist {
    fn default() -> Self {
        let mut allowed = HashSet::new();
        // Git
        allowed.insert("git".to_string());
        // Package managers
        allowed.insert("npm".to_string());
        allowed.insert("pnpm".to_string());
        allowed.insert("yarn".to_string());
        allowed.insert("cargo".to_string());
        allowed.insert("go".to_string());
        allowed.insert("uv".to_string());
        allowed.insert("pip".to_string());
        // Build tools
        allowed.insert("make".to_string());
        allowed.insert("cmake".to_string());
        allowed.insert("gcc".to_string());
        allowed.insert("g++".to_string());
        allowed.insert("rustc".to_string());
        // Code tools
        allowed.insert("claude".to_string());
        allowed.insert("codex".to_string());
        // File tools
        allowed.insert("cat".to_string());
        allowed.insert("head".to_string());
        allowed.insert("tail".to_string());
        allowed.insert("grep".to_string());
        allowed.insert("find".to_string());
        allowed.insert("ls".to_string());
        allowed.insert("stat".to_string());
        allowed.insert("wc".to_string());
        // Shell
        allowed.insert("bash".to_string());
        allowed.insert("sh".to_string());
        allowed.insert("zsh".to_string());

        let mut allowed_paths = HashSet::new();
        allowed_paths.insert("/usr/bin".to_string());
        allowed_paths.insert("/usr/local/bin".to_string());
        allowed_paths.insert("/opt/homebrew/bin".to_string());
        allowed_paths.insert("/Users".to_string()); // macOS home

        CommandWhitelist {
            allowed_commands: allowed,
            allowed_paths,
        }
    }
}

impl CommandWhitelist {
    /// Check if a command is allowed to be executed
    pub fn is_allowed(&self, cmd: &str) -> bool {
        self.allowed_commands.contains(cmd)
    }

    /// Check if a path is allowed for file operations
    pub fn path_allowed(&self, path: &str) -> bool {
        for allowed in &self.allowed_paths {
            if path.starts_with(allowed) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: String,
    pub command: String,
    pub args: Vec<String>,
    pub allowed: bool,
    pub reason: Option<String>,
}

pub struct Sandbox {
    whitelist: CommandWhitelist,
}

impl Default for Sandbox {
    fn default() -> Self {
        Self::new()
    }
}

impl Sandbox {
    pub fn new() -> Self {
        Sandbox {
            whitelist: CommandWhitelist::default(),
        }
    }

    pub fn check_command(&self, cmd: &str) -> Result<(), String> {
        if self.whitelist.is_allowed(cmd) {
            Ok(())
        } else {
            Err(format!("Command '{}' is not in the whitelist. Allowed: {:?}", cmd, self.whitelist.allowed_commands))
        }
    }

    pub fn check_path(&self, path: &str) -> Result<(), String> {
        if self.whitelist.path_allowed(path) {
            Ok(())
        } else {
            Err(format!("Path '{}' is not in the allowed paths list.", path))
        }
    }

    pub fn audit_log(&self, entry: AuditEntry) {
        let status = if entry.allowed { "ALLOWED" } else { "BLOCKED" };
        tracing::info!(
            "[AUDIT] {} | cmd={} args={:?} | reason={:?}",
            status,
            entry.command,
            entry.args,
            entry.reason
        );
    }
}