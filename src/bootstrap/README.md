# `bootstrap/` — Inisialisasi Aplikasi

Folder ini bertanggung jawab untuk **merakit dan memulai aplikasi** Axum. Hanya dijalankan sekali saat startup.

## File

| File | Fungsi |
|---|---|
| `app.rs` | `create_app()` — membuat koneksi DB & Redis, merakit Router + semua middleware |
| `router.rs` | `build_router()` — menggabungkan semua modul route dengan prefix `/api/v1/...` |
| `state.rs` | `AppState` struct — container untuk semua shared dependency (db, redis, jwt, config) |

## Best Practice

- Tambahkan middleware global (rate limiting, auth, dll) di `app.rs`
- Saat menambah modul baru, daftarkan route-nya di `router.rs` saja
- `AppState` harus `Clone` — Axum meng-clone-nya per-request
