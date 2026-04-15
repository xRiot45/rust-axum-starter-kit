# `observability/` — Logging, Tracing & Metrics

## File

| File | Fungsi |
|---|---|
| `tracing.rs` | `init_tracing()` — JSON logs di production, pretty-print di development |
| `metrics.rs` | Prometheus metrics layer via `axum-prometheus` |

## Penggunaan

**Log level** dikontrol via env var:
```bash
RUST_LOG=info,sqlx=warn,hyper=warn
```

**Prometheus metrics** tersedia di endpoint `/metrics` setelah `attach_metrics()` dipanggil di `bootstrap/app.rs`.

## Production Recommendations

- Kirim JSON logs ke agregator: **Datadog**, **Grafana Loki**, atau **AWS CloudWatch**
- Scrape `/metrics` dengan Prometheus dan visualisasikan di **Grafana**
- Tambahkan distributed tracing dengan **OpenTelemetry** (`tracing-opentelemetry` crate)
