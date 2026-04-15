# `modules/` — Fitur Aplikasi

Setiap subfolder adalah **satu domain/fitur** yang berdiri sendiri dengan 3 layer:

```
modules/<nama_modul>/
├── domain/           ← Layer paling dalam: model + repository trait
│   ├── model/        ← Struct domain (pemetaan ke DB rows via sqlx::FromRow)
│   └── repository/   ← Trait abstraksi akses data (bukan implementasi)
├── application/      ← Business logic: service + DTO
│   ├── *_service.rs  ← Mengorkestrasikan repository untuk memenuhi use case
│   └── dto.rs        ← Request/Response structs dengan validasi
└── presentation/     ← HTTP layer: handler + route
    ├── handlers.rs   ← Axum handler functions
    └── routes.rs     ← Route registration dengan axum::Router
```

## Menambah Modul Baru

Ikuti pola yang sama dengan `auth/` atau `users/`. Setelah selesai:
1. Daftarkan di `modules/mod.rs`: `pub mod nama_modul;`
2. Daftarkan route di `bootstrap/router.rs`
