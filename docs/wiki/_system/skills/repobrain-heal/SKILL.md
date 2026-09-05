---
name: repobrain-heal
description: Apply safe fixes from a RepoBrain doctor report — frontmatter, links, orphan edges, inbox routing, GRAPH sync. Use after RepoBrain doctor or when the user invokes the historical wiki heal / fix the wiki phrases.
---

# Skill: RepoBrain Heal (apply fixes)

## When to use
- User says **"RepoBrain heal"** or the historical aliases **"wiki heal"**, "fix the wiki", "apply the doctor report"
- Immediately after **RepoBrain doctor** produced findings
- Lint follow-up when edits are required

## Golden rule
**Heal only fixes what doctor diagnosed.** No speculative rewrites. No new taxonomy.

---

## Prerequisites
1. Fresh report exists at `_system/generated/doctor/latest.json`.
2. Read `_system/docs/SCHEMA.md` §7.

---

## Procedure

### 1. Load findings
Read `doctor-latest.json` (or today's `doctor-YYYY-MM-DD.md`).

### 2. Apply in order (safe → risky)
| Order | Finding class | Fix |
|------:|---------------|-----|
| 1 | invalid / missing frontmatter | `_system/skills/repobrain-label` + SCHEMA required fields |
| 2 | broken wikilinks | repair targets or remove dead links |
| 3 | graph edges → missing nodes | add stub node on owning page **or** remove bad edge |
| 4 | hard orphan nodes | add `related_to` / `applies_to` edges from nearest hub (prefer existing hubs) |
| 5 | stale inbox `pending` | run **triage** (don't silently ingest without classification) |
| 6 | `status: stale` pages | bump content if still true, or mark deprecated |
| 7 | code drift | update wiki to match `src/` / examples (or file inbox item if large) |
| 8 | `graphify_graph_missing` | `./repobrain graph sync` |

### 3. Never auto-heal (escalate)
- New top-level folder needed
- New `domain` / `type` / `rel` / node `kind`
- Thesis-changing contradictions without evidence
- Deleting large pages

For these: create/update an inbox item with `triage_status: needs-human`.

### 4. Finish
```bash
python3 docs/wiki/_system/scripts/sync_graph.py
./repobrain doctor
```

### 5. Log
```markdown
## [YYYY-MM-DD] heal | <summary of fixes>
- Fixed: N items
- Re-doctor score: …/100
- Report: `docs/wiki/_system/generated/doctor/heal-YYYY-MM-DD.md`
```

`repobrain-lint` is the **combined** shortcut: doctor → heal → re-doctor.

Operator manual: `docs/wiki/_system/docs/OPERATOR.md`
Router: `docs/wiki/_system/docs/ROUTER.md`
