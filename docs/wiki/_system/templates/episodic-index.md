---
id: episodic-index
title: Episodic Memory Index
type: meta
status: active
created: 2026-09-12
updated: 2026-09-12
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
related:
  - "[[temporal/TIMELINE]]"
  - "[[_system/docs/CONTEXT_PROTOCOL]]"
  - "[[_system/docs/ROUTER]]"
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

Episodes are **not** compiled wiki truth. They are session notes until
triage → ingest promotes lessons onto semantic pages.

Copy `_system/templates/episode.md` to `docs/wiki/episodic/YYYY-MM-DD-<slug>.md`
and link it from this index after the episode exists.

| Date | Episode | Outcome |
|------|---------|---------|
| _(none yet)_ | — | — |
