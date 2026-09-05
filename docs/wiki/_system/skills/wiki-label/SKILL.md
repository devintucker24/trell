---
name: wiki-label
description: Normalize YAML frontmatter, tags, node ids, and edge relation vocabulary across the Trell wiki. Use when pages lack metadata or graph labels are inconsistent.
---

# Skill: Wiki Label

## When to use
- Pages missing frontmatter
- Inconsistent tags (`Maritime` vs `maritime`)
- Node ids not kebab-case
- Unknown `rel` values on edges

## Procedure
1. Load vocabulary from `docs/wiki/_system/docs/SCHEMA.md`.
2. For each target page:
   - Ensure required fields exist
   - Normalize `tags` to lowercase kebab/single tokens
   - Normalize `nodes[].id` to kebab-case
   - Map illegal `rel` → closest legal relation (`related_to` as last resort)
   - Set `updated` to today when changing metadata
3. Rebuild `_system/generated/claim-graph.yaml`.
4. Log: `## [YYYY-MM-DD] label | <scope>`

## Allowed domains
`core` | `theory` | `applications` | `market` | `roadmap` | `meta` | `episodic` | `temporal`

## Allowed types
`index` | `concept` | `application` | `market` | `roadmap` | `schema` | `meta` | `synthesis` | `raw-pointer` | `inbox-item` | `episode`

Operator manual: `docs/wiki/_system/docs/OPERATOR.md`
Router: `docs/wiki/_system/docs/ROUTER.md`
