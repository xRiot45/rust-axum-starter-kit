# Migrations

SQL migration files managed by `sqlx-cli`.

## Setup

```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
export DATABASE_URL=postgres://user:pass@localhost/mydb
sqlx database create
```

## Commands

| Command | Description |
|---------|-------------|
| `sqlx migrate add <name>` | Create a new migration file |
| `sqlx migrate run` | Apply all pending migrations |
| `sqlx migrate revert` | Revert the last migration |
| `sqlx migrate info` | Show migration status |

## Naming Convention

```
YYYYMMDDHHMMSS_descriptive_name.sql
Example: 20240101120000_create_users_table.sql
```

## Example Migration

```sql
-- 20240101120000_create_users_table.sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name        VARCHAR(100)  NOT NULL,
    email       VARCHAR(255)  NOT NULL UNIQUE,
    password_hash TEXT        NOT NULL,
    is_active   BOOLEAN       NOT NULL DEFAULT TRUE,
    created_at  TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ   NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
```
