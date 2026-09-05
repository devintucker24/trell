# Paste into the host project's AGENTS.md (keep the rest of AGENTS.md project-specific).

## RepoBrain

Host knowledge lives in `docs/wiki/`; portable machinery lives in
`docs/wiki/_system/`.

Every wiki/memory/research task:

1. Read `docs/wiki/_system/docs/ROUTER.md`.
2. Read `docs/wiki/_system/config/router-seeds.md`.
3. Retrieve:

```bash
python3 docs/wiki/_system/scripts/wiki_retrieve.py "<question>" --budget-tokens 3500
```

| Need | Skill (under `docs/wiki/_system/skills/`) |
|------|-----------------------------------|
| Answer | `wiki-retrieve` / `wiki-query` |
| New material | inbox → `wiki-triage` → `wiki-ingest` |
| Health | `wiki-doctor` → `wiki-heal` |
| Context cost | `wiki-usage` |
| Export this brain | `_system/docs/FRAMEWORK.md` + `wiki-setup` |
| Code wiring | `wiki_graphify.py query` |

New checkout: `python3 docs/wiki/_system/scripts/wiki_setup.py`
