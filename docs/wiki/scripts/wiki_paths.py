"""Shared paths and page-scan filters for wiki-brain scripts."""

from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
WIKI = ROOT / "docs" / "wiki"
META = WIKI / "_meta"
USAGE_DIR = META / "usage"
EVENTS_PATH = USAGE_DIR / "events.jsonl"
DASHBOARD_PATH = META / "usage-dashboard.md"
HOST_PATH = WIKI / "HOST.yaml"


def load_host() -> dict:
    """Project overlay. Missing file → empty dict (pack defaults)."""
    if not HOST_PATH.exists():
        return {}
    try:
        import yaml
        return yaml.safe_load(HOST_PATH.read_text(encoding="utf-8")) or {}
    except Exception:  # noqa: BLE001
        return {}

# Not wiki pages: skill playbooks, python, local telemetry files
SKIP_PREFIXES = (
    "skills/",
    "scripts/",
    "pack/",
    "_meta/usage/",
    "generated/",  # Graphify/export dumps if ever copied under the wiki
)


def is_wiki_content_page(rel: str, name: str | None = None) -> bool:
    if name == "log.md" or name == "_TEMPLATE.md":
        return False
    if "inbox/archive" in rel.replace("\\", "/"):
        return False
    rel = rel.replace("\\", "/")
    return not any(rel == p.rstrip("/") or rel.startswith(p) for p in SKIP_PREFIXES)
