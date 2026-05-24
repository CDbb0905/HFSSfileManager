CREATE TABLE IF NOT EXISTS repositories (
  id TEXT PRIMARY KEY,
  root_path TEXT UNIQUE NOT NULL,
  name TEXT NOT NULL,
  created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS projects (
  id TEXT PRIMARY KEY,
  repo_id TEXT NOT NULL,
  project_name TEXT NOT NULL,
  aedt_path TEXT UNIQUE NOT NULL,
  aedt_size_bytes INTEGER NOT NULL DEFAULT 0,
  results_dir_path TEXT,
  results_size_bytes INTEGER NOT NULL DEFAULT 0,
  total_size_bytes INTEGER NOT NULL DEFAULT 0,
  note TEXT NOT NULL DEFAULT '',
  status TEXT NOT NULL DEFAULT 'normal',
  last_modified_at TEXT NOT NULL,
  last_scanned_at TEXT NOT NULL,
  FOREIGN KEY (repo_id) REFERENCES repositories(id)
);

CREATE TABLE IF NOT EXISTS backups (
  id TEXT PRIMARY KEY,
  repo_id TEXT NOT NULL,
  backup_root TEXT NOT NULL,
  snapshot_name TEXT NOT NULL,
  status TEXT NOT NULL,
  started_at TEXT NOT NULL,
  finished_at TEXT,
  file_count INTEGER NOT NULL DEFAULT 0,
  total_bytes INTEGER NOT NULL DEFAULT 0,
  FOREIGN KEY (repo_id) REFERENCES repositories(id)
);

CREATE TABLE IF NOT EXISTS schedules (
  id TEXT PRIMARY KEY,
  repo_id TEXT NOT NULL,
  backup_root TEXT NOT NULL,
  enabled INTEGER NOT NULL DEFAULT 1,
  policy_type TEXT NOT NULL,
  policy_value TEXT NOT NULL,
  retention_count INTEGER NOT NULL DEFAULT 7,
  last_run_at TEXT,
  last_status TEXT,
  next_run_at TEXT,
  UNIQUE(repo_id),
  FOREIGN KEY (repo_id) REFERENCES repositories(id)
);
