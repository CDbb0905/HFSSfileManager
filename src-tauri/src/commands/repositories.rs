use crate::{models::types::Repository, AppState};
use chrono::Utc;
use rusqlite::params;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub fn list_repositories(state: State<'_, AppState>) -> Result<Vec<Repository>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = db
        .conn
        .prepare("SELECT id, name, root_path, created_at FROM repositories ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(Repository {
                id: row.get(0)?,
                name: row.get(1)?,
                root_path: row.get(2)?,
                created_at: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }
    Ok(result)
}

#[tauri::command]
pub fn add_repository(state: State<'_, AppState>, root_path: String, name: Option<String>) -> Result<Repository, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let repo_name = name.unwrap_or_else(|| {
        std::path::PathBuf::from(&root_path)
            .file_name()
            .map(|x| x.to_string_lossy().to_string())
            .unwrap_or_else(|| "HFSS Repository".to_string())
    });
    db.conn
        .execute(
            "INSERT INTO repositories (id, root_path, name, created_at) VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(root_path) DO UPDATE SET name=excluded.name",
            params![id, root_path, repo_name, now],
        )
        .map_err(|e| e.to_string())?;
    let repo: Repository = db
        .conn
        .query_row(
            "SELECT id, name, root_path, created_at FROM repositories WHERE root_path = ?1",
            params![root_path],
            |row| {
                Ok(Repository {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    root_path: row.get(2)?,
                    created_at: row.get(3)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;
    Ok(repo)
}

#[tauri::command]
pub fn update_repository(
    state: State<'_, AppState>,
    repo_id: String,
    root_path: String,
    name: Option<String>,
) -> Result<Repository, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo_name = name.unwrap_or_else(|| {
        std::path::PathBuf::from(&root_path)
            .file_name()
            .map(|x| x.to_string_lossy().to_string())
            .unwrap_or_else(|| "HFSS Repository".to_string())
    });
    db.conn
        .execute(
            "UPDATE repositories SET root_path = ?1, name = ?2 WHERE id = ?3",
            params![root_path, repo_name, repo_id],
        )
        .map_err(|e| e.to_string())?;
    let repo: Repository = db
        .conn
        .query_row(
            "SELECT id, name, root_path, created_at FROM repositories WHERE id = ?1",
            params![repo_id],
            |row| {
                Ok(Repository {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    root_path: row.get(2)?,
                    created_at: row.get(3)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;
    Ok(repo)
}

#[tauri::command]
pub fn remove_repository(state: State<'_, AppState>, repo_id: String) -> Result<(), String> {
    let mut db = state.db.lock().map_err(|e| e.to_string())?;
    let tx = db.conn.transaction().map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM projects WHERE repo_id = ?1", params![repo_id.as_str()])
        .map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM backups WHERE repo_id = ?1", params![repo_id.as_str()])
        .map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM schedules WHERE repo_id = ?1", params![repo_id.as_str()])
        .map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM repositories WHERE id = ?1", params![repo_id.as_str()])
        .map_err(|e| e.to_string())?;
    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}
