---
name: read-handoff
description: Load the newest workspace handoff into this session, then delete it. Use when starting a fresh chat after /handoff.
disable-model-invocation: true
---

Load the latest handoff from this workspace and continue from it.

## Procedure

1. Look in `.handoffs/` for files matching `handoff-*.md`.
2. If none exist, tell the user there is no handoff waiting and stop.
3. Pick the **newest** file (by filename timestamp, then mtime).
4. Read the entire file into context.
5. Delete **only that file** after you have read it.
6. Confirm to the user which path you loaded and deleted.
7. Follow the handoff: purpose, open work, artifacts, and suggested skills. Invoke suggested skills as needed.

Do not delete other files in `.handoffs/`. Do not recreate the handoff unless the user asks to `/handoff` again.
