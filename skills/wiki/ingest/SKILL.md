---
name: wiki-ingest
description: Ingest new sources into the Trell wiki brain — extract claims, update concept/application pages, sync graph nodes/edges, update INDEX and append log. Use when adding research, papers, competitor notes, or new examples.
---

# Skill: Wiki Ingest

## When to use
- New research, competitor doc, regulation, or example lands
- User says "add this to the knowledge base"
- After shipping a major language feature that needs docs

## Procedure
1. **Identify raw source.** Prefer immutable pointer under `docs/wiki/raw/`:
   - Create `docs/wiki/raw/<slug>.md` with frontmatter `type: raw-pointer` and `origin:` path/URL.
   - Do not silently rewrite `THESIS.md` / `src/` as wiki pages; cite them.
2. **Extract.** Key claims, entities, concepts, contradictions with existing wiki.
3. **Integrate.** Create or update pages under the correct `domain` folder.
4. **Frontmatter.** Fill full SCHEMA.md contract including `nodes` and `edges`.
5. **Graph sync.** Update `docs/wiki/_meta/GRAPH.yaml`.
6. **Index.** Add/adjust one-line entry in `docs/wiki/INDEX.md` if structure changed.
7. **Log.** Append to `docs/wiki/log.md`:
   ```markdown
   ## [YYYY-MM-DD] ingest | Short Title
   - Source: ...
   - Pages touched: ...
   - Nodes added: ...
   ```

## Quality bar
- Cross-link related pages.
- Flag `contradicts` edges explicitly when new data conflicts.
- Keep Natural Trell examples compiling with current parser when showing code.
