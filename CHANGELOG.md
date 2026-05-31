# Changelog

## [2.0.0] — 2026-05-26

### Added
- 🔥 Init proyek Flame ADE V2
- 🏗 Tauri 2 + React 19 + Vite 8 scaffolding
- 🎨 Tailwind v4 dengan design tokens dari mockup
- 📁 Struktur folder monorepo (modules, components, hooks, lib, stores)
- ⚙️ Rust backend dengan Tauri plugins (shell, fs, dialog, store, updater, os, opener)
- 🤖 OpenCode configuration (23 agents, 5 MCP servers, 13 commands)
- 📝 Dokumentasi lengkap (AGENTS.md, ARCHITECTURE.md, FLAME.md, WORKFLOW.md, PLAN.md, DESIGN.md, CHANGELOG.md)
- 🔧 Custom skills untuk workflow Flame ADE V2
- 🚀 GitHub Actions CI/CD (test, lint, build, release)

### Phase 1 — UI Shell ✅
- 🔥 App Shell layout (Header + Body + StatusBar + resizable panels)
- 🚦 Header: macOS traffic lights, flame logo, workspace tabs, AI pill, settings
- 📂 Sidebar Rail (9 icon-only buttons: Explorer, Search, Git, History, Preview, Markdown, Extensions, Settings, Account)
- 📁 File Explorer statis (tree, search bar, file/folder icons, badges)
- 🏷 Tab Bar (tabs with colored dot indicators: green/cyan/yellow/indigo)
- 🖥 Terminal Window (block-style output dengan command/output separator)
- 🤖 Flame AI Panel (3D sphere via react-three-fiber, chat messages, slash commands, input bar)
- ✅ Approval Dialog (slide-up modal, action/details, approve/deny)
- 📋 Context Menu (position-aware, keyboard shortcut hints, danger actions)
- 📊 Status Bar (branch, errors, warnings, tests, AI status, encoding, line endings)
- ↔️ Resizable panels (Explorer | Main | AI) via react-resizable-panels v4

### Phase 2 — UI Components ✅
- 🖥 xterm.js terminal with WebGL renderer, Fit, Search, WebLinks addons
- ✎ CodeMirror 6 editor with syntax highlighting (rust, ts, tsx, py, html, css, json, md)
- 🌐 Web Preview panel (iframe with URL bar, sandbox)
- ⎇ Git panel (branches, staged/changed files, commit input, commit history)
- ⚙ Settings page (General, Editor, Terminal, AI, About tabs)
- 🌓 Theme switching (dark/light via CSS variables + data-theme attribute)
- 🔗 Sidebar routing → Git, History, Settings show respective panels
- 📄 Tab routing → terminal, editor, git, preview show correct panels

### Phase 3 — Backend Wiring ✅
- ⚙ Rust commands: read_directory, read_file, write_file, run_command
- 🖥 xterm.js terminal wired to Tauri shell PTY via @tauri-apps/plugin-shell
- 📁 Explorer membaca filesystem real via Tauri commands
- ✎ CodeMirror editor load/save files via Tauri fs commands
- ⎇ Git integration: status, log, branches, commit via git CLI
- 📊 Status Bar menampilkan git branch real
- 🤖 Flame AI provider: OpenRouter API (primary), opencode/gemini CLI (fallback)
- ⚙ Rust AI module dengan reqwest untuk HTTP API calls
- 🔑 AI Settings: API key config, model selector, fallback providers
- 💬 Zustand aiStore untuk conversation state management

### Phase 4 — Polish ✅
- ⌨ Keyboard shortcuts registry (Zustand store-based) + Command Palette (Cmd+K)
- ♿ Accessibility: WCAG AA — focus ring (focus-visible), ARIA labels, landmarks (banner/main/footer/ navigation), role attributes (tab, tablist, log, status, toolbar)
- 🎯 prefers-reduced-motion support (all animations disabled via CSS)
- 📢 Live regions (aria-live) for AI chat loading state
- 🔍 ARIA: sidebar buttons, tab close buttons, search input, terminal, status bar items

### Notes
- UI-first development: semua komponen visual dibangun sebagai static React app sebelum backend
- Berdasarkan desain dari `mock-up FlameV2.html`
- V1 sebagai referensi arsitektur, V2 adalah fresh start dengan pendekatan desain baru
