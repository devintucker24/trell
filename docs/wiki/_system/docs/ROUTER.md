---
id: wiki-router
title: RepoBrain Context Router
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
    label: RepoBrain Context Router
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

# RepoBrain Context Router

Agents must **not** dump the wiki into context. This file is the always-on map; deepen only on demand.

## Tier 0 — Always on (~1–2k tokens)

Load only:

1. `AGENTS.md` (project brief) — Claude Code: also `CLAUDE.md`
2. This file (`docs/wiki/_system/docs/ROUTER.md`)
3. Current task intent (user message / issue)

Do **not** auto-load full `INDEX.md`, full `SCHEMA.md`, full `OPERATOR.md`, or concept folders.

## Tier 1 — Intent → seed pages

**Pack-generic** (always):

| Intent signal | Seed pages (open these first) |
|---|---|
| wiki / memory / RAG / retrieval / context / pack / export / setup / graphify | `FRAMEWORK.md`, `GRAPH.md`, `CONTEXT_PROTOCOL.md` |
| ingest / triage / inbox | `inbox/README.md` → run triage/ingest skills |
| health / orphans / doctor / heal | run `repobrain-doctor` (then `repobrain-heal` if score < 95) |
| usage / tokens / context cost / telemetry | `usage-telemetry.md` + `wiki_usage.py report` |
| session / decisions / what happened / episodic | `episodic/INDEX.md` + latest episode |
| timeline / when / as-of / what changed / temporal | `temporal/TIMELINE.md` + retrieve with `--as-of` |

**Host-specific:** read `docs/wiki/_system/config/router-seeds.md`.

After seeding, expand **one hop** via frontmatter `edges` (claim graph) — not whole categories. For code wiring use Graphify (`wiki_graphify.py query`), not `GRAPH.yaml`.

## Tier 2 — Retrieve + rerank (+ temporal)

```bash
./repobrain retrieve "<query>" --budget-tokens 3500
# time-aware:
./repobrain retrieve "<query>" --as-of 2026-09-04 --budget-tokens 3500
# compiler/structure (Graphify):
./repobrain graph query "<query>"
```

See `docs/wiki/_system/skills/repobrain-retrieve/SKILL.md`.

## Tier 3 — Deep read (cite + update)

Edit with frontmatter discipline; update `log.md`; prefer doctor/heal over ad-hoc graph edits.

## Memory lanes (which store?)

| Need | Lane | Where |
|---|---|---|
| Stable “what is true” | Semantic | dirs in `HOST.yaml` `semantic_dirs` |
| “What we did / decided / failed” | Episodic | `episodic/` |
| “When / as-of / what superseded what” | Temporal | `temporal/TIMELINE.md` + page `temporal:` fields |
| “How to operate” | Procedural | `AGENTS.md`, `docs/wiki/_system/skills/` |
| “Who calls / where defined” | Code graph | `graphify-out/graph.json` via `wiki_graphify.py` |

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
- Pasting full `GRAPH.yaml` or `graph.json`
- Treating Graphify `--wiki` articles or `graphify-seed` drafts as compiled thesis
- Citing inbox / unconsolidated episodes as semantic truth
- Ignoring `valid_until` / superseded pages (stale temporal memory)

## Next-action cheat sheet

| Need | Do |
|---|---|
| New clone / empty host | `repobrain-setup` |
| Find knowledge | `retrieve` → open top hits |
| Code wiring | `wiki_graphify.py query` |
| Add knowledge | `inbox/` → triage → ingest |
| Remember a decision | write `episodic/YYYY-MM-DD-<slug>.md` + append TIMELINE |
| Ask “as of date” | `retrieve --as-of` + TIMELINE |
| Fix health | doctor → heal |
