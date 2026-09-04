---
id: wiki-graph-overview
title: Knowledge Graph Overview
type: meta
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [graph, meta, navigation]
domain: meta
summary: "Human-readable map of GRAPH.yaml hubs, edge types, and traversal tips for agents."
nodes:
  - id: wiki-graph-overview
    kind: concept
edges:
  - from: wiki-graph-overview
    to: wiki-index
    rel: depends_on
related:
  - "[[INDEX]]"
  - "[[SCHEMA]]"
agent:
  priority: high
  read_when:
    - "understanding the knowledge graph"
    - "debugging orphan nodes"
  maintain:
    - "regenerate stats after sync_graph.py"
---

# Knowledge Graph Overview

Machine-readable graph: [`_meta/GRAPH.yaml`](_meta/GRAPH.yaml)

Regenerate:
```bash
python3 skills/wiki/scripts/sync_graph.py
```

## Hub Nodes (high connectivity)

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

## Edge Vocabulary

See [[SCHEMA]] — `depends_on`, `implements`, `reduces_via`, `enforces`, `extends`, `applies_to`, `competes_with`, `accelerates`, `regulated_by`, `owned_by`, `milestone_of`, `contradicts`, `related_to`.

## Agent Traversal Recipes

1. **Explain Trell in one hop:** `belief-type` → `reduces_via` → `certain-type` via `guard-verify`
2. **Industry map:** `three-beat-safety-pattern` → `applies_to` → `app-*`
3. **Competition:** `natural-trell-syntax` → `competes_with` → `comp-*`
4. **Future tech:** `phase-4-iso-silicon` ← `milestone_of` ← `tech-npu-semantic-branching`
