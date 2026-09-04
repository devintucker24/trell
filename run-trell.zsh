#!/usr/bin/env bash
set -euo pipefail

if [ "${1-}" = "" ]; then
  echo "Usage: ./run-trell.zsh <file.trell>" >&2
  exit 1
fi

cargo run --quiet -- "$1"
