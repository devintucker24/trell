---
name: wiki-navigate
description: Navigate the wiki brain via INDEX, GRAPH.yaml, and frontmatter. Use when finding pages, traversing graph edges, or deciding which docs an agent should read first. This Trell repo's hubs are listed below as the instance map.
---

# Skill: Wiki Navigate

## When to use
- User asks "where is X documented?"
- Agent needs epistemic/types/applications/market/roadmap context
- Starting any Trell research session

## Procedure
1. Read `AGENTS.md` (if not already in context).
2. Read `docs/wiki/INDEX.md` — pick candidate pages by section.
3. Optionally load `docs/wiki/_meta/GRAPH.yaml` and walk:
   - Outbound edges from a node (`from == id`)
   - Inbound edges (`to == id`) for hub detection
4. Open only the pages whose `agent.read_when` matches the task.
5. Prefer `agent.priority: critical|high` pages first.

## Graph traversal tips
- Hub concepts: `belief-type`, `certain-type`, `speculative-execution`, `natural-trell-syntax`
- Application entry: `three-beat-safety-pattern`
- Market entry: `comp-langchain`, `reg-eu-ai-act`
- Future: `phase-4-iso-silicon`

## Output
Return wikilinks + one-line `summary` from frontmatter. Do not dump entire pages unless asked.

Operator manual: `docs/wiki/OPERATOR.md`  
Router: `docs/wiki/ROUTER.md`
