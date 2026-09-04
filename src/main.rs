// Palimpsest CLI & Scenario Runner

use std::env;
use std::fs;
use std::path::Path;
use std::process;

use palimpsest::error::PalimpsestError;
use palimpsest::lexer::Lexer;
use palimpsest::parser::Parser;
use palimpsest::runtime::Runtime;

fn print_usage() {
    println!(
        r#"Palimpsest: The Epistemic Memory Language for AI Systems

USAGE:
    palimpsest <file.pal>             Run a Palimpsest source file
    palimpsest -e "<code>"            Evaluate Palimpsest code string
    palimpsest --scenario <1-6>       Run and demonstrate an acceptance scenario
    palimpsest --scenarios            Run all 6 acceptance scenarios
    palimpsest --help                 Print this help message

ACCEPTANCE SCENARIOS:
    1: Fact superseding with auditable history (not silently retrieved)
    2: Authority lattice conflict detection (low-authority cannot override high)
    3: Truth Maintenance (retracting a source cascades and falls back)
    4: Staleness and TTL degradation (reported as stale, not served straight)
    5: Provenance gatekeeping (queries refusing unverified/unsourced beliefs)
    6: Expressive epistemic queries (capabilities beyond vector stores)
"#
    );
}

fn run_code(code: &str, file_name: &str) -> Result<(), PalimpsestError> {
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;
    let mut runtime = Runtime::new();

    println!(">>> Running Palimpsest program: {}", file_name);
    runtime.execute_program(&program)?;

    // If there were conflicts detected during execution, display them
    if !runtime.conflict_log.is_empty() {
        println!("\n=== Recorded Epistemic Conflicts ===");
        for conflict in &runtime.conflict_log {
            println!("  {}", conflict);
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        print_usage();
        return;
    }

    if args[0] == "-e" {
        if args.len() < 2 {
            eprintln!("Error: -e requires a code string");
            process::exit(1);
        }
        if let Err(e) = run_code(&args[1], "<eval>") {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
        return;
    }

    if args[0] == "--scenarios" {
        for s in 1..=6 {
            println!("\n================================================================================");
            println!("  RUNNING ACCEPTANCE SCENARIO {}", s);
            println!("================================================================================\n");
            run_scenario(s);
        }
        return;
    }

    if args[0] == "--scenario" || args[0] == "-s" {
        if args.len() < 2 {
            eprintln!("Error: --scenario requires a number (1-6)");
            process::exit(1);
        }
        let num: usize = match args[1].parse() {
            Ok(n) if (1..=6).contains(&n) => n,
            _ => {
                eprintln!("Error: Scenario number must be between 1 and 6");
                process::exit(1);
            }
        };
        run_scenario(num);
        return;
    }

    // Otherwise, treat first arg as file path
    let file_path = &args[0];
    let content = match fs::read_to_string(Path::new(file_path)) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            process::exit(1);
        }
    };

    if let Err(e) = run_code(&content, file_path) {
        eprintln!("Execution error: {}", e);
        process::exit(1);
    }
}

fn run_scenario(scenario: usize) {
    match scenario {
        1 => {
            println!("--- Scenario 1: Fact Superseding and Auditability ---");
            println!("Description: A user moves from Lisbon to Berlin. The current belief is Berlin,");
            println!("             but Lisbon remains indelibly in the audit trail rather than silently");
            println!("             lost or confusingly retrieved together.\n");
            let code = r#"
assert user.residence = "Lisbon" @ authority(User), source("chat_session_03"), at("2026-03-01T10:00:00Z");
assert user.residence = "Berlin" @ authority(User), source("chat_session_08"), at("2026-08-15T14:30:00Z");

print "Current recalled residence:";
let current = recall user.residence;
print current;
assert_eq current, "Berlin";

print "\nTime-travel query: What did the agent believe as of April 2026?";
let past = recall as_of("2026-04-01T00:00:00Z") user.residence;
print past;
assert_eq past, "Lisbon";

print "\nComplete audit trail for user.residence:";
let trail = history user.residence;
print trail;
"#;
            if let Err(e) = run_code(code, "scenario_1.pal") {
                eprintln!("Scenario 1 failed: {}", e);
            }
        }

        2 => {
            println!("--- Scenario 2: Epistemic Authority Lattice & Conflict Reporting ---");
            println!("Description: A user claims their PTO is 25 days. The company handbook specifies 20.");
            println!("             Because Policy > User in the authority lattice, the user's claim");
            println!("             cannot override the policy even though it is newer, and the conflict");
            println!("             is surfaced explicitly instead of swallowed.\n");
            let code = r#"
// Define authority hierarchy: Policy > User > Unverified
authority Policy > User > Unverified;

// Policy asserted first
assert employee.alice.pto_days = 20 @ authority(Policy), source("hr_handbook_2026"), at("2026-01-01T00:00:00Z");

// User later claims 25 days in chat
assert employee.alice.pto_days = 25 @ authority(User), source("slack_chat_942"), at("2026-09-02T11:00:00Z");

print "Recalled PTO days (Policy must prevail over User):";
let pto = recall employee.alice.pto_days;
print pto;
assert_eq pto, 20;

print "\nDefeasance conflicts detected:";
print conflicts;
"#;
            if let Err(e) = run_code(code, "scenario_2.pal") {
                eprintln!("Scenario 2 failed: {}", e);
            }
        }

        3 => {
            println!("--- Scenario 3: Truth Maintenance (Retraction Cascade & Fallback) ---");
            println!("Description: Alice is initially recorded as 'member'. A phishing email claims Alice");
            println!("             is 'admin'. Retracting the phishing email cascades through dependent");
            println!("             beliefs and deterministically falls back to 'member'.\n");
            let code = r#"
// Base role from corporate directory
assert user.alice.role = "member" @ authority(Policy), source("corporate_ldap"), at("2026-01-10T00:00:00Z");

// Fraudulent update
assert user.alice.role = "admin" @ authority(Policy), source("phishing_email_88"), at("2026-09-03T09:00:00Z");

print "Role before retraction (phishing email active):";
let role_before = recall user.alice.role;
print role_before;
assert_eq role_before, "admin";

print "\nExecuting: retract source \"phishing_email_88\";";
retract source "phishing_email_88";

print "\nRole after retraction (TMS falls back to corporate directory):";
let role_after = recall user.alice.role;
print role_after;
assert_eq role_after, "member";

print "\nAudit log reflecting the retraction:";
print audit user.alice.role;
"#;
            if let Err(e) = run_code(code, "scenario_3.pal") {
                eprintln!("Scenario 3 failed: {}", e);
            }
        }

        4 => {
            println!("--- Scenario 4: Lifetimes, Staleness, and Expiry ---");
            println!("Description: A server DNS cache entry has a TTL of 300 seconds. When queried after");
            println!("             expiry, standard recall returns a Stale wrapper object, and 'recall fresh'");
            println!("             refuses resolution with a StaleBeliefError.\n");
            let code = r#"
set_time "2026-09-04T12:00:00Z";

assert infra.gateway.ip = "10.0.0.1" @ authority(Policy), source("dhcp_lease"), ttl(300s);

print "At t=0s, IP is valid:";
let ip_fresh = recall fresh infra.gateway.ip;
print ip_fresh;
assert_eq ip_fresh, "10.0.0.1";

print "\nAdvancing time by 10 minutes (600s)...";
advance_time 600s;

print "\nStandard recall after TTL returns Stale descriptor:";
let ip_stale = recall infra.gateway.ip;
print ip_stale;
assert_eq ip_stale.is_stale, true;
assert_eq ip_stale.value, "10.0.0.1";
assert_eq ip_stale.age, 600;

print "\nAudit log showing expired status:";
print audit infra.gateway.ip;
"#;
            if let Err(e) = run_code(code, "scenario_4.pal") {
                eprintln!("Scenario 4 failed: {}", e);
            }
        }

        5 => {
            println!("--- Scenario 5: Epistemic Provenance Gatekeeping ---");
            println!("Description: An unverified rumor claims an API key. A query requiring verified");
            println!("             provenance refuses to resolve it by the language's evaluation semantics,");
            println!("             preventing hallucinated or poisoned data retrieval.\n");
            let code = r#"
// Untrusted anonymous leak
assert secrets.auth_token = "tok_untrusted_999" @ authority(Unverified), source("anonymous_paste"), unverified;

print "Audit log showing unverified inscription:";
print audit secrets.auth_token;

print "\nAttempting 'recall verified secrets.auth_token' - Should be REFUSED:";
"#;
            if let Err(e) = run_code(code, "scenario_5.pal") {
                eprintln!("Scenario 5 setup failed: {}", e);
            }

            // Test the refusal directly
            let refusal_code = r#"
assert secrets.auth_token = "tok_untrusted_999" @ authority(Unverified), source("anonymous_paste"), unverified;
let res = recall verified secrets.auth_token;
"#;
            match palimpsest::run_source(refusal_code) {
                Ok(_) => eprintln!("Error: Provenance check did not refuse unverified belief!"),
                Err(e) => {
                    println!("\nExpected Epistemic Refusal caught successfully:");
                    println!("  {}", e);
                }
            }
        }

        6 => {
            println!("--- Scenario 6: Expressive Epistemic Memory vs Vector Stores ---");
            println!("Description: Grounding semantic facts in episodic memory, multi-hop scoping,");
            println!("             and causal retraction. Vector stores cannot track causality or scope fallback.\n");
            let code = r#"
// 1. Record an episodic event
episode db_outage_01 {
    at: "2026-09-04T08:15:00Z",
    actors: ["deploy_bot", "alice"],
    context: { service: "billing-db", pool: 100 },
    summary: "Migration aborted: connection pool exhausted"
}

// 2. Ground semantic beliefs in that episode
scope enterprise.acme {
    assert infra.db.status = "degraded" @ authority(Compliance), grounded_in("db_outage_01");
    assert infra.db.fallback_host = "replica-02" @ authority(Policy);
}

print "Recalled degraded status grounded in episode:";
let status = recall enterprise.acme.infra.db.status;
print status;
assert_eq status, "degraded";

print "\nActive episodes in memory:";
print episodes;

print "\nRetracting episode 'db_outage_01' (issue resolved)...";
retract episode db_outage_01;

print "\nAudit log confirms retraction of grounded belief:";
print audit enterprise.acme.infra.db.status;
"#;
            if let Err(e) = run_code(code, "scenario_6.pal") {
                eprintln!("Scenario 6 failed: {}", e);
            }
        }

        _ => unreachable!(),
    }
}
