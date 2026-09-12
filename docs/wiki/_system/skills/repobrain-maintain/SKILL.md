---
name: repobrain-maintain
description: Keep compiled RepoBrain claims in sync with host code and docs, regenerate the claim graph from frontmatter, and refresh the Graphify code graph after source changes.
---

# Skill: RepoBrain Maintain

## When to use
- Parser/lexer/runtime or equivalent host semantics change
- New examples added under `HOST.yaml` `code_roots`
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

Then update the compiled pages that declare those symbols in `implements_code`.
Do not paste source into wiki pages. Map paths from `HOST.yaml` `semantic_dirs`
and `docs/wiki/_system/config/router-seeds.md`, not from another host's corpus.

## Claim-graph regenerate
1. Scan wiki markdown with YAML frontmatter (skip `skills/`, `scripts/`, `pack/`).
2. Collect `nodes` / `edges` → `_system/generated/claim-graph.yaml`.
3. Deduplicate; set `updated`.

Helper: `python3 docs/wiki/_system/scripts/sync_graph.py`

## Always
- Bump page `updated`
- Append maintain log entry
- Run the host project's test command if claiming examples still compile

Operator manual: `docs/wiki/_system/docs/OPERATOR.md`
Router: `docs/wiki/_system/docs/ROUTER.md`
