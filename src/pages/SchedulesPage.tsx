import { confirm } from "@tauri-apps/plugin-dialog";
import { useEffect, useMemo, useState } from "react";
import { pickDirectory } from "../services/pathPicker";
import { deleteSchedule, listBackups, listSchedules, runScheduleNow, saveSchedule } from "../services/tauriApi";
import { useAppStore } from "../store/useAppStore";

function fmtTs(value: string | null): string {
  if (!value) return "-";
  const d = new Date(value);
  if (Number.isNaN(d.getTime())) return value;
  return d.toLocaleString();
}

export function SchedulesPage() {
  const repositories = useAppStore((s) => s.repositories);
  const schedules = useAppStore((s) => s.schedules);
  const setSchedules = useAppStore((s) => s.setSchedules);
  const setBackups = useAppStore((s) => s.setBackups);

  const [repoId, setRepoId] = useState("");
  const [backupRoot, setBackupRoot] = useState("");
  const [enabled, setEnabled] = useState(true);
  const [policyType, setPolicyType] = useState<"daily" | "weekly">("daily");
  const [policyValue, setPolicyValue] = useState("02:00");
  const [retentionCount, setRetentionCount] = useState(7);
  const [message, setMessage] = useState("");
  const [error, setError] = useState("");

  const refresh = async () => {
    const rows = await listSchedules(repoId || undefined);
    setSchedules(rows);
  };

  useEffect(() => {
    void refresh();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [repoId]);

  const submit = async () => {
    if (!repoId || !backupRoot.trim()) return;
    try {
      setError("");
      setMessage("");
      await saveSchedule({
        repo_id: repoId,
        backup_root: backupRoot.trim(),
        enabled,
        policy_type: policyType,
        policy_value: policyValue.trim(),
        retention_count: retentionCount
      });
      setMessage("计划任务已保存。");
      await refresh();
    } catch (e) {
      setError(String(e));
    }
  };

  const runNow = async (scheduleId: string) => {
    try {
      setError("");
      setMessage("");
      await runScheduleNow(scheduleId);
      setMessage("计划任务已执行一次。");
      setSchedules(await listSchedules(repoId || undefined));
      setBackups(await listBackups(repoId || undefined));
    } catch (e) {
      setError(String(e));
    }
  };

  const removeTask = async (scheduleId: string) => {
    const ok = await confirm("确定删除该计划任务吗？删除后不会再自动执行。", {
      title: "删除任务确认",
      kind: "warning"
    });
    if (!ok) return;
    try {
      setError("");
      setMessage("");
      await deleteSchedule(scheduleId);
      setMessage("计划任务已删除。");
      await refresh();
    } catch (e) {
      setError(String(e));
    }
  };

  const browseBackupRoot = async () => {
    const path = await pickDirectory(backupRoot);
    if (path) setBackupRoot(path);
  };

  const filtered = useMemo(() => {
    if (!repoId) return schedules;
    return schedules.filter((x) => x.repo_id === repoId);
  }, [repoId, schedules]);

  const repoNameMap = useMemo(() => {
    const map = new Map<string, string>();
    for (const repo of repositories) {
      map.set(repo.id, repo.name);
    }
    return map;
  }, [repositories]);

  const editTask = (scheduleId: string) => {
    const row = schedules.find((x) => x.id === scheduleId);
    if (!row) return;
    setRepoId(row.repo_id);
    setBackupRoot(row.backup_root);
    setEnabled(row.enabled);
    setPolicyType(row.policy_type);
    setPolicyValue(row.policy_value);
    setRetentionCount(row.retention_count);
    setMessage("已将策略回填到上方表单，可修改后点击“保存策略”。");
    setError("");
  };

  return (
    <section className="space-y-4">
      <div className="rounded-xl border border-slate-200 bg-white p-4">
        <h2 className="mb-3 text-lg font-semibold">定时备份策略</h2>
        <div className="grid gap-3 md:grid-cols-2">
          <select className="rounded-lg border border-slate-300 px-3 py-2" value={repoId} onChange={(e) => setRepoId(e.target.value)}>
            <option value="">选择仓库</option>
            {repositories.map((repo) => (
              <option key={repo.id} value={repo.id}>
                {repo.name}
              </option>
            ))}
          </select>
          <div>
            <div className="flex gap-2">
              <input
                className="min-w-0 flex-1 rounded-lg border border-slate-300 px-3 py-2"
                value={backupRoot}
                onChange={(e) => setBackupRoot(e.target.value)}
                placeholder="备份根目录，例如 D:\\HFSSBackups"
              />
              <button className="rounded-lg bg-slate-100 px-3 py-2 text-sm" onClick={browseBackupRoot}>
                浏览...
              </button>
            </div>
          </div>
          <select
            className="rounded-lg border border-slate-300 px-3 py-2"
            value={policyType}
            onChange={(e) => setPolicyType(e.target.value as "daily" | "weekly")}
          >
            <option value="daily">每天</option>
            <option value="weekly">每周</option>
          </select>
          <input
            className="rounded-lg border border-slate-300 px-3 py-2"
            value={policyValue}
            onChange={(e) => setPolicyValue(e.target.value)}
            placeholder="本地时间：daily HH:mm | weekly Mon HH:mm"
          />
          <input
            type="number"
            className="rounded-lg border border-slate-300 px-3 py-2"
            min={1}
            value={retentionCount}
            onChange={(e) => setRetentionCount(Number(e.target.value || "7"))}
          />
          <label className="flex items-center gap-2 rounded-lg border border-slate-300 px-3 py-2">
            <input type="checkbox" checked={enabled} onChange={(e) => setEnabled(e.target.checked)} />
            启用此计划任务
          </label>
        </div>
        <div className="mt-3 flex gap-2">
          <button className="rounded-lg bg-marine px-3 py-2 text-sm text-white" onClick={submit}>
            保存策略
          </button>
          <button className="rounded-lg bg-slate-100 px-3 py-2 text-sm" onClick={refresh}>
            刷新
          </button>
        </div>
        {message && <p className="mt-2 text-sm text-emerald-700">{message}</p>}
        {error && <p className="mt-2 text-sm text-rose-600">{error}</p>}
      </div>
      <div className="rounded-xl border border-slate-200 bg-white p-4">
        <h2 className="mb-3 text-lg font-semibold">已配置计划任务</h2>
        <div className="space-y-2">
          {filtered.map((row) => (
            <div key={row.id} className="rounded-lg border border-slate-100 p-3">
              <p className="font-medium">
                {row.enabled ? "已启用" : "已停用"} / {row.policy_type} / {row.policy_value}
              </p>
              <p className="text-sm text-slate-500">仓库名称: {repoNameMap.get(row.repo_id) ?? row.repo_id}</p>
              <p className="text-sm text-slate-500">备份目录: {row.backup_root}</p>
              <p className="text-sm text-slate-500">保留份数: {row.retention_count}</p>
              <p className="text-sm text-slate-500">上次执行: {fmtTs(row.last_run_at)}</p>
              <p className="text-sm text-slate-500">上次状态: {row.last_status ?? "-"}</p>
              <p className="text-sm text-slate-500">下次执行: {fmtTs(row.next_run_at)}</p>
              <div className="mt-2 flex gap-2">
                <button className="rounded bg-slate-100 px-2 py-1 text-xs" onClick={() => editTask(row.id)}>
                  编辑策略
                </button>
                <button className="rounded bg-slate-100 px-2 py-1 text-xs" onClick={() => runNow(row.id)}>
                  立即执行一次
                </button>
                <button className="rounded bg-rose-50 px-2 py-1 text-xs text-rose-700" onClick={() => removeTask(row.id)}>
                  删除任务
                </button>
              </div>
            </div>
          ))}
          {filtered.length === 0 && <p className="text-sm text-slate-500">暂无计划任务。</p>}
        </div>
      </div>
    </section>
  );
}
