---
id: temporal-timeline
title: Temporal Memory Timeline
type: meta
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [temporal, memory, timeline, as-of]
domain: temporal
summary: Chronological spine for as-of recall — when claims, episodes, and schema events occurred or were superseded.
nodes:
  - id: memory-temporal
    kind: concept
    label: Temporal Memory
  - id: temporal-timeline
    kind: concept
    label: Wiki Timeline
edges:
  - from: temporal-timeline
    to: memory-temporal
    rel: implements
  - from: memory-temporal
    to: memory-episodic
    rel: related_to
  - from: memory-temporal
    to: context-protocol
    rel: depends_on
  - from: memory-temporal
    to: wiki-router
    rel: related_to
related:
  - "[[ROUTER]]"
  - "[[_meta/CONTEXT_PROTOCOL]]"
  - "[[episodic/INDEX]]"
  - "[[log]]"
agent:
  priority: high
  read_when:
    - answering when / as-of / what-changed questions
    - consolidating or superseding claims
  maintain:
    - append on every episode, ingest, schema change, and claim supersession
temporal:
  observed_at: 2026-09-04
  valid_from: 2026-09-04
  valid_until: null
  supersedes: []
  superseded_by: null
---

# Temporal Memory — Timeline

Append-only chronological spine. `log.md` records **ops**; this file records **time-relevant knowledge events** for as-of retrieval.

Format per line under a day heading:

```text
- HH:MM | kind | subject | note | page-wikilink
```

Example: `- 14:00 | episode | brain upgrade | router+temporal | episodic/2026-09-04-brain-memory-upgrade`

`kind` ∈ `episode` · `ingest` · `schema` · `supersede` · `decision` · `health` · `release`

---

## 2026-09-04

- — | schema | wiki brain bootstrap | AGENTS + INDEX + SCHEMA + GRAPH | [[SCHEMA]]
- — | ingest | raw pointers | thesis/examples/market research | [[raw/thesis]]
- — | health | doctor→heal | score 72.8 → 100; orphan links | [[_meta/heal-2026-09-04]]
- — | schema | inbox+triage | taxonomy gate | [[inbox/README]]
- — | decision | memory architecture | file RAG; progressive disclosure; no required vectors yet | [[_meta/brain-gap-analysis-2026-09-04]]
- — | episode | brain memory upgrade | router + episodic + temporal + retrieve | [[episodic/2026-09-04-brain-memory-upgrade]]
- — | schema | agent project setup | thin AGENTS.md + CLAUDE.md + .cursor skills/rules | [[OPERATOR]]
- — | schema | temporal lane | `temporal:` frontmatter + domain/type episode | [[SCHEMA]]

---

## How agents use this

1. **As-of query:** find the last TIMELINE day ≤ `--as-of`, note active pages, then `wiki_retrieve --as-of`.
2. **What changed since D?:** read headings after D; open linked pages only.
3. **Supersession:** when replacing a claim, append `supersede` and set both pages’ `temporal.supersedes` / `superseded_by`.
4. Do not paste the entire timeline into context — slice the relevant date range (≤ ~40 lines).
