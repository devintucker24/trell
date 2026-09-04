// Palimpsest Epistemic Errors & Diagnostics

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum PalimpsestError {
    StaleBeliefError {
        path: String,
        age_secs: u64,
        ttl_secs: u64,
        expired_at: String,
    },
    UnverifiedBeliefRefusal {
        path: String,
        source: Option<String>,
        authority: String,
        reason: String,
    },
    InsufficientAuthorityError {
        path: String,
        required_authority: String,
        actual_authority: String,
    },
    ContradictionError {
        path: String,
        conflicting_values: Vec<String>,
        authority: String,
    },
    PathNotFoundError {
        path: String,
        scope: String,
    },
    AssertionFailed {
        message: String,
        left: String,
        right: String,
    },
    TypeError(String),
    ParseError {
        line: usize,
        column: usize,
        message: String,
    },
    RuntimeError(String),
}

impl fmt::Display for PalimpsestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PalimpsestError::StaleBeliefError { path, age_secs, ttl_secs, expired_at } => {
                write!(
                    f,
                    "Epistemic Refusal [StaleBelief]: Memory '{}' expired at {} (age: {}s, ttl: {}s). Query demanded fresh belief.",
                    path, expired_at, age_secs, ttl_secs
                )
            }
            PalimpsestError::UnverifiedBeliefRefusal { path, source, authority, reason } => {
                write!(
                    f,
                    "Epistemic Refusal [UnverifiedBelief]: Memory '{}' from source '{:?}' with authority '{}' refused. {}",
                    path, source.as_deref().unwrap_or("unknown"), authority, reason
                )
            }
            PalimpsestError::InsufficientAuthorityError { path, required_authority, actual_authority } => {
                write!(
                    f,
                    "Epistemic Refusal [InsufficientAuthority]: Memory '{}' requires minimum authority '{}', but resolved authority is '{}'.",
                    path, required_authority, actual_authority
                )
            }
            PalimpsestError::ContradictionError { path, conflicting_values, authority } => {
                write!(
                    f,
                    "Epistemic Contradiction [EqualAuthority]: Path '{}' holds mutually exclusive values {:?} at equal authority '{}' without a resolution order.",
                    path, conflicting_values, authority
                )
            }
            PalimpsestError::PathNotFoundError { path, scope } => {
                write!(f, "Unresolved Path: '{}' in scope '{}'.", path, scope)
            }
            PalimpsestError::AssertionFailed { message, left, right } => {
                write!(f, "Assertion Failed: {} (left: {}, right: {})", message, left, right)
            }
            PalimpsestError::TypeError(msg) => write!(f, "Type Error: {}", msg),
            PalimpsestError::ParseError { line, column, message } => {
                write!(f, "Parse Error at line {}, col {}: {}", line, column, message)
            }
            PalimpsestError::RuntimeError(msg) => write!(f, "Runtime Error: {}", msg),
        }
    }
}

impl std::error::Error for PalimpsestError {}
