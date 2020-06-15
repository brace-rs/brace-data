#!/usr/bin/env bash

# https://github.com/docker-library/postgres/pull/152#issuecomment-230998344

set -e

# Uncomment this line to disable trust authentication for local connections.
# sed -i 's/host/hostssl/g' "$PGDATA"/pg_hba.conf

cp /config/server.{crt,key} "$PGDATA"

chown postgres:postgres "$PGDATA"/server.{crt,key}
chmod 0600 "$PGDATA"/server.key
