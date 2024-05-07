set export

## clean the project
clean:
  cargo clean

## Install the SQLx CLI to run the migrations
install_sqlx:
  cargo install sqlx-cli --version="~0.7" --no-default-features --features postgres,rustls

## Start the given service
start service="":
  docker compose up {{service}} -d

## Stop the given service
stop service="":
  docker compose down {{service}} -v

## Restart the service
reboot service="":
  just stop {{service}}
  just start {{service}}

## Restart the database
reboot-db:
  just reboot db

## Restart redis
reboot-redis:
  just reboot redis

## Start dependencies
start-deps: migrate
  just start redis

_migrate:
  sqlx database create
  sqlx migrate run

## Run the database migrations
migrate:
  just start db
  just _migrate

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

## Run the tests quietly
test testset="":
  cargo test {{testset}}

## Run the tests showing stacktraces
testv testset="":
  RUST_BACKTRACE=1 cargo test {{testset}}

## Run the tests with verbose and colored output
testvv testset="":
  RUST_LOG="sqlx=error,info" \
  TEST_LOG=true \
  RUST_BACKTRACE=full \
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
