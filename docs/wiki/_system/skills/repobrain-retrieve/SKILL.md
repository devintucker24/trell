---
name: repobrain-retrieve
description: Hybrid file-RAG retrieve over a RepoBrain corpus — lexical + graph proximity + temporal validity/rerank with MMR diversity. Use instead of dumping INDEX; prefer before query answers.
---

# Skill: RepoBrain Retrieve

## When to use
- Any question against the RepoBrain corpus (concepts, apps, market, roadmap)
- “What did we decide…?” → `--lane episodic`
- “When / as-of / what changed…?” → `--as-of YYYY-MM-DD` and/or `--lane temporal`
- Before deep-reading more than 2 pages

## Procedure
1. Read `docs/wiki/_system/docs/ROUTER.md` Tier-0/1.
2. Run:

```bash
./repobrain retrieve "<query>" --budget-tokens 3500
```

Useful flags:
- `--k 8` — candidate cap before budget trim
- `--lane semantic|episodic|temporal|meta|all`
- `--as-of 2026-09-04` — validity-window filter + temporal scoring
- `--json` — machine-readable hits
- `--code` — also run Graphify `query` against `graphify-out/graph.json` (AST/call graph)
- `--include-sources` — include bounded, non-authoritative excerpts from the
  committed Git-tracked source inventory

3. Open **only** the top 1–3 paths (or the matching `##` section).
4. Claim-graph one-hop: frontmatter `edges` / `_system/generated/claim-graph.yaml`.
5. Code/structure questions: **do not** invent a second AST graph.

```bash
./repobrain graph query "<same or narrower question>"
./repobrain graph path Parser TypeChecker
./repobrain graph explain TypeChecker
```

6. Answer with citations `[[folder/page]]`. Prefer filing durable answers back (query skill).
7. Log: `## [YYYY-MM-DD] retrieve | <slug>`

## Scoring (v0 file-native)
| Signal | Weight |
|---|---:|
| Lexical | 0.40 |
| Frontmatter / read_when / tags | 0.15 |
| Graph proximity | 0.15 |
| Temporal fit / recency | 0.15 |
| Type prior | 0.05 |
| MMR diversity | 0.10 |

No embeddings required until corpus ≫ ~200 pages.

RepoBrain retrieve ranks **compiled pages** above conflicting raw project
sources. Raw results retain repository-path attribution and can only create an
inbox triage candidate; they never rewrite a compiled claim. Graphify ranks
**code symbols**. These evidence classes are not interchangeable.
`GRAPH.yaml` is only a dump of page frontmatter for claim proximity.

## Temporal rules
- Pages with `temporal.valid_until ≤ as_of` are down-ranked (unless query is historical).
- Episodes decay faster than semantic pages.
- For change-logs, slice `docs/wiki/temporal/TIMELINE.md` then retrieve linked pages.

## Anti-patterns
- Dumping all hits into context past budget
- Skipping retrieve and loading an entire domain folder
- Treating episodic hits as semantic truth without consolidation
- Ignoring `--as-of` when the user asked a when/history question
