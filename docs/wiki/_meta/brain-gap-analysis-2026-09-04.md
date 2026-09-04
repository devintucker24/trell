---
id: brain-gap-analysis-2026-09-04
title: Brain/Wiki Gap Analysis — File RAG, Memory Tiers & Context Engineering
type: synthesis
status: active
created: 2026-09-04
updated: 2026-09-04
tags: [memory, rag, context-engineering, episodic, semantic, retrieval, synthesis]
domain: meta
summary: "Audit of Trell wiki-as-memory vs 2026 agent-memory research; gaps and upgrade path for efficient file RAG."
nodes:
  - id: brain-gap-analysis
    kind: concept
  - id: memory-episodic
    kind: concept
  - id: memory-semantic
    kind: concept
  - id: memory-working
    kind: concept
  - id: retrieval-rerank
    kind: technology
edges:
  - from: memory-semantic
    to: wiki-index
    rel: implements
  - from: memory-episodic
    to: wiki-inbox
    rel: related_to
  - from: retrieval-rerank
    to: brain-gap-analysis
    rel: depends_on
  - from: memory-working
    to: agents-md
    rel: depends_on
related:
  - "[[_meta/CONTEXT_PROTOCOL]]"
  - "[[ROUTER]]"
  - "[[SCHEMA]]"
  - "[[INDEX]]"
agent:
  priority: critical
  read_when:
    - "improving wiki memory / RAG"
    - "context engineering for agents"
    - "episodic vs semantic design"
  maintain:
    - "revisit after retrieve skill lands; keep aligned with research"
  context_tier: 2
implements_code: []
---

# Brain Gap Analysis — File RAG, Memory & Context Engineering

**Date:** 2026-09-04  
**Corpus size:** ~35 wiki pages · ~124 KB · ~32k tokens if naively dumped · 83 graph nodes / 90 edges  
**Verdict:** Strong **semantic knowledge compiler** + ops schema. Weak as a full **agent memory system** (episodic / working / retrieval / rerank / eval). Fixable without leaving the file-first model.

---

## 1. What We Already Have (strengths)

| Capability | Status | Artifact |
|---|---|---|
| Semantic memory (compiled knowledge) | **Strong** | `docs/wiki/{core,theory,applications,market,roadmap}` |
| Schema / ontology | **Strong** | `AGENTS.md`, `SCHEMA.md`, typed nodes/edges |
| Progressive disclosure (partial) | **Partial** | Skills frontmatter + INDEX summaries |
| Graph RAG substrate | **Good** | `GRAPH.yaml` hubs + typed `rel` |
| Ingest pipeline | **Good** | inbox → triage → ingest |
| Health loop | **Good** | doctor → heal → re-doctor |
| Chronological ops log | **Thin episodic** | `log.md` (ops only, not experience narratives) |
| Raw provenance | **Good** | `raw/` pointers |

This matches Karpathy LLM Wiki well: **compile once, query the compilation**.

---

## 2. Map to 2026 Agent-Memory Taxonomy

Research consensus (agent-memory surveys 2025–2026; Amory; xMemory; AMA-Bench; context-engineering guides):

```
┌─────────────────────────────────────────────────────────┐
│ WORKING MEMORY (session)                                │
│  task brief · open files · scratch · budgeted context   │
├─────────────────────────────────────────────────────────┤
│ EPISODIC MEMORY (experiences over time)                 │
│  sessions · decisions · failures · corrections · why    │
├─────────────────────────────────────────────────────────┤
│ SEMANTIC MEMORY (stable knowledge)                      │
│  concepts · domains · code bindings · thesis            │
├─────────────────────────────────────────────────────────┤
│ PROCEDURAL MEMORY (how to act)                          │
│  skills · schemas · doctor/heal/triage playbooks        │
└─────────────────────────────────────────────────────────┘
         ▲ retrieve (hybrid) + rerank + cite
```

| Tier | Trell wiki today | Gap |
|---|---|---|
| **Working** | `ROUTER.md` + `episodic/session-current.md` | Keep scratch capped; avoid reloading fat AGENTS body |
| **Episodic** | `episodic/` narratives + INDEX | Consolidation job still manual |
| **Temporal** | `temporal/TIMELINE.md` + `temporal:` fields + `--as-of` | Bi-temporal claim-level annotations still coarse (page-level) |
| **Semantic** | Excellent compiled pages | Chunk IDs improving via retrieve sections |
| **Procedural** | Skills + retrieve | — |
| **Retrieval** | `wiki_retrieve.py` hybrid + MMR | Optional dense vectors when ≫200 pages |
| **Eval** | `_meta/eval-queries.yaml` | Automate scoring script next |

---

## 3. Critical Gaps (ordered by ROI)

### G2 — No first-class episodic memory
Agents forget *why* a decision was made after the chat ends.  
**Need:** `docs/wiki/episodic/` with episode notes (goal, actions, outcome, lessons) + consolidation job → semantic pages.  
**Status (2026-09-04):** **Landed** — `episodic/`, template, session scratch, first episode.

### G2b — No first-class temporal memory
`updated:` dates ≠ validity. Agents confuse stale claims with current truth; cannot answer as-of / what-changed.  
**Need:** `temporal:` frontmatter (`observed_at`, `valid_from`, `valid_until`, supersession), `temporal/TIMELINE.md` spine, retrieve `--as-of`, doctor expiry checks, decay/consolidation.  
**Status (2026-09-04):** **Landed** — SCHEMA §9, TIMELINE, temporal scoring in retrieve, doctor `expired_still_active`.

### G3 — Retrieval is hand-rolled, not a skill
Navigate skill helps humans; it is not a scored retriever.  
**Need:** `skills/wiki/retrieve` + script: lexical score over title/summary/tags + **graph boost** + optional recency + diversity; return top-k paths with scores (file RAG).  
**Status (2026-09-04):** **Landed** — `wiki_retrieve.py` + retrieve skill (+ temporal + MMR).

### G1 — Always-on context is too fat (context engineering)
`AGENTS.md` + full INDEX + SCHEMA ≈ thousands of tokens before task work.  
**Need:** Tier-1 **ROUTER** (~40–80 lines) always loaded; everything else on demand (`context_tier: 1|2|3`).  
**Status (2026-09-04):** **Landed** — `ROUTER.md` + `CONTEXT_PROTOCOL.md`; AGENTS bootstrap updated.

### G4 — No reranking / redundancy control
Similarity-only (even lexical) returns near-duplicate application pages.  
**Need:** Light rerank: MMR-style diversity + priority boost (`agent.priority`) + graph hop distance from query seeds.

### G5 — Pages lack chunk addresses
Loading a 7KB syntax page for one keyword wastes context.  
**Need:** Stable section anchors + optional `chunks:` frontmatter or “quote with line range” retrieve mode.

### G6 — No embeddings path (optional phase)
Pure files scale to hundreds of pages with lexical+graph; thousands need vectors.  
**Need:** Optional `docs/wiki/_meta/embeddings/` or external index (qmd / sqlite-vec) behind same retrieve skill — **not** required at current ~32k-token corpus.

### G7 — No memory consolidation / forgetting
Inbox + log grow forever; no decay.  
**Need:** Periodic “consolidate episodes → semantic”; archive low-salience episodes; doctor check for stale episodes.

### G8 — No eval harness for memory quality
Doctor checks structure, not “did retrieve return the maritime page for ship questions?”.  
**Need:** Tiny golden query set in `_meta/eval-queries.yaml` + score script.

### G9 — Working memory / session scratch not durable
Cross-session agents re-discover.  
**Need:** `docs/wiki/episodic/session-current.md` (or dated) as writable scratch with hard size cap.

### G10 — Multi-hop graph queries under-specified
GRAPH exists; skills don’t say “expand 1 hop from hubs then read”.  
**Need:** Retrieve protocol: seed pages → expand `applies_to`/`depends_on` ≤1–2 hops → rerank → read.

---

## 4. Target Architecture (file-first RAG)

```
ROUTER.md (tier-1, always)          # ~60 lines: where things live + budgets
    │
    ├─ retrieve skill                # hybrid lexical + graph + priority rerank
    │     └─ returns top-k paths + scores + why
    │
    ├─ semantic pages (tier-2/3)     # existing wiki domains
    ├─ episodic/ (tier-2)            # session narratives, decisions, failures
    ├─ procedural skills (on demand)
    └─ raw/ (tier-3, provenance only)
```

**Context budget policy (default):**
| Slot | Token budget (guideline) |
|---|---:|
| Router + task | ≤ 800 |
| Retrieved summaries (k=5) | ≤ 1,200 |
| Full pages opened (≤2–3) | ≤ 6,000 |
| Scratch / episode | ≤ 800 |
| **Total wiki-derived** | **≤ ~9k** before code tools |

Never dump the whole wiki (~32k tok) into context.

---

## 5. Research Anchors (why these gaps matter)
- **Progressive disclosure** for agents: load index/metadata first; deepen only as needed (reduces context rot).
- **Episodic → semantic consolidation** (Amory-style): narratives offline; facts promoted to stable pages.
- **Beyond raw similarity** (xMemory / GraphRAG / HippoRAG-class ideas): structure-guided retrieval beats flat chunk RAG for multi-fact agent tasks.
- **AMA-Bench / memory surveys (2026):** long-horizon agents need selective forgetting, causal/stateful memory — not only recall.

---

## 6. Implementation Priority
1. ~~ROUTER + CONTEXT_PROTOCOL~~ **done**
2. ~~retrieve skill + hybrid/temporal rerank~~ **done**
3. ~~episodic/ + temporal/ lanes~~ **done**
4. eval harness runner over `eval-queries.yaml` — next
5. Optional embeddings backend when page count ≫ 200
6. Finer-grained claim-level temporal annotations (optional)

---

## 7. Non-goals (keep the brain lean)
- Do not replace the wiki with a hosted vector DB as source of truth  
- Do not auto-embed on every edit in v0  
- Do not load all skills’ full bodies every session  
- Do not cite inbox/episodes as semantic truth until consolidated
- Do not treat `updated:` alone as temporal validity — use `temporal.valid_*`
