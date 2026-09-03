# Toolchain arrangement

Trell is a Rust compiler that emits LLVM IR through inkwell (`llvm18-1`).
Building the compiler therefore needs both a new-enough Rust and an LLVM 18
development install that provides `llvm-config`.

## Rust

`Cargo.toml` uses edition 2024 and `rust-version = "1.85"`.
`rust-toolchain.toml` pins the repo to Rust 1.85.0 so `cargo` picks it
automatically via rustup.

Edition 2024 is not available on Cargo 1.83 and earlier.

## LLVM 18

inkwell 0.6 with the `llvm18-1` feature talks to `llvm-sys` 181. That crate
expects `LLVM_SYS_181_PREFIX` to point at an LLVM 18 prefix, and `llvm-config`
on `PATH`.

`env.sh` (sourced by `env.zsh` from the repo root) looks for, in order:

1. An already-set `LLVM_SYS_181_PREFIX`
2. Homebrew `llvm@18` (`brew --prefix llvm@18`) — macOS
3. `/usr/lib/llvm-18` — Ubuntu / Debian
4. `llvm-config-18` or `llvm-config` on `PATH`

macOS install:

    brew install llvm@18 rustup
    rustup toolchain install 1.85.0

Ubuntu / Debian install:

    sudo apt-get install -y llvm-18-dev llvm-18 libpolly-18-dev libzstd-dev g++
    rustup toolchain install 1.85.0

Then:

    source ./env.sh   # or ./env.zsh from zsh
    cargo build

## Native backend

`cargo run -- examples/42.trell` only writes `out.ll`. Turning that into a
host binary is a second step:

- Preferred: `clang out.ll -o trell-program`
- Fallback: `llc -filetype=obj out.ll -o trell-program.o` then `cc` to link

The old `run-trell.zsh` hardcoded `arm64-apple-macosx15.2` and `xcrun clang`.
That is the author's Mac. Linux (and any other host) should let `llc`/`clang`
use the default triple.

`run-trell.sh` and `run-trell.zsh` now compile for the host and print the
program's exit status. That status *is* the value of the Trell expression
(truncated to 8 bits on Unix).
