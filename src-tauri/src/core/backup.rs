use crate::db::Database;
use crate::models::types::BackupRecord;
use anyhow::Context;
use chrono::{Local, Utc};
use rusqlite::params;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use walkdir::WalkDir;

pub fn create_backup(
    db: &mut Database,
    repo_id: &str,
    backup_root: &str,
    with_checksum: bool,
) -> anyhow::Result<BackupRecord> {
    let repo_root: String = db.conn.query_row(
        "SELECT root_path FROM repositories WHERE id = ?1",
        params![repo_id],
        |row| row.get(0),
    )?;

    fs::create_dir_all(backup_root)?;
    ensure_backup_not_inside_repo(&repo_root, backup_root)?;

    let id = Uuid::new_v4().to_string();
    let started_at = Utc::now();
    let snapshot_name = format!("HFSS_Backup_{}", Local::now().format("%Y%m%d_%H%M%S"));
    let snapshot_dir = PathBuf::from(backup_root).join(&snapshot_name);
    fs::create_dir_all(&snapshot_dir)?;

    db.conn.execute(
        "INSERT INTO backups (id, repo_id, backup_root, snapshot_name, status, started_at, file_count, total_bytes)
         VALUES (?1, ?2, ?3, ?4, 'running', ?5, 0, 0)",
        params![id, repo_id, backup_root, snapshot_name, started_at],
    )?;

    let mut file_count = 0_i64;
    let mut total_bytes = 0_i64;
    let mut manifest_entries = Vec::new();
    let repo_root_path = PathBuf::from(&repo_root);

    let run_result: anyhow::Result<()> = (|| {
        for entry in WalkDir::new(&repo_root_path).into_iter().filter_map(Result::ok) {
            if !entry.file_type().is_file() {
                continue;
            }
            if entry.path().extension().and_then(|s| s.to_str()) != Some("aedt") {
                continue;
            }
            let rel = entry.path().strip_prefix(&repo_root_path)?;
            let target = snapshot_dir.join(rel);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(entry.path(), &target).with_context(|| format!("copy failed: {:?}", entry.path()))?;
            let size = entry.metadata()?.len() as i64;
            file_count += 1;
            total_bytes += size;
            manifest_entries.push(json!({
                "src": entry.path().to_string_lossy().to_string(),
                "dst": target.to_string_lossy().to_string(),
                "size_bytes": size,
                "sha256": if with_checksum { Some(file_sha256(&target)?) } else { None }
            }));
        }

        let manifest = json!({
          "id": id,
          "repo_id": repo_id,
          "snapshot_name": snapshot_name,
          "created_at": Utc::now(),
          "file_count": file_count,
          "total_bytes": total_bytes,
          "files": manifest_entries
        });
        let manifest_path = snapshot_dir.join("manifest.json");
        fs::write(manifest_path, serde_json::to_vec_pretty(&manifest)?)?;
        Ok(())
    })();

    let finished_at = Utc::now();
    let status = if run_result.is_ok() { "success" } else { "failed" };
    db.conn.execute(
        "UPDATE backups SET status=?2, finished_at=?3, file_count=?4, total_bytes=?5 WHERE id=?1",
        params![id, status, finished_at, file_count, total_bytes],
    )?;
    run_result?;

    Ok(BackupRecord {
        id,
        repo_id: repo_id.to_string(),
        backup_root: backup_root.to_string(),
        snapshot_name,
        status: status.to_string(),
        started_at,
        finished_at: Some(finished_at),
        file_count,
        total_bytes,
    })
}

pub fn cleanup_old_backups(
    db: &mut Database,
    repo_id: &str,
    backup_root: &str,
    retention_count: i64,
) -> anyhow::Result<usize> {
    if retention_count < 1 {
        return Ok(0);
    }
    let mut stmt = db.conn.prepare(
        "SELECT id, snapshot_name
         FROM backups
         WHERE repo_id = ?1 AND backup_root = ?2 AND status = 'success'
         ORDER BY started_at DESC",
    )?;
    let rows = stmt.query_map(params![repo_id, backup_root], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;
    let mut deleted = 0_usize;
    for (idx, row) in rows.enumerate() {
        let (id, snapshot_name) = row?;
        if idx < retention_count as usize {
            continue;
        }
        let snapshot_dir = PathBuf::from(backup_root).join(snapshot_name);
        if snapshot_dir.exists() {
            let _ = fs::remove_dir_all(snapshot_dir);
        }
        db.conn.execute("DELETE FROM backups WHERE id = ?1", params![id])?;
        deleted += 1;
    }
    Ok(deleted)
}

pub fn backup_snapshot_path(backup_root: &str, snapshot_name: &str) -> String {
    PathBuf::from(backup_root)
        .join(snapshot_name)
        .to_string_lossy()
        .to_string()
}

pub fn open_in_explorer(path: &str) -> anyhow::Result<()> {
    std::process::Command::new("explorer").arg(path).spawn()?;
    Ok(())
}

pub fn list_backups(db: &Database, repo_id: Option<&str>) -> anyhow::Result<Vec<BackupRecord>> {
    let sql = if repo_id.is_some() {
        "SELECT id, repo_id, backup_root, snapshot_name, status, started_at, finished_at, file_count, total_bytes
         FROM backups WHERE repo_id = ?1 ORDER BY started_at DESC"
    } else {
        "SELECT id, repo_id, backup_root, snapshot_name, status, started_at, finished_at, file_count, total_bytes
         FROM backups ORDER BY started_at DESC"
    };
    let mut stmt = db.conn.prepare(sql)?;
    let mapper = |row: &rusqlite::Row<'_>| {
        Ok(BackupRecord {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            backup_root: row.get(2)?,
            snapshot_name: row.get(3)?,
            status: row.get(4)?,
            started_at: row.get(5)?,
            finished_at: row.get(6)?,
            file_count: row.get(7)?,
            total_bytes: row.get(8)?,
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

fn ensure_backup_not_inside_repo(repo_root: &str, backup_root: &str) -> anyhow::Result<()> {
    let repo = fs::canonicalize(repo_root)?;
    let backup = fs::canonicalize(backup_root).unwrap_or_else(|_| PathBuf::from(backup_root));
    if backup.starts_with(&repo) {
        anyhow::bail!("backup directory cannot be inside repository");
    }
    Ok(())
}

fn file_sha256(path: &Path) -> anyhow::Result<String> {
    let mut f = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0_u8; 8192];
    loop {
        let read = f.read(&mut buf)?;
        if read == 0 {
            break;
        }
        hasher.update(&buf[..read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}
