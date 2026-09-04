use std::collections::BTreeMap;
use trell::check::check;
use trell::compile_source;
use trell::gbnf;
use trell::interp::{self, Host};
use trell::parser::parse;
use trell::plan;
use trell::value::Value;
use trell::wasm;

fn load_example(name: &str) -> String {
    std::fs::read_to_string(format!("examples/{name}")).unwrap()
}

#[test]
fn arithmetic_examples_eval_through_wasm() {
    let cases = [
        ("42.trell", 42),
        ("arithmetic.trell", 64),
        ("precedence.trell", 64),
        ("parentheses.trell", 84),
        ("subtraction.trell", 70),
        ("division.trell", 28),
    ];
    for (name, expected) in cases {
        let source = load_example(name);
        let checked = compile_source(&source).unwrap();
        assert!(checked.program.is_pure_compute(), "{name}");
        let module = wasm::compile(&checked.program).unwrap();
        let value = wasm::interpret(&module, &[]).unwrap();
        assert_eq!(value, expected, "{name}");
        let bytes = wasm::encode_wasm(&module);
        assert_eq!(&bytes[..4], b"\0asm");
    }
}

#[test]
fn fees_respect_cap() {
    let source = load_example("fees.trell");
    let checked = compile_source(&source).unwrap();
    let module = wasm::compile(&checked.program).unwrap();
    assert_eq!(wasm::interpret(&module, &[1]).unwrap(), 499);
    assert_eq!(wasm::interpret(&module, &[5]).unwrap(), 649);
    assert_eq!(wasm::interpret(&module, &[100]).unwrap(), 1999);
}

#[test]
fn pr_review_plans_and_runs() {
    let source = load_example("pr-review.trell");
    let checked = compile_source(&source).unwrap();
    let planned = plan::plan(&checked);
    assert!(planned.allowed.iter().any(|g| g.effect == "ask"));
    assert!(planned.denied.iter().any(|d| d == "write"));
    assert_eq!(planned.spawn_limit, 0);

    let mut host = Host::default();
    host.inputs
        .insert("diff".into(), Value::Text("logout".into()));
    let mut high = BTreeMap::new();
    high.insert("risk".into(), Value::Enum("high".into()));
    high.insert("reason".into(), Value::Text("auth surface".into()));
    host.ask_replies.push(Value::Record(high));

    let result = interp::run(&checked, host).unwrap();
    assert_eq!(result.asks, 1);
    assert_eq!(result.approvals.len(), 1);
    assert_eq!(result.sends.len(), 1);
}

#[test]
fn fail_examples_are_refused() {
    for name in [
        "fail/write-without-grant.trell",
        "fail/write-without-approve.trell",
        "fail/tainted-spawn.trell",
    ] {
        let source = load_example(name);
        let err = compile_source(&source).unwrap_err();
        assert!(!err.is_empty(), "{name} should fail");
    }
}

#[test]
fn gated_write_is_legal() {
    let source = load_example("gated-write.trell");
    let checked = compile_source(&source).unwrap();
    let result = interp::run(&checked, Host::default()).unwrap();
    assert_eq!(result.writes.len(), 1);
    assert_eq!(result.writes[0].0, "notes/out.md");
}

#[test]
fn grammar_snapshot_contains_gbnf_core() {
    let on_disk = std::fs::read_to_string("grammar/trell.gbnf").unwrap();
    let emitted = gbnf::gbnf();
    assert!(emitted.contains("root ::="));
    assert!(on_disk.contains("root ::="));
    assert!(emitted.contains("ask-expr"));
    assert!(on_disk.contains("need-item"));
}

#[test]
fn parse_roundtrip_errors_have_spans() {
    let err = parse("20 + ").unwrap_err();
    assert!(err.span.end >= err.span.start);
}

#[test]
fn check_rejects_unknown_capability() {
    let program = parse("cap { allow lasers }\n1").unwrap();
    let err = check(program).unwrap_err();
    assert!(err.iter().any(|d| d.message.contains("lasers")));
}
