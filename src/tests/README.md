# `tests/` — Unit & Integration Tests

## Struktur

- **Unit tests** — letakkan di file yang sama dengan kode (`#[cfg(test)] mod tests { ... }`) atau di `tests/mod.rs`
- **Integration tests** — gunakan folder `tests/` di root project (di luar `src/`)

## Menjalankan Test

```bash
cargo test                           # semua test
cargo test -- --nocapture            # dengan stdout
cargo test user_tests                # filter by name
cargo test -- --test-threads=1       # serial (untuk test yang share state)
```

## Test Database

Untuk integration test dengan PostgreSQL, set `TEST_DATABASE_URL`:

```bash
TEST_DATABASE_URL=postgres://... cargo test
```

Gunakan `sqlx::test` macro untuk auto-rollback setiap test case.
