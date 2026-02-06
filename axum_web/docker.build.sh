#! /bin/bash

set -e

docker build -t axum_app:latest --file ./Dockerfile .