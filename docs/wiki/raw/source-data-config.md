---
id: raw-sources-data-config
title: "Data and configuration sources"
type: raw-pointer
status: active
created: '2026-09-05'
updated: '2026-09-05'
tags: [raw, source-inventory, data-config]
domain: meta
summary: "Managed non-authoritative pointers to data and configuration sources."
origin: source-manifest
managed_by: repobrain-source-pipeline
nodes: []
edges: []
related:
  - "[[INDEX]]"
agent:
  priority: low
  read_when:
    - consulting original repository sources
  maintain:
    - do not promote these paths into compiled claims
---

# Data and configuration sources

These are inventory pointers to original repository sources. They are raw,
non-authoritative material and are not compiled semantic claims.

- [`.agents/skills/ask-matt/agents/openai.yaml`](../../../.agents/skills/ask-matt/agents/openai.yaml)
- [`.agents/skills/claude-handoff/agents/openai.yaml`](../../../.agents/skills/claude-handoff/agents/openai.yaml)
- [`.agents/skills/code-review/agents/openai.yaml`](../../../.agents/skills/code-review/agents/openai.yaml)
- [`.agents/skills/codebase-design/agents/openai.yaml`](../../../.agents/skills/codebase-design/agents/openai.yaml)
- [`.agents/skills/diagnosing-bugs/agents/openai.yaml`](../../../.agents/skills/diagnosing-bugs/agents/openai.yaml)
- [`.agents/skills/domain-modeling/agents/openai.yaml`](../../../.agents/skills/domain-modeling/agents/openai.yaml)
- [`.agents/skills/git-guardrails-claude-code/agents/openai.yaml`](../../../.agents/skills/git-guardrails-claude-code/agents/openai.yaml)
- [`.agents/skills/grill-me/agents/openai.yaml`](../../../.agents/skills/grill-me/agents/openai.yaml)
- [`.agents/skills/grill-with-docs/agents/openai.yaml`](../../../.agents/skills/grill-with-docs/agents/openai.yaml)
- [`.agents/skills/grilling/agents/openai.yaml`](../../../.agents/skills/grilling/agents/openai.yaml)
- [`.agents/skills/handoff/agents/openai.yaml`](../../../.agents/skills/handoff/agents/openai.yaml)
- [`.agents/skills/implement-spec/agents/openai.yaml`](../../../.agents/skills/implement-spec/agents/openai.yaml)
- [`.agents/skills/implement/agents/openai.yaml`](../../../.agents/skills/implement/agents/openai.yaml)
- [`.agents/skills/improve-codebase-architecture/agents/openai.yaml`](../../../.agents/skills/improve-codebase-architecture/agents/openai.yaml)
- [`.agents/skills/loop-me/agents/openai.yaml`](../../../.agents/skills/loop-me/agents/openai.yaml)
- [`.agents/skills/migrate-to-shoehorn/agents/openai.yaml`](../../../.agents/skills/migrate-to-shoehorn/agents/openai.yaml)
- [`.agents/skills/prototype/agents/openai.yaml`](../../../.agents/skills/prototype/agents/openai.yaml)
- [`.agents/skills/read-handoff/agents/openai.yaml`](../../../.agents/skills/read-handoff/agents/openai.yaml)
- [`.agents/skills/research/agents/openai.yaml`](../../../.agents/skills/research/agents/openai.yaml)
- [`.agents/skills/resolving-merge-conflicts/agents/openai.yaml`](../../../.agents/skills/resolving-merge-conflicts/agents/openai.yaml)
- [`.agents/skills/retro/agents/openai.yaml`](../../../.agents/skills/retro/agents/openai.yaml)
- [`.agents/skills/scaffold-exercises/agents/openai.yaml`](../../../.agents/skills/scaffold-exercises/agents/openai.yaml)
- [`.agents/skills/setup-matt-pocock-skills/agents/openai.yaml`](../../../.agents/skills/setup-matt-pocock-skills/agents/openai.yaml)
- [`.agents/skills/setup-pre-commit/agents/openai.yaml`](../../../.agents/skills/setup-pre-commit/agents/openai.yaml)
- [`.agents/skills/setup-ts-deep-modules/agents/openai.yaml`](../../../.agents/skills/setup-ts-deep-modules/agents/openai.yaml)
- [`.agents/skills/tdd/agents/openai.yaml`](../../../.agents/skills/tdd/agents/openai.yaml)
- [`.agents/skills/teach/agents/openai.yaml`](../../../.agents/skills/teach/agents/openai.yaml)
- [`.agents/skills/to-questionnaire/agents/openai.yaml`](../../../.agents/skills/to-questionnaire/agents/openai.yaml)
- [`.agents/skills/to-spec/agents/openai.yaml`](../../../.agents/skills/to-spec/agents/openai.yaml)
- [`.agents/skills/to-tickets/agents/openai.yaml`](../../../.agents/skills/to-tickets/agents/openai.yaml)
- [`.agents/skills/triage/agents/openai.yaml`](../../../.agents/skills/triage/agents/openai.yaml)
- [`.agents/skills/wait-what/agents/openai.yaml`](../../../.agents/skills/wait-what/agents/openai.yaml)
- [`.agents/skills/wayfinder/agents/openai.yaml`](../../../.agents/skills/wayfinder/agents/openai.yaml)
- [`.agents/skills/wizard/agents/openai.yaml`](../../../.agents/skills/wizard/agents/openai.yaml)
- [`.agents/skills/writing-beats/agents/openai.yaml`](../../../.agents/skills/writing-beats/agents/openai.yaml)
- [`.agents/skills/writing-for-agents/agents/openai.yaml`](../../../.agents/skills/writing-for-agents/agents/openai.yaml)
- [`.agents/skills/writing-fragments/agents/openai.yaml`](../../../.agents/skills/writing-fragments/agents/openai.yaml)
- [`.agents/skills/writing-shape/agents/openai.yaml`](../../../.agents/skills/writing-shape/agents/openai.yaml)
- [`.cargo/config.toml`](../../../.cargo/config.toml)
- [`.claude/skills/ask-matt/agents/openai.yaml`](../../../.claude/skills/ask-matt/agents/openai.yaml)
- [`.claude/skills/claude-handoff/agents/openai.yaml`](../../../.claude/skills/claude-handoff/agents/openai.yaml)
- [`.claude/skills/code-review/agents/openai.yaml`](../../../.claude/skills/code-review/agents/openai.yaml)
- [`.claude/skills/codebase-design/agents/openai.yaml`](../../../.claude/skills/codebase-design/agents/openai.yaml)
- [`.claude/skills/diagnosing-bugs/agents/openai.yaml`](../../../.claude/skills/diagnosing-bugs/agents/openai.yaml)
- [`.claude/skills/domain-modeling/agents/openai.yaml`](../../../.claude/skills/domain-modeling/agents/openai.yaml)
- [`.claude/skills/git-guardrails-claude-code/agents/openai.yaml`](../../../.claude/skills/git-guardrails-claude-code/agents/openai.yaml)
- [`.claude/skills/grill-me/agents/openai.yaml`](../../../.claude/skills/grill-me/agents/openai.yaml)
- [`.claude/skills/grill-with-docs/agents/openai.yaml`](../../../.claude/skills/grill-with-docs/agents/openai.yaml)
- [`.claude/skills/grilling/agents/openai.yaml`](../../../.claude/skills/grilling/agents/openai.yaml)
- [`.claude/skills/handoff/agents/openai.yaml`](../../../.claude/skills/handoff/agents/openai.yaml)
- [`.claude/skills/implement-spec/agents/openai.yaml`](../../../.claude/skills/implement-spec/agents/openai.yaml)
- [`.claude/skills/implement/agents/openai.yaml`](../../../.claude/skills/implement/agents/openai.yaml)
- [`.claude/skills/improve-codebase-architecture/agents/openai.yaml`](../../../.claude/skills/improve-codebase-architecture/agents/openai.yaml)
- [`.claude/skills/loop-me/agents/openai.yaml`](../../../.claude/skills/loop-me/agents/openai.yaml)
- [`.claude/skills/migrate-to-shoehorn/agents/openai.yaml`](../../../.claude/skills/migrate-to-shoehorn/agents/openai.yaml)
- [`.claude/skills/prototype/agents/openai.yaml`](../../../.claude/skills/prototype/agents/openai.yaml)
- [`.claude/skills/research/agents/openai.yaml`](../../../.claude/skills/research/agents/openai.yaml)
- [`.claude/skills/resolving-merge-conflicts/agents/openai.yaml`](../../../.claude/skills/resolving-merge-conflicts/agents/openai.yaml)
- [`.claude/skills/retro/agents/openai.yaml`](../../../.claude/skills/retro/agents/openai.yaml)
- [`.claude/skills/scaffold-exercises/agents/openai.yaml`](../../../.claude/skills/scaffold-exercises/agents/openai.yaml)
- [`.claude/skills/setup-matt-pocock-skills/agents/openai.yaml`](../../../.claude/skills/setup-matt-pocock-skills/agents/openai.yaml)
- [`.claude/skills/setup-pre-commit/agents/openai.yaml`](../../../.claude/skills/setup-pre-commit/agents/openai.yaml)
- [`.claude/skills/setup-ts-deep-modules/agents/openai.yaml`](../../../.claude/skills/setup-ts-deep-modules/agents/openai.yaml)
- [`.claude/skills/tdd/agents/openai.yaml`](../../../.claude/skills/tdd/agents/openai.yaml)
- [`.claude/skills/teach/agents/openai.yaml`](../../../.claude/skills/teach/agents/openai.yaml)
- [`.claude/skills/to-questionnaire/agents/openai.yaml`](../../../.claude/skills/to-questionnaire/agents/openai.yaml)
- [`.claude/skills/to-spec/agents/openai.yaml`](../../../.claude/skills/to-spec/agents/openai.yaml)
- [`.claude/skills/to-tickets/agents/openai.yaml`](../../../.claude/skills/to-tickets/agents/openai.yaml)
- [`.claude/skills/triage/agents/openai.yaml`](../../../.claude/skills/triage/agents/openai.yaml)
- [`.claude/skills/wait-what/agents/openai.yaml`](../../../.claude/skills/wait-what/agents/openai.yaml)
- [`.claude/skills/wayfinder/agents/openai.yaml`](../../../.claude/skills/wayfinder/agents/openai.yaml)
- [`.claude/skills/wizard/agents/openai.yaml`](../../../.claude/skills/wizard/agents/openai.yaml)
- [`.claude/skills/writing-beats/agents/openai.yaml`](../../../.claude/skills/writing-beats/agents/openai.yaml)
- [`.claude/skills/writing-for-agents/agents/openai.yaml`](../../../.claude/skills/writing-for-agents/agents/openai.yaml)
- [`.claude/skills/writing-fragments/agents/openai.yaml`](../../../.claude/skills/writing-fragments/agents/openai.yaml)
- [`.claude/skills/writing-shape/agents/openai.yaml`](../../../.claude/skills/writing-shape/agents/openai.yaml)
- [`.cursor/environment.json`](../../../.cursor/environment.json)
- [`.cursor/skills/ask-matt/agents/openai.yaml`](../../../.cursor/skills/ask-matt/agents/openai.yaml)
- [`.cursor/skills/claude-handoff/agents/openai.yaml`](../../../.cursor/skills/claude-handoff/agents/openai.yaml)
- [`.cursor/skills/code-review/agents/openai.yaml`](../../../.cursor/skills/code-review/agents/openai.yaml)
- [`.cursor/skills/codebase-design/agents/openai.yaml`](../../../.cursor/skills/codebase-design/agents/openai.yaml)
- [`.cursor/skills/diagnosing-bugs/agents/openai.yaml`](../../../.cursor/skills/diagnosing-bugs/agents/openai.yaml)
- [`.cursor/skills/domain-modeling/agents/openai.yaml`](../../../.cursor/skills/domain-modeling/agents/openai.yaml)
- [`.cursor/skills/git-guardrails-claude-code/agents/openai.yaml`](../../../.cursor/skills/git-guardrails-claude-code/agents/openai.yaml)
- [`.cursor/skills/grill-me/agents/openai.yaml`](../../../.cursor/skills/grill-me/agents/openai.yaml)
- [`.cursor/skills/grill-with-docs/agents/openai.yaml`](../../../.cursor/skills/grill-with-docs/agents/openai.yaml)
- [`.cursor/skills/grilling/agents/openai.yaml`](../../../.cursor/skills/grilling/agents/openai.yaml)
- [`.cursor/skills/handoff/agents/openai.yaml`](../../../.cursor/skills/handoff/agents/openai.yaml)
- [`.cursor/skills/implement-spec/agents/openai.yaml`](../../../.cursor/skills/implement-spec/agents/openai.yaml)
- [`.cursor/skills/implement/agents/openai.yaml`](../../../.cursor/skills/implement/agents/openai.yaml)
- [`.cursor/skills/improve-codebase-architecture/agents/openai.yaml`](../../../.cursor/skills/improve-codebase-architecture/agents/openai.yaml)
- [`.cursor/skills/loop-me/agents/openai.yaml`](../../../.cursor/skills/loop-me/agents/openai.yaml)
- [`.cursor/skills/migrate-to-shoehorn/agents/openai.yaml`](../../../.cursor/skills/migrate-to-shoehorn/agents/openai.yaml)
- [`.cursor/skills/prototype/agents/openai.yaml`](../../../.cursor/skills/prototype/agents/openai.yaml)
- [`.cursor/skills/research/agents/openai.yaml`](../../../.cursor/skills/research/agents/openai.yaml)
- [`.cursor/skills/resolving-merge-conflicts/agents/openai.yaml`](../../../.cursor/skills/resolving-merge-conflicts/agents/openai.yaml)
- [`.cursor/skills/retro/agents/openai.yaml`](../../../.cursor/skills/retro/agents/openai.yaml)
- [`.cursor/skills/scaffold-exercises/agents/openai.yaml`](../../../.cursor/skills/scaffold-exercises/agents/openai.yaml)
- [`.cursor/skills/setup-matt-pocock-skills/agents/openai.yaml`](../../../.cursor/skills/setup-matt-pocock-skills/agents/openai.yaml)
- [`.cursor/skills/setup-pre-commit/agents/openai.yaml`](../../../.cursor/skills/setup-pre-commit/agents/openai.yaml)
- [`.cursor/skills/setup-ts-deep-modules/agents/openai.yaml`](../../../.cursor/skills/setup-ts-deep-modules/agents/openai.yaml)
- [`.cursor/skills/tdd/agents/openai.yaml`](../../../.cursor/skills/tdd/agents/openai.yaml)
- [`.cursor/skills/teach/agents/openai.yaml`](../../../.cursor/skills/teach/agents/openai.yaml)
- [`.cursor/skills/to-questionnaire/agents/openai.yaml`](../../../.cursor/skills/to-questionnaire/agents/openai.yaml)
- [`.cursor/skills/to-spec/agents/openai.yaml`](../../../.cursor/skills/to-spec/agents/openai.yaml)
- [`.cursor/skills/to-tickets/agents/openai.yaml`](../../../.cursor/skills/to-tickets/agents/openai.yaml)
- [`.cursor/skills/triage/agents/openai.yaml`](../../../.cursor/skills/triage/agents/openai.yaml)
- [`.cursor/skills/wait-what/agents/openai.yaml`](../../../.cursor/skills/wait-what/agents/openai.yaml)
- [`.cursor/skills/wayfinder/agents/openai.yaml`](../../../.cursor/skills/wayfinder/agents/openai.yaml)
- [`.cursor/skills/wizard/agents/openai.yaml`](../../../.cursor/skills/wizard/agents/openai.yaml)
- [`.cursor/skills/writing-beats/agents/openai.yaml`](../../../.cursor/skills/writing-beats/agents/openai.yaml)
- [`.cursor/skills/writing-for-agents/agents/openai.yaml`](../../../.cursor/skills/writing-for-agents/agents/openai.yaml)
- [`.cursor/skills/writing-fragments/agents/openai.yaml`](../../../.cursor/skills/writing-fragments/agents/openai.yaml)
- [`.cursor/skills/writing-shape/agents/openai.yaml`](../../../.cursor/skills/writing-shape/agents/openai.yaml)
- [`.gitignore`](../../../.gitignore)
- [`.handoffs/.gitignore`](../../../.handoffs/.gitignore)
- [`Cargo.toml`](../../../Cargo.toml)
- [`examples/scenarios/enterprise_clean.json`](../../../examples/scenarios/enterprise_clean.json)
- [`examples/scenarios/risk_alert.json`](../../../examples/scenarios/risk_alert.json)
- [`skills-lock.json`](../../../skills-lock.json)
