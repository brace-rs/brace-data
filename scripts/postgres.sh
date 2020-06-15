#!/usr/bin/env bash

dir=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

cd $dir
cd ..

docker run --rm -it \
  -p 5432:5432 \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=postgres \
  -v $PWD/scripts/postgres-enable-tls.sh:/docker-entrypoint-initdb.d/enable-tls.sh \
  -v $PWD/crates/brace-data-store-postgres/fixtures/:/config/ \
  postgres:11 \
  postgres -l
