# SeaORM Starter Kit

Due to the limited information available on SeaORM, I have created this template to provide a minimal yet extendable setup for working with SeaORM.  
If you encounter any issues, please feel free to raise them in the Issues section.

## What is SeaORM?

SeaORM is an ORM (Object-Relational Mapper) in Rust that allows migration and table access through a CLI tool and crate.  
[SeaORM Official](https://www.sea-ql.org/SeaORM/)

## What is SeaORM CLI?

The SeaORM CLI enables operations like generating migration files, running migrations, rolling them back, and creating entities—all through simple commands.

### Installation

```sh
cargo install sea-orm-cli
```

## Key Commands
To get started, follow the instructions below based on the ⭐ indicators. Commands without a star are for reference purposes only.
Initialization and migration file creation must be done on the host machine.

### ⭐ Initialize Migration
```sh
sea-orm-cli migrate init
```

This command is shortened using Makeifle.
```sh
make migrate-init
```

### ⭐⭐ Create Migration File

```sh
sea-orm-cli migrate generate create_<table_name>_table
```

Example: Create the "user" table.
```sh
sea-orm-cli migrate generate create_user_table
```

This command is also shortened using Makefile.
```sh
make gen table_name=user_table_name
```

Example:
```sh
$ make gen table_name=project
sea-orm-cli migrate generate create_project_table
Generating new migration...
Creating migration file `./migration/src/m20241231_220934_create_project_table.rs`
Adding migration `m20241231_220934_create_project_table` to `./migration/src/lib.rs`
```

### ⭐⭐⭐ Run Migration
```sh
docker exec -it backend sea-orm-cli migrate up -u postgres://postgres:postgres@db/sea-orm-starter
```

This command is shortened using Makefile.

```sh
make migrate-up
```

### Rollback Migration
```sh
docker exec -it backend sea-orm-cli migrate down -u postgres://postgres:postgres@db/sea-orm-starter
```

This command is shortened using Makefile.

```sh
make migrate-down
```

### ⭐⭐⭐⭐ Generate Entity
This command is executed within the container and generates entities for the project on the host machine.

```sh
sea-orm-cli generate entity -u postgres://postgres:postgres@localhost:5432/sea-orm-starter -o src/entities
```

This command is also shortened using Makeifle.
```sh
make gen-entity
```
