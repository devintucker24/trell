# Trell

A small compiled language. Today it is one integer expression: `+`, `-`, `*`,
`/`, and parentheses, lowered to LLVM IR and then to a native `main` that
returns the result.

## Build

Needs **Rust 1.85+** (edition 2024) and **LLVM 18** (`llvm-config` on `PATH`).

```bash
# macOS
brew install llvm@18
rustup toolchain install 1.85.0

# Ubuntu / Debian
sudo apt-get install -y llvm-18-dev llvm-18 libpolly-18-dev libzstd-dev g++
rustup toolchain install 1.85.0

source ./env.sh
cargo build
```

`env.sh` sets `LLVM_SYS_181_PREFIX` from Homebrew `llvm@18` or `/usr/lib/llvm-18`.

## Compile a program

```bash
cargo run -- examples/42.trell    # writes out.ll
./run-trell.sh examples/42.trell  # IR → host binary → print Result
```

The process exit status is the value of the expression.

## Status

The compiler is a learning frontend (lex → parse → LLVM). It is not the
workflow / capability language described in
`docs/research/2026-09-03-agent-language-market.md`.
