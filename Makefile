.PHONY: dev build test lint fmt migrate-run migrate-revert migrate-add docker-up docker-down clean

# ─── Development ──────────────────────────────────────────────────────────────
dev:
	RUST_LOG=debug cargo watch -x run

build:
	cargo build --release

test:
	cargo test -- --test-threads=4

lint:
	cargo clippy -- -D warnings

fmt:
	cargo fmt --all

# ─── Database Migrations ──────────────────────────────────────────────────────
migrate-run:
	sqlx migrate run

migrate-revert:
	sqlx migrate revert

migrate-add:
	@read -p "Migration name: " name; sqlx migrate add $$name

# ─── Docker ───────────────────────────────────────────────────────────────────
docker-up:
	docker compose up -d

docker-down:
	docker compose down

docker-logs:
	docker compose logs -f app

docker-build:
	docker compose build --no-cache

# ─── Cleanup ──────────────────────────────────────────────────────────────────
clean:
	cargo clean
