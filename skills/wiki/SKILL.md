---
name: trell-wiki
description: Operate the Trell Karpathy-style epistemic wiki brain — navigate, triage, ingest, query, doctor, heal, lint, label, and maintain docs/wiki with YAML graph metadata. Use whenever working with Trell knowledge base, AGENTS.md, or long-term research docs.
---

# Trell Wiki Brain — Parent Skill

This skill family implements the **Karpathy LLM Wiki** pattern for Trell.

## Architecture reminder
- **Schema:** `AGENTS.md`, `docs/wiki/SCHEMA.md`, `skills/wiki/*`
- **Wiki:** `docs/wiki/**` with YAML nodes/edges
- **Raw:** `docs/wiki/raw/`, `THESIS.md`, `examples/`, `src/`
- **Inbox:** `docs/wiki/inbox/` → triage → ingest

## Subskills
| Task | Skill |
|------|-------|
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

## Health path
```
wiki doctor  →  (optional) wiki heal  →  wiki doctor again
```

## References
- [frontmatter-schemas.md](references/frontmatter-schemas.md)
- Scripts: [scripts/sync_graph.py](scripts/sync_graph.py), [scripts/wiki_doctor.py](scripts/wiki_doctor.py)

## Session bootstrap
```
1. Read AGENTS.md
2. Read docs/wiki/INDEX.md
3. Choose subskill
4. Append docs/wiki/log.md when done
```
