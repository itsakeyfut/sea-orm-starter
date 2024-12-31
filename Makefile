include .env

########## DOCKER COMMANDS ##########
# コンテナ起動
up:
	docker compose up -d

#  コンテナビルド
build:
	docker compose build --no-cache

# コンテナ停止
down:
	docker compose down -v

# コンテナ・ビルドイメージ削除
clean:
	docker system prune -a --volumes -f && docker builder prune -a -f

# コンテナクリーンアップ&ビルド&起動
again: down clean build up

# コンテナビルド&起動
start: build up

# コンテナ停止&クリーンアップ
end: down clean

# DBテーブル一覧表示
list:
	export $(shell cat .env | sed 's/#.*//;/^$$/d' | xargs) && docker exec -it db psql -U $(POSTGRES_USER) -d $(POSTGRES_DB) -c "\dt"

########## SEA_ORM_CLI COMMANDS ##########
# エンティティ生成
gen-entity:
	export $(shell cat .env | sed 's/#.*//;/^$$/d' | xargs) && sea-orm-cli generate entity -u $(DB_CONTAINER_URL) -o src/entities

# backendコンテナにアクセス
bash:
	docker compose exec backend bash && sea-orm-cli migrate up && ./backend

# マイグレーション初期化
migrate-init:
	sea-orm-cli migrate init

# マイグレーション実行
migrate-up:
	export $(shell cat .env | sed 's/#.*//;/^$$/d' | xargs) && docker exec -it backend sea-orm-cli migrate up -u $(DATABASE_URL)

# マイグレーションロールバック
migrate-down:
	export $(shell cat .env | sed 's/#.*//;/^$$/d' | xargs) && docker exec -it backend sea-orm-cli migrate down -u $(DATABASE_URL)

# マイグレーションファイル生成
MIGRATE_CMD = sea-orm-cli migrate generate create_$(table_name)_table
gen:
	$(MIGRATE_CMD)