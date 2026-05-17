// src-tauri/src/orchestrator/session.rs
// Session orchestrator — manages Agent session lifecycle

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;
use tokio::sync::mpsc;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartSessionInput {
    pub project_id: String,
    pub backend_id: String,
    pub model: String,
    pub context_pack_json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AgentEvent {
    Chunk { text: String },
    ToolCall { tool: String, args: String },
    Verification { task_id: String, passed: bool },
    TokenUpdate { input_tokens: i64, output_tokens: i64 },
    Error { message: String },
}

pub struct SessionOrchestrator {
    active_sessions: HashMap<String, SessionHandle>,
}

struct SessionHandle {
    project_id: String,
    backend_id: String,
    pid: Option<u32>,
}

impl SessionOrchestrator {
    pub fn new() -> Self {
        SessionOrchestrator {
            active_sessions: HashMap::new(),
        }
    }

    /// Start a Claude Code CLI session
    pub async fn start_claude_code(&mut self, input: StartSessionInput) -> Result<String, String> {
        let session_id = uuid::Uuid::new_v4().to_string();

        info!("Starting Claude Code session {} for project {}", session_id, input.project_id);

        // Build the context pack as a temp file for Claude Code to read
        let context_path = format!("/tmp/vibepilot_context_{}.json", session_id);
        tokio::fs::write(&context_path, &input.context_pack_json)
            .await
            .map_err(|e| format!("Failed to write context: {}", e))?;

        // Spawn claude command with --print flag for non-interactive
        let child = Command::new("claude")
            .args([
                "--no-input",
                "--output-format",
                "json-stream",
                "--system-prompts-file",
                &context_path,
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn claude: {}", e))?;

        let pid = child.id();
        self.active_sessions.insert(session_id.clone(), SessionHandle {
            project_id: input.project_id,
            backend_id: input.backend_id,
            pid,
        });

        Ok(session_id)
    }

    /// Send a turn to the active session
    pub async fn send_turn(&self, session_id: &str, message: &str) -> Result<Vec<AgentEvent>, String> {
        // In a real implementation, this would write to the session's stdin
        // and read from stdout to build AgentEvent stream
        warn!("send_turn not fully implemented — needs pty/stdin management");
        Ok(vec![])
    }

    /// Cancel an active session
    pub async fn cancel(&mut self, session_id: &str) -> Result<(), String> {
        if let Some(handle) = self.active_sessions.remove(session_id) {
            if let Some(pid) = handle.pid {
                // On Unix, kill the process group
                #[cfg(unix)]
                {
                    let _ = Command::new("kill").arg("-9").arg(pid.to_string()).spawn();
                }
                #[cfg(windows)]
                {
                    let _ = Command::new("taskkill").arg("/F").arg("/PID").arg(pid.to_string()).spawn();
                }
            }
            info!("Cancelled session {}", session_id);
        }
        Ok(())
    }
}

impl Default for SessionOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}