//! Tests for the native LLVM backend (certain-integer core).
//!
//! These both prove the generated code computes the correct values (via JIT
//! execution) and that the backend refuses to lower epistemic / non-integer
//! constructs, upholding the certain/belief boundary.

use std::path::PathBuf;

use trell::ast::Program;
use trell::lexer::lex;
use trell::llvm_backend::{compile_program_to_ir, jit_run_main};
use trell::parser::Parser;
use trell::typecheck::TypeChecker;

fn compile(source: &str) -> Program {
    let tokens = lex(source).expect("lexing failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("parsing failed");
    let mut checker = TypeChecker::new();
    checker.check_program(&program).expect("type check failed");
    program
}

fn run(source: &str) -> i64 {
    let program = compile(source);
    jit_run_main(&program).expect("native compilation / JIT failed")
}

#[test]
fn returns_integer_literal() {
    assert_eq!(run("fn main() -> int { return 42; }"), 42);
}

#[test]
fn trailing_expression_is_implicit_return() {
    assert_eq!(run("fn main() -> int { 6 * 7 }"), 42);
}

#[test]
fn arithmetic_precedence_matches_interpreter() {
    assert_eq!(run("fn main() -> int { 2 + 3 * 4 }"), 14);
    assert_eq!(run("fn main() -> int { (2 + 3) * 4 }"), 20);
}

#[test]
fn signed_division_and_modulo() {
    assert_eq!(run("fn main() -> int { 17 / 5 }"), 3);
    assert_eq!(run("fn main() -> int { 17 % 5 }"), 2);
    assert_eq!(run("fn main() -> int { 0 - 17 / 5 }"), -3);
}

#[test]
fn let_bindings_assignment_and_calls() {
    let source = r#"
        fn square(n: int) -> int {
            n * n
        }

        fn main() -> int {
            let a: int = 6;
            let b: int = 7;
            let base: int = a * b;
            base + square(3) - 9
        }
    "#;
    assert_eq!(run(source), 42);
}

#[test]
fn ir_defines_i64_main() {
    let program = compile("fn main() -> int { 1 + 1 }");
    let ir = compile_program_to_ir(&program).expect("IR generation failed");
    assert!(ir.contains("define i64 @main()"), "IR was:\n{ir}");
}

#[test]
fn rejects_non_integer_entry_point() {
    // `main` must be `fn main() -> int`; a bool entry point is not lowerable.
    let program = compile("fn main() -> bool { true }");
    let err = compile_program_to_ir(&program)
        .expect_err("expected non-int main to be rejected")
        .to_string();
    assert!(err.contains("main"), "unexpected error: {err}");
}

#[test]
fn rejects_side_effecting_statements() {
    // `print` has no sound integer lowering; the backend must refuse it.
    let source = r#"
        fn main() -> int {
            print(42);
            return 42;
        }
    "#;
    let program = compile(source);
    let err = compile_program_to_ir(&program)
        .expect_err("expected print to be rejected")
        .to_string();
    assert!(
        err.contains("print") || err.contains("not supported"),
        "unexpected error: {err}"
    );
}

#[test]
fn rejects_epistemic_program() {
    // A full epistemic example (oracles, forks, beliefs) must not lower to native
    // code — beliefs are not certainties.
    let path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "examples",
        "medical_diagnosis.trell",
    ]
    .iter()
    .collect();
    let source = std::fs::read_to_string(&path).expect("could not read example");
    let program = compile(&source);
    assert!(
        compile_program_to_ir(&program).is_err(),
        "epistemic program must not compile to native code"
    );
}
