---
id: wiki-framework
title: Wiki-brain pack — portable install and export
type: meta
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [pack, export, portable, wiki-brain, schema]
domain: meta
summary: "How to copy this wiki-brain into another repo: portable files vs host overlay, Graphify code graph, and wiki-setup."
nodes:
  - id: wiki-brain-pack
    kind: concept
    label: Wiki-brain pack
  - id: wiki-brain
    kind: concept
    label: Wiki-brain
  - id: wiki-setup
    kind: concept
    label: Wiki setup
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
  - "[[_meta/GRAPH]]"
  - "[[_meta/usage-telemetry]]"
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

# Wiki-brain pack (portable)

The **pack** is the operator: schema, skills, scripts, router, inbox/episodic/temporal machinery.  
The **host** is the project: thesis, domain pages, `HOST.yaml`, Cursor/Claude launchers.

Matt Pocock (or any other) skills stay in `.cursor/skills/` of the **host** repo. They are not part of this pack.

## Layout

| Path | Pack or host? | Role |
|------|----------------|------|
| `docs/wiki/skills/` | **pack** | Canonical playbooks (`wiki-brain`, retrieve, doctor, …) |
| `docs/wiki/scripts/` | **pack** | `wiki_retrieve.py`, `wiki_doctor.py`, `wiki_usage.py`, `wiki_pack.py`, `wiki_setup.py`, `wiki_graphify.py`, `sync_graph.py` |
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

Then **in the other repo** run setup (this is the agent install skill):

```bash
python3 docs/wiki/scripts/wiki_setup.py --seed-pages
```

That fills `HOST.yaml` from the repo layout, installs launchers, gitignores `graphify-out/`, extracts the Graphify **code** graph (`--code-only`, no LLM), and only if the corpus is empty writes **draft** seed pages from god nodes. Remaining human/agent work: write `HOST.yaml` `anchor`, fill router-seeds, review drafts.

Playbook: `docs/wiki/skills/wiki-setup/SKILL.md`.

Need Graphify: `pip install graphifyy` (CLI is `graphify`).

## Two graphs (machines, not humans)

The wiki does **not** ship a homegrown code knowledge graph. Graphify already does that well (tree-sitter AST, EXTRACTED vs INFERRED edges, `query` / `path` / `explain` / `god-nodes`, optional `--wiki`).

| Graph | Path | What it stores | Who writes it |
|-------|------|----------------|---------------|
| **Code / structure** | `graphify-out/graph.json` (gitignored) | symbols, calls, imports, communities | Graphify (`wiki_graphify.py sync`) |
| **Claims** | page YAML `nodes`/`edges` → `_meta/GRAPH.yaml` | `reduces_via`, `contradicts`, doctrine | Agents compiling wiki pages |

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
6. Retrieve works: `python3 docs/wiki/scripts/wiki_retrieve.py "test query" --budget-tokens 1500`.
7. Graphify installed (`pip install graphifyy`) and `python3 docs/wiki/scripts/wiki_graphify.py status` shows a graph.json.
8. `HOST.yaml` `anchor` is a real thesis paragraph, not the template.

## Why skills live in the wiki

Cursor/Claude discovery folders are **harness-specific**. The wiki must still work in Codex, cloud agents, or a future public `wiki-brain` repo. Canonical `SKILL.md` files therefore live under `docs/wiki/skills/`. Launchers are one-screen pointers.

## What is not portable

Host thesis, product claims, competitor pages, and `AGENTS.md` hard rules. Do not copy those into another company repo.
