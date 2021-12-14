#!/bin/bash
docker system prune -f
docker volume prune -f
docker image rm tp2_alglobo tp2_generic_service tp2_directory tp2_alglobo_manual