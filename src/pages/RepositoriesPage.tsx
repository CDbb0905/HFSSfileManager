import { confirm } from "@tauri-apps/plugin-dialog";
import { Fragment, useState } from "react";
import type { Repository } from "../types";
import { pickDirectory } from "../services/pathPicker";
import {
  addRepository,
  listRepositories,
  removeRepository,
  scanRepositoryFull,
  updateRepository
} from "../services/tauriApi";
import { useAppStore } from "../store/useAppStore";

export function RepositoriesPage() {
  const repositories = useAppStore((s) => s.repositories);
  const setRepositories = useAppStore((s) => s.setRepositories);
  const setScanProgress = useAppStore((s) => s.setScanProgress);
  const setProjects = useAppStore((s) => s.setProjects);
  const setBackups = useAppStore((s) => s.setBackups);
  const setSchedules = useAppStore((s) => s.setSchedules);

  const [rootPath, setRootPath] = useState("");
  const [name, setName] = useState("");
  const [error, setError] = useState("");
  const [editing, setEditing] = useState<Repository | null>(null);
  const [editName, setEditName] = useState("");
  const [editPath, setEditPath] = useState("");
  const [scanningRepoName, setScanningRepoName] = useState("");

  const refresh = async () => {
    const rows = await listRepositories();
    setRepositories(rows);
  };

  const submit = async () => {
    if (!rootPath.trim()) return;
    try {
      setError("");
      await addRepository(rootPath.trim(), name || undefined);
      setRootPath("");
      setName("");
      await refresh();
    } catch (e) {
      setError(String(e));
    }
  };

  const fullScan = async (repoId: string) => {
    const repo = repositories.find((x) => x.id === repoId);
    setScanningRepoName(repo?.name ?? "当前仓库");
    try {
      const progress = await scanRepositoryFull(repoId);
      setScanProgress(progress);
    } finally {
      setScanningRepoName("");
    }
  };

  const browseRoot = async () => {
    const path = await pickDirectory(rootPath);
    if (path) setRootPath(path);
  };

  const openEdit = (repo: Repository) => {
    setEditing(repo);
    setEditName(repo.name);
    setEditPath(repo.root_path);
  };

  const saveEdit = async () => {
    if (!editing) return;
    if (!editPath.trim()) return;
    try {
      setError("");
      await updateRepository(editing.id, editPath.trim(), editName || undefined);
      setEditing(null);
      await refresh();
    } catch (e) {
      setError(String(e));
    }
  };

  const browseEditPath = async () => {
    const path = await pickDirectory(editPath);
    if (path) setEditPath(path);
  };

  const removeRepo = async (repo: Repository) => {
    const ok = await confirm(
      `确定要移除仓库“${repo.name}”吗？\n\n仅会删除应用数据库中的仓库和关联记录，不会删除磁盘上的任何工程文件。`,
      { title: "移除仓库确认", kind: "warning" }
    );
    if (!ok) return;
    try {
      setError("");
      await removeRepository(repo.id);
      setProjects([]);
      setBackups([]);
      setSchedules([]);
      await refresh();
    } catch (e) {
      setError(String(e));
    }
  };

  return (
    <section className="space-y-4">
      <div className="rounded-xl border border-slate-200 bg-white p-4">
        <h2 className="mb-3 text-lg font-semibold">新增仓库</h2>
        <div className="grid gap-3 md:grid-cols-3">
          <input
            className="rounded-lg border border-slate-300 px-3 py-2"
            placeholder="仓库名称（可选）"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
          <div className="md:col-span-2">
            <div className="flex gap-2">
              <input
                className="min-w-0 flex-1 rounded-lg border border-slate-300 px-3 py-2"
                placeholder="例如 D:\\HFSSFile"
                value={rootPath}
                onChange={(e) => setRootPath(e.target.value)}
              />
              <button className="rounded-lg bg-slate-100 px-3 py-2 text-sm" onClick={browseRoot}>
                浏览...
              </button>
            </div>
          </div>
        </div>
        <div className="mt-3 flex gap-2">
          <button className="rounded-lg bg-marine px-3 py-2 text-sm text-white" onClick={submit}>
            保存仓库
          </button>
          <button className="rounded-lg bg-slate-100 px-3 py-2 text-sm" onClick={refresh}>
            刷新列表
          </button>
        </div>
        {error && <p className="mt-2 text-sm text-rose-600">{error}</p>}
      </div>

      <div className="rounded-xl border border-slate-200 bg-white p-4">
        <h2 className="mb-3 text-lg font-semibold">仓库列表</h2>
        <div className="space-y-2">
          {repositories.map((repo) => (
            <Fragment key={repo.id}>
              <div className="rounded-lg border border-slate-100 p-3">
                <p className="font-medium">{repo.name}</p>
                <p className="text-sm text-slate-500">{repo.root_path}</p>
                <div className="mt-2 flex gap-2">
                  <button
                    className="rounded bg-slate-100 px-3 py-1.5 text-sm disabled:opacity-50"
                    disabled={Boolean(scanningRepoName)}
                    onClick={() => fullScan(repo.id)}
                  >
                    全量扫描
                  </button>
                  <button className="rounded bg-slate-100 px-3 py-1.5 text-sm" onClick={() => openEdit(repo)}>
                    编辑仓库
                  </button>
                  <button className="rounded bg-rose-50 px-3 py-1.5 text-sm text-rose-700" onClick={() => removeRepo(repo)}>
                    移除仓库
                  </button>
                </div>
              </div>
              {editing?.id === repo.id && (
                <div className="rounded-lg border border-slate-200 bg-slate-50 p-4">
                  <h3 className="mb-1 text-base font-semibold">编辑仓库（名称/路径）</h3>
                  <p className="mb-3 text-sm text-slate-500">可以同时修改仓库名称和仓库路径。</p>
                  <div className="grid gap-3 md:grid-cols-3">
                    <input
                      className="rounded-lg border border-slate-300 px-3 py-2"
                      placeholder="仓库名称"
                      value={editName}
                      onChange={(e) => setEditName(e.target.value)}
                    />
                    <div className="md:col-span-2">
                      <div className="flex gap-2">
                        <input
                          className="min-w-0 flex-1 rounded-lg border border-slate-300 px-3 py-2"
                          placeholder="仓库路径"
                          value={editPath}
                          onChange={(e) => setEditPath(e.target.value)}
                        />
                        <button className="rounded-lg bg-slate-100 px-3 py-2 text-sm" onClick={browseEditPath}>
                          浏览...
                        </button>
                      </div>
                    </div>
                  </div>
                  <div className="mt-3 flex gap-2">
                    <button className="rounded-lg bg-marine px-3 py-2 text-sm text-white" onClick={saveEdit}>
                      保存修改
                    </button>
                    <button className="rounded-lg bg-slate-100 px-3 py-2 text-sm" onClick={() => setEditing(null)}>
                      取消
                    </button>
                  </div>
                </div>
              )}
            </Fragment>
          ))}
          {repositories.length === 0 && <p className="text-sm text-slate-500">暂无仓库，先新增一个。</p>}
        </div>
      </div>
      {scanningRepoName && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/30">
          <div className="w-full max-w-md rounded-xl bg-white p-5 shadow-xl">
            <p className="text-base font-semibold">正在扫描仓库</p>
            <p className="mt-2 text-sm text-slate-600">
              仓库：{scanningRepoName}
              <br />
              机械硬盘扫描可能较慢，请耐心等待，应用仍在正常工作。
            </p>
          </div>
        </div>
      )}
    </section>
  );
}
