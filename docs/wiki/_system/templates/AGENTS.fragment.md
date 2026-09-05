# Paste into the host project's AGENTS.md (keep the rest of AGENTS.md project-specific).

## RepoBrain

Host knowledge lives in `docs/wiki/`; portable machinery lives in
`docs/wiki/_system/`.

Every wiki/memory/research task:

1. Read `docs/wiki/_system/docs/ROUTER.md`.
2. Read `docs/wiki/_system/config/router-seeds.md`.
3. Retrieve:

```bash
./repobrain retrieve "<question>" --budget-tokens 3500
```

| Need | Skill (under `docs/wiki/_system/skills/`) |
|------|-----------------------------------|
| Answer | `repobrain-retrieve` / `repobrain-query` |
| New material | inbox → `repobrain-triage` → `repobrain-ingest` |
| Health | `repobrain-doctor` → `repobrain-heal` |
| Context cost | `repobrain-usage` |
| Code wiring | `./repobrain graph query` |
| Export this engine | `_system/docs/FRAMEWORK.md` + `repobrain-setup` |

New checkout: `./repobrain setup`
