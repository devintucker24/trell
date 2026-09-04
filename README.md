# Trell

Trell is a tiny language for **untrusted programs written by agents**.

It is not a Python framework and it does not try to replace LangChain. Python SDKs stay as drivers (model calls, tools). Trell owns **authority**: who may `ask`, which tools exist, whether a child agent may spawn, whether a human must approve, and whether the program is even legal.

The analog is Terraform, not LangChain. The file is the workflow. CI runs `trell check`. Invalid grants fail the build *before* an API call.

Today this repository ships that core:

1. A small grammar (`trell grammar` emits GBNF for constrained decoding).
2. A fail-closed checker (`trell check` / `trell plan`).
3. A mock runner so CI is free (`trell run --mock`).
4. Import-free Wasm for pure integer compute — a sandbox with no ambient imports.

What it does *not* ship yet: a Wasmtime spawn host, vendor SDK adapters, or a connector catalog. Those are next, not the identity of the language.

## A workflow

```trell
cap pr_review {
  allow read "src/**"
  allow ask
  allow send
  deny write
  deny net
  deny git
  need approve on write
  budget tokens 8000
  budget cents 25
  spawn 0
}

in diff: text

let review = ask "Classify the risk of this diff. Consider secrets, auth, and data deletion." using diff as {
  risk: enum(low, medium, high),
  reason: text
}

if review.risk == high {
  approve "high-risk change cannot proceed without a human"
}

send review
```

Missing `allow write` is a compile error. `need approve on write` means every path that writes must hit `approve` first. `ask` output is **tainted**: it cannot be fed to `spawn`. Prompt injection is information flow; the language marks values dirty.

## Pure compute (the sandbox wedge)

A Trell file can still be a hermetic integer program. Agents that need a fee schedule, a retry budget, or a rate limit should not `eval` Python. They should run Trell, which compiles to Wasm with **no imports**.

```trell
in items: int

let extra = items - 2
let uncapped = 499 + 50 * extra
if extra < 0 {
  499
} else {
  if uncapped > 1999 {
    1999
  } else {
    uncapped
  }
}
```

```
trell eval examples/fees.trell --set items=5
# 649
```

## Commands

```
trell check examples/pr-review.trell
trell plan  examples/pr-review.trell
trell run   examples/pr-review.trell --set diff="added a logout button" --ask '{"risk":"high","reason":"auth"}'
trell eval  examples/arithmetic.trell
trell compile examples/arithmetic.trell -o out.wasm
trell grammar > grammar/trell.gbnf
```

`trell plan` is the Terraform moment: grants, budgets, spawn ceiling, required approvals, and every effect the program can perform.

## Constrained decoding

`trell grammar` prints a GBNF grammar small enough to mask logits. A model can be forbidden from emitting invalid Trell. Capability errors remain the compiler’s job — syntax is constrained, authority is checked.

## Design rules

- **Fail closed.** Anything not granted does not exist.
- **Tiny grammar.** If you cannot constrain-decode it, it is too big.
- **Taint.** Model and tool output cannot become spawn source or a shell.
- **Approve is an effect.** Destructive work is a gated path, not a comment in a prompt.
- **Wasm is the jail.** Native binaries with host privileges are the wrong default for generated agents.

## Status

Trell started as an LLVM arithmetic toy. That gym is retired. The product path is: checker, mock CI, GBNF, import-free Wasm, then a capability host that runs spawned agents with no ambient authority.
