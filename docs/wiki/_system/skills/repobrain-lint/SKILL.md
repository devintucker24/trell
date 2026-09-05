---
name: repobrain-lint
description: Combined RepoBrain doctor + heal shortcut — diagnose, apply safe fixes, then re-diagnose. Use when the user says lint RepoBrain or the historical lint the wiki alias; prefer explicit doctor/heal skills when separating diagnosis from edits.
---

# Skill: RepoBrain Lint (doctor → heal → re-doctor)

## When to use
- User says "lint RepoBrain" or the historical alias "lint the wiki" (combined pass)
- Pre-release cleanup when they want diagnose+fix in one go

## Prefer explicit skills when
- **"wiki doctor"** → `repobrain-doctor` only (no edits)
- **"wiki heal"** → `repobrain-heal` only (apply last diagnosis)

## Procedure
1. Run **doctor** (`repobrain-doctor` + `./repobrain doctor`)
2. If `heal_recommended` → run **heal** (`repobrain-heal`)
3. Run **doctor** again; compare scores
4. Log as:
```markdown
## [YYYY-MM-DD] lint | doctor+heal
- Before score: …
- After score: …
- Reports: doctor-…md, heal-…md
```

Operator manual: `docs/wiki/_system/docs/OPERATOR.md`
Router: `docs/wiki/_system/docs/ROUTER.md`
