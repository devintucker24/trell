#!/usr/bin/env python3
"""Wiki retrieve — file-native hybrid lexical + graph + temporal rerank.

Usage:
  python3 docs/wiki/_system/scripts/wiki_retrieve.py "maritime colregs belief"
  python3 docs/wiki/_system/scripts/wiki_retrieve.py "what did we decide about memory" --lane episodic
  python3 docs/wiki/_system/scripts/wiki_retrieve.py "epistemic types" --as-of 2026-09-04 --budget-tokens 3500
"""

from __future__ import annotations

import argparse
import math
import re
import sys
import time
from collections import defaultdict
from datetime import date, datetime
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).resolve().parent))
from repobrain_paths import PATHS, ROOT, WIKI, is_wiki_content_page, load_host
from wiki_usage import log_event

GRAPH_PATH = PATHS.claim_graph

STOP = {
    "a", "an", "the", "and", "or", "of", "to", "in", "on", "for", "is", "are",
    "be", "as", "at", "by", "with", "from", "this", "that", "it", "we", "our",
    "how", "what", "when", "why", "does", "do", "did", "about", "into", "vs",
    "keep", "keeps", "keeping", "under", "over", "into", "using", "use", "used",
    "make", "made", "get", "got", "can", "could", "should", "would", "may",
    "safe",  # too generic across all Trell apps; prefer domain nouns
}

TYPE_PRIOR = {
    "concept": 1.0,
    "application": 0.95,
    "synthesis": 0.95,
    "decision": 0.95,
    "episode": 0.9,
    "market": 0.85,
    "roadmap": 0.85,
    "meta": 0.75,
    "schema": 0.7,
    "index": 0.55,
    "raw-pointer": 0.4,
    "inbox-item": 0.2,
}

LANE_DIRS = {
    "semantic": ("core", "theory", "applications", "market", "roadmap"),
    "episodic": ("episodic",),
    "temporal": ("temporal",),
    "meta": ("_meta",),
    "all": None,
}


def parse_fm(text: str):
    if not text.startswith("---\n"):
        return None, text
    end = text.find("\n---\n", 4)
    if end == -1:
        return None, text
    try:
        meta = yaml.safe_load(text[4:end])
    except Exception:  # noqa: BLE001
        return None, text
    body = text[end + 5 :]
    return meta or {}, body


def tokenize(s: str) -> list[str]:
    return [t for t in re.findall(r"[a-z0-9][a-z0-9\-_]{1,}", (s or "").lower()) if t not in STOP]


def parse_day(val) -> date | None:
    if val is None or val == "" or val is False:
        return None
    if isinstance(val, date) and not isinstance(val, datetime):
        return val
    if isinstance(val, datetime):
        return val.date()
    try:
        return date.fromisoformat(str(val)[:10])
    except ValueError:
        return None


def section_chunks(body: str, path: str, max_chars: int = 1200) -> list[dict]:
    parts = re.split(r"(?m)^(#{1,3} .+)$", body)
    chunks: list[dict] = []
    # preamble
    if parts and parts[0].strip():
        chunks.append({"anchor": "preamble", "heading": "", "text": parts[0].strip()[:max_chars]})
    i = 1
    while i < len(parts) - 1:
        heading = parts[i].lstrip("#").strip()
        text = parts[i + 1].strip()
        if text:
            chunks.append({
                "anchor": re.sub(r"[^a-z0-9]+", "-", heading.lower()).strip("-")[:60] or "section",
                "heading": heading,
                "text": text[:max_chars],
            })
        i += 2
    if not chunks:
        chunks.append({"anchor": "body", "heading": "", "text": body.strip()[:max_chars]})
    for c in chunks:
        c["path"] = path
    return chunks


def load_graph():
    if not GRAPH_PATH.exists():
        return {"nodes": [], "edges": []}
    return yaml.safe_load(GRAPH_PATH.read_text(encoding="utf-8")) or {"nodes": [], "edges": []}


def graph_proximity(page_nodes: set[str], seed_nodes: set[str], edges: list[dict]) -> float:
    if not page_nodes:
        return 0.0
    if page_nodes & seed_nodes:
        return 1.0
    # one hop
    neighbors: set[str] = set()
    for e in edges:
        if e.get("from") in seed_nodes:
            neighbors.add(e.get("to"))
        if e.get("to") in seed_nodes:
            neighbors.add(e.get("from"))
    if page_nodes & neighbors:
        return 0.65
    # two hop weak
    n2: set[str] = set()
    for e in edges:
        if e.get("from") in neighbors:
            n2.add(e.get("to"))
        if e.get("to") in neighbors:
            n2.add(e.get("from"))
    if page_nodes & n2:
        return 0.3
    return 0.0


def temporal_score(meta: dict, as_of: date | None, query_tokens: set[str]) -> float:
    """Validity fit + recency. Higher is better."""
    temporal = meta.get("temporal") or {}
    valid_from = parse_day(temporal.get("valid_from")) or parse_day(meta.get("created"))
    valid_until = parse_day(temporal.get("valid_until"))
    updated = parse_day(meta.get("updated")) or valid_from
    observed = parse_day(temporal.get("observed_at")) or updated

    time_sensitive = bool(query_tokens & {"when", "as-of", "asof", "changed", "since", "before", "after", "history", "timeline", "supersede", "deprecated"})

    if as_of is not None:
        if valid_from and valid_from > as_of:
            return 0.05  # not yet true
        if valid_until and valid_until <= as_of:
            return 0.1 if time_sensitive else 0.05  # expired; only useful for history
        # in window
        base = 0.85
    else:
        # prefer currently valid
        if valid_until and valid_until <= date.today():
            base = 0.15 if time_sensitive else 0.05
        else:
            base = 0.7

    # recency boost (half-life ~45 days)
    ref = observed or updated or date.today()
    age_days = max(0, (date.today() - ref).days)
    recency = math.exp(-age_days / 45.0)
    if meta.get("type") == "episode":
        recency = math.exp(-age_days / 14.0)  # episodes decay faster

    # superseded penalty
    if temporal.get("superseded_by") or meta.get("status") in ("deprecated", "stale"):
        base *= 0.35 if not time_sensitive else 0.8

    return min(1.0, 0.55 * base + 0.45 * recency)


def lexical_score(query_tokens: list[str], meta: dict, chunk: dict, path: str) -> float:
    if not query_tokens:
        return 0.0
    titleish = f"{meta.get('id','')} {meta.get('title','')} {meta.get('summary','')}".lower()
    tags = " ".join(meta.get("tags") or []).lower()
    path_l = path.lower().replace("/", " ").replace("-", " ")
    heading = (chunk.get("heading") or "").lower()
    body = (chunk.get("text") or "")[:1600].lower()
    hits = 0.0
    heading_hits = 0
    for t in query_tokens:
        weight = 0.0
        if t in titleish:
            weight = max(weight, 2.4)
        if t in tags:
            weight = max(weight, 2.0)
        if t in path_l:
            weight = max(weight, 1.8)
        if t in heading:
            weight = max(weight, 2.8)  # section match is gold for chunking
            heading_hits += 1
        elif t in body:
            weight = max(weight, 1.0)
        hits += weight
    # Prefer chunks whose heading absorbs multiple query nouns
    if heading_hits >= 2:
        hits += 1.5
    elif heading_hits == 1 and any(t in heading for t in query_tokens if len(t) >= 5):
        hits += 0.6
    return min(1.0, hits / (len(query_tokens) * 1.8))


def frontmatter_boost(query_tokens: set[str], meta: dict) -> float:
    read_when = " ".join(meta.get("agent", {}).get("read_when") or []).lower()
    tags = set(tokenize(" ".join(meta.get("tags") or [])))
    summary = set(tokenize(str(meta.get("summary") or "")))
    overlap = len(query_tokens & (tags | summary | set(tokenize(read_when))))
    # also raw substring hits in read_when (e.g. COLREGs)
    rw_hits = sum(1 for t in query_tokens if t in read_when)
    pri = {"critical": 1.0, "high": 0.85, "medium": 0.6, "low": 0.4}.get(
        (meta.get("agent") or {}).get("priority", "medium"), 0.6
    )
    return min(1.0, 0.45 * pri + 0.35 * min(1.0, overlap / max(1, len(query_tokens) * 0.5)) + 0.2 * min(1.0, rw_hits / 2))


def mmr_select(scored: list[dict], k: int, lambda_: float = 0.7) -> list[dict]:
    """Diversity-aware selection — prefer distinct paths, then distinct anchors."""
    selected: list[dict] = []
    remaining = list(scored)
    while remaining and len(selected) < k:
        if not selected:
            selected.append(remaining.pop(0))
            continue
        best_i, best_val = 0, -1e9
        sel_paths = {s["path"] for s in selected}
        sel_tags = [set(s.get("tags") or []) for s in selected]
        for i, cand in enumerate(remaining):
            red = 0.0
            if cand["path"] in sel_paths:
                red = 0.85  # allow second chunk from same page but penalize
            else:
                ct = set(cand.get("tags") or [])
                if ct and sel_tags:
                    red = max((len(ct & st) / max(1, len(ct | st)) for st in sel_tags), default=0.0)
            val = lambda_ * cand["score"] - (1 - lambda_) * red
            if val > best_val:
                best_val, best_i = val, i
        selected.append(remaining.pop(best_i))
    return selected


def estimate_tokens(text: str) -> int:
    return max(1, len(text) // 4)


def main() -> None:
    ap = argparse.ArgumentParser(description="Wiki-brain hybrid retrieve")
    ap.add_argument("query", help="natural language query")
    ap.add_argument("--k", type=int, default=8, help="max candidates before budget trim")
    ap.add_argument("--budget-tokens", type=int, default=3500)
    ap.add_argument("--as-of", dest="as_of", default=None, help="YYYY-MM-DD validity filter")
    ap.add_argument(
        "--lane",
        choices=list(LANE_DIRS.keys()),
        default="all",
        help="memory lane filter",
    )
    ap.add_argument("--json", action="store_true", help="machine-readable output")
    ap.add_argument("--no-log", action="store_true", help="do not append usage telemetry")
    ap.add_argument(
        "--code",
        action="store_true",
        help="also query Graphify graph.json (AST/call graph) after wiki hits",
    )
    args = ap.parse_args()

    t0 = time.perf_counter()
    host = load_host()
    semantic = tuple(host.get("semantic_dirs") or LANE_DIRS["semantic"])
    lane_map = {**LANE_DIRS, "semantic": semantic}

    as_of = parse_day(args.as_of) if args.as_of else None
    q_tokens = tokenize(args.query)
    q_set = set(q_tokens)
    graph = load_graph()
    edges = graph.get("edges") or []

    # seed nodes: tokens matching node ids/labels
    seed_nodes: set[str] = set()
    for n in graph.get("nodes") or []:
        nid = n.get("id", "")
        label = str(n.get("label", "")).lower()
        if any(t in nid.lower() or t in label for t in q_tokens):
            seed_nodes.add(nid)

    lane_dirs = lane_map[args.lane]
    candidates: list[dict] = []

    for path in sorted(WIKI.rglob("*.md")):
        rel = path.relative_to(WIKI).as_posix()
        if not is_wiki_content_page(rel, path.name):
            continue
        top = rel.split("/", 1)[0]
        if lane_dirs is not None and top not in lane_dirs and not (
            args.lane == "semantic" and rel in ("INDEX.md", "SCHEMA.md", "ROUTER.md")
        ):
            # allow ROUTER etc only in all/meta
            if args.lane != "all":
                continue

        text = path.read_text(encoding="utf-8")
        meta, body = parse_fm(text)
        if not meta:
            continue
        if meta.get("type") == "inbox-item":
            continue

        page_nodes = {n["id"] for n in (meta.get("nodes") or []) if isinstance(n, dict) and "id" in n}
        # also collect edge endpoints owned here
        for e in meta.get("edges") or []:
            if e.get("from"):
                page_nodes.add(e["from"])
            if e.get("to"):
                page_nodes.add(e["to"])

        gprox = graph_proximity(page_nodes, seed_nodes, edges) if seed_nodes else 0.0
        tscore = temporal_score(meta, as_of, q_set)
        fboost = frontmatter_boost(q_set, meta)
        tprior = TYPE_PRIOR.get(meta.get("type", "concept"), 0.7)

        for chunk in section_chunks(body, rel):
            lex = lexical_score(q_tokens, meta, chunk, rel)
            # Gate graph: don't let pure connectivity outrank weak lexical match
            g_eff = gprox * max(lex, 0.25)
            score = (
                0.42 * lex
                + 0.15 * fboost
                + 0.13 * g_eff
                + 0.15 * tscore
                + 0.05 * tprior
            )
            # tiny lane prior when query implies memory kind
            if args.lane == "all":
                if q_set & {"episode", "session", "decided", "decision", "failed", "lesson"} and meta.get("type") == "episode":
                    score += 0.08
                if q_set & {"timeline", "as-of", "asof", "changed", "since", "supersede"} and rel.startswith("temporal/"):
                    score += 0.1

            if score < 0.08:
                continue
            candidates.append({
                "score": round(score, 4),
                "lex": round(lex, 4),
                "graph": round(g_eff, 4),
                "temporal": round(tscore, 4),
                "frontmatter": round(fboost, 4),
                "path": rel,
                "id": meta.get("id"),
                "title": meta.get("title"),
                "type": meta.get("type"),
                "tags": meta.get("tags") or [],
                "anchor": chunk["anchor"],
                "heading": chunk["heading"],
                "excerpt": chunk["text"][:500].replace("\n", " ").strip(),
                "provenance": {
                    "kind": _provenance_kind(rel, semantic),
                    "path": rel,
                    "page_id": meta.get("id"),
                },
                "why": _why(lex, g_eff, tscore, fboost),
            })

    candidates.sort(key=lambda x: x["score"], reverse=True)
    # de-dupe same path+anchor keeping best
    seen = set()
    deduped = []
    for c in candidates:
        key = (c["path"], c["anchor"])
        if key in seen:
            continue
        seen.add(key)
        deduped.append(c)

    selected = mmr_select(deduped, k=max(args.k, 1))

    # budget trim
    packed = []
    used = 0
    for c in selected:
        cost = estimate_tokens(c["excerpt"]) + 40
        if used + cost > args.budget_tokens and packed:
            break
        packed.append(c)
        used += cost

    duration_ms = int((time.perf_counter() - t0) * 1000)
    if not args.no_log:
        log_event(
            "retrieve",
            query=args.query,
            lane=args.lane,
            as_of=args.as_of,
            hits=len(packed),
            tokens_est=used,
            budget_tokens=args.budget_tokens,
            top_score=packed[0]["score"] if packed else 0.0,
            duration_ms=duration_ms,
            hit_paths=[c["path"] for c in packed[:8]],
            source="script",
        )

    code_note = _code_graph_note(args.query, run=args.code)

    if args.json:
        import json
        payload = {
            "query": args.query,
            "as_of": args.as_of,
            "lane": args.lane,
            "packed_tokens": used,
            "budget_tokens": args.budget_tokens,
            "hits": packed,
        }
        if code_note:
            payload["code_graph"] = code_note
        print(json.dumps(payload, indent=2))
        return

    print(f"# retrieve: {args.query!r}")
    print(f"# lane={args.lane} as_of={args.as_of or 'none'} hits={len(packed)} ~tokens={used}")
    if code_note.get("status"):
        print(f"# code-graph: {code_note['status']}")
    print()
    for i, c in enumerate(packed, 1):
        heading = f" › {c['heading']}" if c["heading"] else ""
        print(f"{i}. [{c['score']:.3f}] {c['path']}{heading}")
        print(f"   id={c['id']} type={c['type']} lex={c['lex']} graph={c['graph']} temporal={c['temporal']}")
        print(f"   why: {c['why']}")
        print(f"   {c['excerpt'][:220]}…")
        print()
    if args.code and code_note.get("query_output"):
        print("## code graph (Graphify)")
        print(code_note["query_output"])


def _provenance_kind(rel: str, semantic_dirs: tuple[str, ...]) -> str:
    top = rel.split("/", 1)[0]
    if top in semantic_dirs:
        return "compiled"
    if top == "raw":
        return "raw"
    if top == "episodic":
        return "episodic"
    if top == "temporal":
        return "temporal"
    return "meta"


def _code_graph_note(query: str, run: bool = False) -> dict:
    """Pointer to Graphify; optionally run query. Never dump graph.json."""
    try:
        from wiki_graphify import graph_json_path, load_code_graph, find_graphify, run_graphify
    except Exception:  # noqa: BLE001
        return {}
    path = graph_json_path()
    if not path.exists():
        return {"status": "missing graphify-out/graph.json — wiki_graphify.py sync"}
    g = load_code_graph()
    n, e = len(g.get("nodes") or []), len(g.get("edges") or [])
    note = {
        "status": f"{path.relative_to(ROOT)} ({n}n/{e}e) — python3 docs/wiki/_system/scripts/wiki_graphify.py query {query!r}",
        "nodes": n,
        "edges": e,
    }
    if run and find_graphify():
        import subprocess
        try:
            proc = run_graphify(
                ["query", query, "--graph", str(path), "--budget", "800"],
                check=False,
                capture=True,
            )
            note["query_output"] = ((proc.stdout or "") + (proc.stderr or ""))[:4000]
        except (OSError, subprocess.SubprocessError) as exc:
            note["query_output"] = str(exc)
    return note


def _why(lex, gprox, tscore, fboost) -> str:
    parts = []
    if lex >= 0.35:
        parts.append("lexical")
    if gprox >= 0.5:
        parts.append("graph-near")
    elif gprox >= 0.25:
        parts.append("graph-weak")
    if tscore >= 0.6:
        parts.append("temporal-fit")
    elif tscore <= 0.2:
        parts.append("temporal-stale")
    if fboost >= 0.5:
        parts.append("read_when/tags")
    return "+".join(parts) or "weak"


if __name__ == "__main__":
    main()
