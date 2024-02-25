#!/usr/bin/env bash
# set -x
set -eo pipefail

# if ! [ -x "$(command -v psql)" ]; then
#     echo >&2 "Error: psql is not installed."
#     exit 1
# fi
# if ! [ -x "$(command -v sqlx)" ]; then
#     echo >&2 "Error: sqlx is not installed."
#     echo >&2 "Use:"
#     echo >&2 "
#         cargo install sqlx-cli --no-default-features --features postgres"
#     echo >&2 "to install it."
#     exit 1
# fi

# Check if a custom user has been set, otherwise default to 'root'
DB_USER=${SURREALDB_USER:=root}
# Check if a custom password has been set, otherwise default to 'root'
DB_PASSWORD="${SURREALDB_PASSWORD:=root}"
# Check if a custom database name has been set, otherwise default to 'todo-rs'
DB_NAME="${POSTGRES_DB:=todo-rs}"
# Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${SURREALDB_PORT:=8000}"

# if a container is already running, print instructions to kill it and exit
RUNNING_CONTAINER=$(docker ps --filter "name=surrealdb*" --format '{{.ID}}')
if [[ -n $RUNNING_CONTAINER ]]; then
    echo >&2 "there is a SurrealDb container already running, kill it with"
    echo >&2 "
        docker kill surrealdb_${RUNNING_CONTAINER}"
    exit 1
fi

# Launch postgres using Docker
docker run \
    --name "surrealdb_$(date '+%s')" \
    --rm \
    --pull always \
    -p ${DB_PORT}:${DB_PORT} \
    surrealdb/surrealdb:latest start \
    --log trace \
    --user ${DB_USER} \
    --pass ${DB_PASSWORD} \
    --auth \
    memory

# Keep pinging Postgres until it's ready to accept commands
# export PGPASSWORD="${DB_PASSWORD}"
# until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
#     >&2 echo "Postgres is still unavailable - sleeping"
#     sleep 1
# done
# >&2 echo "Postgres is up and running on port ${DB_PORT}!"
echo "SurrealDb is up and running on port ${DB_PORT}!"

# export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
# echo export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
#  sqlx database create
#  sqlx migrate run
#      >&2 echo "Postgres has been migrated, ready to go!"
