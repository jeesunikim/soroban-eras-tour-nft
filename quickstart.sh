#!/bin/bash

set -e

case "$1" in
standalone)
    echo "Using standalone network"
    QUICKSTART_REF="stellar/quickstart:testing"
    ARGS="--local"
    ;;
futurenet)
    echo "Using Futurenet network"
    QUICKSTART_REF="stellar/quickstart:soroban-dev"
    ARGS="--futurenet"
    ;;
testnet)
    echo "Using Testnet network"
    QUICKSTART_REF="stellar/quickstart:testing"
    ARGS="--testnet"
    ;;
*)
    echo "Usage: $0 standalone|futurenet|testnet"
    exit 1
    ;;
esac

# this is set to the quickstart `soroban-dev` image annointed as the release 
# for a given Soroban Release, it is captured on Soroban Releases - https://soroban.stellar.org/docs/reference/releases 
QUICKSTART_SOROBAN_DOCKER_SHA=stellar/quickstart:soroban-dev

shift

# Run the soroban-preview container
# Remember to do:
# make build-docker

echo "Creating docker soroban network"
(docker network inspect soroban-network -f '{{.Id}}' 2>/dev/null) \
  || docker network create soroban-network

echo "Searching for a previous soroban-preview docker container"
containerID=$(docker ps --filter="name=soroban-preview" --all --quiet)
if [[ ${containerID} ]]; then
    echo "Start removing soroban-preview container."
    docker rm --force soroban-preview
    echo "Finished removing soroban-preview container."
else
    echo "No previous soroban-preview container was found"
fi

# currentDir=$(pwd)
# docker run -dti \
#   --volume ${currentDir}:/workspace \
#   --name soroban-preview \
#   -p 8001:8000 \
#   --ipc=host \
#   --network soroban-network \
#   soroban-preview:10

# Run the stellar quickstart image
docker run --rm -ti \
  -p 8000:8000 \
  --name stellar \
  "$QUICKSTART_REF" \
  $ARGS \
  --enable-soroban-rpc \
  "$@" # Pass through args from the CLI