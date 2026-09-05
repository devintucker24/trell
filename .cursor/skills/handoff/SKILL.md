---
name: handoff
description: Compact the current conversation into a handoff document for another agent to pick up.
argument-hint: "What will the next session be used for?"
disable-model-invocation: true
---

Write a handoff document summarising the current conversation so a fresh agent can continue the work.

Save it under `.handoffs/` in the **current repository** (create the directory if needed). Do **not** use the OS temporary directory or an ignored file: fresh Cloud Agent VMs receive Git-tracked files only.

Use this filename pattern (UTC):

```text
.handoffs/handoff-YYYYMMDD-HHMMSS.md
```

After writing:

1. Confirm the handoff contains no secrets, credentials, tokens, or unnecessary personal data. Git history retains deleted files.
2. Check the current Git branch. If it is the default branch, create a dedicated handoff/feature branch following the repository's branch policy.
3. Stage **only** the handoff and any intentional handoff-protocol changes.
4. Commit with `chore(handoff): prepare next agent session`.
5. Push the current branch.
6. Tell the user the file path **and exact branch name**. The next Cloud Agent must start from that branch, then run `/read-handoff`.

If commit or push fails, report that the handoff is local-only; never claim a new Cloud Agent can see it.

Include a "suggested skills" section in the document, naming which skills the next agent should call the Skill tool for.

Do not duplicate content already captured in other artifacts (specs, plans, ADRs, issues, commits, diffs). Reference them by path or URL instead.

Redact any sensitive information, such as API keys, passwords, or personally identifiable information.

If the user passed arguments, treat them as a description of what the next session will focus on and tailor the doc accordingly.
