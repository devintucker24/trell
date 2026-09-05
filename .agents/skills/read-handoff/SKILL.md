---
name: read-handoff
description: Load the newest workspace handoff into this session, then delete it. Use when starting a fresh chat after /handoff.
disable-model-invocation: true
---

Load the latest Git-backed handoff from this repository and continue from it.

## Procedure

1. Confirm the checkout branch contains the expected handoff. A fresh Cloud Agent must have been started from the branch named by `/handoff`.
2. Look in `.handoffs/` for tracked files matching `handoff-*.md`.
3. If none exist, tell the user there is no handoff waiting on the current branch and stop.
4. Pick the **newest** file by filename timestamp.
5. Read the entire file into context.
6. Delete **only that file** after you have read it.
7. Stage only that deletion, commit with `chore(handoff): consume agent handoff`, and push the current branch. If this fails, report that the remote handoff remains available.
8. Confirm which path and branch you loaded and removed.
9. Follow the handoff: purpose, open work, artifacts, and suggested skills. Invoke suggested skills as needed.

Do not delete other files in `.handoffs/`. Do not recreate the handoff unless the user asks to `/handoff` again. Never assume deleting the working-tree file erases it from Git history.
