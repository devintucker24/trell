---
name: trell-wiki
description: Operate the Trell wiki brain (retrieve, triage, ingest, doctor, heal, maintain). Use for knowledge base, inbox, episodic/temporal memory, or AGENTS/ROUTER questions.
---

# Trell Wiki

Canonical parent skill: `skills/wiki/SKILL.md`

Bootstrap:
1. `AGENTS.md` (brief)
2. `docs/wiki/ROUTER.md`
3. `python3 skills/wiki/scripts/wiki_retrieve.py "<task>" --budget-tokens 3500`
4. Pick a subskill under `skills/wiki/` or `.cursor/skills/wiki-*`

Deep manual: `docs/wiki/OPERATOR.md`
