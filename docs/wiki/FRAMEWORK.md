---
id: wiki-framework
title: Wiki-brain pack — portable install and export
type: meta
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [pack, export, portable, wiki-brain, schema]
domain: meta
summary: "How to copy this wiki-brain into another repo: portable files vs host overlay, launchers, and HOST.yaml."
nodes:
  - id: wiki-brain-pack
    kind: concept
    label: Wiki-brain pack
  - id: wiki-brain
    kind: concept
    label: Wiki-brain
edges:
  - from: wiki-brain-pack
    to: wiki-schema
    rel: depends_on
  - from: wiki-brain-pack
    to: wiki-router
    rel: depends_on
  - from: wiki-brain-pack
    to: wiki-usage-telemetry
    rel: related_to
  - from: wiki-brain
    to: wiki-brain-pack
    rel: implements
related:
  - "[[SCHEMA]]"
  - "[[OPERATOR]]"
  - "[[ROUTER]]"
  - "[[_meta/usage-telemetry]]"
agent:
  priority: high
  read_when:
    - "exporting or installing the wiki into another project"
    - "deciding where skills live"
  maintain:
    - "keep pack/manifest.yaml aligned with real portable paths"
---

# Wiki-brain pack (portable)

The **pack** is the operator: schema, skills, scripts, router, inbox/episodic/temporal machinery.  
The **host** is the project: thesis, domain pages, `HOST.yaml`, Cursor/Claude launchers.

Matt Pocock (or any other) skills stay in `.cursor/skills/` of the **host** repo. They are not part of this pack.

## Layout

| Path | Pack or host? | Role |
|------|----------------|------|
| `docs/wiki/skills/` | **pack** | Canonical playbooks (`wiki-brain`, retrieve, doctor, …) |
| `docs/wiki/scripts/` | **pack** | `wiki_retrieve.py`, `wiki_doctor.py`, `wiki_usage.py`, `wiki_pack.py`, `sync_graph.py` |
| `docs/wiki/SCHEMA.md` `OPERATOR.md` `ROUTER.md` | **pack** | Contracts + progressive disclosure |
| `docs/wiki/pack/` | **pack** | Templates + `manifest.yaml` (not wiki pages) |
| `docs/wiki/HOST.yaml` | **host** | Project name, domains, semantic dirs, code roots |
| `docs/wiki/host/` | **host** | Intent → seed table and other overlays |
| `docs/wiki/core/` … domain folders | **host** | Compiled knowledge |
| `.cursor/skills/wiki-*` | **host adapter** | Thin launchers → `docs/wiki/skills/` |
| Root `skills/` | **not used** | Do not duplicate the pack here |

## Export (from this repo)

```bash
python3 docs/wiki/scripts/wiki_pack.py export /path/to/other-repo
```

Creates `other-repo/docs/wiki/` with portable files and stub `HOST.yaml` / `host/router-seeds.md` if missing. Does **not** copy Trell `core/`, `applications/`, etc.

Then in the other repo:

```bash
python3 docs/wiki/scripts/wiki_pack.py install-launchers
```

Paste `docs/wiki/pack/AGENTS.fragment.md` into that project's `AGENTS.md`. Fill `HOST.yaml` and the router-seeds table. Add domain pages. Run `wiki_doctor.py`.

## Plug-in checklist

1. `docs/wiki/` exists with skills + scripts + SCHEMA/OPERATOR/ROUTER.
2. `HOST.yaml` lists `domains` and `semantic_dirs` you actually have.
3. `host/router-seeds.md` maps *your* keywords to *your* pages.
4. Thin launchers exist for the agent harness you use (Cursor and/or Claude).
5. Python 3 + PyYAML: `python3 -c "import yaml"`.
6. Retrieve works: `python3 docs/wiki/scripts/wiki_retrieve.py "test query" --budget-tokens 1500`.

## Why skills live in the wiki

Cursor/Claude discovery folders are **harness-specific**. The wiki must still work in Codex, cloud agents, or a future public `wiki-brain` repo. Canonical `SKILL.md` files therefore live under `docs/wiki/skills/`. Launchers are one-screen pointers.

## What is not portable

Host thesis, product claims, competitor pages, and `AGENTS.md` hard rules. Do not copy those into another company repo.
