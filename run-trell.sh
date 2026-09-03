#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"
# shellcheck source=env.sh
source ./env.sh

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <source-file.trell>" >&2
  exit 1
fi

source_file="$1"

cargo run -- "$source_file"

if command -v clang >/dev/null 2>&1; then
  clang out.ll -o trell-program
elif command -v llc >/dev/null 2>&1; then
  llc -filetype=obj out.ll -o trell-program.o
  cc trell-program.o -o trell-program
else
  echo "Need clang or llc to produce a native binary from out.ll" >&2
  exit 1
fi

set +e
./trell-program
status=$?
set -e

echo "Result: $status"
exit 0
