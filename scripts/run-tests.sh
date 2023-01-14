#!/bin/bash
podman build -f ./db_dockerfile -t figure-backend-db-test
container_id=$(podman run -p 5432:5432 -d -e POSTGRES_PASSWORD=mysecretpassword figure-backend-db-test)

echo "Waiting for pg to finish init"
until pg_isready -h localhost -p 5432 -U postgres > /dev/null
do
  sleep 1;
done

redis_id=$(podman run -p 6379:6379 -d redis)

cargo test
podman container stop $container_id
podman container stop $redis_id
podman rm --volumes $container_id
podman rm --volumes $redis_id
