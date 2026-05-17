// src-tauri/src/storage/repo.rs
// Repository layer — thin wrapper (actual impl is in mod.rs for now)

use super::StorageError;

// Re-export the Database from mod.rs
pub use super::Database;