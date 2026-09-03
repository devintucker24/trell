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

`env.sh` (sourced by `env.zsh`) looks for, in order:

1. An already-set `LLVM_SYS_181_PREFIX`
2. Homebrew `llvm@18`, but only if `$prefix/bin/llvm-config` exists — macOS
3. `/usr/lib/llvm-18` — Ubuntu / Debian
4. `llvm-config-18` or `llvm-config` on `PATH`

macOS install (unchanged from the original workflow):

    brew install llvm@18 rustup
    rustup toolchain install 1.85.0
    source ./env.zsh
    cargo build

Ubuntu / Debian install:

    sudo apt-get install -y llvm-18-dev llvm-18 libpolly-18-dev libzstd-dev g++
    rustup toolchain install 1.85.0
    source ./env.sh
    cargo build

`LIBRARY_PATH` is adjusted for GCC `libstdc++` **only on Linux**. macOS keeps
Apple clang + libc++ and must not inherit a Homebrew g++ search path.

## Native backend

`cargo run -- examples/42.trell` only writes `out.ll`. `link-trell.sh` turns
that into `./trell-program`:

- **macOS:** `llc` (Homebrew LLVM) then `xcrun clang` (Xcode SDK). On Apple
  Silicon this still passes `-mtriple=arm64-apple-macosx15.2`, matching the
  original `run-trell.zsh`. Do not compile `out.ll` with Homebrew `clang`;
  that binary cannot see the Apple SDK.
- **Linux:** host `clang out.ll -o trell-program`.

`run-trell.zsh` / `run-trell.sh` call that helper and print the program's
exit status. That status *is* the value of the Trell expression (truncated
to 8 bits on Unix).
