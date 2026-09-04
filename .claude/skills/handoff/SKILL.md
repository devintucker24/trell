---
name: handoff
description: Compact the current conversation into a handoff document for another agent to pick up.
argument-hint: "What will the next session be used for?"
disable-model-invocation: true
---

# handoff

Follow the canonical playbook:

```text
skills/handoff/SKILL.md
```

Compact this conversation into a disposable markdown handoff in the OS temp directory (not the workspace). Reference artifacts by path; suggest skills for the next agent; redact secrets.
