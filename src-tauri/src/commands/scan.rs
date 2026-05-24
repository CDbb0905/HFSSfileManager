use crate::{core::scanner, models::types::ScanProgress, AppState};
use tauri::State;

#[tauri::command]
pub fn scan_repository_full(state: State<'_, AppState>, repo_id: String) -> Result<ScanProgress, String> {
    let mut db = state.db.lock().map_err(|e| e.to_string())?;
    scanner::scan_repository_full(&mut db, &repo_id).map_err(|e| e.to_string())
}
