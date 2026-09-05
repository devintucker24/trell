---
id: host-router-seeds-template
title: Host router seeds (template)
type: meta
status: draft
created: 2026-09-04
updated: 2026-09-04
tags: [router, host, pack]
domain: meta
summary: "Replace this table with project-specific intent → seed pages after installing the wiki-brain pack."
nodes:
  - id: host-router-seeds
    kind: concept
    label: Host router seeds
edges:
  - from: host-router-seeds
    to: wiki-router
    rel: depends_on
related:
  - "[[_system/docs/ROUTER]]"
agent:
  priority: high
  read_when:
    - "after ROUTER.md Tier 0, before retrieve"
  maintain:
    - "keep intent keywords aligned with actual page paths"
---

# Host router seeds (template)

Copy to `docs/wiki/_system/config/router-seeds.md` and replace the table.

| Intent signal | Seed pages (open these first) |
|---|---|
| _(your domain keywords)_ | `path/to/page.md` |
