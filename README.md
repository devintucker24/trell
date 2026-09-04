# Palimpsest: An Epistemic Memory Language for AI Systems

> **Palimpsest** (/ˈpæl.ɪmp.sest/): *A manuscript or piece of writing material on which earlier writing has been effaced or overlaid to make room for later writing, yet traces of the prior text remain indelibly preserved, auditable, and recoverable.*

---

## 1. Thesis: Memory is an Epistemic Resolution & Truth Maintenance Problem

Modern AI agents treat memory as a **vector similarity-search problem**. A user says *"I live in Lisbon"* in March and *"I moved to Berlin"* in August. Both text fragments are embedded and placed into a vector database. When the agent is prompted in September, retrieval pulls top-$k$ nearest chunks. Both chunks enter the prompt context window. The stochastic language model arbitrates between them—sometimes guessing right by date proximity, sometimes hallucinating, never explainably, and incapable of defending its choice.

This is a category error. Retrieval-Augmented Generation (RAG) and vector stores solve **content lookup**, not **epistemic belief state**. 

Compilers solved hierarchical name resolution decades ago: nested scopes, lexical frames, shadowing, and precedence rules. But human cognition and AI agent beliefs possess dimensions that classical programming languages never had:
1. **Epistemic Authority Lattices:** Inner scopes cannot unconditionally shadow outer scopes. A user claiming *"my PTO is 25 days"* in chat cannot override company policy in the HR handbook, even though the user is the narrower and more recent scope. Authority is an epistemic partial order $(A, \le)$ that dominates temporal recency.
2. **First-Class Provenance & Epistemic Types:** A belief without provenance is hearsay. In Palimpsest, belief resolution enforces type-level epistemic contracts (`recall verified path`, `recall min_authority(Policy) path`). If a belief was derived from an unverified source or tainted document, the language's *reduction rules refuse to resolve it*. This is an operational semantic invariant, not a prompt instruction asking nicely.
3. **Temporal Shadowing without Erasure:** New facts shadow older facts without destroying them. The underlying parchment retains every layer. Queries can time-travel (`recall as_of("2026-04-01") path`) or inspect the full audit trace (`audit path`).
4. **Lifetimes & Staleness Degradation:** Facts decay. An IP lease, a weather forecast, or a cache entry has a TTL. When evaluated past expiry, evaluation transitions the binding into a distinct `Stale(value, age, ttl)` constructor. Queries demanding freshness (`recall fresh path`) halt with an epistemic error rather than silently serving obsolete policies.
5. **Truth Maintenance & Cascading Retraction:** When an assumption, source, or episode is retracted (`retract source "phishing_email"`), an active Truth Maintenance System (TMS) dependency graph unrolls all derived beliefs and deterministically falls back to the prior valid belief layer in $O(1)$/$O(k)$ time.
6. **Explicit Contradiction as a First-Class Result:** When two conflicting facts exist at identical authority and temporal priority, Palimpsest refuses to silently flip a coin or average them; it produces a `ContradictionError` that demands arbitration.

### Why This Cannot Be a Library (The PL Test)

> **The Test:** *Name something that lives in your language's evaluation semantics or type system and therefore cannot be provided by an SDK.*

An SDK is an ambient runtime API over data structures: you call `store.get("key")`, receive an arbitrary JSON dictionary or string, and pass it to an LLM prompt. The SDK cannot alter how expressions evaluate, cannot enforce that an unverified binding refuses evaluation at the language boundary, and cannot define lexical-authority scope reduction rules.

In Palimpsest:
1. **Defeasible Reduction Semantics:** The evaluation rule for `recall p` is not a map lookup. It evaluates across an environment $\sigma = \langle \text{Lattice}, \text{ScopeStack}, \text{Time}, \text{TMSGraph} \rangle$. An attempted override by a low-authority assertion does not mutate or replace the existing binding; it creates a shadowed inscription and registers a formal `DefeasanceConflict` in the semantic state.
2. **Epistemic Refusal Semantics:** In expressions like `let key = recall verified secrets.token;`, if the only candidate belief has `authority = Unverified` or `verified = false`, the language evaluation semantics *halts with an Epistemic Refusal*. The value does not exist in the language environment. No downstream expression or prompt template can consume it.
3. **Staleness Monad / Wrapper Type:** A binding evaluated past TTL evaluates to a distinct type constructor `Stale { value, age, ttl }`. Operations expecting `T` fail with a type/staleness error unless explicitly inspected via `.is_stale` or unwrapped.
4. **Lexical Scope Shadowing Bound to Epistemic Lattices:** Resolution walks a 2D matrix of lexical scope frames and authority tiers. Inner lexical depth can shadow *only* within equivalent or lower authority tiers. A higher authority tier in an outer scope strictly dominates an inner scope assertion.

---

## 2. Who Writes Palimpsest?

Palimpsest is designed for three distinct participants in an AI system:

1. **The Agent Controller (Machine Ingest):** As an autonomous agent interacts with users, tools, and environments, it emits declarative Palimpsest statements (`assert`, `episode`, `retract`) instead of stuffing raw chat dumps into a vector database.
2. **The Knowledge & Safety Architect (Human System Designer):** Engineers write Palimpsest programs to declare the foundational rules of the agent's mind: the authority hierarchy (`authority Legal > Compliance > Policy > User > Unverified;`), organizational scopes (`scope enterprise.acme { ... }`), foundational company handbooks, and TTL/provenance constraints.
3. **The Auditor & Evaluator (Inspector / Verifier):** Compliance officers and automated eval suites execute Palimpsest queries (`audit`, `history`, `conflicts`, `recall as_of(...)`) to formally verify what an agent knew at a specific time, audit why a fact was superseded, and prove that untrusted inputs did not override security policies.

---

## 3. The 20-Line Tour

Here is a complete Palimpsest program illustrating the core primitives (`examples/tour.pal`):

```palimpsest
authority Compliance > Policy > User > Unverified;

scope enterprise.acme {
    // Foundational policy with 365-day lifetime
    assert policy.vacation_days = 20 @ authority(Policy), source("hr_handbook_2026"), ttl(365d);
    assert security.tier = "restricted" @ authority(Compliance), source("soc2_audit");

    // Employee claims conflicting vacation days in Slack
    assert policy.vacation_days = 25 @ authority(User), source("slack_chat_942");

    // Chronological profile updates
    assert user.alice.city = "Lisbon" @ authority(User), source("onboarding_doc"), at("2026-03-01T00:00:00Z");
    assert user.alice.city = "Berlin" @ authority(User), source("relocation_ticket"), at("2026-08-15T00:00:00Z");
}

let current_city = recall enterprise.acme.user.alice.city;               // Resolves to "Berlin"
let past_city    = recall as_of("2026-04-01T00:00:00Z") enterprise.acme.user.alice.city; // "Lisbon"
let pto_days     = recall enterprise.acme.policy.vacation_days;          // Resolves to 20 (Policy > User!)
let audit_trail  = audit enterprise.acme.user.alice.city;               // Audits Lisbon (shadowed) & Berlin

retract source "relocation_ticket";                                     // TMS dependency cascade
let restored_city = recall enterprise.acme.user.alice.city;             // Deterministically falls back to "Lisbon"
```

---

## 4. Prior Art & Deep Differentiation

Palimpsest is built from a blank thesis, consciously answering and surpassing five distinct lineages of computer science and AI memory:

```
                       ┌──────────────────────────────────────────────────────────┐
                       │                     PALIMPSEST                           │
                       │  • Declarative Epistemic Memory Language                 │
                       │  • 2D Authority-Lattice × Lexical Scopes                 │
                       │  • First-Class TMS Retraction Cascade & Fallback         │
                       │  • Temporal Shadowing & Inscription Audit Trail          │
                       │  • Episodic Grounding & Staleness Degradation            │
                       │  • Provenance-Gated Evaluation Semantics                 │
                       └─────────────┬───────────────────────────────┬────────────┘
                                     │                               │
       ┌─────────────────────────────┴──────────┐     ┌──────────────┴──────────────────────────┐
       ▼                                        ▼     ▼                                         ▼
 Agent Memory Products                 Temporal Databases      Belief Revision & Logic         AI DSLs
 (Letta, Mem0, LangMem, Graphiti)      (Datomic, Datalog,      (AGM, TMS/ATMS, Defeasible      (DSPy, BAML,
                                        RDF / SPARQL)           Logic, SOAR / ACT-R)            LMQL, Guidance)
```

### 1. Agent Memory Products (Letta/MemGPT, Mem0, LangMem, Zep/Graphiti)
- **Letta / MemGPT:** Letta manages prompt context windows by creating editable "memory blocks" (persona, human, core memory) updated via LLM tool calls (`core_memory_append`, `core_memory_replace`). It has no formal type system, no authority hierarchy, no concept of temporal shadowing (it edits text strings in place), and resolution is simply whatever the LLM reads in the context window.
- **Mem0 & LangMem:** Mem0 embeds unstructured facts into vector stores with user/agent IDs and uses LLM agents to CRUD fact cards. It directly suffers from vector collision: when contradictory facts are stored, both return in top-$k$, leaving resolution to prompt serendipity.
- **Zep / Graphiti:** Graphiti is the closest existing prior art. It builds temporal knowledge graphs with bi-temporal models and edge invalidation.
  - *Where Palimpsest goes beyond Graphiti:* Graphiti is a Python library and database engine. It has no syntax, no lexical scoping, no authority lattice (all extracted facts share the same epistemic tier unless hard-coded in weights), no provenance type system that halts resolution when unverified, and no programming language evaluation model where scopes, inheritance, and deterministic fallbacks are first-class linguistic constructs.

### 2. Databases with Time and Immutable Facts (Datomic, Datalog, RDF/SPARQL, Datascript)
- **Datomic & Datascript:** Datomic models immutable EAVT datoms (Entity, Attribute, Value, Time/Tx) with `:db/add`, `:db/retract`, and `as-of` historical queries.
  - *Where Palimpsest goes beyond Datomic:* Datomic is a database storage engine, not an epistemic programming language. Datomic has no notion of cognitive agent scope, authority dominance (a transaction is a transaction; a user transaction overwrites an admin assertion if written later), staleness degradation curves, or provenance refusal semantics.
- **Datalog:** Declarative logic programming over relational tuples. Pure Datalog is monotonic: adding facts can only derive more facts, making update and retraction impossible without stratified negation, modal state threading, or non-monotonic extensions. Datalog lacks lexical scoping and temporal lifetimes.
- **RDF / SPARQL & Named Graphs:** Quad stores `(subject, predicate, object, graph)` use named graphs as coarse containers for provenance. However, named graphs are uninterpreted identifiers: they do not provide shadowing, authority precedence, or automatic fallback resolution.

### 3. Theory (AGM, TMS/ATMS, Defeasible Logic, ASP, IVM, Cognitive Architectures)
- **AGM Belief Revision (Alchourrón, Gärdenfors, Makinson):** AGM formalizes ideal epistemic expansion ($+$), contraction ($-$), and revision ($*$) over deductively closed propositional belief sets. AGM is computationally intractable in general logics and has no concept of programming language scopes, provenance, execution, or temporal decay.
- **Truth Maintenance Systems (TMS / ATMS - Doyle & de Kleer):** TMS maintains dependency networks between assumptions, justifications, and derived facts, recalculating IN/OUT status upon retraction. Palimpsest directly translates the insights of TMS into language evaluation semantics: bindings track their justification chains, and `retract` operations unravel all downstream beliefs in constant/linear time.
- **Non-Monotonic & Defeasible Logic:** Formalizes defeasible rules, rebuttals, and undercutting defeaters. Palimpsest implements defeasible logic through its **Authority Lattice**: an assertion at authority $A_1$ is defeated by an assertion at $A_2$ if $A_2 > A_1$, producing an auditable `DefeasanceConflict`.
- **SOAR and ACT-R (Cognitive Architectures):** ACT-R separates declarative memory (chunks) from procedural memory (production rules) and features activation decay. Palimpsest adopts a principled position on the **Episodic vs Semantic Memory** split:
  - *Semantic Memory* consists of resolvable named paths in the epistemic lattice (`entity.property`).
  - *Episodic Memory* consists of immutable event logs (`episode <id> { at, actors, context, summary }`).
  - *Grounded Justification:* Semantic facts can be grounded in episodes (`grounded_in("<episode_id>")`). The episode is the premise of the fact. When the episode is retracted or invalidated, all grounded semantic beliefs are automatically unseated.

### 4. Adjacent AI DSLs (DSPy, BAML, LMQL, Guidance)
- **DSPy:** Compiles multi-stage prompt pipelines and optimizes few-shot demonstrations. DSPy is about *prompt engineering and pipeline optimization*, not memory.
- **BAML:** A type-safe schema language for parsing LLM structured outputs into typed objects. BAML is about *output schema extraction*, not belief maintenance.
- **LMQL & Guidance:** Constrained text generation using grammars and token-level logit masks.
- *Where Palimpsest goes:* None of these DSLs manage agent memory, belief state, historical shadowing, authority conflict, or retraction cascades. They are generation formatters; Palimpsest is the epistemic memory substrate.

### Differentiation Matrix

| Capability | Vector RAG | MemGPT / Letta | Graphiti | Datomic | DSPy / BAML | **Palimpsest** |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: |
| **Deterministic Shadowing (no LLM coin-toss)** | No | No | Partial | Yes | N/A | **Yes (Formal PL Semantics)** |
| **Epistemic Authority Lattice** | No | No | No | No | N/A | **Yes (`A > B > C`)** |
| **Auditable Historical Inscriptions (`audit`)** | No | No | Yes | Yes | N/A | **Yes (Full Vellum Trace)** |
| **Cascading TMS Retraction (`retract`)** | No | No | No | Partial | N/A | **Yes (Justification DAG)** |
| **Lifetimes & Staleness Degradation (`ttl`)** | No | No | Partial | No | N/A | **Yes (`Stale` Constructor)** |
| **Type-Level Provenance Refusal (`verified`)** | No | No | No | No | N/A | **Yes (Epistemic Refusal)** |
| **Episodic Grounding of Semantic Facts** | No | No | No | No | N/A | **Yes (`grounded_in`)** |
| **Runs Deterministically Without API Keys** | Yes | No | No | Yes | Yes | **Yes (Pure Rust Core)** |

---

## 5. Formal Semantics: The Palimpsest Resolution Algorithm

When an expression `recall p` is evaluated in runtime state $\sigma$ at evaluation time $T_{\text{eval}}$:

$$\sigma = \langle \mathcal{L}, \mathcal{S}, \mathcal{B}, \mathcal{E}, \mathcal{J}, \mathcal{C}, T_{\text{clock}} \rangle$$

Where:
- $\mathcal{L}$ is the partially ordered Authority Lattice (e.g. $\text{Compliance} \succ \text{Policy} \succ \text{User} \succ \text{Unverified}$).
- $\mathcal{S}$ is the lexical scope stack.
- $\mathcal{B} = \{ b_1, b_2, \dots \}$ is the set of all memory inscriptions.
- $\mathcal{E}$ is the episodic memory store.
- $\mathcal{J}$ is the TMS justification graph (dependencies between sources, episodes, and beliefs).
- $\mathcal{C}$ is the defeasance conflict log.

### Step 1: Candidate Inscription Gathering
The engine finds all inscriptions matching path $p$ (scoped or fully qualified) asserted at or before $T_{\text{eval}}$ that are **not retracted**:

$$\mathcal{B}_{\text{cand}} = \{ b \in \mathcal{B} \mid \text{path}(b) \sim p \land \neg\text{retracted}(b) \land \text{time}(b) \le T_{\text{eval}} \}$$

If $\mathcal{B}_{\text{cand}} = \emptyset$, resolution terminates with `PathNotFoundError`.

### Step 2: Authority Lattice Dominance
Authority strictly dominates temporal recency and lexical depth:

$$A_{\max} = \max_{b \in \mathcal{B}_{\text{cand}}} \text{rank}(\text{authority}(b))$$
$$\mathcal{B}_{\text{auth}} = \{ b \in \mathcal{B}_{\text{cand}} \mid \text{rank}(\text{authority}(b)) = A_{\max} \}$$

Any candidate with authority $A < A_{\max}$ is defeated. If an incoming defeated belief attempted to assert a different value than an existing $A_{\max}$ belief, a `DefeasanceConflict` is recorded in $\mathcal{C}$.

### Step 3: Recency Ordering & Contradiction Detection
Among candidates in $\mathcal{B}_{\text{auth}}$, inscriptions are sorted by asserted timestamp and transaction sequence order:

$$b_{\text{winner}} = \operatorname{argmax}_{b \in \mathcal{B}_{\text{auth}}} (\text{time}(b), \text{tx\_id}(b))$$

If another candidate in $\mathcal{B}_{\text{auth}}$ was asserted with an identical explicit timestamp and conflicting value, resolution halts with `ContradictionError`.

### Step 4: Epistemic Provenance Guard
If the query declares `verified`:
If $\neg\text{verified}(b_{\text{winner}})$ or $\text{authority}(b_{\text{winner}}) = \text{Unverified}$, resolution fails closed:

$$\text{halt with } \text{UnverifiedBeliefRefusal}(p, \text{source}(b_{\text{winner}}))$$

### Step 5: Staleness Degradation
If $b_{\text{winner}}$ specifies an expiry $T_{\text{valid}} = \text{time}(b) + \text{ttl}$:
- If $T_{\text{eval}} > T_{\text{valid}}$ and query demanded `fresh`: halts with `StaleBeliefError`.
- If $T_{\text{eval}} > T_{\text{valid}}$ and standard `recall`: evaluates to `Value::Stale { value, age, ttl }`.
- Otherwise: evaluates directly to `value`.

---

## 6. Demonstrating the 6 Acceptance Scenarios

Every acceptance scenario is fully implemented and verifiable either individually or in batch.

### Running All Scenarios
```bash
cargo run -- --scenarios
```

### Scenario 1: Fact Superseding and Auditability
*A user moves from Lisbon in March to Berlin in August. The current belief resolves to Berlin, but Lisbon remains indelibly in the audit trail rather than silently lost or confusingly retrieved together.*

Program (`examples/01_superseding_and_audit.pal`):
```palimpsest
assert user.residence = "Lisbon" @ authority(User), source("chat_session_03"), at("2026-03-01T10:00:00Z");
assert user.residence = "Berlin" @ authority(User), source("chat_session_08"), at("2026-08-15T14:30:00Z");

let current = recall user.residence;
let past = recall as_of("2026-04-01T00:00:00Z") user.residence;
print history user.residence;
```

Execution Output:
```
>>> Running Palimpsest program: examples/01_superseding_and_audit.pal
"--- Current Belief ---"
"Berlin"
"--- Time-Travel (as of April 2026) ---"
"Lisbon"
"--- Full Epistemic Audit Trail ---"
=== Palimpsest Inscription Audit (2) ===
  [#1] user.residence = "Lisbon" | auth: User | src: "chat_session_03" | ver: true | time: 2026-03-01T10:00:00Z | status: SHADOWED (by #2 at 2026-08-15T14:30:00Z)
  [#2] user.residence = "Berlin" | auth: User | src: "chat_session_08" | ver: true | time: 2026-08-15T14:30:00Z | status: ACTIVE
```

---

### Scenario 2: Low-Authority Cannot Override High-Authority
*A user claims their PTO is 25 days. The employee handbook specifies 20 days. Because `Policy > User`, the user's claim cannot override the policy even though it is newer, and the conflict is reported explicitly.*

Program (`examples/02_authority_lattice_conflict.pal`):
```palimpsest
authority Compliance > Policy > User > Unverified;

assert employee.alice.pto_days = 20 @ authority(Policy), source("hr_handbook_2026"), at("2026-01-01T00:00:00Z");
assert employee.alice.pto_days = 25 @ authority(User), source("slack_chat_942"), at("2026-09-02T11:00:00Z");

print "Recalled PTO:";
print recall employee.alice.pto_days;
print conflicts;
```

Execution Output:
```
>>> Running Palimpsest program: examples/02_authority_lattice_conflict.pal
"--- Recalled PTO (Policy wins over User) ---"
20
"--- Defeasance Conflicts ---"
=== Defeasance Conflicts (1) ===
  [Conflict on 'employee.alice.pto_days']: Low-authority 'User' (source: "slack_chat_942", value: 25) was defeated by existing high-authority 'Policy' (source: "hr_handbook_2026", value: 20). Reason: Attempted override by authority 'User' defeated by established authority 'Policy'
```

---

### Scenario 3: Retracting a Source Unravels Beliefs & Falls Back
*Alice is recorded as 'member' by LDAP. A phishing email claims Alice is 'admin'. Retracting the phishing email cascades through the TMS and deterministically falls back to 'member'.*

Program (`examples/03_truth_maintenance_retract.pal`):
```palimpsest
assert user.alice.role = "member" @ authority(Policy), source("corporate_ldap"), at("2026-01-10T00:00:00Z");
assert user.alice.role = "admin" @ authority(Policy), source("phishing_email_88"), at("2026-09-03T09:00:00Z");

print recall user.alice.role;       // "admin"
retract source "phishing_email_88";
print recall user.alice.role;       // Deterministically falls back to "member"
print audit user.alice.role;
```

Execution Output:
```
>>> Running Palimpsest program: examples/03_truth_maintenance_retract.pal
"--- Prior to Retraction (compromised) ---"
"admin"
"--- Retracting Compromised Source ---"
"--- After Retraction (TMS restored prior state) ---"
"member"
"--- Audit Trail ---"
=== Palimpsest Inscription Audit (2) ===
  [#1] user.alice.role = "member" | auth: Policy | src: "corporate_ldap" | ver: true | time: 2026-01-10T00:00:00Z | status: ACTIVE
  [#2] user.alice.role = "admin" | auth: Policy | src: "phishing_email_88" | ver: true | time: 2026-09-03T09:00:00Z | status: RETRACTED (Retraction of source 'phishing_email_88')
```

---

### Scenario 4: Lifetimes, Staleness, and Expiry
*A DHCP IP lease has a TTL of 300 seconds. After 600 seconds, standard recall returns a `Stale` descriptor object with metadata, and `recall fresh` halts with a `StaleBeliefError`.*

Program (`examples/04_staleness_and_lifetimes.pal`):
```palimpsest
set_time "2026-09-04T12:00:00Z";
assert infra.gateway.ip = "10.0.0.1" @ authority(Policy), source("dhcp_lease"), ttl(300s);

let ip_fresh = recall fresh infra.gateway.ip; // OK
advance_time 600s;
let ip_stale = recall infra.gateway.ip;       // Evaluates to Stale wrapper
print ip_stale;
print audit infra.gateway.ip;
```

Execution Output:
```
>>> Running Palimpsest program: examples/04_staleness_and_lifetimes.pal
"--- At t=0s: Memory is Fresh ---"
"10.0.0.1"
"--- Advancing Time by 10 Minutes (600s) ---"
"--- Standard Recall Reports Stale Wrapper ---"
Stale(value: "10.0.0.1", age: 600s, ttl: 300s)
"--- Audit Trail Showing Expiry ---"
=== Palimpsest Inscription Audit (1) ===
  [#1] infra.gateway.ip = "10.0.0.1" | auth: Policy | src: "dhcp_lease" | ver: true | time: 2026-09-04T12:00:00Z | status: EXPIRED (at 2026-09-04T12:05:00Z)
```

---

### Scenario 5: Provenance Gatekeeping & Language-Enforced Refusal
*An unverified rumor claims an API key. A query requiring verified provenance refuses to resolve it by the language's own evaluation semantics, preventing prompt injection or data poisoning.*

Program (`examples/05_provenance_gatekeeping.pal`):
```palimpsest
assert secrets.auth_token = "tok_untrusted_999" @ authority(Unverified), source("anonymous_paste"), unverified;

print audit secrets.auth_token;
let safe_token = recall verified secrets.auth_token;
```

Execution Output:
```
>>> Running Palimpsest program: examples/05_provenance_gatekeeping.pal
"--- Audit Trail (Shows Inscription is Unverified) ---"
=== Palimpsest Inscription Audit (1) ===
  [#1] secrets.auth_token = "tok_untrusted_999" | auth: Unverified | src: "anonymous_paste" | ver: false | time: 2026-09-04T12:00:00Z | status: ACTIVE

"--- Attempting Verified Recall (Language Halts with Epistemic Refusal) ---"
Execution error: Epistemic Refusal [UnverifiedBelief]: Memory 'secrets.auth_token' from source '"anonymous_paste"' with authority 'Unverified' refused. Query explicitly demanded 'verified', but belief lacks verified provenance or authentic authority.
```

---

### Scenario 6: Grounding Semantic Facts in Episodic Memory
*An episodic event represents an operational outage. Semantic infrastructure state is grounded in that episode. When the incident episode is retracted upon remediation, all grounded beliefs are automatically unseated.*

Program (`examples/06_episodic_grounding_and_retract.pal`):
```palimpsest
episode db_outage_01 {
    at: "2026-09-04T08:15:00Z",
    actors: ["deploy_bot", "alice"],
    context: { service: "billing-db", pool_limit: 100 },
    summary: "Migration aborted: connection pool exhausted during schema update"
}

scope enterprise.acme {
    assert infra.db.status = "degraded" @ authority(Compliance), grounded_in("db_outage_01");
}

print recall enterprise.acme.infra.db.status;  // "degraded"
retract episode db_outage_01;                  // Remediated
print audit enterprise.acme.infra.db.status;   // RETRACTED
```

Execution Output:
```
>>> Running Palimpsest program: examples/06_episodic_grounding_and_retract.pal
"--- Recalled Semantic State Grounded in Episode ---"
"degraded"
"--- Active Episodes in Memory ---"
[{ actors: ["deploy_bot", "alice"], at: 2026-09-04T08:15:00Z, context: { pool_limit: 100, service: "billing-db" }, id: "db_outage_01", summary: "Migration aborted: connection pool exhausted during schema update" }]
"--- Retracting Incident Episode (Root Cause Resolved) ---"
"--- Audit Trail Confirms Invalidation of Grounded Fact ---"
=== Palimpsest Inscription Audit (1) ===
  [#1] enterprise.acme.infra.db.status = "degraded" | auth: Compliance | src: "none" | ver: true | time: 2026-09-04T12:00:00Z | status: RETRACTED (Retraction of episode 'db_outage_01')
```

---

## 7. Language Primitives Reference

### Statements
| Syntax | Semantics |
| :--- | :--- |
| `authority T1 > T2 > ...;` | Declares total ordering / lattice ranks for epistemic authority. |
| `scope prefix.path { ... }` | Introduces lexical namespace frames. |
| `assert path = expr @ modifiers;` | Inscribes a belief into the palimpsest. |
| `episode id { at, actors, context, summary }` | Records an episodic event chunk. |
| `retract source expr;` | Retracts all beliefs derived from the specified source. |
| `retract episode id;` | Retracts an episode and all beliefs grounded in it. |
| `retract belief path;` | Retracts all beliefs on a specific path. |
| `let ident = expr;` | Binds an evaluated expression to a local variable. |
| `print expr;` | Evaluates and prints an expression. |
| `assert_eq expr1, expr2;` | Language-level test assertion. |
| `set_time expr;` | Sets virtual clock time. |
| `advance_time expr;` | Advances virtual clock by duration. |

### Assertion Modifiers (`@ ...`)
| Modifier | Purpose |
| :--- | :--- |
| `authority(Ident)` | Sets the authority tier (e.g. `Policy`, `User`). |
| `source(Expr)` | Provenance tag (document, URL, chat session, API). |
| `verified` / `unverified` | Explicit verification flag. |
| `at(Expr)` | Explicit external assertion timestamp. |
| `ttl(Expr)` | Time-To-Live duration (e.g. `300s`, `24h`, `30d`). |
| `valid_until(Expr)` | Absolute expiration timestamp. |
| `grounded_in(Ident)` | Episode ID grounding this belief. |

### Expressions
| Syntax | Result |
| :--- | :--- |
| `recall path` | Resolves current valid belief. |
| `recall as_of(t) path` | Resolves belief as of timestamp `t`. |
| `recall fresh path` | Resolves belief, halting with `StaleBeliefError` if expired. |
| `recall verified path` | Resolves belief, halting with `UnverifiedBeliefRefusal` if unverified. |
| `recall min_authority(A) path` | Resolves belief requiring at least authority `A`. |
| `history path` / `audit path` | Returns structured audit trace of all inscriptions on `path`. |
| `conflicts` | Returns list of recorded defeasance conflicts. |
| `episodes` | Returns list of active episodes. |

---

## 8. Implementation Details

- **Language & Runtime:** Pure standard-library Rust (Edition 2021). Zero external crate dependencies, ensuring instant compilation, zero supply-chain vulnerabilities, and deterministic cross-platform behavior.
- **Parser Architecture:** Hand-written recursive-descent lexer and parser with precedence-climbing arithmetic and boolean expression evaluation.
- **Memory Engine:** Epistemic Environment tracking scopes, monotonic belief IDs, authority ranks, inverted source and episode indices, justification DAG for TMS cascades, and virtual clock state.
- **Test Suite:** 13 unit and integration tests verifying all 6 acceptance scenarios, nested scoping, contradiction detection, time math, and parser error recovery (`cargo test`).

---

## 9. Status: What is Real vs What is Stubbed

### Real & Fully Working Today:
- Complete lexer, parser, AST, and runtime engine.
- 2D Authority Lattice $\times$ Lexical Scope resolution algorithm.
- First-class TMS cascading retractions for sources, beliefs, and episodes.
- Temporal shadowing and indelible vellum auditing (`audit path`).
- Lifetimes, TTLs, virtual clock advancement, and `Stale` value type.
- Provenance gatekeeping and `UnverifiedBeliefRefusal`.
- Simultaneous equal-authority contradiction detection (`ContradictionError`).
- Episodic memory declaration, listing, and semantic grounding.
- CLI runner with single-file execution and `--scenarios` runner.
- Comprehensive test suite in `tests/acceptance_tests.rs` and `tests/parser_tests.rs`.

### Deliberately Out of Scope for v1:
- **Prose Extraction Pipeline:** We deliberately do not include an LLM inference loop inside the core interpreter. The core language is completely deterministic and requires zero API keys. Extraction from prose into Palimpsest statements (`assert`, `episode`) belongs in an agent controller / compiler frontend that targets Palimpsest.
- **Vector Index Backend:** Palimpsest is the deterministic epistemic resolution layer; while a vector similarity index could be added as an auxiliary fuzzy path index, the core semantics rely on hierarchical path resolution and lattices.
- **Distributed Replication:** Palimpsest runs as an in-process or local agent memory engine. Distributed multi-agent gossip consensus across multiple palimpsests is left for future work.
