set export

install_sqlx:
    cargo install sqlx-cli --version="~0.7" --no-default-features --features postgres,rustls

db_start:
    docker compose up -d

db_stop:
    docker compose down -v

db_reboot: db_stop db_start

is_db_running:
    docker compose ps | grep -q 'Up'

migrate:
    sqlx database create
    sqlx migrate run
