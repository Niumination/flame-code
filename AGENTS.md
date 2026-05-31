# Flame ADE V2 — Agent Instructions

## Bahasa

Semua komunikasi, komentar, dokumentasi, dan instruksi — termasuk file ini — ditulis dalam **Bahasa Indonesia**, kecuali nama teknis, kode, atau konfigurasi yang bersifat universal (TypeScript, Rust, JSON, dll).

## Commands

| Command | What it runs |
|---------|-------------|
| `pnpm install` | Install dependencies |
| `pnpm tauri dev` | Vite (port **1420**) + Tauri |
| `pnpm typecheck` | `tsc --noEmit` |
| `pnpm build` | `tsc -b && vite build` (project refs emit) |
| `pnpm test` | `vitest` (frontend only, no Rust tests) |
| `pnpm lint` | `eslint .` |
| `cd src-tauri && cargo check` | Rust compilation |
| `cd src-tauri && cargo clippy -- -D warnings` | Rust lint |

## CI (`.github/workflows/ci.yml`)

macOS-only, Node 26, `pnpm install --frozen-lockfile`. Three independent jobs:
- **lint**: `tsc --noEmit` → `eslint src/` → `cargo clippy`
- **test**: `pnpm test` (Vitest) → `cargo test`
- **build** (needs lint+test, push to main/develop/tags only): `pnpm tauri build`
- **release** (needs build, tags only): upload dmg/tar.gz to GitHub release

## Gotchas

- **Port 1420**, not 5173. `vite.config.ts` sets `strictPort: true`.
- **Tailwind v4** via `@tailwindcss/vite` plugin (no PostCSS). Tokens in `src/App.css` via `@theme`.
- **Window**: 1200×740, min 900×500, `hiddenTitle` + `Overlay` (custom traffic lights).
- **CSP** allows `localhost:*` and `ws://localhost:*` for dev. Locked in `tauri.conf.json`.
- **`tsc -b`** (build with project refs) ≠ **`tsc --noEmit`** (typecheck only). Build uses `tsconfig.json` refs → `tsconfig.app.json` + `tsconfig.node.json`.
- TypeScript ~6.0 with `ignoreDeprecations: "6.0"` in `tsconfig.app.json`.
- Single package (`pnpm-workspace.yaml` only has `allowBuilds: esbuild: true`).
- macOS-only bundle target (`dmg`, `app`). Bundle ID: `app.flame.ade.v2`. Version `2.0.0`.

## Architecture

```
src/main.tsx → src/App.tsx
  Header (titleBarStyle: Overlay)
  SidebarRail + ExplorerPanel | GitPanel | SettingsPage
  Group (resizable): Sidebar | Main (TabBar + ActiveTabPanel) | AI
    ActiveTabPanel routes: terminal → XtermTerminal
                          editor → EditorPane (CodeMirror 6, ErrorBoundary)
                          git → GitPanel (ErrorBoundary)
                          preview → PreviewPane (ErrorBoundary)
  StatusBar
  UpdateChecker (auto, 5s delay)
  ApprovalDialog + CommandPalette + ShortcutHandler
```

- **14 frontend modules** in `src/modules/` (ai, automation, editor, explorer, git, header, preview, settings, shortcuts, sidebar, statusbar, tabs, terminal, theme). 6 are lazy-loaded: AiPanel, EditorPane, PreviewPane, GitPanel, SettingsPage, AutomationDashboard.
- **Rust backend** splits into `commands/`, `models/`, `engine/`, plus `ai.rs`, `lib.rs`, `main.rs`. Tauri invoke handlers in `lib.rs` — FS, shell, AI, automation via `headless_chrome`, plus `tauri-plugin-updater`.
- **Tauri plugins**: shell, fs, dialog, opener, store, os, log, updater.

## Conventions

- `@/` path alias for all imports (`@/modules/*`, `@/components/ui/*`, `@/stores/*`, `@/lib/*`, `@/hooks/*`). No relative imports across modules.
- `cn()` from `@/lib/utils` for Tailwind class merging (clsx + tailwind-merge).
- **5 Zustand stores**: `appStore` (persisted), `aiStore`, `automationStore`, `shortcutsStore`, `themeStore` (persisted).
- **shadcn/ui** primitives in `src/components/ui/`. Custom components in `src/modules/`.
- **motion** (framer-motion successor) for animations. **react-resizable-panels** for 3-panel layout.
- Modules in `src/modules/*/` self-contained with own `lib/` hooks, export via `index.ts`.

## Testing

- **3 test files**: `src/lib/utils.test.ts`, `src/stores/appStore.test.ts`, `src/stores/themeStore.test.ts`. 18 tests total, all pass.
- Vitest config: `vite.config.ts` (`environment: jsdom`, `setupFiles: ./src/test/setup.ts`). localStorage mocked in setup for persist middleware.
- Only pure logic tested (stores, utilities). React components with Tauri `invoke` are not tested (mock-heavy, not stable).
- No Rust tests.

## Code Quality Order (local dev)

1. `pnpm exec tsc --noEmit` (TypeScript typecheck)
2. `cd src-tauri && cargo check` (Rust compilation)
3. `cd src-tauri && cargo clippy -- -D warnings` (Rust lint)
4. `pnpm test` (Vitest)
5. `pnpm lint` (ESLint) — config ignores `dist` and `src-tauri/target`

## Phase Status

PLAN.md checkboxes are stale (`🔲`). All 5 phases complete per code + CHANGELOG. Next work: **Automation Dashboard** (not listed in PLAN.md).

## Skill Usage

Load skill secara otomatis saat tugas cocok dengan deskripsi skill — tanpa perlu diminta manual. Skill bersifat global di OpenCode, tidak terbatas proyek ini. Contoh trigger:

| Skill | Trigger |
|-------|---------|
| `kune` | Implementasi kompleks multi-langkah (Rust + TypeScript) |
| `engon` | Verifikasi proyek menyeluruh |
| `shadcn-ui` | Membuat/mengedit komponen UI shadcn/ui |
| `agent-browser` | Browser QA, testing UI, screenshot |
| `threejs` | 3D sphere visualizer, Three.js/r3f di AI Panel |
| `apple-hig` | macOS-specific UI (traffic lights, window chrome, menu) |
| `color-expert` | Palette generation, design token warna |
| `design-systems` | Design tokens, component specs, theming system |
| `frontend-design` | Layout, typography, responsive design |
| `web-design-guidelines` | Product UI best practices (Vercel) |

## OpenCode Config

- `opencode.json`: 10 MCP servers (github, playwright, memory, fetch, tavily, lsp, lighthouse, cargo-audit, flame-release, sequential-thinking), 22 subagents, 13 commands, 4 plugins (notify, quota, snip, dcp).
- Auto-loads: `ARCHITECTURE.md`, `WORKFLOW.md`, `PLAN.md`, `FLAME.md`, `DESIGN.md`, `AGENTS.md`, `.opencode/instructions/shell-strategy.md` — do not duplicate their content here.
- Custom Rust MCP servers at `src-tauri/mcp/` (`cargo-audit`, `flame-release`). Requires pre-built binaries.
