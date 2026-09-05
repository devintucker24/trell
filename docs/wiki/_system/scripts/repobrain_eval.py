#!/usr/bin/env python3
"""End-to-end baseline evaluation for the RepoBrain knowledge layer."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import sys
import tempfile
from dataclasses import asdict, dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import yaml

sys.path.insert(0, str(Path(__file__).resolve().parent))
from repobrain_paths import PATHS, ROOT, RepoBrainPaths, load_host
from wiki_usage import STRONG_HIT, WEAK_HIT

DEFAULT_CONFIG = PATHS.eval_config
DEFAULT_OUTPUT = PATHS.eval_dir
SCORE_FLOORS = {"weak": 0.08, "relevant": WEAK_HIT, "strong": STRONG_HIT}


@dataclass
class CategoryResult:
    name: str
    required: bool
    passed: bool
    summary: str
    evidence: list[dict[str, Any]] = field(default_factory=list)
    remediation: list[str] = field(default_factory=list)


def estimate_tokens(text: str) -> int:
    """Use the same deterministic estimate as wiki_retrieve.py."""
    return max(1, len(text) // 4)


def run(
    args: list[str],
    *,
    cwd: Path = ROOT,
    check: bool = False,
) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        args,
        cwd=cwd,
        check=check,
        capture_output=True,
        text=True,
    )


def display_text(text: str, root: Path) -> str:
    return text.replace(str(root), ".")


def category_enabled(only: set[str], name: str) -> bool:
    return not only or name in only


def paths_for(root: Path) -> RepoBrainPaths:
    return RepoBrainPaths.discover(
        root / "docs" / "wiki" / "_system" / "scripts" / "repobrain_paths.py"
    )


def evaluate_doctor(root: Path) -> CategoryResult:
    paths = paths_for(root)
    script = paths.scripts / "wiki_doctor.py"
    proc = run([sys.executable, str(script), "--no-log"], cwd=root)
    report_path = paths.doctor_latest
    evidence: list[dict[str, Any]] = [
        {
            "command": "./repobrain doctor --no-log",
            "exit_code": proc.returncode,
            "stdout": display_text(proc.stdout, root).strip(),
            "stderr": display_text(proc.stderr, root).strip(),
        }
    ]
    if proc.returncode != 0 or not report_path.exists():
        return CategoryResult(
            "structural-health",
            True,
            False,
            "Wiki doctor did not produce a readable report.",
            evidence,
            [
                "Run `./repobrain doctor` "
                "and fix the reported error."
            ],
        )

    report = json.loads(report_path.read_text(encoding="utf-8"))
    counts = report.get("counts") or {}
    blocking = int(counts.get("critical", 0)) + int(counts.get("high", 0))
    evidence.append(
        {
            "score": report.get("score"),
            "counts": counts,
            "pages_scanned": report.get("pages_scanned"),
            "blocking_findings": [
                finding
                for finding in report.get("findings") or []
                if finding.get("severity") in {"critical", "high"}
            ],
        }
    )
    return CategoryResult(
        "structural-health",
        True,
        blocking == 0,
        (
            f"Doctor score {report.get('score')}/100 with {blocking} blocking finding(s)."
        ),
        evidence,
        (
            []
            if blocking == 0
            else [
                "Run `./repobrain doctor`, then apply each "
                "critical/high finding before rerunning `./repobrain eval`."
            ]
        ),
    )


def retrieve_case(
    root: Path,
    case: dict[str, Any],
    defaults: dict[str, Any],
) -> tuple[dict[str, Any] | None, dict[str, Any]]:
    top_k = int(case.get("top_k", defaults.get("top_k", 8)))
    budget = int(case.get("budget_tokens", defaults.get("retrieved_tokens", 3500)))
    args = [
        sys.executable,
        str(paths_for(root).scripts / "wiki_retrieve.py"),
        str(case["q"]),
        "--k",
        str(top_k),
        "--budget-tokens",
        str(budget),
        "--lane",
        str(case.get("lane", "all")),
        "--json",
        "--no-log",
    ]
    proc = run(args, cwd=root)
    command = (
        f'./repobrain retrieve "{case["q"]}" '
        f"--k {top_k} --budget-tokens {budget} --lane {case.get('lane', 'all')} "
        "--json --no-log"
    )
    if proc.returncode != 0:
        return None, {
            "id": case["id"],
            "passed": False,
            "command": command,
            "exit_code": proc.returncode,
            "stderr": proc.stderr.strip(),
            "failures": ["retrieval command failed"],
        }
    try:
        payload = json.loads(proc.stdout)
    except json.JSONDecodeError as exc:
        return None, {
            "id": case["id"],
            "passed": False,
            "command": command,
            "exit_code": proc.returncode,
            "stderr": str(exc),
            "failures": ["retrieval output was not valid JSON"],
        }

    hits = payload.get("hits") or []
    ranked: dict[str, tuple[int, dict[str, Any]]] = {}
    for rank, hit in enumerate(hits, 1):
        ranked.setdefault(str(hit.get("path")), (rank, hit))

    expected = case.get("expect_sources")
    if expected is None:
        expected = [{"path": path} for path in case.get("expect_paths") or []]
    floor_name = str(case.get("minimum_score_class", defaults.get("minimum_score_class", "relevant")))
    floor = SCORE_FLOORS.get(floor_name)
    failures: list[str] = []
    checks: list[dict[str, Any]] = []
    if floor is None:
        failures.append(f"unknown score class {floor_name!r}")
        floor = 1.0

    for source in expected:
        source = {"path": source} if isinstance(source, str) else dict(source)
        path = str(source["path"])
        max_rank = int(source.get("max_rank", top_k))
        expected_provenance = str(
            source.get("provenance", case.get("provenance", "compiled"))
        )
        found = ranked.get(path)
        check: dict[str, Any] = {
            "path": path,
            "max_rank": max_rank,
            "minimum_score_class": floor_name,
            "minimum_score": floor,
            "expected_provenance": expected_provenance,
            "found": found is not None,
        }
        if found is None:
            failures.append(f"{path} was absent from the top {top_k}")
        else:
            rank, hit = found
            score = float(hit.get("score", 0.0))
            provenance = str((hit.get("provenance") or {}).get("kind", "unknown"))
            check.update(
                {"rank": rank, "score": score, "provenance": provenance}
            )
            if rank > max_rank:
                failures.append(f"{path} ranked {rank}, below required top {max_rank}")
            if score < floor:
                failures.append(
                    f"{path} score {score:.4f} is below {floor_name} ({floor:.2f})"
                )
            if provenance != expected_provenance:
                failures.append(
                    f"{path} provenance {provenance!r} != {expected_provenance!r}"
                )
        checks.append(check)

    packed_tokens = int(payload.get("packed_tokens", 0))
    reported_budget = int(payload.get("budget_tokens", budget))
    if packed_tokens > budget or reported_budget != budget:
        failures.append(
            f"packed token accounting {packed_tokens}/{reported_budget} "
            f"did not honor configured budget {budget}"
        )

    evidence = {
        "id": case["id"],
        "passed": not failures,
        "command": command,
        "top_k": top_k,
        "packed_tokens": packed_tokens,
        "budget_tokens": budget,
        "source_checks": checks,
        "ranked_paths": [
            {
                "rank": rank,
                "path": hit.get("path"),
                "score": hit.get("score"),
                "provenance": (hit.get("provenance") or {}).get("kind"),
            }
            for rank, hit in enumerate(hits, 1)
        ],
        "failures": failures,
    }
    return payload, evidence


def evaluate_retrieval(
    root: Path,
    config: dict[str, Any],
) -> tuple[CategoryResult, dict[str, dict[str, Any]]]:
    defaults = config.get("budgets") or {}
    payloads: dict[str, dict[str, Any]] = {}
    evidence: list[dict[str, Any]] = []
    for case in config.get("queries") or []:
        payload, item = retrieve_case(root, case, defaults)
        evidence.append(item)
        if payload is not None:
            payloads[str(case["id"])] = payload
    failed = [item for item in evidence if not item.get("passed")]
    return (
        CategoryResult(
            "golden-retrieval",
            True,
            not failed,
            f"{len(evidence) - len(failed)}/{len(evidence)} golden queries passed.",
            evidence,
            (
                []
                if not failed
                else [
                    "Inspect each failed query's ranked paths, then repair the "
                    "corpus metadata/ranking or intentionally update the reviewed golden set."
                ]
            ),
        ),
        payloads,
    )


def _term_present(text: str, requirement: Any) -> bool:
    alternatives = requirement if isinstance(requirement, list) else [requirement]
    return any(str(term).lower() in text.lower() for term in alternatives)


def evaluate_answer_fidelity(
    config: dict[str, Any],
    payloads: dict[str, dict[str, Any]],
) -> CategoryResult:
    evidence: list[dict[str, Any]] = []
    for case in config.get("answer_fidelity") or []:
        query_id = str(case["query_id"])
        payload = payloads.get(query_id)
        failures: list[str] = []
        if payload is None:
            evidence.append(
                {
                    "id": case["id"],
                    "passed": False,
                    "query_id": query_id,
                    "failures": ["the corresponding retrieval did not produce JSON"],
                }
            )
            continue

        forbidden = set(case.get("forbid_paths") or ["INDEX.md"])
        max_excerpt = int(case.get("max_excerpt_chars", 500))
        allowed = [
            hit
            for hit in payload.get("hits") or []
            if hit.get("path") not in forbidden
            and (hit.get("provenance") or {}).get("kind")
            in set(case.get("allow_provenance") or ["compiled"])
        ]
        excerpt_text = "\n".join(str(hit.get("excerpt") or "") for hit in allowed)
        term_checks = []
        for requirement in case.get("required_terms") or []:
            present = _term_present(excerpt_text, requirement)
            term_checks.append({"requirement": requirement, "present": present})
            if not present:
                failures.append(f"retrieved excerpts lack required evidence {requirement!r}")

        citation_checks = []
        allowed_paths = {str(hit.get("path")) for hit in allowed}
        for path in case.get("required_citations") or []:
            present = path in allowed_paths
            citation_checks.append({"path": path, "present": present})
            if not present:
                failures.append(f"retrieval-only context lacks required citation {path}")

        oversized = [
            str(hit.get("path"))
            for hit in allowed
            if len(str(hit.get("excerpt") or "")) > max_excerpt
        ]
        if oversized:
            failures.append(f"unbounded excerpts supplied by {oversized}")

        context_manifest = [
            {
                "kind": "tier-0",
                "path": path,
            }
            for path in (config.get("tier0") or {}).get(
                "paths", ["AGENTS.md", "docs/wiki/_system/docs/ROUTER.md"]
            )
        ] + [
            {
                "kind": "retrieved-excerpt",
                "path": hit.get("path"),
                "anchor": hit.get("anchor"),
                "characters": len(str(hit.get("excerpt") or "")),
            }
            for hit in allowed
        ]
        dumped = [
            entry
            for entry in context_manifest
            if entry["kind"] != "tier-0" and entry["path"] in forbidden
        ]
        if dumped:
            failures.append("forbidden full-index/corpus context entered the answer pack")

        evidence.append(
            {
                "id": case["id"],
                "query_id": query_id,
                "passed": not failures,
                "mode": "deterministic retrieval-only evidence",
                "context_manifest": context_manifest,
                "required_term_checks": term_checks,
                "required_citation_checks": citation_checks,
                "forbidden_paths": sorted(forbidden),
                "full_corpus_reads": 0,
                "failures": failures,
            }
        )

    failed = [item for item in evidence if not item.get("passed")]
    return CategoryResult(
        "answer-fidelity",
        True,
        bool(evidence) and not failed,
        (
            f"{len(evidence) - len(failed)}/{len(evidence)} retrieval-only "
            "evidence checks passed."
        ),
        evidence,
        (
            []
            if evidence and not failed
            else [
                "Ensure bounded retrieved excerpts contain every required fact/citation; "
                "do not add INDEX.md or whole-corpus reads to the answer context."
            ]
        ),
    )


def evaluate_budgets(
    root: Path,
    config: dict[str, Any],
    payloads: dict[str, dict[str, Any]],
) -> CategoryResult:
    tier0 = config.get("tier0") or {}
    paths = tier0.get("paths") or [
        "AGENTS.md",
        "docs/wiki/_system/docs/ROUTER.md",
    ]
    path_costs = []
    missing = []
    for rel in paths:
        path = root / rel
        if not path.exists():
            missing.append(rel)
            continue
        path_costs.append(
            {"path": rel, "estimated_tokens": estimate_tokens(path.read_text(encoding="utf-8"))}
        )
    tier0_tokens = sum(item["estimated_tokens"] for item in path_costs)
    budgets = config.get("budgets") or {}
    tier0_budget = int(budgets.get("tier0_tokens", 2000))
    retrieved_budget = int(budgets.get("retrieved_tokens", 3500))
    total_budget = int(budgets.get("total_context_tokens", 9500))
    retrieval_costs = {
        query_id: int(payload.get("packed_tokens", 0))
        for query_id, payload in payloads.items()
    }
    max_retrieved = max(retrieval_costs.values(), default=0)
    failures = []
    if missing:
        failures.append(f"Tier-0 files missing: {missing}")
    if tier0_tokens > tier0_budget:
        failures.append(f"Tier-0 estimate {tier0_tokens} exceeds {tier0_budget}")
    if max_retrieved > retrieved_budget:
        failures.append(
            f"maximum retrieved context {max_retrieved} exceeds {retrieved_budget}"
        )
    if tier0_tokens + max_retrieved > total_budget:
        failures.append(
            f"combined Tier-0 + retrieved estimate {tier0_tokens + max_retrieved} "
            f"exceeds {total_budget}"
        )
    evidence = [
        {
            "estimator": "characters // 4 (same as wiki_retrieve.py)",
            "tier0": {
                "paths": path_costs,
                "estimated_tokens": tier0_tokens,
                "budget_tokens": tier0_budget,
            },
            "retrieval": {
                "per_query_tokens": retrieval_costs,
                "maximum_tokens": max_retrieved,
                "budget_tokens": retrieved_budget,
            },
            "combined": {
                "maximum_tokens": tier0_tokens + max_retrieved,
                "budget_tokens": total_budget,
            },
            "failures": failures,
        }
    ]
    return CategoryResult(
        "context-budgets",
        True,
        not failures,
        (
            f"Tier-0 {tier0_tokens}/{tier0_budget}; max retrieved "
            f"{max_retrieved}/{retrieved_budget}; combined "
            f"{tier0_tokens + max_retrieved}/{total_budget} estimated tokens."
        ),
        evidence,
        (
            []
            if not failures
            else [
                "Reduce Tier-0 text or retrieval packing, or review and explicitly "
                "change the documented budget before rerunning the evaluation."
            ]
        ),
    )


def _git_output(root: Path, args: list[str]) -> str:
    proc = run(["git", *args], cwd=root)
    return proc.stdout.strip() if proc.returncode == 0 else ""


def evaluate_graphify(root: Path, config: dict[str, Any]) -> CategoryResult:
    paths = paths_for(root)
    graph_case = config.get("graphify") or {}
    symbol = str(graph_case.get("symbol", "Parser"))
    expected_source = str(graph_case.get("source_file", "parser.rs"))
    wrapper = paths.scripts / "wiki_graphify.py"
    status = run([sys.executable, str(wrapper), "status", "--json"], cwd=root)
    query = run(
        [sys.executable, str(wrapper), "query", symbol, "--budget", "800"],
        cwd=root,
    )
    host = load_host(paths)
    graph_cfg = host.get("graphify") or {}
    graph_rel = Path(graph_cfg.get("out") or "graphify-out") / "graph.json"
    graph_path = root / graph_rel
    failures: list[str] = []
    adapter_status: dict[str, Any] = {}
    try:
        adapter_status = json.loads(status.stdout)
    except (json.JSONDecodeError, TypeError):
        failures.append("Graphify adapter status did not return valid JSON")
    graph: dict[str, Any] = {}
    if not graph_path.exists():
        failures.append(f"missing {graph_rel.as_posix()}")
    else:
        try:
            graph = json.loads(graph_path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as exc:
            failures.append(f"invalid Graphify JSON: {exc}")

    nodes = graph.get("nodes") or []
    matches = [
        node
        for node in nodes
        if str(node.get("label", "")).casefold() == symbol.casefold()
    ]
    source_match = next(
        (
            node
            for node in matches
            if Path(str(node.get("source_file", ""))).name == Path(expected_source).name
        ),
        None,
    )
    if source_match is None:
        failures.append(f"symbol {symbol!r} did not resolve to {expected_source}")
    actual_source = root / "src" / expected_source
    if not actual_source.exists():
        failures.append(f"resolved source does not exist: src/{expected_source}")
    if query.returncode != 0:
        failures.append("Graphify query command failed")
    elif symbol not in query.stdout or expected_source not in query.stdout:
        failures.append("Graphify query output omitted the symbol or source file")
    if status.returncode != 0:
        failures.append("Graphify status is not healthy")

    freshness = adapter_status.get("freshness") or {}
    fresh = freshness.get("source") == "fresh"
    if not fresh:
        failures.append("Graphify graph is stale relative to configured code targets")

    evidence = [
        {
            "status_command": "./repobrain graph status",
            "status_exit_code": status.returncode,
            "status": display_text(status.stdout or status.stderr, root).strip(),
            "query_command": (
                "./repobrain graph "
                f"query {symbol} --budget 800"
            ),
            "query_exit_code": query.returncode,
            "query_excerpt": display_text(
                (query.stdout or query.stderr)[:1200], root
            ).strip(),
            "symbol": symbol,
            "expected_source": expected_source,
            "matched_node": source_match,
            "source_exists": actual_source.exists(),
            "nodes": len(nodes),
            "edges": len(graph.get("edges") or graph.get("links") or []),
            "adapter_status": adapter_status,
            "freshness": freshness,
            "failures": failures,
        }
    ]
    return CategoryResult(
        "graphify",
        True,
        not failures,
        (
            f"Graphify resolved {symbol} to {expected_source}; "
            f"freshness={'fresh' if fresh else 'stale'}."
        ),
        evidence,
        (
            []
            if not failures
            else [
                "Install Graphify with "
                "`python3 -m pip install --user 'graphifyy>=0.9.54,<0.10'`, "
                "run `./repobrain graph sync`, "
                "then rerun eval."
            ]
        ),
    )


def _write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def _sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def evaluate_setup_fixture(root: Path) -> CategoryResult:
    with tempfile.TemporaryDirectory(prefix="repobrain-eval-") as tmp:
        fixture = Path(tmp) / "busy-docs-repo"
        fixture.mkdir()
        export = run(
            [
                sys.executable,
                str(paths_for(root).scripts / "wiki_pack.py"),
                "export",
                str(fixture),
            ],
            cwd=root,
        )
        _write(
            fixture / ".gitignore",
            "ignored-notes.md\n.env\nbuild/\nnode_modules/\n",
        )
        _write(fixture / "README.md", "# Busy docs repository\n")
        _write(fixture / "AGENTS.md", "# Agent context\nKeep existing instructions.\n")
        _write(fixture / "CONTEXT.md", "# Domain context\nRaw project context.\n")
        _write(
            fixture / "docs" / "adr" / "0001-use-events.md",
            "# ADR 0001\nUse domain events.\n",
        )
        _write(fixture / "mkdocs.yml", "site_name: Busy Docs\n")
        _write(
            fixture / "docs" / "site" / "index.md",
            "# Existing documentation site\nDo not copy this into semantic claims.\n",
        )
        _write(fixture / "src" / "service.py", "def service():\n    return 'ok'\n")
        _write(fixture / "ignored-notes.md", "must remain ignored\n")
        _write(fixture / ".env", "EVALUATION_PLACEHOLDER=not-a-secret\n")
        _write(fixture / "build" / "generated.md", "ignored build output\n")
        binary = fixture / "assets" / "architecture.png"
        binary.parent.mkdir(parents=True, exist_ok=True)
        binary.write_bytes(b"\x89PNG\r\n\x1a\n" + bytes(range(64)))
        large = fixture / "data" / "large-reference.txt"
        _write(large, "representative large source\n" * 12000)
        existing = fixture / "docs" / "wiki" / "core" / "existing-knowledge.md"
        _write(
            existing,
            "---\nid: existing-knowledge\ntitle: Existing knowledge\ntype: concept\n"
            "status: active\ncreated: 2026-09-05\nupdated: 2026-09-05\n"
            "tags: [existing]\ndomain: core\nsummary: Existing reviewed claim.\n"
            "nodes: []\nedges: []\nrelated: []\nagent:\n  priority: high\n"
            "  read_when: [existing]\n  maintain: []\n---\n\n# Existing knowledge\n"
            "This content must not be overwritten.\n",
        )

        run(["git", "init", "-q"], cwd=fixture)
        run(["git", "config", "user.email", "eval@example.invalid"], cwd=fixture)
        run(["git", "config", "user.name", "RepoBrain Eval"], cwd=fixture)
        run(["git", "add", "."], cwd=fixture)
        run(["git", "commit", "-qm", "fixture baseline"], cwd=fixture)

        tracked = set(_git_output(fixture, ["ls-files"]).splitlines())
        ignored = set(
            line
            for line in _git_output(
                fixture,
                [
                    "ls-files",
                    "--others",
                    "--ignored",
                    "--exclude-standard",
                ],
            ).splitlines()
            if line
        )
        semantic_dir = fixture / "docs" / "wiki" / "core"
        before_semantic = {
            path.relative_to(fixture).as_posix(): _sha256(path)
            for path in semantic_dir.glob("*.md")
        }
        protected = {
            rel: _sha256(fixture / rel)
            for rel in [
                "CONTEXT.md",
                "docs/adr/0001-use-events.md",
                "docs/site/index.md",
                "docs/wiki/core/existing-knowledge.md",
            ]
        }
        agents_before = (fixture / "AGENTS.md").read_text(encoding="utf-8")
        setup = run(
            [
                sys.executable,
                str(paths_for(fixture).scripts / "wiki_setup.py"),
                "--no-graphify",
            ],
            cwd=fixture,
        )
        after_semantic = {
            path.relative_to(fixture).as_posix(): _sha256(path)
            for path in semantic_dir.glob("*.md")
        }
        protected_after = {rel: _sha256(fixture / rel) for rel in protected}
        agents_after = (fixture / "AGENTS.md").read_text(encoding="utf-8")

        fixture_checks = {
            "git_repository": (fixture / ".git").is_dir(),
            "tracked_source": "src/service.py" in tracked,
            "ignored_source": {
                "ignored-notes.md",
                ".env",
                "build/generated.md",
            }.issubset(ignored),
            "adr": "docs/adr/0001-use-events.md" in tracked,
            "context_doc": "CONTEXT.md" in tracked,
            "docs_site_marker": "mkdocs.yml" in tracked,
            "docs_site_page": "docs/site/index.md" in tracked,
            "existing_corpus": "docs/wiki/core/existing-knowledge.md" in tracked,
            "binary_source": "assets/architecture.png" in tracked,
            "large_source": (
                "data/large-reference.txt" in tracked
                and large.stat().st_size >= 250_000
            ),
        }
        safety_checks = {
            "setup_exit_zero": setup.returncode == 0,
            "existing_content_unchanged": protected == protected_after,
            "existing_agent_instructions_preserved": agents_before in agents_after,
            "semantic_file_set_unchanged": before_semantic == after_semantic,
            "raw_docs_not_copied_to_semantic": set(after_semantic) == set(before_semantic),
        }
        failures = [
            f"fixture missing required feature: {name}"
            for name, passed in fixture_checks.items()
            if not passed
        ] + [
            f"setup safety invariant failed: {name}"
            for name, passed in safety_checks.items()
            if not passed
        ]
        detected_line = next(
            (
                line
                for line in setup.stdout.splitlines()
                if line.startswith("detected name=")
            ),
            "",
        )
        evidence = [
            {
                "fixture": "temporary Git repository (deleted after evaluation)",
                "fixture_checks": fixture_checks,
                "tracked_file_count": len(tracked),
                "ignored_file_count": len(ignored),
                "setup_command": (
                    "./repobrain setup --no-graphify"
                ),
                "setup_exit_code": setup.returncode,
                "setup_detection": detected_line,
                "setup_output": display_text(
                    (setup.stdout or setup.stderr)[-1600:], fixture
                ).strip(),
                "safety_checks": safety_checks,
                "baseline_observation": (
                    "Current setup detection reports only its existing narrow raw-source "
                    "set; later source-inventory tickets must add ADR/docs-site discovery."
                ),
                "failures": failures,
            }
        ]
    return CategoryResult(
        "setup-fixture",
        True,
        not failures,
        (
            "Realistic Git fixture passed setup non-overwrite and "
            "no-semantic-copy safety checks."
        ),
        evidence,
        (
            []
            if not failures
            else [
                "Make wiki_setup.py idempotent and preserve existing raw docs and "
                "semantic corpus content, then rerun the fixture evaluation."
            ]
        ),
    )


def write_reports(
    root: Path,
    output_dir: Path,
    config_path: Path,
    categories: list[CategoryResult],
) -> tuple[Path, Path, dict[str, Any]]:
    now = datetime.now(timezone.utc)
    stamp = now.strftime("%Y%m%dT%H%M%SZ")
    output_dir.mkdir(parents=True, exist_ok=True)
    required_failures = [
        result.name for result in categories if result.required and not result.passed
    ]
    try:
        displayed_config = config_path.relative_to(root).as_posix()
    except ValueError:
        displayed_config = str(config_path)
    report = {
        "schema_version": 1,
        "generated_at": now.isoformat(),
        "repository": load_host(paths_for(root)).get("name") or root.name,
        "commit": _git_output(root, ["rev-parse", "HEAD"]) or None,
        "config": displayed_config,
        "status": "pass" if not required_failures else "fail",
        "exit_code": 0 if not required_failures else 1,
        "required_failures": required_failures,
        "categories": [asdict(result) for result in categories],
    }
    json_path = output_dir / f"repobrain-eval-{stamp}.json"
    md_path = output_dir / f"repobrain-eval-{stamp}.md"
    json_path.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")

    lines = [
        f"# RepoBrain baseline evaluation — {now.strftime('%Y-%m-%d %H:%M:%S UTC')}",
        "",
        f"**Overall:** {'PASS' if report['status'] == 'pass' else 'FAIL'}",
        "",
        f"Commit: `{report['commit'] or 'unknown'}`",
        "",
        "| Category | Required | Result | Summary |",
        "|---|---:|---:|---|",
    ]
    for result in categories:
        summary = result.summary.replace("|", "\\|")
        lines.append(
            f"| `{result.name}` | {'yes' if result.required else 'no'} | "
            f"{'PASS' if result.passed else 'FAIL'} | {summary} |"
        )
    for result in categories:
        lines.extend(["", f"## {result.name}", "", result.summary, "", "### Evidence", ""])
        lines.append("```json")
        lines.append(json.dumps(result.evidence, indent=2))
        lines.append("```")
        if result.remediation:
            lines.extend(["", "### Remediation", ""])
            lines.extend(f"- {item}" for item in result.remediation)
    md_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return md_path, json_path, report


def evaluate(
    *,
    root: Path = ROOT,
    config_path: Path = DEFAULT_CONFIG,
    output_dir: Path = DEFAULT_OUTPUT,
    only: set[str] | None = None,
) -> tuple[dict[str, Any], Path, Path]:
    config = yaml.safe_load(config_path.read_text(encoding="utf-8")) or {}
    only = only or set()
    categories: list[CategoryResult] = []
    payloads: dict[str, dict[str, Any]] = {}

    if category_enabled(only, "structural-health"):
        categories.append(evaluate_doctor(root))
    if category_enabled(only, "golden-retrieval") or category_enabled(
        only, "answer-fidelity"
    ) or category_enabled(only, "context-budgets"):
        retrieval, payloads = evaluate_retrieval(root, config)
        if category_enabled(only, "golden-retrieval"):
            categories.append(retrieval)
    if category_enabled(only, "answer-fidelity"):
        categories.append(evaluate_answer_fidelity(config, payloads))
    if category_enabled(only, "graphify"):
        categories.append(evaluate_graphify(root, config))
    if category_enabled(only, "context-budgets"):
        categories.append(evaluate_budgets(root, config, payloads))
    if category_enabled(only, "setup-fixture"):
        categories.append(evaluate_setup_fixture(root))

    md_path, json_path, report = write_reports(
        root, output_dir, config_path, categories
    )
    return report, md_path, json_path


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description="Run the RepoBrain end-to-end baseline evaluation"
    )
    parser.add_argument("--config", type=Path, default=DEFAULT_CONFIG)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument(
        "--only",
        action="append",
        choices=[
            "structural-health",
            "golden-retrieval",
            "answer-fidelity",
            "graphify",
            "context-budgets",
            "setup-fixture",
        ],
        default=[],
        help="run only this category (repeatable)",
    )
    args = parser.parse_args(argv)
    try:
        report, md_path, json_path = evaluate(
            config_path=args.config.resolve(),
            output_dir=args.output_dir.resolve(),
            only=set(args.only),
        )
    except (OSError, ValueError, yaml.YAMLError) as exc:
        print(f"RepoBrain eval configuration/error: {exc}", file=sys.stderr)
        return 2

    print(f"RepoBrain baseline: {report['status'].upper()}")
    for category in report["categories"]:
        print(
            f"  [{'PASS' if category['passed'] else 'FAIL'}] "
            f"{category['name']}: {category['summary']}"
        )
        for remediation in category["remediation"]:
            print(f"    remediation: {remediation}")
    print(f"Markdown report: {md_path}")
    print(f"JSON report: {json_path}")
    return int(report["exit_code"])


if __name__ == "__main__":
    raise SystemExit(main())
