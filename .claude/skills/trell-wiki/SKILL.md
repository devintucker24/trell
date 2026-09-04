---
name: trell-wiki
description: Operate the Trell Karpathy-style epistemic wiki brain — retrieve, navigate, triage, ingest, query, doctor, heal, lint, label, and maintain docs/wiki with YAML graph metadata plus episodic/temporal memory. Use whenever working with Trell knowledge base, AGENTS.md, or long-term research docs.
---

# Trell Wiki Brain — Parent Skill

This skill family implements the **Karpathy LLM Wiki** pattern for Trell, extended as a **file RAG + multi-lane memory** system.

## Architecture
- **Schema (thin):** root `AGENTS.md`, `CLAUDE.md`
- **Schema (deep):** `docs/wiki/OPERATOR.md`, `docs/wiki/SCHEMA.md`, `.cursor/skills/wiki-*`
- **Cursor discovery:** `.cursor/rules/*.mdc`, `.cursor/skills/*/SKILL.md`
- **Wiki:** `docs/wiki/**` with YAML nodes/edges
- **Scripts:** `docs/wiki/scripts/` (`wiki_retrieve.py`, `wiki_doctor.py`, `sync_graph.py`)
- **Memory lanes:** semantic pages · `episodic/` · `temporal/TIMELINE.md`
- **Context:** `docs/wiki/ROUTER.md` + `_meta/CONTEXT_PROTOCOL.md` (progressive disclosure)
- **Raw:** `docs/wiki/raw/`, `THESIS.md`, `examples/`, `src/`
- **Inbox:** `docs/wiki/inbox/` → triage → ingest

## Subskills
| Task | Skill |
|------|-------|
| **Retrieve / file RAG** | [.cursor/skills/wiki-retrieve/SKILL.md](../wiki-retrieve/SKILL.md) |
| Find pages / traverse graph | [.cursor/skills/wiki-navigate/SKILL.md](../wiki-navigate/SKILL.md) |
| Classify inbox drops | [.cursor/skills/wiki-triage/SKILL.md](../wiki-triage/SKILL.md) |
| Add research / sources | [.cursor/skills/wiki-ingest/SKILL.md](../wiki-ingest/SKILL.md) |
| Answer with citations | [.cursor/skills/wiki-query/SKILL.md](../wiki-query/SKILL.md) |
| Diagnose (no edits) | [.cursor/skills/wiki-doctor/SKILL.md](../wiki-doctor/SKILL.md) |
| Apply safe fixes | [.cursor/skills/wiki-heal/SKILL.md](../wiki-heal/SKILL.md) |
| Doctor + heal shortcut | [.cursor/skills/wiki-lint/SKILL.md](../wiki-lint/SKILL.md) |
| Normalize frontmatter | [.cursor/skills/wiki-label/SKILL.md](../wiki-label/SKILL.md) |
| Sync code ↔ wiki ↔ GRAPH | [.cursor/skills/wiki-maintain/SKILL.md](../wiki-maintain/SKILL.md) |

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

Bootstrap:
1. `AGENTS.md` (brief)
2. `docs/wiki/ROUTER.md`
3. `python3 docs/wiki/scripts/wiki_retrieve.py "<task>" --budget-tokens 3500`
4. Pick a subskill under `.cursor/skills/wiki-*`

Deep manual: `docs/wiki/OPERATOR.md`
