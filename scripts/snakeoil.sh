#!/usr/bin/env bash

dir=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

cd $dir
cd ..

openssl req -x509 -newkey rsa:4096 -sha256 -days 3650 -nodes -text \
  -keyout "$(pwd)/crates/brace-data-store-postgres/fixtures/server.key" \
  -out "$(pwd)/crates/brace-data-store-postgres/fixtures/server.crt" \
  -subj '/CN=localhost' \
  -extensions san \
  -config <(echo '[req]'; echo 'distinguished_name=req';
            echo '[san]'; echo 'subjectAltName=DNS:localhost,IP:127.0.0.1')
