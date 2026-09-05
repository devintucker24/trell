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

**Usefulness index:** 82.5/100  
(heuristic: retrieval hit quality + last doctor score + activity − INDEX-dump admissions)

| Metric | Value |
|--------|------:|
| Events | 5 |
| Retrieves | 2 |
| Est. retrieve tokens (sum) | 2257 |
| Est. tokens / event with tokens | 1128.5 |
| Budget utilization | 45.4% |
| Mean top hit score | 0.845 |
| Weak-hit rate (top < 0.25) | 0.0 |
| Strong-hit rate (top ≥ 0.45) | 1.0 |
| Last doctor score | 100.0 |
| Repeat-query groups (≥3) | 0 |
| Dump admissions | 0 |
| Mean latency (ms) | 102.5 |
| Pages opened / cited | 0 / 0 |
| Citation overlap (cited ∩ opened / opened) | — |

## Ops mix

| op | count |
|----|------:|
| `doctor` | 3 |
| `retrieve` | 2 |

## Hottest retrieve pages

| path | hits |
|------|-----:|
| `FRAMEWORK.md` | 5 |
| `INDEX.md` | 2 |
| `OPERATOR.md` | 2 |
| `_meta/usage-telemetry.md` | 2 |
| `_meta/CONTEXT_PROTOCOL.md` | 1 |
| `host/router-seeds.md` | 1 |
| `temporal/TIMELINE.md` | 1 |
| `inbox/README.md` | 1 |

## How to read this

- **High tokens + low strong-hit rate** → retrieve is expensive and missing; add pages or fix ROUTER seeds.
- **Repeat-query groups** → same question keeps coming; the answer should be a first-class page.
- **Dump admissions** → agents skipped retrieve; tighten always-on rules.
- **Doctor score** → structural health, not usefulness. Both are needed.

See [[_meta/usage-telemetry]] for the metric catalog and agent logging protocol.
