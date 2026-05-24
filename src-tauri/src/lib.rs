mod commands;
mod core;
mod db;
mod models;
mod tasks;

use commands::{
    add_repository, create_backup, list_backups, list_projects, list_repositories, list_schedules,
    open_backup_snapshot, open_project_folder, remove_repository, run_schedule_now, save_schedule,
    scan_repository_full, update_project_note, update_repository, delete_schedule,
};
use db::Database;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use tasks::scheduler_worker;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Database>>,
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let handle = app.handle();
            let db_dir = resolve_db_dir(&handle);
            let db = Database::new(&db_dir)?;
            let db = Arc::new(Mutex::new(db));
            scheduler_worker::spawn(db.clone());
            app.manage(AppState {
                db,
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_repositories,
            add_repository,
            update_repository,
            remove_repository,
            scan_repository_full,
            list_projects,
            update_project_note,
            open_project_folder,
            create_backup,
            list_backups,
            open_backup_snapshot,
            save_schedule,
            list_schedules,
            run_schedule_now,
            delete_schedule
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn resolve_db_dir(handle: &tauri::AppHandle) -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let portable_data = parent.join("data");
            if std::fs::create_dir_all(&portable_data).is_ok() {
                return portable_data;
            }
        }
    }
    handle.path().app_data_dir().unwrap_or_else(|_| PathBuf::from("."))
}
