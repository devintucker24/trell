---
id: 2026-09-04-graphify-machine-graph
title: "Episode: Graphify as code graph + wiki-setup"
type: episode
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [episodic, graphify, setup, graph]
domain: episodic
summary: Decision to pull Graphify for the machine code graph and add wiki-setup for portable installs; corpus doctrine stays authored.
nodes:
  - id: episode-graphify-machine-graph
    kind: concept
    label: Graphify machine-graph episode
edges:
  - from: episode-graphify-machine-graph
    to: wiki-graphify-bridge
    rel: related_to
  - from: episode-graphify-machine-graph
    to: wiki-setup
    rel: related_to
related:
  - "[[episodic/INDEX]]"
  - "[[temporal/TIMELINE]]"
  - "[[FRAMEWORK]]"
  - "[[_meta/GRAPH]]"
temporal:
  observed_at: 2026-09-04
  valid_from: 2026-09-04
  valid_until: null
  supersedes: []
  superseded_by: null
agent:
  priority: medium
  read_when:
    - recalling why we do not roll our own code graph
    - portable pack setup design
  maintain:
    - consolidate into FRAMEWORK/_meta/GRAPH; then mark stale
episode:
  goal: "Use Graphify as the agent/machine code graph; add a setup skill that stands up a new host with minimal human work."
  outcome: success
  promote: true
---

# Episode: Graphify as the machine graph

## Goal
Stop treating `GRAPH.yaml` as a human map or a homegrown Graphify. Pull Graphify for code structure. Make portable installs agent-driven.

## Actions
- Installed Graphify (`pip install graphifyy`, CLI `graphify`).
- Extracted Trell `src/` `--code-only`: ~151 nodes / ~513 edges, almost all `EXTRACTED` (calls, imports, contains).
- God nodes on this compiler: Parser, Expr, RuntimeValue, TypeChecker, Token, Stmt, …
- Added `wiki_graphify.py`, `wiki_setup.py`, skill `wiki-setup`.

## Decisions
- **Code graph** = Graphify `graphify-out/graph.json`. We do not build a second AST graph.
- **Claim graph** = page frontmatter compiled to `_meta/GRAPH.yaml` for retrieve hops (`reduces_via`, …). Graphify cannot emit Trell doctrine.
- **Corpus pages** are not auto-generated as thesis. Graphify `--wiki` is regenerated structure. `--seed-pages` writes drafts only on an empty host.
- **Setup** is the portable on-ramp: detect repo → HOST.yaml → folders → launchers → Graphify sync → optional seeds. Human leftover: `anchor` + review drafts.

## Lessons
- Graphify `--code-only` needs no API key; good for CI and cloud agents.
- Query is token-cheap vs reading `src/`. Do not paste `graph.json`.
- INFERRED edges are not wiki truth.

## Open threads
- Folder move of pack machinery into `_system/` still pending (grilling).
- Whether to commit a tiny Graphify hook (`graphify hook install`) later.
