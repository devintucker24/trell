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
type: index|concept|application|market|roadmap|schema|meta|synthesis|raw-pointer|inbox-item
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

---

## 6. Inbox → Triage → Ingest Pipeline

```
docs/wiki/inbox/   type: inbox-item   triage_status: pending
        │
        ▼  skills/wiki/triage
   classified / routed / needs-human / rejected
        │
        ▼  skills/wiki/ingest
   wiki page (domain folder)  +  optional raw/ pointer  +  GRAPH.yaml  +  log.md
        │
        ▼
   inbox item → inbox/archive/  (triage_status: ingested)
```

### Inbox-only fields
| Field | Values |
|-------|--------|
| `triage_status` | `pending` `classified` `routed` `ingested` `rejected` `needs-human` |
| `suggested_domain` | existing domain or `null` |
| `suggested_type` | existing type or `null` |
| `suggested_action` | `merge-existing` `new-page` `raw-only` `discard` `needs-human` |
| `origin` | URL, path, `user-paste`, `chat`, etc. |
| `priority` | `critical` `high` `medium` `low` |

Inbox pages are **not** query authorities until ingested.

---

## 7. Taxonomy Evolution Rules (when agents may invent structure)

### Allowed without human approval
- New **page** under an existing domain folder
- New **node id** (kebab-case, correct prefix)
- New **edge** using an existing `rel`
- Reuse of an existing **tag**
- New **raw-pointer**
- New **inbox-item**

### Requires SCHEMA.md + AGENTS.md update first (and `needs-human` if unsure)
- New **top-level folder** under `docs/wiki/` (e.g. inventing `docs/wiki/hardware/`)
- New **`domain:`** enum value
- New **`type:`** enum value
- New **`rel:`** edge relation
- New **node `kind:`**
- Renaming/splitting a domain

### Decision heuristic
1. Can this claim live on an existing page? → **merge**
2. Can this claim be a new page in an existing folder? → **new-page**
3. Is it only a source artifact? → **raw-only**
4. Otherwise → **needs-human** taxonomy proposal (do not invent folders)

---

## 8. Known Tags (reuse before inventing)

Prefer these. Add new recurring tags here when promoted from inbox.

**Core / theory:** `epistemic-types`, `belief`, `certain`, `syntax`, `natural-trell`, `speculation`, `guards`, `contracts`, `quorum`, `type-theory`, `bayesian`, `affine-types`, `zk-snark`, `hardware`, `npu`

**Applications:** `maritime`, `colregs`, `healthcare`, `finance`, `fedwire`, `grid`, `security`, `iam`, `satellites`, `pattern`, `three-beat`

**Market / roadmap:** `market`, `regulation`, `insurance`, `personas`, `adoption`, `roadmap`, `phases`, `vision`

**Meta:** `inbox`, `triage`, `ingest`, `schema`, `graph`, `index`, `raw`, `health`, `simulation`

One-off adjectives do **not** belong in `tags:` — put them in the body.