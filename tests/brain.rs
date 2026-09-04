//! Markdown brains, and the examples shipped with the repo.

use std::path::Path;

use palimpsest::error::PalimpsestError;
use palimpsest::types::Value;
use palimpsest::{markdown, run_brain, run_file};

#[test]
fn a_page_is_the_provenance_of_the_facts_written_on_it() {
    let page = markdown::extract(
        "wiki/alice.md",
        "# Alice\n\nProse.\n\n```pal\nalice.city is \"Berlin\"\n```\n",
    );
    assert_eq!(page.source, "wiki/alice.md");
    assert_eq!(page.blocks, 1);
}

#[test]
fn prose_around_a_block_is_ignored_but_line_numbers_are_kept() {
    let md = "# Title\n\nWords that are not code.\n\n```pal\nx is\n```\n";
    let page = markdown::extract("p.md", md);
    let err = palimpsest::parse(&page.code).unwrap_err();
    assert!(
        err.to_string().contains("line 6"),
        "diagnostics must point at the markdown line: {}",
        err
    );
}

#[test]
fn the_whole_brain_loads_as_one_belief_store() {
    let rt = run_brain("examples/brain").expect("the example brain should load");

    // The handbook page and Alice's page disagree about leave. The handbook is
    // filed as policy, so it wins across file boundaries.
    let answer = rt.beliefs().iter().find(|b| b.path == "acme.pto.days");
    assert!(answer.is_some(), "facts from separate pages share one namespace");
    assert!(
        !rt.conflicts().is_empty(),
        "the cross-page disagreement should be recorded"
    );
}

#[test]
fn withdrawing_a_page_withdraws_its_episodes_too() {
    let rt = palimpsest::run_quiet(
        r#"
        trust compliance above user
        when outage:
            happened on 2026-09-04T08:15
            summary "pool exhausted"
        "#,
    )
    .unwrap();
    assert_eq!(rt.episodes().count(), 1);

    // Inside a markdown page the episode inherits the page as its source, so
    // forgetting the page reaches it.
    let mut rt = palimpsest::runtime::Runtime::new();
    rt.quiet = true;
    palimpsest::load_file(&mut rt, Path::new("examples/brain/incidents/4471.md")).unwrap();
    assert_eq!(rt.episodes().filter(|e| !e.retracted).count(), 1);

    rt.forget_source("pagerduty_incident_4471");
    assert_eq!(
        rt.episodes().filter(|e| !e.retracted).count(),
        0,
        "the incident report carried the episode and took it with it"
    );
}

#[test]
fn frontmatter_can_name_a_stable_source_id() {
    let mut rt = palimpsest::runtime::Runtime::new();
    rt.quiet = true;
    palimpsest::load_file(&mut rt, Path::new("examples/brain/handbook.md")).unwrap();

    let belief = rt
        .beliefs()
        .iter()
        .find(|b| b.path == "acme.pto.days")
        .expect("the handbook states a leave allowance");

    assert_eq!(
        belief.provenance.source.as_deref(),
        Some("hr_handbook_2026"),
        "frontmatter should override the file path"
    );
}

#[test]
fn every_example_runs() {
    // `provenance.pal` ends in a deliberate refusal; everything else must
    // complete, and every `expect` inside them must hold.
    let cases = [
        ("examples/moving.pal", None),
        ("examples/authority.pal", None),
        ("examples/forgetting.pal", None),
        ("examples/lifetimes.pal", None),
        ("examples/episodes.pal", None),
        ("examples/check.pal", None),
        ("examples/provenance.pal", Some("unverified")),
    ];

    for (path, expected_refusal) in cases {
        let mut rt = palimpsest::runtime::Runtime::new();
        rt.quiet = true;
        let outcome = palimpsest::load_file(&mut rt, Path::new(path));

        match (outcome, expected_refusal) {
            (Ok(()), None) => {}
            (Ok(()), Some(tag)) => panic!("{} should have refused with `{}`", path, tag),
            (Err(err), Some(tag)) => {
                assert_eq!(err.tag(), tag, "{} refused for the wrong reason", path)
            }
            (Err(err), None) => panic!("{} failed: {}", path, err),
        }
    }
}

#[test]
fn the_example_brain_runs_end_to_end() {
    let rt = run_brain("examples/brain").expect("the example brain should run");
    let report = rt.check();
    assert_eq!(
        report.errors(),
        0,
        "the example brain should have no contradictions: {:?}",
        report.findings
    );
}

#[test]
fn a_missing_file_reports_a_readable_error() {
    match run_file("examples/nope.pal") {
        Err(PalimpsestError::Runtime(msg)) => assert!(msg.contains("nope.pal")),
        other => panic!("expected a readable error, got {:?}", other.map(|_| ())),
    }
}

#[test]
fn a_page_with_no_blocks_contributes_nothing() {
    let page = markdown::extract("readme.md", "# Just prose\n\nNothing to run here.\n");
    assert!(page.is_empty());
}

#[test]
fn values_read_as_prose_when_shown() {
    assert_eq!(Value::String("Berlin".into()).plain(), "Berlin");
    assert_eq!(Value::Bool(true).plain(), "yes");
    assert_eq!(Value::Null.plain(), "nothing");
}
