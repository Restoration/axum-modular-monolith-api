.PHONY: up down db dev build test fmt lint check clean new-module

# Docker Compose
up:
	docker compose -f infrastructure/docker-compose.yml up --build

down:
	docker compose -f infrastructure/docker-compose.yml down

db:
	docker compose -f infrastructure/docker-compose.yml up db -d

# ローカル開発
dev:
	DATABASE_URL=postgres://app:app@localhost:5432/app cargo run -p server

build:
	cargo build

test:
	cargo test

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

check:
	cargo fmt -- --check
	cargo clippy -- -D warnings
	cargo test

clean:
	cargo clean

# モジュール作成: make new-module NAME=<module_name>
new-module:
	@if [ -z "$(NAME)" ]; then echo "Usage: make new-module NAME=<module_name>"; exit 1; fi
	@echo "Creating module: $(NAME)"
	@mkdir -p crates/modules/$(NAME)/src
	@printf '[package]\nname = "module-%s"\nversion = "0.1.0"\nedition = "2021"\n\n[dependencies]\naxum = "0.8"\nserde = { version = "1", features = ["derive"] }\nshared = { path = "../../shared" }\nsqlx = { version = "0.8", features = ["runtime-tokio", "tls-native-tls", "postgres"] }\n' "$(NAME)" > crates/modules/$(NAME)/Cargo.toml
	@printf 'use axum::{routing::get, Router};\nuse shared::AppState;\n\npub fn router() -> Router<AppState> {\n    Router::new().route("/%s", get(index))\n}\n\nasync fn index() -> &'\''static str {\n    "%s"\n}\n' "$(NAME)" "$(NAME)" > crates/modules/$(NAME)/src/lib.rs
	@if grep -q '"crates/modules/$(NAME)"' Cargo.toml; then \
		echo "Workspace member already exists."; \
	else \
		sed -i '/^]$$/i\    "crates/modules/$(NAME)",' Cargo.toml; \
	fi
	@echo ""
	@echo "Done! Next steps:"
	@echo "  1. Edit crates/modules/$(NAME)/src/lib.rs"
	@echo "  2. Add .merge(module_$(NAME)::router()) to crates/server/src/main.rs"
	@echo "  3. Add module-$(NAME) to crates/server/Cargo.toml dependencies"
