# Command to compile Trell

## macOS (zsh)

    source ./env.zsh
    cargo build
    cargo run -- examples/42.trell          # writes out.ll
    ./run-trell.zsh examples/42.trell       # llc + xcrun clang, print Result

## Linux (bash)

    source ./env.sh
    cargo build
    cargo run -- examples/42.trell
    ./run-trell.sh examples/42.trell

Expected results for the examples:

| File | Expression | Result |
| --- | --- | --- |
| `examples/42.trell` | `42` | 42 |
| `examples/arithmetic.trell` | `20 + 22 * 2` | 64 |
| `examples/precedence.trell` | `20 + 22 * 2` | 64 |
| `examples/parentheses.trell` | `(20 + 22) * 2` | 84 |
| `examples/subtraction.trell` | `100 - 10 * 3` | 70 |
| `examples/division.trell` | `100 / 4 + 3` | 28 |

If `cargo` complains about `edition2024`, the active toolchain is older than
1.85. Run `rustup show` and confirm `rust-toolchain.toml` is being picked up.
