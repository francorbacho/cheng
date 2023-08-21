#!/bin/bash
#
# Dependencies:
# - cargo
# - wasm-gc
# - wasm-bindgen

set -e

workspace_root=$(git rev-parse --show-toplevel)
cd $workspace_root

mkdir -p web/wasm web/pkg

cargo build --release --package chess-wasm --target wasm32-unknown-unknown
wasm-gc target/wasm32-unknown-unknown/release/chess_wasm.wasm -o web/wasm/cheng.wasm
wasm-bindgen --target web --out-dir web/pkg web/wasm/cheng.wasm
