---
name: handoff
description: Compact the current conversation into a handoff document for another agent to pick up.
argument-hint: "What will the next session be used for?"
disable-model-invocation: true
---

# Skill: handoff

Write a handoff document summarising the current conversation so a fresh agent can continue the work. Save to the temporary directory of the user's OS — **not** the current workspace.

## Procedure

1. If the user passed arguments, treat them as a description of what the next session will focus on and tailor the doc accordingly.
2. Create a disposable path with the OS temp helper, then write the handoff there:

```bash
mktemp -t handoff-XXXXXX.md
```

On Linux, if `-t` naming differs, use:

```bash
mktemp /tmp/handoff-XXXXXX.md
```

3. Write the handoff markdown to that path (read the path before writing).
4. Tell the user the absolute path and a one-line paste for the next session, e.g.:

> Start from this handoff: `/tmp/handoff-….md`

## Handoff document shape

Include these sections (keep the whole doc short — usually under ~1–2k tokens):

1. **Purpose** — what the next session should accomplish (from user args when present)
2. **Done** — what landed in this session (bullets)
3. **Open** — blockers, TODOs, unresolved decisions
4. **Artifacts** — paths/URLs only (PRs, commits, plans, wiki pages, diffs) — no paste dumps
5. **Suggested skills** — which project skills the next agent should invoke first
6. **Bootstrap** — for this repo, remind the next agent to skim `AGENTS.md` (Claude Code: also `CLAUDE.md`) and, for wiki/memory work, `docs/wiki/ROUTER.md`

## Hard rules

- **Reference, don't duplicate.** Do not copy content already captured in specs, plans, ADRs, issues, commits, diffs, or wiki pages — reference them by path or URL instead.
- **Redact** API keys, passwords, tokens, and personally identifiable information.
- **Do not** commit the handoff file into the repo. It is disposable session glue.
- **Do not** dump the full chat transcript into the handoff.

## Durable memory (optional, Trell-only)

Handoff ≠ wiki episodic memory. The temp file is for the *next agent context*.

Only if the user asks to persist decisions in the brain (or the session clearly produced durable project decisions):

1. Write/update a dated episode under `docs/wiki/episodic/` from `_TEMPLATE.md`, **or** refresh `docs/wiki/episodic/session-current.md` (keep under ~800 tokens).
2. Append one line to `docs/wiki/temporal/TIMELINE.md` when knowledge-changing work landed.
3. Still produce the temp handoff for the fresh chat.

## Suggested skills to name (when relevant)

Prefer real project skill names:

- `handoff` (this skill — only if chaining another branch)
- `wiki-retrieve` / `wiki-query`
- `wiki-triage` / `wiki-ingest`
- `wiki-doctor` / `wiki-heal` / `wiki-maintain`
- `cargo-verify`
