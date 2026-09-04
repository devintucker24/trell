# Palimpsest

A language for what an agent knows.

Not what it can retrieve — what it *knows*: which fact is current, which one it
replaced, who is entitled to say so, when it stops being true, and what the
agent is allowed to claim it knows.

*A palimpsest is a manuscript scraped down and written over, where the earlier
text is still legible underneath. That is the data structure. Nothing here is
ever deleted; it is written over, and the layer below stays readable.*

```
trust legal above policy above staff above user above rumor

alice.city is "Lisbon" as user from onboarding_form on 2026-03-01
alice.city is "Berlin" as user from relocation_ticket on 2026-08-15

acme.pto.days is 20 as policy from hr_handbook_2026 on 2026-01-01
acme.pto.days is 25 as user   from slack_thread_942 on 2026-09-02

what is alice.city                     # Berlin
what was alice.city on 2026-04-01      # Lisbon
what is acme.pto.days                  # 20 — newer, narrower, and still outranked
why acme.pto.days                      # both layers, and why 25 lost

forget everything from slack_thread_942
check                                  # what is unsourced, stale, or contested
```

No API key. No embedding model. No vector store. `cargo test` and
`cargo run -- examples/moving.pal` are the whole setup.

---

## Contents

- [The thesis](#the-thesis)
- [Where this sits in the 2026 landscape](#where-this-sits-in-the-2026-landscape)
- [Why this cannot be a library](#why-this-cannot-be-a-library)
- [The language](#the-language)
- [Markdown brains](#markdown-brains)
- [Running it](#running-it)
- [The scenarios, and their real output](#the-scenarios-and-their-real-output)
- [What is real and what is not](#what-is-real-and-what-is-not)

---

## The thesis

The seed idea for this project was that agent memory is treated as a
similarity-search problem and should be treated as a name-resolution problem. I
think that is right and I think it stops one step short.

Name resolution alone gets you shadowing and recency, which compilers have done
since the 1960s. That already beats a vector index, because "Berlin replaced
Lisbon" is a fact about the *relationship between two statements*, and an index
of independent chunks structurally cannot hold it. But scope and recency are not
enough on their own, for a reason the seed thesis names and then leaves as a
corollary: **the most specific and most recent claim is frequently the one you
must not believe.** An employee saying their leave allowance is twenty-five days
is narrower than the handbook and newer than the handbook, and it is wrong.

So the thesis I actually built is one step over:

> **Agent memory is a name-resolution problem where the resolution order is not
> lexical.** Standing comes first, specificity second, recency third. A memory
> language's job is to make that order explicit, deterministic, and auditable,
> and to refuse the question when the order does not settle it.

Which produces the rest of the design:

**Standing is a separate axis from scope depth.** `trust legal above policy above
user` declares an order over who is entitled to be believed. Resolution consults
it before it consults anything else. This is the one line that stops an agent
from being talked out of a policy by whoever spoke most recently.

**Provenance is part of the question, not part of the answer.** `what is verified
deploy.token` is a different operation from `what is deploy.token`. The first
refuses to resolve from anything the brain cannot attribute. Not a warning
attached to the result — the value does not come back, so no downstream
expression can consume it. This is the property that has to be in the evaluation
rule, because anything that hands you the value and a flag is a thing a
programmer forgets to check.

**Forgetting is a semantic operation, not a delete.** `forget everything from
phishing_email_88` withdraws every belief that document taught and every episode
it reported, and the previous answer comes back on its own, because the previous
answer was never destroyed. A vector store can delete the embedding. It cannot
restore what the document displaced, because nothing recorded that it displaced
anything.

**Staleness is a type, not a timestamp comparison.** A belief past its lifetime
resolves to a different type from a fresh one. Code that expected a string does
not silently receive last year's tax rate.

**Contradiction is an outcome.** Two documents of equal standing claiming the
same day and disagreeing is not a ranking problem to be broken by a tiebreak. It
is a fact about the brain, and the language reports it rather than guessing.

**Episodes are not facts.** "We tried the migration and it failed on the
connection pool" has no name to resolve. It is stored separately, never shadowed,
never overwritten — and a resolvable fact may rest on it via `because`, so
withdrawing the episode withdraws the fact.

---

## Where this sits in the 2026 landscape

I went looking for prior art before building, and the most useful thing I found
was that a lot of people have converged on the same problem from the other
direction, and are solving it with conventions rather than semantics.

### The RAG stack, and what it is actually good at

The 2026 production retrieval stack is well-understood and genuinely good:
structure-aware chunking at 512–1024 tokens, a contextual summary prepended to
each chunk before embedding (Anthropic's contextual retrieval, roughly a 49%
reduction in retrieval failures on its own), dense and BM25 retrieval in
parallel fused with Reciprocal Rank Fusion, then a cross-encoder reranker
compressing fifty candidates to five. Pure vector search is now considered an
anti-pattern outside trivial cases.

**Palimpsest does not compete with any of that and should not be compared to
it.** That stack answers "which passages are relevant to this question," which
is a real problem it solves well. It has no answer to "which of these two
passages is currently true," because relevance and truth are different
relations and the pipeline only computes one of them. Reranking a stale policy
above a current one is not a reranker bug. The reranker was never told one
superseded the other, and there is nowhere in the format to say so.

The natural pairing is retrieval for recall over prose and Palimpsest for the
claims that have to be right. Ask the index what documents discuss leave policy;
ask the brain how many days Alice actually has.

### Second brains: LLM Wiki and GBrain

Two things landed in April 2026 and both are more relevant to this project than
any vector database.

**Karpathy's LLM Wiki** proposes that instead of re-retrieving from raw sources
at query time, an agent should incrementally maintain a persistent wiki of
interlinked markdown — knowledge compiled once and kept current. Three layers
(immutable raw sources, an LLM-owned wiki, a schema file governing the agent)
and three operations: ingest, query, and **lint**.

**GBrain** ships the same shape as software: a git repo of markdown is the
system of record, Postgres with pgvector indexes it for hybrid retrieval, and
every page write extracts typed entity edges into a self-wiring knowledge graph
with no LLM calls. Its own documentation is candid about where it stops — it
names temporal reasoning and "what was true last week but isn't now" as
explicitly not a first-class feature.

Both are right about the substrate, and Palimpsest adopts it wholesale: **plain
markdown in git, human-readable and diffable, is the correct system of record
for a brain.** Palimpsest reads exactly that (see [Markdown
brains](#markdown-brains)).

The interesting part is where they stop. Read the LLM Wiki thread and you find
the same missing layer being described over and over by people who built on it:
one commenter concludes "each update needs provenance, scope, freshness, and NOT
VERIFIED"; another adds `data-supersedes` to their page format so "a correction
never erases, it leaves one live target with the history still addressable";
another builds immutable typed cards where "corrections supersede, refuted cards
stay visible as signposted dead ends"; another resolves conflicts at write time
by demoting the loser to a hidden `stale` and keeping it in an audit log, with
predicates declarable as temporal so a superseded value stays queryable by date.

That is five people independently reinventing supersession, provenance,
staleness, and audit — as markdown conventions, YAML keys, and prompt
instructions. **Palimpsest's claim is that this is a language, and they are
writing it without a compiler.** Conventions in a schema file are enforced by an
LLM remembering to follow them. The same rules in evaluation semantics are
enforced by the rules.

The strongest confirmation is a question asked in that thread and left
unanswered: *"Has anyone run both layers together, a deterministic fact store as
the substrate with a maintained wiki as the read surface over it?"* That is
precisely the position this language occupies.

The other tell is `lint`. Karpathy lists it as one of three core operations —
check the wiki for contradictions, stale claims, orphan pages, gaps. One
implementer reports it is "the part people react to most, probably because it's
the one thing a RAG tool structurally can't do." Palimpsest's `check` is that
operation, except it is decidable rather than a model's opinion: contradiction
means two live beliefs at equal standing on the same stated day, and staleness
means a lifetime that has elapsed. Same operation, mechanical answer.

### Agent memory products

**Letta/MemGPT** gives an agent editable memory blocks inside its own context and
lets it rewrite them. The self-edit is the mechanism, which means the model is
the arbiter of what supersedes what. Palimpsest takes that decision away from the
model on purpose: the model writes statements, the language decides which one
holds.

**Mem0** and **LangMem** extract facts from conversation and deduplicate them,
which is genuinely useful and orthogonal. They resolve conflicts by heuristic and
recency at write time. Palimpsest keeps both layers and resolves at read time,
which is what makes `what was alice.city on 2026-04-01` answerable at all.

**Zep/Graphiti** is the closest prior art and deserves a precise answer. It
builds a temporal knowledge graph with bi-temporal edges (`valid_at`,
`invalid_at`) and invalidates facts when new episodes contradict them. It is
genuinely good and the overlap is real: both keep history rather than
overwriting, both answer as-of queries.

Three places Palimpsest goes where Graphiti does not:

1. **Invalidation is decided by an LLM.** Graphiti asks a model whether a new
   edge contradicts an existing one. That is a reasonable engineering choice and
   it is also the exact decision this project exists to remove from the model. In
   Palimpsest, contradiction is a structural property of two beliefs, computed the
   same way every run, with no API key.
2. **There is no authority dimension.** Graphiti's edges carry time but not
   standing. Nothing in the model expresses that the handbook outranks the
   employee. Recency wins, which is the failure mode the whole design is aimed at.
3. **Refusal is not expressible.** Graphiti can tell you a fact's provenance. It
   cannot make a query that structurally cannot return an unsourced fact, because
   the constraint would live in application code, and application code is where
   these checks get forgotten.

### Databases with time and immutable facts

**Datomic** is the ancestor of the storage model and I have taken from it freely:
immutable assertions and retractions, as-of queries, the database as an
accumulating log rather than a mutable cell. What Datomic does not have is an
opinion about *belief*. A datom is a fact asserted by a transaction; there is no
notion that the transactor might not be entitled to assert it, or that the fact
might expire, or that a query might refuse to read an unattributed one. Excision
removes data; it does not restore what the data displaced. Palimpsest is roughly
Datomic's temporal model with an epistemics layer on top and a syntax a
non-programmer can read.

**Datalog** and **Datascript** give recursive querying that Palimpsest does not
have. **RDF/SPARQL named graphs** can express provenance and are the most
complete prior art for attribution — RDF can say anything about anything,
including who said it. What RDF cannot do is *act* on that: there is no SPARQL
form meaning "refuse this query if the triple's named graph is untrusted." The
information is representable and inert. Palimpsest represents less and enforces
it.

### Theory

**AGM belief revision** is the formal backdrop, and Palimpsest is a deliberately
partial implementation of it. AGM's expansion, contraction, and revision map onto
writing a fact, `forget`, and supersession. AGM leaves the selection function —
which beliefs to give up when you must give up something — abstract. Palimpsest's
answer is the trust order, which is less general than AGM allows and has the
advantage of being a thing a person can read off the top of a file.

**Truth maintenance systems** are what `because` and cascading retraction are.
This is a JTMS with one justification per belief rather than a general dependency
network, which is a real limitation (see [What is real and what is
not](#what-is-real-and-what-is-not)) and keeps retraction linear and explicable.

**Defeasible logic** is the closest formal fit: the trust order is a superiority
relation over defeasible rules, and defeat is recorded rather than silent, which
is what `conflicts` reports.

**Answer set programming** would handle the contradiction case by producing
multiple answer sets. That is more expressive and less useful here — an agent
that must act needs to know it is stuck, not receive two consistent worlds.
Palimpsest refuses instead.

**Incremental view maintenance / differential dataflow** is the right frame for
making retraction efficient at scale, and this implementation does not use it
(retraction is a linear scan over an index). Nothing in the semantics prevents
it; it is an implementation upgrade, not a redesign.

### Cognitive architectures

**SOAR** and **ACT-R** split declarative from procedural memory, and ACT-R's
declarative module has base-level activation decaying with time and disuse. That
decay is Palimpsest's lifetimes with a different curve — and a deliberate
difference: ACT-R makes a decayed chunk *harder to retrieve*, while Palimpsest
makes an expired belief *retrievable but differently typed*. Silent
retrieval-failure is the wrong behaviour for a system that has to explain itself.
A brain that cannot recall something should say so.

The declarative/procedural split is also why Palimpsest has no procedural memory
at all: how an agent does things is what every agent framework already does, and
duplicating it would be the "workflow, not the brain" mistake. The
episodic/semantic split *is* in scope, because that is where the trap is — see
`examples/episodes.pal`.

### Adjacent AI DSLs

**DSPy**, **BAML**, **LMQL**, and **Guidance** are all languages for *calling a
model*: prompts, schemas, structured output, constrained decoding. Palimpsest
never calls a model. There is no inference in the crate and no place to put an
API key. They compose cleanly for that reason — BAML for extracting structured
claims from prose, Palimpsest for deciding which of those claims survives.

### The short version

| | keeps history | as-of query | authority | refusal | deterministic | audits itself |
|---|---|---|---|---|---|---|
| Vector RAG | no | no | no | no | no | no |
| LLM Wiki / GBrain | by convention | no | by convention | no | no | LLM lint |
| Letta / Mem0 | partial | no | no | no | no | no |
| Zep / Graphiti | yes | yes | no | no | LLM-decided | no |
| Datomic | yes | yes | no | no | yes | no |
| RDF named graphs | modelable | modelable | modelable | no | yes | no |
| **Palimpsest** | yes | yes | yes | yes | yes | `check` |

---

## Why this cannot be a library

The test posed for this project: name something in the evaluation semantics or
type system that an SDK cannot provide.

**A library can hand you a value with a flag. It cannot fail to hand you the
value.**

```
what is verified deploy.token
```

If the only belief behind that name is unattributed, this expression does not
evaluate. Nothing is bound; no downstream expression, no string interpolation,
and no prompt template can reach it. The nearest SDK equivalent is
`store.get("deploy.token", verified=True)` returning `None`, and the difference
is the whole point: `None` is a value, it flows, and somebody eventually writes
`or ""`. A refusal is not a value.

Three more that live in the same place:

**A stale belief has a different type.** Past its lifetime, a value resolves to
`Stale { value, age, lifetime }`, not to the string it wraps. Code written for
the fresh case does not accidentally run on the expired one — `expect what is ip
is "10.0.0.1"` *fails* once the lease is gone. A library returns the string and a
`stale=True` attribute nobody reads.

**A low-standing write does not overwrite.** Writing `acme.pto.days is 25 as
user` when a `policy` belief exists does not mutate anything, does not fail, and
does not get dropped. It is inscribed as a layer that loses, and the defeat is
recorded in `conflicts`. There is no map, so there is nothing to put a key in.

**`check` has no library shape.** Not because it is hard to write, but because it
is a question about the belief set as a whole — which names are held at equal
standing with no rule to decide between them — and that question only exists once
"standing" and "the same name" are language concepts rather than application
conventions.

The honest boundary: everything Palimpsest does could be *implemented* as a
library, in the sense that everything any language does could be. What a library
cannot do is make the refusal unavoidable. A library's guarantees are advisory,
enforced by every caller remembering to check. A language's are structural.

---

## The language

Palimpsest is meant to be readable by the person whose policy is being encoded,
not only by the engineer wiring it up. There are no semicolons, no sigils, and no
function-call syntax on the common path.

### Facts

A fact is a sentence. The name, `is`, the value, then any number of
prepositional phrases in any order.

```
alice.city is "Berlin"
alice.city is "Berlin" from relocation_ticket
alice.city is "Berlin" as user from relocation_ticket on 2026-08-15
gateway.ip is "10.0.0.1" as policy from dhcp_lease for 5 minutes
db.status is "degraded" as compliance from pagerduty because migration_attempt_3
token   is "tok_999"   as rumor from anonymous_paste unverified
```

| phrase | meaning |
|---|---|
| `as <tier>` | who is entitled to say this. Defaults to the weakest tier. |
| `from <source>` | the document or conversation it came from. |
| `on` / `since <date>` | when it became true. Defaults to now. |
| `for <duration>` | how long it stays true. |
| `until <date>` | when it stops being true. |
| `because <episode>` | the event it rests on. |
| `verified` / `unverified` | override the default attribution judgement. |

Dates are bare: `2026-08-15`, `2026-09-04T08:15`. Durations are written either
way: `for 30 days`, `for 90d`, `for 5 minutes`.

### Trust

```
trust legal above compliance above policy above staff above user above rumor
```

One line, top to bottom, strongest first. This is the whole precedence model, and
it is deliberately a total order rather than a lattice: a person should be able
to predict which claim wins by reading one line, and partial orders make that a
graph search. A tier nobody declared is an error naming the tiers that exist,
rather than a silent default.

### Scopes

```
about acme:
    region is "eu-west-1" as policy from infra_standard

    about alice:
        city is "Berlin" as user from relocation_ticket
```

`about` prefixes names. A question inside a scope searches outward, so `alice`
sees `acme.region`. Scope depth breaks ties *within* a tier and never crosses
one — the point of the nested example above is that `alice` cannot override
`acme.region` by being more specific.

### Questions

```
what is alice.city                    # resolve now
what was alice.city on 2026-04-01     # resolve as of a date
what is verified deploy.token         # refuse unless attributable
what is fresh gateway.ip              # refuse if past its lifetime
what is trusted policy acme.budget    # refuse below a standing
why alice.city                        # every layer, and why each won or lost
conflicts                             # overrides that were refused
episodes                              # the episodic log
check                                 # health of the whole brain
```

A question on its own line prints its answer. Adjectives stack: `what is verified
fresh gateway.ip`.

### Episodes

Things that happened, which have no name to resolve:

```
when migration_attempt_3:
    happened on 2026-09-04T08:15
    involved deploy_bot, alice
    details service is "billing-db", pool_size is 100
    summary "Schema migration aborted: the connection pool was exhausted"
```

Episodes are never shadowed and never overwritten. A fact may rest on one with
`because`, and `forget when migration_attempt_3` takes both.

### Forgetting

```
forget everything from phishing_email_88   # a source, and all it taught
forget when migration_attempt_3            # an episode, and all resting on it
forget alice.city                          # one name
```

### Time

The clock is virtual, so lifetimes and expiry are reproducible. Nothing in the
crate reads the wall clock.

```
now is 2026-09-04T12:00:00Z
later by 10 minutes
```

### Bindings, output, and assertions

```
let lease = what is gateway.ip
show "the lease is " + lease.age + " old"
expect what is alice.city is "Berlin"
```

`expect` is how the examples test themselves; a failure names the line.

### The resolution rule, in full

Every question resolves the same way. Gather every belief written under the name
in any enclosing scope, drop the withdrawn ones and any dated after the moment
being asked about, then:

1. **Highest standing wins.**
2. Among equals, the **most specific scope** wins.
3. Among equals, the **most recent** wins.

If two survivors claim the same explicitly stated moment, that is a
contradiction, and the question is refused. Then, and only then, the demands the
question made (`verified`, `fresh`, `trusted <tier>`) are applied to the winner.

Two consequences worth stating because they are the ones people get wrong.
Demands filter the *answer*, they do not search for a better one — `what is
verified x` refuses if the winning belief is unattributed, even when a weaker
attributable belief exists, because silently answering from a source you did not
choose is the behaviour this language exists to prevent. And beliefs written in
the same tick without an explicit date supersede in file order rather than
contradicting, because two lines in a document are a sequence, not a
simultaneous claim.

---

## Markdown brains

Palimpsest reads markdown, so a brain can be a git repo of pages that a human
reads and an agent maintains — the LLM Wiki and GBrain substrate, with
deterministic resolution underneath.

Fenced `pal` blocks are executed; the prose around them is ignored:

````markdown
---
source: hr_handbook_2026
authority: policy
---

# Employee Handbook 2026

Full-time employees accrue twenty days of paid leave per calendar year.

```pal
acme.pto.days is 20 as policy on 2026-01-01
acme.expenses.per_diem_eur is 75 as policy on 2026-01-01 for 1 year
```
````

**The page is the provenance.** Facts inherit the file path as their source, or
the `source:` frontmatter key if it names something stabler. So `forget
everything from hr_handbook_2026` means what it looks like, and withdrawing a
page withdraws its episodes too. Line numbers are preserved, so a parse error
points at the right line of the markdown.

`examples/brain/` is a four-page worked example — a handbook, a person page, an
incident report, and a page of questions — where the handbook and the person page
disagree about leave and the handbook wins across the file boundary.

```
$ palimpsest examples/brain --check
```

---

## Running it

```bash
cargo test                                # 67 tests
cargo run -- examples/moving.pal          # one program
cargo run -- examples/brain               # a whole markdown brain
cargo run -- examples/brain --check       # ... and audit it, non-zero on errors
cargo run -- -e 'x is 1 as rumor
what is x'
```

Pure Rust, no dependencies, stable toolchain. The crate is a library as well as a
binary; `palimpsest::run_quiet(source)` returns a `Runtime` you can query.

Why an interpreter rather than a compiler: the interesting semantics are all
about the state of a belief store at a moment, and there is no compile-time work
worth doing when every question is a query over data that arrived at runtime.
There is nothing to lower.

---

## The scenarios, and their real output

All seven run. Output below is verbatim.

### 1. A fact is superseded, and the old one is auditable

`cargo run -- examples/moving.pal`

```
Where does Alice live?
Berlin

Where did she live in April, before the move?
Lisbon

Both layers, and why Lisbon is no longer the answer:
history of alice.city (2 layers)
  #1 alice.city = "Lisbon"  [user via onboarding_form on 2026-03-01] -> overwritten by #2 on 2026-08-15
  #2 alice.city = "Berlin"  [user via relocation_ticket on 2026-08-15] -> current
```

### 2. A low-standing source cannot override a high-standing one, and the conflict is reported

`cargo run -- examples/authority.pal`

```
How many PTO days does Alice have?
20

The disagreement is recorded, not discarded:
1 conflict
  acme.alice.pto: user said 25 (via slack_thread_942) but policy outranks it and says 20 (via hr_handbook_2026)

A question that insists on a policy-grade answer still gets one:
20

Authority also beats scope depth, which is the unusual part:
eu-west-1
```

The last line is the sharp one: `region` was written in the innermost scope as
`user` and in the outer scope as `policy`, and the outer scope wins.

### 3. Retracting a source removes what it taught and falls back

`cargo run -- examples/forgetting.pal`

```
While the phishing email is trusted:
admin
no

Withdraw the source, in one line:

Everything it taught is gone, and the prior answer is back:
member

Facts from other sources are untouched:
billing

And the withdrawal is on the record:
history of alice.role (2 layers)
  #1 alice.role = "member"  [policy via corporate_ldap on 2026-01-10] -> current
  #3 alice.role = "admin"  [policy via phishing_email_88 on 2026-09-03] -> forgotten: source `phishing_email_88` was forgotten
```

### 4. An expired belief reports staleness rather than being served straight

`cargo run -- examples/lifetimes.pal`

```
Inside its lifetime, the lease answers normally:
10.0.0.1

Ten minutes pass.

An ordinary question still answers, but the answer says what it is:
STALE "10.0.0.1" (lived 10 minutes, allowed 5 minutes)
  stale?   yes
  value:   10.0.0.1
  age:     10 minutes

The tax rate expired months ago and nobody noticed until now:
checked 2 beliefs (2 live) and 0 episodes
  [warning] stale: #1 gateway.ip expired on 2026-09-04 12:05 and is 5 minutes past its lifetime
  [warning] stale: #2 tax.rate expired on 2026-01-01 and is about 8 months past its lifetime
  0 error(s), 2 warning(s)
```

### 5. A question that would rely on an unsourced belief is refused

`cargo run -- examples/provenance.pal`

```
An ordinary question will hand over the rumour:
tok_untrusted_999

A question that requires provenance will not:
  (the next line stops the program)

refused: deploy.token is only believed as rumor via anonymous_paste. The question asked for a verified answer, so nothing is returned.

This is a refusal, not a crash: the belief store could not answer
that question under the conditions the question set.
```

Exit code 1. The refusal comes from the resolution rule; there is no phrasing of
the question that returns the value.

### 6. Episodic memory, and something a vector store cannot do in one line

`cargo run -- examples/episodes.pal`

```
What is the state of the billing database?
degraded

...

The incident is resolved, so the episode is withdrawn:

The belief that rested on it went with it:
history of billing.db.status (1 layer)
  #1 billing.db.status = "degraded"  [compliance via pagerduty on 2026-09-04 12:00] -> forgotten: episode `migration_attempt_3` was forgotten

The unrelated belief stands, because it rests on a different episode:
400
```

`forget when migration_attempt_3` is the one line. To do this with embeddings you
would have to know which chunks were derived from the incident, delete them, and
then reconstruct what they had displaced — and that last step is impossible,
because nothing recorded a displacement.

### 7. `check` audits the belief store itself

`cargo run -- examples/check.pal`

```
Health of the brain:
checked 7 beliefs (7 live) and 0 episodes
  [warning] unsourced: #2 company.headcount is believed as policy but cites nothing; a question demanding `verified` will refuse it
  [error] orphaned: #7 company.status rests on episode `sec_inquiry`, which this brain has no record of
  [error] contested: company.fiscal_year_end holds "03-31" and "12-31" at equal standing (policy); no rule decides between them
  [warning] stale: #3 company.insurance_policy expired on 2026-01-01 and is about 8 months past its lifetime
  [note] refused: an override of company.name by user was rejected in favour of legal
  2 error(s), 2 warning(s)
```

This is the LLM Wiki `lint` operation with a decidable answer instead of a
model's opinion, and it is the clearest statement of what the language is for.
There is no query you can send a vector index that means "which of your contents
contradict each other," because the index has no notion of two chunks being about
the same fact.

---

## What is real and what is not

Real, tested, and running with no network access:

- Lexer, parser, and tree-walking interpreter in pure Rust, no dependencies.
- The three-axis resolution rule, over nested scopes, with as-of queries.
- Provenance, lifetimes, staleness as a distinct type, the four refusals.
- Cascading retraction by source, by episode, and by name, with fallback.
- Episodic memory and `because` grounding.
- `check` over the whole store.
- Markdown ingestion with the page as provenance.
- 67 tests; every example's `expect` assertions run in CI.

Deliberately out of scope for this version, and honestly so:

- **Extraction from prose.** Turning "I moved to Berlin last month" into
  `alice.city is "Berlin" on 2026-08-15` is a model's job, and it is the hard
  part. Explicit statements are the primary interface. A model that emits
  Palimpsest is the intended integration, and the language is deliberately
  easy to emit.
- **Derived beliefs and inference rules.** The truth maintenance here is a JTMS
  over one justification per belief (`because`), not a general dependency
  network. There are no rules, so nothing derives a belief from other beliefs.
  That is the largest missing piece and the most natural next thing to build.
- **Persistence.** The store is in memory; a brain is rebuilt from its markdown
  on each run. Fine at wiki scale — Karpathy puts that boundary around 50–100k
  tokens — and the wrong architecture past it. Retraction is a linear scan;
  differential dataflow is the known answer if it ever needs to matter.
- **Partial trust orders.** The order is total, on purpose. Two sibling tiers
  that cannot be compared are representable in a lattice and would make the
  question "which claim wins" require running the program.
- **Concurrency.** Single-threaded, sequential file ingestion. The concurrent
  ingest problem is real and unaddressed.

The one thing I would push back on in the framing that produced this: the demand
for something a vector store "genuinely cannot do cleanly" is easy to satisfy and
therefore not the most interesting bar. `forget everything from X` clears it in
one line. The harder and more useful question is whether the deterministic layer
is worth the loss of fuzzy recall, and the answer is that it is not, on its own —
which is why this is a substrate that sits under a wiki and beside a retrieval
index, not a replacement for either.
