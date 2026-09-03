#!/usr/bin/env zsh
set -euo pipefail

cd "${0:A:h}"
source ./env.sh

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <source-file.trell>" >&2
  exit 1
fi

source_file="$1"

cargo run -- "$source_file"
./link-trell.sh

set +e
./trell-program
status=$?
set -e

echo "Result: $status"
exit 0
