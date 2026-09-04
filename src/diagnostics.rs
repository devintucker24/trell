use crate::span::{line_bounds, line_col, Span};
use std::fmt;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub span: Span,
    pub notes: Vec<String>,
}

impl Diagnostic {
    pub fn error(message: impl Into<String>, span: Span) -> Self {
        Self {
            severity: Severity::Error,
            message: message.into(),
            span,
            notes: Vec::new(),
        }
    }

    pub fn warning(message: impl Into<String>, span: Span) -> Self {
        Self {
            severity: Severity::Warning,
            message: message.into(),
            span,
            notes: Vec::new(),
        }
    }

    pub fn note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        write!(f, "{kind}: {}", self.message)
    }
}

pub fn render(filename: &str, source: &str, diagnostic: &Diagnostic) -> String {
    let mut out = String::new();
    let loc = line_col(source, diagnostic.span.start as usize);
    let (line_start, line_end) = line_bounds(source, diagnostic.span.start as usize);
    let line = &source[line_start..line_end];
    let kind = match diagnostic.severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
    };

    out.push_str(&format!("{kind}: {}\n", diagnostic.message));
    out.push_str(&format!(" --> {filename}:{loc}\n"));
    out.push_str("    |\n");
    out.push_str(&format!("{:>3} | {line}\n", loc.line));

    let caret_col = (diagnostic.span.start as usize).saturating_sub(line_start);
    let caret_len = (diagnostic.span.end.saturating_sub(diagnostic.span.start) as usize).max(1);
    let underline: String = std::iter::repeat('^').take(caret_len.min(80)).collect();
    let pad: String = std::iter::repeat(' ').take(caret_col).collect();
    out.push_str(&format!("    | {pad}{underline}\n"));

    for note in &diagnostic.notes {
        out.push_str(&format!("    |\n    = {note}\n"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_caret_under_span() {
        let source = "spawn code.body\n";
        let diag = Diagnostic::error("tainted", Span::new(6, 15)).note("fail closed");
        let rendered = render("demo.trell", source, &diag);
        assert!(rendered.contains("error: tainted"));
        assert!(rendered.contains("demo.trell:1:7"));
        assert!(rendered.contains("spawn code.body"));
        assert!(rendered.contains("^^^^^^"));
        assert!(rendered.contains("fail closed"));
    }
}

pub fn render_all(filename: &str, source: &str, diagnostics: &[Diagnostic]) -> String {
    diagnostics
        .iter()
        .map(|d| render(filename, source, d))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn eprint_all(filename: &str, source: &str, diagnostics: &[Diagnostic]) -> io::Result<()> {
    let mut stderr = io::stderr().lock();
    write!(stderr, "{}", render_all(filename, source, diagnostics))
}
