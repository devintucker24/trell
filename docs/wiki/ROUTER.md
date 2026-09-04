---
id: wiki-router
title: Wiki Context Router
type: meta
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [router, context-engineering, progressive-disclosure, memory, temporal]
domain: meta
summary: Progressive-disclosure router — tiny always-on map; retrieve semantic, episodic, and temporal memory on demand.
nodes:
  - id: wiki-router
    kind: concept
    label: Wiki Context Router
edges:
  - from: wiki-router
    to: wiki-index
    rel: related_to
  - from: wiki-router
    to: context-protocol
    rel: depends_on
  - from: wiki-router
    to: memory-temporal
    rel: depends_on
related:
  - "[[_meta/CONTEXT_PROTOCOL]]"
  - "[[_meta/brain-gap-analysis-2026-09-04]]"
  - "[[host/router-seeds]]"
  - "[[FRAMEWORK]]"
agent:
  priority: critical
  read_when:
    - starting any wiki-related task
    - deciding what to load into context
    - after triage before deep reading
  maintain:
    - keep Tier-1 seed table aligned with real wiki paths
    - bump budgets if corpus grows past ~200 pages
---

# Wiki Context Router

Agents must **not** dump the wiki into context. This file is the always-on map; deepen only on demand.

## Tier 0 — Always on (~1–2k tokens)

Load only:

1. `AGENTS.md` (project brief) — Claude Code: also `CLAUDE.md`
2. This file (`docs/wiki/ROUTER.md`)
3. Current task intent (user message / issue)

Do **not** auto-load full `INDEX.md`, full `SCHEMA.md`, full `OPERATOR.md`, or concept folders.

## Tier 1 — Intent → seed pages

**Pack-generic** (always):

| Intent signal | Seed pages (open these first) |
|---|---|
| wiki / memory / RAG / retrieval / context / pack / export | `FRAMEWORK.md`, `_meta/CONTEXT_PROTOCOL.md`, `_meta/usage-telemetry.md` |
| ingest / triage / inbox | `inbox/README.md` → run triage/ingest skills |
| health / orphans / doctor / heal | run `wiki-doctor` (then `wiki-heal` if score < 95) |
| usage / tokens / context cost / telemetry | `_meta/usage-telemetry.md` + `wiki_usage.py report` |
| session / decisions / what happened / episodic | `episodic/INDEX.md` + latest episode |
| timeline / when / as-of / what changed / temporal | `temporal/TIMELINE.md` + retrieve with `--as-of` |

**Host-specific:** read `docs/wiki/host/router-seeds.md` (path in `HOST.yaml` `router_seeds`).

After seeding, expand **one hop** via frontmatter `edges` or `GRAPH.yaml` — not whole categories.

## Tier 2 — Retrieve + rerank (+ temporal)

```bash
python3 docs/wiki/scripts/wiki_retrieve.py "<query>" --budget-tokens 3500
# time-aware:
python3 docs/wiki/scripts/wiki_retrieve.py "<query>" --as-of 2026-09-04 --budget-tokens 3500
```

See `docs/wiki/skills/wiki-retrieve/SKILL.md`. Open **only** top hits that fit budget.

## Tier 3 — Deep read (cite + update)

Edit with frontmatter discipline; update `log.md`; prefer doctor/heal over ad-hoc graph edits.

## Memory lanes (which store?)

| Need | Lane | Where |
|---|---|---|
| Stable “what is true” | Semantic | dirs in `HOST.yaml` `semantic_dirs` |
| “What we did / decided / failed” | Episodic | `episodic/` |
| “When / as-of / what superseded what” | Temporal | `temporal/TIMELINE.md` + page `temporal:` fields |
| “How to operate” | Procedural | `AGENTS.md`, `docs/wiki/skills/` |

## Hard budget defaults

| Slot | Token budget |
|---|---:|
| Always-on | ≤ 2,000 |
| Retrieved wiki | ≤ 4,000 |
| Working scratch | ≤ 2,000 |
| Episodic + temporal recall | ≤ 1,500 |
| **Wiki-derived total** | **≤ ~9,500** |

## Anti-patterns

- Loading `INDEX.md` “just in case”
- Dumping a whole domain folder
- Pasting full `GRAPH.yaml`
- Citing inbox / unconsolidated episodes as semantic truth
- Ignoring `valid_until` / superseded pages (stale temporal memory)

## Next-action cheat sheet

| Need | Do |
|---|---|
| Find knowledge | `retrieve` → open top hits |
| Add knowledge | `inbox/` → triage → ingest |
| Remember a decision | write `episodic/YYYY-MM-DD-<slug>.md` + append TIMELINE |
| Ask “as of date” | `retrieve --as-of` + TIMELINE |
| Fix health | doctor → heal |
