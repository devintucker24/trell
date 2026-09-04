---
name: wiki-doctor
description: Diagnose wiki-brain health without changing files — frontmatter, graph orphans, broken links, stale inbox, schema drift. Use when user asks for wiki doctor, audit, diagnosis, or before heal.
---

# Skill: Wiki Doctor (diagnose only)

## When to use
- User says **"wiki doctor"**, "audit the wiki", "what's wrong with the brain?"
- Before running **wiki heal**
- After large ingest / before release

## Golden rule
**Doctor does not edit wiki pages.** It only inspects and writes a diagnosis report under `docs/wiki/_meta/`.

---

## Procedure

### 1. Run the automated scanner
```bash
python3 docs/wiki/scripts/wiki_doctor.py
```
This writes:
- `docs/wiki/_meta/doctor-YYYY-MM-DD.md` (human/agent readable)
- `docs/wiki/_meta/doctor-latest.json` (machine readable findings)

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
- Report: docs/wiki/_meta/doctor-YYYY-MM-DD.md
- Critical: N  High: N  Medium: N  Low: N
- Next: heal | none
```

Operator manual: `docs/wiki/OPERATOR.md`  
Router: `docs/wiki/ROUTER.md`
