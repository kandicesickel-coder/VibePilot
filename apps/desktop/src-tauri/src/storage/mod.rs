// src-tauri/src/storage/mod.rs
// SQLite storage layer via rusqlite

use crate::types::*;
use rusqlite::{params, Connection};
use std::path::PathBuf;
use thiserror::Error;
use uuid::Uuid;
use chrono::Utc;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self, StorageError> {
        let db_path = Self::db_path()?;
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(&db_path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let db = Database { conn };
        db.init_schema()?;
        Ok(db)
    }

    fn db_path() -> Result<PathBuf, StorageError> {
        let base = dirs::data_local_dir()
            .ok_or_else(|| StorageError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound, "Could not find data directory")))?;
        Ok(base.join("VibePilot").join("data.db"))
    }

    fn init_schema(&self) -> Result<(), StorageError> {
        self.conn.execute_batch(r#"
            CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                path TEXT NOT NULL UNIQUE,
                repo_url TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                last_session_at TEXT,
                total_token_cost_usd REAL DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS project_facts (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                category TEXT NOT NULL,
                content TEXT NOT NULL,
                source TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS learning_cards (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                type TEXT NOT NULL,
                title TEXT NOT NULL,
                trigger_condition TEXT NOT NULL,
                body TEXT NOT NULL,
                token_cost_usd REAL,
                confirmed_at TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                backend_id TEXT NOT NULL,
                model TEXT NOT NULL,
                workflow_stage TEXT NOT NULL DEFAULT 'spec',
                started_at TEXT NOT NULL,
                finished_at TEXT,
                total_token_input INTEGER DEFAULT 0,
                total_token_output INTEGER DEFAULT 0,
                total_token_cached INTEGER DEFAULT 0,
                total_cost_usd REAL DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS token_usage (
                id TEXT PRIMARY KEY,
                session_id TEXT REFERENCES sessions(id) ON DELETE SET NULL,
                project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                model TEXT NOT NULL,
                input_tokens INTEGER NOT NULL DEFAULT 0,
                output_tokens INTEGER NOT NULL DEFAULT 0,
                cached_tokens INTEGER DEFAULT 0,
                reasoning_tokens INTEGER DEFAULT 0,
                cost_usd REAL NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                session_id TEXT REFERENCES sessions(id) ON DELETE SET NULL,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'pending',
                acceptance_criteria TEXT NOT NULL DEFAULT '[]',
                verification_method TEXT NOT NULL DEFAULT '',
                priority TEXT NOT NULL DEFAULT 'medium',
                dependencies TEXT DEFAULT '[]',
                phase TEXT NOT NULL DEFAULT 'foundation',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                completed_at TEXT
            );
            CREATE TABLE IF NOT EXISTS verifications (
                id TEXT PRIMARY KEY,
                task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
                type TEXT NOT NULL,
                passed INTEGER NOT NULL DEFAULT 0,
                output TEXT NOT NULL DEFAULT '',
                duration_ms INTEGER,
                created_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_projects_path ON projects(path);
            CREATE INDEX IF NOT EXISTS idx_learning_cards_project ON learning_cards(project_id);
            CREATE INDEX IF NOT EXISTS idx_sessions_project ON sessions(project_id);
            CREATE INDEX IF NOT EXISTS idx_tasks_project ON tasks(project_id);
            CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
        "#)?;
        Ok(())
    }

    // ── Projects ───────────────────────────────────────────────────────────

    pub fn create_project(&self, name: &str, path: &str, repo_url: Option<&str>) -> Result<Project, StorageError> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO projects (id, name, path, repo_url, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, name, path, repo_url, &now, &now],
        )?;
        self.get_project(&id)
    }

    pub fn list_projects(&self) -> Result<Vec<Project>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, path, repo_url, created_at, updated_at, last_session_at, total_token_cost_usd FROM projects ORDER BY updated_at DESC"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                repo_url: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
                last_session_at: row.get(6)?,
                total_token_cost_usd: row.get(7)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn get_project(&self, id: &str) -> Result<Project, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, path, repo_url, created_at, updated_at, last_session_at, total_token_cost_usd FROM projects WHERE id = ?1"
        )?;
        let row = stmt.query_row([id], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                repo_url: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
                last_session_at: row.get(6)?,
                total_token_cost_usd: row.get(7)?,
            })
        }).map_err(|_| StorageError::NotFound(format!("Project {}", id)))?;
        Ok(row)
    }

    // ── Learning Cards ─────────────────────────────────────────────────────

    pub fn create_learning_card(&self, card: &CreateLearningCard) -> Result<LearningCard, StorageError> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO learning_cards (id, project_id, type, title, trigger_condition, body, token_cost_usd, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![&id, &card.project_id, &card.card_type, &card.title, &card.trigger, &card.body, &card.token_cost_usd, &now, &now],
        )?;
        self.get_learning_card(&id)
    }

    pub fn list_learning_cards(&self, project_id: &str, confirmed_only: bool) -> Result<Vec<LearningCard>, StorageError> {
        let sql = if confirmed_only {
            "SELECT id, project_id, type, title, trigger_condition, body, token_cost_usd, confirmed_at, created_at, updated_at FROM learning_cards WHERE project_id = ?1 AND confirmed_at IS NOT NULL ORDER BY confirmed_at DESC"
        } else {
            "SELECT id, project_id, type, title, trigger_condition, body, token_cost_usd, confirmed_at, created_at, updated_at FROM learning_cards WHERE project_id = ?1 ORDER BY created_at DESC"
        };
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map([project_id], |row| {
            Ok(LearningCard {
                id: row.get(0)?,
                project_id: row.get(1)?,
                card_type: row.get(2)?,
                title: row.get(3)?,
                trigger: row.get(4)?,
                body: row.get(5)?,
                token_cost_usd: row.get(6)?,
                confirmed_at: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn confirm_learning_card(&self, id: &str) -> Result<(), StorageError> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute("UPDATE learning_cards SET confirmed_at = ?1, updated_at = ?1 WHERE id = ?2", params![&now, id])?;
        Ok(())
    }

    fn get_learning_card(&self, id: &str) -> Result<LearningCard, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, project_id, type, title, trigger_condition, body, token_cost_usd, confirmed_at, created_at, updated_at FROM learning_cards WHERE id = ?1"
        )?;
        let row = stmt.query_row([id], |row| {
            Ok(LearningCard {
                id: row.get(0)?,
                project_id: row.get(1)?,
                card_type: row.get(2)?,
                title: row.get(3)?,
                trigger: row.get(4)?,
                body: row.get(5)?,
                token_cost_usd: row.get(6)?,
                confirmed_at: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        }).map_err(|_| StorageError::NotFound(format!("LearningCard {}", id)))?;
        Ok(row)
    }

    pub fn search_learning_cards(&self, project_id: &str, query: &str) -> Result<Vec<LearningCard>, StorageError> {
        let pattern = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT id, project_id, type, title, trigger_condition, body, token_cost_usd, confirmed_at, created_at, updated_at FROM learning_cards
             WHERE project_id = ?1 AND confirmed_at IS NOT NULL AND (title LIKE ?2 OR trigger_condition LIKE ?2 OR body LIKE ?2)
             ORDER BY confirmed_at DESC LIMIT 20"
        )?;
        let rows = stmt.query_map(params![project_id, &pattern], |row| {
            Ok(LearningCard {
                id: row.get(0)?,
                project_id: row.get(1)?,
                card_type: row.get(2)?,
                title: row.get(3)?,
                trigger: row.get(4)?,
                body: row.get(5)?,
                token_cost_usd: row.get(6)?,
                confirmed_at: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    // ── Sessions ─────────────────────────────────────────────────────────────

    pub fn create_session(&self, project_id: &str, backend_id: &str, model: &str, workflow_stage: &str) -> Result<Session, StorageError> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO sessions (id, project_id, backend_id, model, workflow_stage, started_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![&id, project_id, backend_id, model, workflow_stage, &now],
        )?;
        self.get_session(&id)
    }

    pub fn get_session(&self, id: &str) -> Result<Session, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, project_id, backend_id, model, workflow_stage, started_at, finished_at, total_token_input, total_token_output, total_token_cached, total_cost_usd FROM sessions WHERE id = ?1"
        )?;
        let row = stmt.query_row([id], |row| {
            Ok(Session {
                id: row.get(0)?,
                project_id: row.get(1)?,
                backend_id: row.get(2)?,
                model: row.get(3)?,
                workflow_stage: row.get(4)?,
                started_at: row.get(5)?,
                finished_at: row.get(6)?,
                total_token_input: row.get(7)?,
                total_token_output: row.get(8)?,
                total_token_cached: row.get(9)?,
                total_cost_usd: row.get(10)?,
            })
        }).map_err(|_| StorageError::NotFound(format!("Session {}", id)))?;
        Ok(row)
    }

    // ── Tasks ──────────────────────────────────────────────────────────────

    pub fn create_task(&self, task: &CreateTask) -> Result<Task, StorageError> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO tasks (id, project_id, title, description, acceptance_criteria, verification_method, priority, phase, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![&id, &task.project_id, &task.title, &task.description, &task.acceptance_criteria, &task.verification_method, &task.priority, &task.phase, &now, &now],
        )?;
        self.get_task(&id)
    }

    pub fn list_tasks(&self, project_id: &str) -> Result<Vec<Task>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, project_id, session_id, title, description, status, acceptance_criteria, verification_method, priority, dependencies, phase, created_at, updated_at, completed_at FROM tasks WHERE project_id = ?1 ORDER BY created_at ASC"
        )?;
        let rows = stmt.query_map([project_id], |row| {
            Ok(Task {
                id: row.get(0)?,
                project_id: row.get(1)?,
                session_id: row.get(2)?,
                title: row.get(3)?,
                description: row.get(4)?,
                status: row.get(5)?,
                acceptance_criteria: row.get(6)?,
                verification_method: row.get(7)?,
                priority: row.get(8)?,
                dependencies: row.get(9)?,
                phase: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
                completed_at: row.get(13)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    fn get_task(&self, id: &str) -> Result<Task, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, project_id, session_id, title, description, status, acceptance_criteria, verification_method, priority, dependencies, phase, created_at, updated_at, completed_at FROM tasks WHERE id = ?1"
        )?;
        let row = stmt.query_row([id], |row| {
            Ok(Task {
                id: row.get(0)?,
                project_id: row.get(1)?,
                session_id: row.get(2)?,
                title: row.get(3)?,
                description: row.get(4)?,
                status: row.get(5)?,
                acceptance_criteria: row.get(6)?,
                verification_method: row.get(7)?,
                priority: row.get(8)?,
                dependencies: row.get(9)?,
                phase: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
                completed_at: row.get(13)?,
            })
        }).map_err(|_| StorageError::NotFound(format!("Task {}", id)))?;
        Ok(row)
    }

    pub fn update_task_status(&self, id: &str, status: &str) -> Result<(), StorageError> {
        let now = Utc::now().to_rfc3339();
        let completed_at: Option<String> = if status == "completed" { Some(now.clone()) } else { None };
        self.conn.execute(
            "UPDATE tasks SET status = ?1, updated_at = ?2, completed_at = ?3 WHERE id = ?4",
            params![status, &now, &completed_at, id],
        )?;
        Ok(())
    }

    // ── Verifications ─────────────────────────────────────────────────────

    pub fn create_verification(&self, v: &CreateVerification) -> Result<Verification, StorageError> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let passed_i32: i32 = if v.passed { 1 } else { 0 };
        self.conn.execute(
            "INSERT INTO verifications (id, task_id, type, passed, output, duration_ms, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![&id, &v.task_id, &v.vtype, passed_i32, &v.output, &v.duration_ms, &now],
        )?;
        let mut stmt = self.conn.prepare("SELECT id, task_id, type, passed, output, duration_ms, created_at FROM verifications WHERE id = ?1")?;
        let row = stmt.query_row([&id], |row| {
            Ok(Verification {
                id: row.get(0)?,
                task_id: row.get(1)?,
                vtype: row.get(2)?,
                passed: row.get::<_, i32>(3)? != 0,
                output: row.get(4)?,
                duration_ms: row.get(5)?,
                created_at: row.get(6)?,
            })
        }).map_err(|_| StorageError::NotFound(format!("Verification {}", id)))?;
        Ok(row)
    }

    // ── Token Usage ───────────────────────────────────────────────────────

    pub fn record_token_usage(&self, usage: &CreateTokenUsage) -> Result<(), StorageError> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO token_usage (id, session_id, project_id, model, input_tokens, output_tokens, cached_tokens, reasoning_tokens, cost_usd, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![&id, &usage.session_id, &usage.project_id, &usage.model, usage.input_tokens, usage.output_tokens, usage.cached_tokens, usage.reasoning_tokens, usage.cost_usd, &now],
        )?;
        Ok(())
    }

    pub fn get_token_usage(&self, project_id: &str) -> Result<Vec<TokenUsage>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, session_id, project_id, model, input_tokens, output_tokens, cached_tokens, reasoning_tokens, cost_usd, created_at FROM token_usage WHERE project_id = ?1 ORDER BY created_at DESC LIMIT 100"
        )?;
        let rows = stmt.query_map([project_id], |row| {
            Ok(TokenUsage {
                id: row.get(0)?,
                session_id: row.get(1)?,
                project_id: row.get(2)?,
                model: row.get(3)?,
                input_tokens: row.get(4)?,
                output_tokens: row.get(5)?,
                cached_tokens: row.get(6)?,
                reasoning_tokens: row.get(7)?,
                cost_usd: row.get(8)?,
                created_at: row.get(9)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn get_cost_summary(&self, project_id: &str) -> Result<CostSummary, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT COALESCE(SUM(input_tokens),0), COALESCE(SUM(output_tokens),0), COALESCE(SUM(cached_tokens),0), COALESCE(SUM(cost_usd),0) FROM token_usage WHERE project_id = ?1"
        )?;
        let row = stmt.query_row([project_id], |row| {
            Ok(CostSummary {
                total_input_tokens: row.get(0)?,
                total_output_tokens: row.get(1)?,
                total_cached_tokens: row.get(2)?,
                total_cost_usd: row.get(3)?,
            })
        })?;
        Ok(row)
    }

    // ── Project Rules ──────────────────────────────────────────────────────

    pub fn get_project_rules(&self, project_id: &str) -> Result<ProjectRules, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT content FROM project_facts WHERE project_id = ?1 AND category = 'rules' ORDER BY created_at DESC LIMIT 1"
        )?;
        let agents_md: Option<String> = stmt.query_row([project_id], |row| row.get(0)).ok();
        let mut stmt2 = self.conn.prepare(
            "SELECT content FROM project_facts WHERE project_id = ?1 AND category = 'claude_md' ORDER BY created_at DESC LIMIT 1"
        )?;
        let claude_md: Option<String> = stmt2.query_row([project_id], |row| row.get(0)).ok();
        Ok(ProjectRules { agents_md, claude_md })
    }

    pub fn save_project_rules(&self, project_id: &str, agents_md: Option<&str>, claude_md: Option<&str>) -> Result<(), StorageError> {
        let now = Utc::now().to_rfc3339();
        if let Some(content) = agents_md {
            let id = Uuid::new_v4().to_string();
            self.conn.execute(
                "INSERT INTO project_facts (id, project_id, category, content, source, created_at) VALUES (?1, ?2, 'rules', ?3, 'system', ?4)",
                params![&id, project_id, content, &now],
            )?;
        }
        if let Some(content) = claude_md {
            let id = Uuid::new_v4().to_string();
            self.conn.execute(
                "INSERT INTO project_facts (id, project_id, category, content, source, created_at) VALUES (?1, ?2, 'claude_md', ?3, 'system', ?4)",
                params![&id, project_id, content, &now],
            )?;
        }
        Ok(())
    }
}