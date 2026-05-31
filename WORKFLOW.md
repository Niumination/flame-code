# Flame Code — Development Workflow

## Prerequisites

### System Requirements
- **OS**: macOS Tahoe 26.5 (primary)
- **Hardware**: Intel Core i5-10310U or better
- **RAM**: 8GB minimum, 16GB recommended

### Software Requirements
1. **Rust** (stable) — `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. **Node.js 20+** — `brew install node`
3. **pnpm** — `brew install pnpm`
4. **Tauri prerequisites** — `brew install pkg-config`
5. **OpenCode** — `brew install anomalyco/tap/opencode`

## Daily Workflow

### 1. Start Development Session
```bash
cd ~/Documents/ZMP/Flame-V2
opencode
```

### 2. Install Dependencies
```bash
pnpm install
```

### 3. Start Dev Server
```bash
pnpm tauri dev
```
Or use OpenCode command: `/dev`

### 4. Work on Features
- Gunakan **Build agent** (default) untuk implementation
- Gunakan **Plan agent** (Tab key) untuk planning
- Gunakan **@subagents** untuk specialized tasks

### 5. Check Code Quality
```bash
pnpm exec tsc --noEmit          # TypeScript type check
cd src-tauri && cargo clippy    # Rust lint
```
Or use OpenCode command: `/lint`

### 6. Run Tests
```bash
pnpm test
```
Or use OpenCode command: `/test`

## OpenCode Commands

| Command | Description | Agent |
|---------|-------------|-------|
| `/dev` | Start Tauri dev server | build |
| `/build` | Build production bundle | build |
| `/test` | Run test suite | qa-tester |
| `/lint` | Run linting and type checks | qa-tester |
| `/clippy` | Run cargo clippy | rust-dev |
| `/release` | Prepare new release | release-manager |
| `/audit` | Security audit | security-auditor |
| `/architect` | Architecture review | architect |
| `/next` | Build next pending task | build |
| `/phase` | Build entire phase | build |
| `/design` | Design workflow | build |
| `/document` | Update documentation | docs-writer |
| `/engon` | Project verification | build |

## Branch Strategy
```
main          — stable releases
develop       — integration branch
feature/*     — new features
fix/*         — bug fixes
```

## Code Review Process
1. Self-review: check types + lint + tests
2. Security review: use `@security-auditor`
3. Architecture review: use `@architect`

## Release Process
1. Update CHANGELOG.md
2. Bump version in package.json and Cargo.toml
3. Run full test suite
4. Build: `pnpm tauri build`
5. Create git tag
6. Push to GitHub

## UI Development Workflow (UI-First)
1. **Evaluate**: lihat app yang sedang berjalan, identifikasi pain points
2. **Implement**: buat komponen React + Tailwind v4
3. **Review**: bandingkan dengan kebutuhan use case
4. **Iterate**: perbaiki berdasarkan feedback
