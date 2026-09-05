---
name: repobrain-ingest
description: Ingest triaged inbox items or approved sources into the Trell RepoBrain corpus — merge/create pages, raw pointers, frontmatter nodes/edges, GRAPH sync, INDEX, log. Use after triage, or when user names an explicit target page.
---

# Skill: RepoBrain Ingest

## When to use
- After **triage** sets `suggested_action` to `merge-existing`, `new-page`, or `raw-only`
- User explicitly: "update `market/competitive-analysis` with this"
- Shipping a language feature that must sync docs (still prefer inbox if messy)

## Prerequisite
If material is still a chat paste / URL / unknown blob → run **`repobrain-triage`** first (or drop into `docs/wiki/inbox/`).

---

## Procedure

### A. From a triaged inbox item
1. Open `docs/wiki/inbox/<item>.md` with `triage_status: classified|routed`.
2. Honor `suggested_action` / `suggested_domain` / `suggested_type`.
3. If `needs-human` → stop.
4. Execute the matching path below.
5. Set inbox `triage_status: ingested`, then move file to `docs/wiki/inbox/archive/` (create dir if needed).
6. Sync graph + log.

### B. Paths

#### `merge-existing`
1. Open target page(s) listed in triage.
2. Integrate claims; preserve dual-track Trell thesis (don't dilute).
3. Bump `updated`; add `nodes`/`edges`/`related` as needed.
4. If contradiction with old claims → add `rel: contradicts` or rewrite with note in log.

#### `new-page`
1. Confirm domain folder already exists (`core|theory|applications|market|roadmap|meta|_meta|raw`).
2. Create `docs/wiki/<domain>/<kebab-slug>.md` with **full** SCHEMA frontmatter.
3. Add INDEX one-liner if it's a lasting page.
4. Link from ≥1 existing hub page (`related` both ways when possible).

#### `raw-only`
1. Create `docs/wiki/raw/<slug>.md` (`type: raw-pointer`, `origin: ...`).
2. Optionally schedule a follow-up concept merge (leave a new inbox item if needed).

#### Feature sync (code → wiki)
Use `repobrain-maintain` matrix; still log as `maintain` or `ingest`.

### C. Always finish with
1. `python3 docs/wiki/_system/scripts/sync_graph.py`
2. Update `docs/wiki/INDEX.md` if pages were added/removed/renamed.
3. Append entry to `docs/wiki/log.md`:
```markdown
## [YYYY-MM-DD] ingest | <short title>
- Action: merge-existing | new-page | raw-pointer
- Page: <path>
- Nodes/edges: N added
```

Operator manual: `docs/wiki/_system/docs/OPERATOR.md`
Router: `docs/wiki/_system/docs/ROUTER.md`
