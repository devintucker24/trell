#!/usr/bin/env python3
"""Apply YAML frontmatter to Trell wiki pages and sync GRAPH.yaml."""

from __future__ import annotations

import re
from datetime import date
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[3]
WIKI = ROOT / "docs" / "wiki"
GRAPH_PATH = WIKI / "_meta" / "GRAPH.yaml"
TODAY = "2026-09-04"

# Per-page metadata (body unchanged; frontmatter replaced/inserted)
PAGES: dict[str, dict] = {
    "INDEX.md": {
        "id": "wiki-index",
        "title": "Trell Epistemic Language Knowledge Base Index",
        "type": "index",
        "status": "active",
        "tags": ["index", "navigation"],
        "domain": "meta",
        "summary": "Master catalog of the Trell wiki brain for agent navigation.",
        "nodes": [{"id": "wiki-index", "kind": "concept", "label": "Wiki Index"}],
        "edges": [
            {"from": "wiki-index", "to": "belief-type", "rel": "related_to"},
            {"from": "wiki-index", "to": "three-beat-safety-pattern", "rel": "related_to"},
            {"from": "wiki-index", "to": "ten-year-vision", "rel": "related_to"},
        ],
        "related": [
            "[[SCHEMA]]",
            "[[core/epistemic-foundations]]",
            "[[roadmap/ten-year-vision]]",
        ],
        "agent": {
            "priority": "critical",
            "read_when": ["starting any wiki session", "finding a page"],
            "maintain": ["update on every structural page add/remove"],
        },
    },
    "core/epistemic-foundations.md": {
        "id": "epistemic-foundations",
        "title": "Epistemic Foundations of Trell",
        "type": "concept",
        "status": "active",
        "tags": ["epistemic-types", "belief", "certain", "core"],
        "domain": "core",
        "summary": "Dual-track certain T vs belief<T> and the non-coercion rule.",
        "nodes": [
            {"id": "belief-type", "kind": "type", "label": "belief<T>"},
            {"id": "certain-type", "kind": "type", "label": "certain T"},
            {"id": "epistemic-lie", "kind": "concept", "label": "The Epistemic Lie"},
            {"id": "epistemic-contamination", "kind": "concept"},
        ],
        "edges": [
            {"from": "belief-type", "to": "certain-type", "rel": "reduces_via", "note": "via guard verify/require"},
            {"from": "certain-type", "to": "belief-type", "rel": "extends", "note": "certainty subsumption"},
            {"from": "epistemic-lie", "to": "belief-type", "rel": "depends_on"},
        ],
        "related": [
            "[[core/contract-and-guard-system]]",
            "[[theory/epistemic-type-calculus]]",
            "[[core/speculative-execution-engine]]",
        ],
        "implements_code": ["src/ast.rs", "src/typecheck.rs"],
        "agent": {
            "priority": "critical",
            "read_when": ["explaining what Trell is", "type system questions"],
            "maintain": ["sync Non-Coercion rule with typecheck.is_assignable"],
        },
    },
    "core/natural-syntax-specification.md": {
        "id": "natural-syntax-specification",
        "title": "Natural Trell Syntax Specification",
        "type": "concept",
        "status": "active",
        "tags": ["natural-trell", "syntax", "ebnf", "grammar"],
        "domain": "core",
        "summary": "Colon + indent + end grammar, keywords, and EBNF for Natural Trell.",
        "nodes": [
            {"id": "natural-trell-syntax", "kind": "primitive", "label": "Natural Trell"},
            {"id": "keyword-ask", "kind": "primitive"},
            {"id": "keyword-when", "kind": "primitive"},
            {"id": "keyword-end", "kind": "primitive"},
        ],
        "edges": [
            {"from": "natural-trell-syntax", "to": "belief-type", "rel": "depends_on"},
            {"from": "keyword-ask", "to": "belief-type", "rel": "implements"},
            {"from": "keyword-when", "to": "speculative-execution", "rel": "implements"},
        ],
        "related": [
            "[[core/speculative-execution-engine]]",
            "[[core/contract-and-guard-system]]",
        ],
        "implements_code": ["src/lexer.rs", "src/parser.rs"],
        "agent": {
            "priority": "critical",
            "read_when": ["syntax questions", "writing Natural Trell examples"],
            "maintain": ["keep EBNF aligned with parser.rs"],
        },
    },
    "core/speculative-execution-engine.md": {
        "id": "speculative-execution-engine",
        "title": "Speculative Semantic Execution Engine",
        "type": "concept",
        "status": "active",
        "tags": ["speculation", "fork", "when", "rollback"],
        "domain": "core",
        "summary": "Parallel hypothesis branches with transactional rollback and collapse.",
        "nodes": [
            {"id": "speculative-execution", "kind": "engine"},
            {"id": "branch-collapse", "kind": "engine"},
            {"id": "speculative-fork-trace", "kind": "primitive"},
        ],
        "edges": [
            {"from": "speculative-execution", "to": "belief-type", "rel": "depends_on"},
            {"from": "speculative-execution", "to": "tech-npu-semantic-branching", "rel": "accelerates"},
            {"from": "branch-collapse", "to": "speculative-execution", "rel": "depends_on"},
        ],
        "related": [
            "[[theory/hardware-silicon-codesign]]",
            "[[core/natural-syntax-specification]]",
        ],
        "implements_code": ["src/interpreter.rs"],
        "agent": {
            "priority": "high",
            "read_when": ["latency", "fork/when semantics", "hardware co-design"],
            "maintain": ["sync traces with interpreter SpeculativeForkTrace"],
        },
    },
    "core/contract-and-guard-system.md": {
        "id": "contract-and-guard-system",
        "title": "Model Contracts & Verification Guard System",
        "type": "concept",
        "status": "active",
        "tags": ["contracts", "guards", "quorum", "require"],
        "domain": "core",
        "summary": "Model contracts, deterministic guards, require/verify, and quorum consensus.",
        "nodes": [
            {"id": "model-contract", "kind": "primitive"},
            {"id": "guard-verify", "kind": "primitive", "label": "guard / require / verify"},
            {"id": "quorum-consensus", "kind": "primitive"},
        ],
        "edges": [
            {"from": "guard-verify", "to": "belief-type", "rel": "enforces"},
            {"from": "guard-verify", "to": "certain-type", "rel": "reduces_via"},
            {"from": "quorum-consensus", "to": "belief-type", "rel": "extends"},
            {"from": "model-contract", "to": "affine-cognitive-budget", "rel": "depends_on"},
        ],
        "related": [
            "[[core/epistemic-foundations]]",
            "[[theory/epistemic-type-calculus]]",
            "[[theory/affine-cognitive-economics]]",
        ],
        "implements_code": ["src/oracle.rs", "src/interpreter.rs", "src/typecheck.rs"],
        "agent": {
            "priority": "critical",
            "read_when": ["verification", "quorum", "model invariants"],
            "maintain": ["keep contract fields aligned with ModelContract AST"],
        },
    },
    "theory/epistemic-type-calculus.md": {
        "id": "epistemic-type-calculus",
        "title": "Epistemic Type Calculus & Soundness Proofs",
        "type": "concept",
        "status": "active",
        "tags": ["type-theory", "soundness", "formal-methods"],
        "domain": "theory",
        "summary": "Formal typing rules, non-coercion theorem, progress and preservation.",
        "nodes": [
            {"id": "non-coercion-theorem", "kind": "concept"},
            {"id": "subject-reduction", "kind": "concept"},
            {"id": "taint-freedom-corollary", "kind": "concept"},
        ],
        "edges": [
            {"from": "non-coercion-theorem", "to": "belief-type", "rel": "enforces"},
            {"from": "taint-freedom-corollary", "to": "guard-verify", "rel": "depends_on"},
            {"from": "epistemic-type-calculus", "to": "epistemic-foundations", "rel": "extends"},
        ],
        "related": [
            "[[core/epistemic-foundations]]",
            "[[theory/bayesian-and-distributional-types]]",
        ],
        "implements_code": ["src/typecheck.rs"],
        "agent": {
            "priority": "high",
            "read_when": ["formal proofs", "academic exposition"],
            "maintain": ["keep rules matched to typecheck"],
        },
    },
    "theory/bayesian-and-distributional-types.md": {
        "id": "bayesian-and-distributional-types",
        "title": "Bayesian & Distributional Type Systems",
        "type": "concept",
        "status": "active",
        "tags": ["bayesian", "entropy", "distributions", "roadmap"],
        "domain": "theory",
        "summary": "Future distributional belief types, entropy bounds, Bayesian update.",
        "nodes": [
            {"id": "distributional-types", "kind": "type"},
            {"id": "epistemic-entropy-bound", "kind": "concept"},
        ],
        "edges": [
            {"from": "distributional-types", "to": "belief-type", "rel": "extends"},
            {"from": "distributional-types", "to": "phase-4-iso-silicon", "rel": "milestone_of"},
        ],
        "related": [
            "[[theory/epistemic-type-calculus]]",
            "[[roadmap/ten-year-vision]]",
        ],
        "agent": {
            "priority": "medium",
            "read_when": ["future type system", "entropy bounds"],
            "maintain": ["mark clearly as roadmap vs shipped"],
        },
    },
    "theory/affine-cognitive-economics.md": {
        "id": "affine-cognitive-economics",
        "title": "Affine Cognitive Economics & Resource Invariants",
        "type": "concept",
        "status": "active",
        "tags": ["affine-types", "budgets", "tokens", "energy"],
        "domain": "theory",
        "summary": "Linear/affine budgets for tokens, joules, and dollar cost ceilings.",
        "nodes": [
            {"id": "affine-cognitive-budget", "kind": "concept"},
            {"id": "token-budget", "kind": "primitive"},
        ],
        "edges": [
            {"from": "model-contract", "to": "affine-cognitive-budget", "rel": "enforces"},
            {"from": "affine-cognitive-budget", "to": "app-treasury-fedwire", "rel": "applies_to"},
        ],
        "related": [
            "[[core/contract-and-guard-system]]",
            "[[applications/financial-treasury-and-markets]]",
        ],
        "agent": {
            "priority": "medium",
            "read_when": ["cost ceilings", "runaway agents"],
            "maintain": [],
        },
    },
    "theory/cryptographic-model-provenance.md": {
        "id": "cryptographic-model-provenance",
        "title": "Cryptographic Model Provenance & ZK-Epistemic Proofs",
        "type": "concept",
        "status": "active",
        "tags": ["zk-snark", "provenance", "audit"],
        "domain": "theory",
        "summary": "ZK proofs of model lineage and immutable epistemic receipts.",
        "nodes": [
            {"id": "zk-epistemic-proof", "kind": "technology"},
            {"id": "epistemic-receipt", "kind": "primitive"},
        ],
        "edges": [
            {"from": "zk-epistemic-proof", "to": "belief-type", "rel": "extends"},
            {"from": "epistemic-receipt", "to": "reg-eu-ai-act", "rel": "regulated_by"},
        ],
        "related": [
            "[[market/regulatory-and-insurance-drivers]]",
            "[[core/epistemic-foundations]]",
        ],
        "agent": {
            "priority": "medium",
            "read_when": ["audit trails", "regulatory evidence"],
            "maintain": ["roadmap feature — not shipped in v0.2"],
        },
    },
    "theory/hardware-silicon-codesign.md": {
        "id": "hardware-silicon-codesign",
        "title": "Hardware & Silicon Co-Design for Semantic Branching",
        "type": "concept",
        "status": "active",
        "tags": ["npu", "silicon", "speculation", "hardware"],
        "domain": "theory",
        "summary": "NPU/LPU hardware support for speculative semantic forks and rollback.",
        "nodes": [
            {"id": "tech-npu-semantic-branching", "kind": "technology"},
            {"id": "hardware-rollback", "kind": "technology"},
        ],
        "edges": [
            {"from": "tech-npu-semantic-branching", "to": "speculative-execution", "rel": "accelerates"},
            {"from": "tech-npu-semantic-branching", "to": "phase-4-iso-silicon", "rel": "milestone_of"},
        ],
        "related": [
            "[[core/speculative-execution-engine]]",
            "[[roadmap/phases-and-milestones]]",
        ],
        "agent": {
            "priority": "medium",
            "read_when": ["hardware future", "latency zero-collapse"],
            "maintain": [],
        },
    },
    "applications/overview-and-safety-patterns.md": {
        "id": "overview-and-safety-patterns",
        "title": "Universal Epistemic Safety Pattern",
        "type": "application",
        "status": "active",
        "tags": ["pattern", "three-beat", "safety"],
        "domain": "applications",
        "summary": "The three-beat ask → require → when pattern used across all niches.",
        "nodes": [
            {"id": "three-beat-safety-pattern", "kind": "concept"},
        ],
        "edges": [
            {"from": "three-beat-safety-pattern", "to": "ask-deliberation", "rel": "depends_on"},
            {"from": "three-beat-safety-pattern", "to": "guard-verify", "rel": "depends_on"},
            {"from": "three-beat-safety-pattern", "to": "speculative-execution", "rel": "depends_on"},
        ],
        "related": [
            "[[applications/autonomous-physical-systems]]",
            "[[core/epistemic-foundations]]",
        ],
        "agent": {
            "priority": "critical",
            "read_when": ["how Trell is used in industry", "teaching pattern"],
            "maintain": [],
        },
    },
    "applications/autonomous-physical-systems.md": {
        "id": "autonomous-physical-systems",
        "title": "Autonomous Physical Systems & Robotics",
        "type": "application",
        "status": "active",
        "tags": ["maritime", "drones", "mining", "surgery", "nuclear"],
        "domain": "applications",
        "summary": "Ships, drones, haul trucks, surgical robots, and nuclear coolant control.",
        "nodes": [
            {"id": "app-maritime-colregs", "kind": "application"},
            {"id": "app-drone-airspace", "kind": "application"},
            {"id": "app-mining-haulage", "kind": "application"},
            {"id": "app-robotic-surgery", "kind": "application"},
            {"id": "app-nuclear-coolant", "kind": "application"},
            {"id": "example-autonomous-ship", "kind": "example"},
        ],
        "edges": [
            {"from": "three-beat-safety-pattern", "to": "app-maritime-colregs", "rel": "applies_to"},
            {"from": "example-autonomous-ship", "to": "app-maritime-colregs", "rel": "implements"},
            {"from": "app-maritime-colregs", "to": "reg-imo-mass", "rel": "regulated_by"},
        ],
        "related": [
            "[[applications/overview-and-safety-patterns]]",
            "[[theory/hardware-silicon-codesign]]",
        ],
        "implements_code": ["examples/autonomous_ship.trell"],
        "agent": {
            "priority": "high",
            "read_when": ["ships", "robotics", "COLREGs"],
            "maintain": ["keep ship example compiling"],
        },
    },
    "applications/healthcare-and-life-sciences.md": {
        "id": "healthcare-and-life-sciences",
        "title": "Healthcare & Life Sciences Applications",
        "type": "application",
        "status": "active",
        "tags": ["healthcare", "icu", "radiotherapy", "pharmacy", "genomics"],
        "domain": "applications",
        "summary": "ICU sepsis, radiotherapy dosing, pharmacy, genomics, organ allocation.",
        "nodes": [
            {"id": "app-icu-sepsis", "kind": "application"},
            {"id": "app-radiotherapy", "kind": "application"},
            {"id": "app-pharmacy-robot", "kind": "application"},
            {"id": "app-genomic-oncology", "kind": "application"},
            {"id": "app-organ-allocation", "kind": "application"},
        ],
        "edges": [
            {"from": "three-beat-safety-pattern", "to": "app-icu-sepsis", "rel": "applies_to"},
            {"from": "app-icu-sepsis", "to": "reg-fda-samd", "rel": "regulated_by"},
        ],
        "related": [
            "[[applications/overview-and-safety-patterns]]",
            "[[market/regulatory-and-insurance-drivers]]",
        ],
        "implements_code": ["examples/medical_diagnosis.trell"],
        "agent": {
            "priority": "high",
            "read_when": ["healthcare", "clinical AI"],
            "maintain": [],
        },
    },
    "applications/financial-treasury-and-markets.md": {
        "id": "financial-treasury-and-markets",
        "title": "Financial Treasury & Capital Markets",
        "type": "application",
        "status": "active",
        "tags": ["finance", "fedwire", "hft", "insurance", "fx"],
        "domain": "applications",
        "summary": "RTGS settlement, flash-crash defense, claims, sovereign FX.",
        "nodes": [
            {"id": "app-treasury-fedwire", "kind": "application"},
            {"id": "app-market-making", "kind": "application"},
            {"id": "app-insurance-cat", "kind": "application"},
            {"id": "app-sovereign-fx", "kind": "application"},
            {"id": "example-bank-transfer", "kind": "example"},
        ],
        "edges": [
            {"from": "quorum-consensus", "to": "app-treasury-fedwire", "rel": "applies_to"},
            {"from": "example-bank-transfer", "to": "app-treasury-fedwire", "rel": "implements"},
            {"from": "app-treasury-fedwire", "to": "reg-sr-11-7", "rel": "regulated_by"},
        ],
        "related": [
            "[[theory/affine-cognitive-economics]]",
        ],
        "implements_code": ["examples/bank_transfer.trell", "examples/financial_settlement.trell"],
        "agent": {
            "priority": "high",
            "read_when": ["banking", "treasury", "quorum"],
            "maintain": ["keep bank_transfer.trell green"],
        },
    },
    "applications/critical-infrastructure-and-energy.md": {
        "id": "critical-infrastructure-and-energy",
        "title": "Critical Infrastructure & Energy",
        "type": "application",
        "status": "active",
        "tags": ["grid", "water", "rail", "infrastructure"],
        "domain": "applications",
        "summary": "Smart grid frequency, water dosing, high-speed rail interlocking.",
        "nodes": [
            {"id": "app-smart-grid", "kind": "application"},
            {"id": "app-water-treatment", "kind": "application"},
            {"id": "app-highspeed-rail", "kind": "application"},
        ],
        "edges": [
            {"from": "three-beat-safety-pattern", "to": "app-smart-grid", "rel": "applies_to"},
            {"from": "app-smart-grid", "to": "reg-eu-ai-act", "rel": "regulated_by"},
        ],
        "related": [
            "[[applications/overview-and-safety-patterns]]",
            "[[applications/security-cloud-and-governance]]",
        ],
        "agent": {
            "priority": "high",
            "read_when": ["energy", "utilities", "rail"],
            "maintain": [],
        },
    },
    "applications/security-cloud-and-governance.md": {
        "id": "security-cloud-and-governance",
        "title": "Security, Cloud & Governance Applications",
        "type": "application",
        "status": "active",
        "tags": ["security", "iam", "satellites", "aml", "kernel"],
        "domain": "applications",
        "summary": "Kernel hot-patching, IAM synthesis, orbital deconfliction, AML.",
        "nodes": [
            {"id": "app-kernel-hotpatch", "kind": "application"},
            {"id": "app-iam-zero-trust", "kind": "application"},
            {"id": "app-leo-satellites", "kind": "application"},
            {"id": "app-aml-sar", "kind": "application"},
        ],
        "edges": [
            {"from": "three-beat-safety-pattern", "to": "app-kernel-hotpatch", "rel": "applies_to"},
            {"from": "zk-epistemic-proof", "to": "app-aml-sar", "rel": "applies_to"},
        ],
        "related": [
            "[[theory/cryptographic-model-provenance]]",
            "[[market/competitive-analysis]]",
        ],
        "implements_code": ["examples/code_synth_guard.trell"],
        "agent": {
            "priority": "high",
            "read_when": ["security", "cloud IAM", "space"],
            "maintain": [],
        },
    },
    "market/competitive-analysis.md": {
        "id": "competitive-analysis",
        "title": "Competitive Landscape Analysis",
        "type": "market",
        "status": "active",
        "tags": ["market", "langchain", "baml", "weft", "dspy"],
        "domain": "market",
        "summary": "Why SDKs fail epistemic safety; Trell's uncontested authority slice.",
        "nodes": [
            {"id": "comp-langchain", "kind": "competitor"},
            {"id": "comp-langgraph", "kind": "competitor"},
            {"id": "comp-baml", "kind": "competitor"},
            {"id": "comp-weft", "kind": "competitor"},
            {"id": "comp-dspy", "kind": "competitor"},
            {"id": "comp-openai-agents-sdk", "kind": "competitor"},
            {"id": "tech-xgrammar", "kind": "technology"},
        ],
        "edges": [
            {"from": "natural-trell-syntax", "to": "comp-langchain", "rel": "competes_with"},
            {"from": "natural-trell-syntax", "to": "comp-baml", "rel": "competes_with"},
            {"from": "natural-trell-syntax", "to": "comp-weft", "rel": "competes_with"},
            {"from": "tech-xgrammar", "to": "natural-trell-syntax", "rel": "accelerates"},
        ],
        "related": [
            "[[market/regulatory-and-insurance-drivers]]",
            "[[market/developer-persona-and-adoption]]",
        ],
        "agent": {
            "priority": "high",
            "read_when": ["competition", "positioning", "why not a library"],
            "maintain": ["refresh competitor claims carefully with sources"],
        },
    },
    "market/regulatory-and-insurance-drivers.md": {
        "id": "regulatory-and-insurance-drivers",
        "title": "Regulatory Drivers & Insurance Mandates",
        "type": "market",
        "status": "active",
        "tags": ["regulation", "eu-ai-act", "imo", "insurance"],
        "domain": "market",
        "summary": "EU AI Act, IMO MASS, SR 11-7, and insurance epistemic bounding.",
        "nodes": [
            {"id": "reg-eu-ai-act", "kind": "regulation"},
            {"id": "reg-imo-mass", "kind": "regulation"},
            {"id": "reg-sr-11-7", "kind": "regulation"},
            {"id": "reg-fda-samd", "kind": "regulation"},
            {"id": "insurance-epistemic-bound", "kind": "concept"},
        ],
        "edges": [
            {"from": "app-maritime-colregs", "to": "reg-imo-mass", "rel": "regulated_by"},
            {"from": "guard-verify", "to": "reg-eu-ai-act", "rel": "implements"},
            {"from": "insurance-epistemic-bound", "to": "epistemic-receipt", "rel": "depends_on"},
        ],
        "related": [
            "[[market/competitive-analysis]]",
            "[[theory/cryptographic-model-provenance]]",
        ],
        "agent": {
            "priority": "high",
            "read_when": ["compliance", "insurance", "why enterprises adopt"],
            "maintain": [],
        },
    },
    "market/developer-persona-and-adoption.md": {
        "id": "developer-persona-and-adoption",
        "title": "Developer Personas & Adoption Dynamics",
        "type": "market",
        "status": "active",
        "tags": ["personas", "adoption", "agents", "domain-experts"],
        "domain": "market",
        "summary": "AI agents, safety engineers, domain specialists, and auditors as users.",
        "nodes": [
            {"id": "persona-deliberative-agent", "kind": "persona"},
            {"id": "persona-safety-engineer", "kind": "persona"},
            {"id": "persona-domain-specialist", "kind": "persona"},
            {"id": "persona-auditor", "kind": "persona"},
        ],
        "edges": [
            {"from": "natural-trell-syntax", "to": "persona-deliberative-agent", "rel": "owned_by"},
            {"from": "natural-trell-syntax", "to": "persona-domain-specialist", "rel": "owned_by"},
            {"from": "non-coercion-theorem", "to": "persona-safety-engineer", "rel": "owned_by"},
        ],
        "related": [
            "[[core/natural-syntax-specification]]",
            "[[roadmap/ten-year-vision]]",
        ],
        "agent": {
            "priority": "medium",
            "read_when": ["who writes Trell", "GTM"],
            "maintain": [],
        },
    },
    "roadmap/ten-year-vision.md": {
        "id": "ten-year-vision",
        "title": "Ten-Year Vision (2026–2036)",
        "type": "roadmap",
        "status": "active",
        "tags": ["roadmap", "vision", "2036"],
        "domain": "roadmap",
        "summary": "Trell as the epistemic layer between models and actuators by 2036.",
        "nodes": [
            {"id": "ten-year-vision", "kind": "concept"},
            {"id": "epistemic-liability-era", "kind": "concept"},
        ],
        "edges": [
            {"from": "phase-1-beachhead", "to": "ten-year-vision", "rel": "milestone_of"},
            {"from": "phase-4-iso-silicon", "to": "ten-year-vision", "rel": "milestone_of"},
        ],
        "related": [
            "[[roadmap/phases-and-milestones]]",
            "[[theory/hardware-silicon-codesign]]",
        ],
        "agent": {
            "priority": "high",
            "read_when": ["future of Trell", "strategic narrative"],
            "maintain": [],
        },
    },
    "roadmap/phases-and-milestones.md": {
        "id": "phases-and-milestones",
        "title": "Strategic Phases & Milestones",
        "type": "roadmap",
        "status": "active",
        "tags": ["phases", "milestones", "execution"],
        "domain": "roadmap",
        "summary": "Phase 1 niche → Phase 2 LSP/codegen → Phase 3 AOT/WASM → Phase 4 ISO/silicon.",
        "nodes": [
            {"id": "phase-1-beachhead", "kind": "phase"},
            {"id": "phase-2-lsp-codegen", "kind": "phase"},
            {"id": "phase-3-aot-wasm", "kind": "phase"},
            {"id": "phase-4-iso-silicon", "kind": "phase"},
        ],
        "edges": [
            {"from": "phase-1-beachhead", "to": "app-maritime-colregs", "rel": "applies_to"},
            {"from": "phase-2-lsp-codegen", "to": "tech-xgrammar", "rel": "depends_on"},
            {"from": "phase-3-aot-wasm", "to": "tech-wasmtime", "rel": "depends_on"},
            {"from": "phase-4-iso-silicon", "to": "tech-npu-semantic-branching", "rel": "depends_on"},
        ],
        "related": [
            "[[roadmap/ten-year-vision]]",
            "[[market/competitive-analysis]]",
        ],
        "agent": {
            "priority": "high",
            "read_when": ["planning work", "milestones"],
            "maintain": ["update when phase goals complete"],
        },
    },
}

# Extra nodes only in GRAPH (technologies referenced by edges)
EXTRA_NODES = [
    {"id": "ask-deliberation", "kind": "primitive", "page": "core/contract-and-guard-system", "label": "ask"},
    {"id": "tech-wasmtime", "kind": "technology", "page": "roadmap/phases-and-milestones", "label": "Wasmtime/WASI"},
    {"id": "agents-md", "kind": "concept", "page": None, "label": "AGENTS.md schema"},
    {"id": "epistemic-type-calculus", "kind": "concept", "page": "theory/epistemic-type-calculus"},
]


def strip_existing_frontmatter(text: str) -> str:
    if text.startswith("---\n"):
        end = text.find("\n---\n", 4)
        if end != -1:
            return text[end + 5 :]
        # handle ---\r\n variants
        end = text.find("\n---\r\n", 4)
        if end != -1:
            return text[end + 6 :]
    return text


def dump_frontmatter(meta: dict) -> str:
    payload = {
        "id": meta["id"],
        "title": meta["title"],
        "type": meta["type"],
        "status": meta.get("status", "active"),
        "created": meta.get("created", TODAY),
        "updated": TODAY,
        "tags": meta.get("tags", []),
        "domain": meta["domain"],
        "summary": meta["summary"],
        "nodes": meta.get("nodes", []),
        "edges": meta.get("edges", []),
        "related": meta.get("related", []),
    }
    if meta.get("implements_code"):
        payload["implements_code"] = meta["implements_code"]
    payload["agent"] = meta.get("agent", {"priority": "medium", "read_when": [], "maintain": []})
    body = yaml.safe_dump(payload, sort_keys=False, allow_unicode=True)
    return f"---\n{body}---\n\n"


def apply_pages() -> None:
    for rel, meta in PAGES.items():
        path = WIKI / rel
        if not path.exists():
            print(f"SKIP missing {rel}")
            continue
        original = path.read_text(encoding="utf-8")
        body = strip_existing_frontmatter(original).lstrip("\n")
        path.write_text(dump_frontmatter(meta) + body, encoding="utf-8")
        print(f"OK {rel}")


def sync_graph() -> None:
    nodes: dict[str, dict] = {}
    edges: list[dict] = []
    seen_edges: set[tuple] = set()

    for rel, meta in PAGES.items():
        page = rel[:-3] if rel.endswith(".md") else rel
        if page == "INDEX":
            page = "INDEX"
        for n in meta.get("nodes", []):
            nodes[n["id"]] = {
                "id": n["id"],
                "kind": n.get("kind", "concept"),
                "page": page,
                "label": n.get("label", n["id"]),
            }
        for e in meta.get("edges", []):
            key = (e["from"], e["to"], e["rel"])
            if key in seen_edges:
                continue
            seen_edges.add(key)
            edges.append(
                {
                    "from": e["from"],
                    "to": e["to"],
                    "rel": e["rel"],
                    "page": page,
                    **({"note": e["note"]} if e.get("note") else {}),
                }
            )

    for n in EXTRA_NODES:
        if n["id"] not in nodes:
            nodes[n["id"]] = n

    # Ensure edge endpoints exist as nodes
    for e in edges:
        for endpoint in (e["from"], e["to"]):
            if endpoint not in nodes:
                nodes[endpoint] = {
                    "id": endpoint,
                    "kind": "concept",
                    "page": None,
                    "label": endpoint,
                }

    graph = {
        "version": 1,
        "updated": TODAY,
        "description": "Trell epistemic wiki knowledge graph — nodes and typed edges for agents",
        "nodes": sorted(nodes.values(), key=lambda x: x["id"]),
        "edges": sorted(edges, key=lambda x: (x["from"], x["to"], x["rel"])),
    }
    GRAPH_PATH.parent.mkdir(parents=True, exist_ok=True)
    GRAPH_PATH.write_text(
        yaml.safe_dump(graph, sort_keys=False, allow_unicode=True),
        encoding="utf-8",
    )
    print(f"GRAPH {len(graph['nodes'])} nodes, {len(graph['edges'])} edges → {GRAPH_PATH}")


def main() -> None:
    apply_pages()
    sync_graph()


if __name__ == "__main__":
    main()
