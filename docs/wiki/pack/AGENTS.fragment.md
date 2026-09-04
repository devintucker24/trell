# Paste into the host project's AGENTS.md (keep the rest of AGENTS.md project-specific).

## Wiki brain (portable pack)

Knowledge lives in `docs/wiki/`. Canonical skills: `docs/wiki/skills/`. Scripts: `docs/wiki/scripts/`.
Host overlay: `docs/wiki/HOST.yaml` and `docs/wiki/host/router-seeds.md`.

Every wiki/memory/research task:

1. Read `docs/wiki/ROUTER.md` (do not dump INDEX).
2. Read host router seeds.
3. Retrieve:

```bash
python3 docs/wiki/scripts/wiki_retrieve.py "<question>" --budget-tokens 3500
```

| Need | Skill (under `docs/wiki/skills/`) |
|------|-----------------------------------|
| Answer | `wiki-retrieve` / `wiki-query` |
| New material | inbox → `wiki-triage` → `wiki-ingest` |
| Health | `wiki-doctor` → `wiki-heal` |
| Context cost | `wiki-usage` |
| Export this brain | `docs/wiki/FRAMEWORK.md` + `wiki-setup` |
| Code wiring | `wiki_graphify.py query` |

New checkout: `python3 docs/wiki/scripts/wiki_setup.py`
