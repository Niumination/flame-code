# Flame Design Tokens Skill

Panduan design tokens Flame ADE V2 yang di-extract dari mockup.

## Token Sources
- Primary: `mock-up FlameV2.html` CSS variables
- Secondary: `DESIGN.md` documentation

## Color Enforcement
```css
--bg-base:    #0d0f14
--bg-surface: #12151c
--bg-raised:  #181c26
--bg-overlay: #1e2333
--flame-1:    #ff6a00
--flame-2:    #ff9f45
--indigo:     #6c7cff
```

## Rules
- Jangan pakai hardcoded colors — always use CSS variables
- Flame gradient: `linear-gradient(135deg, var(--flame-1), var(--flame-2), var(--indigo))`
- All spacing based on 4px unit scale
- Font: Inter Variable untuk UI, JetBrains Mono untuk code
