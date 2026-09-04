# Trell Wiki Operations Log

Append-only chronological record for agents. Prefix contract: `## [YYYY-MM-DD] <op> | <title>`

---

## [2026-09-04] schema | Initialize Karpathy LLM Wiki brain

- Created `AGENTS.md` (Layer 3 schema for agents)
- Created `docs/wiki/SCHEMA.md` frontmatter + graph vocabulary
- Created `skills/wiki/` skill family: navigate, ingest, query, lint, label, maintain
- Applied YAML frontmatter (nodes/edges/agent) to all wiki pages
- Generated `docs/wiki/_meta/GRAPH.yaml` (77 nodes, 57 edges)
- Scripts: `skills/wiki/scripts/apply_frontmatter_and_sync_graph.py`, `sync_graph.py`

## [2026-09-04] ingest | Seed raw pointers to immutable sources

- Linked THESIS.md, examples, and prior market research note as raw-layer pointers
- Wiki remains the compiled knowledge layer; raw sources stay authoritative for origins

## [2026-09-04] label | Apply YAML nodes/edges to all wiki pages

- Script: `skills/wiki/scripts/apply_frontmatter_and_sync_graph.py`
- Regenerated GRAPH.yaml via `sync_graph.py` → 82 nodes, 62 edges

## [2026-09-04] lint | Initial health pass

- Report: `docs/wiki/_meta/health-2026-09-04.md`
- Status: PASS (bootstrap) with documented follow-ups

## [2026-09-04] query | wiki-brain simulation grading

- Scenario: "How does Trell keep autonomous ships safe?"
- Report: `docs/wiki/_meta/sim-2026-09-04.md`
- Navigate hit #1: applications/autonomous-physical-systems.md
- Graph: 82 nodes / 62 valid edges; 27 hard-orphan leaf nodes flagged for denser linking

## [2026-09-04] schema | Inbox + triage pipeline

- Added `docs/wiki/inbox/` drop zone, `_TEMPLATE.md`, example item
- Added `skills/wiki/triage/SKILL.md`; updated ingest + AGENTS.md + SCHEMA §6–§8
- Input path: inbox → triage → ingest → wiki/raw/GRAPH/log
- Taxonomy gate: no new folders/types/rels/tags without SCHEMA update

## [2026-09-04] doctor | initial diagnosis

- Script: `skills/wiki/scripts/wiki_doctor.py`
- Score: 72.8/100 (0 critical, 0 high, 27 medium orphan nodes, 1 low)
- Report: `docs/wiki/_meta/doctor-2026-09-04.md`

## [2026-09-04] heal | link hard orphans

- Linked 27 orphan nodes to hubs; archived example inbox item
- Re-doctor score: **100/100**
- Report: `docs/wiki/_meta/heal-2026-09-04.md`

## [2026-09-04] schema | memory lanes + retrieve

- Added ROUTER.md + _meta/CONTEXT_PROTOCOL.md (progressive disclosure budgets)
- Added episodic/ (INDEX, template, session-current, brain-memory-upgrade episode)
- Added temporal/TIMELINE.md + SCHEMA §9 temporal/episode contracts
- Added skills/wiki/retrieve + wiki_retrieve.py (lexical+graph+temporal+MMR)
- Added _meta/eval-queries.yaml golden set; doctor checks expired valid_until
- AGENTS/INDEX/README bootstrap now: ROUTER → retrieve (not full INDEX dump)
- Doctor score after wire: 100/100

## [2026-09-04] retrieve | smoke golden queries

- maritime COLREGs → applications/autonomous-physical-systems (ships section)
- belief/verify → core/epistemic-foundations
- episodic memory decision → episodic/2026-09-04-brain-memory-upgrade
- temporal timeline → temporal/TIMELINE.md

## [2026-09-04] episodic | 2026-09-04-brain-memory-upgrade

- Recorded gap-analysis → router/episodic/temporal/retrieve implementation episode
- Appended temporal/TIMELINE.md

## [2026-09-04] schema | AGENTS.md + CLAUDE.md + Cursor skills

- Split thin always-on `AGENTS.md` / `CLAUDE.md` from detailed `docs/wiki/OPERATOR.md`
- Added `.cursor/rules/*.mdc` (core, rust, wiki)
- Added `.cursor/skills/` + `.claude/skills/` launchers → `skills/wiki/` + `cargo-verify`
- Doctor now requires CLAUDE.md, OPERATOR.md, .cursor skill entrypoints

## [2026-09-04] handoff | Matt Pocock-style /handoff skill

- Canonical playbook: `skills/handoff/SKILL.md`
- Discoverable via `.cursor/skills/handoff` and `.claude/skills/handoff`
- Writes disposable temp handoff docs for fresh agent sessions
