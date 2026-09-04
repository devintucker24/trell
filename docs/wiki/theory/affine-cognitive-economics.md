---
id: affine-cognitive-economics
title: Affine Cognitive Economics & Resource Invariants
type: concept
status: active
created: '2026-09-04'
updated: '2026-09-04'
tags:
- affine-types
- budgets
- tokens
- energy
domain: theory
summary: Linear/affine budgets for tokens, joules, and dollar cost ceilings.
nodes:
- id: affine-cognitive-budget
  kind: concept
- id: token-budget
  kind: primitive
edges:
- from: model-contract
  to: affine-cognitive-budget
  rel: enforces
- from: affine-cognitive-budget
  to: app-treasury-fedwire
  rel: applies_to
related:
- '[[core/contract-and-guard-system]]'
- '[[applications/financial-treasury-and-markets]]'
agent:
  priority: medium
  read_when:
  - cost ceilings
  - runaway agents
  maintain: []
---

# Theory: Affine Cognitive Economics & Resource Invariants

## 1. The Autonomous Agent Resource Dilemma

In traditional programming, unbounded loops cause out-of-memory errors or 100% CPU spikes. In autonomous agent programming, unbounded agentic loops cause **catastrophic economic and physical depletion**:
* An agent caught in an ambiguous reasoning loop can consume thousands of dollars in API credits in minutes.
* On an autonomous maritime vessel, drone, or Mars rover, an infinite inference loop depletes battery reserves and overheats silicon in remote environments.

---

## 2. Affine & Linear Resource Types

Just as Rust utilizes affine types to guarantee that memory is freed exactly once without a garbage collector, Trell applies **Affine Type Economics** to computational resources:

$$\text{Tokens} \times \text{Compute Energy (Joules)} \times \text{Financial Cost (USD)}$$

In Trell, computational budgets are **linear resources**:
$$\text{Budget} \to \text{Budget} - \Delta c$$

If any execution path in an action or recursive agent loop can potentially consume more than the statically allocated budget, **the program fails to compile**:

```trell
action deliberate_mission(ctx: certain EnvironmentContext):
    budget: 4500 tokens
    cost_ceiling: $0.15

    // Statically checked: Recursive branches cannot exceed 4500 total tokens
    let step1 = ask PlannerAI(ctx)
    // ...
end
```

---

## 3. Cognitive Deadlock & Livelock Prevention

Through compile-time call-graph analysis, Trell detects circular delegation between autonomous agents:
$$\text{Agent } A \implies \text{Agent } B \implies \text{Agent } A$$

Because each hop consumes affine budget tokens that cannot be replenished without external human or cron authorization, recursive deliberation loops strictly terminate.

---

## 4. Cross-References
* Model contracts and budgets: [[core/contract-and-guard-system]]
* Hardware co-design: [[theory/hardware-silicon-codesign]]
* Financial settlement guarantees: [[applications/financial-treasury-and-markets]]
