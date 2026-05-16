# CLAUDE.md

## プロジェクト概要

Axum + Tokio + PostgreSQL によるモジュラーモノリス API。

## コマンド

```sh
make up          # Docker Compose でビルド&起動
make down        # Docker Compose 停止
make db          # PostgreSQL のみ起動
make dev         # ローカル開発サーバー起動
make build       # ビルド
make test        # テスト実行
make fmt         # フォーマット
make lint        # clippy
make check       # fmt + clippy + test 一括
make new-module NAME=xxx  # 新規モジュール作成
```

## ディレクトリ構成

```
crates/
├── server/          # エントリーポイント、ルーター統合、マイグレーション
├── shared/          # 共通型 (AppError, AppState, PgPool)
└── modules/
    └── <module>/    # 各ドメインモジュール
migrations/          # SQLマイグレーション (サーバー起動時に自動適用)
```

## モジュラーモノリス規約

### モジュールの構造

各モジュールは `crates/modules/<name>/` に独立した crate として配置する。

```
crates/modules/<name>/
├── Cargo.toml
└── src/
    ├── lib.rs          # pub fn router() -> Router<AppState> を公開
    ├── handler.rs      # HTTPハンドラー (ルーティング定義)
    ├── model.rs        # ドメインモデル、リクエスト/レスポンス型
    ├── repository.rs   # データアクセス層 (trait + 実装)
    └── usecase.rs      # ビジネスロジック層
```

### レイヤー責務

| レイヤー | 責務 | 依存先 |
|----------|------|--------|
| handler | HTTPリクエストの受付・レスポンス変換 | usecase |
| usecase | ビジネスロジック、バリデーション | repository (trait) |
| repository | データアクセスの抽象化と実装 | shared (AppError) |
| model | 構造体定義 (なし) | serde, sqlx |

### 依存方向 (厳守)

```
handler → usecase → repository(trait)
                         ↑
                    PgXxxRepository (実装)
```

- **handler は repository を直接使わない**。必ず usecase を経由する。
- **usecase は repository の trait に依存**し、具象型には依存しない。
- **モジュール間は直接依存しない**。共有が必要なものは `shared` crate に置く。

### 命名規則

| 対象 | 規則 | 例 |
|------|------|----|
| crate名 | `module-<name>` | `module-user` |
| ディレクトリ | `crates/modules/<name>` | `crates/modules/user` |
| Router関数 | `pub fn router() -> Router<AppState>` | - |
| Repository trait | `<Entity>Repository` | `UserRepository` |
| Repository実装 | `Pg<Entity>Repository` | `PgUserRepository` |
| Usecase | `<Entity>Usecase<R: Repository>` | `UserUsecase<R>` |

### モジュール追加手順

1. `make new-module NAME=<name>` でスキャフォールド
2. `crates/modules/<name>/src/lib.rs` にルーター実装
3. `crates/server/Cargo.toml` に依存追加: `module-<name> = { path = "../modules/<name>" }`
4. `crates/server/src/main.rs` で `.merge(module_<name>::router())` で統合
5. 必要に応じて `migrations/` にSQLファイル追加

### テスト規約

- **usecase**: MockRepository を使った単体テスト (DB不要)
- **handler**: `tower::ServiceExt` + `axum::body::Body` によるルーターテスト
- **repository**: 結合テスト (実DB使用、CI環境で実行)
- テストは各ファイル末尾に `#[cfg(test)] mod tests` で記述

### 共通ルール

- エラーは `shared::AppError` を使う (NotFound / BadRequest / Internal)
- DB接続は `AppState.db` (PgPool) を Axum の State で共有
- マイグレーションは `migrations/` に `YYYYMMDDHHMMSS_description.sql` で追加
- 外部クレートの追加は最小限にとどめる
