---
name: repobrain-query
description: Answer questions from the host RepoBrain corpus with citations. Prefer retrieve skill first; compile valuable answers into synthesis pages so explorations compound.
---

# Skill: RepoBrain Query

This is the **`/repobrain-query` skill**. There is no `./repobrain query` CLI
command. Corpus lookup is `./repobrain retrieve`. `./repobrain graph query` is
Graphify (code), not this playbook.

## When to use
- Conceptual questions about the host project
- Continuity questions ("what did we decide?") — also check episodic/temporal

## Procedure
1. Read `docs/wiki/_system/docs/ROUTER.md` Tier-0/1 seeds for the intent.
2. Run **retrieve** (preferred over hand-skimming INDEX):

```bash
./repobrain retrieve "<question>" --budget-tokens 3500
```

   - Decisions/sessions → add `--lane episodic`
   - When/as-of/changed → `--as-of YYYY-MM-DD` and/or `--lane temporal`
   - Lexer / parser / who-calls / syntax implementation → run
     `./repobrain graph query`. If `graphify-out/graph.json` is missing,
     `./repobrain graph sync` then query. Open named `source_file`s only.
3. If the header is `# miss:` or JSON `miss: true`, **stop**. Do not open
   recency hits. Rephrase from `router-seeds.md` once. If it still misses,
   say the corpus did not match.
4. Read 2–6 top pages/sections (not the whole wiki).
5. Answer with:
   - Direct verdict first
   - Citations as `[[folder/page]]`
   - Code snippets only from wiki or host examples
6. **File back** valuable answers:
   - Expand an existing page, OR
   - Create `docs/wiki/<domain>/<slug>.md` with `type: synthesis`
7. Log: `## [YYYY-MM-DD] query | <slug>`
8. Telemetry:

```bash
./repobrain usage log --op query --query "<question>" \
  --pages-opened "path/a.md,path/b.md" --cited "path/a.md" --source agent
```

## Anti-patterns
- Do not invent market share % without sources.
- Do not restate abandoned research sketches as current product truth.
- Do not skip the host `anchor` in `HOST.yaml` when explaining the project.
- Do not dump INDEX + SCHEMA + a whole domain into context.
- Do not skip `./repobrain graph query` when `graphify-out/graph.json` is missing.
- Do not treat episodes as semantic truth until consolidated.
- Do not answer from retrieve hits whose `why` is only `temporal-fit` and
  `lex` is ~0. That is recency, not a synonym match.

Operator manual: `docs/wiki/_system/docs/OPERATOR.md`
Router: `docs/wiki/_system/docs/ROUTER.md`
