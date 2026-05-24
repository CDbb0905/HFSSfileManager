import { useEffect, useMemo, useState } from "react";
import { Fragment } from "react";
import { listProjects, openProjectFolder, updateProjectNote } from "../services/tauriApi";
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

type SizeSort = "desc" | "asc";
type SortField = "size" | "name";
type SortOption = "size_desc" | "size_asc" | "name_desc" | "name_asc";

export function ProjectsPage() {
  const repositories = useAppStore((s) => s.repositories);
  const projects = useAppStore((s) => s.projects);
  const setProjects = useAppStore((s) => s.setProjects);
  const [repoId, setRepoId] = useState<string>("");
  const [keyword, setKeyword] = useState("");
  const [sortOption, setSortOption] = useState<SortOption>("size_desc");
  const [editingId, setEditingId] = useState<string>("");
  const [noteDraft, setNoteDraft] = useState("");

  const [sortField, sizeSort] = useMemo<[SortField, SizeSort]>(() => {
    if (sortOption === "size_asc") return ["size", "asc"];
    if (sortOption === "name_desc") return ["name", "desc"];
    if (sortOption === "name_asc") return ["name", "asc"];
    return ["size", "desc"];
  }, [sortOption]);

  const refresh = async () => {
    const rows = await listProjects(repoId || undefined);
    setProjects(rows);
  };

  useEffect(() => {
    void refresh();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [repoId]);

  const filtered = useMemo(() => {
    const key = keyword.trim().toLowerCase();
    const rows = !key
      ? [...projects]
      : projects.filter((p) => p.project_name.toLowerCase().includes(key) || p.aedt_path.toLowerCase().includes(key));
    if (sortField === "size") {
      rows.sort((a, b) => (sizeSort === "desc" ? b.total_size_bytes - a.total_size_bytes : a.total_size_bytes - b.total_size_bytes));
    } else {
      rows.sort((a, b) =>
        sizeSort === "desc"
          ? b.project_name.localeCompare(a.project_name, "zh-Hans-CN")
          : a.project_name.localeCompare(b.project_name, "zh-Hans-CN")
      );
    }
    return rows;
  }, [projects, keyword, sizeSort, sortField]);

  const saveNote = async () => {
    if (!editingId) return;
    await updateProjectNote(editingId, noteDraft);
    setEditingId("");
    setNoteDraft("");
    await refresh();
  };

  return (
    <section className="space-y-4">
      <div className="rounded-xl border border-slate-200 bg-white p-4">
        <div className="grid gap-3 md:grid-cols-4">
          <select className="rounded-lg border border-slate-300 px-3 py-2" value={repoId} onChange={(e) => setRepoId(e.target.value)}>
            <option value="">全部仓库</option>
            {repositories.map((repo) => (
              <option key={repo.id} value={repo.id}>
                {repo.name}
              </option>
            ))}
          </select>
          <input
            className="rounded-lg border border-slate-300 px-3 py-2 md:col-span-2"
            placeholder="搜索工程名或路径"
            value={keyword}
            onChange={(e) => setKeyword(e.target.value)}
          />
          <select
            className="rounded-lg border border-slate-300 px-3 py-2 text-sm"
            value={sortOption}
            onChange={(e) => setSortOption(e.target.value as SortOption)}
          >
            <option value="size_desc">总大小 - 降序</option>
            <option value="size_asc">总大小 - 升序</option>
            <option value="name_desc">工程名 - 降序</option>
            <option value="name_asc">工程名 - 升序</option>
          </select>
        </div>
      </div>
      <div className="overflow-hidden rounded-xl border border-slate-200 bg-white">
        <table className="w-full table-fixed text-left text-sm">
          <thead className="bg-slate-50 text-slate-600">
            <tr>
              <th className="w-[20%] px-3 py-2">工程名</th>
              <th className="w-[12%] px-3 py-2">AEDT</th>
              <th className="w-[12%] px-3 py-2">Results</th>
              <th className="w-[12%] px-3 py-2">总大小</th>
              <th className="w-[28%] px-3 py-2">备注</th>
              <th className="w-[16%] px-3 py-2">操作</th>
            </tr>
          </thead>
          <tbody>
            {filtered.map((row) => (
              <Fragment key={row.id}>
                <tr className="border-t border-slate-100">
                  <td className="break-words px-3 py-2 align-top whitespace-normal">{row.project_name}</td>
                  <td className="break-words px-3 py-2 align-top whitespace-normal">{formatBytes(row.aedt_size_bytes)}</td>
                  <td className="break-words px-3 py-2 align-top whitespace-normal">{formatBytes(row.results_size_bytes)}</td>
                  <td className="break-words px-3 py-2 align-top font-medium whitespace-normal">{formatBytes(row.total_size_bytes)}</td>
                  <td className="break-words px-3 py-2 align-top text-slate-500 whitespace-normal">{row.note || "-"}</td>
                  <td className="px-3 py-2 align-top">
                    <div className="flex flex-wrap gap-2">
                      <button
                        className="rounded bg-slate-100 px-2 py-1"
                        onClick={() => {
                          setEditingId(row.id);
                          setNoteDraft(row.note);
                        }}
                      >
                        编辑备注
                      </button>
                      <button className="rounded bg-slate-100 px-2 py-1" onClick={() => openProjectFolder(row.id)}>
                        打开目录
                      </button>
                    </div>
                  </td>
                </tr>
                {editingId === row.id && (
                  <tr className="border-t border-slate-100 bg-slate-50/50">
                    <td className="px-3 py-3" colSpan={6}>
                      <div className="space-y-2">
                        <p className="text-sm font-medium">编辑备注</p>
                        <textarea
                          className="h-24 w-full rounded-lg border border-slate-300 p-2 text-sm"
                          value={noteDraft}
                          onChange={(e) => setNoteDraft(e.target.value)}
                        />
                        <div className="flex gap-2">
                          <button className="rounded-lg bg-marine px-3 py-1.5 text-sm text-white" onClick={saveNote}>
                            保存
                          </button>
                          <button className="rounded-lg bg-slate-100 px-3 py-1.5 text-sm" onClick={() => setEditingId("")}>
                            取消
                          </button>
                        </div>
                      </div>
                    </td>
                  </tr>
                )}
              </Fragment>
            ))}
            {filtered.length === 0 && (
              <tr>
                <td className="px-3 py-6 text-center text-slate-500" colSpan={6}>
                  暂无工程数据，先去仓库页执行扫描。
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </section>
  );
}
