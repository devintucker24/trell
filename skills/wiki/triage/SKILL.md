---
name: wiki-triage
description: Classify docs/wiki/inbox items — decide domain, page type, merge vs new page, tags, and whether taxonomy changes need SCHEMA approval. Use before ingest whenever new material arrives.
---

# Skill: Wiki Triage

## When to use
- Anything landed in `docs/wiki/inbox/`
- User says "inbox this", "triage the inbox", or pastes raw research
- Before creating folders, tags, or page types

## Golden rule
**Triage decides. Ingest writes wiki truth. Schema constrains invention.**

You do **not** create a new top-level wiki folder because content is exciting. You classify into the existing taxonomy first.

---

## Procedure

### 1. List pending items
```bash
ls docs/wiki/inbox/*.md | grep -v _TEMPLATE | grep -v README
```
Process `triage_status: pending` (and `needs-human` only if user asked).

### 2. Read schema constraints
Open `docs/wiki/SCHEMA.md` §6–§8 (taxonomy evolution) and `docs/wiki/INDEX.md`.

### 3. For each item — fill the triage card
Update the item's frontmatter:

| Field | How to choose |
|-------|----------------|
| `suggested_domain` | Which existing folder? `core` `theory` `applications` `market` `roadmap` `meta` |
| `suggested_type` | `concept` `application` `market` `roadmap` `synthesis` `raw-pointer` |
| `suggested_action` | see decision tree below |
| `priority` | impact on Trell thesis / safety / market |
| `triage_status` | `classified` → then `routed` when handoff to ingest is clear |

### 4. Decision tree (`suggested_action`)

```
Does this strengthen / update an EXISTING page?
  YES → merge-existing   (name the page paths)
  NO  →
    Is it an immutable source (PDF, URL, code, thesis)?
      YES → raw-only (+ later optional concept pages)
      NO  →
        Is it on-thesis for Trell (epistemic types, speculation, niches, market, roadmap)?
          NO  → discard   (log why)
          YES →
            Can it live under an EXISTING domain folder?
              YES → new-page
              NO  → needs-human  (proposing a NEW folder/type/tag vocabulary)
```

### 5. Taxonomy change gate (IMPORTANT)
If you want any of:
- new **top-level folder** under `docs/wiki/`
- new **`type:`** enum value
- new **`domain:`** enum value
- new **edge `rel:`** 
- a **tag** that will be reused ≥3 times and isn't in the known tag set

→ set `triage_status: needs-human` and write a short proposal in the inbox body:

```markdown
## Taxonomy proposal
- Change: add domain `hardware`
- Why existing domains fail: ...
- Migration impact: INDEX, SCHEMA, AGENTS.md, skills
- Suggested edges/tags: ...
```

**Do not invent folders or enums unilaterally.** Update `SCHEMA.md` + `AGENTS.md` first (with human OK if ambiguous), then ingest.

### 6. Tag selection
1. Prefer tags already used on pages in the same domain (scan frontmatter).
2. Use lowercase kebab / single tokens.
3. One-off descriptors can go in the body; don't pollute `tags:` with unique phrases.
4. New recurring tag → mention in triage notes; add to SCHEMA §8 known tags on ingest.

### 7. Handoff
- `merge-existing` / `new-page` / `raw-only` → run **ingest** skill next
- `discard` → archive/delete + log `## [date] triage | discard <slug>`
- `needs-human` → stop and ask

### 8. Log
```markdown
## [YYYY-MM-DD] triage | <slug>
- Action: merge-existing|new-page|raw-only|discard|needs-human
- Domain: ...
- Targets: ...
```

---

## Anti-patterns
- Creating `docs/wiki/maritime/` because a ship note arrived → **WRONG** (use `applications/`)
- Adding tag `Very Important COLREGS Thing` → **WRONG** (use `maritime`, `colregs`, `regulation`)
- Citing inbox items in answers as settled fact → **WRONG**
- Skipping triage and writing straight into `core/` from a chat paste → **WRONG** (drop to inbox first unless user explicitly says "update page X now")
