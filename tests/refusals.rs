//! The four refusals.
//!
//! Each one is a question the belief store declines to answer, decided by the
//! evaluation rule rather than by a prompt. The point of these tests is that
//! there is no phrasing that gets the value out.

use palimpsest::error::PalimpsestError;
use palimpsest::run_quiet;
use palimpsest::types::Value;

#[test]
fn a_verified_question_refuses_hearsay() {
    let err = run_quiet(
        r#"
        trust policy above user above rumor
        token is "tok_999" as rumor from anonymous_paste unverified
        let x = what is verified token
        "#,
    )
    .unwrap_err();

    match err {
        PalimpsestError::Unverified { path, authority, .. } => {
            assert_eq!(path, "token");
            assert_eq!(authority, "rumor");
        }
        other => panic!("expected a provenance refusal, got {:?}", other),
    }
}

#[test]
fn the_same_belief_answers_an_ordinary_question() {
    // The belief is reachable. It is the demand that changes the outcome,
    // which is what makes this a property of evaluation and not of storage.
    let rt = run_quiet(
        r#"
        trust policy above user above rumor
        token is "tok_999" as rumor from anonymous_paste unverified
        let x = what is token
        "#,
    )
    .unwrap();
    assert_eq!(rt.var("x"), Some(&Value::String("tok_999".into())));
}

#[test]
fn a_belief_with_no_source_is_not_verified() {
    let err = run_quiet(
        r#"
        trust policy above user
        headcount is 240 as policy
        let x = what is verified headcount
        "#,
    )
    .unwrap_err();
    assert_eq!(err.tag(), "unverified");
}

#[test]
fn a_sourced_belief_passes_the_verified_demand() {
    let rt = run_quiet(
        r#"
        trust policy above user
        headcount is 240 as policy from annual_filing
        let x = what is verified headcount
        "#,
    )
    .unwrap();
    assert_eq!(rt.var("x"), Some(&Value::Int(240)));
}

#[test]
fn a_fresh_question_refuses_an_expired_belief() {
    let err = run_quiet(
        r#"
        now is 2026-09-04T12:00:00Z
        trust policy above user
        ip is "10.0.0.1" as policy from dhcp for 5 minutes
        later by 10 minutes
        let x = what is fresh ip
        "#,
    )
    .unwrap_err();

    match err {
        PalimpsestError::Stale { path, over_by, .. } => {
            assert_eq!(path, "ip");
            assert_eq!(over_by.as_secs(), 300);
        }
        other => panic!("expected a staleness refusal, got {:?}", other),
    }
}

#[test]
fn an_ordinary_question_gets_a_value_that_admits_it_is_stale() {
    let rt = run_quiet(
        r#"
        now is 2026-09-04T12:00:00Z
        trust policy above user
        ip is "10.0.0.1" as policy from dhcp for 5 minutes
        later by 10 minutes
        let answer = what is ip
        let flag = answer.stale
        let inner = answer.value
        let age = answer.age
        "#,
    )
    .unwrap();

    assert!(rt.var("answer").unwrap().is_stale());
    assert_eq!(rt.var("flag"), Some(&Value::Bool(true)));
    assert_eq!(rt.var("inner"), Some(&Value::String("10.0.0.1".into())));

    let Some(Value::Duration(age)) = rt.var("age") else {
        panic!("expected a duration");
    };
    assert_eq!(age.as_secs(), 600);
}

#[test]
fn a_stale_value_is_not_equal_to_its_fresh_self() {
    // The wrapper is a different type on purpose: nothing downstream can
    // consume a stale reading as though it were current.
    let err = run_quiet(
        r#"
        now is 2026-09-04T12:00:00Z
        trust policy above user
        ip is "10.0.0.1" as policy from dhcp for 5 minutes
        later by 10 minutes
        expect what is ip is "10.0.0.1"
        "#,
    )
    .unwrap_err();
    assert_eq!(err.tag(), "expectation-failed");
}

#[test]
fn a_belief_inside_its_lifetime_is_not_stale() {
    let rt = run_quiet(
        r#"
        now is 2026-09-04T12:00:00Z
        trust policy above user
        ip is "10.0.0.1" as policy from dhcp for 1 hour
        later by 10 minutes
        let x = what is fresh ip
        "#,
    )
    .unwrap();
    assert_eq!(rt.var("x"), Some(&Value::String("10.0.0.1".into())));
}

#[test]
fn a_question_can_demand_a_minimum_standing() {
    let err = run_quiet(
        r#"
        trust legal above policy above user
        budget is 50000 as user from chat
        let x = what is trusted policy budget
        "#,
    )
    .unwrap_err();

    match err {
        PalimpsestError::Untrusted { required, actual, .. } => {
            assert_eq!(required, "policy");
            assert_eq!(actual, "user");
        }
        other => panic!("expected a standing refusal, got {:?}", other),
    }
}

#[test]
fn a_high_standing_belief_satisfies_the_demand() {
    let rt = run_quiet(
        r#"
        trust legal above policy above user
        budget is 50000 as legal from board_minutes
        let x = what is trusted policy budget
        "#,
    )
    .unwrap();
    assert_eq!(rt.var("x"), Some(&Value::Int(50000)));
}

#[test]
fn equal_standing_on_the_same_day_is_a_contradiction_not_a_coin_toss() {
    let err = run_quiet(
        r#"
        trust policy above user
        region is "us-east-1" as policy from doc_a on 2026-01-01
        region is "eu-west-1" as policy from doc_b on 2026-01-01
        let x = what is region
        "#,
    )
    .unwrap_err();

    match err {
        PalimpsestError::Contested { path, values, .. } => {
            assert_eq!(path, "region");
            assert_eq!(values.len(), 2);
        }
        other => panic!("expected a contradiction, got {:?}", other),
    }
}

#[test]
fn undated_writes_in_the_same_tick_supersede_rather_than_contradict() {
    // Two lines in one file are a sequence, not a simultaneous claim.
    let rt = run_quiet(
        r#"
        trust policy above user
        region is "us-east-1" as policy from doc_a
        region is "eu-west-1" as policy from doc_a
        let x = what is region
        "#,
    )
    .unwrap();
    assert_eq!(rt.var("x"), Some(&Value::String("eu-west-1".into())));
}

#[test]
fn all_four_refusals_are_marked_as_refusals() {
    for source in [
        "trust p above r\nx is 1 as r from s unverified\nlet y = what is verified x",
        "now is 2026-01-01\ntrust p above r\nx is 1 as p from s for 1 minute\nlater by 1 hour\nlet y = what is fresh x",
        "trust p above r\nx is 1 as r from s\nlet y = what is trusted p x",
        "trust p above r\nx is 1 as p from a on 2026-01-01\nx is 2 as p from b on 2026-01-01\nlet y = what is x",
    ] {
        let err = run_quiet(source).unwrap_err();
        assert!(
            err.is_refusal(),
            "{} should be a refusal, not a crash",
            err.tag()
        );
    }
}
