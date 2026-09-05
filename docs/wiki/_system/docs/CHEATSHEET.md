---
id: repobrain-cheatsheet
title: RepoBrain Human Cheat Sheet
type: meta
status: active
created: 2026-09-05
updated: 2026-09-05
tags: [repobrain, cheatsheet, operators, cli]
domain: meta
summary: Human-facing RepoBrain guide for layers, authority, CLI, health, and pasteable agent prompts.
nodes:
  - id: repobrain-cheatsheet
    kind: concept
    label: RepoBrain cheat sheet
edges:
  - from: repobrain-cheatsheet
    to: wiki-router
    rel: related_to
  - from: repobrain-cheatsheet
    to: repobrain-source-pipeline
    rel: related_to
related:
  - "[[_system/docs/ROUTER]]"
  - "[[_system/docs/OPERATOR]]"
  - "[[_system/docs/SOURCES]]"
  - "[[_system/docs/FRAMEWORK]]"
  - "[[INDEX]]"
agent:
  priority: medium
  read_when:
    - onboarding a human operator
    - looking up exact RepoBrain commands
  maintain:
    - keep CLI examples identical to the public ./repobrain surface
---

# RepoBrain cheat sheet

This page is for humans. Agents should **not** auto-load it in Router Tier-0.
Use `./repobrain retrieve` or open this file on demand.

Deprecated `wiki-*` launchers still exist as compatibility aliases. Prefer
`./repobrain …` and `repobrain-*` skills.

## Three layers

| Layer | What it is | Authority |
|---|---|---|
| Raw sources | Git-tracked project docs, ADRs, data, converted Markdown | Non-authoritative inventory |
| Host corpus | Reviewed pages under `docs/wiki/` (minus `_system/`) | Compiled claims |
| Engine pack | Portable `_system/` skills, scripts, schema, router | How RepoBrain operates |

**Pack vs host:** `_system/` is the engine you can export. `HOST.yaml` names this
repository, domains, Graphify roots, and source-conversion policy.

**Graphify** indexes *code structure* (`src/`, etc.). It is not a second wiki
and does not compile claims.

**Authority order:** reviewed corpus pages beat raw inventory and derived
conversion text. Code questions go to Graphify. Inbox and unconsolidated
episodes are not product truth.

## Who does what

**Humans** edit `HOST.yaml`, choose conversion extras and `commit_groups`,
review inbox/conflicts, decide what becomes compiled wiki pages, and treat
eval/doctor scores as release signals.

**Agents/scripts** scan, convert, retrieve, doctor, eval, log usage, and sync
Graphify. They must not coerce raw text into certain claims or invent wiki
taxonomy.

Existing ADRs and docs sites are *detected* (MkDocs, Docusaurus, Mintlify,
Starlight, `CONTEXT.md`). They are not auto-ingested as compiled truth.

## Commands

```bash
./repobrain setup
./repobrain retrieve "<question>" --budget-tokens 3500
./repobrain retrieve "<question>" --include-sources
./repobrain graph sync
./repobrain graph query "<symbol or question>"
./repobrain source scan
./repobrain source convert
./repobrain source status --json
./repobrain doctor
./repobrain eval
./repobrain usage report
./repobrain dashboard html
./repobrain dashboard usage
```

Code-graph HTML remains `./repobrain graph export-html` (or
`./repobrain dashboard graph`).

## Pasteable agent prompts

| Task | Prompt |
|---|---|
| Setup | Install or refresh RepoBrain in this repo with `./repobrain setup`. Do not dump the wiki. |
| Retrieve | Retrieve evidence for: … Use `./repobrain retrieve` within Router budgets. Cite paths. |
| Graph | Query Graphify for how … is wired. Do not treat graph HTML as compiled wiki claims. |
| Sources | Scan Git-tracked sources and convert configured local formats. Keep derived Markdown non-authoritative. |
| Doctor | Run RepoBrain doctor and remediate critical/high findings without inventing taxonomy. |
| Eval | Run `./repobrain eval` and explain any failed category from the latest report. |
| Usage | Generate the usage dashboard and tell me if retrieve is expensive or weakly hitting. |
| Dashboard | Generate the local HTML health overview and summarize warnings with the printed path. |

## Health signals

| Signal | Where | Healthy looks like |
|---|---|---|
| Doctor score | `generated/doctor/latest.json` | High score, no critical/high |
| Golden eval | `generated/eval/repobrain-eval-*.json` | `status: pass`, budgets hold |
| Retrieval quality/cost | usage stats | usefulness_index up, tokens bounded |
| Graphify freshness | adapter `status`, not a second calculator | `artifact.state=ready`, source fresh |
| Source coverage | `generated/sources/manifest.json` | expected files inventoried |
| Conversion failures | manifest `conversion.state=failed` | zero retryable failures |
| Usage heuristics | `./repobrain usage report` | few dumps, strong hits |

## Troubleshooting

- Weak retrieve → add or ingest a compiled page; do not paste whole `INDEX.md`.
- Raw vs compiled disagreement → inbox candidate; compiled stays authoritative.
- Conversion `pending` → install the narrow MarkItDown extra for that format.
- Conversion `blocked` → `allow_external` is required before URL/plugin/OCR flags.
- Stale Graphify → `./repobrain graph sync`.
- Deep operator detail → `OPERATOR.md`, not this page.
