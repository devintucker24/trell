---
id: context-protocol
title: Agent Context Protocol
type: meta
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [context-engineering, memory, agents, budgets, temporal, episodic, semantic]
domain: meta
summary: How agents assemble context from semantic, episodic, and temporal wiki memory under token budgets.
nodes:
  - id: context-protocol
    kind: concept
    label: Agent Context Protocol
  - id: memory-working
    kind: concept
    label: Working Memory
  - id: memory-temporal
    kind: concept
    label: Temporal Memory
edges:
  - from: context-protocol
    to: wiki-router
    rel: depends_on
  - from: context-protocol
    to: memory-temporal
    rel: depends_on
  - from: memory-temporal
    to: memory-episodic
    rel: related_to
  - from: memory-working
    to: wiki-router
    rel: depends_on
related:
  - "[[ROUTER]]"
  - "[[_meta/brain-gap-analysis-2026-09-04]]"
  - "[[episodic/INDEX]]"
  - "[[temporal/TIMELINE]]"
  - "[[SCHEMA]]"
agent:
  priority: critical
  read_when:
    - designing agent prompts or tool loops that use the wiki
    - debugging context bloat or missed recall
    - answering when/as-of/what-changed questions
  maintain:
    - keep budgets and packing rules aligned with retrieve script
---

# Agent Context Protocol

Turns the wiki into an efficient **file RAG + multi-lane memory** system — without dumping ~32k tokens every turn.

## Memory kinds

| Kind | Role | Location | Retrieval |
|---|---|---|---|
| **Semantic** | Stable concepts & claims | `core/`, `theory/`, `applications/`, `market/`, `roadmap/` | `wiki_retrieve` + graph hop |
| **Episodic** | What happened / decided / failed | `docs/wiki/episodic/` | recency + task/tag match |
| **Temporal** | When facts held; supersession; as-of | `temporal/TIMELINE.md` + `temporal:` frontmatter | `--as-of`, validity filter, timeline scan |
| **Procedural** | How to operate | `AGENTS.md`, `skills/wiki/*/SKILL.md` | by skill / intent |
| **Raw / provenance** | Immutable sources | `raw/`, `THESIS.md`, examples | via `sources` / `related` |
| **Working** | Current-task scratch | `episodic/session-current.md` (capped) | discard/consolidate after commit |

Inbox and raw notes are **not** certain until ingested. Episodes are not semantic truth until consolidated.

## Temporal memory (first-class)

Agents need clock-aware recall, not only “similar pages.”

### Page-level `temporal:` block (SCHEMA)

```yaml
temporal:
  observed_at: 2026-09-04      # when we learned / wrote this
  valid_from: 2026-09-04       # when the claim became true
  valid_until: null            # null = still valid; set when superseded
  supersedes: []               # page ids or node ids this replaces
  superseded_by: null          # set when a newer page wins
```

### Timeline index

`docs/wiki/temporal/TIMELINE.md` is append-oriented: dated events linking episodes, schema changes, ingest, and claim supersessions.

### As-of retrieval rules

1. Prefer pages where `valid_from ≤ as_of` and (`valid_until` is null or `> as_of`).
2. Down-rank or exclude `status: deprecated` / superseded pages unless the query asks for history.
3. For “what changed since X?”, scan TIMELINE between X and now, then retrieve linked pages.
4. Recency boost applies to episodic + TIMELINE; semantic pages use `updated:` lightly unless query is time-sensitive.

### Decay & consolidation

- Episodes older than ~30 days with no `promote: true` → candidate for summary → semantic merge, then archive.
- TIMELINE keeps the durable temporal spine even after episode bodies shrink.
- Doctor may flag pages with `valid_until` in the past still marked `status: active`.

## Assembly recipe (every agent turn)

```
1. ALWAYS: AGENTS.md (§2) + ROUTER.md + user task
2. IF time/as-of/changed: TIMELINE slice + retrieve --as-of
3. IF session/decision: last 1–3 matching episodes
4. SEED: ROUTER Tier-1 → open 1–3 pages
5. ELSE/ALSO: wiki_retrieve(query) → top-k under budget
6. OPTIONAL: one-hop GRAPH neighbors from the top hit only
7. NEVER: full INDEX, full SCHEMA, full GRAPH, full category dump
```

## Packing rules

1. Lead with citation (page id + path) before body.
2. Prefer section chunks over whole pages.
3. Cap quotes: ≤ 40 lines or ≤ 800 tokens per page unless editing that page.
4. Collapse duplicates: keep the canonical semantic page.
5. Episodic before semantic for “what did we decide?”; semantic before episodic for “what is Trell?”; temporal first for “when / as-of / changed.”
6. Code (`src/`) over wiki when asking how the compiler behaves *today*.

## Reranking (file-native)

| Signal | Weight | Meaning |
|---|---:|---|
| Lexical overlap | 0.42 | query terms in title/id/headings/body |
| Frontmatter boost | 0.15 | `agent.read_when` / tags / summary |
| Graph proximity | 0.13 | edge distance from seeds (gated by lexical) |
| Temporal fit | 0.15 | validity window vs `--as-of`; recency |
| Type prior | 0.05 | prefer concept/decision over raw |
| Diversity (MMR) | — | applied at selection time |

Embeddings optional when page count ≫ 200. Stay git-native until then.

## Budgets

| Slot | Tokens |
|---|---:|
| Always-on | ≤ 2,000 |
| Retrieved wiki | ≤ 4,000 |
| Working scratch | ≤ 2,000 |
| Episodic + temporal | ≤ 1,500 |
| **Wiki-derived total** | **≤ ~9,500** |

## Failure modes prevented

- Context stuffing · lost-in-the-middle · false memory (inbox-as-truth)
- Amnesia (no episodes) · **temporal confusion** (stale claim treated as current)
- Skill blindness (no retrieve path)

## Related

- `docs/wiki/ROUTER.md`
- `docs/wiki/_meta/brain-gap-analysis-2026-09-04.md`
- `skills/wiki/retrieve/SKILL.md`
- `docs/wiki/episodic/` · `docs/wiki/temporal/`
