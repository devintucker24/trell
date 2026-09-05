#!/usr/bin/env python3
"""Regenerate the RepoBrain claim graph from host corpus frontmatter."""

from __future__ import annotations

from pathlib import Path
import sys

import yaml

sys.path.insert(0, str(Path(__file__).resolve().parent))
from repobrain_paths import PATHS, WIKI, is_wiki_content_page

GRAPH_PATH = PATHS.claim_graph


def parse_frontmatter(text: str) -> dict | None:
    if not text.startswith("---\n"):
        return None
    end = text.find("\n---\n", 4)
    if end == -1:
        return None
    return yaml.safe_load(text[4:end])


def main() -> None:
    nodes: dict[str, dict] = {}
    edges: list[dict] = []
    seen: set[tuple] = set()

    for path in sorted(WIKI.rglob("*.md")):
        rel = path.relative_to(WIKI).as_posix()
        if not is_wiki_content_page(rel, path.name):
            continue
        meta = parse_frontmatter(path.read_text(encoding="utf-8"))
        if not meta:
            continue
        page = rel[:-3] if rel.endswith(".md") else rel
        for n in meta.get("nodes") or []:
            nodes[n["id"]] = {
                "id": n["id"],
                "kind": n.get("kind", "concept"),
                "page": page,
                "label": n.get("label", n["id"]),
            }
        for e in meta.get("edges") or []:
            key = (e["from"], e["to"], e["rel"])
            if key in seen:
                continue
            seen.add(key)
            item = {"from": e["from"], "to": e["to"], "rel": e["rel"], "page": page}
            if e.get("note"):
                item["note"] = e["note"]
            edges.append(item)

    for e in edges:
        for endpoint in (e["from"], e["to"]):
            if endpoint not in nodes:
                nodes[endpoint] = {
                    "id": endpoint,
                    "kind": "concept",
                    "page": None,
                    "label": endpoint,
                }

    from datetime import date

    graph = {
        "version": 1,
        "updated": date.today().isoformat(),
        "description": "Wiki-brain knowledge graph — regenerated from page frontmatter",
        "nodes": sorted(nodes.values(), key=lambda x: x["id"]),
        "edges": sorted(edges, key=lambda x: (x["from"], x["to"], x["rel"])),
    }
    GRAPH_PATH.parent.mkdir(parents=True, exist_ok=True)
    GRAPH_PATH.write_text(yaml.safe_dump(graph, sort_keys=False, allow_unicode=True), encoding="utf-8")
    print(f"Wrote {GRAPH_PATH} ({len(graph['nodes'])} nodes, {len(graph['edges'])} edges)")


if __name__ == "__main__":
    main()
