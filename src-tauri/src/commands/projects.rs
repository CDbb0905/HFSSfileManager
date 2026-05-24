use crate::{core::scanner, models::types::Project, AppState};
use rusqlite::params;
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub fn list_projects(state: State<'_, AppState>, repo_id: Option<String>) -> Result<Vec<Project>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    scanner::list_projects(&db, repo_id.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_project_note(state: State<'_, AppState>, project_id: String, note: String) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.conn
        .execute("UPDATE projects SET note = ?1 WHERE id = ?2", params![note, project_id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn open_project_folder(state: State<'_, AppState>, project_id: String) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let aedt_path: String = db
        .conn
        .query_row(
            "SELECT aedt_path FROM projects WHERE id = ?1",
            params![project_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    let folder = PathBuf::from(aedt_path)
        .parent()
        .map(|x| x.to_path_buf())
        .ok_or_else(|| "project folder not found".to_string())?;
    std::process::Command::new("explorer")
        .arg(folder.to_string_lossy().to_string())
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}
