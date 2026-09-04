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
        "examples/autonomous_ship.trell",
        "examples/bank_transfer.trell",
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

#[test]
fn test_natural_trell_maritime_navigation() {
    let source = r#"
model LookoutAI:
    temperature: 0.1
    budget: 1500
    require: confidence >= 0.85
end

guard ClearWaterway(action: string):
    action == "HoldCourse" or action == "VeerStarboard" or action == "ThrottleDown"
end

action main:
    print "Scanning autonomous maritime radar sector..."

    let obstacle_assessment: belief<string> = ask LookoutAI("Container vessel detected bearing 045 relative, range 1.2 nautical miles")

    let conf = confidence obstacle_assessment
    print "Assessed collision probability confidence:"
    print conf

    let safe_action: certain string = verify obstacle_assessment with ClearWaterway fallback "ThrottleDown"

    when safe_action is:
        case VeerStarboard:
            print "Helm: Rudder starboard 15 degrees. Passing astern."
        case ThrottleDown:
            print "Engine: Reversing screw to half astern."
        else:
            print "Helm: Steady as she goes."
    end

    print "Collision avoidance maneuver verified and executed."
end
    "#;

    let program = parse_and_check(source).expect("Natural Trell maritime navigation should parse and typecheck");
    let mut oracle = MockOracle::new();
    oracle.set_response("ask", "VeerStarboard", 0.94, "COLREGs Rule 14 Head-on situation: Alter course to starboard");
    let mut interp = Interpreter::new(&program, Box::new(oracle));
    interp.run_main().expect("Execution should succeed");

    assert_eq!(interp.traces.len(), 1);
    assert_eq!(interp.traces[0].chosen_branch, "VeerStarboard");
}

#[test]
fn test_natural_trell_quorum_consensus_transfer() {
    let source = r#"
model FraudOracle:
    budget: 800
    require: confidence >= 0.80
end

guard ApprovedSettlement(verdict: string):
    verdict == "ClearWire" or verdict == "EscrowHold"
end

action main:
    print "Validating institutional wire dispatch..."

    let consensus_verdict: belief<string> = quorum(3, 0.70):
        ask FraudOracle("High-speed interbank wire $1,250,000 to offshore clearing agency")
    end

    let verified_decision: certain string = require consensus_verdict with ApprovedSettlement else "EscrowHold"

    when verified_decision is:
        case ClearWire:
            print "Dispatch: SWIFT MT103 authenticated and transmitted."
        else:
            print "Compliance: Diverting transfer to 24-hour escrow hold."
    end

    print "Consensus transaction complete."
end
    "#;

    let program = parse_and_check(source).expect("Natural Trell quorum consensus should parse and typecheck");
    let mut oracle = MockOracle::new();
    oracle.set_response("ask", "ClearWire", 0.92, "Consensus quorum verified low-risk velocity profile");
    let mut interp = Interpreter::new(&program, Box::new(oracle));
    interp.run_main().expect("Execution should succeed");

    assert_eq!(interp.traces.len(), 1);
    assert_eq!(interp.traces[0].chosen_branch, "ClearWire");
}
