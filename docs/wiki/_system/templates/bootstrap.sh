#!/usr/bin/env bash
# Install the RepoBrain engine into a host git repository.
# From a RepoBrain clone:  ./bootstrap.sh /path/to/host-repo
# From a host with curl:   curl -fsSL https://raw.githubusercontent.com/devintucker24/RepoBrain/main/bootstrap.sh | bash
set -euo pipefail

UPSTREAM="${REPOBRAIN_URL:-https://github.com/devintucker24/RepoBrain.git}"
DEST="$(cd "${1:-.}" && pwd)"
HERE="$(cd "$(dirname "$0")" 2>/dev/null && pwd || true)"
SRC="${REPOBRAIN_SRC:-}"

if [[ -z "$SRC" && -n "$HERE" && -x "$HERE/repobrain" && -d "$HERE/docs/wiki/_system" ]]; then
  SRC="$HERE"
fi

if [[ -z "$SRC" ]]; then
  SRC="$(mktemp -d "${TMPDIR:-/tmp}/repobrain.XXXXXX")/RepoBrain"
  git clone --depth 1 "$UPSTREAM" "$SRC"
fi

python3 -m pip install --user -q pyyaml
chmod +x "$SRC/repobrain"
"$SRC/repobrain" bootstrap "$DEST"
echo "RepoBrain installed into $DEST"
echo "Next: edit docs/wiki/_system/config/HOST.yaml (name + anchor), then ./repobrain doctor"
