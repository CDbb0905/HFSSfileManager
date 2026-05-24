use crate::{
    core::scheduler,
    models::types::{BackupRecord, SaveScheduleInput, SchedulePolicy},
    AppState,
};
use tauri::State;

#[tauri::command]
pub fn save_schedule(state: State<'_, AppState>, input: SaveScheduleInput) -> Result<SchedulePolicy, String> {
    let mut db = state.db.lock().map_err(|e| e.to_string())?;
    scheduler::save_schedule(&mut db, &input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_schedules(state: State<'_, AppState>, repo_id: Option<String>) -> Result<Vec<SchedulePolicy>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    scheduler::list_schedules(&db, repo_id.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn run_schedule_now(state: State<'_, AppState>, schedule_id: String) -> Result<BackupRecord, String> {
    let mut db = state.db.lock().map_err(|e| e.to_string())?;
    scheduler::run_schedule_now(&mut db, &schedule_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_schedule(state: State<'_, AppState>, schedule_id: String) -> Result<(), String> {
    let mut db = state.db.lock().map_err(|e| e.to_string())?;
    scheduler::delete_schedule(&mut db, &schedule_id).map_err(|e| e.to_string())
}
