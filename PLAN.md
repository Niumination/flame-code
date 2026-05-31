# Flame Code — Master Development Plan

## Visi
Flame Code adalah AI-native terminal emulator & code editor — workspace lengkap untuk mengembangkan aplikasi dari nol hingga production dalam satu tempat. Desain gas/lit aesthetic: dark, neon orange accents, sharp edges, industrial/cyberpunk feel.

## Target Sistem
- **Primary**: macOS Tahoe 26.5 (Intel + Apple Silicon)
- **Secondary**: macOS 10.15+

## Tech Stack
| Layer | Teknologi |
|-------|-----------|
| **Backend** | Rust (Tauri 2) |
| **Frontend** | React 19, TypeScript, Vite 8 |
| **Terminal** | @xterm/xterm 6 + WebGL |
| **Editor** | CodeMirror 6 |
| **AI SDK** | Vercel AI SDK v6 (7 providers: Anthropic, OpenAI, Google, Groq, xAI, Cerebras, OpenRouter) |
| **UI** | Tailwind v4, shadcn/ui + Radix, motion |
| **State** | Zustand 5 |
| **Panels** | react-resizable-panels |
| **Font** | Inter Variable + JetBrains Mono |
| **Package Manager** | pnpm |

## Current Status
**v0.7.3**. Terax AI v0.7.3 sebagai upstream (divergence total — tidak ada sync ke depan).

---

## Development Phases

### Phase 0: Migrasi ✅
- [x] Rust backend modules (pty, fs, git, shell, secrets, net, workspace, agent, proc)
- [x] 34+ Tauri commands
- [x] Frontend modules (18 modules)
- [x] AI system (Vercel AI SDK, 7 providers, agents)
- [x] All tests passing (122 Rust + 91 Vitest)
- [x] `tsc --noEmit` 0 errors

### Phase 1: Flame Code Identity 🟡
- [ ] Dokumentasi rewrite (AGENTS, ARCHITECTURE, WORKFLOW, PLAN, IDENTITY, DESIGN, CHANGELOG)
- [ ] Gas/Lit default theme — neon orange (#ff6a00), sharp edges, dark industrial
- [ ] App icon/logo
- [ ] Repository URL fix (niumination/flame-code)

### Phase 2: Stabilisasi 🔲
- [ ] ESLint cleanup (100 warnings)
- [ ] Rust component tests
- [ ] More frontend tests (7 → 20+)
- [ ] Accessibility audit (WCAG AA)
- [ ] Security hardening (path traversal, CSP audit)
- [ ] Performance optimization (lazy loading, memo)

### Phase 3: Infrastructure 🔲
- [ ] CI/CD GitHub Actions verification
- [ ] Auto-updater server configuration
- [ ] opencode.json config verification
- [ ] macOS signing (Apple Developer account)

### Phase 4: Flame-Specific Features 🔲
- [ ] Eksplorasi pain points dari penggunaan sehari-hari
- [ ] Fitur khas Flame Code (bukan dari Terax)
- [ ] Custom MCP servers

---

## Milestone Checklist

### v0.7.3 — Flame Code Foundation ✅
- [x] Rust backend migrated from Terax
- [x] Frontend modules migrated from Terax
- [x] All terax references renamed to flame-code
- [x] 0 tsc errors, all tests passing

### v0.8.0 — Flame Code Identity
- [ ] Docs complete and accurate
- [ ] Gas/Lit default theme shipped
- [ ] ESLint clean
- [ ] App icon

### v0.9.0 — Production Ready
- [ ] Accessibility WCAG AA
- [ ] Security hardened
- [ ] CI/CD verified
- [ ] macOS signed build

### v1.0.0 — Stable
- [ ] Production-ready release
- [ ] Flame-specific features
- [ ] Full documentation
- [ ] CI/CD pipeline
