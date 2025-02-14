PROJECT := "cervidae"
REPO := "https://github.com" / "JimHLin" / PROJECT
ROOT_DIR := justfile_directory()
OUTPUT_DIR := ROOT_DIR / "target"

migrate:
    cargo run --bin migrate

export COMPOSE_PROJECT_NAME := "cervidae"

local:
    docker compose up -d

local-min:
    docker compose up -d deer_postgres deer_adminer

local-down:
    docker compose down -v
