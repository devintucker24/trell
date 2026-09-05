#!/usr/bin/env python3
"""Versioned, read-only adapter for Graphify-owned code graphs.

Graphify owns extraction, graph merging, analysis, and rendering.  This module
only validates its artifact, delegates its public CLI, and records RepoBrain
build provenance beside the generated graph.
"""

from __future__ import annotations

import fnmatch
import importlib.metadata
import json
import os
import re
import shutil
import subprocess
import tempfile
from collections import Counter
from collections.abc import Iterator, Mapping, Sequence
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from types import MappingProxyType
from typing import Any

from repobrain_paths import ROOT, load_host


GRAPHIFY_REQUIREMENT = "graphifyy>=0.9.54,<0.10"
GRAPHIFY_INSTALL_COMMAND = (
    "python3 -m pip install --user 'graphifyy>=0.9.54,<0.10'"
)
DEFAULT_EXCLUDES = (
    "**/target/**",
    "**/node_modules/**",
    "**/vendor/**",
    "**/dist/**",
    "**/build/**",
    "**/generated/**",
)
SUPPORTED_MINIMUM = (0, 9, 54)
SUPPORTED_EXCLUSIVE_MAXIMUM = (0, 10, 0)
CONFIDENCE_CLASSES = ("EXTRACTED", "INFERRED", "AMBIGUOUS")
PROVENANCE_FILE = ".repobrain-provenance.json"


class GraphifyAdapterError(RuntimeError):
    """Actionable adapter error suitable for a CLI diagnostic."""


class GraphArtifactError(GraphifyAdapterError):
    """The Graphify artifact is missing, corrupt, or structurally invalid."""


@dataclass(frozen=True)
class CliInfo:
    path: Path | None
    version: str | None
    compatible: bool
    diagnostic: str


class NormalizedGraph(Mapping[str, Any]):
    """Immutable adapter view over raw or NetworkX node-link JSON."""

    def __init__(
        self,
        *,
        nodes: Sequence[Mapping[str, Any]],
        edges: Sequence[Mapping[str, Any]],
        schema: str,
        metadata: Mapping[str, Any],
    ) -> None:
        self.nodes = tuple(MappingProxyType(dict(node)) for node in nodes)
        self.edges = tuple(MappingProxyType(dict(edge)) for edge in edges)
        self.schema = schema
        self.metadata = MappingProxyType(dict(metadata))

    def __getitem__(self, key: str) -> Any:
        if key == "nodes":
            return self.nodes
        if key in ("edges", "links"):
            return self.edges
        if key == "schema":
            return self.schema
        return self.metadata[key]

    def __iter__(self) -> Iterator[str]:
        yield from self.metadata
        yield "nodes"
        yield "edges"
        yield "schema"

    def __len__(self) -> int:
        return len(self.metadata) + 3

    @property
    def confidence_counts(self) -> dict[str, int]:
        counts: Counter[str] = Counter()
        for edge in self.edges:
            value = edge.get("confidence")
            label = str(value).upper() if value not in (None, "") else "UNQUALIFIED"
            counts[label] += 1
        return {
            **{label: counts.pop(label, 0) for label in CONFIDENCE_CLASSES},
            **dict(sorted(counts.items())),
        }


def graphify_cfg(host: dict | None = None) -> dict:
    """Return normalized Graphify configuration with legacy target support."""
    host = host if host is not None else load_host()
    cfg = dict(host.get("graphify") or {})
    cfg.setdefault("enabled", True)
    cfg.setdefault("requirement", GRAPHIFY_REQUIREMENT)
    cfg.setdefault("code_only", True)
    cfg.setdefault("out", "graphify-out")
    roots = cfg.get("roots") or cfg.get("targets") or host.get("code_roots") or ["src"]
    if isinstance(roots, (str, Path)):
        roots = [str(roots)]
    cfg["roots"] = [_clean_relative_path(value) for value in roots if str(value).strip()]
    # Keep the old spelling available to setup/retrieve/eval callers.
    cfg["targets"] = list(cfg["roots"])
    excludes = cfg.get("excludes")
    if excludes is None:
        excludes = DEFAULT_EXCLUDES
    elif isinstance(excludes, str):
        excludes = [excludes]
    cfg["excludes"] = list(dict.fromkeys(str(value) for value in excludes if str(value)))
    cfg.setdefault("emit_html", False)
    return cfg


def _clean_relative_path(value: Any) -> str:
    text = str(value).strip()
    if text in ("", "."):
        return "."
    if Path(text).anchor == text:
        return text
    return text.rstrip("/\\")


def graph_dir(cfg: dict | None = None) -> Path:
    cfg = cfg or graphify_cfg()
    out = Path(cfg.get("out") or "graphify-out")
    if not out.is_absolute():
        out = ROOT / out
    return out if out.name == "graphify-out" else out / "graphify-out"


def graph_json_path(cfg: dict | None = None) -> Path:
    return graph_dir(cfg) / "graph.json"


def graph_html_path(cfg: dict | None = None) -> Path:
    return graph_dir(cfg) / "graph.html"


def find_graphify() -> Path | None:
    found = shutil.which("graphify")
    if found:
        return Path(found)
    fallback = Path.home() / ".local/bin/graphify"
    return fallback if fallback.is_file() else None


def _distribution_version() -> str | None:
    try:
        return importlib.metadata.version("graphifyy")
    except importlib.metadata.PackageNotFoundError:
        return None


def _parse_version(text: str) -> tuple[int, int, int] | None:
    match = re.search(r"(?<!\d)(\d+)\.(\d+)\.(\d+)", text)
    return tuple(map(int, match.groups())) if match else None


def _supported(version: str | None) -> bool:
    parsed = _parse_version(version or "")
    return bool(
        parsed
        and SUPPORTED_MINIMUM <= parsed < SUPPORTED_EXCLUSIVE_MAXIMUM
    )


def cli_info() -> CliInfo:
    path = find_graphify()
    package_version = _distribution_version()
    if path is None:
        package_note = (
            f" Package graphifyy {package_version} is installed, but its "
            "`graphify` executable is not on PATH."
            if package_version
            else ""
        )
        return CliInfo(
            path=None,
            version=package_version,
            compatible=False,
            diagnostic=(
                "Graphify CLI not found."
                f"{package_note} Install the supported release with: "
                f"{GRAPHIFY_INSTALL_COMMAND}"
            ),
        )
    try:
        proc = subprocess.run(
            [str(path), "--version"],
            cwd=ROOT,
            check=False,
            text=True,
            capture_output=True,
        )
    except OSError as exc:
        return CliInfo(
            path=path,
            version=package_version,
            compatible=False,
            diagnostic=f"Cannot execute Graphify CLI at {path}: {exc}",
        )
    output = ((proc.stdout or "") + "\n" + (proc.stderr or "")).strip()
    parsed = _parse_version(output)
    version = ".".join(map(str, parsed)) if parsed else None
    if proc.returncode != 0 or not _supported(version):
        shown = version or output or "unknown"
        return CliInfo(
            path=path,
            version=version,
            compatible=False,
            diagnostic=(
                f"Unsupported Graphify CLI version {shown}; RepoBrain requires "
                f"{GRAPHIFY_REQUIREMENT}. Install with: {GRAPHIFY_INSTALL_COMMAND}"
            ),
        )
    return CliInfo(
        path=path,
        version=version,
        compatible=True,
        diagnostic=f"Graphify {version} satisfies {GRAPHIFY_REQUIREMENT}.",
    )


def require_compatible_cli() -> CliInfo:
    info = cli_info()
    if not info.compatible:
        raise GraphifyAdapterError(info.diagnostic)
    return info


def run_graphify(
    args: list[str],
    check: bool = True,
    capture: bool = False,
) -> subprocess.CompletedProcess[str]:
    info = require_compatible_cli()
    env = os.environ.copy()
    local_bin = str(Path.home() / ".local/bin")
    env["PATH"] = local_bin + os.pathsep + env.get("PATH", "")
    proc = subprocess.run(
        [str(info.path), *args],
        cwd=ROOT,
        env=env,
        check=False,
        text=True,
        capture_output=capture,
    )
    if check and proc.returncode != 0:
        detail = (proc.stderr or proc.stdout or "").strip()
        suffix = f": {detail}" if detail else ""
        raise GraphifyAdapterError(
            f"Graphify command exited {proc.returncode}: "
            f"graphify {' '.join(args)}{suffix}"
        )
    return proc


def normalize_graph(raw: Any, *, source: str = "graph.json") -> NormalizedGraph:
    if not isinstance(raw, dict):
        raise GraphArtifactError(
            f"Malformed Graphify artifact {source}: expected a JSON object. "
            "Run `./repobrain graph sync --force` to rebuild it."
        )
    if "nodes" not in raw:
        raise GraphArtifactError(
            f"Partial Graphify artifact {source}: missing required `nodes` list. "
            "Run `./repobrain graph sync --force`."
        )
    if not isinstance(raw["nodes"], list):
        raise GraphArtifactError(
            f"Malformed Graphify artifact {source}: `nodes` must be a list."
        )
    has_edges, has_links = "edges" in raw, "links" in raw
    if not has_edges and not has_links:
        raise GraphArtifactError(
            f"Partial Graphify artifact {source}: expected an `edges` or `links` list. "
            "Run `./repobrain graph sync --force`."
        )
    if has_edges and has_links:
        if raw["edges"] == raw["links"]:
            edge_key = "edges"
            schema = "edges+links"
        elif not raw["edges"]:
            edge_key = "links"
            schema = "links"
        elif not raw["links"]:
            edge_key = "edges"
            schema = "edges"
        else:
            raise GraphArtifactError(
                f"Conflicting Graphify artifact {source}: `edges` and `links` differ. "
                "Do not hand-edit graph.json; run `./repobrain graph sync --force`."
            )
    else:
        edge_key = "edges" if has_edges else "links"
        schema = edge_key
    edges = raw[edge_key]
    if not isinstance(edges, list):
        raise GraphArtifactError(
            f"Malformed Graphify artifact {source}: `{edge_key}` must be a list."
        )
    if not raw["nodes"]:
        raise GraphArtifactError(
            f"Partial Graphify artifact {source}: `nodes` is empty. "
            "Run `./repobrain graph sync --force`."
        )
    _validate_records(raw["nodes"], "nodes", source, required=("id",))
    _validate_records(edges, edge_key, source, required=("source", "target"))
    node_ids = [str(node["id"]) for node in raw["nodes"]]
    if len(node_ids) != len(set(node_ids)):
        raise GraphArtifactError(
            f"Malformed Graphify artifact {source}: duplicate node IDs."
        )
    known = set(node_ids)
    dangling = sorted(
        {
            str(endpoint)
            for edge in edges
            for endpoint in (edge["source"], edge["target"])
            if str(endpoint) not in known
        }
    )
    if dangling:
        raise GraphArtifactError(
            f"Partial Graphify artifact {source}: edges reference missing nodes "
            f"{', '.join(dangling[:5])}."
        )
    metadata = {
        key: value
        for key, value in raw.items()
        if key not in ("nodes", "edges", "links", "schema")
    }
    return NormalizedGraph(
        nodes=raw["nodes"],
        edges=edges,
        schema=schema,
        metadata=metadata,
    )


def _validate_records(
    values: list[Any],
    field: str,
    source: str,
    *,
    required: tuple[str, ...],
) -> None:
    for index, value in enumerate(values):
        if not isinstance(value, dict):
            raise GraphArtifactError(
                f"Malformed Graphify artifact {source}: {field}[{index}] "
                "must be an object."
            )
        missing = [key for key in required if value.get(key) in (None, "")]
        if missing:
            raise GraphArtifactError(
                f"Partial Graphify artifact {source}: {field}[{index}] is missing "
                f"{', '.join(f'`{key}`' for key in missing)}."
            )


def load_code_graph(cfg: dict | None = None) -> NormalizedGraph:
    path = graph_json_path(cfg)
    return _load_graph_path(path)


def _load_graph_path(path: Path) -> NormalizedGraph:
    if not path.exists():
        raise GraphArtifactError(
            f"Graphify artifact is missing at {path}. "
            "Run `./repobrain graph sync` first."
        )
    try:
        raw = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        raise GraphArtifactError(
            f"Corrupt Graphify artifact {path}: {exc}. "
            "Run `./repobrain graph sync --force` to rebuild it."
        ) from exc
    return normalize_graph(raw, source=str(path))


def _configured_roots(cfg: dict) -> list[Path]:
    roots: list[Path] = []
    for value in cfg["roots"]:
        path = Path(value)
        roots.append(path if path.is_absolute() else ROOT / path)
    return roots


def _extract_args(root: Path, out_root: Path, cfg: dict, force: bool) -> list[str]:
    args = ["extract", str(root), "--out", str(out_root)]
    if cfg.get("code_only", True):
        args.append("--code-only")
    for pattern in cfg["excludes"]:
        args.extend(["--exclude", pattern])
    if force:
        args.append("--force")
    return args


def cmd_sync(force: bool = False, html: bool = False) -> Path:
    cfg = graphify_cfg()
    if not cfg.get("enabled", True):
        raise GraphifyAdapterError("Graphify is disabled by HOST.yaml `graphify.enabled`.")
    require_compatible_cli()
    roots = _configured_roots(cfg)
    missing = [str(path) for path in roots if not path.exists()]
    if missing:
        raise GraphifyAdapterError(
            "Configured Graphify roots do not exist: "
            + ", ".join(missing)
            + ". Update HOST.yaml `graphify.roots`."
        )
    if not roots:
        raise GraphifyAdapterError(
            "No Graphify roots are configured; set HOST.yaml `graphify.roots`."
        )

    destination = graph_json_path(cfg)
    destination.parent.mkdir(parents=True, exist_ok=True)
    try:
        before = load_code_graph(cfg)
    except GraphArtifactError:
        before = None

    with tempfile.TemporaryDirectory(prefix="repobrain-graphify-") as temp:
        temp_root = Path(temp)
        pieces: list[Path] = []
        for index, root in enumerate(roots):
            out_root = temp_root / f"root-{index:03d}"
            args = _extract_args(root, out_root, cfg, force)
            print("graphify " + " ".join(args))
            run_graphify(args)
            pieces.append(out_root / "graphify-out" / "graph.json")
        if len(pieces) == 1:
            candidate = pieces[0]
        else:
            candidate = temp_root / "merged-graph.json"
            merge = [
                "merge-graphs",
                *(str(piece) for piece in pieces),
                "--out",
                str(candidate),
            ]
            print("graphify " + " ".join(merge))
            run_graphify(merge)
        graph = _load_graph_path(candidate)
        if before is not None and len(graph.nodes) < len(before.nodes) and not force:
            raise GraphifyAdapterError(
                "Graphify rebuild contains fewer nodes than the current artifact. "
                "Review the configured roots/excludes, then rerun with "
                "`./repobrain graph sync --force` if the reduction is expected."
            )
        shutil.copy2(candidate, destination)

    _write_provenance(cfg)
    if html or cfg.get("emit_html", False):
        run_graphify(["export", "html", "--graph", str(destination)])
    print(f"code graph → {destination} ({len(graph.nodes)} nodes, {len(graph.edges)} edges)")
    return destination


def _git_head() -> str | None:
    try:
        proc = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=ROOT,
            check=False,
            text=True,
            capture_output=True,
        )
    except OSError:
        return None
    return proc.stdout.strip() if proc.returncode == 0 else None


def _write_provenance(cfg: dict) -> None:
    data = {
        "built_at": datetime.now(timezone.utc).isoformat(),
        "built_commit": _git_head(),
        "roots": cfg["roots"],
        "excludes": cfg["excludes"],
    }
    (graph_dir(cfg) / PROVENANCE_FILE).write_text(
        json.dumps(data, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def _read_provenance(cfg: dict) -> dict:
    path = graph_dir(cfg) / PROVENANCE_FILE
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
        return value if isinstance(value, dict) else {}
    except (OSError, UnicodeError, json.JSONDecodeError):
        return {}


def _artifact_built_commit(graph: NormalizedGraph, cfg: dict) -> str | None:
    provenance = _read_provenance(cfg)
    if provenance.get("built_commit"):
        return str(provenance["built_commit"])
    graph_meta = graph.metadata.get("graph")
    candidates = [graph.metadata]
    if isinstance(graph_meta, dict):
        candidates.insert(0, graph_meta)
    for data in candidates:
        for key in (
            "built_at_commit",
            "built_commit",
            "source_commit",
            "commit",
            "git_commit",
        ):
            if data.get(key):
                return str(data[key])
    return None


def _git_lines(args: list[str]) -> list[str] | None:
    try:
        proc = subprocess.run(
            ["git", *args],
            cwd=ROOT,
            check=False,
            text=True,
            capture_output=True,
        )
    except OSError:
        return None
    if proc.returncode != 0:
        return None
    return [line for line in proc.stdout.splitlines() if line]


def _changed_sources_since(built_commit: str, cfg: dict) -> list[str] | None:
    verify = _git_lines(["cat-file", "-e", f"{built_commit}^{{commit}}"])
    if verify is None:
        return None
    roots = list(cfg["roots"])
    commands = [
        ["diff", "--name-only", f"{built_commit}..HEAD", "--", *roots],
        ["diff", "--name-only", "--", *roots],
        ["diff", "--cached", "--name-only", "--", *roots],
        ["ls-files", "--others", "--exclude-standard", "--", *roots],
    ]
    changed: set[str] = set()
    for command in commands:
        lines = _git_lines(command)
        if lines is None:
            return None
        changed.update(
            line for line in lines if not _is_excluded(line, cfg["excludes"])
        )
    return sorted(changed)


def _is_excluded(relative: str, patterns: Sequence[str]) -> bool:
    posix = relative.replace(os.sep, "/")
    padded = f"/{posix.strip('/')}/"
    for pattern in patterns:
        cleaned = pattern.replace("\\", "/")
        if fnmatch.fnmatch(posix, cleaned) or fnmatch.fnmatch("/" + posix, cleaned):
            return True
        token = cleaned.replace("**/", "").replace("/**", "").strip("/")
        if token and f"/{token}/" in padded:
            return True
    return False


def _latest_source_mtime(cfg: dict) -> float | None:
    latest: float | None = None
    for root in _configured_roots(cfg):
        if root.is_file():
            paths = [root]
        elif root.is_dir():
            paths = (path for path in root.rglob("*") if path.is_file())
        else:
            continue
        for path in paths:
            try:
                relative = str(path.relative_to(ROOT))
            except ValueError:
                relative = str(path)
            if _is_excluded(relative, cfg["excludes"]):
                continue
            try:
                mtime = path.stat().st_mtime
            except OSError:
                continue
            latest = mtime if latest is None else max(latest, mtime)
    return latest


def status_data() -> dict:
    cfg = graphify_cfg()
    info = cli_info()
    path = graph_json_path(cfg)
    missing_roots = [
        str(root)
        for root in _configured_roots(cfg)
        if not root.exists()
    ]
    artifact: dict[str, Any] = {
        "path": str(path),
        "state": "missing",
        "schema": None,
        "nodes": None,
        "edges": None,
        "confidence": {label: 0 for label in CONFIDENCE_CLASSES},
        "diagnostic": (
            f"Graphify artifact is missing at {path}; run `./repobrain graph sync`."
        ),
    }
    graph: NormalizedGraph | None = None
    if path.exists():
        try:
            graph = load_code_graph(cfg)
            artifact.update(
                state="ready",
                schema=graph.schema,
                nodes=len(graph.nodes),
                edges=len(graph.edges),
                confidence=graph.confidence_counts,
                diagnostic=None,
            )
        except GraphArtifactError as exc:
            artifact.update(state="corrupt", diagnostic=str(exc))

    current_commit = _git_head()
    built_commit = _artifact_built_commit(graph, cfg) if graph else None
    if built_commit and current_commit:
        commit_freshness = "fresh" if built_commit == current_commit else "stale"
    else:
        commit_freshness = "unknown"
    changed_sources = (
        _changed_sources_since(built_commit, cfg)
        if graph is not None and built_commit
        else None
    )
    if changed_sources is not None:
        source_freshness = "fresh" if not changed_sources else "stale"
        freshness_method = "git-diff"
    else:
        latest_source = _latest_source_mtime(cfg)
        freshness_method = "mtime"
        changed_sources = []
        if graph is None or latest_source is None:
            source_freshness = "unknown"
        else:
            try:
                source_freshness = (
                    "fresh" if latest_source <= path.stat().st_mtime else "stale"
                )
            except OSError:
                source_freshness = "unknown"

    html_path = graph_html_path(cfg)
    html_available = html_path.exists()
    try:
        html_fresh = html_available and html_path.stat().st_mtime >= path.stat().st_mtime
    except OSError:
        html_fresh = False
    return {
        "cli": {
            "path": str(info.path) if info.path else None,
            "version": info.version,
            "requirement": GRAPHIFY_REQUIREMENT,
            "compatible": info.compatible,
            "diagnostic": info.diagnostic,
            "install_command": GRAPHIFY_INSTALL_COMMAND,
        },
        "config": {
            "enabled": bool(cfg.get("enabled", True)),
            "roots": cfg["roots"],
            "excludes": cfg["excludes"],
            "code_only": bool(cfg.get("code_only", True)),
            "out": str(cfg["out"]),
            "emit_html": bool(cfg.get("emit_html", False)),
            "missing_roots": missing_roots,
        },
        "artifact": artifact,
        "freshness": {
            "built_commit": built_commit,
            "current_commit": current_commit,
            "commit": commit_freshness,
            "source": source_freshness,
            "method": freshness_method,
            "changed_sources": changed_sources,
        },
        "html": {
            "path": str(html_path),
            "available": html_available,
            "fresh": html_fresh,
        },
    }


def cmd_status(as_json: bool = False) -> int:
    data = status_data()
    if as_json:
        print(json.dumps(data, indent=2, sort_keys=True))
    else:
        cli = data["cli"]
        artifact = data["artifact"]
        fresh = data["freshness"]
        html_state = (
            "FRESH"
            if data["html"]["fresh"]
            else "STALE"
            if data["html"]["available"]
            else "MISSING"
        )
        print(f"graphify CLI: {cli['path'] or 'NOT FOUND'}")
        print(
            f"version: {cli['version'] or 'unknown'} "
            f"({'compatible' if cli['compatible'] else 'incompatible'})"
        )
        print(f"requirement: {cli['requirement']}")
        print(f"diagnostic: {cli['diagnostic']}")
        print(f"install: {cli['install_command']}")
        print(f"roots: {data['config']['roots']}")
        if data["config"]["missing_roots"]:
            print(f"missing roots: {data['config']['missing_roots']}")
        print(f"excludes: {data['config']['excludes']}")
        print(f"graph.json: {artifact['path']} {artifact['state'].upper()}")
        if artifact["state"] == "ready":
            print(f"schema: {artifact['schema']}")
            print(f"nodes: {artifact['nodes']}")
            print(f"edges: {artifact['edges']} {artifact['confidence']}")
        else:
            print(f"artifact diagnostic: {artifact['diagnostic']}")
        print(
            f"built commit: {fresh['built_commit'] or 'unknown'} "
            f"(current {fresh['current_commit'] or 'unknown'}; {fresh['commit']})"
        )
        print(
            f"source freshness: {fresh['source']} "
            f"({fresh['method']}; changed={fresh['changed_sources']})"
        )
        print(f"graph.html: {data['html']['path']} {html_state}")
    return 0 if (
        data["cli"]["compatible"]
        and not data["config"]["missing_roots"]
        and data["artifact"]["state"] == "ready"
        and data["freshness"]["source"] != "stale"
    ) else 1


def _preflight_graph(cfg: dict) -> Path:
    path = graph_json_path(cfg)
    load_code_graph(cfg)
    return path


def cmd_operation(verb: str, rest: list[str]) -> int:
    cfg = graphify_cfg()
    path = _preflight_graph(cfg)
    if verb == "export-html":
        args = ["export", "html", *rest, "--graph", str(path)]
    elif verb == "export-wiki":
        args = ["export", "wiki", *rest, "--graph", str(path)]
    else:
        args = [verb, *rest, "--graph", str(path)]
    proc = run_graphify(args, check=False, capture=False)
    return proc.returncode


def seedable_god_nodes(graph: Mapping[str, Any], top: int = 12) -> list[dict]:
    """Draft-only helper retained for setup; public god-nodes delegates."""
    degrees: Counter[str] = Counter()
    for edge in graph.get("edges") or ():
        if edge.get("source"):
            degrees[str(edge["source"])] += 1
        if edge.get("target"):
            degrees[str(edge["target"])] += 1
    by_id = {
        str(node["id"]): node
        for node in (graph.get("nodes") or ())
        if node.get("id") not in (None, "")
    }
    seeds: list[dict] = []
    for node_id, degree in degrees.most_common(top * 2):
        node = dict(by_id.get(node_id) or {})
        label = str(node.get("label") or node.get("id") or "")
        slug = _kebab(label)
        if (
            not label
            or label.startswith(".")
            or "(" in label
            or "'" in label
            or slug in {
                "string", "option", "result", "vec", "hashmap", "box",
                "vecdeque", "error", "ok", "none", "some", "true", "false",
                "self", "super",
            }
            or len(slug) < 3
            or not node.get("source_file")
        ):
            continue
        node.update(degree=degree, slug=slug)
        seeds.append(node)
        if len(seeds) >= top:
            break
    return seeds


def _kebab(label: str) -> str:
    slug = re.sub(r"[^a-z0-9]+", "-", label.strip().lower()).strip("-")
    return re.sub(r"-+", "-", slug)
