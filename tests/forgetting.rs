//! Forgetting, and the `check` pass.
//!
//! Withdrawing a source has to do two things a delete cannot: remove
//! everything the source taught, and restore what it displaced.

use palimpsest::run_quiet;
use palimpsest::types::{Finding, Value};

#[test]
fn withdrawing_a_source_restores_the_previous_answer() {
    let rt = run_quiet(
        r#"
        trust policy above user
        alice.role is "member" as policy from ldap on 2026-01-10
        alice.role is "admin" as policy from phishing on 2026-09-03
        let before = what is alice.role
        forget everything from phishing
        let after = what is alice.role
        "#,
    )
    .unwrap();

    assert_eq!(rt.var("before"), Some(&Value::String("admin".into())));
    assert_eq!(rt.var("after"), Some(&Value::String("member".into())));
}

#[test]
fn withdrawing_a_source_leaves_other_sources_alone() {
    let rt = run_quiet(
        r#"
        trust policy above user
        alice.role is "member" as policy from ldap
        alice.team is "billing" as policy from ldap
        alice.role is "admin" as policy from phishing
        forget everything from phishing
        let role = what is alice.role
        let team = what is alice.team
        "#,
    )
    .unwrap();

    assert_eq!(rt.var("role"), Some(&Value::String("member".into())));
    assert_eq!(rt.var("team"), Some(&Value::String("billing".into())));
}

#[test]
fn withdrawals_unwind_in_order_through_several_layers() {
    let rt = run_quiet(
        r#"
        trust policy above user
        db.url is "primary" as policy from seed
        db.url is "staging" as policy from patch
        db.url is "attacker" as policy from malicious_pr
        let a = what is db.url
        forget everything from malicious_pr
        let b = what is db.url
        forget everything from patch
        let c = what is db.url
        "#,
    )
    .unwrap();

    assert_eq!(rt.var("a"), Some(&Value::String("attacker".into())));
    assert_eq!(rt.var("b"), Some(&Value::String("staging".into())));
    assert_eq!(rt.var("c"), Some(&Value::String("primary".into())));
}

#[test]
fn a_withdrawal_is_recorded_rather_than_erased() {
    let rt = run_quiet(
        r#"
        trust policy above user
        alice.role is "member" as policy from ldap
        alice.role is "admin" as policy from phishing
        forget everything from phishing
        let trail = why alice.role
        "#,
    )
    .unwrap();

    let rendered = rt.var("trail").unwrap().to_string();
    assert!(rendered.contains("admin"), "the withdrawn layer is still visible");
    assert!(rendered.contains("forgotten"));
    assert!(rendered.contains("phishing"));
}

#[test]
fn withdrawing_an_episode_withdraws_what_rested_on_it() {
    let rt = run_quiet(
        r#"
        trust compliance above policy above user

        when outage:
            happened on 2026-09-04T08:15
            involved deploy_bot
            summary "pool exhausted"

        db.status is "degraded" as compliance from pagerduty because outage
        db.owner is "billing" as compliance from pagerduty

        let before = what is db.status
        forget when outage
        let owner = what is db.owner
        "#,
    )
    .unwrap();

    assert_eq!(rt.var("before"), Some(&Value::String("degraded".into())));
    assert_eq!(
        rt.var("owner"),
        Some(&Value::String("billing".into())),
        "only the belief that named the episode should go"
    );

    let err = run_quiet(
        r#"
        trust compliance above user
        when outage:
            happened on 2026-09-04T08:15
            summary "pool exhausted"
        db.status is "degraded" as compliance from pagerduty because outage
        forget when outage
        let after = what is db.status
        "#,
    )
    .unwrap_err();
    assert_eq!(err.tag(), "unknown");
}

#[test]
fn a_name_can_be_withdrawn_directly() {
    let err = run_quiet(
        r#"
        trust policy above user
        secret is "hunter2" as policy from vault
        forget secret
        let x = what is secret
        "#,
    )
    .unwrap_err();
    assert_eq!(err.tag(), "unknown");
}

// ---- check ------------------------------------------------------------

fn findings(source: &str) -> Vec<Finding> {
    run_quiet(source).unwrap().check().findings
}

#[test]
fn check_reports_beliefs_that_cite_nothing() {
    let found = findings(
        r#"
        trust policy above user
        headcount is 240 as policy
        name is "Acme" as policy from certificate
        "#,
    );

    let unsourced: Vec<&Finding> = found
        .iter()
        .filter(|f| matches!(f, Finding::Unsourced { .. }))
        .collect();

    assert_eq!(unsourced.len(), 1);
    assert!(format!("{}", unsourced[0]).contains("headcount"));
}

#[test]
fn check_reports_expired_beliefs() {
    let found = findings(
        r#"
        now is 2026-09-04
        trust policy above user
        rate is 19 as policy from bulletin on 2025-01-01 for 1 year
        "#,
    );

    assert!(
        found.iter().any(|f| matches!(f, Finding::Stale { .. })),
        "an expired belief should be reported before somebody relies on it"
    );
}

#[test]
fn check_reports_contradictions() {
    let found = findings(
        r#"
        trust policy above user
        year_end is "12-31" as policy from handbook on 2026-02-01
        year_end is "03-31" as policy from tax_filing on 2026-02-01
        "#,
    );

    let contested: Vec<&Finding> = found
        .iter()
        .filter(|f| matches!(f, Finding::Contested { .. }))
        .collect();

    assert_eq!(contested.len(), 1);
    assert_eq!(contested[0].severity(), "error");
}

#[test]
fn check_reports_beliefs_resting_on_episodes_that_do_not_exist() {
    let found = findings(
        r#"
        trust compliance above user
        status is "under_review" as compliance from letter because sec_inquiry
        "#,
    );

    assert!(found.iter().any(|f| matches!(f, Finding::Orphaned { .. })));
}

#[test]
fn a_healthy_brain_reports_nothing() {
    let report = run_quiet(
        r#"
        now is 2026-09-04
        trust policy above user
        name is "Acme" as policy from certificate
        region is "eu-west-1" as policy from infra_standard
        "#,
    )
    .unwrap()
    .check();

    assert!(report.is_healthy(), "unexpected findings: {:?}", report.findings);
    assert_eq!(report.live_beliefs, 2);
}

#[test]
fn check_counts_errors_separately_from_warnings() {
    let report = run_quiet(
        r#"
        now is 2026-09-04
        trust policy above user
        unsourced_one is 1 as policy
        clash is "a" as policy from x on 2026-01-01
        clash is "b" as policy from y on 2026-01-01
        "#,
    )
    .unwrap()
    .check();

    assert_eq!(report.errors(), 1, "the contradiction is an error");
    assert_eq!(report.warnings(), 1, "the missing source is a warning");
}
