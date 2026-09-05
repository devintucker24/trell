---
name: wiki-navigate
description: Navigate wiki claim pages and the Graphify code graph. Use when finding pages, traversing edges, or deciding which docs/code an agent should read first.
---

# Skill: Wiki Navigate

## When to use
- User asks "where is X documented?" vs "where is X in the code?"
- Agent needs epistemic/types/applications/market/roadmap context
- Starting any Trell research session

## Two graphs (do not mix)

| Question | Graph | Command |
|----------|-------|---------|
| What do we **claim**? | Corpus frontmatter → `_system/generated/claim-graph.yaml` | retrieve, then open pages |
| How is it **wired in code**? (calls, imports, god nodes) | Graphify `graphify-out/graph.json` | `wiki_graphify.py query/path/explain` |

Do not dump `GRAPH.yaml` or `graph.json` into context. Query them.

## Procedure
1. Read `AGENTS.md` if not already in context.
2. Read `docs/wiki/_system/docs/ROUTER.md` Tier 0–1.
3. Retrieve:

```bash
python3 docs/wiki/_system/scripts/wiki_retrieve.py "<question>" --budget-tokens 3500
```

4. For code wiring, after or instead of wiki hits:

```bash
python3 docs/wiki/_system/scripts/wiki_graphify.py query "<question>"
python3 docs/wiki/_system/scripts/wiki_graphify.py god-nodes
```

5. Open only pages whose `agent.read_when` matches, plus the `source_file` Graphify names.
6. Prefer `agent.priority: critical|high` doctrine pages. Graphify wiki export (`graphify export wiki`) is structural, not Trell thesis.

## Claim-graph hubs (this host)
- Hub concepts: `belief-type`, `certain-type`, `speculative-execution`, `natural-trell-syntax`
- Application entry: `three-beat-safety-pattern`
- Market entry: `comp-langchain`, `reg-eu-ai-act`
- Future: `phase-4-iso-silicon`

## Output
Return wikilinks + one-line `summary` from frontmatter, and/or Graphify node + `source_file`. Do not dump entire pages unless asked.

Operator manual: `docs/wiki/_system/docs/OPERATOR.md`
Router: `docs/wiki/_system/docs/ROUTER.md`
Machine graph protocol: `docs/wiki/_system/docs/GRAPH.md`
