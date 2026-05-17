// src-tauri/src/types.rs
// Shared data types — no storage dependency to avoid circular imports

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: String,
    pub repo_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub last_session_at: Option<String>,
    #[serde(default)]
    pub total_token_cost_usd: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LearningCard {
    pub id: String,
    pub project_id: String,
    pub card_type: String,
    pub title: String,
    pub trigger: String,
    pub body: String,
    #[serde(default)]
    pub token_cost_usd: Option<f64>,
    #[serde(default)]
    pub confirmed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateLearningCard {
    pub project_id: String,
    pub card_type: String,
    pub title: String,
    pub trigger: String,
    pub body: String,
    #[serde(default)]
    pub token_cost_usd: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub id: String,
    pub project_id: String,
    pub backend_id: String,
    pub model: String,
    pub workflow_stage: String,
    pub started_at: String,
    #[serde(default)]
    pub finished_at: Option<String>,
    #[serde(default)]
    pub total_token_input: i64,
    #[serde(default)]
    pub total_token_output: i64,
    #[serde(default)]
    pub total_token_cached: i64,
    #[serde(default)]
    pub total_cost_usd: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub project_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    pub title: String,
    pub description: String,
    pub status: String,
    #[serde(default)]
    pub acceptance_criteria: String,
    #[serde(default)]
    pub verification_method: String,
    #[serde(default)]
    pub priority: String,
    #[serde(default)]
    pub dependencies: String,
    #[serde(default)]
    pub phase: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub completed_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTask {
    pub project_id: String,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub acceptance_criteria: String,
    #[serde(default)]
    pub verification_method: String,
    #[serde(default = "default_priority")]
    pub priority: String,
    #[serde(default)]
    pub phase: String,
}

fn default_priority() -> String { "medium".to_string() }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Verification {
    pub id: String,
    pub task_id: String,
    pub vtype: String,
    pub passed: bool,
    #[serde(default)]
    pub output: String,
    #[serde(default)]
    pub duration_ms: Option<i64>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateVerification {
    pub task_id: String,
    pub vtype: String,
    pub passed: bool,
    #[serde(default)]
    pub output: String,
    #[serde(default)]
    pub duration_ms: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenUsage {
    pub id: String,
    pub session_id: Option<String>,
    pub project_id: String,
    pub model: String,
    pub input_tokens: i64,
    pub output_tokens: i64,
    #[serde(default)]
    pub cached_tokens: i64,
    #[serde(default)]
    pub reasoning_tokens: i64,
    pub cost_usd: f64,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTokenUsage {
    pub session_id: Option<String>,
    pub project_id: String,
    pub model: String,
    pub input_tokens: i64,
    pub output_tokens: i64,
    #[serde(default)]
    pub cached_tokens: i64,
    #[serde(default)]
    pub reasoning_tokens: i64,
    pub cost_usd: f64,
}

#[derive(Debug, Serialize)]
pub struct CostSummary {
    pub total_input_tokens: i64,
    pub total_output_tokens: i64,
    pub total_cached_tokens: i64,
    pub total_cost_usd: f64,
}

#[derive(Debug, Serialize)]
pub struct ProjectRules {
    #[serde(default)]
    pub agents_md: Option<String>,
    #[serde(default)]
    pub claude_md: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TauriInfo {
    pub version: String,
    pub tauri_version: String,
}