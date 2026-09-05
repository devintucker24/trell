# CLAUDE.md — Trell (Claude Code)

Project instructions for **Claude Code**. Cursor and other agents should prefer root `AGENTS.md` (same brief). Keep this file short; do not duplicate the full wiki operator manual here.

---

## Project

Trell = epistemic programming language (Rust). Core invariants:

- `certain T` vs `belief<T>` — no silent coercion
- `verify` / `require` + `guard` for epistemic reduction
- `when` / `fork` speculative execution with rollback
- Natural Trell: colon + indent + `end`

Full always-on brief: **`AGENTS.md`**. Deep RepoBrain ops:
**`docs/wiki/_system/docs/OPERATOR.md`**.

---

## Default workflow

1. Read `AGENTS.md` (and this file if you are Claude Code).
2. Code changes → `src/`, `examples/`, `tests/` → run `cargo test`.
3. Knowledge / memory → `docs/wiki/_system/docs/ROUTER.md` → retrieve:

```bash
python3 docs/wiki/_system/scripts/wiki_retrieve.py "<question>" --budget-tokens 3500
python3 docs/wiki/_system/scripts/wiki_graphify.py query "<code question>"
```

4. New material → `docs/wiki/inbox/` → triage → ingest (never invent taxonomy without `SCHEMA.md`).
5. Decisions / failures → `docs/wiki/episodic/` + append `docs/wiki/temporal/TIMELINE.md`.

---

## Skills

RepoBrain **canonical** playbooks: `docs/wiki/_system/skills/` (parent `repobrain-brain`).
Claude/Cursor `repobrain-*` folders are thin launchers. `wiki-*` launchers are deprecated aliases.

| Slash / name | Purpose |
|--------------|---------|
| `repobrain-brain` | Parent operator skill |
| `repobrain-setup` | New repo / portable install |
| `repobrain-retrieve` / `repobrain-query` | File RAG + cited answers |
| `repobrain-triage` / `repobrain-ingest` | Inbox pipeline |
| `repobrain-doctor` / `repobrain-heal` | Brain health |
| `repobrain-usage` | Telemetry / dashboard |
| `handoff` | Compact session → `.handoffs/handoff-*.md` |
| `read-handoff` | Load newest workspace handoff, then delete it |
| `grill-me` / `grilling` | Relentless interview to stress-test a plan or idea |
| `cargo-verify` | `fmt` / `clippy` / `test` |

Matt Pocock set remains under `.claude/skills/` · `.cursor/skills/` (`skills-lock.json`), not repo-root `skills/`.

---

## Agent skills

### Issue tracker

Issues live in GitHub Issues for `devintucker24/trell` (via `gh`). See `docs/agents/issue-tracker.md`.

### Triage labels

Default five-role vocabulary (`needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, `wontfix`). See `docs/agents/triage-labels.md`.

### Domain docs

Single-context: root `CONTEXT.md` + `docs/adr/`. See `docs/agents/domain.md`.

---

## Do not

- Dump entire `docs/wiki/` into context
- Cite inbox or unconsolidated episodes as product truth
- Treat `docs/research/` abandoned sketches as current Trell
- Skip `cargo test` after compiler/runtime edits
