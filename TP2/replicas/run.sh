#!/bin/bash

REPLICAS="${1:-0}"

# build
sh -c 'cd alglobo && cargo build --release' &&
sh -c 'cd directory && cargo build --release' &&

# run
echo "Running with $REPLICAS replicas" &&
docker-compose up --scale alglobo=$REPLICAS
