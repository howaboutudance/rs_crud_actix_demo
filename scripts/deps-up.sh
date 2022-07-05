#!/bin/bash
set +x

# create network if needed
podman network create rs_crud_actix-network

# create mongo server
podman pod create \
    --network=rs_crud_actix-network \
    -n prometheus \
    -p 9090:9090

podman run -dt --pod prometheus \
    -v ./scripts/data/prometheus:/opt/bitnami/prometheus/data:z \
    docker.io/bitnami/prometheus