---
id: repobrain-cheatsheet
title: RepoBrain Human Cheat Sheet
type: meta
status: active
created: 2026-09-05
updated: 2026-09-05
tags: [repobrain, cheatsheet, operators, cli]
domain: meta
summary: Human-facing RepoBrain guide splitting CLI commands from agent skills, plus health signals.
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
    - keep Commands identical to the public ./repobrain surface
    - keep Skills as playbooks; never invent a CLI verb that does not exist
---

# RepoBrain cheat sheet

This page is for humans. Agents should **not** auto-load it in Router Tier-0.
Use `./repobrain retrieve` or open this file on demand.

**Commands** are `./repobrain` verbs. **Skills** are agent playbooks under
`docs/wiki/_system/skills/repobrain-*/SKILL.md`. A skill may wrap a command
(retrieve, doctor) or have **no CLI** (query, navigate). Temporary `wiki-*`
aliases were removed; see `_system/docs/MIGRATION.md`.

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

These are the public `./repobrain` verbs. There is no `./repobrain query` or
`./repobrain navigate`. `graph query` searches the **code** graph, not the wiki.

```bash
./repobrain setup
./repobrain retrieve "<question>" --budget-tokens 3500
./repobrain retrieve "<question>" --include-sources
./repobrain graph sync
./repobrain graph query "<symbol or question>"
./repobrain graph export-html
./repobrain source scan
./repobrain source convert
./repobrain source status --json
./repobrain doctor
./repobrain eval
./repobrain usage report
./repobrain dashboard html
```

`dashboard html` prints a `file://` URL on stdout. Open that in Chrome, Safari,
or Finder. The filesystem path and Simple Browser warning go to stderr. Do not
paste a `/Users/...` path into Cursor Simple Browser (`https://users/...` →
`ERR_NAME_NOT_RESOLVED`).

The HTML dashboard has Overview, Sources, Code graph (Graphify embed + full-page
fallback), and Cheat sheet tabs.

## Skills

Canonical playbooks: `docs/wiki/_system/skills/repobrain-<name>/SKILL.md`.
Harness launchers (`.cursor/skills/`, `.claude/skills/`) only point here.

| Skill | Kind | Does | Related command |
|---|---|---|---|
| `repobrain-retrieve` | wraps CLI | Ranked wiki RAG (lexical + claim graph + temporal) | `./repobrain retrieve` |
| `repobrain-query` | playbook only | After retrieve: answer with `[[cites]]`, optionally file a synthesis page | uses retrieve; **no** `query` verb |
| `repobrain-navigate` | playbook only | After retrieve: return wikilinks + one-line summaries; for code, Graphify paths | uses retrieve + `graph query`; **no** `navigate` verb |
| `repobrain-triage` | playbook only | Classify inbox; do not ingest yet | — |
| `repobrain-ingest` | playbook only | Promote reviewed inbox into compiled pages | — |
| `repobrain-doctor` | wraps CLI | Corpus health audit | `./repobrain doctor` |
| `repobrain-heal` | playbook only | Repair doctor findings | — |
| `repobrain-lint` | playbook only | Doctor → heal → doctor | `./repobrain doctor` |
| `repobrain-label` | playbook only | Normalize frontmatter to SCHEMA | — |
| `repobrain-maintain` | playbook only | Sync compiled claims and graphs after code/wiki change | `./repobrain graph sync` |
| `repobrain-usage` | wraps CLI | Retrieval cost / usefulness telemetry | `./repobrain usage report` |
| `repobrain-setup` | wraps CLI | Install or refresh the engine in a repo | `./repobrain setup` |
| `repobrain-brain` | playbook only | Parent operator skill | `./repobrain --help` |

Retrieve is the lookup **engine**. Query and navigate are **what the agent does
with those hits** (essay vs map). They are not extra search backends.

## Pasteable prompts

**CLI tasks** (paste when you want a verb run):

| Task | Prompt |
|---|---|
| Setup | Install or refresh RepoBrain in this repo with `./repobrain setup`. Do not dump the wiki. |
| Retrieve | Retrieve evidence for: … Use `./repobrain retrieve` within Router budgets. Cite paths. |
| Graph | Query Graphify for how … is wired. Do not treat graph HTML as compiled wiki claims. |
| Sources | Scan Git-tracked sources and convert configured local formats. Keep derived Markdown non-authoritative. |
| Doctor | Run RepoBrain doctor and remediate critical/high findings without inventing taxonomy. |
| Eval | Run `./repobrain eval` and explain any failed category from the latest report. |
| Usage | Generate the usage report and tell me if retrieve is expensive or weakly hitting. |
| Dashboard | Generate the local HTML health overview and open the printed `file://` URL in the system browser, not Cursor Simple Browser. |

**Skill tasks** (paste when you want a playbook, not a fake CLI):

| Skill | Prompt |
|---|---|
| Query | Answer from cited RepoBrain retrieve hits. Do not invent compiled claims. File a synthesis page if the answer should persist. There is no `./repobrain query`. |
| Navigate | Navigate the corpus: retrieve, then return wikilinks and one-line summaries. For code wiring use `./repobrain graph query`. There is no `./repobrain navigate`. |

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
- Dashboard `https://users/...` / `ERR_NAME_NOT_RESOLVED` → the POSIX path was
  opened as HTTPS. Run `./repobrain dashboard html` and open the printed
  `file://` URL in the system browser (`open <path>` on macOS).
- Stale Graphify → `./repobrain graph sync`.
- Deep operator detail → `OPERATOR.md`, not this page.
