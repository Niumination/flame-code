# Flame Code — Design Tokens

Desain "Gas/Lit" aesthetic — dark industrial, neon orange, sharp edges, monospace heavy.

---

## 1. Color System

### 1.1 Surface Hierarchy
| Token | Nilai | Usage |
|-------|-------|-------|
| `--bg-base` | `oklch(0.07 0.01 255)` | Root background |
| `--bg-surface` | `oklch(0.10 0.015 255)` | Sidebar, header, panels |
| `--bg-raised` | `oklch(0.14 0.02 255)` | Inputs, search bars |
| `--bg-overlay` | `oklch(0.18 0.025 255)` | Dialogs, dropdowns |
| `--bg-hover` | `oklch(0.21 0.03 265)` | Hover states |

### 1.2 Brand Palette (Flame/Amber)
| Token | Nilai | Usage |
|-------|-------|-------|
| `--flame-1` | `oklch(0.65 0.22 40)` | Primary brand, active indicators |
| `--flame-2` | `oklch(0.73 0.18 50)` | Secondary brand, hover accents |
| `--flame-3` | `oklch(0.84 0.12 65)` | Subtle brand, glow effects |
| `--flame-dim` | `oklch(0.50 0.15 40)` | Muted brand, inactive states |

### 1.3 Accent Palette (Teal — complementary to flame)
| Token | Nilai | Usage |
|-------|-------|-------|
| `--teal` | `oklch(0.60 0.18 200)` | Interactive accents, links |
| `--teal-dim` | `oklch(0.45 0.12 200)` | Muted interactive |

### 1.4 Semantic Colors
| Token | Nilai | Usage |
|-------|-------|-------|
| `--green` | `oklch(0.60 0.20 145)` | Success, test pass, git staged |
| `--red` | `oklch(0.60 0.22 25)` | Error, delete actions |
| `--yellow` | `oklch(0.75 0.18 90)` | Warning, pending |
| `--cyan` | `oklch(0.65 0.15 210)` | Info, code syntax |

### 1.5 Text Colors
| Token | Nilai | Usage |
|-------|-------|-------|
| `--text-primary` | `oklch(0.92 0.01 255)` | Primary text |
| `--text-secondary` | `oklch(0.65 0.02 255)` | Secondary text, file tree |
| `--text-muted` | `oklch(0.40 0.02 255)` | Muted, placeholders |
| `--text-flame` | `oklch(0.72 0.20 40)` | AI accent text |

### 1.6 Borders
| Token | Nilai | Usage |
|-------|-------|-------|
| `--border` | `oklch(0.20 0.01 255)` | Default borders |
| `--border-active` | `oklch(0.55 0.20 40)` | Active/focus borders |

---

## 2. Typography

### 2.1 Font Stack
```
--font-ui:   -apple-system, BlinkMacSystemFont, 'Inter Variable', sans-serif
--font-mono: 'JetBrains Mono', 'Cascadia Code', 'Fira Code', monospace
```

### 2.2 Font Usage
| Context | Size | Weight | Family |
|---------|------|--------|--------|
| Tab labels | 11.5px | 400 | --font-ui |
| File tree | 12px | 400 | --font-ui |
| Terminal output | 13px | 400 | --font-mono |
| Terminal prompt | 13px | 400 | --font-mono |
| AI messages | 11.5px | 400 | --font-ui |
| Slash chips | 10px | 400 | --font-mono |
| Status bar | 10px | 400 | --font-ui |
| Panel titles | 10px | 700 | --font-ui |

---

## 3. Layout Tokens

| Token | Value | Component |
|-------|-------|-----------|
| `--sidebar-w` | 48px | Sidebar rail |
| `--header-h` | 38px | Top header bar |
| `--statusbar-h` | 22px | Bottom status bar |
| `--tab-h` | 34px | Tab bar height |

### App Shell Dimensions
- App: 1200 × 740px
- Border radius: minimal (sharp edges)

---

## 4. Motion & Animation

### 4.1 Duration
| Animation | Duration | Easing |
|-----------|----------|--------|
| Tab switch | 150ms | ease |
| Panel slide | 200ms | ease |
| Dialog appear | 300ms | ease |
| Hover | 100ms | ease |
| Cursor blink | 1.1s | steps(1) infinite |
| Streaming dots | 0.8s | ease-in-out infinite alternate |
| Status pulse | 2s | infinite |

### 4.2 What Animates
- Tab active: color/border transition
- Hover states: background/border color transition
- Streaming indicator: bouncing dots
- Dialog: slide-up + fade
- Context menu: scale-in

---

## 5. Gas/Lit aesthetic

### Prinsip Desain
1. **Sharp edges** — minimal border-radius (2px max), industrial feel
2. **Monospace heavy** — JetBrains Mono for code, Inter Variable for UI
3. **High contrast** — neon orange (#ff6a00) against near-black backgrounds
4. **Solid surfaces** — no glassmorphism, opaque panels
5. **Terminal-first** — typography over iconography
6. **Dark industrial** — #0d0f14 base, #12151c surfaces, not pure black

### Gas/Lit Vibe
Dark industrial workspace — seperti night mode di shop floor. Bukan "gaming LED" RGB, bukan "apple clean" minimalis. Lebih ke cyberpunk/industrial — tajam, fungsional, no-nonsense.
