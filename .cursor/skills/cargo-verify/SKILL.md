---
name: cargo-verify
description: Format, lint, and test the Trell Rust project. Use before finishing Rust/compiler/runtime changes or opening a PR that touches src/, tests/, or examples/.
---

# cargo-verify

Run from repo root:

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test
```

If clippy is too strict for an existing warning baseline, run `cargo test` at minimum and report clippy issues honestly.

When epistemic behavior changes, also run wiki maintain (`.cursor/skills/wiki-maintain/SKILL.md`).
