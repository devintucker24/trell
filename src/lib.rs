//! Palimpsest — a language for what an agent knows.
//!
//! Beliefs are written as sentences, resolved by a stated rule rather than by
//! similarity, and kept in layers so the reasoning stays inspectable:
//!
//! ```text
//! trust policy above user
//!
//! alice.city is "Lisbon" from onboarding on 2026-03-01
//! alice.city is "Berlin" from relocation on 2026-08-15
//!
//! what is alice.city              // Berlin
//! what was alice.city on 2026-04-01   // Lisbon
//! why alice.city                  // both layers, and why Lisbon lost
//! ```

pub mod ast;
pub mod error;
pub mod lexer;
pub mod markdown;
pub mod parser;
pub mod runtime;
pub mod time;
pub mod types;

use std::fs;
use std::path::{Path, PathBuf};

use crate::ast::Program;
use crate::error::PalimpsestError;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::runtime::Runtime;

/// Turns source text into a program without running it.
pub fn parse(source: &str) -> Result<Program, PalimpsestError> {
    let tokens = Lexer::new(source).tokenize()?;
    Parser::new(tokens).parse_program()
}

/// Runs source text in a fresh runtime.
pub fn run(source: &str) -> Result<Runtime, PalimpsestError> {
    let mut rt = Runtime::new();
    rt.run(&parse(source)?)?;
    Ok(rt)
}

/// Runs source text without printing, for tests and embedding.
pub fn run_quiet(source: &str) -> Result<Runtime, PalimpsestError> {
    let mut rt = Runtime::new();
    rt.quiet = true;
    rt.run(&parse(source)?)?;
    Ok(rt)
}

/// Runs a `.pal` source file or a `.md` page containing `pal` blocks.
pub fn run_file(path: impl AsRef<Path>) -> Result<Runtime, PalimpsestError> {
    let mut rt = Runtime::new();
    load_file(&mut rt, path.as_ref())?;
    Ok(rt)
}

/// Runs an entire brain directory: every `.pal` and `.md` file beneath it, in
/// sorted order so results do not depend on filesystem enumeration.
pub fn run_brain(root: impl AsRef<Path>) -> Result<Runtime, PalimpsestError> {
    let mut rt = Runtime::new();
    for path in collect(root.as_ref())? {
        load_file(&mut rt, &path)?;
    }
    Ok(rt)
}

/// Loads one file into an existing runtime, so several pages can accumulate
/// into a single belief store.
pub fn load_file(rt: &mut Runtime, path: &Path) -> Result<(), PalimpsestError> {
    let text = fs::read_to_string(path).map_err(|e| {
        PalimpsestError::Runtime(format!("could not read {}: {}", path.display(), e))
    })?;

    let label = path.to_string_lossy().to_string();
    let is_markdown = matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("md") | Some("markdown")
    );

    let (code, source, _authority) = if is_markdown {
        let page = markdown::extract(&label, &text);
        if page.is_empty() {
            return Ok(());
        }
        (page.code, Some(page.source), page.authority)
    } else {
        (text, None, None)
    };

    let previous_origin = std::mem::replace(&mut rt.origin, label);
    let previous_source = std::mem::replace(&mut rt.ambient_source, source);

    let result = parse(&code).and_then(|program| rt.run(&program));

    rt.origin = previous_origin;
    rt.ambient_source = previous_source;

    result
}

/// All Palimpsest-bearing files beneath a root, sorted for determinism.
pub fn collect(root: &Path) -> Result<Vec<PathBuf>, PalimpsestError> {
    let mut found = Vec::new();
    walk(root, &mut found)?;
    found.sort();
    Ok(found)
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), PalimpsestError> {
    if dir.is_file() {
        out.push(dir.to_path_buf());
        return Ok(());
    }

    let entries = fs::read_dir(dir).map_err(|e| {
        PalimpsestError::Runtime(format!("could not read {}: {}", dir.display(), e))
    })?;

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if name.starts_with('.') || name == "target" || name == "node_modules" {
            continue;
        }

        if path.is_dir() {
            walk(&path, out)?;
        } else if matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("pal") | Some("md") | Some("markdown")
        ) {
            out.push(path);
        }
    }

    Ok(())
}
