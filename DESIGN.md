# Flame ADE V2 — Design Tokens

Desain berdasarkan file `mock-up FlameV2.html`. Semua token di-extract dari CSS mockup.

---

## 1. Color System

### 1.1 Surface Hierarchy
| Token | Hex | Usage |
|-------|-----|-------|
| `--bg-base` | `#0d0f14` | Root background |
| `--bg-surface` | `#12151c` | Sidebar, header, panels |
| `--bg-raised` | `#181c26` | Inputs, search bars |
| `--bg-overlay` | `#1e2333` | Dialogs, dropdowns |
| `--bg-hover` | `#232840` | Hover states |

### 1.2 Brand Palette
| Token | Hex | Usage |
|-------|-----|-------|
| `--flame-1` | `#ff6a00` | Primary brand, active indicators |
| `--flame-2` | `#ff9f45` | Secondary brand, hover accents |
| `--flame-3` | `#ffd080` | Subtle brand, glow effects |
| `--indigo` | `#6c7cff` | Contrast accent, links |
| `--indigo-dim` | `#3d47b0` | Muted contrast |

### 1.3 Semantic Colors
| Token | Hex | Usage |
|-------|-----|-------|
| `--green` | `#3ddc84` | Success, test pass, git staged |
| `--red` | `#ff5f5f` | Error, delete actions |
| `--yellow` | `#f5c842` | Warning, pending |
| `--cyan` | `#3dcddc` | Info, code syntax |

### 1.4 Text Colors
| Token | Hex | Usage |
|-------|-----|-------|
| `--text-primary` | `#e8eaf0` | Primary text |
| `--text-secondary` | `#8890a8` | Secondary text, file tree |
| `--text-muted` | `#4e5470` | Muted, placeholders |
| `--text-flame` | `#ff8c30` | AI accent text |

### 1.5 Borders
| Token | Hex | Usage |
|-------|-----|-------|
| `--border` | `rgba(255,255,255,0.07)` | Default borders |
| `--border-active` | `rgba(255,165,50,0.45)` | Active/focus borders |

---

## 2. Typography

### 2.1 Font Stack
```css
--font-ui:   -apple-system, BlinkMacSystemFont, 'Inter Variable', 'Segoe UI', sans-serif
--font-mono: 'JetBrains Mono', 'Cascadia Code', 'Fira Code', monospace
```

### 2.2 Font Usage dari Mockup
| Context | Size | Weight | Family |
|---------|------|--------|--------|
| Brand name | 12px | 600 | --font-ui |
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
| `--explorer-w` | 220px | File explorer panel |
| `--ai-w` | 300px | AI panel |
| `--header-h` | 38px | Top header bar |
| `--statusbar-h` | 22px | Bottom status bar |
| `--tab-h` | 34px | Tab bar height |

### App Shell Dimensions
- App: 1200 × 740px
- Border radius: 14px
- Content inside: flex layout

---

## 4. Motion & Animation

### 4.1 Duration
| Animation | Duration | Easing |
|-----------|----------|--------|
| Tab switch | 150ms | ease |
| Panel slide | 200ms | ease |
| Dialog appear | 300ms | ease |
| Hover | 100ms | ease |
| Logo pulse | 3s | ease-in-out infinite |
| Sphere spin | 6s | linear infinite |
| Sphere pulse | 3s | ease-in-out infinite |
| Particle float | 3.2-5.5s | ease-in-out infinite |
| Cursor blink | 1.1s | steps(1) infinite |
| Streaming dots | 0.8s | ease-in-out infinite alternate |
| Status pulse | 2s | infinite |

### 4.2 What Animates
- **Logo**: gradient + box-shadow pulse
- **AI Sphere**: conic gradient spin + box-shadow pulse
- **Particles**: float up with random delays
- **Status dot**: green pulse
- **Terminal cursor**: blink
- **Streaming indicator**: 3 bouncing dots
- **Dialog**: slide-up + fade in
- **Context menu**: scale-in
- **Tab active**: color/border transition
- **Hover states**: background/border color transition

---

## 5. Component Design dari Mockup

### 5.1 Header
- Traffic lights: red/yellow/green dots with glow
- Flame logo: 20px, gradient background, pulse animation
- Workspace tabs: 28px height, active = bg-base + border-bottom invisible
- AI pill: green pulse dot + model name
- Actions: AI Chat button with flame gradient, settings gear

### 5.2 Sidebar Rail
- 48px wide, icon-only buttons
- Active state: flame-2 color, left gradient indicator bar
- Badge: 14px circle notification badge (Git: 3)
- Bottom section: Extensions, Settings, Account

### 5.3 File Explorer
- Panel header: uppercase "EXPLORER" title + action icons
- Search bar: raised background, search icon, input field
- File tree: indent levels, folder arrows, file icons
- States: active (flame highlight), modified (yellow M badge), new (green U badge)
- Context menu: on right-click, with keyboard shortcuts

### 5.4 Terminal Window
- Header: title "Terminal — zsh", Split/New buttons, cwd path
- Body: block-based output
  - Prompt: ❯ + directory + git status + command
  - Output: dim secondary color
  - Success: green
  - AI highlight: left border + flame-1 accent
  - Cursor: blinking flame-2 block
- Tab bar: tabs with colored dot indicators (green=term, cyan=edit, yellow=git, indigo=prev)

### 5.5 AI Panel
- Header: flame icon + "Flame AI" title + model selector
- 3D Visualizer: 
  - Sphere: conic gradient (flame → indigo → flame), spinning, pulsing glow
  - Particles: floating dots with random colors
  - Status: "AI READY" with green pulse dot
- Messages: user (indigo bg) | AI (flame bg) | streaming (bouncing dots)
- Slash commands: chips /explain /fix /test /refactor /docs
- Input: textarea + footer buttons (attach, voice, context, send)
- Send button: flame → indigo gradient

### 5.6 Approval Dialog
- Position: fixed bottom center
- Warning header, file context, code preview
- Timer bar: gradient (flame → yellow), decreasing
- Actions: Reject (red outline) | Approve (green outline)

### 5.7 Status Bar
- Left: branch (cyan), errors (red), warnings (yellow), tests (green)
- Right: Rust version, encoding, spaces, AI status with pulse
- Background: flame + indigo gradient overlay

### 5.8 Context Menu
- Items: New File, New Folder, Rename, Copy Path, Find in Files, Delete
- Keyboard shortcuts displayed
- Danger item (Delete) in red
