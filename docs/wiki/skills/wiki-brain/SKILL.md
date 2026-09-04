---
name: wiki-brain
description: Operate a portable Karpathy-style wiki brain — retrieve, navigate, triage, ingest, query, doctor, heal, lint, label, maintain, and usage telemetry. Canonical playbooks live in docs/wiki/skills/; Cursor/Claude copies are thin launchers. Use whenever working with the knowledge wiki, AGENTS.md, or long-term research docs.
---

# Wiki-brain — Parent Skill (host-agnostic)

This family implements the **Karpathy LLM Wiki** pattern as a **file RAG + multi-lane memory** system. Domain content (this repo: Trell) is an *instance*; the operator kit is reusable.

**Canonical playbooks:** `docs/wiki/skills/*/SKILL.md`  
**Adapters only:** `.cursor/skills/wiki-*` · `.claude/skills/wiki-*`  
**Export kit:** `docs/wiki/FRAMEWORK.md`

## Architecture
- **Schema (thin):** host project brief (`AGENTS.md`, optional `CLAUDE.md`)
- **Schema (deep):** `docs/wiki/OPERATOR.md`, `docs/wiki/SCHEMA.md`, `docs/wiki/skills/`
- **Wiki:** `docs/wiki/**` with YAML nodes/edges
- **Scripts:** `docs/wiki/scripts/` (`wiki_retrieve.py`, `wiki_doctor.py`, `wiki_usage.py`, `sync_graph.py`)
- **Memory lanes:** semantic pages · `episodic/` · `temporal/TIMELINE.md`
- **Context:** `docs/wiki/ROUTER.md` + `_meta/CONTEXT_PROTOCOL.md` (progressive disclosure)
- **Inbox:** `docs/wiki/inbox/` → triage → ingest
- **Usage:** JSONL at `_meta/usage/events.jsonl` (gitignored) + `_meta/usage-dashboard.md`

## Subskills (read the canonical files)

| Task | Canonical |
|------|-----------|
| **Retrieve / file RAG** | `docs/wiki/skills/wiki-retrieve/SKILL.md` |
| Find pages / traverse graph | `docs/wiki/skills/wiki-navigate/SKILL.md` |
| Classify inbox drops | `docs/wiki/skills/wiki-triage/SKILL.md` |
| Add research / sources | `docs/wiki/skills/wiki-ingest/SKILL.md` |
| Answer with citations | `docs/wiki/skills/wiki-query/SKILL.md` |
| Diagnose (no edits) | `docs/wiki/skills/wiki-doctor/SKILL.md` |
| Apply safe fixes | `docs/wiki/skills/wiki-heal/SKILL.md` |
| Doctor + heal shortcut | `docs/wiki/skills/wiki-lint/SKILL.md` |
| Normalize frontmatter | `docs/wiki/skills/wiki-label/SKILL.md` |
| Sync code ↔ wiki ↔ GRAPH | `docs/wiki/skills/wiki-maintain/SKILL.md` |
| Usage / telemetry | `docs/wiki/skills/wiki-usage/SKILL.md` |

## Input path
```
chat paste / URL / note  →  docs/wiki/inbox/  →  triage  →  ingest  →  wiki + GRAPH + log
```
Do not invent folders during ingest. Taxonomy changes go through SCHEMA §7.

## Memory path
```
decision/failure  →  episodic/YYYY-MM-DD-*.md  →  temporal/TIMELINE.md
durable lesson    →  consolidate into semantic page  →  mark episode stale
as-of question    →  TIMELINE slice + retrieve --as-of
```

## Health path
```
wiki doctor  →  (optional) wiki heal  →  wiki doctor again
```

## Usage path
```
retrieve/doctor (auto JSONL)  →  wiki_usage.py report  →  _meta/usage-dashboard.md
dump INDEX?  →  log op=dump  (hurts usefulness index)
```

Bootstrap:
1. Host `AGENTS.md` (brief)
2. `docs/wiki/ROUTER.md`
3. `python3 docs/wiki/scripts/wiki_retrieve.py "<task>" --budget-tokens 3500`
4. Open the matching playbook under `docs/wiki/skills/`

Deep manual: `docs/wiki/OPERATOR.md`  
Portability: `docs/wiki/FRAMEWORK.md`
