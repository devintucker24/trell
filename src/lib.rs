pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod runtime;
pub mod time;
pub mod types;

use std::fs;
use std::path::Path;
use crate::error::PalimpsestError;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::runtime::Runtime;

/// Execute a Palimpsest source code string and return the final runtime state
pub fn run_source(source: &str) -> Result<Runtime, PalimpsestError> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;
    let mut runtime = Runtime::new();
    runtime.execute_program(&program)?;
    Ok(runtime)
}

/// Execute a Palimpsest source file
pub fn run_file<P: AsRef<Path>>(path: P) -> Result<Runtime, PalimpsestError> {
    let content = fs::read_to_string(path.as_ref()).map_err(|e| PalimpsestError::RuntimeError(format!(
        "Failed to read file '{}': {}",
        path.as_ref().display(),
        e
    )))?;
    run_source(&content)
}
