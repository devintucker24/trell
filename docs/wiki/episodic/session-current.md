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
- Memory upgrade landed: ROUTER, CONTEXT_PROTOCOL, episodic, temporal, retrieve+rerank.

## Decisions (this session)
- Stay file-first (no required vector DB at ~35 pages).
- Progressive disclosure: ROUTER always-on; INDEX on demand.
- Temporal is first-class: `temporal:` frontmatter + TIMELINE + `--as-of` retrieve.

## Blockers
- None.

## Next
- Optional: automate `eval-queries.yaml` scoring script.
- Clear this scratch after PR merges / next session starts.
