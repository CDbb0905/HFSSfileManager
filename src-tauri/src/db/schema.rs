use anyhow::Context;
use rusqlite::{params, Connection};

pub fn apply(conn: &Connection) -> anyhow::Result<()> {
    let sql = include_str!("../../migrations/init.sql");
    conn.execute_batch(sql)
        .context("failed to apply database schema")?;
    ensure_column(conn, "schedules", "backup_root", "TEXT NOT NULL DEFAULT ''")?;
    ensure_column(conn, "schedules", "last_run_at", "TEXT")?;
    ensure_column(conn, "schedules", "last_status", "TEXT")?;
    Ok(())
}

fn ensure_column(conn: &Connection, table: &str, column: &str, definition: &str) -> anyhow::Result<()> {
    let pragma = format!("PRAGMA table_info({table})");
    let mut stmt = conn.prepare(&pragma)?;
    let exists = stmt
        .query_map([], |row| row.get::<_, String>(1))?
        .filter_map(Result::ok)
        .any(|name| name.eq_ignore_ascii_case(column));
    if !exists {
        let alter = format!("ALTER TABLE {table} ADD COLUMN {column} {definition}");
        conn.execute(&alter, params![])?;
    }
    Ok(())
}
