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

# Functions

function prepare_workspace {
  docker stop tp2_alglobo_manual_1 > /dev/null 2>&1 || true &&
  docker rm tp2_alglobo_manual_1 > /dev/null 2>&1 || true > /dev/null 2>&1
}

function clean_workspace {
  docker rm tp2_alglobo_manual_1 > /dev/null 2>&1 || true > /dev/null 2>&1
}

function load_env {
  set -a # automatically export all variables
  source .env
  set +a
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
echo -e "${FC}========================================${NC}"
echo -e "${FC}=${NC}             ${CYANB}Modo Manual              ${FC}=${NC}"
echo -e "${FC}========================================${NC}\n"

# Prepare workspace
printf "> Prepare workspace..."
prepare_workspace
echo -e " ${CHECK}\n"

# Build
printf "> Building..."
docker build -t tp2_alglobo_manual -f "$(pwd)/services/alglobo/manual.Dockerfile" ./services/alglobo > /dev/null 2>&1
echo -e " ${CHECK}\n"

# Run attached
echo -e "${CYANB}> Running...${NC}"
load_env
docker run -it \
-e DATA_PORT=${ALGLOBO_DATA_PORT} \
-e SVC_PORT=${SERVICE_PORT} \
--env-file "./config/alglobo.env" \
--name tp2_alglobo_manual_1 \
--network tp2_external \
tp2_alglobo_manual

printf "\n> Cleaning workspace..."
clean_workspace
echo -e " ${CHECK}\n"

echo -e "${CYANB}Bye bye!${NC}"
