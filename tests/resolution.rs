//! The resolution rule.
//!
//! Authority, then specificity, then recency — in that order, always. These
//! tests pin the order down, including the cases where the axes disagree.

use palimpsest::run_quiet;
use palimpsest::types::Value;

fn text(source: &str, binding: &str) -> String {
    let rt = run_quiet(source).expect("program should run");
    match rt.var(binding) {
        Some(Value::String(s)) => s.clone(),
        other => panic!("expected text in `{}`, found {:?}", binding, other),
    }
}

fn number(source: &str, binding: &str) -> i64 {
    let rt = run_quiet(source).expect("program should run");
    match rt.var(binding) {
        Some(Value::Int(n)) => *n,
        other => panic!("expected a number in `{}`, found {:?}", binding, other),
    }
}

#[test]
fn later_beliefs_shadow_earlier_ones() {
    let out = text(
        r#"
        trust policy above user
        alice.city is "Lisbon" as user from onboarding on 2026-03-01
        alice.city is "Berlin" as user from relocation on 2026-08-15
        let answer = what is alice.city
        "#,
        "answer",
    );
    assert_eq!(out, "Berlin");
}

#[test]
fn a_question_can_be_asked_of_the_past() {
    let out = text(
        r#"
        trust policy above user
        alice.city is "Lisbon" as user from onboarding on 2026-03-01
        alice.city is "Berlin" as user from relocation on 2026-08-15
        let answer = what was alice.city on 2026-04-01
        "#,
        "answer",
    );
    assert_eq!(out, "Lisbon");
}

#[test]
fn writes_dated_in_the_future_are_invisible_until_then() {
    let out = text(
        r#"
        trust policy above user
        rate is "old" as policy from a on 2026-01-01
        rate is "new" as policy from b on 2027-01-01
        let answer = what was rate on 2026-06-01
        "#,
        "answer",
    );
    assert_eq!(out, "old");
}

#[test]
fn authority_beats_recency() {
    let n = number(
        r#"
        trust policy above user
        pto is 20 as policy from handbook on 2026-01-01
        pto is 25 as user from slack on 2026-09-02
        let answer = what is pto
        "#,
        "answer",
    );
    assert_eq!(n, 20, "the newer claim had lower standing and must lose");
}

#[test]
fn authority_beats_scope_specificity() {
    // The inner scope is narrower and would win under ordinary lexical
    // shadowing. Standing overrules it.
    let out = text(
        r#"
        trust policy above user
        about acme:
            region is "eu-west-1" as policy from standard
            about alice:
                region is "us-east-1" as user from preference
                let answer = what is region
        "#,
        "answer",
    );
    assert_eq!(out, "eu-west-1");
}

#[test]
fn specificity_beats_recency_at_equal_authority() {
    let out = text(
        r#"
        trust policy above user
        about acme:
            channel is "email" as user from inner on 2026-01-01
            about team:
                channel is "slack" as user from inner_inner on 2020-01-01
                let answer = what is channel
        "#,
        "answer",
    );
    assert_eq!(
        out, "slack",
        "the innermost scope wins even though it is far older"
    );
}

#[test]
fn an_inner_scope_falls_back_to_an_outer_name() {
    let out = text(
        r#"
        trust policy above user
        about acme:
            domain is "acme.test" as policy from dns
            about team:
                let answer = what is domain
        "#,
        "answer",
    );
    assert_eq!(out, "acme.test");
}

#[test]
fn an_unlabelled_claim_takes_the_weakest_standing() {
    // Nothing that declines to say who is speaking may outrank something that
    // does.
    let n = number(
        r#"
        trust policy above user above rumor
        headcount is 240 as policy from filing
        headcount is 999
        let answer = what is headcount
        "#,
        "answer",
    );
    assert_eq!(n, 240);
}

#[test]
fn history_labels_every_layer() {
    let rt = run_quiet(
        r#"
        trust policy above user
        alice.city is "Lisbon" as user from onboarding on 2026-03-01
        alice.city is "Berlin" as user from relocation on 2026-08-15
        let trail = why alice.city
        "#,
    )
    .unwrap();

    let Some(Value::History(layers)) = rt.var("trail") else {
        panic!("expected a history");
    };

    assert_eq!(layers.len(), 2, "nothing is erased by being superseded");
    let rendered = format!("{}", Value::History(layers.clone()));
    assert!(rendered.contains("Lisbon"));
    assert!(rendered.contains("overwritten by"));
    assert!(rendered.contains("current"));
}

#[test]
fn history_explains_a_loss_by_authority_differently_from_a_loss_by_time() {
    let rt = run_quiet(
        r#"
        trust policy above user
        pto is 20 as policy from handbook on 2026-01-01
        pto is 25 as user from slack on 2026-09-02
        let trail = why pto
        "#,
    )
    .unwrap();

    let rendered = rt.var("trail").unwrap().to_string();
    assert!(
        rendered.contains("outranked by"),
        "a loss on standing must not be reported as a loss on time: {}",
        rendered
    );
}

#[test]
fn an_unknown_name_is_an_error_not_an_empty_answer() {
    let err = run_quiet("let x = what is nobody.knows").unwrap_err();
    assert_eq!(err.tag(), "unknown");
}

#[test]
fn declaring_trust_replaces_the_default_order() {
    let rt = run_quiet(
        r#"
        trust handbook above hearsay
        x is 1 as handbook from doc
        "#,
    )
    .unwrap();
    assert_eq!(rt.trust_order(), &["handbook", "hearsay"]);
}

#[test]
fn an_authority_nobody_declared_is_rejected() {
    let err = run_quiet(
        r#"
        trust policy above user
        x is 1 as wizard from doc
        "#,
    )
    .unwrap_err();

    let message = err.to_string();
    assert!(
        message.contains("wizard") && message.contains("policy"),
        "the error should name the unknown tier and list the real ones: {}",
        message
    );
}
