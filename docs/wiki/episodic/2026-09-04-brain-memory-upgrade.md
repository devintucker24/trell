---
id: 2026-09-04-brain-memory-upgrade
title: "Episode: Brain memory upgrade (RAG + temporal)"
type: episode
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [episodic, memory, rag, temporal, context-engineering]
domain: episodic
summary: Gap analysis and implementation of router, episodic, temporal, and retrieve for the wiki brain.
nodes:
  - id: episode-2026-09-04-brain-memory
    kind: concept
    label: Brain memory upgrade episode
edges:
  - from: episode-2026-09-04-brain-memory
    to: memory-episodic
    rel: related_to
  - from: episode-2026-09-04-brain-memory
    to: memory-temporal
    rel: related_to
  - from: episode-2026-09-04-brain-memory
    to: brain-gap-analysis
    rel: depends_on
related:
  - "[[_meta/brain-gap-analysis-2026-09-04]]"
  - "[[ROUTER]]"
  - "[[_meta/CONTEXT_PROTOCOL]]"
  - "[[temporal/TIMELINE]]"
temporal:
  observed_at: 2026-09-04
  valid_from: 2026-09-04
  valid_until: null
  supersedes: []
  superseded_by: null
agent:
  priority: high
  read_when:
    - continuing wiki memory / RAG work
    - explaining why ROUTER and temporal exist
  maintain:
    - mark stale after consolidation into semantic meta pages
episode:
  goal: "Upgrade wiki to file RAG with episodic + temporal memory and efficient context engineering"
  outcome: partial
  promote: true
---

# Episode: Brain memory upgrade (RAG + temporal)

## Goal
Audit the Karpathy-style wiki as agent memory; close gaps vs 2026 research (episodic, semantic, temporal, rerank, progressive disclosure).

## Actions
- Wrote gap analysis `_meta/brain-gap-analysis-2026-09-04.md`
- Added `ROUTER.md` + `_meta/CONTEXT_PROTOCOL.md`
- Added `episodic/` + `temporal/TIMELINE.md`
- Added `skills/wiki/retrieve` + hybrid lexical/graph/temporal rerank script
- User follow-up: explicitly require **temporal memory**

## Outcome
Partial → landing in same change set: schema wires, doctor, commit.

## Lessons
1. Always-on context must be a slim router — not INDEX+SCHEMA every turn.
2. Episodic ≠ log.md ops lines; need narrative goal/outcome/lessons.
3. Temporal needs validity windows + timeline spine + as-of retrieve, not only `updated:` dates.
4. File-native hybrid retrieve is enough until ≫200 pages.

## Open threads
- Eval harness golden queries
- Optional embeddings backend later
- Doctor check for expired `valid_until` still active

## Citations
- [[_meta/brain-gap-analysis-2026-09-04]]
- [[ROUTER]]
- [[_meta/CONTEXT_PROTOCOL]]
