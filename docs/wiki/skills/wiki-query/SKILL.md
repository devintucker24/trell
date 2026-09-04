---
name: wiki-query
description: Answer questions from the Trell wiki brain with citations. Prefer retrieve skill first; compile valuable answers into synthesis pages so explorations compound.
---

# Skill: Wiki Query

## When to use
- Conceptual questions about Trell
- "What industries use this?" / "What's the 10-year plan?" / "How does belief work?"
- Comparing Trell to LangChain/BAML/Weft
- Continuity questions ("what did we decide?") — also check episodic/temporal

## Procedure
1. Read `docs/wiki/ROUTER.md` Tier-0/1 seeds for the intent.
2. Run **retrieve** (preferred over hand-skimming INDEX):

```bash
python3 docs/wiki/scripts/wiki_retrieve.py "<question>" --budget-tokens 3500
```

   - Decisions/sessions → add `--lane episodic`
   - When/as-of/changed → `--as-of YYYY-MM-DD` and/or `--lane temporal`
3. Read 2–6 top pages/sections (not the whole wiki).
4. Answer with:
   - Direct verdict first
   - Citations as `[[folder/page]]`
   - Code snippets only from wiki or `examples/*.trell`
5. **File back** valuable answers:
   - Expand an existing page, OR
   - Create `docs/wiki/<domain>/<slug>.md` with `type: synthesis`
6. Log: `## [YYYY-MM-DD] query | <slug>`
7. Telemetry:

```bash
python3 docs/wiki/scripts/wiki_usage.py log --op query --query "<question>" \
  --pages-opened "path/a.md,path/b.md" --cited "path/a.md" --source agent
```

## Anti-patterns
- Do not invent market share % without sources.
- Do not restate abandoned research sketches as current product truth.
- Do not skip epistemic dual-track rule when explaining Trell.
- Do not dump INDEX + SCHEMA + a whole domain into context.
- Do not treat episodes as semantic truth until consolidated.

Operator manual: `docs/wiki/OPERATOR.md`  
Router: `docs/wiki/ROUTER.md`
