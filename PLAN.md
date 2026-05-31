# Flame ADE V2 — Master Development Plan

## Visi
Flame ADE V2 adalah AI-Native Terminal & Development Environment — workspace lengkap untuk mengembangkan aplikasi/situs dari nol hingga production dalam satu tempat. Terinspirasi dari Terax AI dengan pendekatan UI-first, desain gelap dengan aksen flame/indigo.

## Target Sistem
- **Primary**: macOS Tahoe 26.5 (Hackintosh Intel + Apple Silicon)
- **Secondary**: macOS 10.15+

## Tech Stack
| Layer | Teknologi |
|-------|-----------|
| **Backend** | Rust (Tauri 2) |
| **Frontend** | React 19, TypeScript, Vite 8 |
| **Terminal** | @xterm/xterm 6 + WebGL |
| **Editor** | CodeMirror 6 |
| **AI SDK** | opencode.ai, gemini-cli, openrouter |
| **UI** | Tailwind v4, shadcn/ui + Radix, motion |
| **State** | Zustand 5 |
| **Panels** | react-resizable-panels |
| **3D** | react-three-fiber + drei |
| **Font** | Inter Variable + JetBrains Mono + Cascadia Code |
| **Package Manager** | pnpm |

## Development Phases

### Phase 0: Scaffolding ✅
- [x] Tauri 2 + React 19 + Vite 8 scaffolding
- [x] Install dependencies (xterm, codemirror, shadcn/ui, zustand, motion, three, dll)
- [x] Tailwind v4 dengan design tokens dari mockup
- [x] Rust backend dengan Tauri plugins (shell, fs, dialog, store, updater, os, opener)
- [x] OpenCode configuration (23 agents, MCP servers, commands)
- [x] Project documentation (AGENTS, ARCHITECTURE, WORKFLOW, FLAME, PLAN, DESIGN, CHANGELOG)
- [x] Folder structure (modules, components, hooks, lib, stores, styles)
- [x] CI/CD GitHub Actions (test, lint, build, release)

### Phase 1: UI Shell 🔲
- [ ] App Shell layout (Header + Body + StatusBar)
- [ ] Header: traffic lights, flame logo, workspace tabs, AI pill, settings btn
- [ ] Sidebar Rail (icon-only: Explorer, Search, Git, History, Preview, Markdown, Extensions, Settings, Account)
- [ ] File Explorer tree statis dengan folder/file icons, search bar
- [ ] Tab Bar dengan dot indicators (green/cyan/yellow/indigo)
- [ ] Status Bar (branch, errors, warnings, tests, AI status)
- [ ] Resizable panels (Explorer | Main | AI)

### Phase 2: UI Components 🔲
- [ ] Terminal Window (header + block-style output command/separator/active cursor)
- [ ] Terminal block highlights (AI suggestion, success/error output)
- [ ] Flame AI Panel header + model selector
- [ ] 3D sphere visualizer + particles + AI status
- [ ] AI chat messages (user/AI/streaming states)
- [ ] Slash command chips (/explain, /fix, /test, /refactor, /docs)
- [ ] AI input area (textarea, attach, voice, context, send buttons)
- [ ] Approval Dialog (warning, file context, timer bar, approve/reject)
- [ ] Context Menu (New File, Rename, Copy Path, Delete, dll)

### Phase 3: Interaksi & Animasi 🔲
- [ ] Tab switching (header tabs + inner tab bar)
- [ ] Sidebar button active states + panel toggle
- [ ] File tree expand/collapse folder
- [ ] Keyboard navigation (up/down in file tree)
- [ ] Resizable panel drag handles
- [ ] Slash chip click → populate input
- [ ] Send button → simulate message
- [ ] Animasi: panel slide, sphere spin, particle float, dialog slide-up, cursor blink, streaming dots, status pulse

### Phase 4: Backend Wiring 🔲
- [ ] PTY + xterm.js terminal (Tauri shell plugin + xterm addons)
- [ ] File system operations (read, write, tree, search)
- [ ] CodeMirror 6 editor (syntax highlighting, multi-language)
- [ ] Git integration (status, diff, stage, commit, log, branches)
- [ ] Flame AI agent (opencode.ai / gemini-cli / openrouter)
- [ ] Tool approval flow (real backend validation)
- [ ] Web Preview (iframe, dev server detection)

### Phase 5: Polish 🔲
- [ ] Keyboard shortcuts registry
- [ ] Settings page (theme, font, AI provider config)
- [ ] Theme switching (multiple themes)
- [ ] Accessibility (WCAG AA, keyboard nav, screen reader)
- [ ] Performance optimization (lazy loading, memo)
- [ ] Security hardening (path guards, CSP, sanitization)
- [ ] Auto-updater

## Milestone Checklist

### v2.0.0-alpha.1 — App Shell
- [ ] App layout + Header + Sidebar + Tab Bar + Status Bar
- [ ] File explorer tree
- [ ] Terminal window (static)
- [ ] AI Panel (static)
- [ ] Approval Dialog
- [ ] Context Menu

### v2.0.0-alpha.2 — Interactions
- [ ] Tab switching
- [ ] Panel toggle
- [ ] Resizable panels
- [ ] Animations
- [ ] Keyboard navigation

### v2.0.0-beta.1 — Backend Integration
- [ ] PTY terminal
- [ ] File explorer + editor real
- [ ] Git operations
- [ ] AI agent real
- [ ] Web preview real

### v2.0.0-rc.1 — Polish
- [ ] Keyboard shortcuts
- [ ] Settings
- [ ] Themes
- [ ] Accessibility
- [ ] Security
- [ ] Auto-updater

### v2.0.0 — Stable
- [ ] Production-ready release
- [ ] Full documentation
- [ ] CI/CD pipeline
