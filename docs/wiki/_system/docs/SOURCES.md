---
id: repobrain-source-pipeline
title: RepoBrain Source Inventory and Conversion
type: meta
status: active
created: 2026-09-05
updated: 2026-09-05
tags: [repobrain, sources, retrieval, markitdown, provenance]
domain: meta
summary: Deterministic raw-source inventory, bounded retrieval, authority rules, and local conversion.
nodes:
  - id: repobrain-source-pipeline
    kind: concept
    label: RepoBrain source pipeline
edges:
  - from: repobrain-source-pipeline
    to: wiki-retrieval
    rel: implements
related:
  - "[[OPERATOR]]"
  - "[[_system/docs/GRAPHIFY]]"
agent:
  priority: high
  read_when:
    - scanning or retrieving repository documentation
    - configuring local document conversion
  maintain:
    - keep the supported MarkItDown requirement and safety policy current
---

# RepoBrain Source Inventory and Conversion

RepoBrain inventories relevant Git-tracked files without copying them into the
compiled corpus. Raw results remain visibly non-authoritative; reviewed active
corpus claims win when raw material disagrees. A disagreement creates an
idempotent inbox candidate for human review instead of changing either source.

## Source operations

```bash
./repobrain source scan
./repobrain source status --json
./repobrain source convert
./repobrain retrieve "architecture decision" --include-sources
```

The committed manifest at
`docs/wiki/_system/generated/sources/manifest.json` records normalized paths,
classification, byte size, content hash, freshness, and conversion state.
Entries are stable-sorted. Group pages under `docs/wiki/raw/` are concise
pointers for ADRs, context maps, and documentation sites—not promoted claims
and not copies of every source file. Code is inventoried but structural lookup
continues to delegate to Graphify.

Generated conversion text lives under
`docs/wiki/_system/generated/sources/cache/` and is ignored by default.

## MarkItDown tracer

The first conversion tracer supports local UTF-8 CSV only:

| Property | Value |
|---|---|
| Python package | `markitdown` |
| Supported version | `0.1.7` |
| Python | `>=3.10` |
| License | [MIT](https://github.com/microsoft/markitdown/blob/v0.1.7/LICENSE) |
| Install | `python3 -m pip install --user 'markitdown==0.1.7'` |

CSV support is in the base package; no document-format extra is required.
RepoBrain calls `MarkItDown(enable_plugins=False).convert_local(path)` only
after resolving the path beneath a configured repository root. It never calls
the permissive URI API. Remote URLs, plugins, OCR, cloud services, model-backed
extraction, and media transcription are outside this tracer and disabled.
The base package still brings Magika and its native NumPy/ONNX Runtime
dependencies; “no extra” does not mean a pure-Python dependency tree.

Cache identity includes source SHA-256, converter version, relevant source
configuration, and format. Derived Markdown includes original-path attribution.
An unchanged source reuses its cache; a changed source gets a new identity.
Conversion failures stay visible and retryable in the manifest. Non-strict
setup continues after a failure.

Primary API and security guidance:
[MarkItDown README](https://github.com/microsoft/markitdown/blob/v0.1.7/README.md)
and
[`convert_local`](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/_markitdown.py).
