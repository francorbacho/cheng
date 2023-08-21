#!/bin/bash

require_command() {
    cmd=$1
    if ! command -v $1 >/dev/null; then
        echo >&2 "error: '$1' command is missing"
        exit 1
    fi
}

set -e

require_command git
require_command wasm-pack

workspace_root=$(git rev-parse --show-toplevel)

wasm-pack build --no-typescript --target web $workspace_root/chess-wasm
ln -sf $workspace_root/chess-wasm/pkg $workspace_root/web/pkg
