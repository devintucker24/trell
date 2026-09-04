pub mod ast;
pub mod check;
pub mod diagnostics;
pub mod gbnf;
pub mod interp;
pub mod lexer;
pub mod parser;
pub mod plan;
pub mod span;
pub mod value;
pub mod wasm;

use crate::check::CheckedProgram;
use crate::diagnostics::Diagnostic;
use crate::parser::parse;

pub fn compile_source(source: &str) -> Result<CheckedProgram, Vec<Diagnostic>> {
    match parse(source) {
        Ok(program) => crate::check::check(program),
        Err(diagnostic) => Err(vec![diagnostic]),
    }
}
