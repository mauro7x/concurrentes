#!/bin/bash

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

# ---

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

echo "Pre-running tasks:"

printf "${GREY}  > Creating services...${NC}"
docker-compose build > /dev/null 2>&1
echo -e " ${CHECK}"

printf "${GREY}  > Removing dangling images...${NC}"
docker image prune -f > /dev/null 2>&1
echo -e " ${CHECK}"

echo -e "\n${CYANB}Running with ${REPLICAS} replicas...${NC}"
docker-compose up --scale alglobo=$REPLICAS

echo -e "\n${CYANB}Bye bye!${NC}"
