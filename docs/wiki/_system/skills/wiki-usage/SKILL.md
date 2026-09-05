---
name: wiki-usage
description: Log and score wiki-brain usage — JSONL events, retrieve token estimates, hit quality, doctor score, dump admissions, usefulness index. Use after retrieve/doctor, when tuning budgets, or when the user asks for a usage dashboard.
---

# Skill: Wiki Usage (telemetry)

## When to use
- After a retrieve/query session (scripts already append events)
- User asks “is the wiki earning its context cost?”, “usage dashboard”, “token spend”
- Tuning `--budget-tokens` or ROUTER seeds
- Agent loaded INDEX or a whole folder (log a **dump** admission)

## What is tracked
Catalog: `docs/wiki/_system/docs/usage-telemetry.md`
Dashboard: `docs/wiki/_system/generated/usage/dashboard.md`
Raw events: `docs/wiki/_system/generated/usage/events.jsonl` (**gitignored**)

## Procedure

### Automatic
`wiki_retrieve.py` and `wiki_doctor.py` append JSONL unless `--no-log`.

### Manual / agent protocol
```bash
# Query turn: pages you opened vs pages you cited
python3 docs/wiki/_system/scripts/wiki_usage.py log --op query \
  --query "<question>" \
  --pages-opened "core/foo.md,applications/bar.md" \
  --cited "core/foo.md" \
  --tokens-est 1800

# Anti-pattern admission (loaded INDEX or a whole folder)
python3 docs/wiki/_system/scripts/wiki_usage.py log --op dump --query "<why>"

# Snapshot
python3 docs/wiki/_system/scripts/wiki_usage.py report --days 30
```

### Scoring
`wiki_usage.py report` writes the dashboard and prints JSON including **usefulness index** (0–100): hit quality + last doctor score + activity − dump penalty.

### Log
```markdown
## [YYYY-MM-DD] usage | dashboard
- Report: `docs/wiki/_system/generated/usage/dashboard.md`
- Usefulness index: N
```

## Anti-patterns
- Committing `events.jsonl`
- Treating usefulness index as a scientific KPI (it is a heuristic)
- Skipping retrieve and not logging `dump` when you stuffed INDEX into context
