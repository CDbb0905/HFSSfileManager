# HFSS File Manager

[English](README.md) | **简体中文**

一个用于管理 HFSS（高频结构仿真）工程项目的桌面应用程序。可以扫描 `.aedt` 项目目录、估算存储占用、添加备注，并执行仅备份 `.aedt` 文件的定时备份策略。

## 功能特性

- **仓库管理** — 添加和管理指向 HFSS 项目位置的扫描目录
- **项目扫描** — 递归扫描仓库中的 `.aedt` / `.aedtresults`，计算文件大小和总存储占用
- **项目列表与搜索** — 浏览所有已扫描项目，支持筛选、排序（按大小/名称）和关键字搜索
- **备注编辑** — 为每个项目添加自定义备注
- **一键备份** — 仅复制 `.aedt` 文件，保留相对目录结构，每次快照生成 `manifest.json`
- **定时策略** — 创建每日或每周备份计划，支持保留份数配置，自动清理过期备份
- **原生文件对话框** — 通过系统对话框浏览和选择目录
- **便携模式** — 数据库优先存储在执行文件旁，不可写入时自动回退到系统应用数据目录

## 截图

<!-- TODO: 在此处添加截图 -->

## 技术栈

| 层级 | 技术 |
|---|---|
| 桌面外壳 | [Tauri 2](https://v2.tauri.app/) |
| 前端 | React 18, TypeScript, Vite |
| 样式 | Tailwind CSS |
| 状态管理 | Zustand |
| 表格与虚拟化 | TanStack React Table, TanStack React Virtual |
| 数据库 | SQLite（rusqlite, bundled） |
| 后端语言 | Rust（edition 2021） |
| Rust 依赖 | serde, chrono, uuid, walkdir, sha2, anyhow, tokio |

## 环境要求（Windows）

1. **Rust**（稳定版）:
   ```bash
   winget install -e --id Rustlang.Rustup
   ```
2. **Visual Studio C++ 构建工具**（如缺少）:
   ```bash
   winget install -e --id Microsoft.VisualStudio.2022.BuildTools
   ```
3. **WebView2 运行时**（Windows 10+ 通常已预装）:
   ```bash
   winget install -e --id Microsoft.EdgeWebView2Runtime
   ```
4. **Node.js**（18+）:

## 快速开始

```bash
# 克隆仓库
git clone https://github.com/CDbb0905/hfss-file-manager.git
cd hfss-file-manager

# 安装前端依赖
npm install

# 开发模式运行
npm run tauri dev

# 构建发布包
npm run tauri build
```

## 使用说明

1. **添加仓库** — 进入"仓库"标签页，添加包含 HFSS 项目（`.aedt` 文件）的根目录
2. **执行扫描** — 点击"全量扫描"递归发现所有 `.aedt` 项目并计算存储大小
3. **浏览项目** — 切换到"工程列表"标签页，搜索、筛选和排序已扫描的项目，编辑各项目的备注
4. **备份数据** — 在"备份中心"标签页中选择仓库和备份根目录，点击"开始备份"。仅复制 `.aedt` 文件并生成 `manifest.json`
5. **定时任务** — 在"计划任务"标签页中配置每日或每周的自动备份策略，设置保留份数

## 项目结构

```
hfss-file-manager/
├── src/                        # React 前端
│   ├── components/             # 共享 UI 组件（Layout）
│   ├── pages/                  # 页面组件（仓库、工程列表、备份中心、计划任务）
│   ├── services/               # API 层（Tauri invoke 封装）
│   ├── store/                  # Zustand 状态管理
│   ├── types/                  # TypeScript 类型定义
│   ├── styles/                 # 全局 CSS（Tailwind）
│   ├── App.tsx                 # 根组件
│   └── main.tsx                # 入口文件
├── src-tauri/                  # Rust 后端
│   ├── src/
│   │   ├── commands/           # Tauri 命令处理器
│   │   ├── core/               # 业务逻辑（扫描、备份、调度、文件工具）
│   │   ├── db/                 # 数据库连接与 Schema 管理
│   │   ├── models/             # 数据类型定义（Rust）
│   │   ├── tasks/              # 后台任务（调度器工作线程）
│   │   ├── lib.rs              # 应用设置与命令注册
│   │   └── main.rs             # 入口
│   ├── migrations/             # SQL 迁移文件
│   ├── icons/                  # 应用图标
│   └── tauri.conf.json         # Tauri 配置
├── scripts/                    # PowerShell 工具脚本
├── docs/                       # 文档
├── .gitignore
├── package.json
├── vite.config.ts
└── tailwind.config.ts
```

## 贡献指南

欢迎贡献代码！请通过 GitHub 提交 Issue 或 Pull Request。

## 许可证

[MIT](LICENSE)
