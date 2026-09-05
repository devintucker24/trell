---
id: wiki-index
title: Trell Epistemic Language Knowledge Base Index
type: index
status: active
created: '2026-09-04'
updated: '2026-09-04'
tags:
- index
- navigation
domain: meta
summary: Master catalog of the Trell wiki brain for agent navigation.
nodes:
- id: wiki-index
  kind: concept
  label: Wiki Index
edges:
- from: wiki-index
  to: belief-type
  rel: related_to
- from: wiki-index
  to: three-beat-safety-pattern
  rel: related_to
- from: wiki-index
  to: ten-year-vision
  rel: related_to
related:
- '[[SCHEMA]]'
- '[[ROUTER]]'
- '[[core/epistemic-foundations]]'
- '[[roadmap/ten-year-vision]]'
- '[[episodic/INDEX]]'
- '[[temporal/TIMELINE]]'
- '[[FRAMEWORK]]'
- '[[_meta/GRAPH]]'
- '[[_meta/usage-telemetry]]'
agent:
  priority: critical
  read_when:
  - starting any wiki session
  - finding a page
  maintain:
  - update on every structural page add/remove
---

# Trell Epistemic Language Knowledge Base & Technical Wiki

Welcome to the **Trell Knowledge Base**, modeled after Andrej Karpathy's networked hyperlinked wiki system (LLM Wiki). This repository is a **compounding brain**: agents compile knowledge into interlinked markdown with YAML graph metadata; they do not re-derive everything from raw sources on every query.

### Agent bootstrap (read first)
1. [`AGENTS.md`](../../AGENTS.md) — thin project brief (always-on)
2. [`CLAUDE.md`](../../CLAUDE.md) — Claude Code twin brief
3. [`_system/docs/ROUTER.md`](_system/docs/ROUTER.md) — context map + budgets
4. [`_system/docs/OPERATOR.md`](_system/docs/OPERATOR.md) — operator manual
5. Retrieve: `python3 docs/wiki/_system/scripts/wiki_retrieve.py "<q>"`
6. Code graph: `python3 docs/wiki/_system/scripts/wiki_graphify.py query "<q>"`
7. This INDEX — catalog when browsing structure
8. [`_system/docs/SCHEMA.md`](_system/docs/SCHEMA.md) — corpus schema
9. Memory lanes: [`episodic/`](episodic/INDEX.md) · [`temporal/TIMELINE.md`](temporal/TIMELINE.md)
10. [`_system/generated/claim-graph.yaml`](_system/generated/claim-graph.yaml)
11. Engine: [`_system/`](_system/README.md)

**Layers:** raw sources → reviewed host corpus → portable `_system/` engine.
**Context rule:** use the Router and retrieval instead of loading this index.

---

## 1. Core Architecture & Language Design
* [[core/epistemic-foundations]]: The fundamental philosophy of epistemic dual-track typing (`certain T` vs `belief<T>`) and why AI requires language-level separation of truth and belief.
* [[core/natural-syntax-specification]]: Complete formal grammar, keywords, and structural conventions of **Natural Trell** (Colon + Indentation + Explicit `end` blocks).
* [[core/speculative-execution-engine]]: How Trell evaluates semantic hypothesis branches concurrently, with transactional rollback and zero-latency superposition collapse.
* [[core/contract-and-guard-system]]: First-class model contracts, token budgets, deterministic verification predicates, and multi-sample statistical quorums.

---

## 2. Scientific, Mathematical & Technological Foundations
* [[theory/epistemic-type-calculus]]: Formal type theory, proofs of epistemic soundness, lattice reduction, and preventing runtime taint contamination.
* [[theory/bayesian-and-distributional-types]]: Moving from scalar confidence intervals to Dirichlet, Gaussian, and categorical probability distributions on the type level.
* [[theory/affine-cognitive-economics]]: Linear and affine type systems applied to token budgets, inference compute bounds, and power consumption invariants.
* [[theory/cryptographic-model-provenance]]: Zero-Knowledge Epistemic Proofs (ZK-EP), verifiable model weights, and immutable execution ledgers.
* [[theory/hardware-silicon-codesign]]: Co-designing Neural Processing Units (NPUs) and LPUs with silicon-level branch prediction and transactional register rollbacks.

---

## 3. High-Stakes Industry Niches & 20 Real-World Applications
* [[applications/overview-and-safety-patterns]]: The universal Three-Beat Epistemic Safety Pattern that bridges probabilistic AI and deterministic physics.
* [[applications/autonomous-physical-systems]]: Deep-dive into Maritime Collision Avoidance (COLREGs), Commercial Drone Airspaces, Heavy Mining Haulage, Robotic Surgery, and Nuclear Core Regulation.
* [[applications/healthcare-and-life-sciences]]: ICU Sepsis Mitigation, Radiation Oncology Dosing, Genomic Cancer Sequencing, and Automated Pharmacy Dispensaries.
* [[applications/financial-treasury-and-markets]]: High-Speed Fedwire/SWIFT RTGS Settlement, Flash Crash Liquidity Defenses, Catastrophic Claim Automation, and Sovereign FX Rebalancing.
* [[applications/critical-infrastructure-and-energy]]: Regional Smart Grid Frequency Regulation, Municipal Water Treatment Dosing, and Bullet Train Braking Interlocks.
* [[applications/security-cloud-and-governance]]: Autonomous Kernel Hot-Patching, Zero-Trust IAM Policy Synthesis, Orbital Satellite Deconfliction, and Federal AML Quorum Verification.

---

## 4. Market Landscape & Competitive Dynamics
* [[market/competitive-analysis]]: Comprehensive breakdown of why Python, LangGraph, BAML, Weft, DSPy, and vendor SDKs (OpenAI, Anthropic, Microsoft) fail to solve the epistemic safety crisis.
* [[market/regulatory-and-insurance-drivers]]: Why the EU AI Act, IMO autonomous maritime standards, FAA mandates, and Lloyd's of London underwriting policies will require verified languages like Trell.
* [[market/developer-persona-and-adoption]]: Who writes Trell: Deliberative AI Agents, Systems Safety Engineers, Regulatory Auditors, and Domain Specialists.

---

## 5. Ten-Year Strategic Roadmap (2026 – 2036)
* [[roadmap/ten-year-vision]]: Detailed evolutionary trajectory from modern compiler prototype to universal ISO/IEEE autonomous systems standard and silicon co-design.
* [[roadmap/phases-and-milestones]]: Phase 1 (Niche Dominance) -> Phase 2 (Ecosystem & Agent Codegen) -> Phase 3 (AOT LLVM/WASM Compilers) -> Phase 4 (Silicon Integration & Global Regulation).

---

## Quick Navigation Index

| Topic | Primary Article | Cross References |
| :--- | :--- | :--- |
| **Epistemic Typing** | [[core/epistemic-foundations]] | [[theory/epistemic-type-calculus]], [[core/contract-and-guard-system]] |
| **Natural Syntax** | [[core/natural-syntax-specification]] | [[examples/autonomous_ship.trell]], [[examples/bank_transfer.trell]] |
| **Speculative Branches** | [[core/speculative-execution-engine]] | [[theory/hardware-silicon-codesign]], [[applications/autonomous-physical-systems]] |
| **Market Niches** | [[applications/overview-and-safety-patterns]] | [[market/competitive-analysis]], [[market/regulatory-and-insurance-drivers]] |
| **Future Roadmap** | [[roadmap/ten-year-vision]] | [[roadmap/phases-and-milestones]], [[theory/bayesian-and-distributional-types]] |
| **Knowledge Graph** | [[_system/docs/GRAPH]] | Graphify code + generated claims |
| **Context Router** | [[_system/docs/ROUTER]] | [[_system/docs/CONTEXT_PROTOCOL]] |
| **Episodic memory** | [[episodic/INDEX]] | [[episodic/session-current]], [[temporal/TIMELINE]] |
| **Temporal memory** | [[temporal/TIMELINE]] | retrieve `--as-of` |

---

## 6. Brain Ops (Schema, Graph, Memory, Raw, Inbox, Log)
* [[_system/docs/ROUTER]]: Progressive-disclosure router + token budgets
* [[_system/docs/OPERATOR]]: Detailed operator manual
* [[_system/docs/CONTEXT_PROTOCOL]]: Context assembly rules
* [[_system/docs/SCHEMA]]: Corpus frontmatter contract
* [[episodic/INDEX]]: Episodic memory catalog (session narratives)
* [[temporal/TIMELINE]]: Temporal spine for as-of / what-changed recall
* [[inbox/README]]: **Drop zone** — how humans/agents input material
* [[_system/docs/GRAPH]]: Machine graph protocol
* [[raw/thesis]]: Pointer to immutable THESIS.md
* [[raw/examples]]: Pointers to executable `.trell` examples
* [[raw/market-research-2026-09-03]]: Historical market note (reconcile carefully)
* `_system/logs/operations.md`: Append-only operations log
* [[_system/docs/FRAMEWORK]]: Portable engine export and setup
* [[_system/docs/usage-telemetry]]: Usage metric catalog
* `_system/config/router-seeds.md`: Trell-specific router seeds

---

## 7. Agent Skills (`docs/wiki/_system/skills/`)
| Skill | Path | Job |
|-------|------|-----|
| Parent | `_system/skills/wiki-brain/SKILL.md` | Portable engine entry |
| **Retrieve** | `_system/skills/wiki-retrieve/SKILL.md` | File RAG |
| Navigate | `_system/skills/wiki-navigate/SKILL.md` | Corpus + graph traversal |
| Triage | `_system/skills/wiki-triage/SKILL.md` | Inbox classification |
| Ingest | `_system/skills/wiki-ingest/SKILL.md` | Reviewed promotion |
| Doctor | `_system/skills/wiki-doctor/SKILL.md` | Structural diagnosis |
| Heal | `_system/skills/wiki-heal/SKILL.md` | Doctor-driven repair |
| Lint | `_system/skills/wiki-lint/SKILL.md` | Doctor → heal → doctor |
| Query | `_system/skills/wiki-query/SKILL.md` | Cited answers |
| Label | `_system/skills/wiki-label/SKILL.md` | Frontmatter normalization |
| Maintain | `_system/skills/wiki-maintain/SKILL.md` | Synchronization |
| Usage | `_system/skills/wiki-usage/SKILL.md` | Context telemetry |
| **Setup** | `_system/skills/wiki-setup/SKILL.md` | Portable install |
| Engine | `_system/docs/FRAMEWORK.md` | Export into another project |
