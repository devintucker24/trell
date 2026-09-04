# AGENTS.md — Trell Epistemic Brain Schema

> **Audience:** AI coding agents (Cursor, Codex, Claude Code, OpenCode) and human maintainers.  
> **Purpose:** Turn any agent from a generic chatbot into a disciplined Trell wiki operator.  
> **Pattern:** Karpathy LLM Wiki — three layers (raw → wiki → schema). This file *is* the schema.

---

## 1. Three-Layer Architecture

```
┌─────────────────────────────────────────────────────────────────┐
| LAYER 3 — SCHEMA (you are here)                                 |
|   AGENTS.md  ·  docs/wiki/SCHEMA.md  ·  skills/wiki/            |
|   Tells agents HOW to navigate, heal, label, ingest, maintain   |
├─────────────────────────────────────────────────────────────────┤
| LAYER 2 — WIKI (LLM-owned, compounding)                         |
|   docs/wiki/**/*.md  ·  INDEX.md  ·  GRAPH.yaml  ·  log.md      |
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

**Rule:** Knowledge is *compiled once* into the wiki and kept current. Do not re-derive the entire thesis from `src/` on every query. Read `docs/wiki/INDEX.md` first, then drill into pages.

---

## 2. First Actions on Every Session

1. Read this file (`AGENTS.md`).
2. Read `docs/wiki/INDEX.md` (catalog).
3. If maintaining graph health: read `docs/wiki/_meta/GRAPH.yaml` and `docs/wiki/log.md` (tail).
4. Load the relevant skill from `skills/wiki/` for the task:
   - Navigate → `skills/wiki/navigate/SKILL.md`
   - Triage → `skills/wiki/triage/SKILL.md`
   - Ingest → `skills/wiki/ingest/SKILL.md`
   - Lint / Heal → `skills/wiki/lint/SKILL.md`
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
| `schema` | `docs/wiki/SCHEMA.md`, `AGENTS.md` | Human+agent co-evolve | Operating rules |
| `meta` | `docs/wiki/_meta/` | Agent-maintained | GRAPH.yaml, health reports |
| `inbox-item` | `docs/wiki/inbox/` | Pending → archive | Unprocessed drops (not wiki truth) |
| `log` | `docs/wiki/log.md` | Append-only | Chronological ops |
| `raw-pointer` | `docs/wiki/raw/` | Append-only pointers | Links to immutable sources |

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
1. Read INDEX → select pages → cite with wikilinks.
2. Prefer filing valuable answers back as new wiki pages (`type: synthesis` or expand existing).
3. Log: `## [YYYY-MM-DD] query | <question slug>`
4. Pending `inbox/` items are not settled knowledge.

### 6.4 Lint / Heal
Check for: orphan pages, missing inbound links, stale `updated` dates, broken wikilinks, nodes without edges, edges pointing to missing ids, contradictions, concepts mentioned in body but lacking pages, frontmatter schema violations, stale inbox (`pending` too long).
Write a health report to `docs/wiki/_meta/health-YYYY-MM-DD.md` and append a log entry.

### 6.5 Label
Normalize tags, domains, node ids (kebab-case), edge relations to the allowed vocabulary in SCHEMA.md.

### 6.6 Maintain (code ↔ wiki sync)
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
| Navigate | `skills/wiki/navigate/SKILL.md` | Finding pages / graph traversal |
| Triage | `skills/wiki/triage/SKILL.md` | Classify inbox; decide merge vs new vs taxonomy gate |
| Ingest | `skills/wiki/ingest/SKILL.md` | Write triaged knowledge into wiki/raw |
| Query | `skills/wiki/query/SKILL.md` | Answering questions from the brain |
| Lint | `skills/wiki/lint/SKILL.md` | Health-check / heal orphans & contradictions |
| Label | `skills/wiki/label/SKILL.md` | Frontmatter + tag normalization |
| Maintain | `skills/wiki/maintain/SKILL.md` | Sync wiki with compiler / examples |

---

## 10. Log Prefix Contract

Every `docs/wiki/log.md` entry must start with:
```markdown
## [YYYY-MM-DD] <op> | <short title>
```
Where `<op>` ∈ `inbox | triage | ingest | query | lint | label | maintain | schema | graph`.

This enables: `grep "^## \[" docs/wiki/log.md | tail -20`

---

## 11. Non-Goals for Wiki Agents

- Do not replace LangChain-style connector catalogs in the wiki as if Trell were an SDK.
- Do not cite abandoned `docs/research/` sketches as Trell product truth unless reconciled into wiki with explicit status.
- Do not invent unverifiable market share numbers; prefer qualitative competitive maps + regulatory drivers.
- Do not delete log entries (append-only).
- Do not invent new wiki folders/types/rels from a single inbox note — triage to `needs-human` and update SCHEMA first.
