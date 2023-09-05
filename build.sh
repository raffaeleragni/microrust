#!/usr/bin/env bash

COMPONENTS="api event_bus"

if [ -z "`which rustc`" ]; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi

docker compose down
docker compose up -d

while [ -z "`docker compose logs kafka-init | grep 'kafka topics created'`" ]; do
  echo -n "."
  sleep 1
done

while [ -z "`docker compose logs mysql | grep '/usr/sbin/mysqld: ready for connections'`" ]; do
  echo -n "."
  sleep 1
done

cargo clippy
cargo build
cargo test
cargo build --release

docker compose down
