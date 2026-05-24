export type AppView = "repositories" | "projects" | "backups" | "schedules";

export interface Repository {
  id: string;
  name: string;
  root_path: string;
  created_at: string;
}

export interface Project {
  id: string;
  repo_id: string;
  project_name: string;
  aedt_path: string;
  aedt_size_bytes: number;
  results_dir_path: string | null;
  results_size_bytes: number;
  total_size_bytes: number;
  note: string;
  status: "normal" | "to_clean";
  last_modified_at: string;
  last_scanned_at: string;
}

export interface BackupRecord {
  id: string;
  repo_id: string;
  backup_root: string;
  snapshot_name: string;
  status: "running" | "success" | "failed" | "partial";
  started_at: string;
  finished_at: string | null;
  file_count: number;
  total_bytes: number;
}

export interface SchedulePolicy {
  id: string;
  repo_id: string;
  backup_root: string;
  enabled: boolean;
  policy_type: "daily" | "weekly";
  policy_value: string;
  retention_count: number;
  last_run_at: string | null;
  last_status: string | null;
  next_run_at: string | null;
}

export interface ScanProgress {
  task_id: string;
  repo_id: string;
  scanned: number;
  total_hint: number;
  done: boolean;
  message: string;
}
