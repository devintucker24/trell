---
id: wiki-usage-dashboard
title: RepoBrain usage dashboard
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
    label: RepoBrain usage dashboard
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
    - "checking whether RepoBrain is earning its context cost"
    - "tuning retrieve budgets"
  maintain:
    - "regenerate via ./repobrain usage report"
---

# RepoBrain usage dashboard

Generated `2026-09-05` from local `docs/wiki/_system/generated/usage/events.jsonl` (last **30** days).
Raw events are gitignored; this page is the shareable snapshot.

**Usefulness index:** 60.0/100

(heuristic: retrieval hit quality + last doctor score + activity − INDEX-dump admissions)

| Metric | Value |
|--------|------:|
| Events | 5 |
| Retrieves | 4 |
| Est. retrieve tokens (sum) | 4661 |
| Est. tokens / event with tokens | 1165.2 |
| Budget utilization | 36.7% |
| Mean top hit score | 0.437 |
| Weak-hit rate (top < 0.25) | 0.0 |
| Strong-hit rate (top ≥ 0.45) | 0.5 |
| Last doctor score | 100.0 |
| Repeat-query groups (≥3) | 0 |
| Dump admissions | 0 |
| Mean latency (ms) | 120.5 |
| Pages opened / cited | 0 / 0 |
| Citation overlap (cited ∩ opened / opened) | — |

## Ops mix

| op | count |
|----|------:|
| `doctor` | 1 |
| `retrieve` | 4 |

## Hottest retrieve pages

| path | hits |
|------|-----:|
| `FRAMEWORK.md` | 3 |
| `core/contract-and-guard-system.md` | 3 |
| `core/epistemic-foundations.md` | 3 |
| `applications/autonomous-physical-systems.md` | 3 |
| `episodic/2026-09-04-graphify-machine-graph.md` | 2 |
| `INDEX.md` | 2 |
| `ROUTER.md` | 2 |
| `_meta/brain-gap-analysis-2026-09-04.md` | 2 |

## How to read this

- **High tokens + low strong-hit rate** → retrieve is expensive and missing; add pages or fix ROUTER seeds.
- **Repeat-query groups** → same question keeps coming; the answer should be a first-class page.
- **Dump admissions** → agents skipped retrieve; tighten always-on rules.
- **Doctor score** → structural health, not usefulness. Both are needed.

See [[_system/docs/usage-telemetry]] for the metric catalog and agent logging protocol.
