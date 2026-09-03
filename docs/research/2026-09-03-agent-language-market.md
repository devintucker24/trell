# Trell and the agent-language market

**Date:** 3 September 2026  
**Status:** Research note, not a product spec  
**Scope:** Whether a small compiled language can enter the market that LangChain, agent SDKs, and a few new DSLs already occupy, and which niche is actually open.

This document is for the Trell repo. It records market research and a recommended beachhead. It does not describe the current compiler as if it already did any of this. Today Trell compiles one integer expression to LLVM IR. The language thesis below is a *direction*, not a shipping claim.

---

## 1. Verdict

Do not try to replace LangChain. LangChain’s product is connectors, tracing, and habit in Python. A new language will not beat that catalog.

Do try to become **the file format for agent workflows that models write and CI checks**, the way Terraform became the file format for cloud graphs and Starlark became the file format for Bazel graphs. Python SDKs stay as *drivers* (model calls, tools). Trell (or a Trell-shaped language) owns *authority*: who may `ask`, which tools exist, whether a child agent may spawn, whether a human must approve, whether the program is even legal.

That is a real market slice. It is not empty, but it is not owned. BAML owns typed LLM *functions*. Weft is building a full AI *systems* language with a graph UI and durable execution. LangGraph owns Python *graphs* for stateful agents. Nobody owns a **tiny, constrainable grammar whose compiler’s job is capabilities**, aimed at **untrusted programs emitted by other AIs**.

Trell cannot “enter the market” as an LLVM arithmetic toy. It can enter if it ships, in order:

1. A small workflow language and a checker (`trell check`).
2. Adapters to existing model/tool SDKs (LangChain, OpenAI Agents SDK, raw HTTP).
3. A grammar for constrained decoding so models emit only valid Trell.
4. Later: a Rust host that runs spawned agents as WebAssembly with no ambient authority.

Rust and LLVM help (4) and the “fail closed” story. They do not help you beat LangChain at `pip install`.

---

## 2. What the incumbents actually are

Names below are taken from first-party docs, not recaps.

### 2.1 LangChain / LangGraph / LangSmith

LangChain Inc. now splits the stack in its own docs ([LangGraph overview](https://docs.langchain.com/oss/python/langgraph/overview), [LangChain overview](https://docs.langchain.com/oss/python/langchain/overview)):

- **LangChain** — agent harness / framework: abstractions and integrations for models, tools, and agent loops. v1 centers on `create_agent` (model + harness). Legacy chains (`LLMChain`, etc.) moved to classic packages ([LangChain v1 release notes](https://docs.langchain.com/oss/python/releases/langchain-v1)).
- **LangGraph** — “the orchestration runtime: durable execution, streaming, human-in-the-loop, and persistence.” It is explicitly “very low-level, and focused entirely on agent orchestration.” Graphs mix “deterministic, hand-coded steps with LLM-driven agentic steps.” Official docs warn beginners that LangGraph is low-level and often recommend starting with LangChain agents instead.
- **LangSmith** — “the platform for tracing, evaluation, prompts, and deployment across frameworks.” Works with many frameworks, not only LangChain.
- **Deep Agents** — a harness (planning, subagents, filesystem tools) *on top of* LangGraph.

LangGraph models a program as `State`, `Nodes` (Python functions), and `Edges` ([Graph API](https://docs.langchain.com/oss/python/langgraph/graph-api.md)). Persistence is checkpoints on a thread, not a separate language.

**Implication for Trell:** LangGraph already *is* the orchestration runtime in Python. Competing with “we also have nodes and edges” is a loss. Competing with “the graph is a reviewable, capability-checked *source file* that is not Python” is a different product. You would more likely *target* LangGraph as a backend than replace it.

### 2.2 OpenAI Agents SDK (and other vendor SDKs)

Official docs ([openai.github.io/openai-agents-python](https://openai.github.io/openai-agents-python/)): a “lightweight, easy-to-use package with very few abstractions,” evolved from Swarm. Primitives: Agents (LLM + instructions + tools), handoffs, guardrails. Design principle: “Python-first: Use built-in language features to orchestrate and chain agents, rather than needing to learn new abstractions.” Also: sandbox agents in isolated workspaces, MCP tools, tracing.

Same pattern elsewhere: Anthropic’s Claude Agent SDK exposes the Claude Code agent loop as a Python/TypeScript library ([code.claude.com docs](https://code.claude.com/docs/en/agent-sdk/overview.md)). Microsoft folded Semantic Kernel and AutoGen into a single Agent Framework and put AutoGen into maintenance mode ([Agent Framework overview](https://learn.microsoft.com/en-us/agent-framework/overview/), [AutoGen](https://github.com/microsoft/autogen)). Vendors are consolidating *libraries*, not inventing new languages.

**Implication:** The vendor SDK’s pitch is *stay in Python/TS*. A language that asks people to leave the host language is swimming upstream unless the program must not *be* host code (untrusted generated code, WASM, git-reviewed policy).

### 2.3 BAML (BoundaryML)

GitHub describes BAML as “The programming language for agents” ([github.com/BoundaryML/baml](https://github.com/BoundaryML/baml)). Docs: a DSL for **type-safe LLM functions** — inputs, return types, client, Jinja prompt — then generate clients in Python, TypeScript, Go, etc. ([docs.boundaryml.com](https://docs.boundaryml.com/)). Analogy they use: TSX for the web, BAML for prompt engineering. Public messaging still centers on that function/schema job; the project has also been evolving a fuller language compiler in-repo. Parsing and JSON repair still happen at runtime — schemas are checked, the model is not proven correct without a parser.

**Implication:** BAML already took “language, not a prompt string” for the *call*. Trell should not clone `function ExtractEmail(...) -> Email`. If Trell is a language, its extra job is **workflow + permissions + spawn**, with typed `ask` as one primitive (interop with BAML is allowed: BAML extracts structure; Trell checks authority / runs sandboxed compute).

### 2.4 Weft (WeaveMindAI)

First-party docs ([weavemind.ai/docs](https://weavemind.ai/docs)): “A programming language for AI systems.” Dual view: dense code for AI, visual graph for humans. “If it compiles, it runs.” LLMs, humans, and services as primitives. Compiler + type system + durable executor described as the stable core; node catalog still small (beta, breaking changes expected). Built in Rust; durable execution via Restate. GitHub README treats the project as early / POC-grade, with an active rebuild and production caution ([github.com/WeaveMindAI/weft](https://github.com/WeaveMindAI/weft)).

**Implication:** Weft is the closest *full-stack* competitor to the “language for AI orchestration” dream, but it is not a finished incumbent. Trell should not try to out-catalog Weft (Slack, WhatsApp, Postgres nodes). Differentiate on **untrusted spawn + capabilities + a grammar small enough to constrain-decode**, and on staying tiny.

### 2.5 DSPy

Stanford NLP: “the framework for programming—not prompting—language models” ([dspy.ai](https://dspy.ai/), [github.com/stanfordnlp/dspy](https://github.com/stanfordnlp/dspy)). Signatures declare typed I/O; optimizers compile better prompts/weights. Still **Python**.

**Implication:** DSPy owns *optimizing* LM modules. Not Trell’s beachhead. A Trell `ask` type could later feed a DSPy signature. Do not build an optimizer first.

### 2.6 Durable execution engines

Restate ([docs.restate.dev](https://docs.restate.dev/index), [durable agents](https://docs.restate.dev/ai/patterns/durable-agents)): a runtime that journals LLM calls and tool steps so agents resume after crashes; integrates *with* existing agent SDKs rather than replacing them. Temporal occupies the same “workflow that survives process death” niche for general backends.

**Implication:** Durability is an *engine*. Trell can compile/check a program that *runs on* Restate or LangGraph checkpoints. Do not build Restate in year one.

### 2.7 Constrained decoding (the LLM-specific lever)

A language with a small context-free grammar can force next-token masks so invalid programs are impossible, not “please don’t.” Primary sources:

- Outlines / Willard & Louf: compile regex/JSON schema/CFG to an FSM and mask logits ([arxiv 2307.09702](https://arxiv.org/abs/2307.09702), commonly cited from Outlines).
- XGrammar: CFG via pushdown automaton + adaptive token mask cache ([arxiv 2411.15100](https://arxiv.org/abs/2411.15100); [MLC blog](https://blog.mlc.ai/2024/11/22/achieving-efficient-flexible-portable-structured-generation-with-xgrammar)). Used in production serving stacks (vLLM/SGLang as of 2025–2026 industry writeups).
- llama.cpp GBNF and related engines (llguidance) occupy the same layer.

**Implication:** This is one of the few techniques that *actually upgrades models* because of a custom language. You cannot constrain-decode “any LangChain Python program.” You can constrain-decode a 20-keyword Trell. That is a technical moat SDKs do not have.

### 2.8 WASM / Wasmtime (the Rust-specific lever)

Wasmtime’s security doc ([docs.wasmtime.dev/security.html](https://docs.wasmtime.dev/security.html)): execute untrusted code in a sandbox; no ambient syscalls; “all interaction with the outside world is done through imports and exports”; WASI filesystem is capability-based. Host language is Rust.

**Implication:** If Trell’s story is “agents that create agents,” the compile *target* that matches the threat model is WASM, not a native `main` with host privileges. LLVM-to-native is the learning path. LLVM-or-Cranelift-to-WASM plus a capability host is the product path.

### 2.9 Historical analog: languages that stole a slice from SDKs

| Domain | SDK era | Language that took the slice | Why the language won |
| --- | --- | --- | --- |
| Cloud resources | boto3 / Azure SDK scripts | Terraform HCL | The *graph* needed to be diffed, planned, and reviewed. |
| Build graphs | Maven XML / imperative scripts | Starlark (`BUILD` / `.bzl`) ([bazel.build/rules/language](https://bazel.build/rules/language)) | Hermetic, restricted Python-like language; not general Python. |
| Policy | ad-hoc `if` in app code | Rego (OPA), CEL | Policy as data, evaluated in a sandbox. |
| Queries | string-built SQL in app code | SQL | The query *is* the artifact. |

None of these replaced the host language. They took the slice where the host language was the wrong representation.

Agent workflows today are still in the boto3 era: objects in Python that *can* express a graph, while the thing you want to review, constrain, and sandbox is the graph itself.

---

## 3. Competitor map (short)

| Name | Kind | Job it actually does | Trell overlap |
| --- | --- | --- | --- |
| LangChain | Library | Integrations + agent loops | Use as backend, do not clone |
| LangGraph | Library / runtime | Stateful graphs, HITL, checkpoints | Possible execution backend |
| LangSmith | SaaS | Trace, eval, deploy | Orthogonal; integrate later |
| OpenAI Agents SDK | Library | Thin agents, handoffs, sandboxes | Backend / competitor for “stay in Python” |
| PydanticAI | Library | Typed Python agents | Same as OpenAI SDK, types-first |
| CrewAI / AutoGen | Library | Multi-agent patterns | Social layer; not a language |
| Mastra / Vercel AI SDK | Library (TS) | JS/TS agents | Same SDK war, different language |
| BAML | DSL + codegen | Typed LLM functions | Overlaps `ask`; do not clone whole product |
| Weft | Language + UI + durable run | AI systems as compiled graphs | Closest full vision; differentiate on size + sandbox spawn |
| DSPy | Library | Optimize prompts/weights | Later interop |
| Restate / Temporal | Engine | Durable replay | Backend, not competitor |
| YAML agent configs / n8n | Config / GUI | No-code or config graphs | Weak language; possible migration source |

---

## 4. Recommended niche (the entrance)

**Name it internally:** *capability-checked workflow source for untrusted authors.*

**Who writes Trell:** coding agents and humans in the same git repo. The file is the workflow. CI runs `trell check`. Invalid tool grants, missing `approve` on destructive effects, or spawn that exceeds parent rights fail the build *before* an API call.

**Who it is not for (yet):** data scientists chaining RAG loaders; people who want 1,000 integrations; notebooks.

**Why this is easier than “replace LangChain”:**

- You do not need a connector catalog. You need `ask`, `tool` grants, `send`, `spawn`, `approve`, `budget`, and a mock mode.
- Your first users can already be **you** (and any agent that edits this repo). The distribution channel is “agents write the language they are constrained to write.”
- Constrained decoding is a demo that SDKs cannot copy without shrinking Python.
- The pain is already visible: CLAUDE.md, agent YAML, LangGraph objects, and chat transcripts are all *almost* the source of truth and none of them check capabilities.

**Why this is still hard:**

- Weft and BAML exist. Messaging must be precise: Trell is not “typed prompts” and not “visual AI IDE.”
- Python gravity is real. Ship `trell check` + “run via your existing SDK” on day one or nobody tries it.
- Trust: a language that runs tools will be treated as malware unless the sandbox story is boringly strict.

**Beachhead ranking (what to own first):**

1. **Git-native agent workflows for software repos** (highest “easy entrance”). Coding agents already write files. Give them a grammar, a checker, and mock execution. Human reviews the diff. Closest analog: Terraform for “what the agent is allowed to do in this repository.”
2. **Sandboxed multi-agent spawn** (the Rust/WASM story). Second, because it needs a host. This is where Python SDKs are structurally wrong (eval, subprocess, hope).
3. **Typed `ask` only** — do not lead with this; BAML is there.

LangChain “adopting Trell” would look like emitting or ingesting `.trell` the way some tools emit HCL or OPA policy, not rewriting LangChain in LLVM.

---

## 5. Can Trell make its own entrance?

Yes, if “Trell” means the *language + checker*, not today’s `ret i64 64`.

Concrete entrance sequence that does not require winning a framework war:

1. **Publish the grammar** (one page) and a `trell check` binary. Even an interpreter is enough. LLVM is optional.
2. **One runnable example:** “agent may `read` this repo, must `approve` before `write` or `git push`, `ask` returns a typed `{ ok, reason }`.” Mock provider so CI is free.
3. **Constrained-decode demo:** a local or open-weight model can *only* emit valid `.trell`. Film that. It is the unique LLM enhancement.
4. **Adapter:** `trell run --backend openai-agents|langchain|http`. You ride their connectors.
5. **WASM spawn** when `spawn` is real. Wasmtime host in Rust. This is the “Rust was the right bet” chapter.

Entrance *channel:* agents (Cursor, Claude Code, Codex) generating Trell because the grammar is in-repo and check fails loudly. That is how Terraform spread (plan in CI), not how LangChain spread (Twitter + pip).

What will **not** create an entrance: more LLVM operators, a blog that says “LangChain killer,” or a visual graph (Weft’s turf).

---

## 6. What Rust and compiling are for in this niche

Honest split:

| Layer | Need compile/Rust? | Why |
| --- | --- | --- |
| Syntax people read | No | An interpreter ships faster (Skiff-shaped). |
| Checker (types, capabilities, budgets) | Optional | Can be Rust; that is “compiling” even without object files. |
| Fail-closed before spend | Checker, not LLVM | Same as `terraform plan`. |
| Constrained decoding | Grammar, not LLVM | Export GBNF/XGrammar from the same grammar. |
| Untrusted `spawn` | Yes: WASM + Rust host | Wasmtime: no ambient I/O; WASI capabilities. Native LLVM binaries are the wrong default for generated agents. |
| Fast glue between asks | Later LLVM | Only if you loop over large data in-language. |
| LangChain-class integrations | No | Call out to Python/TS. |

Learning LLVM on arithmetic remains valid as a gym. Do not confuse the gym with the niche.

---

## 7. Techniques that actually help models (keep these in scope)

These are documented mechanisms, not slogans:

1. **Grammar-constrained generation** of the *workflow file* (XGrammar, Outlines, GBNF). Invalid Trell cannot be sampled.
2. **Typed `ask` / structured outputs** so judgment is data (BAML’s function contract; provider JSON schema modes). Retry on type error, not on “the essay was almost JSON.”
3. **Capability and taint rules in the checker.** Tool output and other-agent text cannot become `spawn` source or a shell tool without `approve` or a sanitizer. Prompt injection is information flow; a language can mark values dirty.
4. **Holes / incomplete programs** (Hazel-style research): pause on `??` instead of regenerating the whole file. Complements constrained decoding.
5. **Durable replay** of `ask` and tools (Restate, LangGraph checkpoints) as a *backend*, so long runs do not double-charge on crash.
6. **WASM isolates per spawned agent** (Wasmtime security model) so “agents creating agents” has a lid: parent rights are a ceiling.

Skip as year-one scope: putting an LLM inside the compiler, competing with LangSmith, a node marketplace.

---

## 8. Risks and non-goals

- **Weft timing.** They already say “language for AI systems” and “if it compiles, it runs.” Treat them as the incumbent vision. Differentiate; do not ape the dashboard.
- **BAML timing.** Do not spend a year on prompt functions.
- **Framework fatigue.** The market is tired of Python frameworks *and* skeptical of new languages. The checker + mock CI path is the least arrogant entrance.
- **Security theater.** A compiled language that links `printf` and the filesystem is not safer than Python. WASM + explicit imports is.
- **Current Trell.** Shipping this research as if the compiler implemented it would be false advertising. Keep README honest.

Non-goals for the niche: replacing Python for RAG, notebooks, or general programming; beating LangChain on download counts; a visual IDE.

---

## 9. Suggested reading (primary)

- LangGraph overview: https://docs.langchain.com/oss/python/langgraph/overview
- LangGraph Graph API: https://docs.langchain.com/oss/python/langgraph/graph-api.md
- OpenAI Agents SDK: https://openai.github.io/openai-agents-python/
- BAML: https://docs.boundaryml.com/ and https://github.com/BoundaryML/baml
- Weft: https://weavemind.ai/docs
- DSPy: https://dspy.ai/
- Restate durable agents: https://docs.restate.dev/ai/patterns/durable-agents
- XGrammar paper: https://arxiv.org/abs/2411.15100
- XGrammar/MLC writeup: https://blog.mlc.ai/2024/11/22/achieving-efficient-flexible-portable-structured-generation-with-xgrammar
- Wasmtime security: https://docs.wasmtime.dev/security.html
- Starlark: https://bazel.build/rules/language

---

## 10. One-line strategy

**Trell’s market is not “LangChain in Rust.” It is Terraform-for-agent-authority: a small language models can be forced to write, humans can review in git, and a checker (later a WASM jailer) can refuse before anything spends or spawns.**

---

## 11. Appendix: near-term wedge from deeper primary-source pass

A follow-up pass over the same market (vendor SDKs, BAML, Weft, constrained decoding, Wasmtime) supports the verdict above and adds a **staging** note that matches Trell’s *current* code:

**Year-zero wedge (honest to today’s compiler):** ship Trell as a **deterministic numeric/WASM tool**, not as an agent framework. Freeze a tiny grammar, publish `trell.gbnf` (or XGrammar), compile to Wasm with **no ambient imports**, expose `trell.eval(...)` / `trell.run` to Python or TypeScript SDKs and MCP. Demo: an agent must compute a fee schedule; Python `eval` is forbidden; Trell is required. That is Starlark-shaped (restricted, hermetic) and Wasmtime-shaped (sandbox), and it does not require a connector catalog.

**Year-one product (this document’s niche):** grow the same grammar into **capability-checked workflow source** (`ask`, tool grants, `approve`, `spawn` ceilings) with `trell check` in CI. Keep LangChain / OpenAI Agents SDK / Claude Agent SDK / Restate as *backends*.

**Do not confuse the wedge with the market.** Arithmetic-in-WASM is how a learning compiler *enters* without lying. Authority-checked workflows are how it *matters*. Rebuilding journals, traces, or 1,000 integrations is how it dies.

Additional primary links from that pass:

- LangChain overview: https://docs.langchain.com/oss/python/langchain/overview
- LangChain v1: https://docs.langchain.com/oss/python/releases/langchain-v1
- Claude Agent SDK: https://code.claude.com/docs/en/agent-sdk/overview.md
- Microsoft Agent Framework: https://learn.microsoft.com/en-us/agent-framework/overview/
- Outlines paper: https://arxiv.org/abs/2307.09702
- LMQL: https://lmql.ai/
- llama.cpp GBNF: https://github.com/ggml-org/llama.cpp/blob/HEAD/grammars/README.md
- Temporal (durable execution): https://docs.temporal.io/evaluate/understanding-temporal
- PydanticAI: https://ai.pydantic.dev/
- Mastra: https://mastra.ai/docs
- CUE: https://cuelang.org/docs/introduction/
- OPA Rego: https://www.openpolicyagent.org/docs/latest/policy-language/
