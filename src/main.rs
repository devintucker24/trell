use std::env;
use std::path::Path;
use std::process;

use anyhow::{anyhow, Result};
use trell::parser;
use trell::runtime;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let mut args = env::args().skip(1);
    let first = args.next().ok_or_else(|| anyhow!(usage()))?;

    if first == "--ast" {
        let path = args.next().ok_or_else(|| anyhow!(usage()))?;
        reject_extra(args.next())?;
        let source = std::fs::read_to_string(&path)?;
        let program = parser::parse(&source)?;
        println!("{program:#?}");
        return Ok(());
    }

    if first == "--help" || first == "-h" {
        println!("{}", usage());
        return Ok(());
    }

    reject_extra(args.next())?;
    let output = runtime::run_file(Path::new(&first))?;
    print!("{output}");
    Ok(())
}

fn reject_extra(extra: Option<String>) -> Result<()> {
    if extra.is_some() {
        Err(anyhow!(usage()))
    } else {
        Ok(())
    }
}

fn usage() -> &'static str {
    "Usage: trell [--ast] <source-file.trell>"
}
