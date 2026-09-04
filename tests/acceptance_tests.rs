use palimpsest::error::PalimpsestError;
use palimpsest::run_source;
use palimpsest::time::{Duration, Timestamp};
use palimpsest::types::Value;

#[test]
fn test_scenario_1_superseding_and_audit() {
    let code = r#"
assert user.residence = "Lisbon" @ authority(User), source("chat_session_03"), at("2026-03-01T10:00:00Z");
assert user.residence = "Berlin" @ authority(User), source("chat_session_08"), at("2026-08-15T14:30:00Z");

let current = recall user.residence;
let past = recall as_of("2026-04-01T00:00:00Z") user.residence;
"#;
    let rt = run_source(code).expect("Should execute successfully");
    assert_eq!(rt.variables.get("current"), Some(&Value::String("Berlin".to_string())));
    assert_eq!(rt.variables.get("past"), Some(&Value::String("Lisbon".to_string())));

    let audit = rt.audit_path(&["user".to_string(), "residence".to_string()]);
    match audit {
        Value::AuditLog(entries) => {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].value, Value::String("Lisbon".to_string()));
            assert!(matches!(entries[0].status, palimpsest::types::AuditStatus::ShadowedBy { .. }));
            assert_eq!(entries[1].value, Value::String("Berlin".to_string()));
            assert!(matches!(entries[1].status, palimpsest::types::AuditStatus::Active));
        }
        _ => panic!("Expected AuditLog"),
    }
}

#[test]
fn test_scenario_2_authority_lattice_conflict() {
    let code = r#"
authority Compliance > Policy > User > Unverified;

assert employee.alice.pto_days = 20 @ authority(Policy), source("hr_handbook_2026"), at("2026-01-01T00:00:00Z");
assert employee.alice.pto_days = 25 @ authority(User), source("slack_chat_942"), at("2026-09-02T11:00:00Z");

let pto = recall employee.alice.pto_days;
"#;
    let rt = run_source(code).expect("Should execute successfully");
    assert_eq!(rt.variables.get("pto"), Some(&Value::Int(20)));
    assert_eq!(rt.conflict_log.len(), 1);
    let conflict = &rt.conflict_log[0];
    assert_eq!(conflict.path, "employee.alice.pto_days");
    assert_eq!(conflict.high_authority, "Policy");
    assert_eq!(conflict.low_authority, "User");
    assert_eq!(conflict.high_value, Value::Int(20));
    assert_eq!(conflict.low_value, Value::Int(25));
}

#[test]
fn test_scenario_3_retraction_cascade_and_fallback() {
    let code = r#"
assert user.alice.role = "member" @ authority(Policy), source("corporate_ldap"), at("2026-01-10T00:00:00Z");
assert user.alice.role = "admin" @ authority(Policy), source("phishing_email_88"), at("2026-09-03T09:00:00Z");

let before = recall user.alice.role;
retract source "phishing_email_88";
let after = recall user.alice.role;
"#;
    let rt = run_source(code).expect("Should execute successfully");
    assert_eq!(rt.variables.get("before"), Some(&Value::String("admin".to_string())));
    assert_eq!(rt.variables.get("after"), Some(&Value::String("member".to_string())));
}

#[test]
fn test_scenario_4_lifetimes_and_staleness() {
    let code_ok = r#"
set_time "2026-09-04T12:00:00Z";
assert infra.gateway.ip = "10.0.0.1" @ authority(Policy), source("dhcp_lease"), ttl(300s);

let ip_fresh = recall fresh infra.gateway.ip;
advance_time 600s;
let ip_stale = recall infra.gateway.ip;
let is_stale_flag = ip_stale.is_stale;
let stale_val = ip_stale.value;
let stale_age = ip_stale.age;
"#;
    let rt = run_source(code_ok).expect("Should execute successfully");
    assert_eq!(rt.variables.get("ip_fresh"), Some(&Value::String("10.0.0.1".to_string())));
    assert_eq!(rt.variables.get("is_stale_flag"), Some(&Value::Bool(true)));
    assert_eq!(rt.variables.get("stale_val"), Some(&Value::String("10.0.0.1".to_string())));
    assert_eq!(rt.variables.get("stale_age"), Some(&Value::Int(600)));

    // Verify 'recall fresh' fails on stale data
    let code_fail = r#"
set_time "2026-09-04T12:00:00Z";
assert infra.gateway.ip = "10.0.0.1" @ authority(Policy), source("dhcp_lease"), ttl(300s);
advance_time 600s;
let ip_err = recall fresh infra.gateway.ip;
"#;
    let err = run_source(code_fail).expect_err("Should fail with StaleBeliefError");
    match err {
        PalimpsestError::StaleBeliefError { path, age_secs, ttl_secs, .. } => {
            assert_eq!(path, "infra.gateway.ip");
            assert_eq!(age_secs, 600);
            assert_eq!(ttl_secs, 300);
        }
        other => panic!("Expected StaleBeliefError, got {:?}", other),
    }
}

#[test]
fn test_scenario_5_provenance_gatekeeping() {
    let code_fail = r#"
assert secrets.auth_token = "tok_untrusted_999" @ authority(Unverified), source("anonymous_paste"), unverified;
let tok = recall verified secrets.auth_token;
"#;
    let err = run_source(code_fail).expect_err("Should refuse unverified belief");
    match err {
        PalimpsestError::UnverifiedBeliefRefusal { path, authority, .. } => {
            assert_eq!(path, "secrets.auth_token");
            assert_eq!(authority, "Unverified");
        }
        other => panic!("Expected UnverifiedBeliefRefusal, got {:?}", other),
    }
}

#[test]
fn test_scenario_6_episodic_grounding_and_retraction() {
    let code = r#"
episode db_outage_01 {
    at: "2026-09-04T08:15:00Z",
    actors: ["deploy_bot", "alice"],
    context: { service: "billing-db", pool: 100 },
    summary: "Migration aborted: connection pool exhausted"
}

scope enterprise.acme {
    assert infra.db.status = "degraded" @ authority(Compliance), grounded_in("db_outage_01");
}

let status_before = recall enterprise.acme.infra.db.status;
retract episode db_outage_01;
"#;
    let rt = run_source(code).expect("Should execute successfully");
    assert_eq!(rt.variables.get("status_before"), Some(&Value::String("degraded".to_string())));

    // After retracting episode, resolving path should fail as unresolved because the only belief was grounded in it
    let res_after = rt.resolve_path(
        &["enterprise".to_string(), "acme".to_string(), "infra".to_string(), "db".to_string(), "status".to_string()],
        None,
        false,
        false,
        None,
    );
    assert!(res_after.is_err(), "Grounded belief must be retracted with episode");
}

#[test]
fn test_minimum_authority_refusal() {
    let code = r#"
authority Compliance > Policy > User > Unverified;
assert project.budget = 50000 @ authority(User), source("chat");
let budget = recall min_authority(Policy) project.budget;
"#;
    let err = run_source(code).expect_err("Should fail with InsufficientAuthorityError");
    match err {
        PalimpsestError::InsufficientAuthorityError { path, required_authority, actual_authority } => {
            assert_eq!(path, "project.budget");
            assert_eq!(required_authority, "Policy");
            assert_eq!(actual_authority, "User");
        }
        other => panic!("Expected InsufficientAuthorityError, got {:?}", other),
    }
}

#[test]
fn test_equal_authority_contradiction() {
    let code = r#"
authority Policy > User > Unverified;
assert config.region = "us-east-1" @ authority(Policy), at("2026-01-01T00:00:00Z");
assert config.region = "eu-central-1" @ authority(Policy), at("2026-01-01T00:00:00Z");
let r = recall config.region;
"#;
    let err = run_source(code).expect_err("Should detect equal authority contradiction");
    match err {
        PalimpsestError::ContradictionError { path, authority, .. } => {
            assert_eq!(path, "config.region");
            assert_eq!(authority, "Policy");
        }
        other => panic!("Expected ContradictionError, got {:?}", other),
    }
}

#[test]
fn test_time_and_duration_math() {
    let t = Timestamp::parse_iso("2026-03-01T12:00:00Z").unwrap();
    let d = Duration::parse_str("48h").unwrap();
    let t2 = t.add_duration(d);
    assert_eq!(t2.to_iso(), "2026-03-03T12:00:00Z");

    let d_month = Duration::parse_str("30d").unwrap();
    assert_eq!(d_month.as_secs(), 30 * 86400);
}
