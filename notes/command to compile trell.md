# Command to compile Trell

From the repo root:

    source ./env.sh
    cargo build

Compile one program to LLVM IR:

    cargo run -- examples/42.trell
    # writes out.ll

Compile and run on the host (prints `Result: <exit status>`):

    ./run-trell.sh examples/42.trell          # bash
    ./run-trell.zsh examples/42.trell         # zsh

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
