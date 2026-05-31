# Flame ADE V2 — Architecture

## Two-Process Model

Flame ADE V2 menggunakan two-process architecture: Rust backend mengontrol akses OS, React webview berkomunikasi melalui Tauri IPC layer.

```
+-----------------------------------------+
|           Webview (React 19)            |
|  Terminal  |  Editor  |  Explorer  | AI |
+----------------------+------------------+
                        | invoke() / Channel
+----------------------+------------------+
|          Rust Backend (Tauri 2)         |
|  PTY | FS | Shell | Git | Secrets | Net |
+-----------------------------------------+
```

### Kenapa Two Processes?
- **Security**: Webview tidak bisa langsung akses filesystem
- **Isolation**: Crash di satu proses tidak mempengaruhi yang lain
- **Performance**: Rust handle I/O berat, React handle UI rendering
- **Cross-platform**: Tauri mengabstraksi OS differences

## Layout Architecture

Berdasarkan mockup Flame ADE V2:

```
┌─────────────────────────────────────────────────────────────┐
│  Header (38px) — traffic lights | logo | tabs | AI pill    │
├────┬──────────────────────────────────────┬─────────────────┤
│    │   Tab Bar (34px) — inner tabs        │                 │
│    ├──────────────────────────────────────┤                 │
│ S  │                                      │    AI Panel     │
│ i  │   Main Workspace                     │    (300px)      │
│ d  │   (Terminal | Editor | Preview)      │                 │
│ e  │                                      │  3D Visualizer  │
│ b  │   Block-based terminal output        │  Chat Messages  │
│ a  │                                      │  Slash Commands │
│ r  │                                      │  Input Area     │
│    │                                      │                 │
│ 48px│                                      │                 │
├────┴──────────────────────────────────────┴─────────────────┤
│  Status Bar (22px) — branch | errors | AI status            │
└─────────────────────────────────────────────────────────────┘
```

### Key Layout Dimensions (dari mockup)
- App: 1200x740px
- Sidebar: 48px
- Explorer: 220px (collapsible)
- AI Panel: 300px (resizable)
- Header: 38px
- Tab Bar: 34px
- Status Bar: 22px

### Tab System
- **Tagged-union types**: `terminal | editor | preview | ai-diff | git | settings`
- **Hidden on switch** (not unmounted): PTY sessions survive via invisible pointer-events-none
- **Visual feedback**: active tab distinct dengan dot indicators

## Module Architecture

### Frontend Modules (React)
Setiap module di `src/modules/` self-contained:
- Exports via index.ts
- Owns its hooks under lib/
- State via Zustand store

### Rust Backend Modules (`src-tauri/src/`)
- `pty/` — PTY session management
- `fs/` — Filesystem operations (tree, file, mutate, search, grep)
- `shell/` — Shell command execution
- `git/` — Git operations
- `secrets/` — OS keychain
- `net/` — SSRF-safe HTTP client

## UI Component Architecture
- **shadcn/ui** — primitives di `src/components/ui/`
- **Custom components** — terminal, explorer, sidebar, ai panel
- **Tailwind v4** — tokens di `src/App.css` via `@theme`
- **motion** — animations (Framer Motion successor)
- **react-resizable-panels** — resizable layouts
- **cn()** dari `@/lib/utils` untuk class merging

## State Management
- **Zustand** untuk global state (tabs, settings, AI sessions)
- **React Context** untuk theme, live context
- **tauri-plugin-store** untuk persistent settings

## AI Architecture
- BVOK (Bring Your Own Key) — opencode.ai, gemini-cli, openrouter
- Agent dengan tools: read/write file, search, shell execution
- Tool approval flow untuk operasi berbahaya
- Live context bridge ke terminal aktif
