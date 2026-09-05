"""Deprecated import path for the canonical RepoBrain path resolver."""

from __future__ import annotations

import sys
import warnings
from pathlib import Path

canonical = Path(__file__).resolve().parents[1] / "_system" / "scripts"
sys.path.insert(0, str(canonical))
warnings.warn(
    "docs/wiki/scripts/wiki_paths.py moved to "
    "docs/wiki/_system/scripts/repobrain_paths.py",
    DeprecationWarning,
    stacklevel=2,
)

from repobrain_paths import *  # noqa: F403,E402
