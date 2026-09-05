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
  - "[[_system/docs/CHEATSHEET]]"
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
./repobrain source convert --strict
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
`sources.conversion.commit_groups` may copy selected classifications into
`docs/wiki/_system/generated/sources/committed/` without changing the global
gitignore default.

## MarkItDown local formats

RepoBrain converts only configured **local** files via
`MarkItDown(enable_plugins=False).convert_local(...)`. It never calls the
permissive URI `convert()` API.

| Property | Value |
|---|---|
| Python package | `markitdown` |
| Supported version | `0.1.7` |
| Python | `>=3.10` |
| License | [MIT](https://github.com/microsoft/markitdown/blob/v0.1.7/LICENSE) |
| Default install | `python3 -m pip install --user 'markitdown==0.1.7'` |

Configure an allowlist in `HOST.yaml` `sources.conversion.formats`. Safe local
formats:

| Format | Extra | Notes |
|---|---|---|
| `csv` | base package | UTF-8 required |
| `html` | base package | UTF-8 required |
| `epub` | base package | local file only |
| `pdf` | `markitdown[pdf]` | install only if enabled |
| `docx` | `markitdown[docx]` | install only if enabled |
| `pptx` | `markitdown[pptx]` | install only if enabled |
| `xlsx` | `markitdown[xlsx]` | install only if enabled |

Never install `markitdown[all]` by default. Example for PDF + Word:

```bash
python3 -m pip install --user 'markitdown[pdf,docx]==0.1.7'
```

Remote URLs, plugins, OCR, cloud document intelligence, model-backed
extraction, and media transcription stay off unless the host sets
`allow_external: true` **and** the matching `allow_*` flag. Even then RepoBrain
still converts only local paths.

Unsupported or oversized binaries remain inventoried (`unsupported`) when they
are not on the allowlist. Non-strict mode records retryable `failed`/`pending`
states and continues. `--strict` (or `conversion.strict: true`) exits nonzero
when configured conversions fail or are blocked.

Cache identity includes source SHA-256, converter version, and conversion
config. Derived Markdown includes original-path attribution. An unchanged
source reuses its cache; a changed source gets a new identity.

Primary API and security guidance:
[MarkItDown README](https://github.com/microsoft/markitdown/blob/v0.1.7/README.md)
and
[`convert_local`](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/_markitdown.py).
