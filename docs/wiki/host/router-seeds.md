---
id: host-router-seeds
title: Host router seeds (Trell)
type: meta
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [router, host]
domain: meta
summary: "Trell-specific ROUTER Tier-1 intent → seed pages. Replace this table when packing the wiki into another project."
nodes:
  - id: host-router-seeds
    kind: concept
    label: Host router seeds
edges:
  - from: host-router-seeds
    to: wiki-router
    rel: depends_on
related:
  - "[[ROUTER]]"
  - "[[FRAMEWORK]]"
agent:
  priority: high
  read_when:
    - "after ROUTER.md Tier 0, before retrieve"
    - "matching user intent to wiki pages"
  maintain:
    - "keep intent keywords aligned with actual page paths"
---

# Host router seeds (Trell)

Generic wiki intents live in [[ROUTER]]. **Project** intents live here (`HOST.yaml` → `router_seeds`).

| Intent signal | Seed pages (open these first) |
|---|---|
| epistemic / certainty / belief / verify / guard | `core/epistemic-foundations.md`, `core/contract-and-guard-system.md`, `theory/epistemic-type-calculus.md` |
| speculative / when / fork / rollback | `core/speculative-execution-engine.md`, `applications/overview-and-safety-patterns.md` |
| models / contracts / quorum / consensus | `core/contract-and-guard-system.md` |
| Natural Trell / colon-indent / syntax | `core/natural-syntax-specification.md` |
| compiler / lexer / parser / typecheck / src | `core/epistemic-foundations.md` + `src/` (code is ground truth) |
| ship / fleet / maritime / COLREGs / drone | `applications/autonomous-physical-systems.md`, `applications/overview-and-safety-patterns.md` |
| finance / bank / Fedwire / insurance | `applications/financial-treasury-and-markets.md` |
| healthcare / ICU / pharma | `applications/healthcare-and-life-sciences.md` |
| grid / water / train / infrastructure | `applications/critical-infrastructure-and-energy.md` |
| security / IAM / satellites / AML | `applications/security-cloud-and-governance.md` |
| market / LangChain / BAML / competitors | `market/competitive-analysis.md` |
| regulation / EU AI Act / Lloyd's | `market/regulatory-and-insurance-drivers.md` |
| vision / 10-year / phases / roadmap | `roadmap/ten-year-vision.md`, `roadmap/phases-and-milestones.md` |
