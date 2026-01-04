#! /bin/bash

docker run -d \
  --name my_postgres \
  -e POSTGRES_USER=hejj \
  -e POSTGRES_PASSWORD=pass12345 \
  -e POSTGRES_DB=postgres \
  -p 5432:5432 \
  postgres:latest
