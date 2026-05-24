# Implementation Checklist

## Done in this scaffold

- [x] Tauri + React monorepo structure
- [x] SQLite schema and startup migration
- [x] Repository CRUD (add/list)
- [x] Full scan command for HFSS projects
- [x] Project list + note update command
- [x] Backup command (`.aedt` only, manifest output)
- [x] Schedule CRUD baseline
- [x] Four-page frontend shell (仓库/工程/备份/计划任务)

## Next coding steps

- [ ] Add incremental scan strategy (mtime + existence diff)
- [ ] Add tags tables + UI
- [ ] Add "open folder" command
- [ ] Add backup retention cleanup execution
- [ ] Add background scheduler worker
- [ ] Add project table virtualization with TanStack
- [ ] Add structured logs and error code mapping
