# Frontmatter Schemas (Quick Copy)

See also: `docs/wiki/SCHEMA.md` (canonical).

## concept

```yaml
---
id: example-concept
title: Example Concept
type: concept
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [example]
domain: core
summary: "One sentence summary."
nodes:
  - id: example-concept
    kind: concept
    label: Example Concept
edges:
  - from: example-concept
    to: belief-type
    rel: depends_on
related:
  - "[[core/epistemic-foundations]]"
implements_code: []
agent:
  priority: medium
  read_when:
    - "when researching example concept"
  maintain:
    - "keep aligned with implementation"
---
```

## application

```yaml
---
id: app-example
title: Example Application Domain
type: application
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [application, vertical]
domain: applications
summary: "Domain niches where Trell epistemic guards matter."
nodes:
  - id: app-example-niche
    kind: application
edges:
  - from: three-beat-safety-pattern
    to: app-example-niche
    rel: applies_to
related:
  - "[[applications/overview-and-safety-patterns]]"
agent:
  priority: high
  read_when:
    - "mapping industry use cases"
  maintain: []
---
```

## raw-pointer

```yaml
---
id: raw-thesis
title: Raw pointer — THESIS.md
type: raw-pointer
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [raw]
domain: meta
summary: "Immutable pointer to the language thesis."
origin: THESIS.md
nodes: []
edges: []
related:
  - "[[core/epistemic-foundations]]"
agent:
  priority: low
  read_when:
    - "need original thesis text"
  maintain:
    - "do not rewrite origin as wiki"
---
```
