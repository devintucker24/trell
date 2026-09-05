---
id: wiki-framework
title: RepoBrain engine — portable install and export
type: meta
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [pack, export, portable, repobrain, schema]
domain: meta
summary: "How to copy RepoBrain into another repo: engine files, host overlay, Graphify code graph, and setup."
nodes:
  - id: wiki-brain-pack
    kind: concept
    label: RepoBrain engine
  - id: wiki-brain
    kind: concept
    label: RepoBrain
  - id: wiki-setup
    kind: concept
    label: RepoBrain setup
  - id: wiki-graphify-bridge
    kind: concept
    label: Graphify code-graph bridge
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
  - from: wiki-brain-pack
    to: wiki-setup
    rel: depends_on
  - from: wiki-graphify-bridge
    to: wiki-brain-pack
    rel: implements
  - from: wiki-brain
    to: wiki-brain-pack
    rel: implements
related:
  - "[[SCHEMA]]"
  - "[[OPERATOR]]"
  - "[[ROUTER]]"
  - "[[_system/docs/GRAPH]]"
  - "[[_system/docs/GRAPHIFY]]"
  - "[[_system/docs/usage-telemetry]]"
agent:
  priority: high
  read_when:
    - "exporting or installing the wiki into another project"
    - "deciding where skills live"
    - "first-run setup in a new repo"
    - "whether to use Graphify vs GRAPH.yaml"
  maintain:
    - "keep pack/manifest.yaml aligned with real portable paths"
---

# RepoBrain engine (portable)

The **pack** is the operator: schema, skills, scripts, router, inbox/episodic/temporal machinery.  
The **host** is the project: thesis, domain pages, `HOST.yaml`, Cursor/Claude launchers.

Matt Pocock (or any other) skills stay in `.cursor/skills/` of the **host** repo. They are not part of this pack.

## Layout

| Path | Pack or host? | Role |
|------|----------------|------|
| `docs/wiki/_system/skills/` | **engine** | Canonical playbooks |
| `docs/wiki/_system/scripts/` | **engine** | Operators + central path resolver |
| `docs/wiki/_system/docs/` | **engine** | Schema, operator, router, framework |
| `docs/wiki/_system/templates/` | **engine** | Setup/export templates |
| `docs/wiki/_system/config/` | **host config** | Host overlay, router seeds, eval contract |
| `docs/wiki/_system/generated/` | **generated** | Claim graph, doctor/eval/usage summaries |
| `docs/wiki/core/` … domain folders | **host** | Compiled knowledge |
| `.cursor/skills/repobrain-*` | **host adapter** | Thin launchers → canonical playbooks |
| Root `skills/` | **not used** | Do not duplicate the pack here |

## Export (from this repo)

```bash
python3 docs/wiki/_system/scripts/wiki_pack.py export /path/to/other-repo
```

Creates `other-repo/repobrain` and `other-repo/docs/wiki/_system/` with portable files plus stub host config. It does **not** copy Trell `core/`, `applications/`, or other host corpus pages.

Then **in the other repo** run setup (this is the agent install skill):

```bash
./repobrain setup --seed-pages
```

That fills `HOST.yaml` from the repo layout, installs launchers, gitignores `graphify-out/`, extracts the Graphify **code** graph (`--code-only`, no LLM), and only if the corpus is empty writes **draft** seed pages from god nodes. Remaining human/agent work: write `HOST.yaml` `anchor`, fill router-seeds, review drafts.

Playbook: `docs/wiki/_system/skills/repobrain-setup/SKILL.md`.

Graphify is optional. Supported versions and the exact install command are in
`docs/wiki/_system/docs/GRAPHIFY.md`.

## Two graphs (machines, not humans)

The wiki does **not** ship a homegrown code knowledge graph. Graphify already does that well (tree-sitter AST, EXTRACTED vs INFERRED edges, `query` / `path` / `explain` / `god-nodes`, optional `--wiki`).

| Graph | Path | What it stores | Who writes it |
|-------|------|----------------|---------------|
| **Code / structure** | `graphify-out/graph.json` (gitignored) | symbols, calls, imports, communities | Graphify (`repobrain graph sync`) |
| **Claims** | corpus YAML → `_system/generated/claim-graph.yaml` | reviewed doctrine | RepoBrain |

Agents query Graphify for “what calls what”. They retrieve wiki pages for “what we assert”. Do not dump either file into context.

`graphify export wiki` produces crawlable community/god-node markdown under `graphify-out/wiki/`. Those articles are **regenerated structure**, not SCHEMA frontmatter, not Trell thesis. Do not ingest them as doctrine.

## Are corpus pages auto-generated?

**Structural pages:** yes — Graphify wiki export + `graph.json`, rebuilt whenever code changes.

**Setup seeds:** once, and only on an empty host, `wiki_setup.py --seed-pages` writes `status: draft` stubs tagged `graphify-seed` pointing at `implements_code`. An agent must fill claims from source; until then they are not wiki truth.

**Compiled corpus** (`core/`, `theory/`, host domains): **no**. Those are authored through inbox → triage → ingest (same PR, human review). Graphify cannot invent `certain` vs `belief` or `reduces_via`. Setup organizes folders and config so that work is small, not so that the thesis is hallucinated from the AST.

## Plug-in checklist

1. `docs/wiki/` exists with skills + scripts + SCHEMA/OPERATOR/ROUTER.
2. `HOST.yaml` lists `domains` and `semantic_dirs` you actually have.
3. `host/router-seeds.md` maps *your* keywords to *your* pages.
4. Thin launchers exist for the agent harness you use (Cursor and/or Claude).
5. Python 3 + PyYAML: `python3 -c "import yaml"`.
6. Retrieve works through `docs/wiki/_system/scripts/wiki_retrieve.py`.
7. Graphify status works through `_system/scripts/wiki_graphify.py`.
8. `HOST.yaml` `anchor` is a real thesis paragraph, not the template.

## Why skills live in the wiki

Harness discovery folders are adapters. Canonical playbooks live under
`docs/wiki/_system/skills/`; launchers are one-screen pointers.

## What is not portable

Host thesis, product claims, competitor pages, and `AGENTS.md` hard rules. Do not copy those into another company repo.
