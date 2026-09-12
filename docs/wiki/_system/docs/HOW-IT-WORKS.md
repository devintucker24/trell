# How RepoBrain works

RepoBrain is a **file pack** in your git repo. It is not an MCP server, a
Cursor marketplace plugin, or a hosted RAG API. After install, the brain is
the files on disk.

## Two parts

| Part | Where | What |
|------|--------|------|
| Engine | `./repobrain`, `docs/wiki/_system/` | Skills, scripts, schema, router |
| Host wiki | `docs/wiki/` minus `_system/` | **Your** claims |

Install copies the engine only. Thin launchers in `.cursor/skills/repobrain-*`
point at `_system/skills/`. Without `_system/`, those launchers are empty.

## Three kinds of knowledge

1. **Raw** — README, source, PDFs, ADRs. Inventory, not compiled truth.
2. **Inbox / episodes / timeline** — notes until triage → ingest.
3. **Compiled pages** — reviewed markdown with schema frontmatter. This is
   what retrieve should cite.

Agents retrieve. They should not dump `docs/wiki/` into context. The map is
[`ROUTER.md`](ROUTER.md).

## Two graphs (do not mix them)

| Graph | File | Needs Graphify? |
|-------|------|-----------------|
| Claims | `_system/generated/claim-graph.yaml` | No. Built from page YAML. |
| Code | `graphify-out/graph.json` (gitignored) | **Yes.** AST / calls / imports. |

`./repobrain retrieve` uses the wiki (and the claim index). It does not need
Graphify.

`./repobrain graph query` uses the **code** graph. No Graphify means no code
graph, not a weaker wiki.

## Graphify (optional)

You do not have to install or use Graphify.

RepoBrain talks to Graphify through an **adapter**
([`GRAPHIFY.md`](GRAPHIFY.md)). Graphify owns parsing and the code graph.
RepoBrain owns config, `./repobrain graph sync`, and diagnostics. RepoBrain
does not vendor Graphify.

- Upstream: [Graphify-Labs/graphify](https://github.com/Graphify-Labs/graphify)
- PyPI package: `graphifyy` (CLI command is still `graphify`)
- Supported range: `graphifyy>=0.9.54,<0.10`

Skip it unless you want agents to query code structure. Wiki install,
retrieve, inbox, and doctor work without it.

## What install does not do

It does not copy another host's compiled pages. It does not invent a
`docs/wiki/core/` folder. It does not require Graphify or MarkItDown.
