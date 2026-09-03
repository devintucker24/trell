# Trell

A small compiled language. Today it is one integer expression: `+`, `-`, `*`,
`/`, and parentheses, lowered to LLVM IR and then to a native `main` that
returns the result.

## Build

Needs **Rust 1.85+** (edition 2024) and **LLVM 18** (`llvm-config` on `PATH`).

### macOS

Same tools as before: Homebrew `llvm@18` and Xcode `clang` via `xcrun`.

```zsh
brew install llvm@18
rustup toolchain install 1.85.0

source ./env.zsh
cargo build
./run-trell.zsh examples/42.trell
```

`env.zsh` still points `LLVM_SYS_181_PREFIX` at `$(brew --prefix llvm@18)`.
Native binaries are still `llc` → `trell-program.o` → `xcrun clang`. Homebrew
clang is not used to link, because it does not see the Apple SDK.

### Ubuntu / Debian

```bash
sudo apt-get install -y llvm-18-dev llvm-18 libpolly-18-dev libzstd-dev g++
rustup toolchain install 1.85.0

source ./env.sh
cargo build
./run-trell.sh examples/42.trell
```

## Compile a program

```bash
cargo run -- examples/42.trell    # writes out.ll
```

The process exit status is the value of the expression.

## Status

The compiler is a learning frontend (lex → parse → LLVM). It is not the
workflow / capability language described in
`docs/research/2026-09-03-agent-language-market.md`.
