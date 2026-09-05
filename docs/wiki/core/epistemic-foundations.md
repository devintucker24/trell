---
id: epistemic-foundations
title: Epistemic Foundations of Trell
type: concept
status: active
created: '2026-09-04'
updated: '2026-09-04'
tags:
- epistemic-types
- belief
- certain
- core
domain: core
summary: Dual-track certain T vs belief<T> and the non-coercion rule.
nodes:
- id: belief-type
  kind: type
  label: belief<T>
- id: certain-type
  kind: type
  label: certain T
- id: epistemic-lie
  kind: concept
  label: The Epistemic Lie
- id: epistemic-contamination
  kind: concept
edges:
- from: belief-type
  to: certain-type
  rel: reduces_via
  note: via guard verify/require
- from: certain-type
  to: belief-type
  rel: extends
  note: certainty subsumption
- from: epistemic-lie
  to: belief-type
  rel: depends_on
- from: belief-type
  to: epistemic-contamination
  rel: related_to
  note: 'heal: link hard orphan'
related:
- '[[core/contract-and-guard-system]]'
- '[[theory/epistemic-type-calculus]]'
- '[[core/speculative-execution-engine]]'
implements_code:
- src/ast.rs
- src/typecheck.rs
agent:
  priority: critical
  read_when:
  - explaining what Trell is
  - type system questions
  - epistemic safety for autonomous ships under COLREGs
  maintain:
  - sync Non-Coercion rule with typecheck.is_assignable
---

# Core: Epistemic Foundations of Trell

## 1. The Epistemic Crisis in Modern Computing

Every standard programming language from Fortran and C to Python, Rust, and TypeScript operates on **classical Boolean certainty**:
$$\forall e \in \text{Expr}, \quad e \Downarrow v \implies v \text{ is grounded ground truth}.$$

When modern developers invoke foundation models or neural reasoning agents from Python (`res = model.generate(prompt)`), the runtime assigns the returned output to a standard primitive type like `str` or `dict`. 

This creates what Trell terms **The Epistemic Lie**:
1. **Semantic Ambiguity:** The host language cannot distinguish between a cryptographically verified hash from a local database and a stochastic hallucination emitted by a model sampling from a softmax distribution. Both share the type `str`.
2. **Epistemic Contamination:** An unverified belief can propagate silently through variables, arguments, and network requests until it triggers an irreversible physical or financial side effect (e.g. moving a ship's rudder, administering a hospital medication, or dispatching a SWIFT wire).
3. **The Silent Failure Trap:** The host program does not crash at compile time or even throw an exception at runtime. It executes valid code on invalid premises, producing catastrophic real-world failure modes.

---

## 2. The Dual-Track Epistemic Type System

Trell resolves this crisis by splitting all values in the universe into two distinct epistemic tracks:

```
                  ┌──────────────────────┐
                  │      Universe        │
                  └──────────┬───────────┘
                             │
            ┌────────────────┴────────────────┐
            ▼                                 ▼
   ┌─────────────────┐               ┌─────────────────┐
   │    certain T    │               │    belief<T>    │
   │  Deterministic  │               │  Probabilistic  │
   │ Cryptographic   │               │   Stochastic    │
   │ Grounded Proof  │               │ Model Reasoning │
   └────────┬────────┘               └────────┬────────┘
            │                                 │
            │    [Promotion: Trivial]         │
            │ ──────────────────────────────► │
            │                                 │
            │   [Reduction: REQUIRES PROOF]   │
            │ ◄────────────────────────────── │
            │   verify b with Guard else FB   │
            └─────────────────────────────────┘
```

### The Inviolable Epistemic Rule
$$\text{belief}\langle T \rangle \not\le \text{certain } T$$
A `belief<T>` can **never** be assigned, passed, or coerced into a `certain T` without an explicit, verifiable reduction step. 

Conversely:
$$\text{certain } T \le \text{belief}\langle T \rangle$$
Any grounded certain fact can be elevated to a belief with a confidence score of $1.0$ and a ground-truth justification.

---

## 3. Structure of a Belief Value

In Trell, a `belief<T>` is not merely an optional value or a wrapped monad. It is a structured epistemic container carrying formal metadata:

```rust
pub struct BeliefValue {
    pub value: Box<RuntimeValue>,     // The underlying data (e.g. "VeerStarboard")
    pub confidence: f64,              // Quantified epistemic certainty [0.0, 1.0]
    pub justification: String,        // Rationale / chain-of-thought provenance trace
    pub model_origin: String,         // Identity of the originating model contract
}
```

This structure guarantees that any decision derived from a model maintains an unbroken chain of custody, enabling regulatory audits and post-mortem analysis.

---

## 4. Epistemic Reduction Primitives

To cross the chasm from `belief<T>` to `certain T`, Trell provides two language-level reduction primitives:

### A. Guard Verification (`verify` / `require`)
A deterministic predicate (`guard`) is evaluated against the candidate value. If the predicate holds, the value is promoted to `certain T`. If it fails, the safe fallback value is returned.

```trell
guard SafeVelocity(v: int):
    v >= 0 and v <= 25
end

let model_speed: belief<int> = ask NavigationModel("Assess channel transit speed")
// Hard compile error if not guarded:
// let speed: certain int = model_speed // ILLEGAL!

// Correct epistemic reduction:
let speed: certain int = require model_speed with SafeVelocity else 5
```

### B. Statistical Quorum Consensus (`consensus` / `quorum`)
Where a single model might suffer from blind spots, Trell allows multi-sample consensus across $N$ independent stochastic evaluations requiring an agreement threshold $T \in (0, 1]$:

```trell
let agreement: belief<string> = quorum(5, 0.80):
    ask DiagnosticModel("Analyze pulmonary CT scan for micro-emboli")
end
```

---

## 5. Cross-References
* Speculative execution of belief distributions: [[core/speculative-execution-engine]]
* Complete grammar for Natural Trell: [[core/natural-syntax-specification]]
* Formal mathematical proofs of type safety: [[theory/epistemic-type-calculus]]
