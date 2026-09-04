---
name: handoff
description: Compact the current conversation into a handoff document for another agent to pick up.
argument-hint: "What will the next session be used for?"
disable-model-invocation: true
---

Write a handoff document summarising the current conversation so a fresh agent can continue the work.

Save it under `.handoffs/` in the **current workspace** (create the directory if needed). Do **not** use the OS temporary directory — cloud agents and fresh chats do not reliably share `/tmp`.

Use this filename pattern (UTC):

```text
.handoffs/handoff-YYYYMMDD-HHMMSS.md
```

Tell the user the path. For the next session, they should open a fresh chat and run `/read-handoff` (which loads the newest handoff and deletes it).

Include a "suggested skills" section in the document, naming which skills the next agent should call the Skill tool for.

Do not duplicate content already captured in other artifacts (specs, plans, ADRs, issues, commits, diffs). Reference them by path or URL instead.

Redact any sensitive information, such as API keys, passwords, or personally identifiable information.

If the user passed arguments, treat them as a description of what the next session will focus on and tailor the doc accordingly.
