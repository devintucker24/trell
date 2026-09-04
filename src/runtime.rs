// Palimpsest Runtime Engine: Epistemic Store, Scopes, Authorities, Lifetimes & TMS

use std::collections::{BTreeMap, HashMap, HashSet};
use crate::ast::*;
use crate::error::PalimpsestError;
use crate::time::{Duration, Timestamp};
use crate::types::*;

#[derive(Debug)]
pub struct Runtime {
    pub current_time: Timestamp,
    pub authority_lattice: HashMap<String, usize>,
    pub next_belief_id: usize,
    pub beliefs: Vec<Belief>,
    pub path_to_beliefs: HashMap<String, Vec<usize>>,
    pub episodes: HashMap<String, Episode>,
    pub retracted_sources: HashSet<String>,
    pub retracted_episodes: HashSet<String>,
    pub source_to_beliefs: HashMap<String, Vec<usize>>,
    pub episode_to_beliefs: HashMap<String, Vec<usize>>,
    pub conflict_log: Vec<DefeasanceConflict>,
    pub scope_stack: Vec<String>,
    pub variables: HashMap<String, Value>,
    pub output_log: Vec<String>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    pub fn new() -> Self {
        let mut authority_lattice = HashMap::new();
        // Default authority lattice
        authority_lattice.insert("System".to_string(), 600);
        authority_lattice.insert("Legal".to_string(), 500);
        authority_lattice.insert("Compliance".to_string(), 400);
        authority_lattice.insert("Policy".to_string(), 300);
        authority_lattice.insert("VerifiedUser".to_string(), 200);
        authority_lattice.insert("User".to_string(), 100);
        authority_lattice.insert("Guest".to_string(), 50);
        authority_lattice.insert("Unverified".to_string(), 0);

        // Default virtual clock: 2026-09-04T12:00:00Z
        let initial_time = Timestamp::parse_iso("2026-09-04T12:00:00Z").unwrap();

        Self {
            current_time: initial_time,
            authority_lattice,
            next_belief_id: 1,
            beliefs: Vec::new(),
            path_to_beliefs: HashMap::new(),
            episodes: HashMap::new(),
            retracted_sources: HashSet::new(),
            retracted_episodes: HashSet::new(),
            source_to_beliefs: HashMap::new(),
            episode_to_beliefs: HashMap::new(),
            conflict_log: Vec::new(),
            scope_stack: Vec::new(),
            variables: HashMap::new(),
            output_log: Vec::new(),
        }
    }

    pub fn execute_program(&mut self, program: &Program) -> Result<(), PalimpsestError> {
        for stmt in &program.statements {
            self.execute_stmt(stmt)?;
        }
        Ok(())
    }

    pub fn execute_stmt(&mut self, stmt: &Stmt) -> Result<(), PalimpsestError> {
        match stmt {
            Stmt::AuthorityDecl(tiers) => {
                self.authority_lattice.clear();
                let count = tiers.len();
                for (idx, tier) in tiers.iter().enumerate() {
                    let rank = (count - idx) * 100;
                    self.authority_lattice.insert(tier.clone(), rank);
                }
                Ok(())
            }

            Stmt::Scope { prefix, body } => {
                let saved_len = self.scope_stack.len();
                self.scope_stack.extend(prefix.clone());
                for inner in body {
                    self.execute_stmt(inner)?;
                }
                self.scope_stack.truncate(saved_len);
                Ok(())
            }

            Stmt::Assert { path, value, modifiers } => {
                let eval_value = self.eval_expr(value)?;
                self.assert_belief(path, eval_value, modifiers)
            }

            Stmt::Episode { id, at, actors, context, summary } => {
                let at_val = self.eval_expr(at)?;
                let at_ts = self.value_to_timestamp(&at_val)?;

                let mut actor_strings = Vec::new();
                for a in actors {
                    let av = self.eval_expr(a)?;
                    match av {
                        Value::String(s) => actor_strings.push(s),
                        other => actor_strings.push(format!("{}", other)),
                    }
                }

                let mut context_map = BTreeMap::new();
                for (k, v_expr) in context {
                    let v = self.eval_expr(v_expr)?;
                    context_map.insert(k.clone(), v);
                }

                let summary_val = self.eval_expr(summary)?;
                let summary_str = match summary_val {
                    Value::String(s) => s,
                    other => format!("{}", other),
                };

                let ep = Episode {
                    id: id.clone(),
                    at: at_ts,
                    actors: actor_strings,
                    context: context_map,
                    summary: summary_str,
                    is_retracted: false,
                };

                self.episodes.insert(id.clone(), ep);
                Ok(())
            }

            Stmt::RetractSource(expr) => {
                let val = self.eval_expr(expr)?;
                let source_name = match val {
                    Value::String(s) => s,
                    other => format!("{}", other),
                };
                self.retract_source(&source_name);
                Ok(())
            }

            Stmt::RetractBelief(path) => {
                let canonical_path = self.resolve_canonical_path(path);
                self.retract_belief(&canonical_path);
                Ok(())
            }

            Stmt::RetractEpisode(ep_id) => {
                self.retract_episode(ep_id);
                Ok(())
            }

            Stmt::Let { name, expr } => {
                let val = self.eval_expr(expr)?;
                self.variables.insert(name.clone(), val);
                Ok(())
            }

            Stmt::Print(expr) => {
                let val = self.eval_expr(expr)?;
                let line = format!("{}", val);
                println!("{}", line);
                self.output_log.push(line);
                Ok(())
            }

            Stmt::AssertEq { left, right } => {
                let l_val = self.eval_expr(left)?;
                let r_val = self.eval_expr(right)?;

                // Compare unwrap_value if both are comparable
                if l_val != r_val {
                    return Err(PalimpsestError::AssertionFailed {
                        message: "Values do not match".to_string(),
                        left: format!("{}", l_val),
                        right: format!("{}", r_val),
                    });
                }
                Ok(())
            }

            Stmt::SetTime(expr) => {
                let val = self.eval_expr(expr)?;
                self.current_time = self.value_to_timestamp(&val)?;
                Ok(())
            }

            Stmt::AdvanceTime(expr) => {
                let val = self.eval_expr(expr)?;
                let dur = self.value_to_duration(&val)?;
                self.current_time = self.current_time.add_duration(dur);
                Ok(())
            }

            Stmt::Expr(expr) => {
                self.eval_expr(expr)?;
                Ok(())
            }
        }
    }

    pub fn get_authority_rank(&self, auth_name: &str) -> usize {
        self.authority_lattice.get(auth_name).copied().unwrap_or(50)
    }

    fn resolve_canonical_path(&self, segments: &[String]) -> String {
        if self.scope_stack.is_empty() {
            segments.join(".")
        } else {
            let mut full = self.scope_stack.clone();
            full.extend_from_slice(segments);
            full.join(".")
        }
    }

    fn assert_belief(
        &mut self,
        path_segments: &[String],
        value: Value,
        modifiers: &AssertModifiers,
    ) -> Result<(), PalimpsestError> {
        let canonical_path = self.resolve_canonical_path(path_segments);

        // Authority
        let authority = modifiers.authority.clone().unwrap_or_else(|| "User".to_string());
        let authority_rank = self.get_authority_rank(&authority);

        // Source
        let source = if let Some(ref src_expr) = modifiers.source {
            let v = self.eval_expr(src_expr)?;
            Some(match v {
                Value::String(s) => s,
                other => format!("{}", other),
            })
        } else {
            None
        };

        // Verified flag
        let verified = if let Some(v) = modifiers.verified {
            v
        } else if authority == "Unverified" {
            false
        } else if authority_rank >= self.get_authority_rank("VerifiedUser") {
            true
        } else {
            source.is_some()
        };

        // Asserted at timestamp
        let (asserted_at, explicit_timestamp) = if let Some(ref at_expr) = modifiers.at {
            let v = self.eval_expr(at_expr)?;
            (self.value_to_timestamp(&v)?, true)
        } else {
            (self.current_time, false)
        };

        // TTL and valid_until
        let valid_until = if let Some(ref vu_expr) = modifiers.valid_until {
            let v = self.eval_expr(vu_expr)?;
            Some(self.value_to_timestamp(&v)?)
        } else if let Some(ref ttl_expr) = modifiers.ttl {
            let v = self.eval_expr(ttl_expr)?;
            let dur = self.value_to_duration(&v)?;
            Some(asserted_at.add_duration(dur))
        } else {
            None
        };

        // Episode grounding
        let grounded_in = modifiers.grounded_in.clone();

        // Check for Defeasance Conflict against existing active beliefs on the same path
        if let Some(existing_ids) = self.path_to_beliefs.get(&canonical_path) {
            for &b_id in existing_ids {
                let existing = &self.beliefs[b_id - 1];
                if !existing.is_retracted && existing.value != value {
                    if existing.authority_rank > authority_rank {
                        // High authority already asserted a different value!
                        // The incoming assertion is defeated by existing higher authority.
                        self.conflict_log.push(DefeasanceConflict {
                            path: canonical_path.clone(),
                            high_authority: existing.authority.clone(),
                            high_source: existing.provenance.source.clone(),
                            high_value: existing.value.clone(),
                            low_authority: authority.clone(),
                            low_source: source.clone(),
                            low_value: value.clone(),
                            reason: format!(
                                "Attempted override by authority '{}' defeated by established authority '{}'",
                                authority, existing.authority
                            ),
                        });
                    } else if existing.authority_rank < authority_rank {
                        // High authority supersedes an older lower-authority belief.
                        // We also log the override notice.
                        self.conflict_log.push(DefeasanceConflict {
                            path: canonical_path.clone(),
                            high_authority: authority.clone(),
                            high_source: source.clone(),
                            high_value: value.clone(),
                            low_authority: existing.authority.clone(),
                            low_source: existing.provenance.source.clone(),
                            low_value: existing.value.clone(),
                            reason: format!(
                                "Higher authority '{}' superseded lower-authority belief from '{}'",
                                authority, existing.authority
                            ),
                        });
                    }
                }
            }
        }

        let id = self.next_belief_id;
        self.next_belief_id += 1;

        let provenance = Provenance::new(source.clone(), verified, grounded_in.clone());

        let belief = Belief {
            id,
            path: canonical_path.clone(),
            value,
            authority,
            authority_rank,
            provenance,
            asserted_at,
            explicit_timestamp,
            valid_until,
            is_retracted: false,
            retraction_reason: None,
        };

        self.beliefs.push(belief);
        self.path_to_beliefs.entry(canonical_path.clone()).or_default().push(id);

        if let Some(src) = source {
            self.source_to_beliefs.entry(src).or_default().push(id);
        }

        if let Some(ep) = grounded_in {
            self.episode_to_beliefs.entry(ep).or_default().push(id);
        }

        Ok(())
    }

    pub fn retract_source(&mut self, source_name: &str) {
        self.retracted_sources.insert(source_name.to_string());
        if let Some(b_ids) = self.source_to_beliefs.get(source_name).cloned() {
            for id in b_ids {
                self.retract_belief_by_id(id, &format!("Retraction of source '{}'", source_name));
            }
        }
    }

    pub fn retract_episode(&mut self, episode_id: &str) {
        self.retracted_episodes.insert(episode_id.to_string());
        if let Some(ep) = self.episodes.get_mut(episode_id) {
            ep.is_retracted = true;
        }
        if let Some(b_ids) = self.episode_to_beliefs.get(episode_id).cloned() {
            for id in b_ids {
                self.retract_belief_by_id(id, &format!("Retraction of episode '{}'", episode_id));
            }
        }
    }

    pub fn retract_belief(&mut self, canonical_path: &str) {
        if let Some(ids) = self.path_to_beliefs.get(canonical_path).cloned() {
            for id in ids {
                self.retract_belief_by_id(id, &format!("Direct retraction of path '{}'", canonical_path));
            }
        }
    }

    fn retract_belief_by_id(&mut self, id: usize, reason: &str) {
        if id > 0 && id <= self.beliefs.len() {
            let belief = &mut self.beliefs[id - 1];
            if !belief.is_retracted {
                belief.is_retracted = true;
                belief.retraction_reason = Some(reason.to_string());
            }
        }
    }

    pub fn resolve_path(
        &self,
        path_segments: &[String],
        as_of: Option<Timestamp>,
        fresh: bool,
        verified_only: bool,
        min_authority: Option<&str>,
    ) -> Result<Value, PalimpsestError> {
        let eval_time = as_of.unwrap_or(self.current_time);

        // Build potential candidate path matches:
        // 1. Qualified in current scope: scope_stack + path_segments
        // 2. Exact match: path_segments
        // 3. Suffix match if partial
        let exact_path = path_segments.join(".");
        let scoped_path = if !self.scope_stack.is_empty() {
            format!("{}.{}", self.scope_stack.join("."), exact_path)
        } else {
            exact_path.clone()
        };

        // Find candidate IDs
        let mut candidate_ids: Vec<usize> = Vec::new();
        if let Some(ids) = self.path_to_beliefs.get(&scoped_path) {
            candidate_ids.extend(ids);
        }
        if scoped_path != exact_path {
            if let Some(ids) = self.path_to_beliefs.get(&exact_path) {
                for id in ids {
                    if !candidate_ids.contains(id) {
                        candidate_ids.push(*id);
                    }
                }
            }
        }

        // If still empty, search for any path ending in exact_path
        if candidate_ids.is_empty() {
            for (p, ids) in &self.path_to_beliefs {
                if p.ends_with(&exact_path) {
                    for id in ids {
                        if !candidate_ids.contains(id) {
                            candidate_ids.push(*id);
                        }
                    }
                }
            }
        }

        if candidate_ids.is_empty() {
            return Err(PalimpsestError::PathNotFoundError {
                path: exact_path,
                scope: self.scope_stack.join("."),
            });
        }

        // Filter valid candidates at eval_time
        let mut valid_candidates: Vec<&Belief> = Vec::new();
        for &id in &candidate_ids {
            let b = &self.beliefs[id - 1];
            // Must not be retracted
            if b.is_retracted {
                continue;
            }
            // Must have been asserted on or before eval_time
            if b.asserted_at.0 > eval_time.0 {
                continue;
            }
            valid_candidates.push(b);
        }

        if valid_candidates.is_empty() {
            return Err(PalimpsestError::PathNotFoundError {
                path: exact_path,
                scope: format!("{}(at time {})", self.scope_stack.join("."), eval_time.to_iso()),
            });
        }

        // Step 1: Authority Lattice Dominance
        // Find highest authority rank
        let max_rank = valid_candidates.iter().map(|b| b.authority_rank).max().unwrap();
        let top_auth_candidates: Vec<&Belief> = valid_candidates
            .into_iter()
            .filter(|b| b.authority_rank == max_rank)
            .collect();

        // Step 2: Recency / Shadowing within highest authority
        // Sort by asserted_at descending, tie-breaking with insertion id descending
        let mut sorted = top_auth_candidates;
        sorted.sort_by(|a, b| (b.asserted_at, b.id).cmp(&(a.asserted_at, a.id)));

        let winning_belief = sorted[0];

        // Check for equal-authority contradiction:
        // Only if two beliefs share an explicitly asserted identical timestamp at equal authority
        for other in sorted.iter().skip(1) {
            if other.explicit_timestamp
                && winning_belief.explicit_timestamp
                && other.asserted_at == winning_belief.asserted_at
                && other.value != winning_belief.value
            {
                return Err(PalimpsestError::ContradictionError {
                    path: exact_path,
                    conflicting_values: vec![
                        format!("{}", winning_belief.value),
                        format!("{}", other.value),
                    ],
                    authority: winning_belief.authority.clone(),
                });
            }
        }

        // Step 3: Provenance & Verification guard
        if verified_only && (!winning_belief.provenance.verified || winning_belief.authority == "Unverified") {
            return Err(PalimpsestError::UnverifiedBeliefRefusal {
                path: winning_belief.path.clone(),
                source: winning_belief.provenance.source.clone(),
                authority: winning_belief.authority.clone(),
                reason: "Query explicitly demanded 'verified', but belief lacks verified provenance or authentic authority."
                    .to_string(),
            });
        }

        // Step 4: Minimum Authority guard
        if let Some(req_auth) = min_authority {
            let req_rank = self.get_authority_rank(req_auth);
            if winning_belief.authority_rank < req_rank {
                return Err(PalimpsestError::InsufficientAuthorityError {
                    path: winning_belief.path.clone(),
                    required_authority: req_auth.to_string(),
                    actual_authority: winning_belief.authority.clone(),
                });
            }
        }

        // Step 5: Staleness / Expiry check
        if let Some(vu) = winning_belief.valid_until {
            if eval_time.0 > vu.0 {
                let age = eval_time.0.saturating_sub(winning_belief.asserted_at.0);
                let ttl = vu.0.saturating_sub(winning_belief.asserted_at.0);

                if fresh {
                    return Err(PalimpsestError::StaleBeliefError {
                        path: winning_belief.path.clone(),
                        age_secs: age,
                        ttl_secs: ttl,
                        expired_at: vu.to_iso(),
                    });
                } else {
                    return Ok(Value::Stale {
                        value: Box::new(winning_belief.value.clone()),
                        age_secs: age,
                        ttl_secs: ttl,
                    });
                }
            }
        }

        Ok(winning_belief.value.clone())
    }

    pub fn audit_path(&self, path_segments: &[String]) -> Value {
        let exact_path = path_segments.join(".");
        let scoped_path = if !self.scope_stack.is_empty() {
            format!("{}.{}", self.scope_stack.join("."), exact_path)
        } else {
            exact_path.clone()
        };

        let mut candidate_ids = Vec::new();
        if let Some(ids) = self.path_to_beliefs.get(&scoped_path) {
            candidate_ids.extend(ids);
        }
        if scoped_path != exact_path {
            if let Some(ids) = self.path_to_beliefs.get(&exact_path) {
                for id in ids {
                    if !candidate_ids.contains(id) {
                        candidate_ids.push(*id);
                    }
                }
            }
        }
        if candidate_ids.is_empty() {
            for (p, ids) in &self.path_to_beliefs {
                if p.ends_with(&exact_path) {
                    for id in ids {
                        if !candidate_ids.contains(id) {
                            candidate_ids.push(*id);
                        }
                    }
                }
            }
        }

        // Find active max authority among non-retracted beliefs
        let max_active_rank = candidate_ids
            .iter()
            .map(|&id| &self.beliefs[id - 1])
            .filter(|b| !b.is_retracted)
            .map(|b| b.authority_rank)
            .max();

        // Find active winner id
        let active_winner_id = if let Some(max_rank) = max_active_rank {
            let mut top: Vec<&Belief> = candidate_ids
                .iter()
                .map(|&id| &self.beliefs[id - 1])
                .filter(|b| !b.is_retracted && b.authority_rank == max_rank)
                .collect();
            top.sort_by(|a, b| b.asserted_at.cmp(&a.asserted_at));
            top.first().map(|b| b.id)
        } else {
            None
        };

        let mut entries = Vec::new();

        for &id in &candidate_ids {
            let b = &self.beliefs[id - 1];
            let status = if b.is_retracted {
                AuditStatus::Retracted {
                    reason: b.retraction_reason.clone().unwrap_or_else(|| "Retracted".to_string()),
                }
            } else if Some(b.id) == active_winner_id {
                if let Some(vu) = b.valid_until {
                    if self.current_time.0 > vu.0 {
                        AuditStatus::Expired { expired_at: vu }
                    } else {
                        AuditStatus::Active
                    }
                } else {
                    AuditStatus::Active
                }
            } else if let Some(max_rank) = max_active_rank {
                if b.authority_rank < max_rank {
                    AuditStatus::DefeatedByHigherAuthority {
                        belief_id: active_winner_id.unwrap_or(0),
                        authority: if let Some(wid) = active_winner_id {
                            self.beliefs[wid - 1].authority.clone()
                        } else {
                            "Higher".to_string()
                        },
                    }
                } else {
                    // Same authority rank, but shadowed by newer assertion
                    AuditStatus::ShadowedBy {
                        belief_id: active_winner_id.unwrap_or(0),
                        timestamp: if let Some(wid) = active_winner_id {
                            self.beliefs[wid - 1].asserted_at
                        } else {
                            b.asserted_at
                        },
                    }
                }
            } else {
                AuditStatus::Active
            };

            entries.push(AuditEntry {
                belief_id: b.id,
                path: b.path.clone(),
                value: b.value.clone(),
                authority: b.authority.clone(),
                source: b.provenance.source.clone(),
                verified: b.provenance.verified,
                timestamp: b.asserted_at,
                valid_until: b.valid_until,
                status,
            });
        }

        Value::AuditLog(entries)
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value, PalimpsestError> {
        match expr {
            Expr::Literal(val) => Ok(val.clone()),

            Expr::Variable(name) => {
                if let Some(v) = self.variables.get(name) {
                    Ok(v.clone())
                } else {
                    // Try resolving as 1-segment path in epistemic memory
                    match self.resolve_path(&[name.clone()], None, false, false, None) {
                        Ok(v) => Ok(v),
                        Err(_) => Err(PalimpsestError::RuntimeError(format!("Undefined variable or memory path: '{}'", name))),
                    }
                }
            }

            Expr::Path(segments) => {
                if !segments.is_empty() && self.variables.contains_key(&segments[0]) {
                    let mut current = self.variables.get(&segments[0]).unwrap().clone();
                    for field in &segments[1..] {
                        current = self.eval_field_access(current, field)?;
                    }
                    Ok(current)
                } else {
                    self.resolve_path(segments, None, false, false, None)
                }
            }

            Expr::Recall { path, as_of, fresh, verified_only, min_authority } => {
                let as_of_ts = if let Some(as_of_expr) = as_of {
                    let v = self.eval_expr(as_of_expr)?;
                    Some(self.value_to_timestamp(&v)?)
                } else {
                    None
                };

                self.resolve_path(
                    path,
                    as_of_ts,
                    *fresh,
                    *verified_only,
                    min_authority.as_deref(),
                )
            }

            Expr::History(path) | Expr::Audit(path) => {
                Ok(self.audit_path(path))
            }

            Expr::Conflicts => {
                Ok(Value::ConflictList(self.conflict_log.clone()))
            }

            Expr::Episodes => {
                let mut list = Vec::new();
                for ep in self.episodes.values() {
                    if !ep.is_retracted {
                        let mut rec = BTreeMap::new();
                        rec.insert("id".to_string(), Value::String(ep.id.clone()));
                        rec.insert("at".to_string(), Value::Timestamp(ep.at));
                        rec.insert("actors".to_string(), Value::List(ep.actors.iter().map(|a| Value::String(a.clone())).collect()));
                        rec.insert("context".to_string(), Value::Record(ep.context.clone()));
                        rec.insert("summary".to_string(), Value::String(ep.summary.clone()));
                        list.push(Value::Record(rec));
                    }
                }
                Ok(Value::List(list))
            }

            Expr::List(exprs) => {
                let mut vals = Vec::new();
                for e in exprs {
                    vals.push(self.eval_expr(e)?);
                }
                Ok(Value::List(vals))
            }

            Expr::Record(fields) => {
                let mut rec = BTreeMap::new();
                for (k, e) in fields {
                    rec.insert(k.clone(), self.eval_expr(e)?);
                }
                Ok(Value::Record(rec))
            }

            Expr::BinaryOp { op, left, right } => {
                let l_val = self.eval_expr(left)?;
                let r_val = self.eval_expr(right)?;
                self.eval_binop(*op, l_val, r_val)
            }

            Expr::UnaryOp { op, expr } => {
                let val = self.eval_expr(expr)?;
                match op {
                    UnOp::Not => match val {
                        Value::Bool(b) => Ok(Value::Bool(!b)),
                        other => Err(PalimpsestError::TypeError(format!("Cannot apply '!' to {}", other.type_name()))),
                    },
                    UnOp::Neg => match val {
                        Value::Int(n) => Ok(Value::Int(-n)),
                        Value::Float(n) => Ok(Value::Float(-n)),
                        other => Err(PalimpsestError::TypeError(format!("Cannot apply '-' to {}", other.type_name()))),
                    },
                }
            }

            Expr::FieldAccess { expr, field } => {
                let base = self.eval_expr(expr)?;
                self.eval_field_access(base, field)
            }
        }
    }

    pub fn eval_field_access(&self, base: Value, field: &str) -> Result<Value, PalimpsestError> {
        match base {
            Value::Record(entries) => {
                entries.get(field).cloned().ok_or_else(|| {
                    PalimpsestError::RuntimeError(format!("Record missing field '{}'", field))
                })
            }
            Value::Stale { value, age_secs, ttl_secs } => {
                match field {
                    "value" => Ok(*value),
                    "age" => Ok(Value::Int(age_secs as i64)),
                    "ttl" => Ok(Value::Int(ttl_secs as i64)),
                    "is_stale" => Ok(Value::Bool(true)),
                    other => Err(PalimpsestError::RuntimeError(format!("Unknown field '{}' on Stale value", other))),
                }
            }
            other => {
                if field == "is_stale" {
                    Ok(Value::Bool(false))
                } else if field == "type" {
                    Ok(Value::String(other.type_name().to_string()))
                } else {
                    Err(PalimpsestError::TypeError(format!(
                        "Cannot access field '{}' on type {}",
                        field,
                        other.type_name()
                    )))
                }
            }
        }
    }

    fn eval_binop(&self, op: BinOp, left: Value, right: Value) -> Result<Value, PalimpsestError> {
        // Handle equality and inequality for all values
        if op == BinOp::Eq {
            return Ok(Value::Bool(left == right));
        }
        if op == BinOp::NotEq {
            return Ok(Value::Bool(left != right));
        }

        match (op, left, right) {
            (BinOp::Add, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (BinOp::Add, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (BinOp::Add, Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (BinOp::Sub, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (BinOp::Sub, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (BinOp::Mul, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (BinOp::Mul, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (BinOp::Div, Value::Int(a), Value::Int(b)) => {
                if b == 0 {
                    Err(PalimpsestError::RuntimeError("Division by zero".to_string()))
                } else {
                    Ok(Value::Int(a / b))
                }
            }
            (BinOp::Div, Value::Float(a), Value::Float(b)) => {
                if b == 0.0 {
                    Err(PalimpsestError::RuntimeError("Division by zero".to_string()))
                } else {
                    Ok(Value::Float(a / b))
                }
            }
            (BinOp::Lt, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            (BinOp::LtEq, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            (BinOp::Gt, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            (BinOp::GtEq, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            (BinOp::Lt, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
            (BinOp::LtEq, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
            (BinOp::Gt, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
            (BinOp::GtEq, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
            (BinOp::And, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a && b)),
            (BinOp::Or, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a || b)),
            (op, l, r) => Err(PalimpsestError::TypeError(format!(
                "Invalid operands for {:?}: {} and {}",
                op,
                l.type_name(),
                r.type_name()
            ))),
        }
    }

    pub fn value_to_timestamp(&self, val: &Value) -> Result<Timestamp, PalimpsestError> {
        match val {
            Value::Timestamp(t) => Ok(*t),
            Value::String(s) => Timestamp::parse_iso(s).map_err(|e| PalimpsestError::RuntimeError(e)),
            Value::Int(i) => Ok(Timestamp::from_secs(*i as u64)),
            other => Err(PalimpsestError::TypeError(format!("Expected timestamp, found {}", other.type_name()))),
        }
    }

    pub fn value_to_duration(&self, val: &Value) -> Result<Duration, PalimpsestError> {
        match val {
            Value::Duration(d) => Ok(*d),
            Value::String(s) => Duration::parse_str(s).map_err(|e| PalimpsestError::RuntimeError(e)),
            Value::Int(i) => Ok(Duration::from_secs(*i as u64)),
            other => Err(PalimpsestError::TypeError(format!("Expected duration, found {}", other.type_name()))),
        }
    }
}
