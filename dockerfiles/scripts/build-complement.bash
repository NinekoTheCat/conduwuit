#!/bin/bash
# Builds base file
cargo vendor ./.docker-home >  ./.docker-home/.cargo-config.toml
pushd ./dockerfiles
docker buildx build -f ./Dockerfile.base -t tuwunel-base:latest ../ --progress=plain 2>&1 | tee build.log 
# Builds test file
docker buildx build -f ./Dockerfile.test-main -t tuwunel-test:latest ../
# Builds complement
popd