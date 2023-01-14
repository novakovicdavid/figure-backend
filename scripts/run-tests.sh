#!/bin/bash

podman build -f ./db_dockerfile -t figure-backend-db-test > /dev/null
container_id=$(podman run -p 5432:5432 -d -e POSTGRES_PASSWORD=mysecretpassword figure-backend-db-test)

echo "Waiting for postgres to finish initialization..."
until pg_isready -h localhost -p 5432 -U postgres > /dev/null
do
  sleep 1;
done

redis_id=$(podman run -p 6379:6379 -d redis)

cargo test test_routes_unauthenticated -- --nocapture
# cargo test test_routes_authenticated -- --nocapture
podman container stop $container_id > /dev/null
podman container stop $redis_id > /dev/null
podman rm --volumes $container_id > /dev/null
podman rm --volumes $redis_id > /dev/null
