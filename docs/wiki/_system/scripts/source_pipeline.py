"""Deterministic, git-native source inventory for RepoBrain.

This module owns discovery, classification, manifests, grouped raw pointers,
bounded raw excerpts, and the document-conversion cache seam.  It deliberately
does not write semantic corpus pages and never indexes code content; code is
only inventoried and delegated to Graphify.
"""

from __future__ import annotations

import argparse
import fnmatch
import hashlib
import json
import re
import subprocess
import sys
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path, PurePosixPath
from typing import Any, Mapping, Protocol


SCHEMA_VERSION = 1
MANAGED_BY = "repobrain-source-pipeline"
DEFAULT_EXCLUDES = (
    ".git/**",
    "**/.git/**",
    "node_modules/**",
    "**/node_modules/**",
    "target/**",
    "**/target/**",
    "vendor/**",
    "**/vendor/**",
    "dist/**",
    "**/dist/**",
    "build/**",
    "**/build/**",
    ".cache/**",
    "**/.cache/**",
    "graphify-out/**",
    "docs/wiki/_system/generated/**",
    "docs/wiki/raw/**",
    "docs/wiki/inbox/**",
    "docs/wiki/episodic/**",
    "docs/wiki/temporal/**",
)
CODE_EXTENSIONS = {
    ".c", ".cc", ".cpp", ".cs", ".ex", ".exs", ".go", ".h", ".hpp",
    ".java", ".js", ".jsx", ".kt", ".lua", ".php", ".py", ".rb", ".rs",
    ".scala", ".sh", ".swift", ".ts", ".tsx", ".trell",
}
DOC_EXTENSIONS = {
    ".adoc", ".asc", ".markdown", ".md", ".mdx", ".org", ".rst", ".tex",
    ".txt",
}
CONFIG_EXTENSIONS = {
    ".cfg", ".conf", ".ini", ".json", ".properties", ".toml", ".yaml", ".yml",
}
DATA_EXTENSIONS = {
    ".csv", ".jsonl", ".ndjson", ".parquet", ".sql", ".sqlite", ".tsv", ".xml",
}
MEDIA_BINARY_EXTENSIONS = {
    ".7z", ".avi", ".bmp", ".doc", ".docx", ".eot", ".epub", ".gif", ".gz",
    ".ico", ".jpeg", ".jpg", ".mov", ".mp3", ".mp4", ".odp", ".ods", ".odt",
    ".pdf", ".png", ".ppt", ".pptx", ".svg", ".tar", ".tif", ".tiff", ".ttf",
    ".wav", ".webm", ".webp", ".woff", ".woff2", ".xls", ".xlsx", ".zip",
}
SAFE_LOCAL_FORMATS = ("csv", "html", "epub", "pdf", "docx", "pptx", "xlsx")
FORMAT_EXTENSIONS = {
    "csv": {".csv"},
    "html": {".html", ".htm"},
    "epub": {".epub"},
    "pdf": {".pdf"},
    "docx": {".docx"},
    "pptx": {".pptx"},
    "xlsx": {".xlsx"},
}
FORMAT_EXTRAS = {
    "pdf": "pdf",
    "docx": "docx",
    "pptx": "pptx",
    "xlsx": "xlsx",
}
FORMAT_EXTRA_MODULES = {
    "pdf": "pdfminer",
    "docx": "mammoth",
    "pptx": "pptx",
    "xlsx": "openpyxl",
}
CONVERTIBLE_EXTENSIONS = {
    suffix for suffixes in FORMAT_EXTENSIONS.values() for suffix in suffixes
}
TRACER_FORMAT = "csv"
MARKITDOWN_REQUIREMENT = "markitdown==0.1.7"
MARKITDOWN_LICENSE = "MIT"
EXTERNAL_CONVERSION_FLAGS = (
    "allow_plugins",
    "allow_urls",
    "allow_ocr",
    "allow_media",
    "allow_cloud",
    "enable_plugins",
)
NATIVE_SEARCH_CLASSES = {"adr", "config", "data", "docs"}
SECRET_NAMES = {
    ".env", ".envrc", "authorized_keys", "credentials", "credentials.json",
    "id_dsa", "id_ecdsa", "id_ed25519", "id_rsa", "known_hosts",
    "secrets.json",
}
SECRET_SUFFIXES = {".key", ".keystore", ".p12", ".pfx", ".pem"}
SECRET_SEGMENTS = {".aws", ".gnupg", ".ssh", "secrets"}
POINTER_GROUPS = (
    ("context", "Context maps"),
    ("adrs", "Architecture decisions"),
    ("documentation", "Documentation sources"),
    ("data-config", "Data and configuration sources"),
)
DEFAULT_POINTER_TEMPLATE = """---
id: raw-sources-{{ group_id }}
title: "{{ title }}"
type: raw-pointer
status: active
created: '{{ date }}'
updated: '{{ date }}'
tags: [raw, source-inventory, {{ group_id }}]
domain: meta
summary: "Managed non-authoritative pointers to {{ title_lower }}."
origin: source-manifest
managed_by: repobrain-source-pipeline
nodes: []
edges: []
related:
  - "[[INDEX]]"
agent:
  priority: low
  read_when:
    - consulting original repository sources
  maintain:
    - do not promote these paths into compiled claims
---

# {{ title }}

These are inventory pointers to original repository sources. They are raw,
non-authoritative material and are not compiled semantic claims.

{{ entries }}
"""


class SourcePipelineError(RuntimeError):
    """Base error for source inventory failures."""


class SourcePathError(SourcePipelineError):
    """A configured or discovered path escaped its managed repository root."""


@dataclass(frozen=True)
class SourceConfig:
    """Normalized host-owned source inventory configuration."""

    include: tuple[str, ...] = ()
    exclude: tuple[str, ...] = ()
    deny_globs: tuple[str, ...] = ()
    conversion: Mapping[str, Any] | None = None
    max_index_bytes: int = 262144
    max_excerpt_chars: int = 1200
    enabled: bool = True
    scan_on_setup: bool = True

    @property
    def effective_excludes(self) -> tuple[str, ...]:
        return tuple(dict.fromkeys((*DEFAULT_EXCLUDES, *self.exclude)))


class DocumentConverter(Protocol):
    """Narrow converter interface; implementations write Markdown locally."""

    name: str
    version: str

    def convert(
        self,
        source: Path,
        destination: Path,
        config: Mapping[str, Any],
    ) -> None:
        """Convert ``source`` to Markdown at ``destination``."""


class MarkItDownConverter:
    """Optional local converter kept behind a replaceable helper."""

    name = "markitdown"

    def __init__(self, version: str = "unknown") -> None:
        self.version = version

    def convert(
        self,
        source: Path,
        destination: Path,
        config: Mapping[str, Any],
    ) -> None:
        del config
        markdown = _markitdown_local_file(source)
        destination.write_text(
            (
                "<!--\n"
                "  repobrain-derived: non-authoritative\n"
                f"  source: {source.name}\n"
                f"  converter: markitdown@{self.version}\n"
                "-->\n\n"
                + markdown
            ),
            encoding="utf-8",
        )


def _markitdown_local_file(source: Path) -> str:
    """The only MarkItDown-specific call site.

    Uses the local-file API with plugins disabled. The generic `convert()`
    method is intentionally not used because it also accepts remote URIs.
    """

    from markitdown import MarkItDown, StreamInfo

    source = Path(source)
    suffix = source.suffix.lower()
    if suffix in {".csv", ".html", ".htm"}:
        try:
            source.read_bytes().decode("utf-8")
        except UnicodeDecodeError as exc:
            raise SourcePipelineError(
                f"{suffix.lstrip('.')} conversion requires UTF-8 input: {source.name}"
            ) from exc
        stream_info = StreamInfo(extension=suffix, charset="utf-8")
    else:
        stream_info = StreamInfo(extension=suffix)
    result = MarkItDown(enable_plugins=False).convert_local(
        source,
        stream_info=stream_info,
    )
    return result.markdown


def enabled_formats(conversion: Mapping[str, Any] | None) -> tuple[str, ...]:
    conversion = conversion or {}
    raw = conversion.get("formats")
    if raw is None and conversion.get("format"):
        raw = [conversion.get("format")]
    if raw is None:
        if "formats" not in conversion and "format" not in conversion:
            return SAFE_LOCAL_FORMATS if conversion.get("enabled", True) else ()
        return (TRACER_FORMAT,)
    if isinstance(raw, str):
        raw = [raw]
    formats = tuple(
        dict.fromkeys(str(item).lower().lstrip(".") for item in raw if str(item).strip())
    )
    return formats or (TRACER_FORMAT,)


def format_for_suffix(suffix: str) -> str | None:
    suffix = suffix.lower()
    for name, extensions in FORMAT_EXTENSIONS.items():
        if suffix in extensions:
            return name
    return None


def extra_install_command(formats: tuple[str, ...] | None = None) -> str:
    extras = [
        FORMAT_EXTRAS[name]
        for name in (formats or ())
        if name in FORMAT_EXTRAS
    ]
    extras = list(dict.fromkeys(extras))
    if not extras:
        return f"python3 -m pip install --user '{MARKITDOWN_REQUIREMENT}'"
    joined = ",".join(extras)
    return (
        f"python3 -m pip install --user 'markitdown[{joined}]==0.1.7'"
    )


def format_extra_missing(fmt: str | None) -> str | None:
    if not fmt:
        return None
    module = FORMAT_EXTRA_MODULES.get(fmt)
    if not module:
        return None
    try:
        __import__(module)
    except ImportError:
        extra = FORMAT_EXTRAS[fmt]
        return (
            f"MarkItDown extra `{extra}` is not installed. "
            + extra_install_command((fmt,))
        )
    return None


def external_conversion_blocked(conversion: Mapping[str, Any] | None) -> str | None:
    conversion = conversion or {}
    if conversion.get("allow_external"):
        return None
    enabled = [flag for flag in EXTERNAL_CONVERSION_FLAGS if conversion.get(flag)]
    if not enabled:
        return None
    return (
        "External conversion flags require sources.conversion.allow_external: "
        + ", ".join(enabled)
    )


def load_source_config(
    host: Mapping[str, Any] | Path | None = None,
) -> SourceConfig:
    """Load and normalize the ``sources:`` HOST overlay.

    A mapping is accepted directly for callers that already loaded HOST.yaml.
    A path is loaded with PyYAML.  ``sources`` may be a list (include shorthand)
    or a mapping with ``include``, ``exclude``, and ``conversion`` keys.
    """

    if isinstance(host, Path):
        import yaml

        loaded = yaml.safe_load(host.read_text(encoding="utf-8")) or {}
        if not isinstance(loaded, Mapping):
            raise SourcePipelineError("HOST config must be a mapping")
        host = loaded
    host = host or {}
    raw = host.get("sources", {})
    if raw is None:
        raw = {}
    if isinstance(raw, (list, tuple)):
        raw = {"include": raw}
    if not isinstance(raw, Mapping):
        raise SourcePipelineError("HOST `sources` must be a mapping or list")
    include = _normalize_patterns(
        raw.get("include") or raw.get("includes"),
        "include",
    )
    exclude = _normalize_patterns(
        raw.get("exclude") or raw.get("excludes"),
        "exclude",
    )
    deny_globs = _normalize_patterns(raw.get("deny_globs"), "deny_globs")
    conversion = raw.get("conversion") or {}
    if not isinstance(conversion, Mapping):
        raise SourcePipelineError("HOST `sources.conversion` must be a mapping")
    return SourceConfig(
        include=include,
        exclude=exclude,
        deny_globs=deny_globs,
        conversion=dict(conversion),
        max_index_bytes=int(raw.get("max_index_bytes") or 262144),
        max_excerpt_chars=int(raw.get("max_excerpt_chars") or 1200),
        enabled=bool(raw.get("enabled", True)),
        scan_on_setup=bool(raw.get("scan_on_setup", True)),
    )


def _normalize_patterns(value: Any, field: str) -> tuple[str, ...]:
    if value is None:
        return ()
    if isinstance(value, str):
        value = [value]
    if not isinstance(value, (list, tuple)):
        raise SourcePipelineError(f"HOST `sources.{field}` must be a list")
    normalized: list[str] = []
    for item in value:
        if not isinstance(item, str) or not item.strip():
            raise SourcePipelineError(
                f"HOST `sources.{field}` entries must be non-empty strings"
            )
        pattern = item.strip().replace("\\", "/")
        pure = PurePosixPath(pattern)
        if pure.is_absolute() or ".." in pure.parts:
            raise SourcePathError(
                f"Source {field} pattern must remain under repository: {item}"
            )
        normalized.append(pattern.removeprefix("./").rstrip("/"))
    return tuple(dict.fromkeys(normalized))


def ensure_under(path: Path, root: Path, label: str = "source") -> Path:
    """Resolve ``path`` and require it to remain below ``root``."""

    resolved_root = root.resolve()
    resolved = path.resolve()
    try:
        resolved.relative_to(resolved_root)
    except ValueError as exc:
        raise SourcePathError(
            f"{label.capitalize()} path must remain under repository root "
            f"{resolved_root}: {path}"
        ) from exc
    return resolved


class SourceInventory:
    """Deep module facade for deterministic source discovery and conversion."""

    def __init__(
        self,
        repo_root: Path,
        *,
        host: Mapping[str, Any] | Path | None = None,
        cache_root: Path | None = None,
        converter: DocumentConverter | None = None,
    ) -> None:
        self.repo_root = repo_root.resolve()
        if not (self.repo_root / ".git").exists():
            raise SourcePipelineError(
                f"Source inventory requires a Git repository: {self.repo_root}"
            )
        self.config = load_source_config(host)
        default_cache = (
            self.repo_root
            / "docs/wiki/_system/generated/sources/cache"
        )
        self.cache_root = ensure_under(
            cache_root or default_cache,
            self.repo_root,
            "cache",
        )
        self.converter = converter

    def scan(
        self,
        *,
        previous: Mapping[str, Any] | Path | None = None,
        generated_at: str | None = None,
    ) -> dict[str, Any]:
        """Scan tracked/default plus host-included files into one manifest."""

        previous_manifest = _load_manifest(previous)
        previous_entries = {
            entry["path"]: entry
            for entry in previous_manifest.get("entries", [])
            if isinstance(entry, Mapping) and isinstance(entry.get("path"), str)
        }
        candidates = set(_git_tracked(self.repo_root))
        for pattern in self.config.include:
            candidates.update(_expand_pattern(self.repo_root, pattern))

        denied: list[dict[str, str]] = []
        entries: list[dict[str, Any]] = []
        for relative in sorted(candidates):
            if _matches_any(relative, self.config.effective_excludes):
                continue
            if likely_secret(relative) or _matches_any(
                relative, self.config.deny_globs
            ):
                denied.append({"path": relative, "reason": "likely-secret"})
                continue
            source = ensure_under(
                self.repo_root / relative,
                self.repo_root,
                "source",
            )
            if not source.is_file():
                continue
            digest = _sha256(source)
            old = previous_entries.get(relative)
            freshness = (
                "added"
                if old is None
                else "unchanged"
                if old.get("sha256") == digest
                else "modified"
            )
            classification = classify_source(relative, source)
            conversion = self._conversion_state(
                source,
                relative,
                digest,
                classification,
                previous_entry=old,
            )
            entries.append(
                {
                    "path": relative,
                    "classification": classification,
                    "size": source.stat().st_size,
                    "sha256": digest,
                    "freshness": freshness,
                    "conversion": conversion,
                }
            )

        current_paths = {entry["path"] for entry in entries}
        added = sorted(
            entry["path"] for entry in entries
            if entry["freshness"] == "added"
        )
        modified = sorted(
            entry["path"] for entry in entries
            if entry["freshness"] == "modified"
        )
        deleted = sorted(set(previous_entries) - current_paths)
        return {
            "schema_version": SCHEMA_VERSION,
            "generated_at": generated_at or _utc_now(),
            "config": {
                "mode": "git-tracked",
                "include": list(self.config.include),
                "exclude": list(self.config.exclude),
                "deny_globs": list(self.config.deny_globs),
                "conversion": dict(self.config.conversion or {}),
                "max_index_bytes": self.config.max_index_bytes,
                "max_excerpt_chars": self.config.max_excerpt_chars,
            },
            "entries": entries,
            "structures": detect_structures(entries, self.repo_root),
            "changes": {
                "added": added,
                "modified": modified,
                "deleted": deleted,
            },
            "denied": sorted(denied, key=lambda item: item["path"]),
        }

    def _conversion_state(
        self,
        source: Path,
        relative: str,
        digest: str,
        classification: str,
        previous_entry: Mapping[str, Any] | None = None,
    ) -> dict[str, Any]:
        previous_conversion = (previous_entry or {}).get("conversion") or {}
        if (
            previous_entry
            and previous_entry.get("sha256") == digest
            and previous_conversion.get("state") == "failed"
            and self.converter is None
        ):
            return dict(previous_conversion)
        if classification == "code":
            return {
                "state": "graphify-delegate",
                "retryable": False,
            }
        suffix = source.suffix.lower()
        fmt = format_for_suffix(suffix)
        enabled = enabled_formats(self.config.conversion)
        conversion_on = bool((self.config.conversion or {}).get("enabled", True))
        should_convert = bool(conversion_on and fmt and fmt in enabled)
        if not should_convert:
            if fmt and conversion_on:
                return {
                    "state": "skipped",
                    "retryable": False,
                    "format": fmt,
                    "diagnostic": f"format {fmt} is not enabled",
                }
            if classification == "media-binary":
                return {"state": "unsupported", "retryable": False}
            return {"state": "native", "retryable": False}
        blocked = external_conversion_blocked(self.config.conversion)
        if blocked:
            return {
                "state": "blocked",
                "retryable": False,
                "format": fmt,
                "diagnostic": blocked,
            }
        if self.converter is None:
            missing = format_extra_missing(fmt)
            if missing:
                return {
                    "state": "pending",
                    "retryable": True,
                    "format": fmt,
                    "diagnostic": missing,
                }
        converter = self.converter or live_converter()
        if converter is None:
            return {
                "state": "pending",
                "retryable": True,
                "format": fmt,
            }
        if self.converter is None:
            cached = conversion_cache_state(
                source,
                source_path=relative,
                source_hash=digest,
                repo_root=self.repo_root,
                cache_root=self.cache_root,
                converter=converter,
                config=self.config.conversion or {},
            )
            if cached["state"] == "cached":
                return cached
            return {
                **cached,
                "state": "pending",
                "retryable": True,
            }
        result = convert_with_cache(
            source,
            source_path=relative,
            source_hash=digest,
            repo_root=self.repo_root,
            cache_root=self.cache_root,
            converter=self.converter,
            config=self.config.conversion or {},
        )
        if result.get("state") in {"cached", "converted"}:
            _maybe_commit_derived(
                result,
                relative=relative,
                classification=classification,
                conversion=self.config.conversion or {},
                repo_root=self.repo_root,
            )
        return result


def scan_sources(
    repo_root: Path,
    *,
    host: Mapping[str, Any] | Path | None = None,
    previous: Mapping[str, Any] | Path | None = None,
    cache_root: Path | None = None,
    converter: DocumentConverter | None = None,
    generated_at: str | None = None,
) -> dict[str, Any]:
    """Functional wrapper around :class:`SourceInventory`."""

    return SourceInventory(
        repo_root,
        host=host,
        cache_root=cache_root,
        converter=converter,
    ).scan(previous=previous, generated_at=generated_at)


def _git_tracked(root: Path) -> list[str]:
    proc = subprocess.run(
        ["git", "ls-files", "-z", "--cached"],
        cwd=root,
        check=False,
        capture_output=True,
    )
    if proc.returncode:
        diagnostic = proc.stderr.decode("utf-8", errors="replace").strip()
        raise SourcePipelineError(
            f"Cannot list Git-tracked sources: {diagnostic or proc.returncode}"
        )
    return sorted(
        item.decode("utf-8", errors="surrogateescape").replace("\\", "/")
        for item in proc.stdout.split(b"\0")
        if item
    )


def _expand_pattern(root: Path, pattern: str) -> list[str]:
    # Pattern safety is established by load_source_config before globbing.
    base = ensure_under(root / pattern.split("*", 1)[0], root, "source")
    del base
    matches: list[str] = []
    for path in root.glob(pattern):
        resolved = ensure_under(path, root, "source")
        if resolved.is_dir():
            matches.extend(
                child.relative_to(root).as_posix()
                for child in sorted(resolved.rglob("*"))
                if child.is_file()
            )
        elif resolved.is_file():
            matches.append(path.relative_to(root).as_posix())
    return matches


def _matches_any(path: str, patterns: tuple[str, ...]) -> bool:
    pure = PurePosixPath(path)
    for pattern in patterns:
        if fnmatch.fnmatchcase(path, pattern) or pure.match(pattern):
            return True
        prefix = pattern.rstrip("/")
        if not any(char in prefix for char in "*?[") and (
            path == prefix or path.startswith(prefix + "/")
        ):
            return True
    return False


def likely_secret(relative: str) -> bool:
    """Conservatively deny common credential and private-key paths."""

    path = PurePosixPath(relative.lower())
    name = path.name
    if any(part in SECRET_SEGMENTS for part in path.parts):
        return True
    if name in SECRET_NAMES or name.startswith(".env."):
        return True
    if path.suffix in SECRET_SUFFIXES:
        return True
    return bool(
        re.search(
            r"(^|[-_.])(credential|private[-_]?key|secret|token)s?($|[-_.])",
            name,
        )
    )


def classify_source(relative: str, source: Path | None = None) -> str:
    """Classify one normalized source path using stable path/extension rules."""

    path = PurePosixPath(relative.lower())
    name = path.name
    suffix = path.suffix
    parts = set(path.parts)
    if (
        "tests" in parts
        or "test" in parts
        or name.startswith(("test_", "test-"))
        or name.endswith(("_test.py", ".test.js", ".test.ts", ".spec.js", ".spec.ts"))
    ):
        return "tests"
    if (
        "adr" in parts
        or "adrs" in parts
        or "decisions" in parts
        or re.match(r"^adr[-_]?\d{3,}[-_]", name)
    ):
        return "adr"
    if suffix in CODE_EXTENSIONS:
        return "code"
    if suffix in DOC_EXTENSIONS or name in {
        "context", "context.md", "readme", "readme.md", "license",
    }:
        return "docs"
    if (
        suffix in CONFIG_EXTENSIONS
        or name
        in {
            ".gitignore", ".npmrc", "dockerfile", "makefile",
            "docusaurus.config.js", "docusaurus.config.ts",
            "astro.config.js", "astro.config.mjs", "astro.config.ts",
        }
    ):
        return "config"
    if suffix in DATA_EXTENSIONS or "data" in parts or "datasets" in parts:
        return "data"
    if suffix in MEDIA_BINARY_EXTENSIONS or (
        source is not None and _looks_binary(source)
    ):
        return "media-binary"
    return "docs"


def detect_structures(
    entries: list[dict[str, Any]],
    repo_root: Path,
) -> dict[str, Any]:
    """Detect context maps, ADRs, and common documentation-site markers."""

    paths = [entry["path"] for entry in entries]
    context = sorted(
        path for path in paths
        if PurePosixPath(path).name.lower()
        in {"context", "context.md", "context.mdx", "context.rst"}
    )
    adrs = sorted(
        entry["path"]
        for entry in entries
        if entry["classification"] == "adr"
    )
    sites: list[dict[str, str]] = []
    for path in paths:
        name = PurePosixPath(path).name.lower()
        kind: str | None = None
        if name in {"mkdocs.yml", "mkdocs.yaml"}:
            kind = "mkdocs"
        elif name in {
            "docusaurus.config.js", "docusaurus.config.mjs",
            "docusaurus.config.ts",
        }:
            kind = "docusaurus"
        elif name in {"mint.json", "docs.json"}:
            kind = "mintlify"
        elif name.startswith("astro.config."):
            source = ensure_under(repo_root / path, repo_root, "source")
            text = _read_text(source, max_chars=32_000).lower()
            if "starlight" in text:
                kind = "astro-starlight"
        if kind:
            sites.append({"kind": kind, "path": path})
    sites.sort(key=lambda item: (item["kind"], item["path"]))
    return {"context": context, "adr": adrs, "docs_sites": sites}


def _cache_key(identity: Mapping[str, Any]) -> str:
    return hashlib.sha256(
        json.dumps(
            identity,
            sort_keys=True,
            separators=(",", ":"),
            ensure_ascii=False,
        ).encode("utf-8")
    ).hexdigest()


def conversion_cache_state(
    source: Path,
    *,
    source_path: str,
    source_hash: str,
    repo_root: Path,
    cache_root: Path,
    converter: DocumentConverter,
    config: Mapping[str, Any],
) -> dict[str, Any]:
    source = ensure_under(source, repo_root, "source")
    cache_root = ensure_under(cache_root, repo_root, "cache")
    identity = {
        "source_sha256": source_hash,
        "converter": converter.name,
        "converter_version": converter.version,
        "config": dict(config),
    }
    cache_key = _cache_key(identity)
    destination = ensure_under(cache_root / f"{cache_key}.md", repo_root, "cache")
    relative_destination = destination.relative_to(repo_root).as_posix()
    base = {
        "converter": {
            "name": converter.name,
            "version": converter.version,
        },
        "cache_key": cache_key,
        "derived_path": relative_destination,
        "source_path": source_path,
        "format": format_for_suffix(source.suffix),
    }
    if destination.is_file():
        return {**base, "state": "cached", "retryable": False}
    return {**base, "state": "pending", "retryable": True}


def _maybe_commit_derived(
    result: Mapping[str, Any],
    *,
    relative: str,
    classification: str,
    conversion: Mapping[str, Any],
    repo_root: Path,
) -> None:
    groups = conversion.get("commit_groups") or []
    if not groups:
        return
    wanted = {str(item).lower() for item in groups}
    if classification.lower() not in wanted and "all" not in wanted:
        return
    derived = result.get("derived_path")
    if not isinstance(derived, str):
        return
    source = ensure_under(repo_root / derived, repo_root, "cache")
    if not source.is_file():
        return
    destination = ensure_under(
        repo_root / "docs/wiki/_system/generated/sources/committed" / f"{relative}.md",
        repo_root,
        "committed conversion",
    )
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(source.read_text(encoding="utf-8"), encoding="utf-8")


def convert_with_cache(
    source: Path,
    *,
    source_path: str,
    source_hash: str,
    repo_root: Path,
    cache_root: Path,
    converter: DocumentConverter,
    config: Mapping[str, Any],
) -> dict[str, Any]:
    """Convert to a content-addressed local cache and return status shape."""

    base = conversion_cache_state(
        source,
        source_path=source_path,
        source_hash=source_hash,
        repo_root=repo_root,
        cache_root=cache_root,
        converter=converter,
        config=config,
    )
    if base["state"] == "cached":
        return base
    destination = ensure_under(
        repo_root / str(base["derived_path"]),
        repo_root,
        "cache",
    )

    destination.parent.mkdir(parents=True, exist_ok=True)
    temporary = ensure_under(
        destination.with_suffix(".tmp"),
        repo_root,
        "cache",
    )
    try:
        converter.convert(source, temporary, config)
        if not temporary.is_file():
            raise SourcePipelineError("converter did not write Markdown output")
        temporary.replace(destination)
    except Exception as exc:  # noqa: BLE001
        temporary.unlink(missing_ok=True)
        return {
            **base,
            "state": "failed",
            "retryable": True,
            "diagnostic": str(exc),
        }
    return {**base, "state": "converted", "retryable": False}


def write_manifest(
    path: Path,
    manifest: Mapping[str, Any],
    *,
    repo_root: Path,
) -> Path:
    """Atomically write canonical JSON, preserving time for equal payloads."""

    destination = ensure_under(path, repo_root, "manifest")
    payload = dict(manifest)
    if destination.is_file():
        try:
            existing = json.loads(destination.read_text(encoding="utf-8"))
        except (json.JSONDecodeError, OSError):
            existing = {}
        if _without_generated_at(existing) == _without_generated_at(payload):
            payload["generated_at"] = existing.get(
                "generated_at",
                payload.get("generated_at"),
            )
    encoded = (
        json.dumps(payload, indent=2, sort_keys=True, ensure_ascii=False)
        + "\n"
    ).encode("utf-8")
    destination.parent.mkdir(parents=True, exist_ok=True)
    temporary = destination.with_suffix(destination.suffix + ".tmp")
    ensure_under(temporary, repo_root, "manifest").write_bytes(encoded)
    temporary.replace(destination)
    return destination


def _without_generated_at(value: Mapping[str, Any]) -> dict[str, Any]:
    result = dict(value)
    result.pop("generated_at", None)
    return result


def write_raw_group_pointers(
    manifest: Mapping[str, Any],
    *,
    wiki_root: Path,
    template_path: Path,
) -> dict[str, list[Path]]:
    """Write only fixed managed pages under ``raw/``; preserve collisions."""

    repository = wiki_root.resolve().parent.parent
    wiki_root = ensure_under(wiki_root, repository, "wiki")
    template_path = ensure_under(template_path, repository, "template")
    template = template_path.read_text(encoding="utf-8")
    raw_root = ensure_under(wiki_root / "raw", repository, "raw pointer")
    groups = _pointer_members(manifest)
    written: list[Path] = []
    collisions: list[Path] = []
    removed: list[Path] = []
    for group_id, title in POINTER_GROUPS:
        destination = ensure_under(
            raw_root / f"source-{group_id}.md",
            repository,
            "raw pointer",
        )
        members = groups[group_id]
        if not members:
            if destination.exists() and f"managed_by: {MANAGED_BY}" in (
                destination.read_text(encoding="utf-8", errors="replace")
            ):
                destination.unlink()
                removed.append(destination)
            continue
        if destination.exists() and f"managed_by: {MANAGED_BY}" not in (
            destination.read_text(encoding="utf-8", errors="replace")
        ):
            collisions.append(destination)
            continue
        links = "\n".join(
            f"- [`{path}`](../../../{path})"
            for path in members
        )
        rendered = _render_pointer(
            template,
            group_id=group_id,
            title=title,
            entries=links,
            date=str(manifest.get("generated_at") or _utc_now())[:10],
        )
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_text(rendered, encoding="utf-8")
        written.append(destination)
    return {"written": written, "collisions": collisions, "removed": removed}


def _pointer_members(
    manifest: Mapping[str, Any],
) -> dict[str, list[str]]:
    entries = manifest.get("entries") or []
    structures = manifest.get("structures") or {}
    contexts = set(structures.get("context") or [])
    adrs = set(structures.get("adr") or [])
    site_markers = {
        site["path"]
        for site in structures.get("docs_sites") or []
        if isinstance(site, Mapping) and isinstance(site.get("path"), str)
    }
    docs = {
        entry["path"]
        for entry in entries
        if entry.get("classification") == "docs"
    }
    data_config = {
        entry["path"]
        for entry in entries
        if entry.get("classification") in {"config", "data"}
    }
    return {
        "context": sorted(contexts),
        "adrs": sorted(adrs),
        "documentation": sorted((docs | site_markers) - contexts - adrs),
        "data-config": sorted(data_config - site_markers),
    }


def _render_pointer(
    template: str,
    *,
    group_id: str,
    title: str,
    entries: str,
    date: str,
) -> str:
    values = {
        "group_id": group_id,
        "title": title,
        "title_lower": title.lower(),
        "entries": entries,
        "date": date,
    }
    rendered = template
    for key, value in values.items():
        rendered = rendered.replace("{{ " + key + " }}", value)
    return rendered.rstrip() + "\n"


def search_raw_sources(
    query: str,
    manifest: Mapping[str, Any],
    *,
    repo_root: Path,
    token_budget: int = 800,
    per_result_tokens: int = 240,
    max_results: int = 5,
) -> list[dict[str, Any]]:
    """Return bounded native/derived excerpts with raw provenance.

    Code and test content are never read.  A code-only query therefore returns
    no raw hits and remains available for the caller to route to Graphify.
    """

    if token_budget <= 0 or per_result_tokens <= 0 or max_results <= 0:
        return []
    terms = _query_terms(query)
    if not terms:
        return []
    candidates: list[tuple[int, str, dict[str, Any], str, str]] = []
    for entry in manifest.get("entries") or []:
        classification = entry.get("classification")
        conversion = entry.get("conversion") or {}
        derived = (
            conversion.get("derived_path")
            if conversion.get("state") in {"converted", "cached"}
            else None
        )
        if classification not in NATIVE_SEARCH_CLASSES and not derived:
            continue
        relative = entry.get("path")
        if not isinstance(relative, str):
            continue
        content_path = derived or relative
        source = ensure_under(repo_root / content_path, repo_root, "source")
        if not source.is_file():
            continue
        if source.stat().st_size > int(
            (manifest.get("config") or {}).get("max_index_bytes") or 262144
        ) and not derived:
            continue
        text = _read_text(source, max_chars=max(16_000, token_budget * 16))
        lowered = text.lower()
        score = sum(lowered.count(term) for term in terms)
        if score <= 0:
            continue
        candidates.append(
            (
                score,
                relative,
                entry,
                text,
                "derived" if derived else "native",
            )
        )
    candidates.sort(key=lambda item: (-item[0], item[1]))

    results: list[dict[str, Any]] = []
    remaining = token_budget
    for score, relative, entry, text, content_kind in candidates:
        if len(results) >= max_results or remaining <= 0:
            break
        cap = min(per_result_tokens, remaining)
        excerpt = _bounded_excerpt(text, terms, cap)
        tokens = estimate_tokens(excerpt)
        if tokens > cap:
            excerpt = excerpt[: cap * 4]
            tokens = estimate_tokens(excerpt)
        if not excerpt:
            continue
        results.append(
            {
                "score": score,
                "classification": entry["classification"],
                "excerpt": excerpt,
                "tokens": tokens,
                "provenance": {
                    "authority": "non-authoritative",
                    "kind": "raw",
                    "source_path": relative,
                    "content": content_kind,
                },
            }
        )
        remaining -= tokens
    return results


def estimate_tokens(text: str) -> int:
    return max(1, (len(text) + 3) // 4)


def _query_terms(query: str) -> list[str]:
    return list(
        dict.fromkeys(
            token
            for token in re.findall(r"[a-z0-9][a-z0-9_-]{1,}", query.lower())
            if token
            not in {
                "a", "an", "and", "are", "for", "from", "in", "is", "of",
                "on", "or", "the", "to", "what", "where", "which", "with",
            }
        )
    )


def _bounded_excerpt(text: str, terms: list[str], token_cap: int) -> str:
    char_cap = max(1, token_cap * 4)
    lowered = text.lower()
    positions = [
        lowered.find(term)
        for term in terms
        if lowered.find(term) >= 0
    ]
    start = max(0, (min(positions) if positions else 0) - char_cap // 4)
    excerpt = " ".join(text[start : start + char_cap].split())
    if start:
        excerpt = "… " + excerpt
    if start + char_cap < len(text):
        excerpt += " …"
    while estimate_tokens(excerpt) > token_cap and excerpt:
        excerpt = excerpt[:-1]
    return excerpt


def _read_text(path: Path, max_chars: int) -> str:
    return path.read_text(encoding="utf-8", errors="replace")[:max_chars]


def _looks_binary(path: Path) -> bool:
    try:
        sample = path.read_bytes()[:4096]
    except OSError:
        return False
    return b"\0" in sample


def _sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def _load_manifest(
    value: Mapping[str, Any] | Path | None,
) -> Mapping[str, Any]:
    if value is None:
        return {}
    if isinstance(value, Path):
        if not value.exists():
            return {}
        loaded = json.loads(value.read_text(encoding="utf-8"))
        if not isinstance(loaded, Mapping):
            raise SourcePipelineError("Previous manifest must be a JSON object")
        return loaded
    return value


def _utc_now() -> str:
    return (
        datetime.now(timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def markitdown_info() -> dict[str, Any]:
    """Report the optional local converter without importing plugins."""

    install = extra_install_command()
    try:
        from importlib.metadata import version

        installed = version("markitdown")
    except Exception:  # noqa: BLE001
        return {
            "name": "markitdown",
            "available": False,
            "version": None,
            "requirement": MARKITDOWN_REQUIREMENT,
            "license": MARKITDOWN_LICENSE,
            "install_command": install,
            "diagnostic": "MarkItDown is not installed.",
        }
    compatible = installed == "0.1.7"
    return {
        "name": "markitdown",
        "available": True,
        "version": installed,
        "requirement": MARKITDOWN_REQUIREMENT,
        "license": MARKITDOWN_LICENSE,
        "install_command": install,
        "compatible": compatible,
        "diagnostic": (
            f"MarkItDown {installed} satisfies {MARKITDOWN_REQUIREMENT}."
            if compatible
            else f"MarkItDown {installed} is not the supported pin {MARKITDOWN_REQUIREMENT}."
        ),
    }


def live_converter() -> MarkItDownConverter | None:
    info = markitdown_info()
    if not info.get("available"):
        return None
    return MarkItDownConverter(version=str(info["version"]))


def detect_and_emit_conflicts(
    query: str,
    packed: list[Mapping[str, Any]],
    *,
    repo_root: Path,
    wiki_root: Path,
) -> list[dict[str, Any]]:
    """Keep compiled claims authoritative and emit idempotent inbox triage."""

    compiled = [
        hit
        for hit in packed
        if (hit.get("provenance") or {}).get("kind") == "compiled"
    ]
    raw = [
        hit
        for hit in packed
        if (hit.get("provenance") or {}).get("kind") == "raw"
    ]
    if not compiled or not raw:
        return []
    inbox = ensure_under(wiki_root / "inbox", repo_root, "inbox")
    inbox.mkdir(parents=True, exist_ok=True)
    conflicts: list[dict[str, Any]] = []
    for compiled_hit in compiled:
        for raw_hit in raw:
            if not _excerpts_disagree(
                str(compiled_hit.get("excerpt") or ""),
                str(raw_hit.get("excerpt") or ""),
            ):
                continue
            key = hashlib.sha256(
                f"{compiled_hit['path']}|{raw_hit['path']}|{query}".encode("utf-8")
            ).hexdigest()[:12]
            relative = f"inbox/{_utc_now()[:10]}-source-conflict-{key}.md"
            destination = ensure_under(wiki_root / relative, repo_root, "inbox")
            existing = next(inbox.glob(f"*-source-conflict-{key}.md"), None)
            if existing is not None:
                relative = existing.relative_to(wiki_root).as_posix()
            else:
                destination.write_text(
                    _conflict_inbox_page(
                        key=key,
                        query=query,
                        compiled_hit=compiled_hit,
                        raw_hit=raw_hit,
                    ),
                    encoding="utf-8",
                )
            conflicts.append(
                {
                    "compiled_path": compiled_hit["path"],
                    "raw_path": raw_hit["path"],
                    "triage_path": relative,
                    "idempotency_key": key,
                    "authoritative": compiled_hit["path"],
                }
            )
    return conflicts


def _excerpts_disagree(compiled: str, raw: str) -> bool:
    compiled_l = compiled.lower()
    raw_l = raw.lower()
    negation = ("do not", "don't", "must not", "never", "not ")
    compiled_neg = any(token in compiled_l for token in negation)
    raw_neg = any(token in raw_l for token in negation)
    return compiled_neg != raw_neg


def _conflict_inbox_page(
    *,
    key: str,
    query: str,
    compiled_hit: Mapping[str, Any],
    raw_hit: Mapping[str, Any],
) -> str:
    today = _utc_now()[:10]
    compiled_path = compiled_hit["path"]
    raw_path = raw_hit["path"]
    return (
        "---\n"
        f"id: source-conflict-{key}\n"
        f"title: Source conflict {compiled_path} vs {raw_path}\n"
        "type: inbox-item\n"
        "status: draft\n"
        f"created: {today}\n"
        f"updated: {today}\n"
        "tags: [inbox, source-conflict]\n"
        "domain: meta\n"
        "summary: Raw inventory disagrees with an active compiled claim.\n"
        "triage_status: pending\n"
        "suggested_action: needs-human\n"
        "suggested_type: synthesis\n"
        "origin: source-conflict\n"
        "nodes: []\n"
        "edges: []\n"
        "related: []\n"
        "agent:\n"
        "  priority: high\n"
        "  read_when:\n"
        "    - resolving raw/compiled disagreements\n"
        "  maintain: []\n"
        "source_conflict:\n"
        f"  idempotency_key: {key}\n"
        f"  query: {json.dumps(query)}\n"
        f"  compiled: {json.dumps(compiled_path)}\n"
        f"  raw: {json.dumps(raw_path)}\n"
        "---\n\n"
        "# Source conflict\n\n"
        "Compiled claim is authoritative. Do not auto-promote the raw source.\n\n"
        f"- Compiled: `{compiled_path}`\n"
        f"  {compiled_hit.get('excerpt', '')[:400]}\n"
        f"- Raw: `{raw_path}`\n"
        f"  {raw_hit.get('excerpt', '')[:400]}\n"
    )


def status_data(
    *,
    repo_root: Path | None = None,
    host: Mapping[str, Any] | Path | None = None,
) -> dict[str, Any]:
    from repobrain_paths import PATHS

    root = repo_root or PATHS.repository
    inventory = SourceInventory(root, host=host or PATHS.host_config)
    manifest_path = PATHS.source_manifest
    manifest = _load_manifest(manifest_path if repo_root is None else root / manifest_path.relative_to(PATHS.repository))
    if repo_root is not None:
        manifest_path = (
            root / "docs/wiki/_system/generated/sources/manifest.json"
        )
        manifest = _load_manifest(manifest_path)
    entries = list(manifest.get("entries") or [])
    conversion_states: dict[str, int] = {}
    for entry in entries:
        state = str((entry.get("conversion") or {}).get("state") or "unknown")
        conversion_states[state] = conversion_states.get(state, 0) + 1
    return {
        "enabled": inventory.config.enabled,
        "manifest": {
            "path": manifest_path.as_posix(),
            "present": manifest_path.exists(),
            "generated_at": manifest.get("generated_at"),
            "entries": len(entries),
        },
        "changes": manifest.get("changes") or {},
        "conversion": conversion_states,
        "converter": markitdown_info(),
        "structures": manifest.get("structures") or {},
        "denied": len(manifest.get("denied") or []),
    }


def cmd_status(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(prog="repobrain source status")
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args(argv or [])
    data = status_data()
    if args.json:
        print(json.dumps(data, indent=2, sort_keys=True))
        return 0
    manifest = data["manifest"]
    state = "present" if manifest["present"] else "not generated"
    print(f"Source manifest: {manifest['path']} ({state})")
    print(f"Entries: {manifest['entries']}")
    print(f"Conversion: {data['conversion'] or '{}'}")
    converter = data["converter"]
    print(
        "MarkItDown: "
        + str(converter.get("version") or "unavailable")
        + f" ({converter.get('diagnostic')})"
    )
    return 0


def cmd_scan(argv: list[str] | None = None) -> int:
    from repobrain_paths import PATHS, load_host

    parser = argparse.ArgumentParser(prog="repobrain source scan")
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--no-pointers", action="store_true")
    parser.add_argument("--convert", action="store_true")
    parser.add_argument("--strict", action="store_true")
    args = parser.parse_args(argv or [])
    host = load_host()
    conversion = (host.get("sources") or {}).get("conversion") or {}
    args.strict = args.strict or bool(conversion.get("strict"))
    converter = live_converter() if args.convert else None
    manifest = scan_sources(
        PATHS.repository,
        host=host,
        previous=PATHS.source_manifest,
        cache_root=PATHS.source_cache_dir,
        converter=converter,
    )
    if args.dry_run:
        print(json.dumps({"entries": len(manifest["entries"]), "changes": manifest["changes"]}, indent=2))
        return 0
    write_manifest(PATHS.source_manifest, manifest, repo_root=PATHS.repository)
    PATHS.source_provenance.parent.mkdir(parents=True, exist_ok=True)
    PATHS.source_provenance.write_text(
        json.dumps(
            {
                "generated_at": manifest["generated_at"],
                "entries": len(manifest["entries"]),
            },
            indent=2,
        )
        + "\n",
        encoding="utf-8",
    )
    if not args.no_pointers:
        template = PATHS.templates / "raw-group-pointer.md"
        write_raw_group_pointers(
            manifest,
            wiki_root=PATHS.corpus,
            template_path=template if template.exists() else template,
        )
    print(
        f"Wrote {PATHS.source_manifest.relative_to(PATHS.repository)} "
        f"({len(manifest['entries'])} entries)"
    )
    failed = [
        entry["path"]
        for entry in manifest["entries"]
        if (entry.get("conversion") or {}).get("state") in {"failed", "blocked"}
    ]
    if failed:
        print("Conversion failures: " + ", ".join(failed))
        if args.strict:
            return 1
    return 0


def cmd_convert(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(prog="repobrain source convert")
    parser.add_argument("--path")
    parser.add_argument("--force", action="store_true")
    parser.add_argument("--strict", action="store_true")
    args = parser.parse_args(argv or [])
    from repobrain_paths import PATHS, load_host

    host = load_host()
    conversion = (host.get("sources") or {}).get("conversion") or {}
    args.strict = args.strict or bool(conversion.get("strict"))
    converter = live_converter()
    if converter is None:
        info = markitdown_info()
        print(info["diagnostic"], file=sys.stderr)
        print(info["install_command"], file=sys.stderr)
        return 2

    if args.force and PATHS.source_cache_dir.exists():
        for cached in PATHS.source_cache_dir.glob("*.md"):
            cached.unlink()
    previous = PATHS.source_manifest if PATHS.source_manifest.exists() else None
    manifest = scan_sources(
        PATHS.repository,
        host=host,
        previous=previous,
        cache_root=PATHS.source_cache_dir,
        converter=converter,
    )
    write_manifest(PATHS.source_manifest, manifest, repo_root=PATHS.repository)
    converted = [
        entry["path"]
        for entry in manifest["entries"]
        if (entry.get("conversion") or {}).get("state") in {"cached", "converted"}
        and (args.path is None or entry["path"] == args.path)
    ]
    failures = [
        entry["path"]
        for entry in manifest["entries"]
        if (entry.get("conversion") or {}).get("state") in {"failed", "blocked"}
        and (args.path is None or entry["path"] == args.path)
    ]
    print(f"Converted or cached: {len(converted)}")
    if failures:
        print("Failed: " + ", ".join(failures))
        if args.strict:
            return 1
    return 0
