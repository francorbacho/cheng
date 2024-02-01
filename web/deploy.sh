#!/bin/bash

set -e

verbose() {
	echo >&2 "+ $@"
	"$@"
}

help_and_exit() {
	echo "usage: $0 [--copy] [--low-nbits]"
    exit 1
}

require_command() {
    cmd=$1
    if ! command -v $1 >/dev/null; then
        echo >&2 "$base_name: error: '$1' command is missing"
        exit 1
    fi
}

base_name=$0

verbose require_command git
verbose require_command wasm-pack

for flag in $*
do
    case "$flag" in
        --copy) copy=1 ;;
        --low-nbits) cargoflags="$cargoflags --features low_nbits" ;;
        *) help_and_exit ;;
    esac
done

workspace_root=$(git rev-parse --show-toplevel)

verbose wasm-pack build --no-typescript --release --target web $workspace_root/chess-wasm $cargoflags

test -e $workspace_root/web/pkg && rm -r $workspace_root/web/pkg

if [[ "$copy" -eq 1 ]]; then
    verbose cp -r $workspace_root/chess-wasm/pkg $workspace_root/web/pkg
    verbose rm $workspace_root/web/pkg/.gitignore
else
    verbose ln -sf $workspace_root/chess-wasm/pkg $workspace_root/web/pkg
fi
