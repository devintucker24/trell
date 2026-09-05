---
name: wiki-lint
description: Combined wiki doctor + heal shortcut — diagnose then apply safe fixes, then re-diagnose. Use when user says lint the wiki; prefer explicit doctor/heal skills when separating diagnosis from edits.
---

# Skill: Wiki Lint (doctor → heal → re-doctor)

## When to use
- User says "lint the wiki" (combined pass)
- Pre-release cleanup when they want diagnose+fix in one go

## Prefer explicit skills when
- **"wiki doctor"** → `wiki-doctor` only (no edits)
- **"wiki heal"** → `wiki-heal` only (apply last diagnosis)

## Procedure
1. Run **doctor** (`wiki-doctor` + `_system/scripts/wiki_doctor.py`)
2. If `heal_recommended` → run **heal** (`wiki-heal`)
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
