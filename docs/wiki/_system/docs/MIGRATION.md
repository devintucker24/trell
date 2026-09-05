---
id: repobrain-migration
title: RepoBrain alias and path contraction
type: meta
status: active
created: 2026-09-05
updated: 2026-09-05
tags: [repobrain, migration, aliases]
domain: meta
summary: Removal of temporary wiki-* skill aliases and pre-_system script shims; replacements for operators.
nodes:
  - id: repobrain-migration
    kind: concept
    label: RepoBrain contraction
edges:
  - from: repobrain-migration
    to: wiki-router
    rel: related_to
related:
  - "[[_system/docs/FRAMEWORK]]"
  - "[[_system/docs/CHEATSHEET]]"
agent:
  priority: medium
  read_when:
    - looking up removed wiki-brain aliases
  maintain:
    - keep replacement table current after further renames
---

# RepoBrain contraction notes

`docs/wiki/` remains the corpus storage root. Engine code lives in
`docs/wiki/_system/`. Temporary `wiki-*` skill aliases and
`docs/wiki/scripts/` shims are removed.

## Replacements

| Removed | Use instead |
|---|---|
| `wiki-brain`, `wiki-retrieve`, `wiki-query`, … skills | `repobrain-*` under `docs/wiki/_system/skills/` |
| `.cursor/.claude/.agents/skills/wiki-*` | `.cursor/.claude/.agents/skills/repobrain-*` |
| `docs/wiki/scripts/*.py` shims | `docs/wiki/_system/scripts/*.py` or `./repobrain …` |
| `wiki_setup.py` as the public command | `./repobrain setup` |
| `wiki_retrieve.py` as the public command | `./repobrain retrieve` |
| `wiki_graphify.py` as the public command | `./repobrain graph` |
| `wiki_doctor.py` as the public command | `./repobrain doctor` |
| `wiki_usage.py` as the public command | `./repobrain usage` |

Internal Python module filenames under `_system/scripts/` (for example
`wiki_retrieve.py`) still exist so existing imports keep working. They are not
product names. The public surface is `./repobrain`.

Historical tags and episode titles that mention “wiki-brain” are storage
history, not live skill discovery.
