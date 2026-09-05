# AGENTS.md — Trell project brief

Trell is an epistemic programming language and Rust compiler/runtime:

- `certain T` and `belief<T>` are distinct; never silently coerce between them.
- Reduction requires `verify` / `require` plus a deterministic `guard`.
- `when` / `fork` speculation rolls back.
- Natural Trell uses colon + indentation + explicit `end`.

`src/` is behavioral truth. `docs/research/` contains abandoned sketches unless
they have been reconciled into the reviewed corpus.

## Repository map

| Path | Owner |
|---|---|
| `src/`, `examples/`, `tests/` | compiler/runtime |
| `docs/wiki/` | host knowledge corpus |
| `docs/wiki/_system/` | portable RepoBrain engine |
| `docs/wiki/_system/config/HOST.yaml` | host overlay |
| `graphify-out/` | Graphify-generated code graph |
| `THESIS.md` | immutable language thesis |

## Workflow

For compiler work, use `src/`, examples, and tests; run `cargo test`.

For knowledge, memory, research, or RepoBrain work:

1. Read `docs/wiki/_system/docs/ROUTER.md`.
2. Retrieve instead of loading the full index:

```bash
./repobrain retrieve "<question>" --budget-tokens 3500
./repobrain graph query "<code question>"
```

3. Follow the matching canonical playbook under
   `docs/wiki/_system/skills/`.

## Hard rules

1. Belief is not certainty; preserve the explicit reduction boundary.
2. Inbox, episodes, timeline entries, raw sources, and generated pages are not
   semantic truth until reviewed and consolidated.
3. Update `docs/wiki/_system/docs/SCHEMA.md` before adding wiki taxonomy.
4. Keep wiki-derived context within the Router budget.
5. Graphify is the only code AST/call graph; never hand-edit its output.
6. Structural changes require
   `python3 docs/wiki/_system/scripts/sync_graph.py`; compiler changes also
   require Graphify sync and `cargo test`.

## Pointers

| Need | Read/run |
|---|---|
| Operator/schema | `docs/wiki/_system/docs/OPERATOR.md`, `SCHEMA.md` |
| Setup/export | `docs/wiki/_system/docs/FRAMEWORK.md`, `repobrain-setup` |
| Retrieve/query | `repobrain-retrieve`, `repobrain-query` |
| Inbox promotion | `repobrain-triage` then `repobrain-ingest` |
| Health | `repobrain-doctor` then `repobrain-heal` |
| Code ↔ knowledge sync | `repobrain-maintain` |
| Session transfer | `/handoff`, then `/read-handoff` |

Cursor Cloud: Python scripts require Python 3 and PyYAML. Verify Rust with
`cargo test`.
