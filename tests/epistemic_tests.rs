use std::path::Path;
use anyhow::Result;
use trell::ast::*;
use trell::interpreter::{Interpreter, MockOracle};
use trell::lexer::lex;
use trell::parser::Parser;
use trell::typecheck::TypeChecker;

fn parse_and_check(source: &str) -> Result<Program> {
    let tokens = lex(source)?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;
    let mut checker = TypeChecker::new();
    checker.check_program(&program)?;
    Ok(program)
}

#[test]
fn test_epistemic_type_safety_rejects_unverified_belief() {
    let source = r#"
        contract QuickOracle {
            model: fast;
        }

        fn main() {
            let b: belief<string> = oracle<QuickOracle>.ask("test");
            // ILLEGAL: Cannot assign belief<string> to certain string without verification!
            let c: certain string = b;
        }
    "#;

    let res = parse_and_check(source);
    assert!(res.is_err(), "Expected type checker to reject unverified belief assignment");
    let err_str = res.unwrap_err().to_string();
    assert!(err_str.contains("Type mismatch"), "Error message should mention type mismatch: {}", err_str);
}

#[test]
fn test_epistemic_verify_promotes_belief_to_certain() {
    let source = r#"
        contract OracleA {
            model: reasoning;
        }

        guard IsSafe(s: string) {
            s == "Clean"
        }

        fn main() {
            let b: belief<string> = oracle<OracleA>.assess("test");
            let c: certain string = verify b with IsSafe fallback "FallbackClean";
            assert c == "Clean" || c == "FallbackClean", "Verification should produce certain string";
        }
    "#;

    let program = parse_and_check(source).expect("Epistemic verification should pass type checker");
    let mut oracle = MockOracle::new();
    oracle.set_response("assess", "Clean", 0.95, "Grounded rationale");
    let mut interp = Interpreter::new(&program, Box::new(oracle));
    interp.run_main().expect("Execution should succeed");
}

#[test]
fn test_speculative_fork_and_branch_collapse() {
    let source = r#"
        contract Arbiter {
            model: reasoning;
        }

        fn main() {
            let hyp: belief<string> = oracle<Arbiter>.judge("Evaluate hypothesis");
            let outcome: int = 0;

            fork hyp {
                case HypothesisA => {
                    outcome = 100;
                }
                case HypothesisB => {
                    outcome = 200;
                }
                fallback => {
                    outcome = 999;
                }
            } collapse;

            assert outcome == 100, "Should commit matching speculative branch";
        }
    "#;

    let program = parse_and_check(source).expect("Fork and collapse should typecheck");
    let mut oracle = MockOracle::new();
    oracle.set_response("judge", "HypothesisA", 0.91, "Matches HypothesisA");
    let mut interp = Interpreter::new(&program, Box::new(oracle));
    interp.run_main().expect("Should execute speculative fork");

    assert_eq!(interp.traces.len(), 1);
    assert_eq!(interp.traces[0].chosen_branch, "HypothesisA");
    assert_eq!(interp.traces[0].rolled_back_branches, vec!["HypothesisB".to_string()]);
}

#[test]
fn test_semantic_consensus_voting() {
    let source = r#"
        contract Voter {
            model: fast;
        }

        fn main() {
            let verdict: belief<string> = consensus(3, 0.60) {
                oracle<Voter>.vote("Query")
            };
            let conf: float = confidence(verdict);
            assert conf > 0.5, "Consensus confidence must exceed threshold";
        }
    "#;

    let program = parse_and_check(source).expect("Consensus should typecheck");
    let mut oracle = MockOracle::new();
    oracle.set_response("vote", "AgreedConsensus", 0.90, "Voted in agreement");
    let mut interp = Interpreter::new(&program, Box::new(oracle));
    interp.run_main().expect("Consensus evaluation should succeed");
}

#[test]
fn test_model_contract_invariant_enforcement() {
    let source = r#"
        contract StrictContract {
            model: reasoning;
            invariant: confidence >= 0.95;
        }

        fn main() {
            let res: belief<string> = oracle<StrictContract>.assess("Test invariant");
        }
    "#;

    let program = parse_and_check(source).expect("Type check passes");
    let mut oracle = MockOracle::new();
    // Confidence 0.80 is below contract's 0.95 requirement!
    oracle.set_response("assess", "LowConfResult", 0.80, "Low certainty deliberation");
    let mut interp = Interpreter::new(&program, Box::new(oracle));
    let err = interp.run_main();
    assert!(err.is_err(), "Should fail when model confidence violates contract invariant");
    let msg = err.unwrap_err().to_string();
    assert!(msg.contains("invariant failed") || msg.contains("violates contract"), "Message: {}", msg);
}

#[test]
fn test_all_example_files_check_and_run() {
    let examples = [
        "examples/medical_diagnosis.trell",
        "examples/financial_settlement.trell",
        "examples/code_synth_guard.trell",
        "examples/deterministic_math.trell",
    ];

    for path_str in examples {
        let path = Path::new(path_str);
        let src = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read {}", path_str));
        let tokens = lex(&src).expect("Lex failed");
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program().expect("Parse failed");
        let mut checker = TypeChecker::new();
        checker.check_program(&program).expect("Typecheck failed");

        let mut interp = Interpreter::new(&program, Box::new(MockOracle::new()));
        interp.run_main().expect("Run main failed");
    }
}
