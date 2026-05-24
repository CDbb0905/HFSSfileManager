import { create } from "zustand";
import type { AppView, BackupRecord, Project, Repository, ScanProgress, SchedulePolicy } from "../types";

interface AppState {
  view: AppView;
  repositories: Repository[];
  projects: Project[];
  backups: BackupRecord[];
  schedules: SchedulePolicy[];
  scanProgress: ScanProgress | null;
  setView: (view: AppView) => void;
  setRepositories: (rows: Repository[]) => void;
  setProjects: (rows: Project[]) => void;
  setBackups: (rows: BackupRecord[]) => void;
  setSchedules: (rows: SchedulePolicy[]) => void;
  setScanProgress: (value: ScanProgress | null) => void;
}

export const useAppStore = create<AppState>((set) => ({
  view: "repositories",
  repositories: [],
  projects: [],
  backups: [],
  schedules: [],
  scanProgress: null,
  setView: (view) => set({ view }),
  setRepositories: (repositories) => set({ repositories }),
  setProjects: (projects) => set({ projects }),
  setBackups: (backups) => set({ backups }),
  setSchedules: (schedules) => set({ schedules }),
  setScanProgress: (scanProgress) => set({ scanProgress })
}));
