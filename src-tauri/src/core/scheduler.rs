use crate::core::backup;
use crate::db::Database;
use crate::models::types::{BackupRecord, SaveScheduleInput, SchedulePolicy};
use chrono::{Datelike, Duration, Local, TimeZone, Utc, Weekday};
use rusqlite::params;
use uuid::Uuid;

pub fn save_schedule(db: &mut Database, input: &SaveScheduleInput) -> anyhow::Result<SchedulePolicy> {
    if input.backup_root.trim().is_empty() {
        anyhow::bail!("backup_root is required");
    }
    let next = compute_next_run(&input.policy_type, &input.policy_value)?;
    let id: String = db
        .conn
        .query_row(
            "SELECT id FROM schedules WHERE repo_id = ?1",
            params![input.repo_id],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| Uuid::new_v4().to_string());
    db.conn.execute(
      "INSERT INTO schedules (id, repo_id, backup_root, enabled, policy_type, policy_value, retention_count, last_run_at, last_status, next_run_at)
       VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, NULL, NULL, ?8)
       ON CONFLICT(repo_id) DO UPDATE SET
         backup_root=excluded.backup_root,
         enabled=excluded.enabled,
         policy_type=excluded.policy_type,
         policy_value=excluded.policy_value,
         retention_count=excluded.retention_count,
         next_run_at=excluded.next_run_at",
      params![
        id,
        input.repo_id,
        input.backup_root,
        if input.enabled { 1 } else { 0 },
        input.policy_type,
        input.policy_value,
        input.retention_count,
        next
      ],
    )?;
    Ok(SchedulePolicy {
        id,
        repo_id: input.repo_id.clone(),
        backup_root: input.backup_root.clone(),
        enabled: input.enabled,
        policy_type: input.policy_type.clone(),
        policy_value: input.policy_value.clone(),
        retention_count: input.retention_count,
        last_run_at: None,
        last_status: None,
        next_run_at: Some(next),
    })
}

pub fn list_schedules(db: &Database, repo_id: Option<&str>) -> anyhow::Result<Vec<SchedulePolicy>> {
    let sql = if repo_id.is_some() {
        "SELECT id, repo_id, backup_root, enabled, policy_type, policy_value, retention_count, last_run_at, last_status, next_run_at
         FROM schedules WHERE repo_id = ?1"
    } else {
        "SELECT id, repo_id, backup_root, enabled, policy_type, policy_value, retention_count, last_run_at, last_status, next_run_at
         FROM schedules"
    };
    let mut stmt = db.conn.prepare(sql)?;
    let mapper = |row: &rusqlite::Row<'_>| {
        Ok(SchedulePolicy {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            backup_root: row.get(2)?,
            enabled: row.get::<_, i64>(3)? == 1,
            policy_type: row.get(4)?,
            policy_value: row.get(5)?,
            retention_count: row.get(6)?,
            last_run_at: row.get(7)?,
            last_status: row.get(8)?,
            next_run_at: row.get(9)?,
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

pub fn run_schedule_now(db: &mut Database, schedule_id: &str) -> anyhow::Result<BackupRecord> {
    let (repo_id, backup_root, policy_type, policy_value, retention_count): (String, String, String, String, i64) = db.conn.query_row(
        "SELECT repo_id, backup_root, policy_type, policy_value, retention_count FROM schedules WHERE id = ?1",
        params![schedule_id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
    )?;
    execute_schedule(db, schedule_id, &repo_id, &backup_root, &policy_type, &policy_value, retention_count)
}

pub fn delete_schedule(db: &mut Database, schedule_id: &str) -> anyhow::Result<()> {
    db.conn
        .execute("DELETE FROM schedules WHERE id = ?1", params![schedule_id])?;
    Ok(())
}

pub fn run_due_schedules(db: &mut Database) -> anyhow::Result<usize> {
    let now = Utc::now();
    let mut stmt = db.conn.prepare(
        "SELECT id, repo_id, backup_root, policy_type, policy_value, retention_count
         FROM schedules
         WHERE enabled = 1 AND next_run_at IS NOT NULL AND next_run_at <= ?1",
    )?;
    let rows = stmt.query_map(params![now], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, i64>(5)?,
        ))
    })?;
    let mut due = Vec::new();
    for row in rows {
        due.push(row?);
    }
    drop(stmt);
    let mut count = 0_usize;
    for (id, repo_id, backup_root, policy_type, policy_value, retention_count) in due {
        let _ = execute_schedule(
            db,
            &id,
            &repo_id,
            &backup_root,
            &policy_type,
            &policy_value,
            retention_count,
        );
        count += 1;
    }
    Ok(count)
}

fn execute_schedule(
    db: &mut Database,
    schedule_id: &str,
    repo_id: &str,
    backup_root: &str,
    policy_type: &str,
    policy_value: &str,
    retention_count: i64,
) -> anyhow::Result<BackupRecord> {
    let run_at = Utc::now();
    let backup_result = backup::create_backup(db, repo_id, backup_root, false);
    let next_run = compute_next_run(policy_type, policy_value)?;
    let status = if backup_result.is_ok() { "success" } else { "failed" };
    db.conn.execute(
        "UPDATE schedules SET last_run_at = ?2, last_status = ?3, next_run_at = ?4 WHERE id = ?1",
        params![schedule_id, run_at, status, next_run],
    )?;
    if backup_result.is_ok() {
        let _ = backup::cleanup_old_backups(db, repo_id, backup_root, retention_count);
    }
    backup_result
}

pub fn compute_next_run(policy_type: &str, policy_value: &str) -> anyhow::Result<chrono::DateTime<Utc>> {
    let now_local = Local::now();
    match policy_type {
        "daily" => {
            let (h, m) = parse_hhmm(policy_value)?;
            let today = now_local.date_naive();
            let mut candidate = Local
                .with_ymd_and_hms(today.year(), today.month(), today.day(), h, m, 0)
                .single()
                .ok_or_else(|| anyhow::anyhow!("invalid daily schedule"))?;
            if candidate <= now_local {
                candidate += Duration::days(1);
            }
            Ok(candidate.with_timezone(&Utc))
        }
        "weekly" => {
            let parts: Vec<_> = policy_value.split_whitespace().collect();
            if parts.len() != 2 {
                anyhow::bail!("weekly format must be 'Mon HH:mm'");
            }
            let weekday = parse_weekday(parts[0])?;
            let (h, m) = parse_hhmm(parts[1])?;
            let now_day = now_local.weekday().num_days_from_monday() as i64;
            let target_day = weekday.num_days_from_monday() as i64;
            let mut delta = target_day - now_day;
            if delta < 0 {
                delta += 7;
            }
            let date = now_local.date_naive() + Duration::days(delta);
            let mut candidate = Local
                .with_ymd_and_hms(date.year(), date.month(), date.day(), h, m, 0)
                .single()
                .ok_or_else(|| anyhow::anyhow!("invalid weekly schedule"))?;
            if candidate <= now_local {
                candidate += Duration::days(7);
            }
            Ok(candidate.with_timezone(&Utc))
        }
        _ => anyhow::bail!("unsupported policy type"),
    }
}

fn parse_hhmm(input: &str) -> anyhow::Result<(u32, u32)> {
    let parts: Vec<_> = input.split(':').collect();
    if parts.len() != 2 {
        anyhow::bail!("time format must be HH:mm");
    }
    Ok((parts[0].parse()?, parts[1].parse()?))
}

fn parse_weekday(input: &str) -> anyhow::Result<Weekday> {
    match input {
        "Mon" => Ok(Weekday::Mon),
        "Tue" => Ok(Weekday::Tue),
        "Wed" => Ok(Weekday::Wed),
        "Thu" => Ok(Weekday::Thu),
        "Fri" => Ok(Weekday::Fri),
        "Sat" => Ok(Weekday::Sat),
        "Sun" => Ok(Weekday::Sun),
        _ => anyhow::bail!("weekday must be Mon..Sun"),
    }
}
