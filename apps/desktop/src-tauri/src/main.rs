// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod storage;
mod types;

use std::sync::{Arc, Mutex};

pub struct AppState {
    pub db: Arc<Mutex<storage::Database>>,
}

fn main() {
    println!("Starting VibePilot Desktop v0.1.0");

    let db = match storage::Database::new() {
        Ok(db) => Arc::new(Mutex::new(db)),
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            std::process::exit(1);
        }
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .manage(AppState { db })
        .invoke_handler(tauri::generate_handler![
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
            commands::scan_project,
        ])
        .setup(|_app| {
            println!("VibePilot Desktop initialized successfully");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}