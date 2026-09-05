---
name: repobrain-brain
description: Operate the portable RepoBrain engine over a host corpus — setup, retrieve, navigate, triage, ingest, query, doctor, heal, lint, label, maintain, Graphify, and usage.
---

# RepoBrain — Parent Skill (host-agnostic)

This family implements the **Karpathy LLM Wiki** pattern as a **file RAG + multi-lane memory** system. Domain content (this repo: Trell) is an *instance*; the operator kit is reusable.

**Canonical playbooks:** `docs/wiki/_system/skills/*/SKILL.md`
**Adapters only:** `.cursor/skills/repobrain-*` · `.claude/skills/repobrain-*` · `.agents/skills/repobrain-*`  
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
| **Retrieve / file RAG** | `_system/skills/repobrain-retrieve/SKILL.md` |
| Find pages / traverse graphs | `_system/skills/repobrain-navigate/SKILL.md` |
| **First install / new repo** | `_system/skills/repobrain-setup/SKILL.md` |
| Classify inbox drops | `_system/skills/repobrain-triage/SKILL.md` |
| Add research / sources | `_system/skills/repobrain-ingest/SKILL.md` |
| Answer with citations | `_system/skills/repobrain-query/SKILL.md` |
| Diagnose (no edits) | `_system/skills/repobrain-doctor/SKILL.md` |
| Apply safe fixes | `_system/skills/repobrain-heal/SKILL.md` |
| Doctor + heal shortcut | `_system/skills/repobrain-lint/SKILL.md` |
| Normalize frontmatter | `_system/skills/repobrain-label/SKILL.md` |
| Sync code ↔ wiki ↔ Graphify | `_system/skills/repobrain-maintain/SKILL.md` |
| Usage / telemetry | `_system/skills/repobrain-usage/SKILL.md` |

## Input path
```
chat paste / URL / note  →  docs/wiki/inbox/  →  triage  →  ingest  →  wiki + claim GRAPH + log
code change                 →  ./repobrain graph sync  →  maintain claim pages
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
RepoBrain doctor ("wiki doctor")  →  optional heal ("wiki heal")  →  doctor again
```

## Usage path
```
retrieve/doctor (auto JSONL)  →  ./repobrain dashboard usage  →  generated/usage/dashboard.md
dump INDEX?  →  log op=dump  (hurts usefulness index)
```

Bootstrap:
1. Host `AGENTS.md` (brief)
2. `docs/wiki/_system/docs/ROUTER.md`
3. `./repobrain retrieve "<task>" --budget-tokens 3500`
4. Open the matching playbook under `docs/wiki/_system/skills/`

Deep manual: `docs/wiki/_system/docs/OPERATOR.md`
Portability: `docs/wiki/_system/docs/FRAMEWORK.md`
New clone / empty host: `docs/wiki/_system/skills/repobrain-setup/SKILL.md`
Code graph: `./repobrain graph query "<q>"`
