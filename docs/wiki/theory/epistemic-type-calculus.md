# Theory: Epistemic Type Calculus & Soundness Proofs

## 1. Abstract Syntax & Type Grammar

Let $\mathcal{T}$ be the set of types in Trell:
$$\tau ::= \text{certain } \sigma \mid \text{belief}\langle \sigma \rangle \mid ()$$
$$\sigma ::= \text{Int} \mid \text{Float} \mid \text{Bool} \mid \text{String} \mid \mathcal{C}$$
where $\mathcal{C}$ denotes user-defined algebraic and nominal struct types.

### Expressions
$$e ::= x \mid c \mid e_1 \text{ op } e_2 \mid \text{ask } M(e) \mid \text{require } e_1 \text{ with } g \text{ else } e_2 \mid \text{quorum}(n, \theta): e \mid \text{confidence}(e) \mid \text{justification}(e)$$

---

## 2. Epistemic Subtyping Lattice

We define the subtyping relation $\le$ over types in $\mathcal{T}$:

$$\frac{}{\text{certain } \sigma \le \text{belief}\langle \sigma \rangle} \quad (\text{Certainty Subsumption})$$

$$\frac{}{\tau \le \tau} \quad (\text{Reflexivity})$$

$$\frac{\tau_1 \le \tau_2 \quad \tau_2 \le \tau_3}{\tau_1 \le \tau_3} \quad (\text{Transitivity})$$

### The Non-Coercion Theorem (Fundamental Epistemic Invariant)
$$\forall \sigma \in \text{PrimType}, \quad \text{belief}\langle \sigma \rangle \not\le \text{certain } \sigma$$

*Proof:*
Assume $\text{belief}\langle \sigma \rangle \le \text{certain } \sigma$. Then there exists an implicit conversion $\kappa: \text{BeliefValue} \to \sigma$ such that for any stochastic sample $s \sim P_{\text{model}}(Y|X)$, $\kappa(s)$ evaluates to a grounded mathematical truth without verification. But by definition of stochastic generation, $\exists s$ such that $s \ne y^*$, violating the truth guarantee of $\text{certain } \sigma$. Thus, by contradiction, $\text{belief}\langle \sigma \rangle \not\le \text{certain } \sigma$. $\blacksquare$

---

## 3. Typing Rules (Natural Deduction)

### Model Deliberation (`ask`)
Given model contract $M$ with signature $M: \text{certain String} \to \text{belief}\langle \sigma \rangle$:
$$\frac{\Gamma \vdash e : \text{certain String}}{\Gamma \vdash \text{ask } M(e) : \text{belief}\langle \sigma \rangle}$$

### Epistemic Reduction (`require` / `verify`)
Let $g$ be a guard predicate defined as $g: \text{certain } \sigma \to \text{certain Bool}$:
$$\frac{\Gamma \vdash e_1 : \text{belief}\langle \sigma \rangle \quad g : \text{certain } \sigma \to \text{certain Bool} \quad \Gamma \vdash e_2 : \text{certain } \sigma}{\Gamma \vdash (\text{require } e_1 \text{ with } g \text{ else } e_2) : \text{certain } \sigma}$$

### Quorum Consensus
$$\frac{n \in \mathbb{N}_{\ge 1} \quad \theta \in (0.0, 1.0] \quad \Gamma \vdash e : \text{belief}\langle \sigma \rangle}{\Gamma \vdash \text{quorum}(n, \theta): e : \text{belief}\langle \sigma \rangle}$$

---

## 4. Progress and Preservation (Type Soundness)

### Theorem: Subject Reduction (Preservation)
If $\Gamma \vdash e : \tau$ and $e \to e'$, then $\Gamma \vdash e' : \tau'$ where $\tau' \le \tau$.

### Theorem: Progress
If $e$ is a well-typed closed expression ($\emptyset \vdash e : \tau$), then either $e$ is a value or $\exists e'$ such that $e \to e'$.

### Corollary: Epistemic Taint Freedom
No runtime execution of a well-typed Trell program can transition an unverified model belief into a physical side effect without passing through an explicit guard reduction node.

---

## 5. Cross-References
* Core epistemic design: [[core/epistemic-foundations]]
* Bayesian distribution types: [[theory/bayesian-and-distributional-types]]
* Guard runtime enforcement: [[core/contract-and-guard-system]]
