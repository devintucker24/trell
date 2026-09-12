---
id: repobrain-install
title: Install RepoBrain into a host repository
type: meta
status: active
created: 2026-09-12
updated: 2026-09-12
tags: [repobrain, setup, install, bootstrap]
domain: meta
summary: Agent playbook to clone the empty engine and stand it up in the current repo.
nodes:
  - id: repobrain-install
    kind: concept
    label: RepoBrain install
edges:
  - from: repobrain-install
    to: wiki-setup
    rel: implements
  - from: repobrain-install
    to: wiki-brain-pack
    rel: depends_on
related:
  - "[[FRAMEWORK]]"
  - "[[_system/skills/repobrain-setup/SKILL]]"
agent:
  priority: critical
  read_when:
    - installing RepoBrain
    - user pastes the bootstrap prompt
    - the host repo has no ./repobrain yet
  maintain:
    - keep the paste prompt identical to README.md
---

# Install RepoBrain

This engine is a **file pack** in the host git repo. It is not an MCP server,
Cursor marketplace plugin, or hosted RAG API. After setup, thin launchers live
in `.cursor/skills/repobrain-*` and `.claude/skills/repobrain-*`. Those are
pointers at `docs/wiki/_system/skills/`. Without `_system/`, the launchers are
empty.

Do not copy another host's compiled wiki pages. Install copies the engine only.

## Paste this prompt (any agent, in the host repo)

```text
Install RepoBrain into this repository.

RepoBrain is a portable file-native knowledge engine. Clone it, copy the
engine into this repo, and run setup. Do not copy any other project's wiki
pages.

1. Require Python 3.10+ and `python3 -m pip install --user pyyaml`.
2. If ./repobrain is missing:
   git clone --depth 1 https://github.com/devintucker24/RepoBrain.git /tmp/RepoBrain
   chmod +x /tmp/RepoBrain/repobrain /tmp/RepoBrain/bootstrap.sh
   /tmp/RepoBrain/repobrain bootstrap "$PWD"
   (equivalent: /tmp/RepoBrain/bootstrap.sh "$PWD")
3. If ./repobrain already exists: ./repobrain setup
4. Edit docs/wiki/_system/config/HOST.yaml: set `name` and `anchor` from this
   repo's README. Map keywords in docs/wiki/_system/config/router-seeds.md.
   Do not invent new wiki folders.
5. ./repobrain doctor
6. ./repobrain retrieve "what is this repo" --budget-tokens 1500
7. Stop. Remaining pages are written via inbox → triage → ingest.

Done when: ./repobrain --help works, HOST.yaml has a real anchor, doctor has
no critical/high findings you introduced, and this repo has no other project's
compiled wiki pages.
```

## From a RepoBrain clone (humans)

```bash
./bootstrap.sh /path/to/your-project
# or
./repobrain bootstrap /path/to/your-project
```

Then in the host repo: fill `HOST.yaml` `anchor`, run `./repobrain doctor`.

## Completion criteria

| Step | Done when |
|------|-----------|
| Clone | `/tmp/RepoBrain/repobrain` is executable |
| Bootstrap | host has `./repobrain` and `docs/wiki/_system/docs/SCHEMA.md` |
| Setup | `.cursor/skills/repobrain-setup/SKILL.md` exists; `INDEX.md` exists |
| Overlay | `HOST.yaml` `name` is this repo, `anchor` is not the template sentence |
| Verify | `./repobrain retrieve "what is this repo"` returns host hits |

Graphify and MarkItDown stay optional. Skip them unless the host needs a code
graph or office/PDF conversion.
