---
name: wiki-setup
description: Stand up the portable wiki-brain in this repo (or after export) with minimal human work — HOST.yaml, folders, launchers, Graphify code graph, optional seed pages. Use when installing the pack in a new project, cloning onto a machine, or moving the brain.
---

# Skill: Wiki Setup

## When to use
- First install of wiki-brain in a **new repo**
- After `wiki_pack.py export /path/to/other-repo`
- Agent lands in a checkout that has the pack but empty `HOST.yaml` / no launchers
- User says “set up the wiki” / “make this portable” / “bootstrap the brain”

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
python3 docs/wiki/_system/scripts/wiki_setup.py
# empty corpus only — draft pages from Graphify god nodes:
python3 docs/wiki/_system/scripts/wiki_setup.py --seed-pages
```

If the pack is not in the dest repo yet, run this **from the source wiki-brain repo**:

```bash
python3 docs/wiki/_system/scripts/wiki_pack.py export /path/to/other-repo
# then in the other repo:
python3 docs/wiki/_system/scripts/wiki_setup.py --seed-pages
```

Flags:
- `--dry-run` — detect + print, no writes
- `--no-graphify` — skip code-graph extract
- `--seed-pages` — write draft concept stubs from Graphify god nodes **only if** semantic dirs are almost empty
- `--force-seed` — seed even if pages already exist (still skips colliding slugs)

Equivalent: `python3 docs/wiki/_system/scripts/wiki_pack.py setup`

## What the script does

1. Detect repo name (git remote), `src/`/`lib/`/… code roots, `README.md`/`THESIS.md`
2. Write or **merge** `HOST.yaml` (never overwrite a filled file; may append `graphify:`)
3. Create `inbox/`, `episodic/`, `temporal/`, `raw/`, `host/`, domain folders from HOST
4. Stub `INDEX.md` / `log.md` / router-seeds **if missing**
5. Gitignore `graphify-out/`
6. Install `.cursor/skills/wiki-*` and `.claude/skills/wiki-*` launchers
7. Append `pack/AGENTS.fragment.md` only if `AGENTS.md` does not already mention retrieve
8. `wiki_graphify.py sync` → `graphify extract --code-only` (no LLM)
9. Optional seed pages from god nodes
10. `sync_graph.py` (claim index from frontmatter)

## Corpus: auto-generated or authored?

| Layer | Auto? | Owner |
|-------|-------|--------|
| Code/structure graph `graphify-out/graph.json` | **Yes** — regenerate on code change | Graphify |
| Graphify `--wiki` community/god-node articles | **Yes** — regenerated, not SCHEMA doctrine | Graphify |
| Draft seed pages (`graphify-seed`) | **Once** at empty-repo setup | Setup script |
| Compiled claim pages (`core/`, …) | **No** — agent-authored via inbox → ingest, then human-reviewed in the PR | Wiki |
| `_system/generated/claim-graph.yaml` | **Yes** — compiled from corpus frontmatter | `sync_graph.py` |

Graphify cannot emit Trell-style `reduces_via` / `contradicts`. Do not treat god-node articles as the thesis.

## After setup (agent)

1. Set `HOST.yaml` `anchor`
2. Map keywords in `host/router-seeds.md`
3. Fill or delete seed drafts by reading `implements_code` files — **do not paste source into wiki pages**
4. `python3 docs/wiki/_system/scripts/wiki_doctor.py`
5. `python3 docs/wiki/_system/scripts/wiki_retrieve.py "what is this repo" --budget-tokens 1500`

Code questions: `python3 docs/wiki/_system/scripts/wiki_graphify.py query "…"`

Need Graphify: `pip install graphifyy` (CLI name is still `graphify`).

Operator: `docs/wiki/_system/docs/FRAMEWORK.md`
Claim vs code graphs: `docs/wiki/_system/docs/GRAPH.md`
