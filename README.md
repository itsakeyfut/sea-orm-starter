# SeaORM スターターキット

SeaORM に関する情報が限られているため、最小限で拡張可能なセットアップテンプレートを作成しました。
問題が発生した場合は、お気軽に Issues セクションで報告してください。

## SeaORM とは？

SeaORM は Rust 製の ORM（オブジェクト・リレーショナル・マッパー）であり、CLI ツールとクレートを通じてマイグレーションやテーブル操作が可能です。
[SeaORM Official](https://www.sea-ql.org/SeaORM/)

## SeaORM CLI とは？

SeaORM CLI を使用すると、マイグレーションファイルの生成、マイグレーションの実行・ロールバック、エンティティの作成などをシンプルなコマンドで行うことができます。

### インストール方法

```sh
cargo install sea-orm-cli
```

### 環境

- Windows
- WSL2 (Ubuntu)

### 手順

すでにマイグレーションファイルを作成していますので、まずは動かしてみましょう。

1. SeaORM CLI をインストールしてください。

```sh
cargo install sea-orm-cli
```

2. コンテナを作成

```sh
make start
```

3. マイグレーション

※もし失敗する場合は、sea-orm と sqlx のバージョン不一致で失敗している可能性が高いです。SeaORM の常習的なエラーなので、sea-orm のバージョンアップをお願いします。

```sh
make migrate-up
```

4. HTTP テスト

client.http にエンドポイントを 2 つ用意していますので、「Send Request」してテストしてみてください。

例：ユーザー取得

```sh
curl -X GET http://localhost:8000/users/1
```

例：ユーザー追加

```sh
curl -X POST http://localhost:8000/users \
  -H "Content-Type: application/json" \
  -d '{"username": "test_user1", "cognito_id": "1"}'
```

## 主要なコマンド一覧

### マイグレーションの初期化

migration/がまだない場合は実行する必要があります。

```sh
sea-orm-cli migrate init
```

Makefile を使って短縮可能です：

```sh
make migrate-init
```

### マイグレーションファイルの作成

```sh
sea-orm-cli migrate generate create_<table_name>_table
```

例：「user」テーブルを作成する場合：

```sh
sea-orm-cli migrate generate create_user_table
```

このコマンドも Makefile で短縮可能です：

```sh
make gen table_name=user
```

例：

```sh
$ make gen table_name=project
sea-orm-cli migrate generate create_project_table
Generating new migration...
Creating migration file `./migration/src/m20241231_220934_create_project_table.rs`
Adding migration `m20241231_220934_create_project_table` to `./migration/src/lib.rs`
```

### マイグレーションの実行

```sh
docker exec -it backend sea-orm-cli migrate up -u postgres://postgres:postgres@db/sea-orm-starter
```

Makefile を使って短縮可能です：

```sh
make migrate-up
```

### マイグレーションのロールバック

```sh
docker exec -it backend sea-orm-cli migrate down -u postgres://postgres:postgres@db/sea-orm-starter
```

Makefile を使って短縮可能です：

```sh
make migrate-down
```

### エンティティの生成

このコマンドは **コンテナ内で実行** し、ホストマシン上にエンティティファイルを生成します。

```sh
sea-orm-cli generate entity -u postgres://postgres:postgres@localhost:5432/sea-orm-starter -o src/entities
```

Makefile でも短縮できます：

```sh
make gen-entity
```
