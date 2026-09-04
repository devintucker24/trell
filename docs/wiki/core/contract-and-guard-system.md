---
id: contract-and-guard-system
title: Model Contracts & Verification Guard System
type: concept
status: active
created: '2026-09-04'
updated: '2026-09-04'
tags:
- contracts
- guards
- quorum
- require
domain: core
summary: Model contracts, deterministic guards, require/verify, and quorum consensus.
nodes:
- id: model-contract
  kind: primitive
- id: guard-verify
  kind: primitive
  label: guard / require / verify
- id: quorum-consensus
  kind: primitive
edges:
- from: guard-verify
  to: belief-type
  rel: enforces
- from: guard-verify
  to: certain-type
  rel: reduces_via
- from: quorum-consensus
  to: belief-type
  rel: extends
- from: model-contract
  to: affine-cognitive-budget
  rel: depends_on
related:
- '[[core/epistemic-foundations]]'
- '[[theory/epistemic-type-calculus]]'
- '[[theory/affine-cognitive-economics]]'
implements_code:
- src/oracle.rs
- src/interpreter.rs
- src/typecheck.rs
agent:
  priority: critical
  read_when:
  - verification
  - quorum
  - model invariants
  maintain:
  - keep contract fields aligned with ModelContract AST
---

# Core: Model Contracts & Verification Guard System

## 1. Model Contracts as First-Class Signatures

In conventional languages, a function signature specifies types (e.g., `def assess(text: str) -> str`). It says nothing about:
* What temperature or sampling parameters the model must use.
* What context or token budget it is constrained to.
* What formal statistical confidence threshold it must satisfy.

In Trell, **Model Contracts** (`model` or `contract`) are first-class language definitions that establish cognitive invariants enforced at runtime.

### Natural Trell Syntax
```trell
model DiagnosticOracle:
    temperature: 0.1
    budget: 2000
    require: confidence >= 0.85
end
```

### Compiler Invariants
1. **Sampling Guard:** The model is bounded to a maximum temperature of `0.1` to prevent divergent or creative hallucination in safety-critical domains.
2. **Affine Token Budget:** The model invocation is bounded to 2,000 tokens. Recursive or runaway reasoning loops are rejected.
3. **Cognitive Invariant Enforcement:** When the model returns, if its calculated epistemic confidence is $< 0.85$, the Trell runtime triggers an invariant violation:
```text
Cognitive Invariant Violation: Model confidence 0.80 violates contract 'DiagnosticOracle' minimum of 0.85
```

---

## 2. Deterministic Verification Guards (`guard`)

A `guard` is a pure, deterministic predicate that serves as the **epistemic bouncer** between the probabilistic world of AI and the physical world of deterministic hardware.

### Structure of a Guard
```trell
guard ClearWaterway(action: string):
    action == "HoldCourse" or action == "VeerStarboard" or action == "ThrottleDown"
end
```

### Invariants of Guards
* **Purely Deterministic:** A guard cannot call an oracle, allocate unbounded memory, or produce non-deterministic side effects.
* **Boolean Grounding:** A guard must return `certain bool`.
* **Exhaustive Formal Soundness:** The guard defines the exact subset of the domain space that is physically or legally admissible.

---

## 3. The Epistemic Reduction Construct (`require` / `verify`)

The bridge from `belief<T>` to `certain T` is constructed using `require ... with ... else ...`:

```trell
let safe_action: certain string = require obstacle_belief with ClearWaterway else "ThrottleDown"
```

### Operational Semantics
1. The expression evaluates `obstacle_belief`, yielding an inner value $v$ and confidence $c$.
2. The deterministic predicate `ClearWaterway(v)` is executed.
3. If `ClearWaterway(v) == true`, $v$ is promoted to `certain string`.
4. If `ClearWaterway(v) == false`, the fallback value `"ThrottleDown"` is evaluated and returned as `certain string`.
5. Under no circumstances can an invalid, hallucinated string escape into the remainder of the program.

---

## 4. Statistical Quorum Consensus (`quorum` / `consensus`)

In high-stakes treasury or security operations, a single model sample—no matter how confident—may represent a localized distribution failure. Trell provides a native **Statistical Quorum** primitive:

```trell
let verified_verdict: belief<string> = quorum(3, 0.70):
    ask FraudOracle("High-speed interbank wire $1,250,000 to offshore clearing agency")
end
```

### Execution Flow
1. Trell dispatches $N = 3$ independent stochastic queries to the oracle.
2. The runtime aggregates returned semantic representations and calculates the agreement ratio $R = \frac{\text{matching\_votes}}{N}$.
3. If $R \ge 0.70$, the consensus verdict is emitted with an amplified joint confidence score $C_{\text{joint}} = \bar{C} \times R$.
4. If $R < 0.70$, the consensus fails, triggering an immediate fallback or escalation path.

---

## 5. Cross-References
* Dual-track type foundations: [[core/epistemic-foundations]]
* Mathematical type calculus and proofs: [[theory/epistemic-type-calculus]]
* Bank transfer consensus implementation: `examples/bank_transfer.trell`
