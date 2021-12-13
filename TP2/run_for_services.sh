#!/bin/bash

# Exit when any command fails
set -e

# Navigate to services
cd services

# Run $1 command for each service
for service_dir in ./*; do
    service="$(basename "$service_dir")"
    echo "Running for ${service}..."
    cd "${service}" &&
    $($1) &&
    cd ..
done
