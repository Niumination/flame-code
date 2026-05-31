# Changelog

## [0.7.3] — 2026-05-31

### Added
- 🔥 Migrasi penuh dari Terax AI v0.7.3 sebagai foundation
- ⚙ Rust backend modules: pty, fs, git, shell, secrets, net, workspace, agent, proc
- 🖥 34+ Tauri commands across all backend modules
- 📦 18 frontend modules: ai, agents, editor, explorer, git-history, header, markdown, preview, settings, shortcuts, sidebar, source-control, statusbar, tabs, terminal, theme, updater, workspace
- 🤖 AI System: Vercel AI SDK v6 with 7 providers (Anthropic, OpenAI, Google, Groq, xAI, Cerebras, OpenRouter) + local models (LM Studio, Ollama, MLX)
- 🧩 35 shadcn/ui primitives + 8 AI elements
- 🎨 Tailwind v4 with OKLCH color system
- 🔄 Multi-pane terminal (max 8 panes per tab)
- ✎ CodeMirror 6 with 9+ languages, vim mode, autocomplete, merge view, 9 themes
- 📁 File explorer with real FS backend, search, file watcher
- ⎇ Git integration: status, diff, stage, commit, push, pull, log, history graph
- 🤖 AI agent system: tool approval flow, code write/read/search tools, sub-agents
- 💬 AI mini window (floating chat)
- 📝 AI snippets + todos (persistent storage)
- 📄 Markdown preview
- 🌐 Web preview (iframe + address bar)
- ⚙ Settings window (separate window, 900x700)
- 🔄 Auto-updater with manual check
- 📌 Source control panel
- 🕹 Git history graph (lane-based visual commit graph)
- 🏗 Workspace environment (WSL/local path authorization)
- ⌨ Keyboard shortcuts dialog (Cmd+K)
- 🌓 17 global themes (9 CodeMirror + custom theme engine)

### Changed
- Nama proyek: `flame-ade-v2` → `flame-code`
- Bundle ID: `app.flame.ade.v2` → `app.flame.code`
- Lisensi: Proprietary → Apache 2.0
- Versi: `2.0.0` → `0.7.3`
- AI System: Flame native module (3 providers) → Vercel AI SDK v6 (7 providers + local models)
- Backend: Flame commands/engine/models → Terax modules (pty, fs, git, shell, secrets, net)
- Tema: Flame indigo → Terax default (akan diganti Gas/Lit)
- Font: Inter + JetBrains Mono + Cascadia Code
- Sidebar: Text labels → Icon-only rail

### Removed
- 🔮 3D sphere visualizer (three.js, react-three-fiber, drei)
- 🔧 Automation Dashboard
- 🎯 Flame ADE V2 design identity (indigo theme, rounded UI)
- 🗑 Tauri plugins: shell, fs, dialog
- 📚 Dependencies: lucide-react, tanstack/react-query, recharts, headless_chrome
