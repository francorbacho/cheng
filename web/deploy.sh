#!/bin/bash
#
# Dependencies:
# - cargo
# - wasm-gc
# - jq

workspace_root=$(cargo metadata --format-version 1 | jq -r '.workspace_root')
cd $workspace_root

cargo build --release --package chess-wasm --target wasm32-unknown-unknown
wasm-gc target/wasm32-unknown-unknown/release/chess_wasm.wasm -o web/wasm/cheng.wasm