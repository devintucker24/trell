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
2. Create a disposable path, then write the handoff there:

```bash
mktemp /tmp/handoff-XXXXXX.md
```

3. Tell the user the absolute path and a one-line paste for the next session, e.g.:

> Start from this handoff: `/tmp/handoff-….md`

## Handoff document shape

Keep the whole doc short (usually under ~1–2k tokens):

1. **Purpose** — what the next session should accomplish
2. **Done** — what landed in this session
3. **Open** — blockers, TODOs, unresolved decisions
4. **Artifacts** — paths/URLs only (specs, plans, PRs, commits, diffs)
5. **Suggested skills** — which skills the next agent should invoke first

## Hard rules

- **Reference, don't duplicate.** Do not copy content already captured in specs, plans, ADRs, issues, commits, or diffs — reference them by path or URL instead.
- **Redact** API keys, passwords, tokens, and personally identifiable information.
- **Do not** commit the handoff file into the repo. It is disposable session glue.
- **Do not** dump the full chat transcript into the handoff.
