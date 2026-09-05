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

**Usefulness index:** 53.5/100

(heuristic: retrieval hit quality + last doctor score + activity − INDEX-dump admissions)

| Metric | Value |
|--------|------:|
| Events | 7 |
| Retrieves | 6 |
| Est. retrieve tokens (sum) | 7031 |
| Est. tokens / event with tokens | 1171.8 |
| Budget utilization | 35.8% |
| Mean top hit score | 0.417 |
| Weak-hit rate (top < 0.25) | 0.0 |
| Strong-hit rate (top ≥ 0.45) | 0.333 |
| Last doctor score | 100.0 |
| Repeat-query groups (≥3) | 0 |
| Dump admissions | 0 |
| Mean latency (ms) | 146.3 |
| Pages opened / cited | 0 / 0 |
| Citation overlap (cited ∩ opened / opened) | — |

## Graphify adapter

| Signal | Value |
|---|---|
| CLI | 0.9.54 |
| Artifact | ready |
| Nodes / edges | 151 / 486 |
| Source freshness | fresh |
| Visualization | fresh |

## Ops mix

| op | count |
|----|------:|
| `doctor` | 1 |
| `retrieve` | 6 |

## Hottest retrieve pages

| path | hits |
|------|-----:|
| `core/epistemic-foundations.md` | 5 |
| `episodic/2026-09-04-graphify-machine-graph.md` | 4 |
| `INDEX.md` | 4 |
| `core/contract-and-guard-system.md` | 4 |
| `FRAMEWORK.md` | 3 |
| `applications/autonomous-physical-systems.md` | 3 |
| `applications/security-cloud-and-governance.md` | 3 |
| `core/natural-syntax-specification.md` | 3 |

## How to read this

- **High tokens + low strong-hit rate** → retrieve is expensive and missing; add pages or fix ROUTER seeds.
- **Repeat-query groups** → same question keeps coming; the answer should be a first-class page.
- **Dump admissions** → agents skipped retrieve; tighten always-on rules.
- **Doctor score** → structural health, not usefulness. Both are needed.

See [[_system/docs/usage-telemetry]] for the metric catalog and agent logging protocol.
