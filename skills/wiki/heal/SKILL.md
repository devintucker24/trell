---
name: wiki-heal
description: Apply safe fixes from a wiki doctor report — frontmatter, links, orphan edges, inbox routing, GRAPH sync. Use after wiki doctor, or when user says wiki heal / fix the wiki.
---

# Skill: Wiki Heal (apply fixes)

## When to use
- User says **"wiki heal"**, "fix the wiki", "apply the doctor report"
- Immediately after **wiki doctor** produced findings
- Lint follow-up when edits are required

## Golden rule
**Heal only fixes what doctor diagnosed.** No speculative rewrites. No new taxonomy.

---

## Prerequisites
1. Fresh doctor report exists: `docs/wiki/_meta/doctor-latest.json` (run doctor if missing/stale).
2. Read `docs/wiki/SCHEMA.md` §7 — do not invent folders/types/rels during heal.

---

## Procedure

### 1. Load findings
Read `doctor-latest.json` (or today's `doctor-YYYY-MM-DD.md`).

### 2. Apply in order (safe → risky)
| Order | Finding class | Fix |
|------:|---------------|-----|
| 1 | invalid / missing frontmatter | `skills/wiki/label` + SCHEMA required fields |
| 2 | broken wikilinks | repair targets or remove dead links |
| 3 | graph edges → missing nodes | add stub node on owning page **or** remove bad edge |
| 4 | hard orphan nodes | add `related_to` / `applies_to` edges from nearest hub (prefer existing hubs) |
| 5 | stale inbox `pending` | run **triage** (don't silently ingest without classification) |
| 6 | `status: stale` pages | bump content if still true, or mark deprecated |
| 7 | code drift | update wiki to match `src/` / examples (or file inbox item if large) |

### 3. Never auto-heal (escalate)
- New top-level folder needed
- New `domain` / `type` / `rel` / node `kind`
- Thesis-changing contradictions without evidence
- Deleting large pages

For these: create/update an inbox item with `triage_status: needs-human`.

### 4. Finish
```bash
python3 skills/wiki/scripts/sync_graph.py
python3 skills/wiki/scripts/wiki_doctor.py   # re-diagnose; expect fewer findings
```

Write `docs/wiki/_meta/heal-YYYY-MM-DD.md`:
- What was fixed
- What was skipped / needs-human
- Doctor score before → after

### 5. Log
```markdown
## [YYYY-MM-DD] heal | apply doctor report
- Fixed: ...
- Skipped: ...
- Re-doctor: critical=0 high=…
```

---

## Relationship to other skills
```
doctor (read-only diagnosis)
   │
   ▼
heal ──► may call label / triage / maintain
   │
   ▼
doctor again (verify)
```

`skills/wiki/lint` is the **combined** shortcut: doctor → heal → re-doctor.
