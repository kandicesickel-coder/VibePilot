// src-tauri/src/commands/mod.rs
// Tauri commands — the public API exposed to the React frontend

pub mod scanner;

use crate::storage;
use crate::types::*;
use crate::AppState;
use tauri::State;

fn with_db<T>(
    state: &State<'_, AppState>,
    f: impl FnOnce(&storage::Database) -> Result<T, storage::StorageError>,
) -> Result<T, String> {
    let guard = state.db.lock().unwrap();
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
    with_db(&state, |db| db.create_project(&name, &path, repo_url.as_deref()))
}

#[tauri::command]
pub async fn list_projects(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    with_db(&state, |db| db.list_projects())
}

#[tauri::command]
pub async fn get_project(id: String, state: State<'_, AppState>) -> Result<Project, String> {
    with_db(&state, |db| db.get_project(&id))
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
    with_db(&state, |db| {
        db.create_session(&project_id, &backend_id, &model, &workflow_stage)
    })
}

#[tauri::command]
pub async fn get_session(id: String, state: State<'_, AppState>) -> Result<Session, String> {
    with_db(&state, |db| db.get_session(&id))
}

// ── Learning Card Commands ───────────────────────────────────────────────────

#[tauri::command]
pub async fn create_learning_card(
    card: CreateLearningCard,
    state: State<'_, AppState>,
) -> Result<LearningCard, String> {
    with_db(&state, |db| db.create_learning_card(&card))
}

#[tauri::command]
pub async fn list_learning_cards(
    project_id: String,
    confirmed_only: bool,
    state: State<'_, AppState>,
) -> Result<Vec<LearningCard>, String> {
    with_db(&state, |db| db.list_learning_cards(&project_id, confirmed_only))
}

#[tauri::command]
pub async fn confirm_learning_card(id: String, state: State<'_, AppState>) -> Result<(), String> {
    with_db(&state, |db| db.confirm_learning_card(&id))
}

// ── Memory Search (MCP tools) ────────────────────────────────────────────────

#[tauri::command]
pub async fn search_memory(
    project_id: String,
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<LearningCard>, String> {
    with_db(&state, |db| db.search_learning_cards(&project_id, &query))
}

#[tauri::command]
pub async fn get_context_pack(
    project_id: String,
    session_id: String,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    with_db(&state, |db| {
        let rules = db.get_project_rules(&project_id)?;
        let cards = db.list_learning_cards(&project_id, true)?;
        let tasks = db.list_tasks(&project_id)?;
        Ok(serde_json::json!({
            "project_id": project_id,
            "session_id": session_id,
            "rules": rules,
            "learning_cards": cards,
            "active_tasks": tasks.into_iter().filter(|t| t.status != "completed").collect::<Vec<_>>()
        }))
    })
}

#[tauri::command]
pub async fn record_outcome(
    project_id: String,
    session_id: String,
    outcome: serde_json::Value,
    state: State<'_, AppState>,
) -> Result<(), String> {
    with_db(&state, |db| {
        let title = outcome.get("title").and_then(|v| v.as_str()).unwrap_or("Untitled");
        let body = serde_json::to_string(&outcome).unwrap_or_default();
        let card = CreateLearningCard {
            project_id: project_id.clone(),
            card_type: "outcome".to_string(),
            title: title.to_string(),
            trigger: format!("session {} completed", session_id),
            body,
            token_cost_usd: None,
        };
        db.create_learning_card(&card)?;
        Ok(())
    })
}

#[tauri::command]
pub async fn get_project_rules(project_id: String, state: State<'_, AppState>) -> Result<ProjectRules, String> {
    with_db(&state, |db| db.get_project_rules(&project_id))
}

// ── Task Commands ───────────────────────────────────────────────────────────

#[tauri::command]
pub async fn create_task(task: CreateTask, state: State<'_, AppState>) -> Result<Task, String> {
    with_db(&state, |db| db.create_task(&task))
}

#[tauri::command]
pub async fn list_tasks(project_id: String, state: State<'_, AppState>) -> Result<Vec<Task>, String> {
    with_db(&state, |db| db.list_tasks(&project_id))
}

#[tauri::command]
pub async fn update_task_status(id: String, status: String, state: State<'_, AppState>) -> Result<(), String> {
    with_db(&state, |db| db.update_task_status(&id, &status))
}

// ── Verification Commands ───────────────────────────────────────────────────

#[tauri::command]
pub async fn create_verification(
    v: CreateVerification,
    state: State<'_, AppState>,
) -> Result<Verification, String> {
    with_db(&state, |db| db.create_verification(&v))
}

// ── Token Usage Commands ────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_token_usage(project_id: String, state: State<'_, AppState>) -> Result<Vec<TokenUsage>, String> {
    with_db(&state, |db| db.get_token_usage(&project_id))
}

#[tauri::command]
pub async fn get_cost_summary(project_id: String, state: State<'_, AppState>) -> Result<CostSummary, String> {
    with_db(&state, |db| db.get_cost_summary(&project_id))
}

// ── Agent Rules Scan ───────────────────────────────────────────────────────

#[tauri::command]
pub async fn scan_agents_md(project_id: String, state: State<'_, AppState>) -> Result<ProjectRules, String> {
    with_db(&state, |db| {
        let project = db.get_project(&project_id)?;
        let path = std::path::Path::new(&project.path);
        let mut agents_md = None;
        let mut claude_md = None;
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name == "AGENTS.md" || name == ".claude.md" {
                    if let Ok(content) = std::fs::read_to_string(entry.path()) {
                        if name == "AGENTS.md" {
                            agents_md = Some(content);
                        } else {
                            claude_md = Some(content);
                        }
                    }
                }
            }
        }
        if agents_md.is_some() || claude_md.is_some() {
            db.save_project_rules(&project_id, agents_md.as_deref(), claude_md.as_deref())?;
        }
        Ok(ProjectRules { agents_md, claude_md })
    })
}

// ── Info ─────────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_tauri_info() -> Result<TauriInfo, String> {
    Ok(TauriInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        tauri_version: "2".to_string(),
    })
}

// ── Scan Project (from scanner module) ─────────────────────────────────────

#[tauri::command]
pub async fn scan_project(path: String, _state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let result = scanner::scan_project(&path).map_err(|e| e.to_string())?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}