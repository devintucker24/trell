---
id: episodic-index
title: Episodic Memory Index
type: meta
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [episodic, memory, sessions, decisions]
domain: episodic
summary: Catalog of session episodes — decisions, failures, corrections — for agent continuity.
nodes:
  - id: memory-episodic
    kind: concept
    label: Episodic Memory
edges:
  - from: memory-episodic
    to: memory-temporal
    rel: related_to
  - from: memory-episodic
    to: wiki-router
    rel: depends_on
  - from: memory-episodic
    to: context-protocol
    rel: depends_on
related:
  - "[[ROUTER]]"
  - "[[temporal/TIMELINE]]"
  - "[[_meta/CONTEXT_PROTOCOL]]"
  - "[[episodic/session-current]]"
agent:
  priority: high
  read_when:
    - recalling prior decisions or session outcomes
    - starting work that continues a previous thread
  maintain:
    - list new episodes here
    - consolidate old episodes into semantic pages
---

# Episodic Memory

Narrative memory of **what happened** — not the compiled thesis. Cite episodes for continuity; promote durable facts into semantic pages before treating them as wiki truth.

## How to write an episode

1. Copy `episodic/_TEMPLATE.md` → `episodic/YYYY-MM-DD-<slug>.md`
2. Fill goal, actions, outcome, lessons, open threads
3. Set `temporal.observed_at` / `valid_from`
4. Append a line to `temporal/TIMELINE.md`
5. Log: `## [YYYY-MM-DD] episodic | <slug>`

## Active episodes

| Date | Page | Salience |
|---|---|---|
| 2026-09-04 | [[episodic/2026-09-04-brain-memory-upgrade]] | high — memory architecture |
| 2026-09-04 | [[episodic/2026-09-04-graphify-machine-graph]] | high — Graphify code graph + wiki-setup |

## Session scratch

- [[episodic/session-current]] — writable working memory (hard size cap; reset after consolidate)

## Consolidation rule

When an episode’s lessons are durable:

1. Merge facts into the right `core/` / `theory/` / `applications/` / `_meta/` page
2. Set episode `status: stale` or archive note in body
3. Keep TIMELINE entry (temporal spine survives)
4. Never delete the episode file without a TIMELINE pointer
