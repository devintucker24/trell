"""Canonical repository and RepoBrain path resolver.

All engine code crosses this interface instead of deriving paths independently.
The resolver is location-based, so an exported engine works in any repository.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class RepoBrainPaths:
    """Resolved ownership roots for one RepoBrain installation."""

    repository: Path
    corpus: Path
    system: Path
    config: Path
    skills: Path
    scripts: Path
    templates: Path
    logs: Path
    generated: Path
    graphify: Path

    @classmethod
    def discover(cls, anchor: Path | None = None) -> "RepoBrainPaths":
        resolved = (anchor or Path(__file__)).resolve()
        system = next(
            (parent for parent in resolved.parents if parent.name == "_system"),
            None,
        )
        if system is None:
            raise RuntimeError(
                f"Cannot locate docs/wiki/_system from resolver anchor {resolved}"
            )
        corpus = system.parent
        if corpus.name != "wiki" or corpus.parent.name != "docs":
            raise RuntimeError(
                f"RepoBrain system must live at docs/wiki/_system, got {system}"
            )
        repository = corpus.parent.parent
        return cls(
            repository=repository,
            corpus=corpus,
            system=system,
            config=system / "config",
            skills=system / "skills",
            scripts=system / "scripts",
            templates=system / "templates",
            logs=system / "logs",
            generated=system / "generated",
            graphify=repository / "graphify-out",
        )

    @property
    def host_config(self) -> Path:
        return self.config / "HOST.yaml"

    @property
    def router_seeds(self) -> Path:
        return self.config / "router-seeds.md"

    @property
    def claim_graph(self) -> Path:
        return self.generated / "claim-graph.yaml"

    @property
    def doctor_dir(self) -> Path:
        return self.generated / "doctor"

    @property
    def doctor_latest(self) -> Path:
        return self.doctor_dir / "latest.json"

    @property
    def eval_config(self) -> Path:
        return self.config / "eval-queries.yaml"

    @property
    def eval_dir(self) -> Path:
        return self.generated / "eval"

    @property
    def usage_dir(self) -> Path:
        return self.generated / "usage"

    @property
    def usage_events(self) -> Path:
        return self.usage_dir / "events.jsonl"

    @property
    def usage_dashboard(self) -> Path:
        return self.usage_dir / "dashboard.md"


PATHS = RepoBrainPaths.discover()

# Transitional aliases keep existing operator imports working while every
# caller migrates to PATHS. New code should use the ownership names above.
ROOT = PATHS.repository
WIKI = PATHS.corpus
SYSTEM = PATHS.system
CONFIG = PATHS.config
SKILLS = PATHS.skills
SCRIPTS = PATHS.scripts
TEMPLATES = PATHS.templates
LOGS = PATHS.logs
GENERATED = PATHS.generated
META = PATHS.generated
HOST_PATH = PATHS.host_config
USAGE_DIR = PATHS.usage_dir
EVENTS_PATH = PATHS.usage_events
DASHBOARD_PATH = PATHS.usage_dashboard


def load_host(paths: RepoBrainPaths = PATHS) -> dict:
    """Load the host-owned overlay from the system config area."""
    if not paths.host_config.exists():
        return {}
    try:
        import yaml

        return yaml.safe_load(
            paths.host_config.read_text(encoding="utf-8")
        ) or {}
    except Exception:  # noqa: BLE001
        return {}


def corpus_dirs(paths: RepoBrainPaths = PATHS) -> set[str]:
    host = load_host(paths)
    semantic = host.get("semantic_dirs") or []
    return set(semantic) | {"inbox", "episodic", "temporal", "raw"}


def is_wiki_content_page(
    rel: str,
    name: str | None = None,
    paths: RepoBrainPaths = PATHS,
) -> bool:
    """Return whether a Markdown path is host corpus rather than engine state."""
    rel = rel.replace("\\", "/")
    filename = name or Path(rel).name
    if filename == "_TEMPLATE.md" or "inbox/archive" in rel:
        return False
    if rel == "INDEX.md":
        return True
    if "/" not in rel:
        return False
    return rel.split("/", 1)[0] in corpus_dirs(paths)
