---
name: wiki-brain
description: Operate the portable RepoBrain engine over a host corpus — setup, retrieve, navigate, triage, ingest, query, doctor, heal, lint, label, maintain, Graphify, and usage.
---

# Wiki-brain — Parent Skill (host-agnostic)

This family implements the **Karpathy LLM Wiki** pattern as a **file RAG + multi-lane memory** system. Domain content (this repo: Trell) is an *instance*; the operator kit is reusable.

**Canonical playbooks:** `docs/wiki/_system/skills/*/SKILL.md`
**Adapters only:** `.cursor/skills/wiki-*` · `.claude/skills/wiki-*`  
**Export kit:** `docs/wiki/_system/docs/FRAMEWORK.md`

## Architecture
- **Schema (thin):** host project brief (`AGENTS.md`, optional `CLAUDE.md`)
- **Schema (deep):** `docs/wiki/_system/docs/`, `docs/wiki/_system/skills/`
- **Wiki:** `docs/wiki/**` with YAML nodes/edges
- **Scripts:** `docs/wiki/_system/scripts/`
- **Memory lanes:** semantic pages · `episodic/` · `temporal/TIMELINE.md`
- **Context:** `_system/docs/ROUTER.md` + `_system/docs/CONTEXT_PROTOCOL.md`
- **Inbox:** `docs/wiki/inbox/` → triage → ingest
- **Usage:** `_system/generated/usage/`

## Subskills (read the canonical files)

| Task | Canonical |
|------|-----------|
| **Retrieve / file RAG** | `_system/skills/wiki-retrieve/SKILL.md` |
| Find pages / traverse graphs | `_system/skills/wiki-navigate/SKILL.md` |
| **First install / new repo** | `_system/skills/wiki-setup/SKILL.md` |
| Classify inbox drops | `_system/skills/wiki-triage/SKILL.md` |
| Add research / sources | `_system/skills/wiki-ingest/SKILL.md` |
| Answer with citations | `_system/skills/wiki-query/SKILL.md` |
| Diagnose (no edits) | `_system/skills/wiki-doctor/SKILL.md` |
| Apply safe fixes | `_system/skills/wiki-heal/SKILL.md` |
| Doctor + heal shortcut | `_system/skills/wiki-lint/SKILL.md` |
| Normalize frontmatter | `_system/skills/wiki-label/SKILL.md` |
| Sync code ↔ wiki ↔ Graphify | `_system/skills/wiki-maintain/SKILL.md` |
| Usage / telemetry | `_system/skills/wiki-usage/SKILL.md` |

## Input path
```
chat paste / URL / note  →  docs/wiki/inbox/  →  triage  →  ingest  →  wiki + claim GRAPH + log
code change                 →  wiki_graphify.py sync  →  maintain claim pages
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
retrieve/doctor (auto JSONL)  →  wiki_usage.py report  →  generated/usage/dashboard.md
dump INDEX?  →  log op=dump  (hurts usefulness index)
```

Bootstrap:
1. Host `AGENTS.md` (brief)
2. `docs/wiki/_system/docs/ROUTER.md`
3. `python3 docs/wiki/_system/scripts/wiki_retrieve.py "<task>" --budget-tokens 3500`
4. Open the matching playbook under `docs/wiki/_system/skills/`

Deep manual: `docs/wiki/_system/docs/OPERATOR.md`
Portability: `docs/wiki/_system/docs/FRAMEWORK.md`
New clone / empty host: `docs/wiki/_system/skills/wiki-setup/SKILL.md`
Code graph: `python3 docs/wiki/_system/scripts/wiki_graphify.py query "<q>"`
