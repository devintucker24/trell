---
id: wiki-usage-dashboard
title: Wiki-brain usage dashboard
type: meta
status: active
created: 2026-09-04
updated: 2026-09-05
tags: [usage, telemetry, context-cost]
domain: meta
summary: "Generated usage snapshot — retrieve tokens, hit quality, doctor score, usefulness index."
nodes:
  - id: wiki-usage-dashboard
    kind: concept
    label: Wiki usage dashboard
edges:
  - from: wiki-usage-dashboard
    to: wiki-usage-telemetry
    rel: implements
related:
  - "[[_system/docs/usage-telemetry]]"
  - "[[_system/docs/ROUTER]]"
agent:
  priority: medium
  read_when:
    - "checking whether the wiki brain is earning its context cost"
    - "tuning retrieve budgets"
  maintain:
    - "regenerate via python3 docs/wiki/_system/scripts/wiki_usage.py report"
---

# Wiki-brain usage dashboard

Generated `2026-09-05` from local `docs/wiki/_system/generated/usage/events.jsonl` (last **30** days).
Raw events are gitignored; this page is the shareable snapshot.

**Usefulness index:** 56.0/100

(heuristic: retrieval hit quality + last doctor score + activity − INDEX-dump admissions)

| Metric | Value |
|--------|------:|
| Events | 3 |
| Retrieves | 3 |
| Est. retrieve tokens (sum) | 3426 |
| Est. tokens / event with tokens | 1142 |
| Budget utilization | 37.2% |
| Mean top hit score | 0.479 |
| Weak-hit rate (top < 0.25) | 0.0 |
| Strong-hit rate (top ≥ 0.45) | 0.667 |
| Last doctor score | — |
| Repeat-query groups (≥3) | 0 |
| Dump admissions | 0 |
| Mean latency (ms) | 106.3 |
| Pages opened / cited | 0 / 0 |
| Citation overlap (cited ∩ opened / opened) | — |

## Ops mix

| op | count |
|----|------:|
| `retrieve` | 3 |

## Hottest retrieve pages

| path | hits |
|------|-----:|
| `FRAMEWORK.md` | 3 |
| `core/contract-and-guard-system.md` | 3 |
| `episodic/2026-09-04-graphify-machine-graph.md` | 2 |
| `ROUTER.md` | 2 |
| `_meta/brain-gap-analysis-2026-09-04.md` | 2 |
| `core/epistemic-foundations.md` | 2 |
| `applications/autonomous-physical-systems.md` | 2 |
| `_meta/CONTEXT_PROTOCOL.md` | 1 |

## How to read this

- **High tokens + low strong-hit rate** → retrieve is expensive and missing; add pages or fix ROUTER seeds.
- **Repeat-query groups** → same question keeps coming; the answer should be a first-class page.
- **Dump admissions** → agents skipped retrieve; tighten always-on rules.
- **Doctor score** → structural health, not usefulness. Both are needed.

See [[_system/docs/usage-telemetry]] for the metric catalog and agent logging protocol.
