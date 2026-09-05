---
name: repobrain-setup
description: Stand up the portable RepoBrain engine in this repo (or after export) with minimal human work — HOST.yaml, folders, launchers, Graphify code graph, optional seed pages. Use when installing the pack in a new project, cloning onto a machine, or moving the brain.
---

# Skill: RepoBrain Setup

## When to use
- First install of RepoBrain in a **new repo**
- After `wiki_pack.py export /path/to/other-repo`
- Agent lands in a checkout that has the pack but empty `HOST.yaml` / no launchers
- User says “set up RepoBrain” or the historical aliases “set up the wiki” / “make this portable” / “bootstrap the brain”

Do **not** use this to rewrite Trell doctrine pages. Setup is idempotent and will not clobber a filled corpus.

## What “minimal” means

The human (or the installing agent) should only have to:

1. Confirm `docs/wiki/_system/config/HOST.yaml` `anchor`
2. Skim `host/router-seeds.md` after the agent fills obvious keywords
3. Review any **draft seed pages** (`status: draft`, tag `graphify-seed`) — they are not truth yet

Everything else is mechanical: folders, launchers, gitignore, Graphify AST extract, claim-graph compile, doctor.

## Procedure

From a repo that already has the RepoBrain engine:

```bash
./repobrain setup
# empty corpus only — draft pages from Graphify god nodes:
./repobrain setup --seed-pages
```

If the pack is not in the dest repo yet, run this **from the source RepoBrain repository**:

```bash
python3 docs/wiki/_system/scripts/wiki_pack.py export /path/to/other-repo
# then in the other repo:
./repobrain setup --seed-pages
```

Flags:
- `--dry-run` — detect + print, no writes
- `--no-graphify` — skip code-graph extract
- `--no-sources` — skip the Git-tracked source inventory scan
- `--seed-pages` — write draft concept stubs from Graphify god nodes **only if** semantic dirs are almost empty
- `--force-seed` — seed even if pages already exist (still skips colliding slugs)

## What the script does

1. Detect repo name (git remote), `src/`/`lib/`/… code roots, `README.md`/`THESIS.md`
2. Write or **merge** `HOST.yaml` (never overwrite a filled file; may append `graphify:`)
3. Create `inbox/`, `episodic/`, `temporal/`, `raw/`, `host/`, domain folders from HOST
4. Stub `INDEX.md` / `log.md` / router-seeds **if missing**
5. Gitignore `graphify-out/`
6. Install `.cursor/skills/repobrain-*`, `.claude/skills/repobrain-*`, and `.agents/skills/repobrain-*` launchers
7. Append `pack/AGENTS.fragment.md` only if `AGENTS.md` does not already mention retrieve
8. `./repobrain graph sync` → `graphify extract --code-only` (no LLM)
9. `./repobrain source scan` → deterministic manifest and grouped raw pointers
10. Optional seed pages from god nodes
11. `sync_graph.py` (claim index from frontmatter)

## Corpus: auto-generated or authored?

| Layer | Auto? | Owner |
|-------|-------|--------|
| Code/structure graph `graphify-out/graph.json` | **Yes** — regenerate on code change | Graphify |
| Graphify `--wiki` community/god-node articles | **Yes** — regenerated, not SCHEMA doctrine | Graphify |
| Draft seed pages (`graphify-seed`) | **Once** at empty-repo setup | Setup script |
| Compiled claim pages (`core/`, …) | **No** — agent-authored via inbox → ingest, then human-reviewed in the PR | Wiki |
| `_system/generated/claim-graph.yaml` | **Yes** — compiled from corpus frontmatter | `sync_graph.py` |
| `_system/generated/sources/manifest.json` | **Yes** — inventory of Git-tracked project sources | Source pipeline |
| `_system/generated/sources/cache/` | **Yes, ignored** — local derived Markdown | MarkItDown adapter |

Graphify cannot emit Trell-style `reduces_via` / `contradicts`. Do not treat god-node articles as the thesis.

## After setup (agent)

1. Set `HOST.yaml` `anchor`
2. Map keywords in `host/router-seeds.md`
3. Fill or delete seed drafts by reading `implements_code` files — **do not paste source into wiki pages**
4. `./repobrain doctor`
5. `./repobrain retrieve "what is this repo" --budget-tokens 1500`

Code questions: `./repobrain graph query "…"`

Supported Graphify install:
`python3 -m pip install --user 'graphifyy>=0.9.54,<0.10'`.

Supported local CSV conversion install:
`python3 -m pip install --user 'markitdown==0.1.7'`.

Operator: `docs/wiki/_system/docs/FRAMEWORK.md`
Claim vs code graphs: `docs/wiki/_system/docs/GRAPH.md`
Adapter contract: `docs/wiki/_system/docs/GRAPHIFY.md`
Source contract: `docs/wiki/_system/docs/SOURCES.md`
