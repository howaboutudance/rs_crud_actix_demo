#!/bin/bash
set +x

# create network if needed
podman network create rs_crud_actix-network

# create mongo server
podman pod create \
    --network=rs_crud_actix-network \
    -n prometheus \
    -p 27017:27017

podman run -dt --pod mongo \
    -v ./data/mongodb:/data/db:z \
    docker.io/mongo:3