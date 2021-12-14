#!/bin/bash

# Exit when any command fails
set -e

# Constants
CYAN='\033[0;36m'
CYANB='\033[1;36m'
GREY='\033[0;30m'
GREEN='\033[0;32m'
WHITEB='\033[1;37m'
NC='\033[0m' # No Color
CHECK="${GREEN}\xE2\x9C\x94${NC}"
FC=$CYAN

# Vars
REPLICAS="${1:-0}"
PAYMENTS_FILE="${2:-./examples/payments.csv}"
ACCOUNTS_FILE="${3:-./examples/accounts.csv}"

# Functions

function copy_build_dependencies {
  # Shared files
  mkdir -p "./services/alglobo/shared" &&
  mkdir -p "./services/directory/shared" &&
  mkdir -p "./services/generic-service/shared" &&

  # Protocol: alglobo <-> generic-service
  cp ./common/alglobo_generic-service_protocol.rs ./services/alglobo/shared/alglobo_generic-service_protocol.rs &&
  cp ./common/alglobo_generic-service_protocol.rs ./services/generic-service/shared/alglobo_generic-service_protocol.rs

  # Protocol: alglobo <-> directory
  cp ./common/alglobo_directory_protocol.rs ./services/alglobo/shared/alglobo_directory_protocol.rs &&
  cp ./common/alglobo_directory_protocol.rs ./services/directory/shared/alglobo_directory_protocol.rs

}

function copy_runtime_dependencies {
  # Payments source
  mkdir -p "./.tmp" &&
  cp "${PAYMENTS_FILE}" ./.tmp/payments.csv &&
  cp "${ACCOUNTS_FILE}" ./.tmp/accounts.csv
}

# Main script

echo -e "${FC}========================================${NC}"
echo -e "${FC}=${NC}       ${CYANB}AlGlobo: Sistema de Pagos      ${FC}=${NC}"
echo -e "${FC}========================================${NC}"
echo -e "${FC}=${NC} Técnicas de Programación Concurrente ${FC}=${NC}"
echo -e "${FC}=${NC}          Trabajo Práctico 2          ${FC}=${NC}"
echo -e "${FC}========================================${NC}"
echo -e "${FC}=${NC}         ${WHITEB}Grupo Context Switch         ${FC}=${NC}"
echo -e "${FC}=${NC}                                      ${FC}=${NC}"
echo -e "${FC}=${NC}            Mauro Parafati            ${FC}=${NC}"
echo -e "${FC}=${NC}            Santiago Klein            ${FC}=${NC}"
echo -e "${FC}=${NC}            Tomás  Nocetti            ${FC}=${NC}"
echo -e "${FC}========================================${NC}\n"

printf "> Copying build dependencies..."
copy_build_dependencies
echo -e " ${CHECK}\n"

printf "> Building and creating services..."
docker-compose build > /dev/null 2>&1
echo -e " ${CHECK}\n"

printf "> Removing dangling images..."
docker image prune -f > /dev/null 2>&1
echo -e " ${CHECK}\n"

printf "> Copying runtime dependencies..."
copy_runtime_dependencies
echo -e " ${CHECK}\n"

echo -e "${CYANB}> Running with ${REPLICAS} replicas...${NC}"
docker-compose up --scale alglobo=$REPLICAS

echo -e "\n${CYANB}Bye bye!${NC}"
