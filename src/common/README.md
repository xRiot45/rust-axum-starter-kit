# `common/` — Shared Code

Berisi kode yang digunakan bersama oleh **semua modul**. Tidak boleh mengimport apapun dari `modules/`.

## Sub-folder

| Folder | Isi |
|---|---|
| `errors/` | `AppError` enum + `AppResult<T>` — satu sumber kebenaran untuk error handling |
| `extractors/` | `ValidatedJson<T>` (deserialize + validate), `AuthUser` (parse JWT) |
| `middlewares/` | `cors_layer()`, `UuidRequestId` untuk `x-request-id` header |
| `traits/` | `Repository<T>` dan `Service` marker trait |
| `utils/` | `hash_password`, `verify_password`, `generate_access_token`, `PaginationQuery` |

## Konvensi

- Tambahkan utility baru di sini **hanya** jika dipakai minimal 2 modul berbeda
- Jangan tambahkan business logic ke folder ini — hanya infrastruktur/helper
