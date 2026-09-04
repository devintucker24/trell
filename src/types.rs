// Palimpsest values, beliefs, episodes, and the reports the language emits.

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

    /// A belief that outlived its declared lifetime. It still carries the
    /// value, but the type is different, so nothing can consume it by accident.
    Stale {
        value: Box<Value>,
        age: Duration,
        lifetime: Duration,
    },

    /// The layered history of one name, newest last.
    History(Vec<Layer>),

    /// Overrides that were refused because the claimant lacked the standing.
    Conflicts(Vec<Conflict>),

    /// The output of `check`.
    Report(Report),
}

impl Value {
    pub fn is_stale(&self) -> bool {
        matches!(self, Value::Stale { .. })
    }

    /// The value underneath a staleness wrapper, if any.
    pub fn settled(&self) -> &Value {
        match self {
            Value::Stale { value, .. } => value.as_ref(),
            other => other,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::String(_) => "text",
            Value::Int(_) => "number",
            Value::Float(_) => "number",
            Value::Bool(_) => "yes/no",
            Value::Duration(_) => "duration",
            Value::Timestamp(_) => "date",
            Value::List(_) => "list",
            Value::Record(_) => "record",
            Value::Null => "nothing",
            Value::Stale { .. } => "stale value",
            Value::History(_) => "history",
            Value::Conflicts(_) => "conflicts",
            Value::Report(_) => "report",
        }
    }

    /// Rendering used when a program shows a value. Text prints without quotes
    /// so output reads as prose rather than as a debug dump.
    pub fn plain(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", if *b { "yes" } else { "no" }),
            Value::Duration(d) => write!(f, "{}", d.humanize()),
            Value::Timestamp(t) => write!(f, "{}", t.to_iso()),
            Value::Null => write!(f, "nothing"),

            Value::List(items) => {
                // Lists of records are usually episodes or query results, and
                // are unreadable on one line.
                if items.iter().any(|i| matches!(i, Value::Record(_))) {
                    for (i, item) in items.iter().enumerate() {
                        if i > 0 {
                            writeln!(f)?;
                        }
                        write!(f, "- {}", item)?;
                    }
                    return Ok(());
                }
                let rendered: Vec<String> = items.iter().map(|i| i.to_string()).collect();
                write!(f, "[{}]", rendered.join(", "))
            }

            Value::Record(fields) => {
                let rendered: Vec<String> =
                    fields.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
                write!(f, "{{ {} }}", rendered.join(", "))
            }

            Value::Stale { value, age, lifetime } => write!(
                f,
                "STALE {} (lived {}, allowed {})",
                value,
                age.humanize(),
                lifetime.humanize()
            ),

            Value::History(layers) => {
                if layers.is_empty() {
                    return write!(f, "no history");
                }
                let n = layers.len();
                writeln!(
                    f,
                    "history of {} ({} layer{})",
                    layers[0].path,
                    n,
                    if n == 1 { "" } else { "s" }
                )?;
                for layer in layers {
                    writeln!(f, "  {}", layer)?;
                }
                Ok(())
            }

            Value::Conflicts(conflicts) => {
                if conflicts.is_empty() {
                    return write!(f, "no conflicts");
                }
                let n = conflicts.len();
                writeln!(f, "{} conflict{}", n, if n == 1 { "" } else { "s" })?;
                for c in conflicts {
                    writeln!(f, "  {}", c)?;
                }
                Ok(())
            }

            Value::Report(report) => write!(f, "{}", report),
        }
    }
}

// ---- beliefs -----------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provenance {
    pub source: Option<String>,
    pub verified: bool,
    pub because: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Belief {
    pub id: usize,
    pub path: String,
    pub value: Value,
    pub authority: String,
    pub rank: usize,
    pub provenance: Provenance,
    pub asserted_at: Timestamp,
    /// Whether the author wrote an explicit date. Two beliefs that merely
    /// landed in the same tick are ordered by arrival; two that claim the same
    /// stated moment are a genuine contradiction.
    pub dated_explicitly: bool,
    pub expires_at: Option<Timestamp>,
    pub retracted: Option<String>,
    /// Where the belief was written, for diagnostics.
    pub origin: String,
}

impl Belief {
    pub fn is_live(&self) -> bool {
        self.retracted.is_none()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Episode {
    pub id: String,
    pub happened: Timestamp,
    pub involved: Vec<String>,
    pub details: BTreeMap<String, Value>,
    pub summary: String,
    /// The page or document that reported the episode, so withdrawing the
    /// document withdraws the episode too.
    pub source: Option<String>,
    pub retracted: bool,
}

// ---- history -----------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Standing {
    /// The belief a query resolves to right now.
    Current,
    /// Outranked by a later belief of the same authority.
    Overwritten { by: usize, at: Timestamp },
    /// Outranked by a belief of higher authority.
    Outranked { by: usize, authority: String },
    /// Removed, along with the reason.
    Forgotten { reason: String },
    /// Current, but past its lifetime.
    Expired { at: Timestamp },
}

impl fmt::Display for Standing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Standing::Current => write!(f, "current"),
            Standing::Overwritten { by, at } => {
                write!(f, "overwritten by #{} on {}", by, at.to_date())
            }
            Standing::Outranked { by, authority } => {
                write!(f, "outranked by #{} ({})", by, authority)
            }
            Standing::Forgotten { reason } => write!(f, "forgotten: {}", reason),
            Standing::Expired { at } => write!(f, "expired on {}", at.to_date()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    pub id: usize,
    pub path: String,
    pub value: Value,
    pub authority: String,
    pub source: Option<String>,
    pub verified: bool,
    pub asserted_at: Timestamp,
    pub standing: Standing,
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "#{} {} = {}  [{} via {} on {}] -> {}",
            self.id,
            self.path,
            self.value,
            self.authority,
            self.source.as_deref().unwrap_or("no source"),
            self.asserted_at.to_date(),
            self.standing
        )
    }
}

// ---- conflicts ---------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Conflict {
    pub path: String,
    pub winner_authority: String,
    pub winner_source: Option<String>,
    pub winner_value: Value,
    pub loser_authority: String,
    pub loser_source: Option<String>,
    pub loser_value: Value,
}

impl fmt::Display for Conflict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} said {} (via {}) but {} outranks it and says {} (via {})",
            self.path,
            self.loser_authority,
            self.loser_value,
            self.loser_source.as_deref().unwrap_or("no source"),
            self.winner_authority,
            self.winner_value,
            self.winner_source.as_deref().unwrap_or("no source"),
        )
    }
}

// ---- check report ------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Finding {
    /// A live belief that a question demanding `verified` would refuse:
    /// either it cites nothing, or it was recorded as hearsay on purpose.
    Unsourced {
        id: usize,
        path: String,
        authority: String,
        source: Option<String>,
    },
    /// A live belief past its lifetime that queries still resolve to.
    Stale { id: usize, path: String, expired_at: Timestamp, over_by: Duration },
    /// Two live beliefs claiming the same stated moment at equal standing.
    Contested { path: String, authority: String, values: Vec<String> },
    /// An override that was refused.
    Refused { path: String, loser: String, winner: String },
    /// A belief whose justifying episode no longer exists.
    Orphaned { id: usize, path: String, episode: String },
}

impl Finding {
    pub fn severity(&self) -> &'static str {
        match self {
            Finding::Contested { .. } | Finding::Orphaned { .. } => "error",
            Finding::Unsourced { .. } | Finding::Stale { .. } => "warning",
            Finding::Refused { .. } => "note",
        }
    }
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Finding::Unsourced {
                id,
                path,
                authority,
                source,
            } => match source {
                Some(src) => write!(
                    f,
                    "unverified: #{} {} cites {} but is recorded as hearsay; a question demanding `verified` will refuse it",
                    id, path, src
                ),
                None => write!(
                    f,
                    "unsourced: #{} {} is believed as {} but cites nothing; a question demanding `verified` will refuse it",
                    id, path, authority
                ),
            },
            Finding::Stale { id, path, expired_at, over_by } => write!(
                f,
                "stale: #{} {} expired on {} and is {} past its lifetime",
                id,
                path,
                expired_at.to_date(),
                over_by.humanize()
            ),
            Finding::Contested { path, authority, values } => write!(
                f,
                "contested: {} holds {} at equal standing ({}); no rule decides between them",
                path,
                values.join(" and "),
                authority
            ),
            Finding::Refused { path, loser, winner } => write!(
                f,
                "refused: an override of {} by {} was rejected in favour of {}",
                path, loser, winner
            ),
            Finding::Orphaned { id, path, episode } => write!(
                f,
                "orphaned: #{} {} rests on episode `{}`, which this brain has no record of",
                id, path, episode
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    pub findings: Vec<Finding>,
    pub total_beliefs: usize,
    pub live_beliefs: usize,
    pub episodes: usize,
}

impl Report {
    pub fn errors(&self) -> usize {
        self.findings.iter().filter(|f| f.severity() == "error").count()
    }

    pub fn warnings(&self) -> usize {
        self.findings.iter().filter(|f| f.severity() == "warning").count()
    }

    pub fn is_healthy(&self) -> bool {
        self.errors() == 0 && self.warnings() == 0
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let plural = |n: usize, word: &str| format!("{} {}{}", n, word, if n == 1 { "" } else { "s" });
        writeln!(
            f,
            "checked {} ({} live) and {}",
            plural(self.total_beliefs, "belief"),
            self.live_beliefs,
            plural(self.episodes, "episode")
        )?;

        if self.findings.is_empty() {
            return write!(f, "  everything is sourced, fresh, and uncontested");
        }

        for finding in &self.findings {
            writeln!(f, "  [{}] {}", finding.severity(), finding)?;
        }

        write!(
            f,
            "  {} error(s), {} warning(s)",
            self.errors(),
            self.warnings()
        )
    }
}
