---
name: repobrain-maintain
description: Keep compiled RepoBrain claims in sync with the compiler/examples/THESIS, regenerate the claim graph from frontmatter, and refresh the Graphify code graph after src changes.
---

# Skill: RepoBrain Maintain

## When to use
- Parser/lexer/typechecker/interpreter semantics change
- New `examples/*.trell` added
- Roadmap or market thesis updates
- Code moved/renamed (Graphify graph stale)
- Need to regenerate `_system/generated/claim-graph.yaml`

## Always (code changed)

```bash
./repobrain graph status
./repobrain graph sync
```

That **pulls** Graphify’s AST graph. Do not hand-edit `graphify-out/graph.json`. Do not rebuild a parallel call graph.
Use `sync --force` only to recover a corrupt or refactor-reduced graph. See
`docs/wiki/_system/docs/GRAPHIFY.md` for adapter diagnostics and supported versions.

Then update the few claim pages in the matrix below. Point at code with `implements_code:`; do not paste `src/` into wiki pages.

## Sync matrix

| Code / artifact change | Wiki pages to touch |
|------------------------|---------------------|
| Dual-track types | `core/epistemic-foundations`, `theory/epistemic-type-calculus` |
| Natural syntax keywords | `core/natural-syntax-specification`, README, THESIS §Natural |
| Fork/when semantics | `core/speculative-execution-engine` |
| Guards/contracts/quorum | `core/contract-and-guard-system` |
| New vertical example | matching `applications/*` + INDEX |
| Competitor landscape | `market/competitive-analysis` |

## Claim-graph regenerate
1. Scan wiki markdown with YAML frontmatter (skip `skills/`, `scripts/`, `pack/`).
2. Collect `nodes` / `edges` → `_system/generated/claim-graph.yaml`.
3. Deduplicate; set `updated`.

Helper: `python3 docs/wiki/_system/scripts/sync_graph.py`

## Always
- Bump page `updated`
- Append maintain log entry
- Run `cargo test` if claiming examples still compile

Operator manual: `docs/wiki/_system/docs/OPERATOR.md`
Router: `docs/wiki/_system/docs/ROUTER.md`
