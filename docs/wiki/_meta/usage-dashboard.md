---
id: wiki-usage-dashboard
title: Wiki-brain usage dashboard
type: meta
status: active
created: 2026-09-04
updated: 2026-09-04
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
  - "[[_meta/usage-telemetry]]"
  - "[[ROUTER]]"
agent:
  priority: medium
  read_when:
    - "checking whether the wiki brain is earning its context cost"
    - "tuning retrieve budgets"
  maintain:
    - "regenerate via python3 docs/wiki/scripts/wiki_usage.py report"
---

# Wiki-brain usage dashboard

Generated `2026-09-04` from local `docs/wiki/_meta/usage/events.jsonl` (last **30** days).
Raw events are gitignored; this page is the shareable snapshot.

**Usefulness index:** 81.0/100  
(heuristic: retrieval hit quality + last doctor score + activity − INDEX-dump admissions)

| Metric | Value |
|--------|------:|
| Events | 2 |
| Retrieves | 1 |
| Est. retrieve tokens (sum) | 1228 |
| Est. tokens / event with tokens | 1228 |
| Budget utilization | 61.4% |
| Mean top hit score | 0.853 |
| Weak-hit rate (top < 0.25) | 0.0 |
| Strong-hit rate (top ≥ 0.45) | 1.0 |
| Last doctor score | 100.0 |
| Repeat-query groups (≥3) | 0 |
| Dump admissions | 0 |
| Mean latency (ms) | 98 |
| Pages opened / cited | 0 / 0 |
| Citation overlap (cited ∩ opened / opened) | — |

## Ops mix

| op | count |
|----|------:|
| `doctor` | 1 |
| `retrieve` | 1 |

## Hottest retrieve pages

| path | hits |
|------|-----:|
| `FRAMEWORK.md` | 3 |
| `INDEX.md` | 1 |
| `_meta/CONTEXT_PROTOCOL.md` | 1 |
| `host/router-seeds.md` | 1 |
| `OPERATOR.md` | 1 |
| `_meta/usage-telemetry.md` | 1 |

## How to read this

- **High tokens + low strong-hit rate** → retrieve is expensive and missing; add pages or fix ROUTER seeds.
- **Repeat-query groups** → same question keeps coming; the answer should be a first-class page.
- **Dump admissions** → agents skipped retrieve; tighten always-on rules.
- **Doctor score** → structural health, not usefulness. Both are needed.

See [[_meta/usage-telemetry]] for the metric catalog and agent logging protocol.
