#!/usr/bin/env python3
"""Wiki Doctor — read-only diagnosis of the Trell wiki brain.

Writes under `docs/wiki/_system/generated/doctor/`.
"""

from __future__ import annotations

import json
import re
import sys
from collections import Counter, defaultdict
from datetime import date
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).resolve().parent))
from repobrain_paths import PATHS, ROOT, WIKI, is_wiki_content_page, load_host
from wiki_usage import log_event, write_dashboard, compute_stats, load_events

TODAY = date.today().isoformat()

REQUIRED_FIELDS = [
    "id", "title", "type", "status", "created", "updated",
    "tags", "domain", "summary", "nodes", "edges", "related", "agent",
]
DEFAULT_TYPES = {
    "index", "concept", "application", "market", "roadmap", "schema",
    "meta", "synthesis", "raw-pointer", "inbox-item", "episode",
}
DEFAULT_DOMAINS = {
    "meta", "episodic", "temporal",
}
ALLOWED_RELS = {
    "depends_on", "implements", "contradicts", "extends", "applies_to",
    "competes_with", "enforces", "reduces_via", "accelerates", "regulated_by",
    "owned_by", "milestone_of", "related_to",
}
LINK_RE = re.compile(r"\[\[([^\]|#]+)(?:#[^\]|]+)?(?:\|[^\]]+)?\]\]")


def parse_fm(text: str):
    if not text.startswith("---\n"):
        return None
    end = text.find("\n---\n", 4)
    if end == -1:
        return None
    try:
        return yaml.safe_load(text[4:end])
    except Exception as e:  # noqa: BLE001
        return {"_error": str(e)}


def add(findings, severity, code, message, path=None, fix=None):
    findings.append({
        "severity": severity,
        "code": code,
        "message": message,
        "path": path,
        "fix": fix,
    })


def main() -> None:
    no_log = "--no-log" in sys.argv
    findings: list[dict] = []
    host = load_host()
    allowed_types = set(DEFAULT_TYPES) | set(host.get("types_extra") or [])
    allowed_domains = set(DEFAULT_DOMAINS) | set(host.get("domains") or [])
    md_files = sorted(
        p for p in WIKI.rglob("*.md")
        if is_wiki_content_page(p.relative_to(WIKI).as_posix(), p.name)
    )

    metas: dict[str, dict] = {}
    for p in md_files:
        rel = p.relative_to(WIKI).as_posix()
        # skip template placeholders with YYYY
        if p.name == "_TEMPLATE.md":
            continue
        meta = parse_fm(p.read_text(encoding="utf-8"))
        if meta is None:
            add(findings, "high", "missing_frontmatter", "Page lacks YAML frontmatter", rel,
                "Run label skill / add SCHEMA frontmatter")
            continue
        if "_error" in meta:
            add(findings, "critical", "invalid_yaml", f"YAML parse error: {meta['_error']}", rel,
                "Fix frontmatter YAML")
            continue
        metas[rel] = meta
        for f in REQUIRED_FIELDS:
            if f not in meta and meta.get("type") != "inbox-item":
                # inbox-item has extra fields; still needs core set
                if f in REQUIRED_FIELDS:
                    add(findings, "high", "schema_field", f"Missing field `{f}`", rel, "label skill")
        if meta.get("type") and meta["type"] not in allowed_types:
            add(findings, "critical", "bad_type", f"Unknown type `{meta['type']}`", rel, "SCHEMA update or HOST.yaml types_extra")
        if meta.get("domain") and meta["domain"] not in allowed_domains:
            add(findings, "critical", "bad_domain", f"Unknown domain `{meta['domain']}`", rel, "SCHEMA gate or HOST.yaml domains")
        for e in meta.get("edges") or []:
            if e.get("rel") not in ALLOWED_RELS:
                add(findings, "high", "bad_rel", f"Unknown rel `{e.get('rel')}`", rel, "Use SCHEMA rel vocabulary")
        agent = meta.get("agent") or {}
        for f in ("priority", "read_when", "maintain"):
            if f not in agent and meta.get("type") != "inbox-item":
                add(findings, "medium", "agent_block", f"agent missing `{f}`", rel, "label skill")

    # Graph
    graph_path = PATHS.claim_graph
    if not graph_path.exists():
        add(
            findings,
            "critical",
            "no_graph",
            "Generated claim graph missing",
            str(graph_path.relative_to(ROOT)),
            "python3 docs/wiki/_system/scripts/sync_graph.py",
        )
        graph = {"nodes": [], "edges": []}
    else:
        graph = yaml.safe_load(graph_path.read_text(encoding="utf-8"))

    node_ids = {n["id"] for n in graph.get("nodes") or []}
    inbound = defaultdict(int)
    outbound = defaultdict(int)
    for e in graph.get("edges") or []:
        outbound[e["from"]] += 1
        inbound[e["to"]] += 1
        if e["from"] not in node_ids or e["to"] not in node_ids:
            add(findings, "critical", "broken_edge",
                f"Edge {e['from']} -[{e['rel']}]-> {e['to']} has missing endpoint",
                e.get("page"), "Add node or remove edge; sync_graph.py")

    hard_orphans = sorted(n for n in node_ids if inbound[n] == 0 and outbound[n] == 0)
    for n in hard_orphans:
        add(findings, "medium", "hard_orphan_node",
            f"Node `{n}` has no edges", str(graph_path.relative_to(ROOT)),
            "Link from hub via applies_to/related_to on owning page")

    # Wikilinks (skip generated reports — they quote prior findings)
    corpus_stems = {
        p.relative_to(WIKI).as_posix()[:-3]
        for p in WIKI.rglob("*.md")
        if is_wiki_content_page(p.relative_to(WIKI).as_posix(), p.name)
    }
    reference_stems = {
        p.relative_to(WIKI).as_posix()[:-3]
        for p in WIKI.rglob("*.md")
    }
    all_stems = corpus_stems | reference_stems
    aliases = set(all_stems) | {s.split("/")[-1] for s in all_stems} | {
        "INDEX", "SCHEMA", "log", "ROUTER", "OPERATOR", "FRAMEWORK", "inbox/README",
        "episodic/INDEX",
        "temporal/TIMELINE",
        "_system/docs/usage-telemetry",
        "_system/generated/usage/dashboard",
        "ROUTER",
        "FRAMEWORK",
        "SCHEMA",
        "OPERATOR",
    }
    skip_link_scan = ("_meta/doctor-", "_meta/heal-", "_meta/health-", "_meta/sim-")
    for p in md_files:
        if p.name == "_TEMPLATE.md":
            continue
        rel = p.relative_to(WIKI).as_posix()
        if any(rel.startswith(s) for s in skip_link_scan):
            continue
        for m in LINK_RE.finditer(p.read_text(encoding="utf-8")):
            target = m.group(1).strip()
            if target.startswith("examples/") or target in ("path/page", "folder/page-name", "path/page-name"):
                continue
            ok = target in aliases or any(s == target or s.endswith("/" + target) for s in all_stems)
            if not ok:
                add(findings, "high", "broken_wikilink", f"Broken wikilink [[{target}]]", rel,
                    "Fix target path or add page")

    # Inbox pending
    inbox_dir = WIKI / "inbox"
    if inbox_dir.exists():
        for p in inbox_dir.glob("*.md"):
            if p.name in ("README.md", "_TEMPLATE.md"):
                continue
            meta = parse_fm(p.read_text(encoding="utf-8")) or {}
            if meta.get("triage_status") == "pending" and "example" not in p.name:
                add(findings, "high", "inbox_pending",
                    f"Inbox item still pending: {p.name}", p.relative_to(WIKI).as_posix(),
                    "Run triage skill")
            elif meta.get("triage_status") == "pending":
                add(findings, "low", "inbox_example_pending",
                    f"Example inbox item still pending (ok for tutorial): {p.name}",
                    p.relative_to(WIKI).as_posix(), "Archive when no longer needed")

    # Bootstrap artifacts
    for req in [
        ROOT / "AGENTS.md",
        WIKI / "INDEX.md",
        PATHS.system / "docs" / "SCHEMA.md",
        PATHS.system / "docs" / "OPERATOR.md",
        PATHS.system / "docs" / "ROUTER.md",
        PATHS.logs / "operations.md",
        PATHS.claim_graph,
        PATHS.system / "docs" / "FRAMEWORK.md",
        PATHS.host_config,
        PATHS.skills / "wiki-brain" / "SKILL.md",
        PATHS.skills / "wiki-usage" / "SKILL.md",
        PATHS.skills / "wiki-retrieve" / "SKILL.md",
        PATHS.skills / "wiki-doctor" / "SKILL.md",
        PATHS.skills / "wiki-heal" / "SKILL.md",
        PATHS.skills / "wiki-triage" / "SKILL.md",
        PATHS.scripts / "wiki_retrieve.py",
        PATHS.scripts / "wiki_doctor.py",
        PATHS.scripts / "wiki_usage.py",
        PATHS.scripts / "wiki_graphify.py",
        PATHS.scripts / "wiki_setup.py",
        PATHS.scripts / "sync_graph.py",
        PATHS.skills / "wiki-setup" / "SKILL.md",
        WIKI / "episodic" / "INDEX.md",
        WIKI / "temporal" / "TIMELINE.md",
    ]:
        if not req.exists():
            add(findings, "critical", "missing_artifact", f"Missing {req.relative_to(ROOT)}",
                str(req.relative_to(ROOT)), "Restore from git / recreate")

    gcfg = host.get("graphify") or {}
    if gcfg.get("enabled", False):
        from wiki_graphify import graph_json_path, find_graphify
        gj = graph_json_path(gcfg if "out" in gcfg else None)
        if not find_graphify():
            add(findings, "low", "graphify_cli_missing",
                "graphify CLI not on PATH (code graph owner)",
                "graphify-out/graph.json", "pip install graphifyy")
        elif not gj.exists():
            add(findings, "low", "graphify_graph_missing",
                "HOST.yaml enables Graphify but graph.json is missing",
                str(gj.relative_to(ROOT)) if ROOT in gj.parents else str(gj),
                "python3 docs/wiki/_system/scripts/wiki_graphify.py sync")

    # Temporal hygiene: active pages with expired valid_until
    today = date.today()
    for rel, meta in metas.items():
        temporal = meta.get("temporal") or {}
        until = temporal.get("valid_until")
        until_d = None
        if until not in (None, "", "null"):
            try:
                until_d = until if isinstance(until, date) else date.fromisoformat(str(until)[:10])
            except ValueError:
                add(findings, "low", "bad_valid_until", f"Unparseable temporal.valid_until={until!r}", rel,
                    "Use YYYY-MM-DD or null")
                continue
        if until_d and until_d <= today and meta.get("status") == "active":
            add(findings, "medium", "expired_still_active",
                f"valid_until {until_d} but status=active", rel,
                "Set status stale/deprecated or clear valid_until")
        if meta.get("type") == "episode":
            ep = meta.get("episode") or {}
            if not (temporal.get("observed_at") and temporal.get("valid_from")):
                add(findings, "high", "episode_missing_temporal",
                    "episode lacks temporal.observed_at/valid_from", rel, "SCHEMA §9")
            if not ep.get("goal") and "TEMPLATE" not in rel:
                add(findings, "medium", "episode_missing_goal",
                    "episode.goal missing", rel, "Fill episode.goal")

    # Score
    sev = Counter(f["severity"] for f in findings)
    score = 100
    score -= sev.get("critical", 0) * 15
    score -= sev.get("high", 0) * 5
    score -= sev.get("medium", 0) * 1
    score -= sev.get("low", 0) * 0.25
    score = max(0, round(score, 1))

    report = {
        "date": TODAY,
        "score": score,
        "counts": dict(sev),
        "graph": {
            "nodes": len(node_ids),
            "edges": len(graph.get("edges") or []),
            "hard_orphans": len(hard_orphans),
        },
        "pages_with_frontmatter": len(metas),
        "pages_scanned": len([p for p in md_files if p.name != "_TEMPLATE.md"]),
        "findings": findings,
        "heal_recommended": sev.get("critical", 0) + sev.get("high", 0) + sev.get("medium", 0) > 0,
    }

    PATHS.doctor_dir.mkdir(parents=True, exist_ok=True)
    latest = PATHS.doctor_latest
    latest.write_text(json.dumps(report, indent=2), encoding="utf-8")

    md_path = PATHS.doctor_dir / f"doctor-{TODAY}.md"
    lines = [
        "---",
        f"id: doctor-{TODAY}",
        f"title: Wiki Doctor Report {TODAY}",
        "type: meta",
        "status: active",
        f"created: {TODAY}",
        f"updated: {TODAY}",
        "tags: [doctor, health]",
        "domain: meta",
        f"summary: \"Doctor score {score}/100 — critical={sev.get('critical',0)} high={sev.get('high',0)} medium={sev.get('medium',0)}.\"",
        "nodes: []",
        "edges: []",
        "related:",
        '  - "[[_system/docs/GRAPH]]"',
        "agent:",
        "  priority: medium",
        "  read_when:",
        '    - "after wiki doctor"',
        "  maintain: []",
        "---",
        "",
        f"# Wiki Doctor Report — {TODAY}",
        "",
        f"**Score:** {score}/100",
        "",
        f"| Severity | Count |",
        f"|----------|------:|",
        f"| critical | {sev.get('critical', 0)} |",
        f"| high | {sev.get('high', 0)} |",
        f"| medium | {sev.get('medium', 0)} |",
        f"| low | {sev.get('low', 0)} |",
        "",
        f"Graph: {report['graph']['nodes']} nodes / {report['graph']['edges']} edges "
        f"({report['graph']['hard_orphans']} hard orphans)",
        "",
        f"Heal recommended: **{'yes' if report['heal_recommended'] else 'no'}** "
        f"(use `wiki-heal`)",
        "",
        "## Findings",
        "",
    ]
    if not findings:
        lines.append("_No findings. Wiki looks healthy._")
    else:
        by = defaultdict(list)
        for f in findings:
            by[f["severity"]].append(f)
        for severity in ("critical", "high", "medium", "low"):
            if not by[severity]:
                continue
            lines.append(f"### {severity}")
            lines.append("")
            for f in by[severity]:
                loc = f" (`{f['path']}`)" if f.get("path") else ""
                fix = f" → fix: {f['fix']}" if f.get("fix") else ""
                lines.append(f"- **{f['code']}**{loc}: {f['message']}{fix}")
            lines.append("")

    md_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    if not no_log:
        log_event(
            "doctor",
            doctor_score=score,
            tokens_est=None,
            source="script",
            hits=len(findings),
        )
        try:
            write_dashboard(compute_stats(load_events(30)), 30)
        except Exception:  # noqa: BLE001
            pass
    print(f"Doctor score {score}/100")
    print(f"Wrote {md_path}")
    print(f"Wrote {latest}")
    print(f"Counts: {dict(sev)}")
    print(f"Heal recommended: {report['heal_recommended']}")


if __name__ == "__main__":
    main()
