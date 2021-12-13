#!/bin/bash

REPLICAS="${1:-1}"
echo "Setting AlGlobo instances to $REPLICAS replicas"
docker-compose up --scale alglobo=$REPLICAS -d
