# UI-First Workflow Skill

Workflow untuk development UI-first Flame ADE V2.

## Prinsip
Semua UI dibangun sebagai static React app (tanpa backend) sebelum backend wiring.

## Workflow
1. **Design Phase**: Analisa mockup, identifikasi komponen
2. **Token Phase**: Ekstrak design tokens (warna, spacing, font) ke Tailwind @theme
3. **Component Phase**: Buat komponen React dengan data statis/props
4. **Interaction Phase**: Tambah state management (Zustand) untuk interaksi frontend-only
5. **Animation Phase**: Tambah motion/animasi
6. **Backend Phase**: Wire backend setelah UI sempurna

## Verification
- Bandingkan dengan mockup secara visual
- Cek pixel-perfect untuk spacing, warna, typography
- Test keyboard navigation
- Test animation timing
