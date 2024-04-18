#!/usr/bin/env zsh

echo "Running 3 servers at 8000, 8001 and 8002"
cargo run --bin distributed-server 8000 &
cargo run --bin distributed-server 8001 &
cargo run --bin distributed-server 8002 &