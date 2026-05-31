# FLAME.md — Project Memory

Flame Code loads `FLAME.md` dari workspace root sebagai agent memory. Dokumen ini mencatat identitas proyek, perbedaan dari upstream Terax AI, dan keputusan desain.

---

## Project Identity

- **Name**: Flame Code
- **Tagline**: *Lit. Fast. Yours.*
- **Bundle ID**: `app.flame.code`
- **Versi**: `0.7.3`
- **Inspirasi**: Terax AI v0.7.3 — divergence total (tidak ada sync ke depan)
- **Tech Stack**: Tauri 2, Rust, React 19, TypeScript, Tailwind v4, xterm.js, CodeMirror 6, Zustand
- **AI Providers**: Vercel AI SDK v6 — 7 providers (Anthropic, OpenAI, Google, Groq, xAI, Cerebras, OpenRouter) + local models (LM Studio, Ollama, MLX)
- **Platform Utama**: macOS (Intel + Apple Silicon)
- **Package Manager**: pnpm

---

## Perbedaan dari Terax AI

| # | Aspek | Terax AI | Flame Code |
|---|-------|----------|------------|
| 1 | **Pendekatan** | Backend-first, iteratif | Fork total, divergence |
| 2 | **Tema** | Indigo/neutral | Flame/amber #ff6a00 accent |
| 3 | **Sidebar** | Text labels | Icon-only rail (48px) |
| 4 | **Identitas** | Terax brand | Gas/lit aesthetic, sharp edges |
| 5 | **Font** | Inter + JetBrains Mono | Same + Cascadia Code |

---

## Design Decisions

### D1: Divergence Total
Flame Code adalah hard fork dari Terax AI v0.7.3. Tidak akan ada sync upstream. Setiap perubahan adalah milik Flame Code.

### D2: Gas/Lit Aesthetic
Visual identity: dark background, neon orange (#ff6a00) gradient accents, sharp edges, monospace heavy, industrial/cyberpunk feel. BUKAN rounded/glow style Flame ADE V2, BUKAN indigo/neutral Terax.

### D3: Eksplorasi Fitur Bertahap
Flame Code tidak memaksakan fitur baru. Stabilisasi dulu, observasi pain points dari penggunaan sehari-hari, baru fitur khas Flame Code.

### D4: macOS Tahoe Priority
Focus pada macOS Tahoe 26.5. Cross-platform setelah stabil di macOS.

---

## Development System

- **OS**: Hackintosh macOS Tahoe 26.5
- **Hardware**: ThinkPad X13 Yoga Gen 1, Intel Core i5-10310U
- **Shell**: zsh
- **AI Provider**: opencode.ai

## Commands

```bash
pnpm install                    # Install dependencies
pnpm tauri dev                  # Development mode (port 1420)
pnpm tauri build                # Production build
pnpm exec tsc --noEmit          # TypeScript type check
cd src-tauri && cargo check     # Rust compilation check
cd src-tauri && cargo clippy    # Rust lint
pnpm test                       # Run frontend tests (vitest)
cd src-tauri && cargo test      # Run Rust tests (122 tests)
```
