import { useEffect, useMemo } from "react";
import { Layout } from "./components/Layout";
import { BackupsPage } from "./pages/BackupsPage";
import { ProjectsPage } from "./pages/ProjectsPage";
import { RepositoriesPage } from "./pages/RepositoriesPage";
import { SchedulesPage } from "./pages/SchedulesPage";
import { listBackups, listProjects, listRepositories, listSchedules } from "./services/tauriApi";
import { useAppStore } from "./store/useAppStore";

function App() {
  const view = useAppStore((s) => s.view);
  const scanProgress = useAppStore((s) => s.scanProgress);
  const setRepositories = useAppStore((s) => s.setRepositories);
  const setProjects = useAppStore((s) => s.setProjects);
  const setBackups = useAppStore((s) => s.setBackups);
  const setSchedules = useAppStore((s) => s.setSchedules);

  useEffect(() => {
    const run = async () => {
      setRepositories(await listRepositories());
      setProjects(await listProjects());
      setBackups(await listBackups());
      setSchedules(await listSchedules());
    };
    void run();
  }, [setBackups, setProjects, setRepositories, setSchedules]);

  const content = useMemo(() => {
    switch (view) {
      case "repositories":
        return <RepositoriesPage />;
      case "projects":
        return <ProjectsPage />;
      case "backups":
        return <BackupsPage />;
      case "schedules":
        return <SchedulesPage />;
      default:
        return null;
    }
  }, [view]);

  return (
    <Layout>
      {scanProgress && (
        <div className="mb-4 rounded-lg border border-teal-200 bg-teal-50 px-3 py-2 text-sm text-teal-800">
          扫描任务: {scanProgress.message} ({scanProgress.scanned}/{scanProgress.total_hint})
        </div>
      )}
      {content}
    </Layout>
  );
}

export default App;
