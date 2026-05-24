import { invoke } from "@tauri-apps/api/core";
import type { BackupRecord, Project, Repository, ScanProgress, SchedulePolicy } from "../types";

export async function listRepositories(): Promise<Repository[]> {
  return invoke("list_repositories");
}

export async function addRepository(rootPath: string, name?: string): Promise<Repository> {
  return invoke("add_repository", { rootPath, name });
}

export async function updateRepository(repoId: string, rootPath: string, name?: string): Promise<Repository> {
  return invoke("update_repository", { repoId, rootPath, name });
}

export async function removeRepository(repoId: string): Promise<void> {
  return invoke("remove_repository", { repoId });
}

export async function scanRepositoryFull(repoId: string): Promise<ScanProgress> {
  return invoke("scan_repository_full", { repoId });
}

export async function listProjects(repoId?: string): Promise<Project[]> {
  return invoke("list_projects", { repoId });
}

export async function updateProjectNote(projectId: string, note: string): Promise<void> {
  return invoke("update_project_note", { projectId, note });
}

export async function openProjectFolder(projectId: string): Promise<void> {
  return invoke("open_project_folder", { projectId });
}

export async function listBackups(repoId?: string): Promise<BackupRecord[]> {
  return invoke("list_backups", { repoId });
}

export async function createBackup(repoId: string, backupRoot: string): Promise<BackupRecord> {
  return invoke("create_backup", { repoId, backupRoot, withChecksum: false });
}

export async function openBackupSnapshot(backupRoot: string, snapshotName: string): Promise<void> {
  return invoke("open_backup_snapshot", { backupRoot, snapshotName });
}

export async function listSchedules(repoId?: string): Promise<SchedulePolicy[]> {
  return invoke("list_schedules", { repoId });
}

export async function saveSchedule(input: {
  repo_id: string;
  backup_root: string;
  enabled: boolean;
  policy_type: "daily" | "weekly";
  policy_value: string;
  retention_count: number;
}): Promise<SchedulePolicy> {
  return invoke("save_schedule", { input });
}

export async function runScheduleNow(scheduleId: string): Promise<BackupRecord> {
  return invoke("run_schedule_now", { scheduleId });
}

export async function deleteSchedule(scheduleId: string): Promise<void> {
  return invoke("delete_schedule", { scheduleId });
}
