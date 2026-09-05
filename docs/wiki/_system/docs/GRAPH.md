---
id: wiki-graph-overview
title: Machine graph protocol (Graphify + claim index)
type: meta
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [graph, meta, navigation, graphify]
domain: meta
summary: "Agent/machine protocol: Graphify owns the code graph; GRAPH.yaml is a compiled claim index. Not a human map."
nodes:
  - id: wiki-graph-overview
    kind: concept
  - id: wiki-graphify-bridge
    kind: concept
    label: Graphify code-graph bridge
edges:
  - from: wiki-graph-overview
    to: wiki-index
    rel: depends_on
  - from: wiki-graphify-bridge
    to: wiki-graph-overview
    rel: implements
related:
  - "[[INDEX]]"
  - "[[SCHEMA]]"
  - "[[FRAMEWORK]]"
  - "[[GRAPHIFY]]"
agent:
  priority: high
  read_when:
    - "understanding the knowledge graph"
    - "code vs claim questions"
    - "debugging orphan nodes"
  maintain:
    - "keep Graphify vs GRAPH.yaml split honest"
---

# Machine graph protocol

These files are for **agents and scripts**. Do not render them as a human product surface. Do not paste `GRAPH.yaml` or `graph.json` into context.

## Code graph (pull from Graphify)

Path: `graphify-out/graph.json` (gitignored).

```bash
./repobrain graph sync
./repobrain graph status --json
./repobrain graph query "who uses TypeChecker"
./repobrain graph path Parser TypeChecker
./repobrain graph explain TypeChecker
./repobrain graph affected TypeChecker
./repobrain graph god-nodes
```

Supported dependency, configuration, recovery, and ownership:
[`GRAPHIFY.md`](GRAPHIFY.md).

Every Graphify edge is `EXTRACTED`, `INFERRED`, or `AMBIGUOUS`. Prefer EXTRACTED when asserting what the compiler does.

Optional: `repobrain graph export-wiki` → `graphify-out/wiki/` (regenerated community articles; **not** RepoBrain doctrine).

## Claim graph (compiled from pages)

Path: [`GRAPH.yaml`](GRAPH.yaml)

```bash
python3 docs/wiki/_system/scripts/sync_graph.py
```

This index exists so retrieve can hop `reduces_via` / `contradicts` without parsing every page. Source of truth for those rels is **page frontmatter**, not this dump.

## Claim hubs (this host, for one-hop recipes)

| Node | Kind | Why it matters |
|------|------|----------------|
| `belief-type` | type | Center of epistemic system |
| `certain-type` | type | Grounded track |
| `guard-verify` | primitive | Only legal reduction path |
| `speculative-execution` | engine | when/fork runtime |
| `natural-trell-syntax` | primitive | Surface syntax |
| `three-beat-safety-pattern` | concept | Universal application pattern |
| `ten-year-vision` | concept | Strategic north star |
| `phase-4-iso-silicon` | phase | Hardware endgame |

Edge vocabulary: [[SCHEMA]] — `depends_on`, `implements`, `reduces_via`, `enforces`, `extends`, `applies_to`, `competes_with`, `accelerates`, `regulated_by`, `owned_by`, `milestone_of`, `contradicts`, `related_to`.

## Agent recipes

1. **Doctrine:** retrieve wiki → open 1–3 pages. Example hop: `belief-type` → `reduces_via` → `certain-type`.
2. **Compiler wiring:** Graphify query/path. Example: `Type` in `ast.rs` → `TypeChecker` in `typecheck.rs`.
3. **Do not** cite Graphify INFERRED edges as Trell thesis. Do not cite seed pages (`graphify-seed`) as truth.
