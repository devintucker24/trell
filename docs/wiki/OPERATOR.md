---
id: wiki-operator
title: Wiki Brain Operator Manual
type: schema
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [schema, agents, operator, wiki]
domain: meta
summary: Detailed wiki operator manual — progressive disclosure after root AGENTS.md / CLAUDE.md.
nodes:
  - id: wiki-operator
    kind: concept
    label: Wiki Operator Manual
  - id: agents-md
    kind: concept
    label: AGENTS.md
edges:
  - from: wiki-operator
    to: wiki-schema
    rel: depends_on
  - from: wiki-operator
    to: wiki-router
    rel: depends_on
  - from: agents-md
    to: wiki-operator
    rel: related_to
related:
  - "[[SCHEMA]]"
  - "[[ROUTER]]"
  - "[[INDEX]]"
agent:
  priority: critical
  read_when:
    - operating the wiki brain in depth
    - triage ingest doctor heal workflows
  maintain:
    - keep in sync with root AGENTS.md pointers and skills/wiki
---

# OPERATOR.md — Trell Wiki Brain Operator Manual

> **Audience:** AI coding agents (Cursor, Codex, Claude Code, OpenCode) and human maintainers.  
> **Purpose:** Turn any agent from a generic chatbot into a disciplined Trell wiki operator.  
> **Pattern:** Karpathy LLM Wiki — three layers (raw → wiki → schema). This file is the **detailed wiki operator manual**. Thin always-on project brief: root `AGENTS.md` / `CLAUDE.md`.

---

## 1. Three-Layer Architecture

```
┌─────────────────────────────────────────────────────────────────┐
| LAYER 3 — SCHEMA                                                    |
|   root AGENTS.md / CLAUDE.md (thin)  ·  docs/wiki/OPERATOR.md       |
|   docs/wiki/SCHEMA.md  ·  skills/wiki/  ·  .cursor/skills/          |
|   Tells agents HOW to navigate, heal, label, ingest, maintain       |
├─────────────────────────────────────────────────────────────────┤
| LAYER 2 — WIKI (LLM-owned, compounding)                         |
|   docs/wiki/**/*.md  ·  ROUTER.md  ·  INDEX.md  ·  GRAPH.yaml   |
|   episodic/ · temporal/ · inbox/ → triage → ingested pages      |
|   Syntheses, concepts, applications, market, roadmap            |
|   Every page has YAML frontmatter with nodes + edges            |
├─────────────────────────────────────────────────────────────────┤
| LAYER 1 — RAW (immutable source of truth)                       |
|   docs/wiki/raw/  ·  THESIS.md  ·  examples/*.trell  ·  src/    |
|   Agents READ raw sources; they NEVER silently rewrite them     |
|   as if they were wiki pages. Code changes go through normal    |
|   engineering PRs; wiki pages are the compiled knowledge layer. |
└─────────────────────────────────────────────────────────────────┘
```

**Rule:** Knowledge is *compiled once* into the wiki and kept current. Do not re-derive the entire thesis from `src/` on every query. Start from `docs/wiki/ROUTER.md`, then **retrieve** — do not dump the whole INDEX into context.

---

## 2. First Actions on Every Session

1. Read root `AGENTS.md` (project brief), then this file §§1–2 and §8 when doing wiki ops.
2. Read `docs/wiki/ROUTER.md` (tiered loading + budgets). **Do not** dump full INDEX into context.
3. Retrieve on demand: `python3 skills/wiki/scripts/wiki_retrieve.py "<task>"` (skill: `skills/wiki/retrieve`).
4. For when/as-of/what-changed: `docs/wiki/temporal/TIMELINE.md` + retrieve `--as-of`.
5. For prior decisions: `docs/wiki/episodic/INDEX.md` (then dated episodes).
6. Only then open `INDEX.md` / `SCHEMA.md` if browsing or editing structure.
7. Load the relevant skill from `skills/wiki/` for the task:
   - Retrieve → `skills/wiki/retrieve/SKILL.md`  ← **prefer for Q&A**
   - Navigate → `skills/wiki/navigate/SKILL.md`
   - Triage → `skills/wiki/triage/SKILL.md`
   - Ingest → `skills/wiki/ingest/SKILL.md`
   - Doctor → `skills/wiki/doctor/SKILL.md`
   - Heal → `skills/wiki/heal/SKILL.md`
   - Lint (doctor+heal) → `skills/wiki/lint/SKILL.md`
   - Label → `skills/wiki/label/SKILL.md`
   - Maintain → `skills/wiki/maintain/SKILL.md`
   - Query / Answer → `skills/wiki/query/SKILL.md`

---

## 3. Page Types (Directory Contract)

| Type | Path | Mutability | Purpose |
|------|------|------------|---------|
| `index` | `docs/wiki/INDEX.md` | Rewrite on every structural change | Master catalog |
| `concept` | `docs/wiki/core/`, `docs/wiki/theory/` | Rewriteable | Ideas, type systems, engines |
| `application` | `docs/wiki/applications/` | Rewriteable | Domain niches + Trell code |
| `market` | `docs/wiki/market/` | Rewriteable | Competitors, regulation, personas |
| `roadmap` | `docs/wiki/roadmap/` | Rewriteable | Vision + phased milestones |
| `schema` | `docs/wiki/SCHEMA.md`, `docs/wiki/OPERATOR.md`, root `AGENTS.md` | Human+agent co-evolve | Operating rules |
| `meta` | `docs/wiki/_meta/`, `docs/wiki/ROUTER.md` | Agent-maintained | GRAPH, health, router, protocols |
| `episode` | `docs/wiki/episodic/` | Append + consolidate | Episodic memory (not semantic truth) |
| `inbox-item` | `docs/wiki/inbox/` | Pending → archive | Unprocessed drops (not wiki truth) |
| `log` | `docs/wiki/log.md` | Append-only | Chronological ops |
| `raw-pointer` | `docs/wiki/raw/` | Append-only pointers | Links to immutable sources |

**Temporal spine:** `docs/wiki/temporal/TIMELINE.md` (`domain: temporal`) — knowledge chronology for as-of recall.

---

## 4. Frontmatter Is Mandatory

Every wiki markdown page **must** begin with YAML frontmatter conforming to `docs/wiki/SCHEMA.md`.

Minimum required fields:
- `id`, `title`, `type`, `status`, `created`, `updated`, `tags`, `domain`, `summary`
- `nodes` (list of graph nodes owned by this page)
- `edges` (list of typed relations)
- `related` (wikilinks to neighbor pages)
- `agent` (priority, read_when, maintain hooks)

If you create or edit a page without valid frontmatter, you have failed the schema. Run `skills/wiki/label` to heal.

---

## 5. Graph Model (Nodes & Edges)

### Node kinds
`concept` · `type` · `primitive` · `engine` · `application` · `competitor` · `regulation` · `persona` · `phase` · `technology` · `example`

### Edge relations (typed)
| Relation | Meaning |
|----------|---------|
| `depends_on` | A requires B to be understood / implemented |
| `implements` | Code or page implements a concept |
| `contradicts` | Claim A conflicts with claim B (flag for lint) |
| `extends` | A is a future/advanced form of B |
| `applies_to` | Concept applies in a domain niche |
| `competes_with` | Market alternative |
| `enforces` | Guard/contract enforces invariant |
| `reduces_via` | belief → certain via this mechanism |
| `accelerates` | Hardware/tech accelerates a runtime feature |
| `regulated_by` | Domain constrained by a regulation |
| `owned_by` | Persona / agent role responsible |
| `milestone_of` | Phase milestone belongs to vision |

Canonical graph dump: `docs/wiki/_meta/GRAPH.yaml`  
Agents must update GRAPH.yaml when adding/removing nodes or edges.

---

## 6. Operational Workflows

### 6.0 Inbox drop (default on-ramp)
1. Create `docs/wiki/inbox/YYYY-MM-DD-<slug>.md` from `inbox/_TEMPLATE.md`  
   — or accept user phrase: *"Inbox this: …"*
2. Leave `triage_status: pending`.
3. Do **not** cite inbox content as wiki truth yet.
4. See `docs/wiki/inbox/README.md`.

### 6.1 Triage (classify before writing)
Skill: `skills/wiki/triage/SKILL.md`  
Decide `suggested_action`: `merge-existing` | `new-page` | `raw-only` | `discard` | `needs-human`.  
**Never invent a new top-level folder / type / domain / edge `rel` / recurring tag without updating `docs/wiki/SCHEMA.md` first** (and ask a human if unsure).

### 6.2 Ingest (write wiki truth)
Skill: `skills/wiki/ingest/SKILL.md`  
1. Execute triaged action (merge / new page / raw pointer).
2. Full frontmatter per SCHEMA; sync `_meta/GRAPH.yaml`.
3. Update `INDEX.md` if structure changed.
4. Archive inbox item → `inbox/archive/`; set `triage_status: ingested`.
5. Append `## [YYYY-MM-DD] ingest | <title>` to `docs/wiki/log.md`.

### 6.3 Query (answer from wiki)
1. Prefer `skills/wiki/retrieve` (scored top-k) over skimming all of INDEX.
2. For time questions: consult `temporal/TIMELINE.md` and/or `retrieve --as-of`.
3. Read 2–6 relevant pages (not the whole wiki); cite with wikilinks.
4. Prefer filing valuable answers back as new wiki pages (`type: synthesis` or expand existing).
5. Log: `## [YYYY-MM-DD] query | <question slug>`
6. Pending `inbox/` items and unconsolidated episodes are not settled knowledge.

### 6.3b Retrieve (file RAG)
Skill: `skills/wiki/retrieve/SKILL.md`  
`python3 skills/wiki/scripts/wiki_retrieve.py "<q>" --budget-tokens 3500`  
Hybrid lexical + graph + **temporal** rerank + MMR diversity.

### 6.3c Episodic / temporal write path
1. New session decision/failure → `episodic/YYYY-MM-DD-<slug>.md` from `_TEMPLATE.md`
2. Append event to `temporal/TIMELINE.md`
3. Keep `episodic/session-current.md` under ~800 tokens
4. On consolidate: merge lessons into semantic pages; set episode `status: stale` if done
5. Log: `## [YYYY-MM-DD] episodic | <slug>` or `temporal | <slug>`

### 6.4 Wiki Doctor (diagnose only)
Skill: `skills/wiki/doctor/SKILL.md`  
Run `python3 skills/wiki/scripts/wiki_doctor.py` → writes `_meta/doctor-YYYY-MM-DD.md` + `doctor-latest.json`.  
**No wiki content edits.**

### 6.5 Wiki Heal (apply safe fixes)
Skill: `skills/wiki/heal/SKILL.md`  
Consume the doctor report; fix frontmatter/links/orphan edges/inbox routing; never invent taxonomy.  
Re-run doctor to verify. Log `## [date] heal | …`.

### 6.6 Lint (shortcut)
Skill: `skills/wiki/lint/SKILL.md` = doctor → heal → re-doctor.

### 6.7 Label
Normalize tags, domains, node ids (kebab-case), edge relations to the allowed vocabulary in SCHEMA.md.

### 6.8 Maintain (code ↔ wiki sync)
When `src/` or `examples/` change epistemic semantics:
1. Diff against wiki claims in `core/` and `theory/`.
2. Update pages; bump `updated`.
3. Note implementation binding under `agent.maintain` and `implements` edges to `src/*.rs` paths.

---

## 7. Naming & Wikilink Conventions

- Page files: `kebab-case.md`
- Node ids: `kebab-case` globally unique across GRAPH.yaml
- Wikilinks: `[[folder/page-name]]` or `[[folder/page-name|Display]]`
- Do **not** invent purple/cream marketing prose in docs; keep technical clarity for agents + academics.

---

## 8. What Trell Is (Brain Anchor — Do Not Dilute)

Trell is an **epistemic programming language**:
- Dual-track types: `certain T` vs `belief<T>`
- Epistemic reduction only via `verify`/`require` + `guard`
- Speculative semantic execution: `when`/`fork` with rollback
- Model contracts + quorums
- Natural Trell: colon + indent + `end`

**Goal:** Become the authority layer between stochastic models and irreversible actuators (ships, surgery, grids, treasury) — the Ada/Rust of the AI era by ~2036.

If a wiki edit weakens this thesis without evidence, reject it in lint.

---

## 9. Skills Index

| Skill | Path | Use when |
|-------|------|----------|
| Handoff | `skills/handoff/SKILL.md` | Compact this session → `.handoffs/handoff-*.md` (`/handoff`) |
| Read handoff | `skills/read-handoff/SKILL.md` | Fresh chat: load newest handoff, then delete it (`/read-handoff`) |
| Grill me | `skills/grill-me/SKILL.md` | User-invoked front door → runs `grilling` |
| Grilling | `skills/grilling/SKILL.md` | Relentless design-tree interview (rounds / frontier) |
| Retrieve | `skills/wiki/retrieve/SKILL.md` | File RAG: scored top-k with temporal/graph rerank |
| Navigate | `skills/wiki/navigate/SKILL.md` | Finding pages / graph traversal |
| Triage | `skills/wiki/triage/SKILL.md` | Classify inbox; decide merge vs new vs taxonomy gate |
| Ingest | `skills/wiki/ingest/SKILL.md` | Write triaged knowledge into wiki/raw |
| Doctor | `skills/wiki/doctor/SKILL.md` | Diagnose only (wiki doctor) |
| Heal | `skills/wiki/heal/SKILL.md` | Apply safe fixes from doctor report |
| Lint | `skills/wiki/lint/SKILL.md` | Shortcut: doctor → heal → re-doctor |
| Query | `skills/wiki/query/SKILL.md` | Answering questions from the brain |
| Label | `skills/wiki/label/SKILL.md` | Frontmatter + tag normalization |
| Maintain | `skills/wiki/maintain/SKILL.md` | Sync wiki with compiler / examples |

---

## 10. Log Prefix Contract

Every `docs/wiki/log.md` entry must start with:
```markdown
## [YYYY-MM-DD] <op> | <short title>
```
Where `<op>` ∈ `inbox | triage | ingest | query | retrieve | doctor | heal | lint | label | maintain | schema | graph | episodic | temporal`.

This enables: `grep "^## \[" docs/wiki/log.md | tail -20`

---

## 11. Non-Goals for Wiki Agents

- Do not replace LangChain-style connector catalogs in the wiki as if Trell were an SDK.
- Do not cite abandoned `docs/research/` sketches as Trell product truth unless reconciled into wiki with explicit status.
- Do not invent unverifiable market share numbers; prefer qualitative competitive maps + regulatory drivers.
- Do not delete log entries (append-only).
- Do not invent new wiki folders/types/rels from a single inbox note — triage to `needs-human` and update SCHEMA first.
- Do not dump the full wiki (~32k tokens) into context; follow `ROUTER.md` budgets.
- Do not cite episodic or temporal timeline entries as semantic product truth until consolidated into domain pages.
