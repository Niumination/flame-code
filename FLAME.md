# FLAME.md — Project Memory

Flame ADE V2 loads `FLAME.md` dari workspace root sebagai agent memory. Dokumen ini mencatat identitas proyek, perbedaan dari V1, dan keputusan desain.

---

## Project Identity

- **Name**: Flame ADE V2
- **Bundle ID**: `app.flame.ade.v2`
- **Inspirasi**: Terax AI dengan perubahan selera
- **Tech Stack**: Tauri 2, Rust, React 19, TypeScript, Tailwind v4, xterm.js, CodeMirror 6, Zustand
- **AI Providers**: opencode.ai, gemini-cli, openrouter
- **Platform Utama**: macOS (Intel + Apple Silicon)
- **Package Manager**: pnpm

---

## Perbedaan dari Flame ADE V1

| # | Aspek | V1 | V2 |
|---|-------|----|----|
| 1 | **Pendekatan** | Backend-first, porting dari Terax AI | UI-first, desain dari mockup |
| 2 | **Tema** | Indigo/primary #4338ca | Flame/amber #ff6a00 accent dominan |
| 3 | **Sidebar** | Text labels | Icon-only rail (48px) |
| 4 | **AI Panel** | Chat standar | 3D sphere visualizer + gradient |
| 5 | **Terminal** | Standard xterm | Block-based command/output |
| 6 | **Layout** | Standard panel | Gradient glow header, flame accents |
| 7 | **Font** | Inter + JetBrains Mono | + Cascadia Code |
| 8 | **Provider** | 12+ AI providers | 3 utama (opencode.ai, gemini-cli, openrouter) |
| 9 | **Platform** | Cross-platform | macOS first |
| 10 | **Arsitektur** | Monorepo fork Terax | Monorepo fresh start |

---

## Design Decisions

### D1: UI-First Development
Keputusan untuk membangun semua komponen visual sebagai static React app sebelum backend. Alasan: mockup sudah jelas, UI adalah prioritas utama. Backend Tauri menyusul setelah UI sempurna.

### D2: Hybrid Components
shadcn/ui untuk complex primitives (dialog, select, tooltip), custom components untuk yang simpel. Menyeimbangkan speed development dengan kontrol desain.

### D3: 3 Provider AI
Hanya opencode.ai, gemini-cli, openrouter — cukup untuk MVP. Provider tambahan bisa ditambah nanti via plugin system.

### D4: Flame Color Identity
Palette flame (#ff6a00 → #ff9f45 → #ffd080) sebagai identitas visual utama, dikombinasikan dengan indigo (#6c7cff) untuk kontras. Inspirasi dari api/cahaya.

### D5: macOS Tahoe Priority
Focus pada macOS Tahoe 26.5 (Hackintosh + Intel). Cross-platform setelah stabil di macOS.

---

## Development System

- **OS**: Hackintosh macOS Tahoe 26.5
- **Hardware**: ThinkPad X13 Yoga Gen 1, Intel Core i5-10310U
- **Shell**: zsh
- **AI Provider**: opencode.ai

## Commands

```bash
pnpm install                    # Install dependencies
pnpm tauri dev                  # Development mode
pnpm tauri build                # Production build
pnpm exec tsc --noEmit          # TypeScript type check
cd src-tauri && cargo check     # Rust compilation check
cd src-tauri && cargo clippy    # Rust lint
pnpm test                       # Run tests (vitest)
```
