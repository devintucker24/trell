---
id: inbox-readme
title: Wiki Inbox — Drop Zone for Unprocessed Knowledge
type: meta
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [inbox, triage, ingest]
domain: meta
summary: "Human/agent drop zone. Nothing here is wiki truth until triaged and ingested."
nodes:
  - id: wiki-inbox
    kind: concept
edges:
  - from: wiki-inbox
    to: wiki-schema
    rel: depends_on
related:
  - "[[SCHEMA]]"
  - "[[INDEX]]"
agent:
  priority: critical
  read_when:
    - "adding new material to the wiki"
    - "user pastes research, links, notes, or files"
  maintain:
    - "keep inbox empty of stale items older than triage SLA"
---

# Inbox

This folder is the **only approved on-ramp** for messy new material.

```
YOU / OTHER AGENTS          AGENT (triage skill)           AGENT (ingest skill)
     │                              │                              │
     │  drop note / URL / paste     │                              │
     ▼                              ▼                              ▼
 docs/wiki/inbox/*.md  ──►  classify + route  ──►  wiki pages + raw/ + GRAPH
                              (or reject)
```

## Rules
1. **Drop first, organize later.** Do not invent a new wiki folder because a note feels important.
2. Every inbox item uses `_TEMPLATE.md` frontmatter (`type: inbox-item`, `triage_status: pending`).
3. Inbox items are **not** citable wiki truth. Agents answering queries must not treat pending inbox as settled knowledge.
4. After successful ingest, move the item to `docs/wiki/inbox/archive/` or delete it and log the ingest.
5. If triage is ambiguous → set `triage_status: needs-human` and stop.

## How to drop something (humans or agents)

```bash
# Copy the template
cp docs/wiki/inbox/_TEMPLATE.md docs/wiki/inbox/2026-09-04-my-topic.md
# Fill body with paste / URL / notes
# Then tell the agent: "Triage the inbox" or "Ingest inbox item …"
```

Or say in chat:
> Inbox this: \<paste article / claim / link / meeting note\>

The agent creates `docs/wiki/inbox/YYYY-MM-DD-<slug>.md`, then runs **triage** → **ingest**.

## Skills
- Triage: `docs/wiki/skills/wiki-triage/SKILL.md`
- Ingest: `docs/wiki/skills/wiki-ingest/SKILL.md`
- Schema / taxonomy rules: `docs/wiki/SCHEMA.md` §6–§8
