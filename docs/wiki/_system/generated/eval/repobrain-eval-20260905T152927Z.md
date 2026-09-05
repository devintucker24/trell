# RepoBrain baseline evaluation — 2026-09-05 15:29:27 UTC

**Overall:** PASS

Commit: `aaf47a2cf0bf07f62c549a6989f6dc4fd3697321`

| Category | Required | Result | Summary |
|---|---:|---:|---|
| `structural-health` | yes | PASS | Doctor score 100.0/100 with 0 blocking finding(s). |
| `golden-retrieval` | yes | PASS | 7/7 golden queries passed. |
| `answer-fidelity` | yes | PASS | 3/3 retrieval-only evidence checks passed. |
| `graphify` | yes | PASS | Graphify resolved Parser to parser.rs; freshness=fresh. |
| `context-budgets` | yes | PASS | Tier-0 1796/2000; max retrieved 1197/3500; combined 2993/9500 estimated tokens. |
| `setup-fixture` | yes | PASS | Realistic Git fixture passed setup non-overwrite and no-semantic-copy safety checks. |

## structural-health

Doctor score 100.0/100 with 0 blocking finding(s).

### Evidence

```json
[
  {
    "command": "./repobrain doctor --no-log",
    "exit_code": 0,
    "stdout": "Doctor score 100.0/100\nWrote ./docs/wiki/_system/generated/doctor/doctor-2026-09-05.md\nWrote ./docs/wiki/_system/generated/doctor/latest.json\nCounts: {}\nHeal recommended: False",
    "stderr": ""
  },
  {
    "score": 100.0,
    "counts": {},
    "pages_scanned": 32,
    "blocking_findings": []
  }
]
```

## golden-retrieval

7/7 golden queries passed.

### Evidence

```json
[
  {
    "id": "maritime-safety",
    "passed": true,
    "command": "./repobrain retrieve \"How does Trell keep autonomous ships safe under COLREGs?\" --k 8 --budget-tokens 3500 --lane all --json --no-log",
    "top_k": 8,
    "packed_tokens": 1105,
    "budget_tokens": 3500,
    "source_checks": [
      {
        "path": "applications/autonomous-physical-systems.md",
        "max_rank": 3,
        "minimum_score_class": "strong",
        "minimum_score": 0.45,
        "expected_provenance": "compiled",
        "found": true,
        "rank": 1,
        "score": 0.8611,
        "provenance": "compiled"
      },
      {
        "path": "applications/overview-and-safety-patterns.md",
        "max_rank": 8,
        "minimum_score_class": "strong",
        "minimum_score": 0.45,
        "expected_provenance": "compiled",
        "found": true,
        "rank": 4,
        "score": 0.7417,
        "provenance": "compiled"
      },
      {
        "path": "core/epistemic-foundations.md",
        "max_rank": 8,
        "minimum_score_class": "strong",
        "minimum_score": 0.45,
        "expected_provenance": "compiled",
        "found": true,
        "rank": 2,
        "score": 0.8283,
        "provenance": "compiled"
      }
    ],
    "conflicts": [],
    "ranked_paths": [
      {
        "rank": 1,
        "path": "applications/autonomous-physical-systems.md",
        "score": 0.8611,
        "provenance": "compiled"
      },
      {
        "rank": 2,
        "path": "core/epistemic-foundations.md",
        "score": 0.8283,
        "provenance": "compiled"
      },
      {
        "rank": 3,
        "path": "core/natural-syntax-specification.md",
        "score": 0.695,
        "provenance": "compiled"
      },
      {
        "rank": 4,
        "path": "applications/overview-and-safety-patterns.md",
        "score": 0.7417,
        "provenance": "compiled"
      },
      {
        "rank": 5,
        "path": "market/competitive-analysis.md",
        "score": 0.586,
        "provenance": "compiled"
      },
      {
        "rank": 6,
        "path": "INDEX.md",
        "score": 0.5548,
        "provenance": "meta"
      },
      {
        "rank": 7,
        "path": "raw/examples.md",
        "score": 0.5466,
        "provenance": "raw-pointer"
      },
      {
        "rank": 8,
        "path": "applications/security-cloud-and-governance.md",
        "score": 0.5369,
        "provenance": "compiled"
      }
    ],
    "failures": []
  },
  {
    "id": "belief-certain",
    "passed": true,
    "command": "./repobrain retrieve \"What is belief vs certain and how does verify work?\" --k 8 --budget-tokens 3500 --lane all --json --no-log",
    "top_k": 8,
    "packed_tokens": 1094,
    "budget_tokens": 3500,
    "source_checks": [
      {
        "path": "core/epistemic-foundations.md",
        "max_rank": 3,
        "minimum_score_class": "strong",
        "minimum_score": 0.45,
        "expected_provenance": "compiled",
        "found": true,
        "rank": 1,
        "score": 0.8438,
        "provenance": "compiled"
      },
      {
        "path": "core/contract-and-guard-system.md",
        "max_rank": 5,
        "minimum_score_class": "strong",
        "minimum_score": 0.45,
        "expected_provenance": "compiled",
        "found": true,
        "rank": 2,
        "score": 0.68,
        "provenance": "compiled"
      }
    ],
    "conflicts": [],
    "ranked_paths": [
      {
        "rank": 1,
        "path": "core/epistemic-foundations.md",
        "score": 0.8438,
        "provenance": "compiled"
      },
      {
        "rank": 2,
        "path": "core/contract-and-guard-system.md",
        "score": 0.68,
        "provenance": "compiled"
      },
      {
        "rank": 3,
        "path": "theory/epistemic-type-calculus.md",
        "score": 0.6436,
        "provenance": "compiled"
      },
      {
        "rank": 4,
        "path": "theory/cryptographic-model-provenance.md",
        "score": 0.4434,
        "provenance": "compiled"
      },
      {
        "rank": 5,
        "path": "market/competitive-analysis.md",
        "score": 0.4338,
        "provenance": "compiled"
      },
      {
        "rank": 6,
        "path": "theory/bayesian-and-distributional-types.md",
        "score": 0.4238,
        "provenance": "compiled"
      },
      {
        "rank": 7,
        "path": "core/natural-syntax-specification.md",
        "score": 0.394,
        "provenance": "compiled"
      },
      {
        "rank": 8,
        "path": "roadmap/phases-and-milestones.md",
        "score": 0.3924,
        "provenance": "compiled"
      }
    ],
    "failures": []
  },
  {
    "id": "natural-syntax",
    "passed": true,
    "command": "./repobrain retrieve \"Natural Trell colon indent end syntax\" --k 8 --budget-tokens 3500 --lane all --json --no-log",
    "top_k": 8,
    "packed_tokens": 1174,
    "budget_tokens": 3500,
    "source_checks": [
      {
        "path": "core/natural-syntax-specification.md",
        "max_rank": 3,
        "minimum_score_class": "strong",
        "minimum_score": 0.45,
        "expected_provenance": "compiled",
        "found": true,
        "rank": 1,
        "score": 0.8738,
        "provenance": "compiled"
      }
    ],
    "conflicts": [],
    "ranked_paths": [
      {
        "rank": 1,
        "path": "core/natural-syntax-specification.md",
        "score": 0.8738,
        "provenance": "compiled"
      },
      {
        "rank": 2,
        "path": "core/contract-and-guard-system.md",
        "score": 0.7458,
        "provenance": "compiled"
      },
      {
        "rank": 3,
        "path": "market/developer-persona-and-adoption.md",
        "score": 0.7027,
        "provenance": "compiled"
      },
      {
        "rank": 4,
        "path": "INDEX.md",
        "score": 0.5834,
        "provenance": "meta"
      },
      {
        "rank": 5,
        "path": "core/natural-syntax-specification.md",
        "score": 0.8738,
        "provenance": "compiled"
      },
      {
        "rank": 6,
        "path": "core/natural-syntax-specification.md",
        "score": 0.8738,
        "provenance": "compiled"
      },
      {
        "rank": 7,
        "path": "core/natural-syntax-specification.md",
        "score": 0.8738,
        "provenance": "compiled"
      },
      {
        "rank": 8,
        "path": "core/natural-syntax-specification.md",
        "score": 0.8738,
        "provenance": "compiled"
      }
    ],
    "failures": []
  },
  {
    "id": "competitors",
    "passed": true,
    "command": "./repobrain retrieve \"Why not LangChain or BAML for epistemic safety?\" --k 8 --budget-tokens 3500 --lane all --json --no-log",
    "top_k": 8,
    "packed_tokens": 1197,
    "budget_tokens": 3500,
    "source_checks": [
      {
        "path": "market/competitive-analysis.md",
        "max_rank": 3,
        "minimum_score_class": "strong",
        "minimum_score": 0.45,
        "expected_provenance": "compiled",
        "found": true,
        "rank": 1,
        "score": 0.8411,
        "provenance": "compiled"
      }
    ],
    "conflicts": [],
    "ranked_paths": [
      {
        "rank": 1,
        "path": "market/competitive-analysis.md",
        "score": 0.8411,
        "provenance": "compiled"
      },
      {
        "rank": 2,
        "path": "applications/overview-and-safety-patterns.md",
        "score": 0.7392,
        "provenance": "compiled"
      },
      {
        "rank": 3,
        "path": "core/epistemic-foundations.md",
        "score": 0.7044,
        "provenance": "compiled"
      },
      {
        "rank": 4,
        "path": "theory/epistemic-type-calculus.md",
        "score": 0.5,
        "provenance": "compiled"
      },
      {
        "rank": 5,
        "path": "INDEX.md",
        "score": 0.4891,
        "provenance": "meta"
      },
      {
        "rank": 6,
        "path": "market/competitive-analysis.md",
        "score": 0.8411,
        "provenance": "compiled"
      },
      {
        "rank": 7,
        "path": "market/competitive-analysis.md",
        "score": 0.8411,
        "provenance": "compiled"
      },
      {
        "rank": 8,
        "path": "market/competitive-analysis.md",
        "score": 0.8411,
        "provenance": "compiled"
      }
    ],
    "failures": []
  },
  {
    "id": "ten-year",
    "passed": true,
    "command": "./repobrain retrieve \"What is the ten year vision for Trell?\" --k 8 --budget-tokens 3500 --lane all --json --no-log",
    "top_k": 8,
    "packed_tokens": 998,
    "budget_tokens": 3500,
    "source_checks": [
      {
        "path": "roadmap/ten-year-vision.md",
        "max_rank": 3,
        "minimum_score_class": "strong",
        "minimum_score": 0.45,
        "expected_provenance": "compiled",
        "found": true,
        "rank": 1,
        "score": 0.8411,
        "provenance": "compiled"
      }
    ],
    "conflicts": [],
    "ranked_paths": [
      {
        "rank": 1,
        "path": "roadmap/ten-year-vision.md",
        "score": 0.8411,
        "provenance": "compiled"
      },
      {
        "rank": 2,
        "path": "INDEX.md",
        "score": 0.7965,
        "provenance": "meta"
      },
      {
        "rank": 3,
        "path": "market/competitive-analysis.md",
        "score": 0.6624,
        "provenance": "compiled"
      },
      {
        "rank": 4,
        "path": "market/developer-persona-and-adoption.md",
        "score": 0.63,
        "provenance": "compiled"
      },
      {
        "rank": 5,
        "path": "core/natural-syntax-specification.md",
        "score": 0.6186,
        "provenance": "compiled"
      },
      {
        "rank": 6,
        "path": "core/speculative-execution-engine.md",
        "score": 0.5264,
        "provenance": "compiled"
      },
      {
        "rank": 7,
        "path": "core/contract-and-guard-system.md",
        "score": 0.4795,
        "provenance": "compiled"
      },
      {
        "rank": 8,
        "path": "roadmap/ten-year-vision.md",
        "score": 0.8411,
        "provenance": "compiled"
      }
    ],
    "failures": []
  },
  {
    "id": "episodic-memory-decision",
    "passed": true,
    "command": "./repobrain retrieve \"What did we decide about wiki memory and RAG?\" --k 8 --budget-tokens 3500 --lane episodic --json --no-log",
    "top_k": 8,
    "packed_tokens": 623,
    "budget_tokens": 3500,
    "source_checks": [
      {
        "path": "episodic/2026-09-04-brain-memory-upgrade.md",
        "max_rank": 3,
        "minimum_score_class": "strong",
        "minimum_score": 0.45,
        "expected_provenance": "episodic",
        "found": true,
        "rank": 1,
        "score": 0.8555,
        "provenance": "episodic"
      }
    ],
    "conflicts": [],
    "ranked_paths": [
      {
        "rank": 1,
        "path": "episodic/2026-09-04-brain-memory-upgrade.md",
        "score": 0.8555,
        "provenance": "episodic"
      },
      {
        "rank": 2,
        "path": "episodic/2026-09-04-brain-memory-upgrade.md",
        "score": 0.8555,
        "provenance": "episodic"
      },
      {
        "rank": 3,
        "path": "episodic/2026-09-04-brain-memory-upgrade.md",
        "score": 0.8555,
        "provenance": "episodic"
      },
      {
        "rank": 4,
        "path": "episodic/2026-09-04-brain-memory-upgrade.md",
        "score": 0.8555,
        "provenance": "episodic"
      },
      {
        "rank": 5,
        "path": "episodic/2026-09-04-brain-memory-upgrade.md",
        "score": 0.8555,
        "provenance": "episodic"
      },
      {
        "rank": 6,
        "path": "episodic/2026-09-04-brain-memory-upgrade.md",
        "score": 0.8555,
        "provenance": "episodic"
      },
      {
        "rank": 7,
        "path": "episodic/INDEX.md",
        "score": 0.581,
        "provenance": "episodic"
      },
      {
        "rank": 8,
        "path": "episodic/session-current.md",
        "score": 0.4658,
        "provenance": "episodic"
      }
    ],
    "failures": []
  },
  {
    "id": "temporal-timeline",
    "passed": true,
    "command": "./repobrain retrieve \"What changed in the wiki brain timeline?\" --k 8 --budget-tokens 3500 --lane temporal --json --no-log",
    "top_k": 8,
    "packed_tokens": 453,
    "budget_tokens": 3500,
    "source_checks": [
      {
        "path": "temporal/TIMELINE.md",
        "max_rank": 3,
        "minimum_score_class": "strong",
        "minimum_score": 0.45,
        "expected_provenance": "temporal",
        "found": true,
        "rank": 1,
        "score": 0.8099,
        "provenance": "temporal"
      }
    ],
    "conflicts": [],
    "ranked_paths": [
      {
        "rank": 1,
        "path": "temporal/TIMELINE.md",
        "score": 0.8099,
        "provenance": "temporal"
      },
      {
        "rank": 2,
        "path": "temporal/TIMELINE.md",
        "score": 0.7488,
        "provenance": "temporal"
      },
      {
        "rank": 3,
        "path": "temporal/TIMELINE.md",
        "score": 0.6724,
        "provenance": "temporal"
      }
    ],
    "failures": []
  }
]
```

## answer-fidelity

3/3 retrieval-only evidence checks passed.

### Evidence

```json
[
  {
    "id": "belief-reduction",
    "query_id": "belief-certain",
    "passed": true,
    "mode": "deterministic retrieval-only evidence",
    "context_manifest": [
      {
        "kind": "tier-0",
        "path": "AGENTS.md"
      },
      {
        "kind": "tier-0",
        "path": "docs/wiki/_system/docs/ROUTER.md"
      },
      {
        "kind": "retrieved-excerpt",
        "path": "core/epistemic-foundations.md",
        "anchor": "a-guard-verification-verify-require",
        "characters": 500
      },
      {
        "kind": "retrieved-excerpt",
        "path": "core/contract-and-guard-system.md",
        "anchor": "3-the-epistemic-reduction-construct-require-verify",
        "characters": 207
      },
      {
        "kind": "retrieved-excerpt",
        "path": "theory/epistemic-type-calculus.md",
        "anchor": "epistemic-reduction-require-verify",
        "characters": 361
      },
      {
        "kind": "retrieved-excerpt",
        "path": "theory/cryptographic-model-provenance.md",
        "anchor": "2-zero-knowledge-epistemic-proofs-zk-ep",
        "characters": 500
      },
      {
        "kind": "retrieved-excerpt",
        "path": "market/competitive-analysis.md",
        "anchor": "3-the-uncontested-market-slice",
        "characters": 500
      },
      {
        "kind": "retrieved-excerpt",
        "path": "theory/bayesian-and-distributional-types.md",
        "anchor": "1-beyond-scalar-confidence",
        "characters": 392
      },
      {
        "kind": "retrieved-excerpt",
        "path": "core/natural-syntax-specification.md",
        "anchor": "2-keywords-reserved-words",
        "characters": 499
      },
      {
        "kind": "retrieved-excerpt",
        "path": "roadmap/phases-and-milestones.md",
        "anchor": "roadmap-strategic-phases-milestones",
        "characters": 145
      }
    ],
    "required_term_checks": [
      {
        "requirement": "belief",
        "present": true
      },
      {
        "requirement": "certain",
        "present": true
      },
      {
        "requirement": [
          "guard",
          "predicate"
        ],
        "present": true
      },
      {
        "requirement": [
          "verify",
          "require"
        ],
        "present": true
      }
    ],
    "required_citation_checks": [
      {
        "path": "core/epistemic-foundations.md",
        "present": true
      },
      {
        "path": "core/contract-and-guard-system.md",
        "present": true
      }
    ],
    "forbidden_paths": [
      "INDEX.md",
      "SCHEMA.md"
    ],
    "full_corpus_reads": 0,
    "failures": []
  },
  {
    "id": "natural-syntax-shape",
    "query_id": "natural-syntax",
    "passed": true,
    "mode": "deterministic retrieval-only evidence",
    "context_manifest": [
      {
        "kind": "tier-0",
        "path": "AGENTS.md"
      },
      {
        "kind": "tier-0",
        "path": "docs/wiki/_system/docs/ROUTER.md"
      },
      {
        "kind": "retrieved-excerpt",
        "path": "core/natural-syntax-specification.md",
        "anchor": "core-natural-trell-syntax-specification",
        "characters": 324
      },
      {
        "kind": "retrieved-excerpt",
        "path": "core/contract-and-guard-system.md",
        "anchor": "natural-trell-syntax",
        "characters": 110
      },
      {
        "kind": "retrieved-excerpt",
        "path": "market/developer-persona-and-adoption.md",
        "anchor": "why-models-love-writing-natural-trell",
        "characters": 499
      },
      {
        "kind": "retrieved-excerpt",
        "path": "core/natural-syntax-specification.md",
        "anchor": "1-syntax-philosophy-colon-indentation-explicit-end",
        "characters": 500
      },
      {
        "kind": "retrieved-excerpt",
        "path": "core/natural-syntax-specification.md",
        "anchor": "2-keywords-reserved-words",
        "characters": 499
      },
      {
        "kind": "retrieved-excerpt",
        "path": "core/natural-syntax-specification.md",
        "anchor": "3-formal-ebnf-grammar",
        "characters": 497
      },
      {
        "kind": "retrieved-excerpt",
        "path": "core/natural-syntax-specification.md",
        "anchor": "4-comprehensive-natural-trell-canonical-example",
        "characters": 499
      }
    ],
    "required_term_checks": [
      {
        "requirement": "Natural Trell",
        "present": true
      },
      {
        "requirement": "end",
        "present": true
      },
      {
        "requirement": "syntax",
        "present": true
      }
    ],
    "required_citation_checks": [
      {
        "path": "core/natural-syntax-specification.md",
        "present": true
      }
    ],
    "forbidden_paths": [
      "INDEX.md",
      "SCHEMA.md"
    ],
    "full_corpus_reads": 0,
    "failures": []
  },
  {
    "id": "maritime-safety-mechanism",
    "query_id": "maritime-safety",
    "passed": true,
    "mode": "deterministic retrieval-only evidence",
    "context_manifest": [
      {
        "kind": "tier-0",
        "path": "AGENTS.md"
      },
      {
        "kind": "tier-0",
        "path": "docs/wiki/_system/docs/ROUTER.md"
      },
      {
        "kind": "retrieved-excerpt",
        "path": "applications/autonomous-physical-systems.md",
        "anchor": "applications-autonomous-physical-systems-robotics",
        "characters": 129
      },
      {
        "kind": "retrieved-excerpt",
        "path": "core/epistemic-foundations.md",
        "anchor": "1-the-epistemic-crisis-in-modern-computing",
        "characters": 499
      },
      {
        "kind": "retrieved-excerpt",
        "path": "core/natural-syntax-specification.md",
        "anchor": "4-comprehensive-natural-trell-canonical-example",
        "characters": 499
      },
      {
        "kind": "retrieved-excerpt",
        "path": "applications/overview-and-safety-patterns.md",
        "anchor": "applications-the-universal-epistemic-safety-pattern",
        "characters": 131
      },
      {
        "kind": "retrieved-excerpt",
        "path": "market/competitive-analysis.md",
        "anchor": "d-weft-weavemindai",
        "characters": 394
      },
      {
        "kind": "retrieved-excerpt",
        "path": "applications/security-cloud-and-governance.md",
        "anchor": "18-autonomous-operating-system-kernel-hot-patching",
        "characters": 500
      }
    ],
    "required_term_checks": [
      {
        "requirement": [
          "COLREGs",
          "maritime"
        ],
        "present": true
      },
      {
        "requirement": [
          "guard",
          "ClearWaterway"
        ],
        "present": true
      },
      {
        "requirement": [
          "belief",
          "unverified"
        ],
        "present": true
      }
    ],
    "required_citation_checks": [
      {
        "path": "applications/autonomous-physical-systems.md",
        "present": true
      }
    ],
    "forbidden_paths": [
      "INDEX.md",
      "SCHEMA.md"
    ],
    "full_corpus_reads": 0,
    "failures": []
  }
]
```

## graphify

Graphify resolved Parser to parser.rs; freshness=fresh.

### Evidence

```json
[
  {
    "status_command": "./repobrain graph status",
    "status_exit_code": 0,
    "status": "{\n  \"artifact\": {\n    \"confidence\": {\n      \"AMBIGUOUS\": 0,\n      \"EXTRACTED\": 481,\n      \"INFERRED\": 5\n    },\n    \"diagnostic\": null,\n    \"edges\": 486,\n    \"nodes\": 151,\n    \"path\": \"./graphify-out/graph.json\",\n    \"schema\": \"links\",\n    \"state\": \"ready\"\n  },\n  \"cli\": {\n    \"compatible\": true,\n    \"diagnostic\": \"Graphify 0.9.54 satisfies graphifyy>=0.9.54,<0.10.\",\n    \"install_command\": \"python3 -m pip install --user 'graphifyy>=0.9.54,<0.10'\",\n    \"path\": \"/home/ubuntu/.local/bin/graphify\",\n    \"requirement\": \"graphifyy>=0.9.54,<0.10\",\n    \"version\": \"0.9.54\"\n  },\n  \"config\": {\n    \"code_only\": true,\n    \"emit_html\": false,\n    \"enabled\": true,\n    \"excludes\": [\n      \"**/target/**\",\n      \"**/node_modules/**\",\n      \"**/vendor/**\",\n      \"**/dist/**\",\n      \"**/build/**\",\n      \"**/generated/**\"\n    ],\n    \"missing_roots\": [],\n    \"out\": \"graphify-out\",\n    \"roots\": [\n      \"src\"\n    ]\n  },\n  \"freshness\": {\n    \"built_commit\": \"3b15cc6a224f72002bd365d9734f0c4adffd43b4\",\n    \"changed_sources\": [],\n    \"commit\": \"stale\",\n    \"current_commit\": \"aaf47a2cf0bf07f62c549a6989f6dc4fd3697321\",\n    \"method\": \"git-diff\",\n    \"source\": \"fresh\"\n  },\n  \"html\": {\n    \"available\": true,\n    \"fresh\": true,\n    \"path\": \"./graphify-out/graph.html\"\n  }\n}",
    "query_command": "./repobrain graph query Parser --budget 800",
    "query_exit_code": 0,
    "query_excerpt": "Graph: graphify-out/graph.json (151 nodes) | Traversal: BFS depth=2 | Start: ['Parser', 'parser.rs'] | 66 nodes found\n\n[!] TRUNCATED: showing 45 of 66 nodes (~800-token budget). The answer may be among the 21 cut nodes \u2014 raise the token budget (CLI: --budget) or narrow the query (e.g. context_filter=['call'], or get_node for a specific symbol).\n\nNODE Parser [src=parser.rs loc=L6 community=3]\nNODE parser.rs [src=parser.rs loc=L1 community=2]\nNODE .advance() [src=parser.rs loc=L24 community=3]\nNODE ast.rs [src=ast.rs loc=L1 community=0]\nNODE .new() [src=parser.rs loc=L12 community=3]\nNODE .check() [src=parser.rs loc=L20 community=3]\nNODE Token [src=lexer.rs loc=L4 community=2]\nNODE .parse_primary() [src=parser.rs loc=L718 community=3]\nNODE .consume() [src=parser.rs loc=L35 community=3]\nNODE .parse_stmt() [src=parser.rs loc=L453 community=3]\nNODE .consume_ident() [src=parser.rs loc=L43 community=3]\nNODE .parse_function() [src=parser.rs loc=L319 community=3]\nNODE .parse_postfix() [src=parser.rs loc=L663 community=3]\nNODE .parse_type() [src=parser.rs loc=L401 community=3]\nNODE .parse_program() [src=parser.rs loc=L86 community=3]\nNODE .parse_struct() [src=parser.rs loc=L241 community=3]",
    "symbol": "Parser",
    "expected_source": "parser.rs",
    "matched_node": {
      "id": "parser_parser",
      "label": "Parser",
      "_origin": "ast",
      "community": 3,
      "file_type": "code",
      "norm_label": "parser",
      "source_file": "parser.rs",
      "source_location": "L6"
    },
    "source_exists": true,
    "nodes": 151,
    "edges": 486,
    "adapter_status": {
      "artifact": {
        "confidence": {
          "AMBIGUOUS": 0,
          "EXTRACTED": 481,
          "INFERRED": 5
        },
        "diagnostic": null,
        "edges": 486,
        "nodes": 151,
        "path": "/workspace/graphify-out/graph.json",
        "schema": "links",
        "state": "ready"
      },
      "cli": {
        "compatible": true,
        "diagnostic": "Graphify 0.9.54 satisfies graphifyy>=0.9.54,<0.10.",
        "install_command": "python3 -m pip install --user 'graphifyy>=0.9.54,<0.10'",
        "path": "/home/ubuntu/.local/bin/graphify",
        "requirement": "graphifyy>=0.9.54,<0.10",
        "version": "0.9.54"
      },
      "config": {
        "code_only": true,
        "emit_html": false,
        "enabled": true,
        "excludes": [
          "**/target/**",
          "**/node_modules/**",
          "**/vendor/**",
          "**/dist/**",
          "**/build/**",
          "**/generated/**"
        ],
        "missing_roots": [],
        "out": "graphify-out",
        "roots": [
          "src"
        ]
      },
      "freshness": {
        "built_commit": "3b15cc6a224f72002bd365d9734f0c4adffd43b4",
        "changed_sources": [],
        "commit": "stale",
        "current_commit": "aaf47a2cf0bf07f62c549a6989f6dc4fd3697321",
        "method": "git-diff",
        "source": "fresh"
      },
      "html": {
        "available": true,
        "fresh": true,
        "path": "/workspace/graphify-out/graph.html"
      }
    },
    "freshness": {
      "built_commit": "3b15cc6a224f72002bd365d9734f0c4adffd43b4",
      "changed_sources": [],
      "commit": "stale",
      "current_commit": "aaf47a2cf0bf07f62c549a6989f6dc4fd3697321",
      "method": "git-diff",
      "source": "fresh"
    },
    "failures": []
  }
]
```

## context-budgets

Tier-0 1796/2000; max retrieved 1197/3500; combined 2993/9500 estimated tokens.

### Evidence

```json
[
  {
    "estimator": "characters // 4 (same as wiki_retrieve.py)",
    "tier0": {
      "paths": [
        {
          "path": "AGENTS.md",
          "estimated_tokens": 598
        },
        {
          "path": "docs/wiki/_system/docs/ROUTER.md",
          "estimated_tokens": 1198
        }
      ],
      "estimated_tokens": 1796,
      "budget_tokens": 2000
    },
    "retrieval": {
      "per_query_tokens": {
        "maritime-safety": 1105,
        "belief-certain": 1094,
        "natural-syntax": 1174,
        "competitors": 1197,
        "ten-year": 998,
        "episodic-memory-decision": 623,
        "temporal-timeline": 453
      },
      "maximum_tokens": 1197,
      "budget_tokens": 3500
    },
    "combined": {
      "maximum_tokens": 2993,
      "budget_tokens": 9500
    },
    "failures": []
  }
]
```

## setup-fixture

Realistic Git fixture passed setup non-overwrite and no-semantic-copy safety checks.

### Evidence

```json
[
  {
    "fixture": "temporary Git repository (deleted after evaluation)",
    "fixture_checks": {
      "git_repository": true,
      "tracked_source": true,
      "ignored_source": true,
      "adr": true,
      "context_doc": true,
      "docs_site_marker": true,
      "docs_site_page": true,
      "existing_corpus": true,
      "binary_source": true,
      "large_source": true,
      "source_manifest": true,
      "inventory_adr": true,
      "inventory_context": true,
      "inventory_docs_site": true,
      "inventory_excludes_compiled_corpus": true,
      "code_delegates_graphify": true,
      "manifest_stable_after_unchanged_scan": true,
      "grouped_raw_pointers": true,
      "conversion_success": true,
      "conversion_failure_visible": true,
      "raw_retrieval_paths": true,
      "raw_results_non_authoritative": true
    },
    "tracked_file_count": 72,
    "ignored_file_count": 3,
    "setup_command": "./repobrain setup --no-graphify",
    "setup_exit_code": 0,
    "setup_detection": "detected name=busy-docs-repo code_roots=['src/'] raw=['README.md']",
    "setup_output": "o/.agents/skills/repobrain-label/SKILL.md\nwrote ./.agents/skills/repobrain-maintain/SKILL.md\nwrote ./.agents/skills/repobrain-usage/SKILL.md\nwrote ./.agents/skills/repobrain-setup/SKILL.md\nWrote ./docs/wiki/_system/generated/claim-graph.yaml (7 nodes, 4 edges)\nrepo: .\ndetected name=busy-docs-repo code_roots=['src/'] raw=['README.md']\nHOST.yaml: wrote HOST.yaml from detection\nmkdir: docs/wiki/inbox/archive, docs/wiki/temporal, docs/wiki/raw, docs/wiki/_system/generated/eval, docs/wiki/_system/generated/usage, docs/wiki/_system/generated/sources\nstubs: INDEX.md, log.md, temporal/TIMELINE.md\ngitignore: gitignore graphify-out/\ngitignore: gitignore source cache\ngitignore: gitignore HTML dashboard\nAGENTS.md: appended AGENTS.fragment.md\nlaunchers: installed canonical repobrain-* skills\ngraphify: skipped (--no-graphify)\nWrote docs/wiki/_system/generated/sources/manifest.json (13 entries)\nsources: scanned\n\nSetup complete. Remaining human/agent steps (minimal):\n  1. Edit docs/wiki/_system/config/HOST.yaml `anchor` \u2014 one paragraph the wiki must not dilute\n  2. Fill docs/wiki/_system/config/router-seeds.md with your keywords \u2192 pages\n  3. If seed drafts exist, fill them from source (do not paste code into wiki pages)\n  4. python3 -m pip install --user 'graphifyy>=0.9.54,<0.10'  # if skipped\n  5. ./repobrain doctor\n  6. ./repobrain retrieve \"what is this repo\" --budget-tokens 1500",
    "conversion_output": "Converted or cached: 1\nFailed: data/malformed.csv",
    "safety_checks": {
      "setup_exit_zero": true,
      "conversion_non_strict_exit_zero": true,
      "rescan_exit_zero": true,
      "existing_content_unchanged": true,
      "existing_agent_instructions_preserved": true,
      "semantic_file_set_unchanged": true,
      "raw_docs_not_copied_to_semantic": true
    },
    "baseline_observation": "Setup inventories Git-tracked ADRs, context maps, and docs-site sources into a deterministic manifest and grouped raw pointers without copying them into semantic folders.",
    "failures": []
  }
]
```
