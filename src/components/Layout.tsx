import type { ReactNode } from "react";
import { useAppStore } from "../store/useAppStore";
import type { AppView } from "../types";

const tabs: { key: AppView; label: string }[] = [
  { key: "repositories", label: "仓库" },
  { key: "projects", label: "工程列表" },
  { key: "backups", label: "备份中心" },
  { key: "schedules", label: "计划任务" }
];

export function Layout({ children }: { children: ReactNode }) {
  const view = useAppStore((s) => s.view);
  const setView = useAppStore((s) => s.setView);

  return (
    <div className="min-h-screen bg-gradient-to-br from-sand via-mist to-white text-ink">
      <header className="border-b border-slate-200 bg-white/80 backdrop-blur">
        <div className="mx-auto flex max-w-7xl items-center justify-between px-6 py-4">
          <h1 className="text-xl font-semibold tracking-tight">HFSS File Manager</h1>
          <nav className="flex gap-2">
            {tabs.map((tab) => (
              <button
                key={tab.key}
                className={`rounded-lg px-3 py-1.5 text-sm ${
                  view === tab.key ? "bg-marine text-white" : "bg-slate-100 text-slate-700"
                }`}
                onClick={() => setView(tab.key)}
              >
                {tab.label}
              </button>
            ))}
          </nav>
        </div>
      </header>
      <main className="mx-auto max-w-7xl px-6 py-6">{children}</main>
    </div>
  );
}
