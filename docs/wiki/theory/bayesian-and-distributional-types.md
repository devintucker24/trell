---
id: bayesian-and-distributional-types
title: Bayesian & Distributional Type Systems
type: concept
status: active
created: '2026-09-04'
updated: '2026-09-04'
tags:
- bayesian
- entropy
- distributions
- roadmap
domain: theory
summary: Future distributional belief types, entropy bounds, Bayesian update.
nodes:
- id: distributional-types
  kind: type
- id: epistemic-entropy-bound
  kind: concept
edges:
- from: distributional-types
  to: belief-type
  rel: extends
- from: distributional-types
  to: phase-4-iso-silicon
  rel: milestone_of
related:
- '[[theory/epistemic-type-calculus]]'
- '[[roadmap/ten-year-vision]]'
agent:
  priority: medium
  read_when:
  - future type system
  - entropy bounds
  maintain:
  - mark clearly as roadmap vs shipped
---

# Theory: Bayesian & Distributional Type Systems

## 1. Beyond Scalar Confidence

In Trell v0.2.0, belief confidence is modeled as a scalar float:
$$c \in [0.0, 1.0].$$

While scalar confidence handles binary acceptance or rejection against contract invariants (e.g. `confidence >= 0.85`), complex autonomous systems require full **Distributional Typing**. In the 10-year evolutionary roadmap, Trell elevates probability distributions into compile-time type parameters.

---

## 2. Distributional Type Signatures

In Trell 2036, a model belief carries its underlying parametric or non-parametric family:

```trell
// Categorical probability distribution over discrete hypotheses
belief<DifferentialDiagnosis, Categorical(K=8)>

// Gaussian belief for continuous physical parameters (e.g., vessel drift velocity)
belief<SpeedKnots, Gaussian(mu: float, sigma: float)>

// Dirichlet prior over multi-agent consensus weights
belief<ResourceVector, Dirichlet(alpha: float[4])>
```

---

## 3. Epistemic Entropy Bounds

In critical systems, a model may emit an answer with high nominal confidence on its top token, but with massive **epistemic entropy** across its latent beam search. Trell introduces the `entropy` type invariant:

$$H(X) = -\sum_{i=1}^K p(x_i) \log_2 p(x_i)$$

```trell
model CollisionPredictor:
    budget: 3000
    require: entropy <= 1.2 bits
    require: confidence >= 0.90
end
```

If weather noise or sensor corruption causes the distribution to disperse widely, $H(X)$ exceeds $1.2$ bits, triggering automatic fail-safe branching before any actuator can be engaged.

---

## 4. Bayesian Posterior Updating at the Language Level

Trell's type system allows deterministic Bayesian updating when new sensory evidence is introduced:

$$P(\theta \mid D) = \frac{P(D \mid \theta) P(\theta)}{P(D)}$$

```trell
// Prior belief from radar model
let prior: belief<CourseChange, Gaussian(15.0, 4.0)> = ask RadarAI(scan)

// Update belief with deterministic sonar measurement
let posterior = update prior with SonarTelemetry(depth: 18.5)
```

The compiler proves statically whether the posterior variance $\sigma^2$ is small enough to clear the safety guard.

---

## 5. Cross-References
* Epistemic calculus: [[theory/epistemic-type-calculus]]
* Affine cognitive budgets: [[theory/affine-cognitive-economics]]
* Maritime collision applications: [[applications/autonomous-physical-systems]]
