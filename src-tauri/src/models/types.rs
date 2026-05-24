use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: String,
    pub name: String,
    pub root_path: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub repo_id: String,
    pub project_name: String,
    pub aedt_path: String,
    pub aedt_size_bytes: i64,
    pub results_dir_path: Option<String>,
    pub results_size_bytes: i64,
    pub total_size_bytes: i64,
    pub note: String,
    pub status: String,
    pub last_modified_at: DateTime<Utc>,
    pub last_scanned_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupRecord {
    pub id: String,
    pub repo_id: String,
    pub backup_root: String,
    pub snapshot_name: String,
    pub status: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub file_count: i64,
    pub total_bytes: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulePolicy {
    pub id: String,
    pub repo_id: String,
    pub backup_root: String,
    pub enabled: bool,
    pub policy_type: String,
    pub policy_value: String,
    pub retention_count: i64,
    pub last_run_at: Option<DateTime<Utc>>,
    pub last_status: Option<String>,
    pub next_run_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveScheduleInput {
    pub repo_id: String,
    pub backup_root: String,
    pub enabled: bool,
    pub policy_type: String,
    pub policy_value: String,
    pub retention_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub task_id: String,
    pub repo_id: String,
    pub scanned: usize,
    pub total_hint: usize,
    pub done: bool,
    pub message: String,
}
