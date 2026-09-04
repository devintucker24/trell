use palimpsest::error::PalimpsestError;
use palimpsest::run_source;
use palimpsest::types::Value;

#[test]
fn test_nested_scopes() {
    let code = r#"
scope enterprise {
    assert domain = "corp.internal" @ authority(Policy);
    scope team_infra {
        assert cluster = "prod-east" @ authority(Policy);
        assert domain = "infra.corp.internal" @ authority(Policy);
    }
}

let outer_domain = recall enterprise.domain;
let inner_domain = recall enterprise.team_infra.domain;
let inner_cluster = recall enterprise.team_infra.cluster;
"#;
    let rt = run_source(code).expect("Should run nested scopes");
    assert_eq!(rt.variables.get("outer_domain"), Some(&Value::String("corp.internal".to_string())));
    assert_eq!(rt.variables.get("inner_domain"), Some(&Value::String("infra.corp.internal".to_string())));
    assert_eq!(rt.variables.get("inner_cluster"), Some(&Value::String("prod-east".to_string())));
}

#[test]
fn test_multi_step_retraction_cascade() {
    let code = r#"
assert config.db_url = "postgres://primary:5432" @ authority(Policy), source("seed_config");
assert config.db_url = "postgres://staging:5432" @ authority(Policy), source("staging_patch");
assert config.db_url = "postgres://attacker:5432" @ authority(Policy), source("malicious_pr");

let v1 = recall config.db_url;
retract source "malicious_pr";
let v2 = recall config.db_url;
retract source "staging_patch";
let v3 = recall config.db_url;
"#;
    let rt = run_source(code).expect("Should run cascade");
    assert_eq!(rt.variables.get("v1"), Some(&Value::String("postgres://attacker:5432".to_string())));
    assert_eq!(rt.variables.get("v2"), Some(&Value::String("postgres://staging:5432".to_string())));
    assert_eq!(rt.variables.get("v3"), Some(&Value::String("postgres://primary:5432".to_string())));
}

#[test]
fn test_records_and_expressions() {
    let code = r#"
let meta = { status: "healthy", code: 200, active: true };
let code_num = meta.code;
let is_active = meta.active;
let sum = 10 + 20 * 2;
"#;
    let rt = run_source(code).expect("Should evaluate records and expressions");
    assert_eq!(rt.variables.get("code_num"), Some(&Value::Int(200)));
    assert_eq!(rt.variables.get("is_active"), Some(&Value::Bool(true)));
    assert_eq!(rt.variables.get("sum"), Some(&Value::Int(50)));
}

#[test]
fn test_syntax_errors() {
    let code_bad = "assert = 123;";
    let err = run_source(code_bad);
    assert!(err.is_err());
    match err.unwrap_err() {
        PalimpsestError::ParseError { line, column, .. } => {
            assert_eq!(line, 1);
            assert_eq!(column, 8);
        }
        other => panic!("Expected ParseError, got {:?}", other),
    }
}
