---
id: raw-examples
title: "Raw pointer — examples/*.trell"
type: raw-pointer
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [raw, examples]
domain: meta
summary: "Immutable pointers to executable Trell examples used by the wiki."
origin: examples/
nodes:
  - id: raw-examples-bundle
    kind: example
edges:
  - from: example-autonomous-ship
    to: raw-examples-bundle
    rel: depends_on
  - from: example-bank-transfer
    to: raw-examples-bundle
    rel: depends_on
related:
  - "[[applications/autonomous-physical-systems]]"
  - "[[applications/financial-treasury-and-markets]]"
agent:
  priority: medium
  read_when:
    - "need runnable code samples"
  maintain:
    - "after example changes, sync application wiki pages and cargo test"
---

# Raw: Executable Examples

| File | Domain |
|------|--------|
| [`examples/autonomous_ship.trell`](../../../examples/autonomous_ship.trell) | Maritime COLREGs |
| [`examples/bank_transfer.trell`](../../../examples/bank_transfer.trell) | Treasury quorum |
| [`examples/medical_diagnosis.trell`](../../../examples/medical_diagnosis.trell) | Clinical biomarkers |
| [`examples/financial_settlement.trell`](../../../examples/financial_settlement.trell) | Classic settlement |
| [`examples/code_synth_guard.trell`](../../../examples/code_synth_guard.trell) | Code capability guard |
| [`examples/deterministic_math.trell`](../../../examples/deterministic_math.trell) | Deterministic arithmetic |

Wiki application pages cite these; the examples remain the executable source of truth.
