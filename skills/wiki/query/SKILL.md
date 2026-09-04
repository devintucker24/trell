---
name: wiki-query
description: Answer questions from the Trell wiki brain with citations. Prefer compiling answers into new synthesis pages so explorations compound. Use when explaining Trell, applications, market, or roadmap.
---

# Skill: Wiki Query

## When to use
- Conceptual questions about Trell
- "What industries use this?" / "What's the 10-year plan?" / "How does belief work?"
- Comparing Trell to LangChain/BAML/Weft

## Procedure
1. Navigate via INDEX + GRAPH (see `skills/wiki/navigate`).
2. Read 2–6 relevant pages (not the whole wiki).
3. Answer with:
   - Direct verdict first
   - Citations as `[[folder/page]]`
   - Code snippets only from wiki or `examples/*.trell`
4. **File back** valuable answers:
   - Expand an existing page, OR
   - Create `docs/wiki/<domain>/<slug>.md` with `type: synthesis`
5. Log: `## [YYYY-MM-DD] query | <slug>`

## Anti-patterns
- Do not invent market share % without sources.
- Do not restate abandoned research sketches as current product truth.
- Do not skip epistemic dual-track rule when explaining Trell.
