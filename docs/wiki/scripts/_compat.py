"""Temporary old-path dispatcher for the RepoBrain engine migration."""

from __future__ import annotations

import runpy
import sys
from pathlib import Path


def dispatch(filename: str) -> None:
    canonical = (
        Path(__file__).resolve().parents[1]
        / "_system"
        / "scripts"
        / filename
    )
    print(
        f"DEPRECATED: docs/wiki/scripts/{filename} moved to "
        f"docs/wiki/_system/scripts/{filename}",
        file=sys.stderr,
    )
    sys.path.insert(0, str(canonical.parent))
    runpy.run_path(str(canonical), run_name="__main__")
