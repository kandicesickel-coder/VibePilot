// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod orchestrator;
mod sandbox;
mod storage;

use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;
use tracing::info;
use tracing_subscriber;

pub struct AppState {
    pub db: Arc<Mutex<storage::Database>>,
}

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter("vibepilot=info")
        .init();

    info!("Starting VibePilot Desktop v0.1.0");

    // Initialize database
    let db = storage::Database::new().expect("Failed to initialize database");
    let db = Arc::new(Mutex::new(db));

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .manage(AppState { db })
        .invoke_handler(tauri::generate_handler![
            commands::scan_project,
            commands::create_project,
            commands::list_projects,
            commands::get_project,
            commands::create_session,
            commands::get_session,
            commands::create_learning_card,
            commands::list_learning_cards,
            commands::confirm_learning_card,
            commands::search_memory,
            commands::get_context_pack,
            commands::record_outcome,
            commands::get_project_rules,
            commands::create_task,
            commands::list_tasks,
            commands::update_task_status,
            commands::create_verification,
            commands::get_token_usage,
            commands::get_cost_summary,
            commands::scan_agents_md,
            commands::get_tauri_info,
        ])
        .setup(|app| {
            info!("VibePilot Desktop initialized successfully");

            // Set up panic hook for logging
            let app_handle = app.handle().clone();
            std::panic::set_hook(Box::new(move |panic_info| {
                tracing::error!("PANIC: {}", panic_info);
                let _ = app_handle.emit("vibepilot:panic", panic_info.to_string());
            }));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}