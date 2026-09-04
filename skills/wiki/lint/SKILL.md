---
name: wiki-lint
description: Health-check and heal the Trell wiki — orphans, broken links, missing frontmatter, stale pages, graph inconsistencies, contradictions. Use periodically or before releases.
---

# Skill: Wiki Lint / Heal

## When to use
- User asks to lint, heal, or audit the knowledge base
- Before merging large doc PRs
- After bulk ingest

## Checklist
1. **Frontmatter:** Every `.md` under `docs/wiki/` (except maybe log) has valid YAML per SCHEMA.md.
2. **Orphans:** Pages with zero inbound `related` / edges pointing to them (except INDEX/SCHEMA).
3. **Broken wikilinks:** `[[...]]` targets resolve to files.
4. **Graph integrity:** All edge endpoints exist in GRAPH.yaml nodes.
5. **Stale:** `status: active` but `updated` older than major code changes affecting claims.
6. **Contradictions:** Follow `rel: contradicts` edges; resolve or document.
7. **Missing concepts:** Body mentions a major Trell primitive lacking a page.
8. **Code drift:** Claims about syntax must match `src/parser.rs` / examples.

## Output
Write `docs/wiki/_meta/health-YYYY-MM-DD.md` with findings + fixes applied.  
Append log: `## [YYYY-MM-DD] lint | health pass`

## Heal order
1. Fix SCHEMA violations (label skill)
2. Repair broken links
3. Add missing edges for orphans
4. Update stale summaries
5. Sync GRAPH.yaml
