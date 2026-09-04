---
id: wiki-schema
title: Trell Wiki Frontmatter & Graph Schema
type: schema
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [schema, frontmatter, graph, agents]
domain: meta
summary: "Canonical YAML contracts for page frontmatter, node kinds, and edge relations."
nodes:
  - id: wiki-schema
    kind: concept
edges:
  - from: wiki-schema
    to: agents-md
    rel: depends_on
related:
  - "[[INDEX]]"
agent:
  priority: critical
  read_when:
    - "creating or editing any wiki page"
    - "validating frontmatter"
  maintain:
    - "Keep vocabulary in sync with AGENTS.md §5"
---

# SCHEMA.md — Frontmatter & Graph Contracts

This document is the **machine-oriented contract** for every page under `docs/wiki/`. Human narrative lives in page bodies; agents enforce this schema.

---

## 1. Required Frontmatter Fields

```yaml
---
id: string                 # globally unique kebab-case slug
title: string              # human title
type: index|concept|application|market|roadmap|schema|meta|synthesis|raw-pointer
status: draft|active|stale|deprecated
created: YYYY-MM-DD
updated: YYYY-MM-DD
tags: [string, ...]        # lowercase kebab or single words
domain: core|theory|applications|market|roadmap|meta
summary: string            # <= 160 chars, one sentence
nodes:                     # graph nodes this page owns or defines
  - id: string
    kind: concept|type|primitive|engine|application|competitor|regulation|persona|phase|technology|example
    label: string          # optional display label
edges:                     # typed relations (may reference nodes on other pages)
  - from: string
    to: string
    rel: depends_on|implements|contradicts|extends|applies_to|competes_with|enforces|reduces_via|accelerates|regulated_by|owned_by|milestone_of|related_to
    note: string           # optional
related:                   # wikilink strings
  - "[[path/page]]"
implements_code:           # optional binding to repo paths
  - src/typecheck.rs
agent:
  priority: critical|high|medium|low
  read_when: [string, ...]
  maintain: [string, ...]
---
```

### Validation rules
1. `id` must match the filename stem when practical (e.g. `epistemic-foundations.md` → `epistemic-foundations` or prefixed `core-epistemic-foundations`).
2. Every `edges[].from` and `edges[].to` must exist as some page's `nodes[].id` or in `_meta/GRAPH.yaml` after sync.
3. `rel` must be from the allowed vocabulary above.
4. `updated` must bump on every substantive edit.
5. `summary` must not be empty.

---

## 2. Node ID Conventions

| Prefix (optional) | Example | Use |
|-------------------|---------|-----|
| none / concept | `belief-type` | Core concepts |
| `app-` | `app-maritime-colregs` | Application niches |
| `comp-` | `comp-langchain` | Competitors |
| `reg-` | `reg-eu-ai-act` | Regulations |
| `phase-` | `phase-1-beachhead` | Roadmap phases |
| `tech-` | `tech-xgrammar` | External technologies |

---

## 3. Edge Semantics Cheatsheet

| `rel` | Direction meaning |
|-------|-------------------|
| `depends_on` | from needs to |
| `implements` | from implements to (often code → concept) |
| `reduces_via` | belief reduces to certain via mechanism |
| `enforces` | guard/contract enforces invariant |
| `extends` | future form extends present |
| `applies_to` | concept applies to niche |
| `competes_with` | market alternatives |
| `accelerates` | silicon/tech accelerates feature |
| `regulated_by` | niche regulated by statute |
| `owned_by` | concept owned by persona |
| `milestone_of` | phase milestone of vision |
| `contradicts` | lint flag — claims conflict |
| `related_to` | weak association (prefer stronger rel) |

---

## 4. GRAPH.yaml Sync

Canonical aggregated graph: `docs/wiki/_meta/GRAPH.yaml`

Shape:
```yaml
version: 1
updated: YYYY-MM-DD
nodes:
  - id: belief-type
    kind: type
    page: core/epistemic-foundations
    label: belief<T>
edges:
  - from: belief-type
    to: certain-type
    rel: reduces_via
    via: guard-verify
    page: core/epistemic-foundations
```

After editing any page frontmatter nodes/edges, regenerate or surgically update GRAPH.yaml (skill: `skills/wiki/maintain`).

---

## 5. Dataview / Agent Query Hints

Agents can filter pages by:
- `type: application` + `tags: contains maritime`
- `agent.priority: critical`
- `status: stale` for lint targets
- nodes with `kind: competitor` for market maps
