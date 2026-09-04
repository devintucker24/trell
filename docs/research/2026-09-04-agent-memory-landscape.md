# What agent memory looks like in 2026

Research notes gathered before and during the redesign of Palimpsest's surface
syntax. Written down because the conclusions shaped the language, and because
the most useful finding was not a product but a pattern of things people keep
rebuilding by hand.

Sources are linked inline. Everything here was read in September 2026.

---

## 1. The retrieval stack has stabilised

The 2026 production RAG pipeline is no longer contested. Across
[a production architecture guide](https://cadence.withremote.ai/blog/production-rag-architecture),
[a pipeline guide](https://datarmatics.com/how-to-build-rag-pipeline-guide-2026/),
and [NVIDIA's enterprise reference architecture](https://docs.nvidia.com/enterprise-reference-architectures/enterprise-rag-retrieval-scaling-and-sizing-guide/latest/rag-retrieval-accuracy-performance.html)
the same shape appears:

| stage | current default |
|---|---|
| chunking | structure-aware, 512 tokens for factoid corpora, 1024 for analytical; 128 is now considered harmful |
| contextual embedding | a 50–100 token document-level summary prepended to each chunk before embedding |
| storage | pgvector under 1M vectors; Qdrant, Pinecone, Weaviate, Milvus above |
| retrieval | dense and BM25 in parallel, fused with Reciprocal Rank Fusion, top 50 |
| reranking | cross-encoder (Cohere Rerank 3, Voyage rerank-2.5, BGE v2-m3) down to top 5–8 |
| control flow | agentic loop: plan, retrieve, judge sufficiency, iterate with a hard cap |

Two numbers worth keeping: Anthropic's contextual retrieval reduces failed
retrievals by roughly 49% on its own and 67% with a reranker; pure vector search
is now described as an anti-pattern outside trivial cases, failing at retrieval
roughly 40% of the time.

**Implication for Palimpsest.** This stack is good at what it does and Palimpsest
should not be compared to it. Every stage optimises *relevance* — which passages
bear on this question. None of them computes *currency* — which of two relevant
passages is the one that is true now. Those are different relations, and the
format has nowhere to record the second. A reranker that puts a superseded policy
above the current one is not misbehaving; it was never told one superseded the
other.

## 2. The second-brain pattern, and where it stops

Two things landed in April 2026 and are more relevant to this project than any
vector database.

### Karpathy's LLM Wiki

[The gist](https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f)
(~5,000 stars and forks on a single markdown file) proposes that instead of
re-retrieving from raw sources at query time, the agent incrementally maintains a
persistent wiki of interlinked markdown. Knowledge is compiled once and kept
current, not re-derived per query.

- **Three layers.** Immutable raw sources; an LLM-owned markdown wiki; a schema
  file (`CLAUDE.md` / `AGENTS.md`) that makes the agent a disciplined maintainer
  rather than a chatbot.
- **Three operations.** Ingest, query, **lint**.
- **Two navigation files.** `index.md` as catalogue, `log.md` as timeline.
- **Scale boundary.** The author puts the crossover where a wiki beats RAG at
  roughly 50k–100k tokens; RAG for millions.
- **Division of labour.** The human curates sources and asks questions; the LLM
  does summarising, cross-referencing, filing, and bookkeeping. The argument is
  explicitly economic: wikis fail because maintenance cost grows faster than
  value, and LLMs drive that cost near zero.

The lint operation is described as: contradictions between pages, stale claims
newer sources have superseded, orphan pages, missing cross-references, gaps.

### GBrain

[GBrain](https://github.com/garrytan/gbrain) (MIT, ~14k stars) is the same
pattern shipped as software:

- **Markdown in a git repo is the system of record.** Postgres with pgvector
  indexes it; deletes in git become soft-deletes in the database. Knowledge stays
  diffable, branchable, and human-readable.
- **Hybrid retrieval.** HNSW vector plus tsvector keyword, fused with RRF.
- **A self-wiring knowledge graph.** Every page write extracts entity references
  from wikilinks and creates typed edges (`works_at`, `invested_in`, `founded`,
  `advises`) with zero LLM calls. Reported as +31.4 points P@5 over its
  graph-disabled variant.
- **30+ operations over CLI and MCP**, plus 34 markdown skill files.

Its own documentation is candid about the boundary: temporal reasoning and
queries of the form "what was true last week but isn't now" are named as *not* a
first-class feature.

### What both are right about

Plain markdown in git is the correct system of record for a brain. It is
readable, diffable, reviewable, portable, and survives the tool that produced it.
Palimpsest adopted this wholesale — `src/markdown.rs` reads fenced `pal` blocks
out of markdown pages and treats the page as the provenance of the facts on it.

## 3. The convergent gap

The most useful reading was not either project but the comment thread under the
LLM Wiki gist, where people who built on the pattern report what broke. The same
missing layer is described repeatedly, by people who do not appear to be talking
to each other:

- *"The missing layer is proof: a wiki can become consistent around one stale
  claim. Each update needs provenance, scope, freshness, and NOT VERIFIED."*
- A page format adding `data-supersedes`, so that *"a correction never erases, it
  leaves one live target with the history still addressable."*
- Knowledge as *"immutable typed cards (corrections supersede; refuted cards stay
  visible as signposted dead ends)."*
- Imported content held separately as *"unverified until reconciled"* pending
  human sign-off, because an external auditor will challenge the provenance of
  every claim.
- A deterministic store resolving conflicts at write time: *"an entrenched fact
  holds; a strong fresh one flips it and demotes the loser to a hidden `stale`
  ... nothing is deleted, so the audit log keeps the loser,"* with predicates
  declarable as temporal so a superseded value stays queryable by date.

That is five independent reinventions of supersession, provenance, staleness,
and audit — implemented as markdown conventions, YAML keys, HTML attributes, and
prompt instructions.

**This is the finding that justifies the project.** These are language features
being maintained as conventions. A convention in a schema file is enforced by a
model remembering to follow it. The same rule in evaluation semantics is enforced
by the rule.

Two more signals from the same thread:

- On lint: *"the wiki audits itself: contradictions with the exact conflicting
  sentences quoted from both pages, stale claims, orphan pages, gaps ... This
  turned out to be the part people react to most, probably because it's the one
  thing a RAG tool structurally can't do."* This is why `check` became a
  first-class operation rather than a CLI flag.
- Left unanswered: *"Has anyone run both layers together, a deterministic fact
  store as the substrate with a maintained wiki as the read surface over it?"*
  That is exactly the position Palimpsest occupies.

There is also a failure mode worth recording that lint does not reach — copied
state going stale. The example given: a date baked into a runnable staleness
check, `date(2026, 3, 15)`, sitting inside code that executes. *"A stale literal
in prose is wrong and looks wrong. A stale literal inside code computes a
confidently wrong number and never errors."* Palimpsest's answer is that a
lifetime is declared (`for 1 year`) rather than computed against a hardcoded
date, and the clock is explicit and virtual.

## 4. Consequences for the language

Four changes came directly out of the above.

**Prose syntax.** If the substrate is markdown that a non-engineer reads, the
statements embedded in it have to be readable too. `assert user.residence =
"Berlin" @ authority(User), source("chat_03");` fails that test.
`alice.city is "Berlin" as user from chat_03` passes it. Prepositions instead of
named arguments, bare dates, layout instead of braces.

**`check` as a first-class operation.** Karpathy's third operation, with a
decidable answer instead of a model's opinion. Contradiction means two live
beliefs at equal standing on the same stated day; staleness means an elapsed
lifetime.

**Markdown ingestion, with the page as provenance.** In a wiki, the page *is* the
document a claim came from, so provenance should be structural rather than
restated on every line. This is what makes `forget everything from
hr_handbook_2026` mean what it looks like.

**Positioning.** Not "better than vector search." A deterministic substrate
underneath a maintained wiki and beside a retrieval index — the layering the gist
thread asked about and nobody had built.

## 5. Prior art checked and deliberately not followed

Recorded briefly; the reasoning is in the README.

- **Letta/MemGPT** — self-editing memory blocks. The model arbitrates
  supersession, which is the decision this project removes from the model.
- **Mem0, LangMem** — extraction and dedup, resolving at write time by heuristic.
  Orthogonal and composable; Palimpsest resolves at read time so as-of queries
  remain answerable.
- **Zep/Graphiti** — closest prior art. Bi-temporal knowledge graph with
  `valid_at`/`invalid_at` and LLM-driven fact invalidation. Goes further than
  Palimpsest on graph structure; has no authority dimension, no refusal, and
  invalidation is a model call rather than a structural property.
- **Datomic** — the ancestor of the storage model. Immutable assertions, as-of
  queries, accumulating log. No notion of a transactor not being entitled to
  assert, of expiry, or of a query refusing an unattributed datom.
- **Datalog / Datascript** — recursive querying Palimpsest lacks.
- **RDF/SPARQL named graphs** — the most complete prior art for attribution. RDF
  can represent everything Palimpsest represents about provenance and cannot act
  on it: there is no SPARQL form meaning "refuse if this graph is untrusted."
- **AGM belief revision** — the formal backdrop. Palimpsest's trust order is a
  concrete, readable selection function where AGM leaves one abstract.
- **TMS/ATMS** — `because` plus cascading retraction is a JTMS with one
  justification per belief.
- **Defeasible logic** — closest formal fit; the trust order is a superiority
  relation and defeat is recorded rather than silent.
- **Answer set programming** — would return multiple answer sets for a
  contradiction. An agent that must act needs to know it is stuck.
- **Differential dataflow / IVM** — the right answer for retraction at scale;
  not implemented, not precluded.
- **SOAR, ACT-R** — the declarative/procedural split is why there is no
  procedural memory here. ACT-R's activation decay makes a stale chunk harder to
  retrieve; Palimpsest makes it retrievable but differently typed, because silent
  retrieval failure is wrong for a system that must explain itself.
- **DSPy, BAML, LMQL, Guidance** — languages for calling a model. Palimpsest
  never calls one. Complementary: BAML to extract claims, Palimpsest to decide
  which survives.
