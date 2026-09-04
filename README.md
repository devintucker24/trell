# Trell: An Epistemic Programming Language for Speculative Semantic Execution

> **"Traditional languages model Boolean certainty. Trell models probabilistic belief, cognitive invariants, and speculative semantic superpositions."**

Trell is an AI-native programming language designed for orchestrating foundation models, deliberative agents, and mission-critical decision workflows where hallucinations cannot be tolerated.

---

## 1. Why Trell Exists

General-purpose languages (Python, Rust, TypeScript) treat language models as foreign black-box RPC endpoints returning unverified strings. This creates fundamental systemic vulnerabilities:

1. **Epistemic Contamination:** In Python, a hallucinated string from an LLM and a cryptographically verified hash share the exact same type (`str`). The type checker cannot prevent untrusted model outputs from triggering high-stakes side effects.
2. **Sequential Latency Tax:** In agentic pipelines, every branching decision requires multi-second autoregressive generation. Sequential execution forces systems to wait idly on deliberative chains.
3. **Absence of Speculative Semantic Forks:** If an AI makes an assumption that is invalidated downstream, traditional runtimes cannot isolate, track, and roll back the unchosen speculative branches.

Trell introduces **epistemic dual-track types (`certain T` vs `belief<T>`)**, **model contracts with cognitive invariants**, and **speculative branch execution (`fork` / `collapse`)** directly into the core language syntax and runtime.

---

## 2. Core Language Primitives

### 1. Dual-Track Epistemic Types
- `certain T`: Mathematically, statically, or cryptographically grounded values (e.g. deterministic constants, validated schema fields).
- `belief<T>`: Probabilistic values emitted by model oracles, carrying confidence metrics (`0.0` to `1.0`), justification traces, and model lineage.

**The Epistemic Rule:** A `belief<T>` can **never** be assigned to a `certain T` without an explicit epistemic reduction (`verify with <guard>`).

### 2. Model Contracts (`contract`)
Contracts specify cognitive parameters and formal invariants that the model must satisfy at runtime:
```trell
contract DiagnosticOracle {
    model: reasoning;
    temperature: 0.1;
    budget: 2000;
    invariant: confidence >= 0.85;
}
```

### 3. Epistemic Verification Guards (`guard` and `verify`)
Promote a probabilistic belief into grounded certainty by proving a deterministic invariant:
```trell
guard ValidBiomarker(marker: string) {
    marker == "BacterialInfection" || marker == "ViralSyndrome"
}

// Promotes belief<string> to certain string; falls back if the guard fails:
let pathology: certain string = verify diagnosis with ValidBiomarker fallback "Unidentified";
```

### 4. Speculative Semantic Forks (`fork` / `collapse`)
Execute across hypothesis branches speculatively. When deliberation concludes, Trell commits the matching semantic branch and rolls back unchosen speculative paths:
```trell
fork diagnosis {
    case BacterialInfection => {
        print("Action: Initiate targeted antibiotic therapy.");
    }
    case ViralSyndrome => {
        print("Action: Prescribe antiviral supportive care.");
    }
    fallback => {
        print("Action: Escalate to emergency medical board.");
    }
} collapse;
```

### 5. Multi-Sample Semantic Consensus (`consensus`)
Achieve statistical quorum across $N$ independent semantic samples before proceeding:
```trell
let verdict: belief<string> = consensus(3, 0.70) {
    oracle<RiskOracle>.evaluate_risk("Evaluate settlement velocity for $750,000")
};
```

---

## 3. Natural Trell Syntax (Colon + Indent + `end`)

In addition to classic syntax, Trell natively supports **Natural Trell**: a clean, human-readable syntax designed for domain experts, combining Python-like clarity with explicit `end` block safety.

```trell
// Maritime Autonomous Collision Avoidance in Natural Trell
model LookoutAI:
    temperature: 0.1
    budget: 1500
    require: confidence >= 0.85
end

guard ClearWaterway(action: string):
    action == "HoldCourse" or action == "VeerStarboard" or action == "ThrottleDown"
end

action main:
    print "Scanning autonomous maritime radar sector..."

    let obstacle: belief<string> = ask LookoutAI("Container ship bearing 045 relative, range 1.2nm")
    let conf = confidence obstacle

    let safe_action: certain string = require obstacle with ClearWaterway else "ThrottleDown"

    when safe_action is:
        case VeerStarboard:
            print "Helm: Alter course to starboard 15 degrees."
        case ThrottleDown:
            print "Engine: Reversing screw to half astern."
        else:
            print "Helm: Maintain heading and speed."
    end
end
```

---

## 4. Classic Trell Program

```trell
// Medical Diagnosis with Epistemic Verification & Speculative Collapse
contract DiagnosticOracle {
    model: reasoning;
    temperature: 0.1;
    budget: 2000;
    invariant: confidence >= 0.85;
}

guard ValidBiomarker(marker: string) {
    marker == "BacterialInfection" || marker == "ViralSyndrome" || marker == "AutoimmuneFlare"
}

fn main() {
    print("Initiating patient diagnostic assessment...");

    let diagnosis: belief<string> = oracle<DiagnosticOracle>.assess_medical(
        "Patient exhibits leukocytosis, fever, and acute inflammation"
    );

    let conf: certain float = confidence(diagnosis);
    print("Epistemic confidence:");
    print(conf);

    let verified_pathology: certain string = verify diagnosis with ValidBiomarker fallback "Unidentified";

    fork diagnosis {
        case BacterialInfection => {
            print("Action: Administer intravenous antibiotic therapy.");
        }
        case ViralSyndrome => {
            print("Action: Prescribe supportive hydration.");
        }
        fallback => {
            print("Action: Escalate to clinical specialist.");
        }
    } collapse;
}
```

---

## 5. CLI Usage

### Check Epistemic Type Soundness
```bash
trell check examples/medical_diagnosis.trell
```
Guarantees type safety and verifies that ungrounded beliefs cannot escape without proof guards.

### Run with Deliberation & Speculative Execution
```bash
trell run examples/medical_diagnosis.trell
```

### Run with Scenario Configurations (Testing & CI)
```bash
trell run examples/medical_diagnosis.trell --scenario examples/scenarios/risk_alert.json
```

### Inspect Program AST & Epistemic Boundaries
```bash
trell inspect examples/medical_diagnosis.trell
```

### Compile to Trell Executable Package
```bash
trell compile examples/medical_diagnosis.trell -o build/diagnosis.trellc
```

---

## 6. Comprehensive Knowledge Base & Technical Wiki

For complete documentation on the mathematical theory, type calculi, 20-niche industry matrix, competitive landscape, and 10-year evolutionary roadmap, see the **[Trell Technical Wiki](docs/wiki/INDEX.md)**.

**Karpathy LLM Wiki brain (for agents):**
- Schema: [`AGENTS.md`](AGENTS.md) · [`docs/wiki/SCHEMA.md`](docs/wiki/SCHEMA.md)
- **Inbox / triage:** [`docs/wiki/inbox/`](docs/wiki/inbox/README.md) → `skills/wiki/triage` → `skills/wiki/ingest`
- Graph: [`docs/wiki/_meta/GRAPH.yaml`](docs/wiki/_meta/GRAPH.yaml)
- Skills: [`skills/wiki/SKILL.md`](skills/wiki/SKILL.md) (navigate · triage · ingest · query · lint · label · maintain)
- Log: [`docs/wiki/log.md`](docs/wiki/log.md)

* **Core & Syntax**: [Epistemic Foundations](docs/wiki/core/epistemic-foundations.md), [Natural Trell Syntax Specification](docs/wiki/core/natural-syntax-specification.md), [Speculative Execution](docs/wiki/core/speculative-execution-engine.md), [Contracts & Guards](docs/wiki/core/contract-and-guard-system.md)
* **Science & Theory**: [Epistemic Type Calculus & Soundness Proofs](docs/wiki/theory/epistemic-type-calculus.md), [Bayesian & Distributional Types](docs/wiki/theory/bayesian-and-distributional-types.md), [Affine Cognitive Economics](docs/wiki/theory/affine-cognitive-economics.md), [Cryptographic ZK Provenance](docs/wiki/theory/cryptographic-model-provenance.md), [Hardware & Silicon Co-Design](docs/wiki/theory/hardware-silicon-codesign.md)
* **20 Real-World Applications**: [Universal Safety Pattern](docs/wiki/applications/overview-and-safety-patterns.md), [Autonomous Systems & Robotics](docs/wiki/applications/autonomous-physical-systems.md), [Healthcare & Life Sciences](docs/wiki/applications/healthcare-and-life-sciences.md), [Financial Treasury](docs/wiki/applications/financial-treasury-and-markets.md), [Critical Infrastructure](docs/wiki/applications/critical-infrastructure-and-energy.md), [Security & Cloud Governance](docs/wiki/applications/security-cloud-and-governance.md)
* **Market & Roadmap**: [Competitive Landscape](docs/wiki/market/competitive-analysis.md), [Regulatory & Insurance Drivers](docs/wiki/market/regulatory-and-insurance-drivers.md), [Developer Personas](docs/wiki/market/developer-persona-and-adoption.md), [10-Year Strategic Vision](docs/wiki/roadmap/ten-year-vision.md), [Strategic Phases & Milestones](docs/wiki/roadmap/phases-and-milestones.md)

---

## 7. Repository Structure

- `THESIS.md`: The philosophical and architectural thesis for Trell.
- `src/ast.rs`: Abstract Syntax Tree with first-class epistemic and speculative constructs.
- `src/lexer.rs`: Tokenizer for contracts, guards, beliefs, oracles, and fork/collapse syntax.
- `src/parser.rs`: Recursive-descent parser with backwards compatibility for legacy arithmetic.
- `src/typecheck.rs`: Dual-track epistemic type checker preventing unverified belief assignment.
- `src/interpreter.rs`: Runtime execution engine with speculative fork tracking and rollback.
- `src/oracle.rs`: Configurable model oracle interface with scenario support.
- `src/codegen.rs`: Package compiler emitting verified execution units.
- `examples/`:
  - `autonomous_ship.trell`: Maritime obstacle collision avoidance in Natural Trell syntax (colon + indent + end).
  - `bank_transfer.trell`: High-speed interbank wire transfer with statistical quorum in Natural Trell.
  - `medical_diagnosis.trell`: Clinical arbitration and biomarker guard verification.
  - `financial_settlement.trell`: Treasury risk consensus and multi-sig escrow dispatch.
  - `code_synth_guard.trell`: Sandboxed capability enforcement for AI-generated code.
  - `deterministic_math.trell`: Deterministic arithmetic integrated into the type system.
  - `scenarios/`: Mock scenarios simulating normal and adversarial model behaviors.
- `tests/`: Comprehensive test suite verifying epistemic safety and speculative execution.
