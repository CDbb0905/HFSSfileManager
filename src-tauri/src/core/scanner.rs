use crate::db::Database;
use crate::models::types::{Project, ScanProgress};
use anyhow::Context;
use chrono::Utc;
use rusqlite::params;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use walkdir::WalkDir;

use super::fs_utils::dir_size;

pub fn scan_repository_full(db: &mut Database, repo_id: &str) -> anyhow::Result<ScanProgress> {
    let root: String = db.conn.query_row(
        "SELECT root_path FROM repositories WHERE id = ?1",
        params![repo_id],
        |row| row.get(0),
    )?;
    let root_path = PathBuf::from(root);

    let mut aedt_files: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(&root_path)
        .into_iter()
        .filter_entry(|e| should_enter_scan(e.path()))
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry
            .path()
            .extension()
            .map(|ext| ext.to_string_lossy().to_ascii_lowercase() == "aedt")
            .unwrap_or(false)
        {
            aedt_files.push(entry.path().to_path_buf());
        }
    }

    let total = aedt_files.len();
    for file in &aedt_files {
        upsert_project(db, repo_id, file).with_context(|| format!("failed to upsert {:?}", file))?;
    }

    Ok(ScanProgress {
        task_id: Uuid::new_v4().to_string(),
        repo_id: repo_id.to_string(),
        scanned: total,
        total_hint: total,
        done: true,
        message: "full scan finished".to_string(),
    })
}

fn should_enter_scan(path: &Path) -> bool {
    match path.file_name().and_then(|x| x.to_str()) {
        Some(name) => !name.to_ascii_lowercase().ends_with(".aedtresults"),
        None => true,
    }
}

fn upsert_project(db: &mut Database, repo_id: &str, aedt_path: &Path) -> anyhow::Result<()> {
    let metadata = std::fs::metadata(aedt_path)?;
    let aedt_size = metadata.len() as i64;
    let modified = metadata.modified()?;
    let modified_at: chrono::DateTime<Utc> = modified.into();

    let stem = aedt_path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let parent = aedt_path.parent().unwrap_or(Path::new("."));
    let results_dir = parent.join(format!("{stem}.aedtresults"));
    let results_size = dir_size(&results_dir) as i64;
    let total_size = aedt_size + results_size;
    let now = Utc::now();
    let id: String = db
        .conn
        .query_row(
            "SELECT id FROM projects WHERE aedt_path = ?1",
            params![aedt_path.to_string_lossy().to_string()],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| Uuid::new_v4().to_string());

    db.conn.execute(
        "INSERT INTO projects (
            id, repo_id, project_name, aedt_path, aedt_size_bytes, results_dir_path, results_size_bytes,
            total_size_bytes, note, status, last_modified_at, last_scanned_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, COALESCE((SELECT note FROM projects WHERE aedt_path = ?4), ''), 
                  COALESCE((SELECT status FROM projects WHERE aedt_path = ?4), 'normal'), ?9, ?10)
        ON CONFLICT(aedt_path) DO UPDATE SET
            repo_id = excluded.repo_id,
            project_name = excluded.project_name,
            aedt_size_bytes = excluded.aedt_size_bytes,
            results_dir_path = excluded.results_dir_path,
            results_size_bytes = excluded.results_size_bytes,
            total_size_bytes = excluded.total_size_bytes,
            last_modified_at = excluded.last_modified_at,
            last_scanned_at = excluded.last_scanned_at",
        params![
            id,
            repo_id,
            stem,
            aedt_path.to_string_lossy().to_string(),
            aedt_size,
            if results_dir.exists() {
                Some(results_dir.to_string_lossy().to_string())
            } else {
                None
            },
            results_size,
            total_size,
            modified_at,
            now
        ],
    )?;
    Ok(())
}

pub fn list_projects(db: &Database, repo_id: Option<&str>) -> anyhow::Result<Vec<Project>> {
    let sql = if repo_id.is_some() {
        "SELECT id, repo_id, project_name, aedt_path, aedt_size_bytes, results_dir_path, results_size_bytes, total_size_bytes, note, status, last_modified_at, last_scanned_at
         FROM projects WHERE repo_id = ?1 ORDER BY total_size_bytes DESC"
    } else {
        "SELECT id, repo_id, project_name, aedt_path, aedt_size_bytes, results_dir_path, results_size_bytes, total_size_bytes, note, status, last_modified_at, last_scanned_at
         FROM projects ORDER BY total_size_bytes DESC"
    };

    let mut stmt = db.conn.prepare(sql)?;
    let mapper = |row: &rusqlite::Row<'_>| {
        Ok(Project {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            project_name: row.get(2)?,
            aedt_path: row.get(3)?,
            aedt_size_bytes: row.get(4)?,
            results_dir_path: row.get(5)?,
            results_size_bytes: row.get(6)?,
            total_size_bytes: row.get(7)?,
            note: row.get(8)?,
            status: row.get(9)?,
            last_modified_at: row.get(10)?,
            last_scanned_at: row.get(11)?,
        })
    };

    let rows = if let Some(repo_id) = repo_id {
        stmt.query_map(params![repo_id], mapper)?
    } else {
        stmt.query_map([], mapper)?
    };
    let mut result = Vec::new();
    for row in rows {
        result.push(row?);
    }
    Ok(result)
}
