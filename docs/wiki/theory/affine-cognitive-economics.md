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
