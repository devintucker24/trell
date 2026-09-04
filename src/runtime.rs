// The Palimpsest runtime.
//
// Resolution is the whole point of the language, so the rule is stated once,
// here, and never varies:
//
//   1. Highest authority wins.
//   2. Among equals, the most specific scope wins.
//   3. Among equals, the most recent wins.
//
// Everything else — lifetimes, provenance demands, retraction — is applied to
// the belief that rule selects, never as a way of selecting it.

use std::collections::{BTreeMap, HashMap, HashSet};

use crate::ast::*;
use crate::error::PalimpsestError;
use crate::time::{Duration, Timestamp};
use crate::types::*;

/// The default trust order, used when a program does not declare its own.
const DEFAULT_TRUST: &[&str] = &[
    "system",
    "legal",
    "compliance",
    "policy",
    "staff",
    "user",
    "guest",
    "rumor",
];

#[derive(Debug)]
pub struct Runtime {
    pub now: Timestamp,

    trust: Vec<String>,
    ranks: HashMap<String, usize>,

    beliefs: Vec<Belief>,
    by_path: HashMap<String, Vec<usize>>,
    by_source: HashMap<String, Vec<usize>>,
    by_episode: HashMap<String, Vec<usize>>,

    episodes: BTreeMap<String, Episode>,
    episodes_by_source: HashMap<String, Vec<String>>,
    forgotten_sources: HashSet<String>,

    conflicts: Vec<Conflict>,
    scope: Vec<String>,
    vars: HashMap<String, Value>,

    /// Set while ingesting a markdown page so facts inherit the page as their
    /// source without repeating it on every line.
    pub ambient_source: Option<String>,
    /// Human-readable location used in diagnostics.
    pub origin: String,

    pub output: Vec<String>,
    pub quiet: bool,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    pub fn new() -> Self {
        let trust: Vec<String> = DEFAULT_TRUST.iter().map(|s| s.to_string()).collect();
        let ranks = build_ranks(&trust);

        Self {
            // A fixed default clock keeps every example reproducible.
            now: Timestamp::parse_iso("2026-09-04T12:00:00Z").unwrap(),
            trust,
            ranks,
            beliefs: Vec::new(),
            by_path: HashMap::new(),
            by_source: HashMap::new(),
            by_episode: HashMap::new(),
            episodes: BTreeMap::new(),
            episodes_by_source: HashMap::new(),
            forgotten_sources: HashSet::new(),
            conflicts: Vec::new(),
            scope: Vec::new(),
            vars: HashMap::new(),
            ambient_source: None,
            origin: "<input>".into(),
            output: Vec::new(),
            quiet: false,
        }
    }

    // ---- accessors used by tests and tooling ---------------------------

    pub fn beliefs(&self) -> &[Belief] {
        &self.beliefs
    }

    pub fn conflicts(&self) -> &[Conflict] {
        &self.conflicts
    }

    pub fn episodes(&self) -> impl Iterator<Item = &Episode> {
        self.episodes.values()
    }

    pub fn var(&self, name: &str) -> Option<&Value> {
        self.vars.get(name)
    }

    pub fn trust_order(&self) -> &[String] {
        &self.trust
    }

    // ---- execution ------------------------------------------------------

    pub fn run(&mut self, program: &Program) -> Result<(), PalimpsestError> {
        for stmt in &program.statements {
            self.exec(stmt)?;
        }
        Ok(())
    }

    pub fn exec(&mut self, stmt: &Stmt) -> Result<(), PalimpsestError> {
        match stmt {
            Stmt::Trust(tiers) => {
                self.trust = tiers.iter().map(|t| t.to_ascii_lowercase()).collect();
                self.ranks = build_ranks(&self.trust);
                Ok(())
            }

            Stmt::About { prefix, body } => {
                let depth = self.scope.len();
                self.scope.extend(prefix.iter().cloned());
                let result = body.iter().try_for_each(|s| self.exec(s));
                self.scope.truncate(depth);
                result
            }

            Stmt::Fact {
                path,
                value,
                facets,
                line,
            } => {
                let value = self.eval(value)?;
                self.inscribe(path, value, facets, *line)
            }

            Stmt::Episode {
                id,
                happened,
                involved,
                details,
                summary,
            } => {
                let happened = match happened {
                    Some(expr) => {
                        let v = self.eval(expr)?;
                        self.as_timestamp(&v)?
                    }
                    None => self.now,
                };

                let mut people = Vec::new();
                for expr in involved {
                    people.push(self.eval(expr)?.plain());
                }

                let mut detail_map = BTreeMap::new();
                for (key, expr) in details {
                    let v = self.eval(expr)?;
                    detail_map.insert(key.clone(), v);
                }

                let summary = match summary {
                    Some(expr) => self.eval(expr)?.plain(),
                    None => String::new(),
                };

                let source = self.ambient_source.clone();
                if let Some(src) = &source {
                    self.episodes_by_source
                        .entry(src.clone())
                        .or_default()
                        .push(id.clone());
                }

                self.episodes.insert(
                    id.clone(),
                    Episode {
                        id: id.clone(),
                        happened,
                        involved: people,
                        details: detail_map,
                        summary,
                        source,
                        retracted: false,
                    },
                );
                Ok(())
            }

            Stmt::ForgetSource(expr) => {
                let name = self.eval(expr)?.plain();
                self.forget_source(&name);
                Ok(())
            }

            Stmt::ForgetEpisode(id) => {
                self.forget_episode(id);
                Ok(())
            }

            Stmt::ForgetPath(path) => {
                let full = self.qualify(path);
                self.forget_path(&full);
                Ok(())
            }

            Stmt::Let { name, expr } => {
                let value = self.eval(expr)?;
                self.vars.insert(name.clone(), value);
                Ok(())
            }

            Stmt::Show(expr) => {
                let value = self.eval(expr)?;
                let text = value.plain();
                if !self.quiet {
                    println!("{}", text);
                }
                self.output.push(text);
                Ok(())
            }

            Stmt::Expect { left, right, line } => {
                let a = self.eval(left)?;
                let b = self.eval(right)?;
                if a == b {
                    return Ok(());
                }
                // A stale wrapper compares equal to its settled value only when
                // the expectation names the value explicitly.
                if a.settled() == b.settled() && !a.is_stale() && !b.is_stale() {
                    return Ok(());
                }
                Err(PalimpsestError::ExpectationFailed {
                    line: *line,
                    left: a.to_string(),
                    right: b.to_string(),
                })
            }

            Stmt::NowIs(expr) => {
                let v = self.eval(expr)?;
                self.now = self.as_timestamp(&v)?;
                Ok(())
            }

            Stmt::LaterBy(expr) => {
                let v = self.eval(expr)?;
                let d = self.as_duration(&v)?;
                self.now = self.now.plus(d);
                Ok(())
            }
        }
    }

    // ---- writing --------------------------------------------------------

    fn qualify(&self, path: &[String]) -> String {
        if self.scope.is_empty() {
            path.join(".")
        } else {
            let mut all = self.scope.clone();
            all.extend_from_slice(path);
            all.join(".")
        }
    }

    fn rank_of(&self, authority: &str) -> Result<usize, PalimpsestError> {
        self.ranks
            .get(&authority.to_ascii_lowercase())
            .copied()
            .ok_or_else(|| {
                PalimpsestError::Runtime(format!(
                    "`{}` is not one of the authorities this program trusts. Known authorities, strongest first: {}. Declare more with `trust a above b above c`.",
                    authority,
                    self.trust.join(", ")
                ))
            })
    }

    fn inscribe(
        &mut self,
        path: &[String],
        value: Value,
        facets: &Facets,
        line: usize,
    ) -> Result<(), PalimpsestError> {
        let full = self.qualify(path);

        let authority = facets
            .authority
            .clone()
            .unwrap_or_else(|| self.default_authority());
        let rank = self.rank_of(&authority)?;

        let source = match &facets.source {
            Some(expr) => Some(self.eval(expr)?.plain()),
            None => self.ambient_source.clone(),
        };

        // The weakest tier is hearsay by definition; everything else counts as
        // verified once it can name where it came from.
        let bottom = self.ranks.values().copied().min().unwrap_or(0);
        let verified = facets
            .verified
            .unwrap_or_else(|| source.is_some() && rank > bottom);

        let (asserted_at, dated_explicitly) = match &facets.asserted_at {
            Some(expr) => {
                let v = self.eval(expr)?;
                (self.as_timestamp(&v)?, true)
            }
            None => (self.now, false),
        };

        let expires_at = match (&facets.until, &facets.ttl) {
            (Some(expr), _) => {
                let v = self.eval(expr)?;
                Some(self.as_timestamp(&v)?)
            }
            (None, Some(expr)) => {
                let v = self.eval(expr)?;
                let d = self.as_duration(&v)?;
                Some(asserted_at.plus(d))
            }
            (None, None) => None,
        };

        self.record_conflicts(&full, &value, &authority, rank, source.as_deref());

        let id = self.beliefs.len() + 1;
        self.beliefs.push(Belief {
            id,
            path: full.clone(),
            value,
            authority,
            rank,
            provenance: Provenance {
                source: source.clone(),
                verified,
                because: facets.because.clone(),
            },
            asserted_at,
            dated_explicitly,
            expires_at,
            retracted: None,
            origin: format!("{}:{}", self.origin, line),
        });

        self.by_path.entry(full).or_default().push(id);
        if let Some(src) = source {
            self.by_source.entry(src).or_default().push(id);
        }
        if let Some(ep) = &facets.because {
            self.by_episode.entry(ep.clone()).or_default().push(id);
        }

        Ok(())
    }

    /// The tier a fact is filed under when it does not name one. This is the
    /// weakest tier, so an unlabelled claim can never quietly outrank a
    /// labelled one.
    fn default_authority(&self) -> String {
        self.trust.last().cloned().unwrap_or_else(|| "rumor".into())
    }

    /// Notes any live belief on the same name that this write disagrees with
    /// across a difference in standing. Same-standing disagreement is ordinary
    /// supersession and is not a conflict.
    fn record_conflicts(
        &mut self,
        path: &str,
        value: &Value,
        authority: &str,
        rank: usize,
        source: Option<&str>,
    ) {
        let Some(ids) = self.by_path.get(path) else {
            return;
        };

        let mut found = Vec::new();
        for &id in ids {
            let existing = &self.beliefs[id - 1];
            if !existing.is_live() || &existing.value == value || existing.rank == rank {
                continue;
            }

            let incoming_wins = rank > existing.rank;
            found.push(Conflict {
                path: path.to_string(),
                winner_authority: if incoming_wins {
                    authority.to_string()
                } else {
                    existing.authority.clone()
                },
                winner_source: if incoming_wins {
                    source.map(str::to_string)
                } else {
                    existing.provenance.source.clone()
                },
                winner_value: if incoming_wins {
                    value.clone()
                } else {
                    existing.value.clone()
                },
                loser_authority: if incoming_wins {
                    existing.authority.clone()
                } else {
                    authority.to_string()
                },
                loser_source: if incoming_wins {
                    existing.provenance.source.clone()
                } else {
                    source.map(str::to_string)
                },
                loser_value: if incoming_wins {
                    existing.value.clone()
                } else {
                    value.clone()
                },
            });
        }

        self.conflicts.extend(found);
    }

    // ---- forgetting -----------------------------------------------------

    pub fn forget_source(&mut self, name: &str) {
        self.forgotten_sources.insert(name.to_string());

        if let Some(ids) = self.by_source.get(name).cloned() {
            let reason = format!("source `{}` was forgotten", name);
            for id in ids {
                self.retract(id, &reason);
            }
        }

        // Episodes reported by the same document go with it, and so does
        // anything resting on them.
        if let Some(episode_ids) = self.episodes_by_source.get(name).cloned() {
            for episode in episode_ids {
                self.forget_episode(&episode);
            }
        }
    }

    pub fn forget_episode(&mut self, id: &str) {
        if let Some(ep) = self.episodes.get_mut(id) {
            ep.retracted = true;
        }
        if let Some(ids) = self.by_episode.get(id).cloned() {
            let reason = format!("episode `{}` was forgotten", id);
            for bid in ids {
                self.retract(bid, &reason);
            }
        }
    }

    pub fn forget_path(&mut self, path: &str) {
        if let Some(ids) = self.by_path.get(path).cloned() {
            let reason = format!("`{}` was forgotten directly", path);
            for id in ids {
                self.retract(id, &reason);
            }
        }
    }

    fn retract(&mut self, id: usize, reason: &str) {
        if let Some(b) = self.beliefs.get_mut(id - 1) {
            if b.retracted.is_none() {
                b.retracted = Some(reason.to_string());
            }
        }
    }

    // ---- resolution -----------------------------------------------------

    /// Every candidate for a name, paired with the scope depth it was found
    /// at. Depth is searched innermost-first so specificity can break ties.
    fn candidates(&self, path: &[String]) -> Vec<(usize, usize)> {
        let tail = path.join(".");
        let mut out = Vec::new();
        let mut seen = HashSet::new();

        for depth in (0..=self.scope.len()).rev() {
            let full = if depth == 0 {
                tail.clone()
            } else {
                format!("{}.{}", self.scope[..depth].join("."), tail)
            };

            if let Some(ids) = self.by_path.get(&full) {
                for &id in ids {
                    if seen.insert(id) {
                        out.push((id, depth));
                    }
                }
            }
        }

        out
    }

    pub fn resolve(
        &self,
        path: &[String],
        as_of: Option<Timestamp>,
        demands: &Demands,
    ) -> Result<Value, PalimpsestError> {
        let at = as_of.unwrap_or(self.now);
        let name = path.join(".");

        let mut live: Vec<(&Belief, usize)> = self
            .candidates(path)
            .into_iter()
            .map(|(id, depth)| (&self.beliefs[id - 1], depth))
            .filter(|(b, _)| b.is_live() && b.asserted_at <= at)
            .collect();

        if live.is_empty() {
            return Err(PalimpsestError::Unknown {
                path: name,
                scope: self.scope.join("."),
            });
        }

        // Authority, then specificity, then recency.
        live.sort_by(|(a, ad), (b, bd)| {
            b.rank
                .cmp(&a.rank)
                .then(bd.cmp(ad))
                .then(b.asserted_at.cmp(&a.asserted_at))
                .then(b.id.cmp(&a.id))
        });

        let (winner, winner_depth) = live[0];

        // Two beliefs that claim the same stated moment at the same standing
        // are a genuine contradiction; the language will not pick one.
        let tied: Vec<&Belief> = live
            .iter()
            .filter(|(b, d)| {
                b.rank == winner.rank
                    && *d == winner_depth
                    && b.dated_explicitly
                    && winner.dated_explicitly
                    && b.asserted_at == winner.asserted_at
                    && b.value != winner.value
            })
            .map(|(b, _)| *b)
            .collect();

        if !tied.is_empty() {
            let mut values = vec![winner.value.to_string()];
            values.extend(tied.iter().map(|b| b.value.to_string()));
            return Err(PalimpsestError::Contested {
                path: name,
                authority: winner.authority.clone(),
                values,
            });
        }

        if demands.verified && !winner.provenance.verified {
            return Err(PalimpsestError::Unverified {
                path: winner.path.clone(),
                source: winner.provenance.source.clone(),
                authority: winner.authority.clone(),
            });
        }

        if let Some(required) = &demands.min_authority {
            let needed = self.rank_of(required)?;
            if winner.rank < needed {
                return Err(PalimpsestError::Untrusted {
                    path: winner.path.clone(),
                    required: required.to_ascii_lowercase(),
                    actual: winner.authority.clone(),
                });
            }
        }

        if let Some(expiry) = winner.expires_at {
            if at > expiry {
                let over_by = at.since(expiry);
                if demands.fresh {
                    return Err(PalimpsestError::Stale {
                        path: winner.path.clone(),
                        expired_at: expiry,
                        over_by,
                    });
                }
                return Ok(Value::Stale {
                    value: Box::new(winner.value.clone()),
                    age: at.since(winner.asserted_at),
                    lifetime: expiry.since(winner.asserted_at),
                });
            }
        }

        Ok(winner.value.clone())
    }

    /// Every layer ever written under a name, oldest first, each labelled with
    /// why it is or is not the answer today.
    pub fn history(&self, path: &[String]) -> Value {
        let mut ids: Vec<usize> = self.candidates(path).into_iter().map(|(id, _)| id).collect();
        ids.sort_unstable();

        let winner = self
            .resolve_winner_id(path)
            .filter(|_| !ids.is_empty());

        let top_rank = ids
            .iter()
            .map(|&id| &self.beliefs[id - 1])
            .filter(|b| b.is_live())
            .map(|b| b.rank)
            .max();

        let layers = ids
            .into_iter()
            .map(|id| {
                let b = &self.beliefs[id - 1];
                let standing = if let Some(reason) = &b.retracted {
                    Standing::Forgotten {
                        reason: reason.clone(),
                    }
                } else if Some(id) == winner {
                    match b.expires_at {
                        Some(exp) if self.now > exp => Standing::Expired { at: exp },
                        _ => Standing::Current,
                    }
                } else if top_rank.map(|r| b.rank < r).unwrap_or(false) {
                    let w = winner.map(|w| &self.beliefs[w - 1]);
                    Standing::Outranked {
                        by: winner.unwrap_or(0),
                        authority: w.map(|w| w.authority.clone()).unwrap_or_default(),
                    }
                } else {
                    Standing::Overwritten {
                        by: winner.unwrap_or(0),
                        at: winner
                            .map(|w| self.beliefs[w - 1].asserted_at)
                            .unwrap_or(b.asserted_at),
                    }
                };

                Layer {
                    id: b.id,
                    path: b.path.clone(),
                    value: b.value.clone(),
                    authority: b.authority.clone(),
                    source: b.provenance.source.clone(),
                    verified: b.provenance.verified,
                    asserted_at: b.asserted_at,
                    standing,
                }
            })
            .collect();

        Value::History(layers)
    }

    /// The id the ordinary resolution rule selects, ignoring every demand.
    fn resolve_winner_id(&self, path: &[String]) -> Option<usize> {
        let mut live: Vec<(&Belief, usize)> = self
            .candidates(path)
            .into_iter()
            .map(|(id, depth)| (&self.beliefs[id - 1], depth))
            .filter(|(b, _)| b.is_live() && b.asserted_at <= self.now)
            .collect();

        live.sort_by(|(a, ad), (b, bd)| {
            b.rank
                .cmp(&a.rank)
                .then(bd.cmp(ad))
                .then(b.asserted_at.cmp(&a.asserted_at))
                .then(b.id.cmp(&a.id))
        });

        live.first().map(|(b, _)| b.id)
    }

    // ---- check ----------------------------------------------------------

    /// A health pass over the whole store. This is the operation that has no
    /// equivalent in a retrieval system: it reports on the belief set itself
    /// rather than on any particular question.
    pub fn check(&self) -> Report {
        let mut findings = Vec::new();
        let live: Vec<&Belief> = self.beliefs.iter().filter(|b| b.is_live()).collect();

        for b in &live {
            // Anything a `verified` question would refuse is worth reporting
            // before somebody asks that question in production.
            if !b.provenance.verified {
                findings.push(Finding::Unsourced {
                    id: b.id,
                    path: b.path.clone(),
                    authority: b.authority.clone(),
                    source: b.provenance.source.clone(),
                });
            }

            if let Some(ep) = &b.provenance.because {
                if !self.episodes.contains_key(ep) {
                    findings.push(Finding::Orphaned {
                        id: b.id,
                        path: b.path.clone(),
                        episode: ep.clone(),
                    });
                }
            }
        }

        // Staleness and contradiction are properties of what a query would
        // actually reach, so they are evaluated per resolvable name.
        let mut names: Vec<&String> = self.by_path.keys().collect();
        names.sort();

        for name in names {
            let segments: Vec<String> = name.split('.').map(str::to_string).collect();

            let Some(winner_id) = self.resolve_winner_id(&segments) else {
                continue;
            };
            let winner = &self.beliefs[winner_id - 1];

            if let Some(exp) = winner.expires_at {
                if self.now > exp {
                    findings.push(Finding::Stale {
                        id: winner.id,
                        path: winner.path.clone(),
                        expired_at: exp,
                        over_by: self.now.since(exp),
                    });
                }
            }

            if let Err(PalimpsestError::Contested {
                authority, values, ..
            }) = self.resolve(&segments, None, &Demands::default())
            {
                findings.push(Finding::Contested {
                    path: name.clone(),
                    authority,
                    values,
                });
            }
        }

        for c in &self.conflicts {
            findings.push(Finding::Refused {
                path: c.path.clone(),
                loser: c.loser_authority.clone(),
                winner: c.winner_authority.clone(),
            });
        }

        Report {
            findings,
            total_beliefs: self.beliefs.len(),
            live_beliefs: live.len(),
            episodes: self.episodes.values().filter(|e| !e.retracted).count(),
        }
    }

    // ---- evaluation -----------------------------------------------------

    pub fn eval(&mut self, expr: &Expr) -> Result<Value, PalimpsestError> {
        match expr {
            Expr::Literal(v) => Ok(v.clone()),

            Expr::Variable(name) => {
                if let Some(v) = self.vars.get(name) {
                    return Ok(v.clone());
                }
                self.resolve(
                    std::slice::from_ref(name),
                    None,
                    &Demands::default(),
                )
            }

            Expr::Ask {
                path,
                as_of,
                demands,
            } => {
                // A dotted name whose head is a bound variable is a field
                // lookup on that value, not a question about memory.
                if as_of.is_none() && !demands.any() && path.len() > 1 {
                    if let Some(base) = self.vars.get(&path[0]).cloned() {
                        let mut current = base;
                        for field in &path[1..] {
                            current = self.field(current, field)?;
                        }
                        return Ok(current);
                    }
                }

                let at = match as_of {
                    Some(expr) => {
                        let v = self.eval(expr)?;
                        Some(self.as_timestamp(&v)?)
                    }
                    None => None,
                };

                self.resolve(path, at, demands)
            }

            Expr::Why(path) => Ok(self.history(path)),

            Expr::Conflicts => Ok(Value::Conflicts(self.conflicts.clone())),

            Expr::Check => Ok(Value::Report(self.check())),

            Expr::Episodes => {
                let list = self
                    .episodes
                    .values()
                    .filter(|e| !e.retracted)
                    .map(|e| {
                        let mut rec = BTreeMap::new();
                        rec.insert("name".into(), Value::String(e.id.clone()));
                        rec.insert("happened".into(), Value::Timestamp(e.happened));
                        rec.insert(
                            "involved".into(),
                            Value::List(
                                e.involved.iter().cloned().map(Value::String).collect(),
                            ),
                        );
                        rec.insert("details".into(), Value::Record(e.details.clone()));
                        rec.insert("summary".into(), Value::String(e.summary.clone()));
                        Value::Record(rec)
                    })
                    .collect();
                Ok(Value::List(list))
            }

            Expr::List(items) => {
                let mut out = Vec::with_capacity(items.len());
                for item in items {
                    out.push(self.eval(item)?);
                }
                Ok(Value::List(out))
            }

            Expr::Record(fields) => {
                let mut out = BTreeMap::new();
                for (k, v) in fields {
                    let value = self.eval(v)?;
                    out.insert(k.clone(), value);
                }
                Ok(Value::Record(out))
            }

            Expr::Binary { op, left, right } => {
                let a = self.eval(left)?;
                let b = self.eval(right)?;
                binary(*op, a, b)
            }

            Expr::Unary { op, expr } => {
                let v = self.eval(expr)?;
                match (op, v) {
                    (UnOp::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
                    (UnOp::Neg, Value::Int(n)) => Ok(Value::Int(-n)),
                    (UnOp::Neg, Value::Float(n)) => Ok(Value::Float(-n)),
                    (_, other) => Err(PalimpsestError::TypeError(format!(
                        "cannot apply that operator to {}",
                        other.type_name()
                    ))),
                }
            }

            Expr::Field { expr, field } => {
                let base = self.eval(expr)?;
                self.field(base, field)
            }
        }
    }

    fn field(&self, base: Value, field: &str) -> Result<Value, PalimpsestError> {
        match base {
            Value::Record(fields) => fields.get(field).cloned().ok_or_else(|| {
                PalimpsestError::Runtime(format!("this record has no field `{}`", field))
            }),

            Value::Stale {
                value,
                age,
                lifetime,
            } => match field {
                "value" => Ok(*value),
                "stale" => Ok(Value::Bool(true)),
                "age" => Ok(Value::Duration(age)),
                "lifetime" => Ok(Value::Duration(lifetime)),
                other => Err(PalimpsestError::Runtime(format!(
                    "a stale value has `value`, `stale`, `age` and `lifetime`, not `{}`",
                    other
                ))),
            },

            other => match field {
                "stale" => Ok(Value::Bool(false)),
                "value" => Ok(other),
                _ => Err(PalimpsestError::TypeError(format!(
                    "cannot read `{}` from {}",
                    field,
                    other.type_name()
                ))),
            },
        }
    }

    fn as_timestamp(&self, v: &Value) -> Result<Timestamp, PalimpsestError> {
        match v {
            Value::Timestamp(t) => Ok(*t),
            Value::String(s) => Timestamp::parse_iso(s).map_err(PalimpsestError::Runtime),
            Value::Int(n) => Ok(Timestamp::from_secs(*n as u64)),
            other => Err(PalimpsestError::TypeError(format!(
                "expected a date, found {}",
                other.type_name()
            ))),
        }
    }

    fn as_duration(&self, v: &Value) -> Result<Duration, PalimpsestError> {
        match v {
            Value::Duration(d) => Ok(*d),
            Value::String(s) => Duration::parse_str(s).map_err(PalimpsestError::Runtime),
            Value::Int(n) => Ok(Duration::from_secs(*n as u64)),
            other => Err(PalimpsestError::TypeError(format!(
                "expected a length of time, found {}",
                other.type_name()
            ))),
        }
    }
}

fn build_ranks(trust: &[String]) -> HashMap<String, usize> {
    let n = trust.len();
    trust
        .iter()
        .enumerate()
        .map(|(i, name)| (name.to_ascii_lowercase(), (n - i) * 100))
        .collect()
}

fn binary(op: BinOp, a: Value, b: Value) -> Result<Value, PalimpsestError> {
    use BinOp::*;

    if op == Eq {
        return Ok(Value::Bool(a == b));
    }
    if op == NotEq {
        return Ok(Value::Bool(a != b));
    }

    // Adding anything to text builds a sentence. This is the one implicit
    // conversion in the language, and it exists because programs are mostly
    // written to be read aloud.
    if op == Add && (matches!(a, Value::String(_)) || matches!(b, Value::String(_))) {
        return Ok(Value::String(format!("{}{}", a.plain(), b.plain())));
    }

    let out = match (op, &a, &b) {
        (Add, Value::Int(x), Value::Int(y)) => Value::Int(x + y),
        (Add, Value::Float(x), Value::Float(y)) => Value::Float(x + y),
        (Sub, Value::Int(x), Value::Int(y)) => Value::Int(x - y),
        (Sub, Value::Float(x), Value::Float(y)) => Value::Float(x - y),
        (Mul, Value::Int(x), Value::Int(y)) => Value::Int(x * y),
        (Mul, Value::Float(x), Value::Float(y)) => Value::Float(x * y),

        (Div, Value::Int(_), Value::Int(0)) => {
            return Err(PalimpsestError::Runtime("cannot divide by zero".into()))
        }
        (Div, Value::Int(x), Value::Int(y)) => Value::Int(x / y),
        (Div, Value::Float(x), Value::Float(y)) if *y != 0.0 => Value::Float(x / y),

        (Lt, Value::Int(x), Value::Int(y)) => Value::Bool(x < y),
        (LtEq, Value::Int(x), Value::Int(y)) => Value::Bool(x <= y),
        (Gt, Value::Int(x), Value::Int(y)) => Value::Bool(x > y),
        (GtEq, Value::Int(x), Value::Int(y)) => Value::Bool(x >= y),
        (Lt, Value::Float(x), Value::Float(y)) => Value::Bool(x < y),
        (LtEq, Value::Float(x), Value::Float(y)) => Value::Bool(x <= y),
        (Gt, Value::Float(x), Value::Float(y)) => Value::Bool(x > y),
        (GtEq, Value::Float(x), Value::Float(y)) => Value::Bool(x >= y),

        (Lt, Value::Duration(x), Value::Duration(y)) => Value::Bool(x < y),
        (Gt, Value::Duration(x), Value::Duration(y)) => Value::Bool(x > y),

        (And, Value::Bool(x), Value::Bool(y)) => Value::Bool(*x && *y),
        (Or, Value::Bool(x), Value::Bool(y)) => Value::Bool(*x || *y),

        _ => {
            return Err(PalimpsestError::TypeError(format!(
                "cannot combine {} and {} that way",
                a.type_name(),
                b.type_name()
            )))
        }
    };

    Ok(out)
}
