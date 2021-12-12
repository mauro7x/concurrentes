#!/bin/bash

REPLICAS="${1:-0}"

echo "Running with $REPLICAS replicas"
docker-compose up --build --scale alglobo=$REPLICAS

echo "Removing dangling images"
docker image prune -f