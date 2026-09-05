---
name: repobrain-doctor
description: Diagnose RepoBrain health without changing corpus files — frontmatter, graph orphans, broken links, stale inbox, schema drift. Use when the user asks for RepoBrain doctor or the historical wiki doctor alias, an audit, diagnosis, or a pre-heal check.
---

# Skill: RepoBrain Doctor (diagnose only)

## When to use
- User says **"RepoBrain doctor"** or the historical aliases **"wiki doctor"**, "audit the wiki", "what's wrong with the brain?"
- Before running **RepoBrain heal**
- After large ingest / before release

## Golden rule
**Doctor does not edit corpus pages.** It writes under `_system/generated/doctor/`.

---

## Procedure

### 1. Run the automated scanner
```bash
./repobrain doctor
```
This writes:
- `_system/generated/doctor/doctor-YYYY-MM-DD.md`
- `_system/generated/doctor/latest.json`

### 2. Manual deep checks (optional, if scanner flags issues)
| Check | How |
|-------|-----|
| Thesis dilution | Spot-check that core pages still assert dual-track non-coercion |
| Code drift | Compare Natural Trell claims to `src/parser.rs` / examples |
| Contradictions | Follow any `rel: contradicts` edges in GRAPH.yaml |
| Inbox SLA | `docs/wiki/inbox/*.md` still `triage_status: pending` too long? |

### 3. Severity rubric
| Severity | Meaning | Heal allowed? |
|----------|---------|---------------|
| `critical` | Broken schema / invalid YAML / broken graph endpoints | Yes, immediate |
| `high` | Missing frontmatter, broken wikilinks, stale inbox | Yes |
| `medium` | Hard orphan nodes, weak inbound links | Yes (edge/link adds) |
| `low` | Style, tag suggestions, optional denser linking | Optional |
| `blocker` | Would require new folder/type/rel | **No** — triage `needs-human` / SCHEMA change |

### 4. Log
```markdown
## [YYYY-MM-DD] doctor | diagnosis
- Report: `docs/wiki/_system/generated/doctor/doctor-YYYY-MM-DD.md`
- Critical: N  High: N  Medium: N  Low: N
- Next: heal | none
```

Operator manual: `docs/wiki/_system/docs/OPERATOR.md`
Router: `docs/wiki/_system/docs/ROUTER.md`
