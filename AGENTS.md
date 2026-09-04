# AGENTS.md — Trell Project Brief

> Always-on instructions for **Cursor, Claude Code, Codex, and other coding agents**.  
> Keep this file short. Deep wiki procedures live in `docs/wiki/OPERATOR.md` and `docs/wiki/skills/`.

---

## What this repo is

**Trell** is an epistemic programming language (Rust compiler/runtime):

- Dual-track types: `certain T` vs `belief<T>`
- Reduction only via `verify` / `require` + `guard`
- Speculative execution: `when` / `fork` with rollback
- Model contracts + quorums
- Natural Trell: colon + indent + `end`

Do **not** dilute this thesis. Abandoned LangChain-workflow sketches in `docs/research/` are not product truth unless reconciled into the wiki.

---

## Layout

| Path | Role |
|------|------|
| `src/` | Compiler / runtime (ground truth for behavior) |
| `examples/*.trell` | Executable language examples |
| `tests/` | Rust tests |
| `docs/wiki/` | Compounding knowledge brain (semantic + episodic + temporal) |
| `docs/wiki/skills/` | Canonical wiki-brain playbooks (`wiki-brain`, retrieve, doctor, …) |
| `docs/wiki/HOST.yaml` | Host overlay (project name, domains) — not copied as Trell corpus |
| `docs/wiki/scripts/` | Wiki automation (`wiki_retrieve.py`, `wiki_doctor.py`, `wiki_setup.py`, `wiki_graphify.py`, `wiki_pack.py`, `sync_graph.py`) |
| `.cursor/rules/` | Cursor rules (file-scoped conventions and triggers) |
| `.cursor/skills/` | Thin launchers + other Cursor skills (Matt Pocock, cargo-verify) |
| `.claude/skills/` | Thin launchers + other Claude Code skills |
| `THESIS.md` | Immutable language thesis (raw layer) |

---

## Every session (progressive disclosure)

1. Skim this file.
2. For **wiki / memory / research** tasks: read `docs/wiki/ROUTER.md`, then retrieve — **do not dump** `INDEX.md` or the whole wiki into context.
3. For **code** tasks: prefer `src/` + `examples/` + `tests/`; update wiki when epistemic semantics change.
4. Load a skill only when needed (see below).

```bash
# File RAG over the wiki brain
python3 docs/wiki/scripts/wiki_retrieve.py "<question>" --budget-tokens 3500
# Code/structure graph (Graphify — do not dump graph.json)
python3 docs/wiki/scripts/wiki_graphify.py query "<question>"
```

---

## Skills (how to invoke)

**Canonical wiki-brain:** `docs/wiki/skills/*/SKILL.md` (parent: `wiki-brain`).  
**IDE adapters:** `.cursor/skills/wiki-*` · `.claude/skills/wiki-*` (thin launchers). Export: `docs/wiki/FRAMEWORK.md`.

| Need | Skill |
|------|--------|
| Wiki-brain parent | `wiki-brain` |
| Answer from wiki | `wiki-retrieve` / `wiki-query` |
| Add notes / research | inbox → `wiki-triage` → `wiki-ingest` |
| Health | `wiki-doctor` → `wiki-heal` (or shortcut `wiki-lint`) |
| Usage / tokens | `wiki-usage` → `python3 docs/wiki/scripts/wiki_usage.py report` |
| Code ↔ wiki sync | `wiki-maintain` |
| New repo / empty host | `wiki-setup` |
| Code wiring (AST/calls) | `wiki_graphify.py query` (Graphify) |
| Rust verify | `cargo-verify` |
| Session handoff | `/handoff` → `.handoffs/` → next chat `/read-handoff` |
| Stress-test a plan | `/grill-me` (uses `/grilling`) |

Human phrases that should trigger agents: *“Inbox this…”*, *“Retrieve…”*, *“Wiki doctor”*, *“Usage dashboard”*, *“Write an episode”*, *“Set up the wiki”* / *“wiki-setup”*, *“Handoff…”* / `/handoff`, *“Read handoff”* / `/read-handoff`, *“Grill me…”* / `/grill-me`.

**Matt Pocock skills:** installed project-wide via `npx skills add mattpocock/skills` (see `skills-lock.json`). Those stay under `.cursor/skills/` · `.claude/skills/` — **not** a repo-root `skills/` tree and **not** mixed into `docs/wiki/skills/`.

---

## Hard rules

1. **Belief ≠ certain** — never coerce without guard/verify in language design or docs.
2. **Inbox is not truth** until triage + ingest.
3. **Episodes / timeline ≠ semantic truth** until consolidated into domain pages.
4. **No new wiki taxonomy** (folders / types / domains / `rel`s) without updating `docs/wiki/SCHEMA.md` first.
5. **Context budget** — follow `docs/wiki/ROUTER.md`; wiki-derived context ≤ ~9.5k tokens.
6. Before finishing Rust changes: `cargo test` (and prefer `/cargo-verify`).

---

## Where to go deeper

| Topic | Read |
|-------|------|
| Wiki operator manual | `docs/wiki/OPERATOR.md` |
| Frontmatter / graph schema | `docs/wiki/SCHEMA.md` · `docs/wiki/_meta/GRAPH.md` |
| Portable pack / setup | `docs/wiki/FRAMEWORK.md` · `docs/wiki/skills/wiki-setup/SKILL.md` |
| Context / memory protocol | `docs/wiki/_meta/CONTEXT_PROTOCOL.md` |
| Claude Code twin brief | `CLAUDE.md` |

---

## Cursor Cloud notes

- Verify with `cargo test`.
- Wiki scripts need Python 3 + PyYAML (`python3 -c "import yaml"`).
- Prefer retrieve over loading large markdown trees in cloud agent context.
