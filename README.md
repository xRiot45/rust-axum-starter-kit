# 🦀 Rust Axum Starter Kit

> **Production-ready** backend starter kit untuk membangun RESTful API skala menengah hingga enterprise menggunakan [Axum](https://github.com/tokio-rs/axum) + [Tokio](https://tokio.rs/) + [SQLx](https://github.com/launchbadge/sqlx).

[![Rust](https://img.shields.io/badge/rust-1.77+-orange.svg)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/axum-0.7-blue.svg)](https://github.com/tokio-rs/axum)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

---

## 📑 Daftar Isi

- [Project Overview](#-project-overview)
- [Tech Stack](#-tech-stack)
- [Arsitektur](#-arsitektur)
- [Struktur Folder](#-struktur-folder)
- [Quick Start](#-quick-start)
- [Environment Variables](#-environment-variables)
- [Menjalankan Project](#-menjalankan-project)
- [Migrasi Database](#-migrasi-database)
- [Build & Deploy](#-build--deploy)
- [Menambah Modul Baru](#-menambah-modul-baru)
- [Testing](#-testing)
- [API Endpoints](#-api-endpoints)

---

## 📌 Project Overview

Starter kit ini menyediakan fondasi yang solid untuk membangun backend API yang:

- **Scalable** — struktur modular memudahkan penambahan fitur baru tanpa menyentuh kode lama
- **Maintainable** — separation of concern yang ketat antar layer (presentation → application → domain)
- **Observable** — structured logging (JSON di production, pretty-print di development), Prometheus metrics ready
- **Secure** — password hashing dengan Argon2, JWT access + refresh token, middleware CORS yang dapat dikonfigurasi
- **Production-ready** — Dockerfile multi-stage, docker-compose dengan health checks, konfigurasi lewat environment variables

---

## 🛠 Tech Stack

| Kategori | Library | Keterangan |
|---|---|---|
| **Web Framework** | `axum 0.7` | Ergonomis, composable, dibangun di atas Tower |
| **Async Runtime** | `tokio 1` | Runtime async paling mature untuk Rust |
| **Database ORM** | `sqlx 0.7` | Compile-time SQL verification, async, PostgreSQL |
| **Caching** | `redis 0.25` | Connection manager async untuk session & cache |
| **Auth** | `jsonwebtoken 9` | JWT encode/decode; `argon2 0.5` untuk password hash |
| **Validasi** | `validator 0.18` | Anotasi derive pada struct DTO |
| **Serialisasi** | `serde + serde_json` | Standar de-facto Rust untuk JSON |
| **Konfigurasi** | `config 0.14` + `dotenvy` | Layered config dari env vars dan file `.env` |
| **Tracing** | `tracing` + `tracing-subscriber` | Structured logging; JSON di prod, pretty di dev |
| **Metrics** | `axum-prometheus 0.6` | Prometheus scrape endpoint `/metrics` |
| **Error** | `thiserror` + `anyhow` | Typed errors di library code, anyhow di binary |
| **Middleware** | `tower-http` | CORS, compression, timeout, request tracing |
| **Background Jobs** | `tokio-cron-scheduler` | Cron-based scheduled tasks |
| **Email** | `lettre 0.11` | SMTP async email sending |
| **HTTP Client** | `reqwest 0.12` | Untuk webhook dan integrasi external API |

---

## 🏗 Arsitektur

Project ini menerapkan **Layered Architecture** (mirip Clean Architecture / Hexagonal) dengan struktur per-modul (domain-driven):

```
┌─────────────────────────────────────────────────────────────┐
│                      HTTP Request                           │
└───────────────────────────┬─────────────────────────────────┘
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              PRESENTATION LAYER                             │
│    handlers.rs — menerima request, return response          │
│    routes.rs   — mendaftarkan endpoint ke Axum Router       │
└───────────────────────────┬─────────────────────────────────┘
                            ▼
┌─────────────────────────────────────────────────────────────┐
│             APPLICATION LAYER                               │
│    *_service.rs — orchestrates business logic               │
│    dto.rs       — request/response data transfer objects    │
└───────────────────────────┬─────────────────────────────────┘
                            ▼
┌─────────────────────────────────────────────────────────────┐
│               DOMAIN LAYER                                  │
│    model/     — struct domain + sqlx::FromRow               │
│    repository/ — trait abstraksi akses data                 │
└───────────────────────────┬─────────────────────────────────┘
                            ▼
┌─────────────────────────────────────────────────────────────┐
│            INFRASTRUCTURE (Database / Redis)                │
└─────────────────────────────────────────────────────────────┘
```

**Prinsip utama:**
- Setiap layer hanya boleh bergantung ke layer di bawahnya, **tidak pernah ke atas**
- `domain` layer **tidak boleh** import apapun dari `application` atau `presentation`
- Repository adalah **trait** — implementasi konkret bisa diganti tanpa mengubah service
- `AppState` di-inject via Axum `State<AppState>`, **bukan** global variable

---

## 📁 Struktur Folder

```
rust-axum-starter-kit/
├── src/
│   ├── main.rs                        # Entry point — Tokio runtime + bootstrap
│   │
│   ├── bootstrap/                     # Inisialisasi aplikasi
│   │   ├── app.rs                     # Merakit Router + middleware stack
│   │   ├── router.rs                  # Menggabungkan semua modul route
│   │   └── state.rs                   # Definisi AppState (db, redis, jwt, config)
│   │
│   ├── configs/                       # Konfigurasi aplikasi
│   │   ├── app_config.rs              # Struct config + AppConfig::load() dari env
│   │   ├── database.rs                # Membuat SQLx PgPool
│   │   ├── jwt.rs                     # JwtKeys helper
│   │   └── redis.rs                   # Redis ConnectionManager
│   │
│   ├── common/                        # Shared code lintas modul
│   │   ├── errors/mod.rs              # AppError enum + AppResult<T> type alias
│   │   ├── extractors/
│   │   │   ├── validated_json.rs      # Extractor: deserialize + validate sekaligus
│   │   │   └── auth_user.rs           # Extractor: parse & verify JWT Bearer token
│   │   ├── middlewares/
│   │   │   ├── cors.rs                # CorsLayer yang dapat dikonfigurasi
│   │   │   └── request_id.rs          # Inject x-request-id UUID per request
│   │   ├── traits/
│   │   │   ├── repository.rs          # Generic Repository<T> trait
│   │   │   └── service.rs             # Marker trait untuk Service
│   │   └── utils/
│   │       ├── hash.rs                # hash_password / verify_password (Argon2)
│   │       ├── jwt.rs                 # generate_access_token / decode_token
│   │       └── pagination.rs          # PaginationQuery + PaginatedResponse<T>
│   │
│   ├── modules/                       # Fitur-fitur aplikasi (domain-driven)
│   │   ├── auth/
│   │   │   ├── domain/
│   │   │   │   ├── model/             # AuthToken, TokenPair structs
│   │   │   │   └── repository/        # AuthRepository trait
│   │   │   ├── application/
│   │   │   │   ├── auth_service.rs    # login, refresh, logout logic
│   │   │   │   └── dto.rs             # LoginRequest, RefreshRequest, AuthResponse
│   │   │   └── presentation/
│   │   │       ├── handlers.rs        # login(), refresh_token(), logout() handlers
│   │   │       └── routes.rs          # /auth/login, /auth/refresh, /auth/logout
│   │   │
│   │   └── users/
│   │       ├── domain/
│   │       │   ├── model/             # User, UserProfile structs
│   │       │   └── repository/        # UserRepository trait
│   │       ├── application/
│   │       │   ├── user_service.rs    # create, get, list, update, delete
│   │       │   └── dto.rs             # CreateUserRequest, UpdateUserRequest
│   │       └── presentation/
│   │           ├── handlers.rs        # CRUD handlers
│   │           └── routes.rs          # /users routes
│   │
│   ├── background/                    # Background tasks & scheduled jobs
│   │   ├── scheduler.rs               # tokio-cron-scheduler setup
│   │   └── jobs/
│   │       ├── email_job.rs           # Email sending job (lettre)
│   │       └── audit_job.rs           # Async audit log job
│   │
│   ├── observability/                 # Logging, tracing, metrics
│   │   ├── tracing.rs                 # init_tracing() — JSON/pretty berdasar ENV
│   │   └── metrics.rs                 # Prometheus metrics layer
│   │
│   ├── migrations/                    # Folder SQL migrations (sqlx-cli)
│   │   └── README.md                  # Panduan sqlx migrate
│   │
│   ├── docs/                          # Dokumentasi API (OpenAPI/Swagger)
│   │   └── README.md                  # Panduan integrasi utoipa
│   │
│   └── tests/                         # Unit & integration tests
│       └── mod.rs                     # Test modules
│
├── .env.example                       # Template environment variables
├── Cargo.toml                         # Dependencies & build config
├── Dockerfile                         # Multi-stage production Docker build
├── docker-compose.yml                 # App + PostgreSQL + Redis stack
├── Makefile                           # Shortcut commands
└── README.md
```

---

## ⚡ Quick Start

### Prasyarat

- [Rust](https://rustup.rs/) >= 1.77
- [Docker](https://docs.docker.com/get-docker/) & Docker Compose (untuk PostgreSQL + Redis)
- `sqlx-cli` (untuk migrasi):

```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

### 1. Clone & Setup

```bash
git clone https://github.com/your-org/rust-axum-starter-kit.git
cd rust-axum-starter-kit
cp .env.example .env
```

### 2. Jalankan Database & Redis

```bash
make docker-up
# atau manual:
docker compose up -d postgres redis
```

### 3. Jalankan Migrasi

```bash
make migrate-run
```

### 4. Jalankan Server

```bash
make dev       # hot-reload via cargo-watch
# atau:
cargo run
```

Server berjalan di `http://localhost:8080`

---

## 🔐 Environment Variables

Salin `.env.example` ke `.env` dan sesuaikan nilainya:

```bash
cp .env.example .env
```

| Variable | Default | Keterangan |
|---|---|---|
| `ENV` | `development` | `development` atau `production` (mempengaruhi format log) |
| `SERVER__PORT` | `8080` | Port server berjalan |
| `DATABASE__URL` | — | PostgreSQL connection string |
| `DATABASE__MAX_CONNECTIONS` | `10` | Ukuran connection pool |
| `JWT__SECRET` | — | **Wajib diubah** — min 64 karakter acak di production |
| `JWT__ACCESS_TOKEN_EXPIRY_SECS` | `900` | 15 menit |
| `JWT__REFRESH_TOKEN_EXPIRY_SECS` | `2592000` | 30 hari |
| `REDIS__URL` | `redis://localhost:6379` | Redis connection string |
| `RUST_LOG` | `info,sqlx=warn` | Log level per-crate |

> **Catatan:** Konfigurasi menggunakan separator `__` untuk nested values (e.g., `SERVER__PORT` → `config.server.port`)

---

## 🚀 Menjalankan Project

### Mode Development (dengan hot-reload)

```bash
# Install cargo-watch jika belum ada
cargo install cargo-watch

make dev
# atau:
cargo watch -x run
```

### Mode Normal

```bash
cargo run
```

### Dengan Docker Compose (Full Stack)

```bash
make docker-up       # start semua service
make docker-logs     # lihat log aplikasi
make docker-down     # stop semua service
```

---

## 🗄 Migrasi Database

```bash
# Buat migration baru
make migrate-add
# atau: sqlx migrate add create_users_table

# Jalankan semua migration pending
make migrate-run

# Rollback migration terakhir
make migrate-revert

# Lihat status migration
sqlx migrate info
```

File migration disimpan di `src/migrations/` dengan format:

```
20240101120000_create_users_table.sql
```

---

## 📦 Build & Deploy

### Build Binary

```bash
make build
# Hasil: target/release/rust-axum-starter-kit
```

### Build Docker Image

```bash
docker build -t myapp:latest .
```

### Deploy ke Production

```bash
# 1. Set environment variables di server / Kubernetes secret
# 2. Jalankan migrasi
DATABASE_URL=postgres://... sqlx migrate run

# 3. Jalankan container
docker run -d \
  --name myapp \
  -p 8080:8080 \
  -e ENV=production \
  -e DATABASE__URL=postgres://... \
  -e JWT__SECRET=your_very_long_secret \
  myapp:latest
```

> **Tips Production:**
> - Gunakan `ENV=production` agar log otomatis dalam format JSON
> - Generate JWT secret dengan: `openssl rand -base64 64`
> - Jalankan sebagai non-root user (sudah dikonfigurasi di Dockerfile)

---

## 🧩 Menambah Modul Baru

Ikuti langkah ini saat menambahkan fitur baru, misalnya modul `products`:

```bash
# 1. Buat struktur folder
mkdir -p src/modules/products/{domain/{model,repository},application,presentation}

# 2. Buat file mod.rs di setiap layer (ikuti pola auth/ atau users/)

# 3. Daftarkan di src/modules/mod.rs
#    pub mod products;

# 4. Daftarkan route di src/bootstrap/router.rs
#    .nest("/api/v1/products", product_routes())
```

**Checklist modul baru:**

- [ ] `domain/model/mod.rs` — struct domain + `sqlx::FromRow`
- [ ] `domain/repository/mod.rs` — trait repository
- [ ] `application/dto.rs` — request/response DTO dengan validasi
- [ ] `application/*_service.rs` — business logic
- [ ] `presentation/handlers.rs` — Axum handlers
- [ ] `presentation/routes.rs` — route definitions
- [ ] Migration SQL di `src/migrations/`
- [ ] Unit test di `src/tests/`

---

## 🧪 Testing

```bash
# Jalankan semua test
make test

# Test dengan output verbose
cargo test -- --nocapture

# Test satu modul spesifik
cargo test user_tests

# Test dengan coverage (install tarpaulin dulu)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

---

## 🌐 API Endpoints

Base URL: `http://localhost:8080/api/v1`

### Auth

| Method | Path | Deskripsi | Auth |
|---|---|---|---|
| `POST` | `/auth/login` | Login, mendapatkan access + refresh token | ❌ |
| `POST` | `/auth/refresh` | Refresh access token | ❌ |
| `POST` | `/auth/logout` | Revoke refresh token | ✅ |

**Contoh Login:**

```bash
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "password123"}'
```

### Users

| Method | Path | Deskripsi | Auth |
|---|---|---|---|
| `POST` | `/users` | Buat user baru | ❌ |
| `GET` | `/users?page=1&per_page=20` | List semua user (paginated) | ✅ |
| `GET` | `/users/:id` | Ambil user by ID | ✅ |
| `PATCH` | `/users/:id` | Update user | ✅ |
| `DELETE` | `/users/:id` | Hapus user | ✅ Admin |

**Contoh request dengan auth:**

```bash
curl http://localhost:8080/api/v1/users \
  -H "Authorization: Bearer <access_token>"
```

### Format Response

**Success:**
```json
{
  "data": { ... },
  "meta": { "page": 1, "per_page": 20, "total": 100, "total_pages": 5 }
}
```

**Error:**
```json
{
  "success": false,
  "error": {
    "code": 422,
    "message": "email: Invalid email address"
  }
}
```

---

## 📄 Lisensi

MIT © 2024 — bebas digunakan dan dimodifikasi untuk project komersial maupun open source.
