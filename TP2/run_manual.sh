#!/bin/bash

# Exit when any command fails
set -e

# Build
docker build -t tp2_alglobo_manual -f "$(pwd)/services/alglobo/manual.Dockerfile" ./services/alglobo

# Destroy if its running
docker stop tp2_alglobo_manual_1 || true &&
docker rm tp2_alglobo_manual_1 || true

# Run attached
docker run -it --name tp2_alglobo_manual_1 --network tp2_external tp2_alglobo_manual

# Clean up
docker rm tp2_alglobo_manual_1 || true
