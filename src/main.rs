// The `palimpsest` command.

use std::path::Path;
use std::process;

use palimpsest::error::PalimpsestError;
use palimpsest::runtime::Runtime;
use palimpsest::{collect, load_file};

const USAGE: &str = "\
palimpsest — a language for what an agent knows

USAGE
    palimpsest <file.pal>        run a program
    palimpsest <page.md>         run the pal blocks inside a markdown page
    palimpsest <directory>       run every .pal and .md file beneath it
    palimpsest -e \"<source>\"     run source given on the command line

OPTIONS
    --check      after running, report unsourced, stale and contested beliefs
                 and exit non-zero if any are found
    --quiet      suppress program output, print only diagnostics
    -h, --help   show this message

EXAMPLES
    palimpsest examples/moving.pal
    palimpsest examples/brain --check
";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{}", USAGE);
        return;
    }

    let check = args.iter().any(|a| a == "--check");
    let quiet = args.iter().any(|a| a == "--quiet");
    let positional: Vec<&String> = args.iter().filter(|a| !a.starts_with('-')).collect();

    let mut rt = Runtime::new();
    rt.quiet = quiet;

    let outcome = if let Some(index) = args.iter().position(|a| a == "-e") {
        match args.get(index + 1) {
            Some(source) => {
                rt.origin = "<argument>".into();
                palimpsest::parse(source).and_then(|program| rt.run(&program))
            }
            None => {
                eprintln!("error: -e needs some source to run");
                process::exit(2);
            }
        }
    } else {
        let Some(target) = positional.first() else {
            eprintln!("error: nothing to run");
            process::exit(2);
        };
        run_target(&mut rt, target)
    };

    if let Err(err) = outcome {
        report(&err);
        process::exit(1);
    }

    if check {
        let report = rt.check();
        println!("\n{}", report);
        if report.errors() > 0 {
            process::exit(1);
        }
    }
}

fn run_target(rt: &mut Runtime, target: &str) -> Result<(), PalimpsestError> {
    let path = Path::new(target);

    if !path.exists() {
        return Err(PalimpsestError::Runtime(format!(
            "{} does not exist",
            path.display()
        )));
    }

    if path.is_file() {
        return load_file(rt, path);
    }

    let files = collect(path)?;
    if files.is_empty() {
        return Err(PalimpsestError::Runtime(format!(
            "no .pal or .md files under {}",
            path.display()
        )));
    }

    for file in files {
        load_file(rt, &file)?;
    }
    Ok(())
}

/// Refusals are the language working as designed, so they are labelled
/// differently from mistakes in the program.
fn report(err: &PalimpsestError) {
    if err.is_refusal() {
        eprintln!("\n{}", err);
        eprintln!("\nThis is a refusal, not a crash: the belief store could not answer");
        eprintln!("that question under the conditions the question set.");
    } else {
        eprintln!("\nerror: {}", err);
    }
}
