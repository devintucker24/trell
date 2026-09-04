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
