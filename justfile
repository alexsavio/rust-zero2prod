set export

## Install the SQLx CLI to run the migrations
install_sqlx:
    cargo install sqlx-cli --version="~0.7" --no-default-features --features postgres,rustls

## Start the database
db-start:
    docker compose up -d

## Stop the database
db-stop:
    docker compose down -v

## Restart the database
db-reboot: db-stop db-start

## Check if the database is running
is_db_running:
    docker compose ps | grep -q 'Up'

## Run the database migrations
migrate:
    sqlx database create
    sqlx migrate run

## Start the server
run:
    cargo run

## Run the tests
test:
  RUST_BACKTRACE=1 cargo test

## Run the tests with verbose output
testv:
  RUST_BACKTRACE=1 cargo test -- --nocapture

## Run the linter
lint:
    cargo clippy
