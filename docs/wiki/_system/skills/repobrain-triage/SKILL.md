---
name: repobrain-triage
description: Classify RepoBrain inbox items — decide domain, page type, merge vs new page, tags, and whether taxonomy changes need SCHEMA approval. Use before ingest whenever new material arrives.
---

# Skill: RepoBrain Triage

## When to use
- Anything landed in `docs/wiki/inbox/`
- User says "RepoBrain triage" or the historical aliases "inbox this", "triage the inbox", or pastes raw research
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
Open `docs/wiki/_system/docs/SCHEMA.md` §6–§8 and `docs/wiki/INDEX.md`.

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
        Does it fit an EXISTING folder + SCHEMA type?
          YES → new-page (suggest path: docs/wiki/<domain>/<slug>.md)
          NO  →
            Needs new folder/type/rel/domain?
              YES → needs-human (DO NOT CREATE FOLDER; propose in card)
              NO  → discard (irrelevant to Trell thesis)
```

### 5. Tag normalization
Check `_system/docs/SCHEMA.md` §8 known tags. Record recurring additions for schema review.

### 6. Log
```markdown
## [YYYY-MM-DD] triage | <item title>
- Action: merge-existing | new-page | raw-only | needs-human | discard
- Target: <path or proposal>
```

Next: if ready → run **`repobrain-ingest`**.

Operator manual: `docs/wiki/_system/docs/OPERATOR.md`
Router: `docs/wiki/_system/docs/ROUTER.md`
