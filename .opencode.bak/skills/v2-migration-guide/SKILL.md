# V2 Migration Guide Skill

Panduan migrasi dari Flame ADE V1 ke V2.

## Perbedaan Utama
1. **UI-First**: V2 dibangun UI dulu, V1 backend dulu
2. **Flame Theme**: V2 punya design tokens sendiri (bukan copy Terax)
3. **Leaner**: Hanya 3 AI providers (V1: 12+), fokus kualitas
4. **macOS First**: V2 macOS only, V1 cross-platform

## Code Migration
- Jangan copy seluruh file dari V1
- Ambil inspirasi arsitektur dari V1 (module structure, store pattern)
- Tulis ulang komponen dengan design tokens V2
- Backend plugins: sama (Tauri 2), konfigurasi disesuaikan

## Files to Reference from V1
- `/Users/zaryu/Desktop/Flame ADE/src/modules/*` — module patterns
- `/Users/zaryu/Desktop/Flame ADE/src-tauri/src/*` — backend patterns
- `/Users/zaryu/Desktop/Flame ADE/ARCHITECTURE.md` — architecture patterns
