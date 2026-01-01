#! /bin/bash

set +e

# 创建开发调试用的PostgreSQL数据库
docker run \
  --detach \
 --name pg_dev \
 --rm \
 --publish 5432:5432 \
 --env POSTGRES_USER=hejj \
 --env POSTGRES_PASSWORD=pass12345 \
 --env POSTGRES_DB=postgres \
 postgres:latest