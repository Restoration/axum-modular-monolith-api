# rust-api

Axum + Tokio + PostgreSQL によるモジュラーモノリス API。

## 構成

```
crates/
├── server/          # エントリーポイント、ルーター統合、マイグレーション
├── shared/          # 共通型 (AppError, AppState, PgPool)
└── modules/
    ├── health/      # GET /health
    └── user/        # User CRUD (handler → usecase → repository)
migrations/          # SQLマイグレーション
```

## 起動

### Docker Compose (推奨)

```sh
docker compose up --build
```

### ローカル開発

```sh
# PostgreSQLのみ起動
docker compose up db -d

# アプリ起動
DATABASE_URL=postgres://app:app@localhost:5432/app cargo run -p server
```

## エンドポイント

```sh
# ヘルスチェック
curl http://localhost:8080/health

# ユーザー一覧
curl http://localhost:8080/users

# ユーザー取得
curl http://localhost:8080/users/1

# ユーザー作成
curl -X POST http://localhost:8080/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Charlie"}'
```

## マイグレーション

`migrations/` にSQLファイルを追加すると、サーバー起動時に自動適用されます。

## モジュール追加

1. `crates/modules/<name>/` に crate を作成
2. `pub fn router() -> Router<AppState>` を公開
3. `Cargo.toml` (workspace members) に追加
4. `crates/server/src/main.rs` で `.merge()` で統合
