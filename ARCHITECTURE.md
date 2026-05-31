# Flame Code — Architecture

## Two-Process Model

Flame Code menggunakan two-process architecture: Rust backend mengontrol akses OS, React webview berkomunikasi melalui Tauri IPC layer.

```
+-----------------------------------------+
|           Webview (React 19)            |
|  Terminal  |  Editor  |  Explorer  | AI |
+----------------------+------------------+
                        | invoke() / Channel
+----------------------+------------------+
|          Rust Backend (Tauri 2)         |
|  PTY | FS | Git | Shell | Secrets | Net |
+-----------------------------------------+
```

### Kenapa Two Processes?
- **Security**: Webview tidak bisa langsung akses filesystem
- **Isolation**: Crash di satu proses tidak mempengaruhi yang lain
- **Performance**: Rust handle I/O berat, React handle UI rendering
- **Cross-platform**: Tauri mengabstraksi OS differences

## Layout Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Header (38px) — traffic lights | tabs | search | settings │
├────┬──────────────────────────────────────┬─────────────────┤
│    │                                      │                 │
│ S  │  ResizablePanelGroup                 │  ai-input-bar   │
│ i  │  ┌──────┬─────────────────────────┐  │                 │
│ d  │  │File  │  7 Hidden Layers:       │  │                 │
│ e  │  │Expl. │  terminal  editor       │  │                 │
│ b  │  │  or  │  preview  markdown      │  │                 │
│ a  │  │SrcCtl│  ai-diff  git-diff      │  │                 │
│ r  │  │Panel │  git-history            │  │                 │
│    │  └──────┴─────────────────────────┘  │                 │
│ 48px│                                      │                 │
├────┴──────────────────────────────────────┴─────────────────┤
│  Status Bar (22px) — branch | errors | AI status            │
├─────────────────────────────────────────────────────────────┤
│  Floating: Toaster | AiMiniWindow | SelectionAskAi          │
│            ShortcutsDialog | UpdaterDialog                   │
└─────────────────────────────────────────────────────────────┘
```

### Key Dimensions
- App: 1200x740px
- Sidebar: 48px
- Header: 38px
- Status Bar: 22px

## Tab System

- **Tagged-union**: `terminal | editor | preview | markdown | ai-diff | git-diff | git-commit-file | git-history`
- **Hidden on switch** (not unmounted): PTY sessions survive via `invisible pointer-events-none`
- **Multi-pane terminal**: max 8 panes per tab

## Module Architecture

### Frontend Modules (React) — 18 Modul

```
ai, agents, editor, explorer, git-history, header, markdown, preview, settings,
shortcuts, sidebar, source-control, statusbar, tabs, terminal, theme, updater, workspace
```

Setiap module di `src/modules/` self-contained:
- Exports via `index.ts`
- Owns hooks under `lib/`
- State via Zustand store di `store/`

### Rust Backend Modules (`src-tauri/src/`)

```
pty/       — PTY session management
fs/        — Filesystem operations (tree, file, mutate, search, grep)
git/       — Git commands, operations, parser
shell/     — Shell command execution
secrets/   — OS keychain (apple-native)
net/       — SSRF-safe HTTP client
workspace/ — Path authorization, WSL distro management
agent/     — Claude Code hooks
proc/      — Process utilities (Windows hide_console)
```

34+ Tauri commands across all modules.

## UI Component Architecture

- **shadcn/ui** — 35 primitives di `src/components/ui/`
- **AI elements** — 8 components di `src/components/ai-elements/`
- **Tailwind v4** — OKLCH color space via `@theme inline` di `src/styles/globals.css`
- **motion** — animations
- **react-resizable-panels** — resizable layouts
- **cn()** dari `@/lib/utils` untuk class merging (clsx + tailwind-merge)

## State Management

- **Zustand** per module (no centralized `src/stores/`)
- **React Context** untuk theme, live context
- **tauri-plugin-store** untuk persistent settings

## AI Architecture

- **Vercel AI SDK v6** dengan 7 providers: Anthropic, OpenAI, Google (Gemini), Groq, xAI, Cerebras, OpenRouter
- **Local models**: LM Studio, Ollama, MLX, OpenAI-compatible, custom endpoints
- **Agent system**: tool approval flow, code write/read/search tool, sub-agents
- **Live context bridge**: terminal CWD + buffer ke AI session
- **BYOK** (Bring Your Own Key) — semua API key disimpan via OS keychain (keyring crate, apple-native)
- **Claude TUI hooks**: OSC 777 notify untuk managed agent spawning
