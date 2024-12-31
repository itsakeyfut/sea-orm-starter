# SeaORM スターターキット

<p>SeaORM に関する情報が少ないため、SeaORM のテンプレを用意した。</p>
<p>なるべくミニマムに設計し、拡張できるように工夫してある。</p>
<p>問題点があれば、Issues を挙げてもらえると幸甚です。</p>

## SeaORM とは

<p>Rust で ORM を使用してマイグレーション・テーブルアクセスができる CLI およびクレート</p>
<p>[SeaORM](https://www.sea-ql.org/SeaORM/)</p>

## 主なコマンド

<p>手順として、⭐の数を見て環境構築してもらえると問題ないです。</p>
<p>⭐のないコマンドは紹介に留めておきたいものです。</p>
<p>初期化とマイグレーションファイルの作成のみはホストマシンで行うようにした。</p>

### ⭐ マイグレーションの初期化

```bash
sea-orm-cli migrate init
```

<p>Makefile でコマンドを短縮した</p>
make migrate-init

### ⭐⭐ マイグレーションファイルの作成

```bash
sea-orm-cli migrate generate create_<table_name>_table
```

例. user テーブルを作成

```bash
sea-orm-cli migrate generate create_user_table
```

<p>Makefile でコマンドを短縮した</p>
make gen table_name=user_table_name

e.g.

```bash
$ make gen table_name=project
sea-orm-cli migrate generate create_project_table
Generating new migration...
Creating migration file `./migration\src\m20241231_220934_create_project_table.rs`
Adding migration `m20241231_220934_create_project_table` to `./migration\src\lib.rs`
```

### ⭐⭐⭐ マイグレーション実行

```bash
docker exec -it backend sea-orm-cli migrate up -u postgres://postgres:postgres@db/sea-orm-starter
```

<p>Makefile でコマンドを短縮した</p>
make migrate-up

### マイグレーションロールバック

```bash
docker exec -it backend sea-orm-cli migrate down -u postgres://postgres:postgres@db/sea-orm-starter
```

<p>Makefile でコマンドを短縮した</p>
make migrate-down

### ⭐⭐⭐⭐ エンティティ生成

<p>コンテナ内で実行し、ホストマシンのプロジェクトにエンティティを生成するようにした。</p>

```bash
sea-orm-cli generate entity -u postgres://postgres:postgres@localhost:5432/sea-orm-starter -o src/entities
```

<p>Makefile でコマンドを短縮した</p>
make gen-entity
