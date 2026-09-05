---
id: session-current
title: Current Session Scratch
type: episode
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [episodic, working-memory, scratch]
domain: episodic
summary: Capped working-memory scratch for the active agent session; not semantic truth.
nodes:
  - id: session-scratch
    kind: concept
    label: Session Scratch
edges:
  - from: session-scratch
    to: memory-episodic
    rel: related_to
related:
  - "[[episodic/INDEX]]"
  - "[[ROUTER]]"
temporal:
  observed_at: 2026-09-04
  valid_from: 2026-09-04
  valid_until: null
  supersedes: []
  superseded_by: null
agent:
  priority: medium
  read_when:
    - resuming an in-progress agent session
  maintain:
    - clear or archive after episode file is written
episode:
  goal: "Active session working memory"
  outcome: partial
  promote: false
---

# Session Current

> **Hard cap:** keep body under ~800 tokens. Overflow → write a dated episode and reset.

## Now
- Graphify is the code graph; wiki-setup stands up new hosts.

## Decisions (this session)
- Stay file-first (no required vector DB at ~35 pages).
- Progressive disclosure: ROUTER always-on; INDEX on demand.
- Temporal is first-class: `temporal:` frontmatter + TIMELINE + `--as-of` retrieve.
- Do not roll a homegrown AST graph — pull Graphify. GRAPH.yaml = claim index only.
- Corpus doctrine is authored; Graphify wiki/seeds are structure/drafts.

## Blockers
- None.

## Next
- Optional: `_system/` folder move (grilling).
- Clear this scratch after PR merges / next session starts.
