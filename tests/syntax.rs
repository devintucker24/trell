//! Surface syntax: the shapes a person is expected to type.

use palimpsest::types::Value;
use palimpsest::{parse, run_quiet};

#[test]
fn a_fact_needs_nothing_but_a_name_and_a_value() {
    let rt = run_quiet("alice.city is \"Berlin\"\nlet x = what is alice.city").unwrap();
    assert_eq!(rt.var("x"), Some(&Value::String("Berlin".into())));
}

#[test]
fn facets_may_appear_in_any_order() {
    let orders = [
        r#"x is 1 as policy from doc on 2026-01-01 for 30 days"#,
        r#"x is 1 from doc for 30 days as policy on 2026-01-01"#,
        r#"x is 1 on 2026-01-01 for 30 days from doc as policy"#,
    ];

    for source in orders {
        let program = format!("trust policy above user\n{}\n", source);
        let rt = run_quiet(&program).unwrap_or_else(|e| panic!("`{}` failed: {}", source, e));
        let belief = &rt.beliefs()[0];
        assert_eq!(belief.authority, "policy");
        assert_eq!(belief.provenance.source.as_deref(), Some("doc"));
        assert_eq!(belief.asserted_at.to_date(), "2026-01-01");
        assert!(belief.expires_at.is_some());
    }
}

#[test]
fn durations_may_be_spaced_or_joined() {
    for text in ["for 30 days", "for 30d", "for 720 hours", "for 720h"] {
        let source = format!("trust policy above user\nx is 1 as policy from d {}\n", text);
        let rt = run_quiet(&source).unwrap_or_else(|e| panic!("`{}` failed: {}", text, e));
        let belief = &rt.beliefs()[0];
        let span = belief.expires_at.unwrap().since(belief.asserted_at);
        assert_eq!(span.as_secs(), 30 * 86_400, "`{}` should be thirty days", text);
    }
}

#[test]
fn dates_are_written_without_quotes() {
    let rt = run_quiet(
        r#"
        trust policy above user
        x is 1 as policy from d on 2026-08-15
        y is 2 as policy from d on 2026-08-15T14:30:00Z
        "#,
    )
    .unwrap();

    assert_eq!(rt.beliefs()[0].asserted_at.to_date(), "2026-08-15");
    assert_eq!(rt.beliefs()[1].asserted_at.to_date(), "2026-08-15 14:30");
}

#[test]
fn blocks_may_be_written_with_indentation_or_braces() {
    let indented = run_quiet(
        "trust policy above user\nabout acme:\n    x is 1 as policy from d\nlet v = what is acme.x",
    )
    .unwrap();

    let braced = run_quiet(
        "trust policy above user\nabout acme { x is 1 as policy from d }\nlet v = what is acme.x",
    )
    .unwrap();

    assert_eq!(indented.var("v"), braced.var("v"));
    assert_eq!(indented.var("v"), Some(&Value::Int(1)));
}

#[test]
fn scopes_nest() {
    let rt = run_quiet(
        r#"
        trust policy above user
        about acme:
            about eu:
                about billing:
                    owner is "alice" as policy from org_chart
        let v = what is acme.eu.billing.owner
        "#,
    )
    .unwrap();
    assert_eq!(rt.var("v"), Some(&Value::String("alice".into())));
}

#[test]
fn words_that_are_keywords_elsewhere_can_still_name_beliefs() {
    // Keywords are positional, so a fact may be called `summary` or `context`.
    let rt = run_quiet(
        r#"
        trust policy above user
        report.summary is "all clear" as policy from ops
        report.context is "quarterly" as policy from ops
        let a = what is report.summary
        let b = what is report.context
        "#,
    )
    .unwrap();

    assert_eq!(rt.var("a"), Some(&Value::String("all clear".into())));
    assert_eq!(rt.var("b"), Some(&Value::String("quarterly".into())));
}

#[test]
fn comments_use_hash_or_slashes() {
    let rt = run_quiet(
        r#"
        # a hash comment
        // a slash comment
        trust policy above user
        x is 1 as policy from d   # trailing comment
        let v = what is x
        "#,
    )
    .unwrap();
    assert_eq!(rt.var("v"), Some(&Value::Int(1)));
}

#[test]
fn blank_lines_do_not_end_a_block() {
    let rt = run_quiet(
        "trust policy above user\nabout acme:\n    a is 1 as policy from d\n\n    b is 2 as policy from d\nlet v = what is acme.b",
    )
    .unwrap();
    assert_eq!(rt.var("v"), Some(&Value::Int(2)));
}

#[test]
fn a_bare_question_prints_its_answer() {
    let mut rt = palimpsest::runtime::Runtime::new();
    rt.quiet = true;
    rt.run(&parse("trust policy above user\nx is 41 as policy from d\nwhat is x").unwrap())
        .unwrap();
    assert_eq!(rt.output, vec!["41"]);
}

#[test]
fn text_addition_builds_sentences() {
    let mut rt = palimpsest::runtime::Runtime::new();
    rt.quiet = true;
    rt.run(
        &parse(
            "trust policy above user\ncount is 3 as policy from d\nshow \"found \" + what is count",
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(rt.output, vec!["found 3"]);
}

#[test]
fn expect_reports_the_line_it_failed_on() {
    let err = run_quiet("trust policy above user\nx is 1 as policy from d\nexpect what is x is 2")
        .unwrap_err();
    let message = err.to_string();
    assert!(message.contains("line 3"), "got: {}", message);
}

#[test]
fn episodes_carry_participants_and_details() {
    let rt = run_quiet(
        r#"
        when outage:
            happened on 2026-09-04T08:15
            involved deploy_bot, alice
            details service is "billing-db", pool is 100
            summary "pool exhausted"
        "#,
    )
    .unwrap();

    let episode = rt.episodes().next().expect("one episode");
    assert_eq!(episode.id, "outage");
    assert_eq!(episode.involved, vec!["deploy_bot", "alice"]);
    assert_eq!(episode.details.len(), 2);
    assert_eq!(episode.summary, "pool exhausted");
}

#[test]
fn parse_errors_name_the_line_and_say_what_was_expected() {
    let err = parse("trust policy above user\nx is\n").unwrap_err();
    let message = err.to_string();
    assert!(message.contains("line 2"), "got: {}", message);
    assert!(message.contains("Expected"), "got: {}", message);
}

#[test]
fn an_unclosed_quote_is_reported_clearly() {
    let err = parse("x is \"oops\n").unwrap_err();
    assert!(err.to_string().contains("closing quote"));
}

#[test]
fn misaligned_indentation_is_reported() {
    let err = parse("about acme:\n    a is 1\n  b is 2\n").unwrap_err();
    assert!(err.to_string().contains("line up"), "got: {}", err);
}
