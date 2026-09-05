#!/usr/bin/env python3
"""Wiki-brain usage telemetry — append JSONL events and score a dashboard.

Events are local (gitignored JSONL). The dashboard markdown is a wiki page
agents and humans can read in git.

Usage:
  python3 docs/wiki/_system/scripts/wiki_usage.py log --op retrieve --query "..." --tokens-est 1200 --hits 8
  python3 docs/wiki/_system/scripts/wiki_usage.py report --days 30
  python3 docs/wiki/_system/scripts/wiki_usage.py score --days 30
"""

from __future__ import annotations

import argparse
import json
import statistics
import sys
from collections import Counter
from datetime import date, datetime, timezone, timedelta
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from repobrain_paths import DASHBOARD_PATH, EVENTS_PATH, USAGE_DIR

OPS = {
    "retrieve", "query", "navigate", "triage", "ingest", "doctor", "heal",
    "lint", "label", "maintain", "session", "dump", "usage",
    # dump = agent admitted it loaded INDEX/folder
}

WEAK_HIT = 0.25
STRONG_HIT = 0.45


def _now() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def log_event(op: str, **fields) -> Path:
    """Append one JSON object. Safe to call from other scripts."""
    USAGE_DIR.mkdir(parents=True, exist_ok=True)
    rec = {"ts": fields.pop("ts", None) or _now(), "op": op}
    rec.update({k: v for k, v in fields.items() if v is not None})
    with EVENTS_PATH.open("a", encoding="utf-8") as f:
        f.write(json.dumps(rec, ensure_ascii=False) + "\n")
    return EVENTS_PATH


def load_events(days: int | None = None) -> list[dict]:
    if not EVENTS_PATH.exists():
        return []
    cutoff = None
    if days is not None:
        cutoff = datetime.now(timezone.utc) - timedelta(days=days)
    out = []
    for line in EVENTS_PATH.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            ev = json.loads(line)
        except json.JSONDecodeError:
            continue
        if cutoff:
            ts = ev.get("ts") or ""
            try:
                dt = datetime.fromisoformat(ts.replace("Z", "+00:00"))
            except ValueError:
                continue
            if dt < cutoff:
                continue
        out.append(ev)
    return out


def compute_stats(events: list[dict]) -> dict:
    by_op = Counter(e.get("op") for e in events)
    retrieves = [e for e in events if e.get("op") == "retrieve"]
    tokens = [int(e["tokens_est"]) for e in events if e.get("tokens_est") is not None]
    budgets = [int(e["budget_tokens"]) for e in retrieves if e.get("budget_tokens")]
    top_scores = [float(e["top_score"]) for e in retrieves if e.get("top_score") is not None]
    weak = sum(1 for s in top_scores if s < WEAK_HIT)
    strong = sum(1 for s in top_scores if s >= STRONG_HIT)
    doctors = [e for e in events if e.get("op") == "doctor" and e.get("doctor_score") is not None]
    last_doctor = doctors[-1]["doctor_score"] if doctors else None
    queries = [e.get("query") for e in retrieves if e.get("query")]
    qcount = Counter(queries)
    repeats = sum(1 for _q, n in qcount.items() if n >= 3)
    pages = Counter()
    for e in retrieves:
        for p in e.get("hit_paths") or []:
            pages[p] += 1
    dumps = by_op.get("dump", 0)
    retrieve_n = len(retrieves) or 1
    avg_tokens = round(statistics.mean(tokens), 1) if tokens else 0
    total_tokens = sum(tokens)
    latencies = [int(e["duration_ms"]) for e in events if e.get("duration_ms") is not None]
    opened = cited = overlap = 0
    for e in events:
        po = e.get("pages_opened") or []
        ci = e.get("cited") or []
        if po or ci:
            opened += len(po)
            cited += len(ci)
            overlap += len(set(po) & set(ci))
    citation_ratio = round(overlap / max(opened, 1), 3) if opened else None
    budget_use = None
    if retrieves and budgets:
        used = [int(e.get("tokens_est") or 0) / max(int(e.get("budget_tokens") or 1), 1) for e in retrieves]
        budget_use = round(100 * statistics.mean(used), 1)

    # Usefulness index 0–100 (heuristic, not a scientific KPI)
    hit_q = 50.0
    if top_scores:
        hit_q = min(100.0, 100.0 * (strong / max(len(top_scores), 1)))
        hit_q -= 20.0 * (weak / max(len(top_scores), 1))
    health = float(last_doctor) if last_doctor is not None else 70.0
    dump_pen = min(30.0, dumps * 10.0)
    activity = min(15.0, len(events) * 0.5)
    index = max(0.0, min(100.0, 0.45 * hit_q + 0.35 * health + activity - dump_pen))

    return {
        "events": len(events),
        "by_op": dict(by_op),
        "retrieve_count": len(retrieves),
        "tokens_total_est": total_tokens,
        "tokens_avg_est": avg_tokens,
        "budget_utilization_pct": budget_use,
        "top_score_avg": round(statistics.mean(top_scores), 3) if top_scores else None,
        "weak_hit_rate": round(weak / retrieve_n, 3) if retrieves else None,
        "strong_hit_rate": round(strong / retrieve_n, 3) if retrieves else None,
        "last_doctor_score": last_doctor,
        "repeat_query_groups": repeats,
        "dump_admissions": dumps,
        "hot_pages": pages.most_common(8),
        "latency_ms_avg": round(statistics.mean(latencies), 1) if latencies else None,
        "pages_opened": opened,
        "pages_cited": cited,
        "citation_overlap_ratio": citation_ratio,
        "usefulness_index": round(index, 1),
    }


def write_dashboard(stats: dict, days: int) -> Path:
    today = date.today().isoformat()
    by_op = stats["by_op"] or {}
    op_rows = "\n".join(f"| `{k}` | {v} |" for k, v in sorted(by_op.items())) or "| _(none)_ | 0 |"
    hot = stats["hot_pages"] or []
    hot_rows = "\n".join(f"| `{p}` | {n} |" for p, n in hot) or "| _(none)_ | 0 |"
    body = f"""---
id: wiki-usage-dashboard
title: Wiki-brain usage dashboard
type: meta
status: active
created: 2026-09-04
updated: {today}
tags: [usage, telemetry, context-cost]
domain: meta
summary: "Generated usage snapshot — retrieve tokens, hit quality, doctor score, usefulness index."
nodes:
  - id: wiki-usage-dashboard
    kind: concept
    label: Wiki usage dashboard
edges:
  - from: wiki-usage-dashboard
    to: wiki-usage-telemetry
    rel: implements
related:
  - "[[_system/docs/usage-telemetry]]"
  - "[[_system/docs/ROUTER]]"
agent:
  priority: medium
  read_when:
    - "checking whether the wiki brain is earning its context cost"
    - "tuning retrieve budgets"
  maintain:
    - "regenerate via python3 docs/wiki/_system/scripts/wiki_usage.py report"
---

# Wiki-brain usage dashboard

Generated `{today}` from local `docs/wiki/_system/generated/usage/events.jsonl` (last **{days}** days).
Raw events are gitignored; this page is the shareable snapshot.

**Usefulness index:** {stats['usefulness_index']}/100  
(heuristic: retrieval hit quality + last doctor score + activity − INDEX-dump admissions)

| Metric | Value |
|--------|------:|
| Events | {stats['events']} |
| Retrieves | {stats['retrieve_count']} |
| Est. retrieve tokens (sum) | {stats['tokens_total_est']} |
| Est. tokens / event with tokens | {stats['tokens_avg_est']} |
| Budget utilization | {stats['budget_utilization_pct'] if stats['budget_utilization_pct'] is not None else '—'}% |
| Mean top hit score | {stats['top_score_avg'] if stats['top_score_avg'] is not None else '—'} |
| Weak-hit rate (top < {WEAK_HIT}) | {stats['weak_hit_rate'] if stats['weak_hit_rate'] is not None else '—'} |
| Strong-hit rate (top ≥ {STRONG_HIT}) | {stats['strong_hit_rate'] if stats['strong_hit_rate'] is not None else '—'} |
| Last doctor score | {stats['last_doctor_score'] if stats['last_doctor_score'] is not None else '—'} |
| Repeat-query groups (≥3) | {stats['repeat_query_groups']} |
| Dump admissions | {stats['dump_admissions']} |
| Mean latency (ms) | {stats['latency_ms_avg'] if stats['latency_ms_avg'] is not None else '—'} |
| Pages opened / cited | {stats['pages_opened']} / {stats['pages_cited']} |
| Citation overlap (cited ∩ opened / opened) | {stats['citation_overlap_ratio'] if stats['citation_overlap_ratio'] is not None else '—'} |

## Ops mix

| op | count |
|----|------:|
{op_rows}

## Hottest retrieve pages

| path | hits |
|------|-----:|
{hot_rows}

## How to read this

- **High tokens + low strong-hit rate** → retrieve is expensive and missing; add pages or fix ROUTER seeds.
- **Repeat-query groups** → same question keeps coming; the answer should be a first-class page.
- **Dump admissions** → agents skipped retrieve; tighten always-on rules.
- **Doctor score** → structural health, not usefulness. Both are needed.

See [[_system/docs/usage-telemetry]] for the metric catalog and agent logging protocol.
"""
    DASHBOARD_PATH.write_text(body, encoding="utf-8")
    return DASHBOARD_PATH


def cmd_log(args: argparse.Namespace) -> None:
    extra = {}
    if args.query:
        extra["query"] = args.query
    if args.lane:
        extra["lane"] = args.lane
    if args.as_of:
        extra["as_of"] = args.as_of
    if args.hits is not None:
        extra["hits"] = args.hits
    if args.tokens_est is not None:
        extra["tokens_est"] = args.tokens_est
    if args.budget_tokens is not None:
        extra["budget_tokens"] = args.budget_tokens
    if args.top_score is not None:
        extra["top_score"] = args.top_score
    if args.duration_ms is not None:
        extra["duration_ms"] = args.duration_ms
    if args.doctor_score is not None:
        extra["doctor_score"] = args.doctor_score
    if args.pages_opened:
        extra["pages_opened"] = [p.strip() for p in args.pages_opened.split(",") if p.strip()]
    if args.cited:
        extra["cited"] = [p.strip() for p in args.cited.split(",") if p.strip()]
    if args.hit_paths:
        extra["hit_paths"] = [p.strip() for p in args.hit_paths.split(",") if p.strip()]
    extra["source"] = args.source
    path = log_event(args.op, **extra)
    print(f"logged {args.op} → {path}")


def cmd_report(args: argparse.Namespace) -> None:
    events = load_events(days=args.days)
    stats = compute_stats(events)
    path = write_dashboard(stats, args.days)
    print(json.dumps(stats, indent=2))
    print(f"Wrote {path}")


def main() -> None:
    ap = argparse.ArgumentParser(description="Wiki-brain usage telemetry")
    sub = ap.add_subparsers(dest="cmd", required=True)

    lg = sub.add_parser("log", help="append one event")
    lg.add_argument("--op", required=True, choices=sorted(OPS))
    lg.add_argument("--query", default=None)
    lg.add_argument("--lane", default=None)
    lg.add_argument("--as-of", dest="as_of", default=None)
    lg.add_argument("--hits", type=int, default=None)
    lg.add_argument("--tokens-est", dest="tokens_est", type=int, default=None)
    lg.add_argument("--budget-tokens", dest="budget_tokens", type=int, default=None)
    lg.add_argument("--top-score", dest="top_score", type=float, default=None)
    lg.add_argument("--duration-ms", dest="duration_ms", type=int, default=None)
    lg.add_argument("--doctor-score", dest="doctor_score", type=float, default=None)
    lg.add_argument("--pages-opened", dest="pages_opened", default=None)
    lg.add_argument("--cited", default=None)
    lg.add_argument("--hit-paths", dest="hit_paths", default=None)
    lg.add_argument("--source", default="agent")
    lg.set_defaults(func=cmd_log)

    rp = sub.add_parser("report", help="write usage-dashboard.md")
    rp.add_argument("--days", type=int, default=30)
    rp.set_defaults(func=cmd_report)

    sc = sub.add_parser("score", help="print usefulness index JSON")
    sc.add_argument("--days", type=int, default=30)
    sc.set_defaults(func=lambda a: print(json.dumps(compute_stats(load_events(a.days)), indent=2)))

    args = ap.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
