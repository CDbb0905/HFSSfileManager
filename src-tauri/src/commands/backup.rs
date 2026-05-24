use crate::{core::backup, models::types::BackupRecord, AppState};
use tauri::State;

#[tauri::command]
pub fn create_backup(
    state: State<'_, AppState>,
    repo_id: String,
    backup_root: String,
    with_checksum: bool,
) -> Result<BackupRecord, String> {
    let mut db = state.db.lock().map_err(|e| e.to_string())?;
    backup::create_backup(&mut db, &repo_id, &backup_root, with_checksum).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_backups(state: State<'_, AppState>, repo_id: Option<String>) -> Result<Vec<BackupRecord>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    backup::list_backups(&db, repo_id.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_backup_snapshot(
    state: State<'_, AppState>,
    backup_root: String,
    snapshot_name: String,
) -> Result<(), String> {
    let _guard = state.db.lock().map_err(|e| e.to_string())?;
    let path = backup::backup_snapshot_path(&backup_root, &snapshot_name);
    backup::open_in_explorer(&path).map_err(|e| e.to_string())
}
