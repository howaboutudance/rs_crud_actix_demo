#!/usr/bin/env bash
set +x

if podman network exists microblogpub-network; then
    podman pod kill prometheus
    podman pod rm prometheus
    podman network rm rs_crud_actix-network
else
    echo "microblogpub-network does not exists... exiting..."
fi