**English** | [简体中文](README.zh-CN.md)

# HFSS File Manager

A desktop application for managing HFSS (High Frequency Structure Simulator) engineering projects. Scan `.aedt` project directories, estimate storage usage, add notes, and run `.aedt`-only backups with scheduled policies.

## Features

- **Repository Management** — Add and manage scan directories pointing to HFSS project locations
- **Project Scanning** — Recursively scan repositories for `.aedt` / `.aedtresults`, calculate file sizes and total storage usage
- **Project List & Search** — Browse all scanned projects with filtering, sorting (by size / name), and keyword search
- **Note Editing** — Annotate projects with custom notes
- **One-Click Backup** — Copy only `.aedt` files while preserving relative directory structure, with a generated `manifest.json` for each snapshot
- **Schedule Policies** — Create daily or weekly backup schedules with configurable retention counts; auto-cleanup old backups
- **Native File Dialogs** — Browse and select directories via system dialog
- **Portable Mode** — Database stored alongside the executable when writable; falls back to OS app data directory

## Screenshots

<!-- TODO: Add screenshots here -->

## Tech Stack

| Layer | Technology |
|---|---|
| Desktop Shell | [Tauri 2](https://v2.tauri.app/) |
| Frontend | React 18, TypeScript, Vite |
| Styling | Tailwind CSS |
| State Management | Zustand |
| Table & Virtualization | TanStack React Table, TanStack React Virtual |
| Database | SQLite (rusqlite, bundled) |
| Backend Language | Rust (edition 2021) |
| Rust Crates | serde, chrono, uuid, walkdir, sha2, anyhow, tokio |

## Prerequisites (Windows)

1. **Rust** (stable):
   ```bash
   winget install -e --id Rustlang.Rustup
   ```
2. **Visual Studio C++ build tools** (if missing):
   ```bash
   winget install -e --id Microsoft.VisualStudio.2022.BuildTools
   ```
3. **WebView2 runtime** (usually pre-installed on Windows 10+):
   ```bash
   winget install -e --id Microsoft.EdgeWebView2Runtime
   ```
4. **Node.js** (18+):

## Getting Started

```bash
# clone the repository
git clone https://github.com/YOUR_USERNAME/hfss-file-manager.git
cd hfss-file-manager

# install frontend dependencies
npm install

# run in development mode
npm run tauri dev

# build for production
npm run tauri build
```

## Usage

1. **Add a Repository** — Go to the "仓库" tab and add a root directory containing HFSS projects (`.aedt` files).
2. **Scan** — Click "全量扫描" to recursively discover all `.aedt` projects and calculate their storage sizes.
3. **Browse Projects** — Switch to the "工程列表" tab to search, filter, and sort scanned projects. Edit notes for each project.
4. **Backup** — In the "备份中心" tab, select a repository and backup root directory, then click "开始备份". Only `.aedt` files are copied; a `manifest.json` is generated.
5. **Schedule** — In the "计划任务" tab, configure daily or weekly automatic backups with retention count.

## Project Structure

```
hfss-file-manager/
├── src/                        # React frontend
│   ├── components/             # Shared UI components (Layout)
│   ├── pages/                  # Page components (Repositories, Projects, Backups, Schedules)
│   ├── services/               # API layer (Tauri invoke wrappers)
│   ├── store/                  # Zustand state management
│   ├── types/                  # TypeScript type definitions
│   ├── styles/                 # Global CSS (Tailwind)
│   ├── App.tsx                 # Root app component
│   └── main.tsx                # Entry point
├── src-tauri/                  # Rust backend
│   ├── src/
│   │   ├── commands/           # Tauri command handlers
│   │   ├── core/               # Business logic (scanner, backup, scheduler, fs_utils)
│   │   ├── db/                 # Database connection & schema management
│   │   ├── models/             # Data type definitions (Rust)
│   │   ├── tasks/              # Background tasks (scheduler worker)
│   │   ├── lib.rs              # App setup & command registration
│   │   └── main.rs             # Entry point
│   ├── migrations/             # SQL migration files
│   ├── icons/                  # App icons
│   └── tauri.conf.json         # Tauri configuration
├── scripts/                    # PowerShell utility scripts
├── docs/                       # Documentation
├── .gitignore
├── package.json
├── vite.config.ts
└── tailwind.config.ts
```

## Contributing

Contributions are welcome! Please open an issue or pull request on GitHub.

## License

[MIT](LICENSE)
