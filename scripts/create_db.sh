#!/bin/bash
docker run --name sqlx_demo \
       -p 3306:3306 \
       -e MARIADB_ROOT_USER=admin \
       -e MARIADB_ROOT_PASSWORD=password \
       -e MARIADB_DATABASE=sqlx_demo \
       -v $PWD/init.sql:/docker-entrypoint-initdb.d/init.sql \
       -d mariadb:latest
