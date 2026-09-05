---
id: repobrain-graphify-adapter
title: RepoBrain Graphify Adapter
type: meta
status: active
created: 2026-09-05
updated: 2026-09-05
tags: [repobrain, graphify, code-graph, adapter]
domain: meta
summary: Supported Graphify dependency, ownership seam, configuration, diagnostics, and recovery.
nodes:
  - id: repobrain-graphify-adapter
    kind: concept
    label: RepoBrain Graphify adapter
edges:
  - from: repobrain-graphify-adapter
    to: wiki-graphify-bridge
    rel: implements
related:
  - "[[GRAPH]]"
  - "[[FRAMEWORK]]"
agent:
  priority: high
  read_when:
    - configuring or diagnosing the Graphify adapter
    - changing code graph synchronization
  maintain:
    - keep the supported requirement aligned with adapter tests
---

# RepoBrain Graphify Adapter

Graphify is RepoBrain's optional code-intelligence dependency. Graphify owns
AST extraction, calls/imports, community analysis, graph operations, and HTML
rendering. RepoBrain owns only configuration, invocation, normalized artifact
inspection, freshness, and actionable diagnostics.

## Supported dependency

| Property | Value |
|---|---|
| Python package | `graphifyy` |
| CLI | `graphify` |
| Supported versions | `>=0.9.54,<0.10` |
| Python | `>=3.10` |
| License | [Apache-2.0](https://github.com/Graphify-Labs/graphify) |
| Install | `python3 -m pip install --user 'graphifyy>=0.9.54,<0.10'` |

RepoBrain neither vendors Graphify source nor commits `graphify-out/`
artifacts. Setup reports the exact install command when the CLI is missing or
unsupported; it never installs hooks, watchers, background services, semantic
document extraction, or hosted features.

The adapter records local sync provenance beside the ignored graph as
`graphify-out/.repobrain-provenance.json`.

## Host configuration

```yaml
graphify:
  enabled: true
  requirement: graphifyy>=0.9.54,<0.10
  code_only: true
  out: graphify-out
  roots: [src]
  excludes:
    - "**/target/**"
    - "**/node_modules/**"
    - "**/vendor/**"
    - "**/dist/**"
    - "**/build/**"
    - "**/generated/**"
  emit_html: false
```

`targets` remains a deprecated alias for `roots`. Paths resolve from the
repository root. Graphify receives each root and exclusion; RepoBrain does not
parse or merge code graphs itself.

## Operations

```bash
./repobrain graph status
./repobrain graph status --json
./repobrain graph sync
./repobrain graph sync --force
./repobrain graph sync --html
./repobrain graph query "who calls TypeChecker"
./repobrain graph affected TypeChecker
./repobrain graph export-html
```

Status distinguishes CLI compatibility, missing or malformed artifacts,
`edges` versus NetworkX `links` shapes, committed/staged/dirty/untracked source
freshness under configured roots, confidence classes, and visualization
availability. Use `sync --force` to recover from corruption or a graph reduced
by refactoring. Graphify edges retain `EXTRACTED`,
`INFERRED`, or `AMBIGUOUS` confidence; only extracted evidence supports an
unqualified code assertion.
