---
name: trell-wiki
description: Operate the Trell Karpathy-style epistemic wiki brain — retrieve, navigate, triage, ingest, query, doctor, heal, lint, label, and maintain docs/wiki with YAML graph metadata plus episodic/temporal memory. Use whenever working with Trell knowledge base, AGENTS.md, or long-term research docs.
---

# Trell Wiki Brain — Parent Skill

This skill family implements the **Karpathy LLM Wiki** pattern for Trell, extended as a **file RAG + multi-lane memory** system.

## Architecture reminder
- **Schema (thin):** root `AGENTS.md`, `CLAUDE.md`
- **Schema (deep):** `docs/wiki/OPERATOR.md`, `docs/wiki/SCHEMA.md`, `skills/wiki/*`
- **Cursor discovery:** `.cursor/rules/*.mdc`, `.cursor/skills/*/SKILL.md` (launchers)
- **Claude discovery:** `.claude/skills/*/SKILL.md` (same launchers)
- **Wiki:** `docs/wiki/**` with YAML nodes/edges
- **Memory lanes:** semantic pages · `episodic/` · `temporal/TIMELINE.md`
- **Context:** `docs/wiki/ROUTER.md` + `_meta/CONTEXT_PROTOCOL.md` (progressive disclosure)
- **Raw:** `docs/wiki/raw/`, `THESIS.md`, `examples/`, `src/`
- **Inbox:** `docs/wiki/inbox/` → triage → ingest

## Subskills
| Task | Skill |
|------|-------|
| **Retrieve / file RAG** | [retrieve/SKILL.md](retrieve/SKILL.md) |
| Find pages / traverse graph | [navigate/SKILL.md](navigate/SKILL.md) |
| Classify inbox drops | [triage/SKILL.md](triage/SKILL.md) |
| Add research / sources | [ingest/SKILL.md](ingest/SKILL.md) |
| Answer with citations | [query/SKILL.md](query/SKILL.md) |
| Diagnose (no edits) | [doctor/SKILL.md](doctor/SKILL.md) |
| Apply safe fixes | [heal/SKILL.md](heal/SKILL.md) |
| Doctor + heal shortcut | [lint/SKILL.md](lint/SKILL.md) |
| Normalize frontmatter | [label/SKILL.md](label/SKILL.md) |
| Sync code ↔ wiki ↔ GRAPH | [maintain/SKILL.md](maintain/SKILL.md) |

## Input path (remember this)
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

## References
- [frontmatter-schemas.md](references/frontmatter-schemas.md)
- Scripts: [scripts/wiki_retrieve.py](scripts/wiki_retrieve.py), [scripts/sync_graph.py](scripts/sync_graph.py), [scripts/wiki_doctor.py](scripts/wiki_doctor.py)

## Session bootstrap
```
1. Read AGENTS.md (brief) — Claude Code also CLAUDE.md
2. Read docs/wiki/ROUTER.md
3. retrieve "<task>"  (not full INDEX)
4. Choose subskill (skills/wiki/ or .cursor/skills/)
5. Append docs/wiki/log.md when done
```

Deep ops: `docs/wiki/OPERATOR.md`
