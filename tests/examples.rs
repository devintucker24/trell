use trell::runtime::run_file;

fn run_example(name: &str) -> String {
    let path = format!("examples/{name}");
    run_file(std::path::Path::new(&path)).unwrap_or_else(|error| panic!("{name}: {error}"))
}

#[test]
fn letter_speaks_the_intimate_offer_and_its_clinical_shadow() {
    let output = run_example("letter.trell");
    let lower = output.to_ascii_lowercase();
    assert!(
        lower.contains("thinking of you") || lower.contains("did not leave"),
        "{output}"
    );
    assert!(lower.contains("shadow of letter"), "{output}");
    assert!(
        lower.contains("ward was closed")
            || lower.contains("lights out")
            || lower.contains("next of kin")
            || lower.contains("empty corridors")
            || lower.contains("figures will be released"),
        "shadow should speak the blotter, got:\n{output}"
    );
}

#[test]
fn voices_split_one_incident() {
    let output = run_example("voices.trell");
    let lower = output.to_ascii_lowercase();
    assert!(
        lower.contains("incident logged") || lower.contains("statement will be released"),
        "{output}"
    );
    assert!(
        lower.contains("hands") || lower.contains("loved you") || lower.contains("kitchen"),
        "{output}"
    );
}

#[test]
fn winnow_has_a_shadow() {
    let output = run_example("winnow.trell");
    assert!(output.contains("shadow of public_voice"), "{output}");
}

#[test]
fn recipe_applies_the_same_path_twice() {
    let output = run_example("recipe.trell");
    assert!(output.contains("night_letter"), "{output}");
    assert!(output.contains("morning_letter"), "{output}");
}

#[test]
fn fork_takes_the_small_scale_branch() {
    let output = run_example("fork.trell");
    let lower = output.to_ascii_lowercase();
    assert!(
        lower.contains("light on") || lower.contains("chart says") || lower.contains("empty"),
        "{output}"
    );
}
