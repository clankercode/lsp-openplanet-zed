#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
cargo fmt --all -- --check
cargo test --locked -j 2
cargo clippy --locked --all-targets -j 2 -- -D warnings
cargo build --locked --release --target wasm32-wasip2 -j 2
