# `configs/` — Konfigurasi Aplikasi

Semua konfigurasi di-load dari **environment variables** menggunakan crate `config` dengan separator `__`.

## File

| File | Fungsi |
|---|---|
| `app_config.rs` | `AppConfig::load()` — membaca env vars ke typed structs |
| `database.rs` | Membuat `PgPool` (SQLx connection pool) |
| `jwt.rs` | `JwtKeys` — menyimpan secret + expiry untuk digunakan di service |
| `redis.rs` | Membuat `ConnectionManager` untuk Redis async |

## Cara Konfigurasi

Set environment variables dengan format nested menggunakan `__`:

```bash
SERVER__PORT=8080
DATABASE__URL=postgres://...
JWT__SECRET=your_secret
```

Lihat `.env.example` untuk daftar lengkap semua variabel yang tersedia.
