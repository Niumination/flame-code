# Flame Code — Agent Instructions

## Bahasa

Semua komunikasi, komentar, dokumentasi, dan instruksi — termasuk file ini — ditulis dalam **Bahasa Indonesia**, kecuali nama teknis, kode, atau konfigurasi yang bersifat universal (TypeScript, Rust, JSON, dll).

## Identity

- **Nama**: Flame Code
- **Tagline**: *Lit. Fast. Yours.*
- **Bundle ID**: `app.flame.code`
- **Versi**: `0.7.3`
- **Repository**: `github.com/niumination/flame-code`
- **Kategori**: AI-native terminal emulator + code editor
- **Platform**: macOS (Intel + Apple Silicon)

## Commands

| Command | What it runs |
|---------|-------------|
| `pnpm install` | Install dependencies |
| `pnpm tauri dev` | Vite (port **1420**) + Tauri |
| `pnpm typecheck` | `tsc --noEmit` |
| `pnpm build` | `tsc -b && vite build` |
| `pnpm test` | `vitest` (frontend only, 91 tests) |
| `pnpm lint` | `eslint .` |
| `cd src-tauri && cargo check` | Rust compilation |
| `cd src-tauri && cargo clippy -- -D warnings` | Rust lint |
| `cd src-tauri && cargo test` | Rust tests (122 tests) |

## CI (`.github/workflows/ci.yml`)

macOS-only, Node 26, `pnpm install --frozen-lockfile`. Three independent jobs:
- **lint**: `tsc --noEmit` → `eslint src/` → `cargo clippy`
- **test**: `pnpm test` (Vitest) → `cargo test`
- **build** (needs lint+test, push only): `pnpm tauri build`
- **release** (needs build, tags only): upload dmg/tar.gz to GitHub release

## Gotchas

- **Port 1420**, not 5173. `strictPort: true`.
- **Tailwind v4** via `@tailwindcss/vite` plugin (no PostCSS). Tokens in `src/styles/globals.css` via `@theme inline`.
- **OKLCH color space** throughout (not hex). CSS variables via `oklch()`.
- **Window**: 1200×740 default, `hiddenTitle` + `Overlay` (macOS traffic lights). Two windows: main + settings (900×700, always_on_top, Overlay).
- **CSP** allows `localhost:*` and `ws://localhost:*` for dev. Locked in `tauri.conf.json`.
- **Settings window** at `settings.html` entry point, separate build chunk.
- **`tsc --noEmit`** (typecheck) ≠ **`tsc -b`** (build with project refs).
- TypeScript ~6.0 with `ignoreDeprecations: "6.0"` in `tsconfig.json`.
- **Vite chunk splitting** — 14+ named chunks (ai-anthropic, ai-google, xterm, codemirror, motion, react, radix).
- macOS-only bundle target (`dmg`, `multipart.zip`). Bundle ID: `app.flame.code`.

## Architecture

```
src/main.tsx → src/app/App.tsx
  Header (titleBarStyle: Overlay)
  ResizablePanelGroup (sidebar | workspace)
    sidebar: FileExplorer | SourceControlPanel + SidebarRail
    workspace: TerminalStack | EditorStack | PreviewStack | MarkdownStack
               | AiDiffStack | GitDiffStack | GitHistoryStack
  StatusBar
  Toaster + AgentNotificationsBridge + AiComposerProvider
  AiMiniWindow (AnimatePresence) + SelectionAskAi
  ShortcutsDialog + NewEditorDialog + UpdaterDialog
```

- **18 frontend modules** in `src/modules/` (ai, agents, editor, explorer, git-history, header, markdown, preview, settings, shortcuts, sidebar, source-control, statusbar, tabs, terminal, theme, updater, workspace).
- **Rust backend** modules in `src-tauri/src/modules/` — pty, fs, git, shell, plus agent.rs, secrets.rs, net.rs, workspace.rs, proc.rs.
- **34+ Tauri commands** across PTY (5), FS (11), Git (17), Shell (8), Workspace (5), Secrets (4), Net (3), Agent (2).
- **Tauri plugins**: process, updater, window_state, autostart, store, os, notification, log, opener.

## Conventions

- `@/` path alias for all imports (`@/modules/*`, `@/components/ui/*`, `@/lib/*`, `@/app/*`). No relative imports across modules.
- `cn()` from `@/lib/utils` for Tailwind class merging (clsx + tailwind-merge).
- **Zustand stores** per module (no centralized `src/stores/`): aiStore (chatStore, agentsStore, snippetsStore, planStore, todoStore), preferencesStore, shortcutsStore, themeStore, workspaceEnvStore, managedAgentsStore.
- **shadcn/ui** primitives in `src/components/ui/` (35 files). AI elements in `src/components/ai-elements/` (8 files).
- **motion** for animations. **react-resizable-panels** for sidebar layout.
- Modules in `src/modules/*/` self-contained with own `store/`, `lib/`, hooks.

## Tab Types (Tagged Union)

8 variants: `terminal | editor | preview | markdown | ai-diff | git-diff | git-commit-file | git-history`

Hidden layers on switch (not unmount): PTY sessions survive via `invisible pointer-events-none`.

## AI System

- **Vercel AI SDK v6** with 7 providers: Anthropic, OpenAI, Google (Gemini), Groq, xAI, Cerebras, OpenRouter.
- **Local models**: LM Studio, Ollama, MLX, OpenAI-compatible, custom endpoints.
- **Agent system**: tool approval flow, code write/read/search tool, sub-agents, live context bridge (terminal CWD + buffer).
- **Claude TUI hooks**: OSC 777 notify for managed agent spawning.
- **BYOK** — all API keys stored via OS keychain (keyring crate, apple-native).

## Code Quality Order (local dev)

1. `pnpm exec tsc --noEmit` (TypeScript typecheck)
2. `cd src-tauri && cargo check` (Rust compilation)
3. `cd src-tauri && cargo clippy -- -D warnings` (Rust lint)
4. `pnpm test` (Vitest, 91 tests)
5. `cd src-tauri && cargo test` (Rust tests, 122 tests)
6. `pnpm lint` (ESLint) — config ignores `dist` and `src-tauri/target`

## Testing

- **7 test files**: keymap, OSC handlers, PreviewPane, geometry, config, shellQuote, security. 91 tests total.
- Vitest config: `vite.config.ts` (`environment: jsdom`, `setupFiles: ./src/test/setup.ts`).
- **122 Rust tests** in workspace (auth/path resolution, FS operations, shell, PTY).
- Only pure logic tested. React components with Tauri `invoke` are not tested.

## Skill Usage

| Skill | Trigger |
|-------|---------|
| `kune` | Implementasi kompleks multi-langkah (Rust + TypeScript) |
| `engon` | Verifikasi proyek menyeluruh |
| `shadcn-ui` | Membuat/mengedit komponen UI shadcn/ui |
| `agent-browser` | Browser QA, testing UI, screenshot |
| `apple-hig` | macOS-specific UI (traffic lights, window chrome, menu) |
| `color-expert` | Palette generation, design token warna |
| `design-systems` | Design tokens, component specs, theming system |
| `frontend-design` | Layout, typography, responsive design |

## OpenCode Config

- `opencode.json`: 10 MCP servers, 22 subagents, 13 commands, 4 plugins.
- Auto-loads: `ARCHITECTURE.md`, `WORKFLOW.md`, `PLAN.md`, `FLAME.md`, `DESIGN.md`, `AGENTS.md`, `.opencode/instructions/shell-strategy.md`.
- Custom Rust MCP servers at `src-tauri/mcp/` (`cargo-audit`, `flame-release`). Requires pre-built binaries.
