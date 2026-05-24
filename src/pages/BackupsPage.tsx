import { useEffect, useMemo, useState } from "react";
import { pickDirectory } from "../services/pathPicker";
import { createBackup, listBackups, openBackupSnapshot } from "../services/tauriApi";
import { useAppStore } from "../store/useAppStore";

function formatBytes(input: number): string {
  if (input < 1024) return `${input} B`;
  const units = ["KB", "MB", "GB", "TB"];
  let size = input;
  let idx = -1;
  while (size >= 1024 && idx < units.length - 1) {
    size /= 1024;
    idx += 1;
  }
  return `${size.toFixed(2)} ${units[idx]}`;
}

export function BackupsPage() {
  const repositories = useAppStore((s) => s.repositories);
  const backups = useAppStore((s) => s.backups);
  const setBackups = useAppStore((s) => s.setBackups);
  const [repoId, setRepoId] = useState("");
  const [backupRoot, setBackupRoot] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  const refresh = async () => {
    const rows = await listBackups(repoId || undefined);
    setBackups(rows);
  };

  useEffect(() => {
    void refresh();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [repoId]);

  const runBackup = async () => {
    if (!repoId || !backupRoot.trim()) return;
    try {
      setLoading(true);
      setError("");
      await createBackup(repoId, backupRoot.trim());
      await refresh();
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  const browseBackupRoot = async () => {
    const path = await pickDirectory(backupRoot);
    if (path) setBackupRoot(path);
  };

  const filtered = useMemo(() => {
    if (!repoId) return backups;
    return backups.filter((x) => x.repo_id === repoId);
  }, [backups, repoId]);

  return (
    <section className="space-y-4">
      <div className="rounded-xl border border-slate-200 bg-white p-4">
        <h2 className="mb-3 text-lg font-semibold">立即备份</h2>
        <div className="grid gap-3 md:grid-cols-3">
          <select className="rounded-lg border border-slate-300 px-3 py-2" value={repoId} onChange={(e) => setRepoId(e.target.value)}>
            <option value="">选择仓库</option>
            {repositories.map((repo) => (
              <option key={repo.id} value={repo.id}>
                {repo.name}
              </option>
            ))}
          </select>
          <div className="md:col-span-2">
            <div className="flex gap-2">
              <input
                className="min-w-0 flex-1 rounded-lg border border-slate-300 px-3 py-2"
                placeholder="备份根目录，例如 D:\\HFSSBackups"
                value={backupRoot}
                onChange={(e) => setBackupRoot(e.target.value)}
              />
              <button className="rounded-lg bg-slate-100 px-3 py-2 text-sm" onClick={browseBackupRoot}>
                浏览...
              </button>
            </div>
          </div>
        </div>
        <div className="mt-3 flex gap-2">
          <button className="rounded-lg bg-marine px-3 py-2 text-sm text-white disabled:opacity-50" disabled={loading} onClick={runBackup}>
            {loading ? "备份中..." : "开始备份"}
          </button>
          <button className="rounded-lg bg-slate-100 px-3 py-2 text-sm" onClick={refresh}>
            刷新记录
          </button>
        </div>
        {error && <p className="mt-2 text-sm text-rose-600">{error}</p>}
      </div>
      <div className="rounded-xl border border-slate-200 bg-white p-4">
        <h2 className="mb-3 text-lg font-semibold">备份历史</h2>
        <div className="space-y-2">
          {filtered.map((row) => (
            <div key={row.id} className="rounded-lg border border-slate-100 p-3">
              <p className="font-medium">
                {row.snapshot_name} <span className="text-slate-500">({row.status})</span>
              </p>
              <p className="text-sm text-slate-500">
                文件数: {row.file_count}，大小: {formatBytes(row.total_bytes)}
              </p>
              <p className="text-sm text-slate-500">{row.backup_root}</p>
              <div className="mt-2">
                <button
                  className="rounded bg-slate-100 px-2 py-1 text-xs"
                  onClick={() => openBackupSnapshot(row.backup_root, row.snapshot_name)}
                >
                  打开备份目录
                </button>
              </div>
            </div>
          ))}
          {filtered.length === 0 && <p className="text-sm text-slate-500">暂无备份记录。</p>}
        </div>
      </div>
      {loading && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/30">
          <div className="w-full max-w-md rounded-xl bg-white p-5 shadow-xl">
            <p className="text-base font-semibold">正在执行备份</p>
            <p className="mt-2 text-sm text-slate-600">
              机械硬盘备份可能较慢，请耐心等待，应用仍在正常工作。
            </p>
          </div>
        </div>
      )}
    </section>
  );
}
