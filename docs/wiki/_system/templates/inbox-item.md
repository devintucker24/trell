---
id: inbox-YYYY-MM-DD-slug
title: "INBOX: short descriptive title"
type: inbox-item
status: draft
created: YYYY-MM-DD
updated: YYYY-MM-DD
tags: [inbox]
domain: meta
summary: "Unprocessed note — not wiki truth until triaged."
triage_status: pending   # pending | classified | routed | ingested | rejected | needs-human
suggested_domain: null   # core | theory | applications | market | roadmap | meta | null
suggested_type: null     # concept | application | market | roadmap | synthesis | raw-pointer | null
suggested_action: null   # merge-existing | new-page | raw-only | discard | needs-human
origin: null             # URL, file path, chat, or "user-paste"
priority: medium         # critical | high | medium | low
nodes: []
edges: []
related: []
agent:
  priority: medium
  read_when:
    - "triaging inbox"
  maintain:
    - "clear triage_status after ingest"
---

# Inbox item

## Source
- Origin:
- Date seen:
- Why it might matter to Trell:

## Raw notes / paste
<!-- paste freely; messy is fine -->

## Claims to extract (optional; agent fills during triage)
- [ ]

## Suspected wiki targets (optional)
- Existing page to merge into:
- New page slug (only if merge impossible):
