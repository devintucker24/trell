// The ways a Palimpsest program can stop.
//
// Refusals are not failures of the interpreter. They are the language
// declining to answer a question it cannot answer honestly, which is the
// behaviour the rest of the design exists to make possible.

use std::fmt;

use crate::time::{Duration, Timestamp};

#[derive(Debug, Clone, PartialEq)]
pub enum PalimpsestError {
    /// A query demanded `fresh` and the belief had outlived its lifetime.
    Stale {
        path: String,
        expired_at: Timestamp,
        over_by: Duration,
    },

    /// A query demanded `verified` and the belief cites nothing trustworthy.
    Unverified {
        path: String,
        source: Option<String>,
        authority: String,
    },

    /// A query demanded a minimum standing the belief does not have.
    Untrusted {
        path: String,
        required: String,
        actual: String,
    },

    /// Two beliefs claim the same stated moment at equal standing.
    Contested {
        path: String,
        authority: String,
        values: Vec<String>,
    },

    /// Nothing is known by that name.
    Unknown { path: String, scope: String },

    /// An `expect` did not hold.
    ExpectationFailed {
        line: usize,
        left: String,
        right: String,
    },

    TypeError(String),

    ParseError {
        line: usize,
        column: usize,
        message: String,
    },

    Runtime(String),
}

impl PalimpsestError {
    /// Short machine-friendly tag, used in tests and tooling.
    pub fn tag(&self) -> &'static str {
        match self {
            PalimpsestError::Stale { .. } => "stale",
            PalimpsestError::Unverified { .. } => "unverified",
            PalimpsestError::Untrusted { .. } => "untrusted",
            PalimpsestError::Contested { .. } => "contested",
            PalimpsestError::Unknown { .. } => "unknown",
            PalimpsestError::ExpectationFailed { .. } => "expectation-failed",
            PalimpsestError::TypeError(_) => "type-error",
            PalimpsestError::ParseError { .. } => "parse-error",
            PalimpsestError::Runtime(_) => "runtime",
        }
    }

    /// True for the four refusals that are deliberate epistemic outcomes
    /// rather than mistakes in the program.
    pub fn is_refusal(&self) -> bool {
        matches!(
            self,
            PalimpsestError::Stale { .. }
                | PalimpsestError::Unverified { .. }
                | PalimpsestError::Untrusted { .. }
                | PalimpsestError::Contested { .. }
        )
    }
}

impl fmt::Display for PalimpsestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PalimpsestError::Stale { path, expired_at, over_by } => write!(
                f,
                "refused: {} expired on {}, {} ago. The question asked for a fresh answer and there isn't one.",
                path,
                expired_at.to_date(),
                over_by.humanize()
            ),

            PalimpsestError::Unverified { path, source, authority } => write!(
                f,
                "refused: {} is only believed as {} via {}. The question asked for a verified answer, so nothing is returned.",
                path,
                authority,
                source.as_deref().unwrap_or("no source at all")
            ),

            PalimpsestError::Untrusted { path, required, actual } => write!(
                f,
                "refused: {} is only believed as {}, and the question required at least {}.",
                path, actual, required
            ),

            PalimpsestError::Contested { path, authority, values } => write!(
                f,
                "refused: {} holds {} at equal standing ({}). Nothing in the trust order decides between them, so the answer is a contradiction rather than a guess.",
                path,
                values.join(" and "),
                authority
            ),

            PalimpsestError::Unknown { path, scope } => {
                if scope.is_empty() {
                    write!(f, "nothing is known by the name {}", path)
                } else {
                    write!(f, "nothing is known by the name {} (looking inside {})", path, scope)
                }
            }

            PalimpsestError::ExpectationFailed { line, left, right } => write!(
                f,
                "line {}: expected {} but got {}",
                line, right, left
            ),

            PalimpsestError::TypeError(msg) => write!(f, "{}", msg),

            PalimpsestError::ParseError { line, column, message } => {
                write!(f, "line {}, column {}: {}", line, column, message)
            }

            PalimpsestError::Runtime(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for PalimpsestError {}
