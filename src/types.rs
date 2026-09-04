// Palimpsest Types, Values, Beliefs, and Audit Entries

use std::collections::BTreeMap;
use std::fmt;
use crate::time::{Duration, Timestamp};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Duration(Duration),
    Timestamp(Timestamp),
    List(Vec<Value>),
    Record(BTreeMap<String, Value>),
    Null,
    Stale {
        value: Box<Value>,
        age_secs: u64,
        ttl_secs: u64,
    },
    AuditLog(Vec<AuditEntry>),
    ConflictList(Vec<DefeasanceConflict>),
}

impl Value {
    pub fn is_stale(&self) -> bool {
        matches!(self, Value::Stale { .. })
    }

    pub fn unwrap_value(&self) -> &Value {
        match self {
            Value::Stale { value, .. } => value.as_ref(),
            other => other,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::String(_) => "String",
            Value::Int(_) => "Int",
            Value::Float(_) => "Float",
            Value::Bool(_) => "Bool",
            Value::Duration(_) => "Duration",
            Value::Timestamp(_) => "Timestamp",
            Value::List(_) => "List",
            Value::Record(_) => "Record",
            Value::Null => "Null",
            Value::Stale { .. } => "Stale",
            Value::AuditLog(_) => "AuditLog",
            Value::ConflictList(_) => "ConflictList",
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Duration(d) => write!(f, "{}", d.format_human()),
            Value::Timestamp(t) => write!(f, "{}", t.to_iso()),
            Value::List(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::Record(entries) => {
                write!(f, "{{ ")?;
                for (i, (k, v)) in entries.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, " }}")
            }
            Value::Null => write!(f, "null"),
            Value::Stale { value, age_secs, ttl_secs } => {
                write!(f, "Stale(value: {}, age: {}s, ttl: {}s)", value, age_secs, ttl_secs)
            }
            Value::AuditLog(entries) => {
                writeln!(f, "=== Palimpsest Inscription Audit ({}) ===", entries.len())?;
                for entry in entries {
                    writeln!(f, "  {}", entry)?;
                }
                Ok(())
            }
            Value::ConflictList(conflicts) => {
                if conflicts.is_empty() {
                    write!(f, "No epistemic conflicts detected.")
                } else {
                    writeln!(f, "=== Defeasance Conflicts ({}) ===", conflicts.len())?;
                    for c in conflicts {
                        writeln!(f, "  {}", c)?;
                    }
                    Ok(())
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provenance {
    pub source: Option<String>,
    pub verified: bool,
    pub grounded_in: Option<String>, // Episode ID if grounded
}

impl Provenance {
    pub fn new(source: Option<String>, verified: bool, grounded_in: Option<String>) -> Self {
        Self { source, verified, grounded_in }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Belief {
    pub id: usize,
    pub path: String,
    pub value: Value,
    pub authority: String,
    pub authority_rank: usize,
    pub provenance: Provenance,
    pub asserted_at: Timestamp,
    pub explicit_timestamp: bool,
    pub valid_until: Option<Timestamp>,
    pub is_retracted: bool,
    pub retraction_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Episode {
    pub id: String,
    pub at: Timestamp,
    pub actors: Vec<String>,
    pub context: BTreeMap<String, Value>,
    pub summary: String,
    pub is_retracted: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefeasanceConflict {
    pub path: String,
    pub high_authority: String,
    pub high_source: Option<String>,
    pub high_value: Value,
    pub low_authority: String,
    pub low_source: Option<String>,
    pub low_value: Value,
    pub reason: String,
}

impl fmt::Display for DefeasanceConflict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Conflict on '{}']: Low-authority '{}' (source: {:?}, value: {}) was defeated by existing high-authority '{}' (source: {:?}, value: {}). Reason: {}",
            self.path,
            self.low_authority,
            self.low_source.as_deref().unwrap_or("none"),
            self.low_value,
            self.high_authority,
            self.high_source.as_deref().unwrap_or("none"),
            self.high_value,
            self.reason
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AuditStatus {
    Active,
    ShadowedBy { belief_id: usize, timestamp: Timestamp },
    DefeatedByHigherAuthority { belief_id: usize, authority: String },
    Retracted { reason: String },
    Expired { expired_at: Timestamp },
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuditEntry {
    pub belief_id: usize,
    pub path: String,
    pub value: Value,
    pub authority: String,
    pub source: Option<String>,
    pub verified: bool,
    pub timestamp: Timestamp,
    pub valid_until: Option<Timestamp>,
    pub status: AuditStatus,
}

impl fmt::Display for AuditEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match &self.status {
            AuditStatus::Active => "ACTIVE".to_string(),
            AuditStatus::ShadowedBy { belief_id, timestamp } => {
                format!("SHADOWED (by #{} at {})", belief_id, timestamp.to_iso())
            }
            AuditStatus::DefeatedByHigherAuthority { belief_id, authority } => {
                format!("DEFEATED (by higher authority '{}' #{} )", authority, belief_id)
            }
            AuditStatus::Retracted { reason } => format!("RETRACTED ({})", reason),
            AuditStatus::Expired { expired_at } => format!("EXPIRED (at {})", expired_at.to_iso()),
        };

        write!(
            f,
            "[#{}] {} = {} | auth: {} | src: {:?} | ver: {} | time: {} | status: {}",
            self.belief_id,
            self.path,
            self.value,
            self.authority,
            self.source.as_deref().unwrap_or("none"),
            self.verified,
            self.timestamp.to_iso(),
            status_str
        )
    }
}
