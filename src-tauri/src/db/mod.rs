mod schema;

use anyhow::Context;
use rusqlite::Connection;
use std::path::{Path, PathBuf};

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new(app_data_dir: &Path) -> anyhow::Result<Self> {
        std::fs::create_dir_all(app_data_dir).context("failed to create app data dir")?;
        let db_path: PathBuf = app_data_dir.join("hfss_file_manager.db");
        let conn = Connection::open(db_path).context("failed to open sqlite db")?;
        schema::apply(&conn)?;
        Ok(Self { conn })
    }
}
