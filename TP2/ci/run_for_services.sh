#!/bin/bash

# Navigate to services
SERVICES_DIR=../services
SERVICES="${SERVICES_DIR}/*"

# Run $1 command for each service
cd ${SERVICES_DIR}
for service_dir in ${SERVICES}; do
    service="$(basename "$service_dir")"
    echo "Running for ${service}..."
    cd ${service} &&
    $($1) &&
    cd ..
done
