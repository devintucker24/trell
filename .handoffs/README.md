# `.handoffs/`

Git-backed session-handoff notes for agents.

- `/handoff` writes `handoff-YYYYMMDD-HHMMSS.md`, commits it, and pushes the current branch
- Start the next Cloud Agent from that same branch
- `/read-handoff` loads the newest file, then commits and pushes its deletion

Handoff Markdown is intentionally tracked so a fresh Cloud Agent VM receives it
with the checkout. Deleting a handoff does not remove it from Git history:
**never put secrets, credentials, tokens, or unnecessary personal data here.**

GitHub issues/specs remain the durable source of truth. Handoffs should contain
only immediate continuation state and links to those artifacts.
