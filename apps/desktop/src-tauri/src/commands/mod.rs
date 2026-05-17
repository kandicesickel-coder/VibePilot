// src-tauri/src/commands/mod.rs
// Tauri commands — the public API exposed to the React frontend

pub mod scanner;

use crate::AppState;
use crate::storage::schema::*;
use crate::storage::repo::Database as Db;
use tauri::State;
use std::sync::Arc;
use tokio::sync::Mutex;

type StateDb = Arc<Mutex<Db>>;

fn with_db<F, T>(state: &State<AppState>, f: F) -> Result<T, String>
where
    F: FnOnce(&Db) -> Result<T, String>,
{
    // Run blocking DB operation in a blocking context
    let guard = state.db.blocking_lock();
    f(&guard).map_err(|e| e.to_string())
}

// ── Project Commands ─────────────────────────────────────────────────────────

#[tauri::command]
pub async fn create_project(
    name: String,
    path: String,
    repo_url: Option<String>,
    state: State<'_, AppState>,
) -> Result<Project, String> {
    let guard = state.db.lock().await;
    guard.create_project(&name, &path, repo_url.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_projects(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    let guard = state.db.lock().await;
    guard.list_projects().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_project(id: String, state: State<'_, AppState>) -> Result<Project, String> {
    let guard = state.db.lock().await;
    guard.get_project(&id).map_err(|e| e.to_string())
}

// ── Session Commands ─────────────────────────────────────────────────────────

#[tauri::command]
pub async fn create_session(
    project_id: String,
    backend_id: String,
    model: String,
    workflow_stage: String,
    state: State<'_, AppState>,
) -> Result<Session, String> {
    let guard = state.db.lock().await;
    guard.create_session(&project_id, &backend_id, &model, &workflow_stage)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_session(id: String, state: State<'_, AppState>) -> Result<Session, String> {
    let guard = state.db.lock().await;
    guard.get_session(&id).map_err(|e| e.to_string())
}

// ── Learning Card Commands ───────────────────────────────────────────────────

#[tauri::command]
pub async fn create_learning_card(
    card: CreateLearningCard,
    state: State<'_, AppState>,
) -> Result<LearningCard, String> {
    let guard = state.db.lock().await;
    guard.create_learning_card(&card).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_learning_cards(
    project_id: String,
    confirmed_only: bool,
    state: State<'_, AppState>,
) -> Result<Vec<LearningCard>, String> {
    let guard = state.db.lock().await;
    guard.list_learning_cards(&project_id, confirmed_only)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn confirm_learning_card(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let guard = state.db.lock().await;
    guard.confirm_learning_card(&id).map_err(|e| e.to_string())
}

// ── Memory Search (MCP tools) ─────────────────────────────────────────────────

#[tauri::command]
pub async fn search_memory(
    project_id: String,
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<LearningCard>, String> {
    let guard = state.db.lock().await;
    guard.search_learning_cards(&project_id, &query).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_context_pack(
    project_id: String,
    task_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<ContextPack, String> {
    let guard = state.db.lock().await;

    // Fetch confirmed learning cards
    let cards = guard.list_learning_cards(&project_id, true).map_err(|e| e.to_string())?;

    // Fetch active tasks
    let tasks = guard.list_tasks(&project_id).map_err(|e| e.to_string())?;
    let active_tasks: Vec<Task> = tasks.into_iter()
        .filter(|t| t.status == "pending" || t.status == "in_progress")
        .take(5)
        .collect();

    // Fetch project rules
    let rules = guard.get_project_rules(&project_id).map_err(|e| e.to_string())?;
    let rules_content = format!(
        "{}\n\n{}",
        rules.agents_md.as_deref().unwrap_or(""),
        rules.claude_md.as_deref().unwrap_or("")
    );

    // Estimate tokens (rough: 4 chars per token)
    let rules_tokens = (rules_content.len() as i64) / 4;
    let cards_tokens: i64 = cards.iter().map(|c| (c.body.len() as i64) / 4).sum();
    let total = rules_tokens + cards_tokens + (active_tasks.len() as i64) * 50;

    Ok(ContextPack {
        project_id,
        session_id: None,
        rules_content,
        learning_cards: cards,
        repo_map_summary: String::new(),
        active_tasks,
        total_tokens_estimate: total,
        components: vec![
            ContextPackComponent {
                name: "rules".to_string(),
                token_estimate: rules_tokens,
                content_preview: format!("{} chars", rules_content.len()),
            },
            ContextPackComponent {
                name: "learning_cards".to_string(),
                token_estimate: cards_tokens,
                content_preview: format!("{} cards", cards.len()),
            },
        ],
    })
}

#[tauri::command]
pub async fn record_outcome(
    record: OutcomeRecord,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let guard = state.db.lock().await;

    // Generate a Learning Card from the outcome
    let card_type = match record.outcome.as_str() {
        "success" => "success_path",
        "failed" => "failed_attempt",
        _ => "root_cause",
    };

    let card = CreateLearningCard {
        project_id: record.project_id.clone(),
        card_type: card_type.to_string(),
        title: format!("Task {}: {}", record.task_id, record.outcome),
        trigger: record.details.chars().take(200).collect(),
        body: record.details,
        token_cost_usd: None,
    };

    guard.create_learning_card(&card).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_project_rules(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<ProjectRules, String> {
    let guard = state.db.lock().await;
    guard.get_project_rules(&project_id).map_err(|e| e.to_string())
}

// ── Task Commands ───────────────────────────────────────────────────────────

#[tauri::command]
pub async fn create_task(
    task: CreateTask,
    state: State<'_, AppState>,
) -> Result<Task, String> {
    let guard = state.db.lock().await;
    guard.create_task(&task).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_tasks(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Task>, String> {
    let guard = state.db.lock().await;
    guard.list_tasks(&project_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_task_status(
    id: String,
    status: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let guard = state.db.lock().await;
    guard.update_task_status(&id, &status).map_err(|e| e.to_string())
}

// ── Verification Commands ────────────────────────────────────────────────────

#[tauri::command]
pub async fn create_verification(
    v: CreateVerification,
    state: State<'_, AppState>,
) -> Result<Verification, String> {
    let guard = state.db.lock().await;
    guard.create_verification(&v).map_err(|e| e.to_string())
}

// ── Token / Cost Commands ────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_token_usage(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<TokenUsage>, String> {
    let guard = state.db.lock().await;
    guard.get_token_usage(&project_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_cost_summary(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<CostSummary, String> {
    let guard = state.db.lock().await;
    guard.get_cost_summary(&project_id).map_err(|e| e.to_string())
}

// ── Project Rules Scan ───────────────────────────────────────────────────────

#[tauri::command]
pub async fn scan_agents_md(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<ProjectRules, String> {
    let guard = state.db.lock().await;
    guard.get_project_rules(&project_id).map_err(|e| e.to_string())
}

// ── Tauri Info ───────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_tauri_info() -> Result<String, String> {
    Ok(format!(
        "VibePilot v{} | Tauri {} | Rust {}",
        env!("CARGO_PKG_VERSION"),
        tauri::VERSION,
        rustc_version::version().unwrap_or_else(|_| "unknown".to_string())
    ))
}