// src-tauri/src/storage/schema.rs
// Domain types matching the SQLite schema

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: String,
    pub repo_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub last_session_at: Option<String>,
    pub total_token_cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningCard {
    pub id: String,
    pub project_id: String,
    pub card_type: String,  // "failed_attempt" | "success_path" | "root_cause" | "verification_evidence"
    pub title: String,
    pub trigger: String,
    pub body: String,
    pub token_cost_usd: Option<f64>,
    pub confirmed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLearningCard {
    pub project_id: String,
    pub card_type: String,
    pub title: String,
    pub trigger: String,
    pub body: String,
    pub token_cost_usd: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub project_id: String,
    pub backend_id: String,   // "claude-code" | "codex" | "gemini" | "ollama"
    pub model: String,
    pub workflow_stage: String,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub total_token_input: i64,
    pub total_token_output: i64,
    pub total_token_cached: i64,
    pub total_cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub project_id: String,
    pub session_id: Option<String>,
    pub title: String,
    pub description: String,
    pub status: String,  // "pending" | "in_progress" | "completed" | "blocked"
    pub acceptance_criteria: String,  // JSON array
    pub verification_method: String,
    pub priority: String,
    pub dependencies: String,  // JSON array
    pub phase: String,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTask {
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub acceptance_criteria: String,
    pub verification_method: String,
    pub priority: String,
    pub phase: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verification {
    pub id: String,
    pub task_id: String,
    pub vtype: String,  // "test" | "lint" | "typecheck" | "build" | "manual"
    pub passed: bool,
    pub output: String,
    pub duration_ms: Option<i64>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVerification {
    pub task_id: String,
    pub vtype: String,
    pub passed: bool,
    pub output: String,
    pub duration_ms: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub id: String,
    pub session_id: Option<String>,
    pub project_id: String,
    pub model: String,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub cached_tokens: i64,
    pub reasoning_tokens: i64,
    pub cost_usd: f64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTokenUsage {
    pub session_id: Option<String>,
    pub project_id: String,
    pub model: String,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub cached_tokens: i64,
    pub reasoning_tokens: i64,
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostSummary {
    pub total_input_tokens: i64,
    pub total_output_tokens: i64,
    pub total_cached_tokens: i64,
    pub total_cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRules {
    pub agents_md: Option<String>,
    pub claude_md: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectScanResult {
    pub name: String,
    pub path: String,
    pub languages: Vec<String>,
    pub package_managers: Vec<String>,
    pub test_commands: Vec<String>,
    pub build_commands: Vec<String>,
    pub has_agents_md: bool,
    pub has_claude_md: bool,
    pub agents_md_content: Option<String>,
    pub claude_md_content: Option<String>,
    pub directory_tree: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextPack {
    pub project_id: String,
    pub session_id: Option<String>,
    pub rules_content: String,
    pub learning_cards: Vec<LearningCard>,
    pub repo_map_summary: String,
    pub active_tasks: Vec<Task>,
    pub total_tokens_estimate: i64,
    pub components: Vec<ContextPackComponent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextPackComponent {
    pub name: String,
    pub token_estimate: i64,
    pub content_preview: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutcomeRecord {
    pub project_id: String,
    pub task_id: String,
    pub outcome: String,  // "success" | "failed" | "partial"
    pub details: String,
}