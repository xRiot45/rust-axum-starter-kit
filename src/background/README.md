# `background/` — Background Jobs & Scheduled Tasks

## File

| File | Fungsi |
|---|---|
| `scheduler.rs` | Inisialisasi `tokio-cron-scheduler` dengan daftar cron jobs |
| `jobs/email_job.rs` | Pengiriman email async via `lettre` |
| `jobs/audit_job.rs` | Audit logging async tanpa memblokir request path |

## Menambah Job Baru

1. Buat file baru di `jobs/`, contoh: `jobs/report_job.rs`
2. Daftarkan di `jobs/mod.rs`: `pub mod report_job;`
3. Tambahkan ke scheduler di `scheduler.rs`:

```rust
scheduler.add(Job::new_async("0 0 8 * * MON", |_, _| Box::pin(async {
    report_job::generate_weekly_report().await.ok();
}))?).await?;
```

## Cron Syntax

```
sec  min  hour  day  month  weekday
 *    *    *    *      *       *
```
