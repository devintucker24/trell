---
name: wiki-maintain
description: Keep the Trell wiki synchronized with the compiler, examples, THESIS, and GRAPH.yaml. Use after language changes, new examples, or roadmap updates.
---

# Skill: Wiki Maintain

## When to use
- Parser/lexer/typechecker/interpreter semantics change
- New `examples/*.trell` added
- Roadmap or market thesis updates
- Need to regenerate GRAPH.yaml from page frontmatter

## Sync matrix

| Code / artifact change | Wiki pages to touch |
|------------------------|---------------------|
| Dual-track types | `core/epistemic-foundations`, `theory/epistemic-type-calculus` |
| Natural syntax keywords | `core/natural-syntax-specification`, README, THESIS §Natural |
| Fork/when semantics | `core/speculative-execution-engine` |
| Guards/contracts/quorum | `core/contract-and-guard-system` |
| New vertical example | matching `applications/*` + INDEX |
| Competitor landscape | `market/competitive-analysis` |

## GRAPH regenerate algorithm
1. Scan all `docs/wiki/**/*.md` with YAML frontmatter.
2. Collect `nodes` → GRAPH.yaml `nodes` (include `page:` relative path).
3. Collect `edges` → GRAPH.yaml `edges`.
4. Deduplicate by `(id)` for nodes and `(from,to,rel)` for edges.
5. Set `updated` date.

Optional helper: `skills/wiki/scripts/sync_graph.py`

## Always
- Bump page `updated`
- Append maintain log entry
- Run `cargo test` if claiming examples still compile
