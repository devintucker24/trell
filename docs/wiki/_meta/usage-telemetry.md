---
id: wiki-usage-telemetry
title: Wiki-brain usage telemetry
type: meta
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [usage, telemetry, context-cost, retrieval, rag]
domain: meta
summary: "What the wiki logs: retrieve tokens, hit quality, doctor score, dumps, citations, and a heuristic usefulness index."
nodes:
  - id: wiki-usage-telemetry
    kind: concept
    label: Wiki usage telemetry
edges:
  - from: wiki-usage-telemetry
    to: wiki-router
    rel: related_to
  - from: wiki-usage-telemetry
    to: wiki-brain-pack
    rel: depends_on
related:
  - "[[FRAMEWORK]]"
  - "[[ROUTER]]"
  - "[[_meta/CONTEXT_PROTOCOL]]"
agent:
  priority: medium
  read_when:
    - "designing or reading wiki usage metrics"
    - "reducing context cost"
  maintain:
    - "keep event fields aligned with wiki_usage.py"
---

# Wiki-brain usage telemetry

Goal: see whether the brain is **used**, whether retrieve **hits**, and what it **costs** in tokens — without putting PII in git by default.

## Stores

| Store | Path | Git? |
|-------|------|------|
| Event log | `_meta/usage/events.jsonl` | **ignored** (local / CI artifact) |
| Dashboard | `_meta/usage-dashboard.md` | commit when you want a snapshot |
| Ops log | `log.md` | append-only human ops (`usage` prefix ok) |

## Automatic events (scripts)

| `op` | Source | Fields |
|------|--------|--------|
| `retrieve` | `wiki_retrieve.py` | `query`, `lane`, `as_of`, `hits`, `tokens_est`, `budget_tokens`, `top_score`, `duration_ms`, `hit_paths` |
| `doctor` | `wiki_doctor.py` | `doctor_score`, `hits` (finding count) |

`--no-log` disables this.

## Agent-logged events (protocol)

| `op` | When |
|------|------|
| `query` | After answering from wiki; set `pages_opened`, `cited`, `tokens_est` |
| `navigate` `triage` `ingest` `heal` `lint` `label` `maintain` | After that skill |
| `dump` | You loaded INDEX or a whole folder instead of retrieve (penalty) |
| `session` | Optional session start |

```bash
python3 docs/wiki/scripts/wiki_usage.py log --op query --query "..." \
  --pages-opened "a.md,b.md" --cited "a.md" --tokens-est 3200
python3 docs/wiki/scripts/wiki_usage.py report --days 30
```

## Metrics to watch (now)

- **Retrieve count** — is the agent actually using the brain?
- **Est. tokens (sum / avg)** — context cost of packed excerpts (`len/4` heuristic, not provider billing)
- **Budget utilization** — `tokens_est / budget_tokens`; chronically ~100% means raise budget or tighten excerpts
- **Mean top_score / weak-hit rate** — retrieve miss vs hit (`weak` = top < 0.25)
- **Repeat-query groups** — same question ≥3 times → missing or unclear page
- **Dump admissions** — skipped retrieve
- **Last doctor score** — structural health (not usefulness)
- **Hottest pages** — coverage vs dead weight
- **Mean retrieve latency** — `duration_ms` (script cost, not LLM)
- **Citation overlap** — `cited ∩ pages_opened / pages_opened` when agents log query events

## Metrics worth adding later

- **Citation precision** — `cited ⊆ pages_opened ⊆ hit_paths`
- **Time-to-first-retrieve** in a session
- **Inbox SLA** — hours pending (doctor already flags pending items)
- **Episode consolidation rate** — promote:true vs stale
- **Provider-billed tokens** if a harness exposes them (do not guess)
- **Eval set** — `_meta/eval-queries.yaml` pass rate after retrieve
- **Heal churn** — same `code` finding recurring after heal

## Usefulness index (heuristic)

`wiki_usage.py score` mixes hit quality, last doctor score, activity, minus dump penalty. It is a **dashboard number**, not a proof the wiki is “correct.”

Skill: `docs/wiki/skills/wiki-usage/SKILL.md`
