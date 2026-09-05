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
  - "[[FRAMEWORK]]"
  - "[[_system/docs/GRAPH]]"
agent:
  priority: critical
  read_when:
    - operating the wiki brain in depth
    - triage ingest doctor heal workflows
  maintain:
    - keep in sync with root AGENTS.md pointers and _system/skills
---

# OPERATOR.md — Wiki Brain Operator Manual

> **Audience:** AI coding agents (Cursor, Codex, Claude Code, OpenCode) and human maintainers.  
> **Purpose:** Turn any agent from a generic chatbot into a disciplined wiki operator.  
> **Pattern:** three layers (raw → corpus → engine). Host config:
> `docs/wiki/_system/config/HOST.yaml`. Thin brief: root `AGENTS.md`.
> **Portable pack:** [[FRAMEWORK]]

---

## 1. Three-Layer Architecture

```
┌─────────────────────────────────────────────────────────────────┐
| LAYER 3 — SCHEMA                                                    |
|   root AGENTS.md / CLAUDE.md (thin)  ·  docs/wiki/_system/docs/     |
|   _system/skills/  ·  _system/scripts/  ·  _system/config/          |
|   HOST.yaml (project overlay)  ·  thin .cursor/.claude launchers    |
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

**Rule:** Knowledge is compiled once into the corpus and kept current. Start
from `docs/wiki/_system/docs/ROUTER.md`, then retrieve.

---

## 2. First Actions on Every Session

1. Read root `AGENTS.md` (project brief), then this file §§1–2 and §8 when doing wiki ops.
2. Read `docs/wiki/_system/docs/ROUTER.md`.
3. Run `python3 docs/wiki/_system/scripts/wiki_retrieve.py "<task>"`.
4. For when/as-of/what-changed: `docs/wiki/temporal/TIMELINE.md` + retrieve `--as-of`.
5. For prior decisions: `docs/wiki/episodic/INDEX.md` (then dated episodes).
6. Only then open `INDEX.md` / `SCHEMA.md` if browsing or editing structure.
   - Canonical playbooks → `docs/wiki/_system/skills/`

---

## 3. Page Types (Directory Contract)

| Type | Path | Mutability | Purpose |
|------|------|------------|---------|
| `index` | `docs/wiki/INDEX.md` | Rewrite on every structural change | Master catalog |
| `concept` | `docs/wiki/core/`, `docs/wiki/theory/` | Rewriteable | Ideas, type systems, engines |
| `application` | `docs/wiki/applications/` | Rewriteable | Domain niches + Trell code |
| `market` | `docs/wiki/market/` | Rewriteable | Competitors, regulation, personas |
| `roadmap` | `docs/wiki/roadmap/` | Rewriteable | Vision + phased milestones |
| `engine` | `docs/wiki/_system/` | Human+agent co-evolve | Operators and generated state |
| `corpus` | `docs/wiki/` except `_system` | Human-reviewed | Host knowledge |
| `episode` | `docs/wiki/episodic/` | Append + consolidate | Episodic memory (not semantic truth) |
| `inbox-item` | `docs/wiki/inbox/` | Pending → archive | Unprocessed drops (not wiki truth) |
| `log` | `docs/wiki/log.md` | Append-only | Chronological ops |
| `raw-pointer` | `docs/wiki/raw/` | Append-only pointers | Links to immutable sources |

**Temporal spine:** `docs/wiki/temporal/TIMELINE.md` (`domain: temporal`) — knowledge chronology for as-of recall.

---

## 4. Frontmatter Is Mandatory

Every corpus page must conform to `docs/wiki/_system/docs/SCHEMA.md`.

Minimum required fields:
- `id`, `title`, `type`, `status`, `created`, `updated`, `tags`, `domain`, `summary`
- `nodes` (list of graph nodes owned by this page)
- `edges` (list of typed relations)
- `related` (wikilinks to neighbor pages)
- `agent` (priority, read_when, maintain hooks)

If you create or edit a page without valid frontmatter, you have failed the schema. Run `wiki-label` to heal.

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

Canonical **claim** dump: `_system/generated/claim-graph.yaml`.
Canonical **code** graph: `graphify-out/graph.json`. See [[_system/docs/GRAPH]].

---

## 6. Operational Workflows

### 6.0 New repo / portable install
Skill: `docs/wiki/_system/skills/wiki-setup/SKILL.md`
Setup: `python3 docs/wiki/_system/scripts/wiki_setup.py`.
Export: `python3 docs/wiki/_system/scripts/wiki_pack.py export /path/to/other-repo`.
Human leftover: `HOST.yaml` `anchor` + review any `graphify-seed` drafts. Details: [[FRAMEWORK]].

### 6.0b Inbox drop (default on-ramp)
1. Create `docs/wiki/inbox/YYYY-MM-DD-<slug>.md` from `inbox/_TEMPLATE.md`  
   — or accept user phrase: *"Inbox this: …"*
2. Leave `triage_status: pending`.
3. Do **not** cite inbox content as wiki truth yet.
4. See `docs/wiki/inbox/README.md`.

### 6.1 Triage (classify before writing)
Skill: `docs/wiki/_system/skills/wiki-triage/SKILL.md`
Decide `suggested_action`: `merge-existing` | `new-page` | `raw-only` | `discard` | `needs-human`.  
Update `_system/docs/SCHEMA.md` before adding taxonomy.

### 6.2 Ingest (write wiki truth)
Skill: `docs/wiki/_system/skills/wiki-ingest/SKILL.md`
1. Execute triaged action (merge / new page / raw pointer).
2. Full frontmatter per SCHEMA; sync the generated claim graph.
3. Update `INDEX.md` if structure changed.
4. Archive inbox item → `inbox/archive/`; set `triage_status: ingested`.
5. Append `## [YYYY-MM-DD] ingest | <title>` to `docs/wiki/log.md`.

### 6.3 Query (answer from wiki)
1. Prefer `wiki-retrieve` (scored top-k) over skimming all of INDEX.
2. For time questions: consult `temporal/TIMELINE.md` and/or `retrieve --as-of`.
3. Read 2–6 relevant pages (not the whole wiki); cite with wikilinks.
4. Prefer filing valuable answers back as new wiki pages (`type: synthesis` or expand existing).
5. Log: `## [YYYY-MM-DD] query | <question slug>`
6. Pending `inbox/` items and unconsolidated episodes are not settled knowledge.

### 6.3b Retrieve (file RAG)
Skill: `docs/wiki/_system/skills/wiki-retrieve/SKILL.md`
`python3 docs/wiki/_system/scripts/wiki_retrieve.py "<q>" --budget-tokens 3500`
Hybrid lexical + graph + **temporal** rerank + MMR diversity.

### 6.3c Episodic / temporal write path
1. New session decision/failure → `episodic/YYYY-MM-DD-<slug>.md` from `_TEMPLATE.md`
2. Append event to `temporal/TIMELINE.md`
3. Keep `episodic/session-current.md` under ~800 tokens
4. On consolidate: merge lessons into semantic pages; set episode `status: stale` if done
5. Log: `## [YYYY-MM-DD] episodic | <slug>` or `temporal | <slug>`

### 6.4 Wiki Doctor (diagnose only)
Skill: `docs/wiki/_system/skills/wiki-doctor/SKILL.md`
Run `python3 docs/wiki/_system/scripts/wiki_doctor.py`.
**No wiki content edits.**

### 6.5 Wiki Heal (apply safe fixes)
Skill: `docs/wiki/_system/skills/wiki-heal/SKILL.md`
Consume the doctor report; fix frontmatter/links/orphan edges/inbox routing; never invent taxonomy.  
Re-run doctor to verify. Log `## [date] heal | …`.

### 6.6 Lint (shortcut)
Skill: `docs/wiki/_system/skills/wiki-lint/SKILL.md` = doctor → heal → re-doctor.

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
| Handoff | `.cursor/skills/handoff/SKILL.md` | Compact this session → `.handoffs/handoff-*.md` (`/handoff`) |
| Read handoff | `.cursor/skills/read-handoff/SKILL.md` | Fresh chat: load newest handoff, then delete it (`/read-handoff`) |
| Grill me | `.cursor/skills/grill-me/SKILL.md` | User-invoked front door → runs `grilling` |
| Grilling | `.cursor/skills/grilling/SKILL.md` | Relentless design-tree interview (rounds / frontier) |
| Wiki-brain (parent) | `_system/skills/wiki-brain/SKILL.md` | Portable operator kit |
| Retrieve | `_system/skills/wiki-retrieve/SKILL.md` | File RAG |
| Navigate | `_system/skills/wiki-navigate/SKILL.md` | Finding pages / graph traversal |
| Triage | `_system/skills/wiki-triage/SKILL.md` | Classify inbox |
| Ingest | `_system/skills/wiki-ingest/SKILL.md` | Promote reviewed knowledge |
| Doctor | `_system/skills/wiki-doctor/SKILL.md` | Diagnose corpus health |
| Heal | `_system/skills/wiki-heal/SKILL.md` | Apply doctor-driven fixes |
| Lint | `_system/skills/wiki-lint/SKILL.md` | Doctor → heal → doctor |
| Query | `_system/skills/wiki-query/SKILL.md` | Cited answers |
| Label | `_system/skills/wiki-label/SKILL.md` | Frontmatter normalization |
| Maintain | `_system/skills/wiki-maintain/SKILL.md` | Code/corpus synchronization |
| Usage | `_system/skills/wiki-usage/SKILL.md` | Telemetry + dashboard |

---

## 10. Log Prefix Contract

Every `docs/wiki/log.md` entry must start with:
```markdown
## [YYYY-MM-DD] <op> | <short title>
```
Where `<op>` ∈ `inbox | triage | ingest | query | retrieve | doctor | heal | lint | label | maintain | schema | graph | episodic | temporal | usage`.

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
