set export

## Install the SQLx CLI to run the migrations
install_sqlx:
    cargo install sqlx-cli --version="~0.7" --no-default-features --features postgres,rustls

## Start the database
db-start:
    docker compose up db -d

## Stop the database
db-stop:
    docker compose down db -v

## Restart the database
db-reboot: db-stop db-start

## Check if the database is running
is_db_running:
    docker compose ps | grep -q 'Up'

## Run the database migrations
migrate: db-start
    sqlx database create
    sqlx migrate run

## Build and run the service with docker-compose
docker-run:
  docker compose up --build -d

## Stop the service
docker-stop:
  docker compose down -v

## Restart the service
docker-reboot: docker-stop docker-run

## Start the server
run:
    cargo run

## Run the tests
test testset="":
  RUST_BACKTRACE=1 cargo test {{testset}}

## Run the tests with verbose and colored output
testv testset="":
  RUST_LOG="sqlx=error,info" \
  TEST_LOG=true \
  RUST_BACKTRACE=1 \
  cargo test {{testset}} -- --nocapture | bunyan

## Format the code
format:
    cargo fmt

## Run the linter
lint:
    cargo check
    cargo clippy

## Authenticate against DigitalOcean
deploy-auth:
  doctl auth init

## Deploy the service to DigitalOcean
deploy-create:
  doctl apps create --spec spec.yaml

## Update the service on DigitalOcean
deploy-update appid:
  doctl apps update {{appid}} --spec spec.yaml

## List the apps on DigitalOcean
do-list-apps:
  doctl apps list --format ID

## Auth and Deploy the service to DigitalOcean
deploy: deploy-auth deploy-create

## Run the migrations on DigitalOcean
migrate-do dbconnection:
  DATABASE_URL={{dbconnection}} sqlx migrate run

## Update sqlx offline mode
sqlx-prepare:
  cargo sqlx prepare --workspace

## MacOS - fix ulimit
# https://github.com/WeareJH/config-gen/issues/44
fix-ulimit:
  ulimit -n 2048
