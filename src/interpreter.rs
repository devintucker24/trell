use std::collections::HashMap;
use std::fmt;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::ast::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeliefValue {
    pub value: Box<RuntimeValue>,
    pub confidence: f64,
    pub justification: String,
    pub model_origin: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RuntimeValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Unit,
    Struct {
        name: String,
        fields: HashMap<String, RuntimeValue>,
    },
    Belief(BeliefValue),
}

impl RuntimeValue {
    pub fn is_truthy(&self) -> bool {
        match self {
            RuntimeValue::Bool(b) => *b,
            RuntimeValue::Int(n) => *n != 0,
            RuntimeValue::String(s) => !s.is_empty(),
            RuntimeValue::Belief(b) => b.value.is_truthy(),
            _ => false,
        }
    }

    pub fn unwrap_belief(&self) -> (&RuntimeValue, Option<f64>, Option<&str>) {
        match self {
            RuntimeValue::Belief(b) => (&b.value, Some(b.confidence), Some(&b.justification)),
            other => (other, Some(1.0), None),
        }
    }
}

impl fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeValue::Int(n) => write!(f, "{}", n),
            RuntimeValue::Float(fl) => write!(f, "{:.4}", fl),
            RuntimeValue::Bool(b) => write!(f, "{}", b),
            RuntimeValue::String(s) => write!(f, "{}", s),
            RuntimeValue::Unit => write!(f, "()"),
            RuntimeValue::Struct { name, fields } => {
                write!(f, "{} {{ ", name)?;
                let mut first = true;
                for (k, v) in fields {
                    if !first { write!(f, ", ")?; }
                    write!(f, "{}: {}", k, v)?;
                    first = false;
                }
                write!(f, " }}")
            }
            RuntimeValue::Belief(b) => {
                write!(f, "belief<{}>({} [confidence: {:.2}, rationale: \"{}\"])",
                    b.model_origin, b.value, b.confidence, b.justification
                )
            }
        }
    }
}

pub trait ModelOracle {
    fn query(&mut self, contract: &ModelContract, method: &str, prompt: &str) -> Result<BeliefValue>;
}

/// A deterministic, self-contained mock oracle for testing and CI
pub struct MockOracle {
    canned_responses: HashMap<String, (String, f64, String)>,
}

impl MockOracle {
    pub fn new() -> Self {
        let mut canned = HashMap::new();
        // Default heuristics for common methods and prompts
        canned.insert("assess_medical".to_string(), (
            "BacterialInfection".to_string(),
            0.94,
            "Elevated leukocytes and high inflammatory markers indicate bacterial pathogenesis".to_string(),
        ));
        canned.insert("evaluate_risk".to_string(), (
            "ApproveTransfer".to_string(),
            0.89,
            "Transaction risk metrics are well within acceptable velocity bounds".to_string(),
        ));
        canned.insert("audit_code".to_string(), (
            "ApprovedPatch".to_string(),
            0.96,
            "No dangerous shell interpolation or unescaped memory access identified".to_string(),
        ));
        Self { canned_responses: canned }
    }

    pub fn set_response(&mut self, key: &str, value: &str, confidence: f64, justification: &str) {
        self.canned_responses.insert(key.to_string(), (value.to_string(), confidence, justification.to_string()));
    }
}

impl ModelOracle for MockOracle {
    fn query(&mut self, contract: &ModelContract, method: &str, prompt: &str) -> Result<BeliefValue> {
        // Match prompt or method against registered heuristics
        let (val, conf, just) = if let Some(resp) = self.canned_responses.get(method) {
            resp.clone()
        } else {
            let mut matched = None;
            for (key, resp) in &self.canned_responses {
                if prompt.to_lowercase().contains(&key.to_lowercase()) {
                    matched = Some(resp.clone());
                    break;
                }
            }
            matched.unwrap_or_else(|| {
                // Heuristic mock response
                (
                    "Positive".to_string(),
                    0.89,
                    format!("Model deliberated on contract '{}' with method '{}'", contract.name, method),
                )
            })
        };

        // Check contract minimum confidence invariant if defined
        if let Some(min_conf) = contract.min_confidence {
            if conf < min_conf {
                return Err(anyhow!(
                    "ModelContract '{}' invariant failed: returned confidence {:.2} is lower than required min_confidence {:.2}",
                    contract.name,
                    conf,
                    min_conf
                ));
            }
        }

        Ok(BeliefValue {
            value: Box::new(RuntimeValue::String(val)),
            confidence: conf,
            justification: just,
            model_origin: contract.name.clone(),
        })
    }
}

pub struct SpeculativeForkTrace {
    pub target_value: String,
    pub chosen_branch: String,
    pub rolled_back_branches: Vec<String>,
}

pub struct Interpreter<'a> {
    program: &'a Program,
    oracle: Box<dyn ModelOracle + 'a>,
    scopes: Vec<HashMap<String, RuntimeValue>>,
    pub traces: Vec<SpeculativeForkTrace>,
    pub execution_log: Vec<String>,
}

impl<'a> Interpreter<'a> {
    pub fn new(program: &'a Program, oracle: Box<dyn ModelOracle + 'a>) -> Self {
        Self {
            program,
            oracle,
            scopes: vec![HashMap::new()],
            traces: Vec::new(),
            execution_log: Vec::new(),
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn define_var(&mut self, name: &str, val: RuntimeValue) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), val);
        }
    }

    fn set_var(&mut self, name: &str, val: RuntimeValue) -> Result<()> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), val);
                return Ok(());
            }
        }
        Err(anyhow!("Cannot assign to undefined variable '{}'", name))
    }

    fn get_var(&self, name: &str) -> Option<RuntimeValue> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Some(val.clone());
            }
        }
        None
    }

    pub fn run_main(&mut self) -> Result<RuntimeValue> {
        let main_fn = self.program.items.iter().find_map(|item| {
            if let Item::Function(f) = item {
                if f.name == "main" {
                    return Some(f);
                }
            }
            None
        }).ok_or_else(|| anyhow!("No 'fn main()' entry point found in program"))?;

        self.push_scope();
        let mut ret = RuntimeValue::Unit;
        for stmt in &main_fn.body {
            if let Some(r) = self.eval_stmt(stmt)? {
                ret = r;
                break;
            }
        }
        self.pop_scope();
        Ok(ret)
    }

    pub fn call_function(&mut self, name: &str, args: Vec<RuntimeValue>) -> Result<RuntimeValue> {
        let f = self.program.items.iter().find_map(|item| {
            if let Item::Function(f) = item {
                if f.name == name {
                    return Some(f.clone());
                }
            }
            None
        }).ok_or_else(|| anyhow!("Undefined function '{}'", name))?;

        if f.params.len() != args.len() {
            return Err(anyhow!("Function '{}' expected {} args, got {}", name, f.params.len(), args.len()));
        }

        self.push_scope();
        for (param, arg) in f.params.iter().zip(args) {
            self.define_var(&param.name, arg);
        }

        let mut ret = RuntimeValue::Unit;
        let num_stmts = f.body.len();
        for (idx, stmt) in f.body.iter().enumerate() {
            if let Some(r) = self.eval_stmt(stmt)? {
                ret = r;
                break;
            } else if idx == num_stmts - 1 {
                // If last statement is an Expr, treat its value as implicit return
                if let Stmt::Expr(expr) = stmt {
                    ret = self.eval_expr(expr)?;
                }
            }
        }
        self.pop_scope();
        Ok(ret)
    }

    fn eval_stmt(&mut self, stmt: &Stmt) -> Result<Option<RuntimeValue>> {
        match stmt {
            Stmt::Let { name, value, .. } => {
                let val = self.eval_expr(value)?;
                self.define_var(name, val);
                Ok(None)
            }
            Stmt::Assign { target, value } => {
                let val = self.eval_expr(value)?;
                self.set_var(target, val)?;
                Ok(None)
            }
            Stmt::Return(opt_expr) => {
                let val = if let Some(expr) = opt_expr {
                    self.eval_expr(expr)?
                } else {
                    RuntimeValue::Unit
                };
                Ok(Some(val))
            }
            Stmt::Print(expr) => {
                let val = self.eval_expr(expr)?;
                let output = format!("{}", val);
                self.execution_log.push(output.clone());
                println!("{}", output);
                Ok(None)
            }
            Stmt::Assert { condition, message } => {
                let cond_val = self.eval_expr(condition)?;
                if !cond_val.is_truthy() {
                    let msg = message.as_deref().unwrap_or("Assertion failed");
                    return Err(anyhow!("Runtime invariant assertion failure: {}", msg));
                }
                Ok(None)
            }
            Stmt::Expr(expr) => {
                self.eval_expr(expr)?;
                Ok(None)
            }
        }
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> Result<RuntimeValue> {
        match expr {
            Expr::Lit(lit) => Ok(match lit {
                Literal::Int(n) => RuntimeValue::Int(*n),
                Literal::Float(f) => RuntimeValue::Float(*f),
                Literal::Bool(b) => RuntimeValue::Bool(*b),
                Literal::String(s) => RuntimeValue::String(s.clone()),
            }),
            Expr::Ident(name) => {
                self.get_var(name).ok_or_else(|| anyhow!("Undefined variable '{}'", name))
            }
            Expr::Binary { left, op, right } => {
                let l_val = self.eval_expr(left)?;
                let r_val = self.eval_expr(right)?;
                self.eval_binary_op(&l_val, *op, &r_val)
            }
            Expr::UnaryNot(operand) => {
                let val = self.eval_expr(operand)?;
                Ok(RuntimeValue::Bool(!val.is_truthy()))
            }
            Expr::Block(stmts, tail) => {
                self.push_scope();
                for stmt in stmts {
                    if let Some(ret) = self.eval_stmt(stmt)? {
                        self.pop_scope();
                        return Ok(ret);
                    }
                }
                let ret = if let Some(t) = tail {
                    self.eval_expr(t)?
                } else {
                    RuntimeValue::Unit
                };
                self.pop_scope();
                Ok(ret)
            }
            Expr::Call { function, args } => {
                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(self.eval_expr(arg)?);
                }
                self.call_function(function, evaluated_args)
            }
            Expr::FieldAccess { target, field } => {
                let target_val = self.eval_expr(target)?;
                match target_val {
                    RuntimeValue::Struct { fields, .. } => {
                        fields.get(field).cloned().ok_or_else(|| {
                            anyhow!("Field '{}' not found on struct instance", field)
                        })
                    }
                    RuntimeValue::Belief(b) => {
                        // Accessing field on belief unwraps belief field
                        if let RuntimeValue::Struct { fields, .. } = *b.value {
                            fields.get(field).cloned().ok_or_else(|| {
                                anyhow!("Field '{}' not found on struct belief", field)
                            })
                        } else {
                            Err(anyhow!("Cannot access field on non-struct belief"))
                        }
                    }
                    other => Err(anyhow!("Cannot access field '{}' on value {}", field, other)),
                }
            }
            Expr::StructInit { name, fields } => {
                let mut field_map = HashMap::new();
                for (f_name, f_expr) in fields {
                    let f_val = self.eval_expr(f_expr)?;
                    field_map.insert(f_name.clone(), f_val);
                }
                Ok(RuntimeValue::Struct {
                    name: name.clone(),
                    fields: field_map,
                })
            }
            Expr::Confidence(target) => {
                let val = self.eval_expr(target)?;
                match val {
                    RuntimeValue::Belief(b) => Ok(RuntimeValue::Float(b.confidence)),
                    _ => Ok(RuntimeValue::Float(1.0)),
                }
            }
            Expr::Justification(target) => {
                let val = self.eval_expr(target)?;
                match val {
                    RuntimeValue::Belief(b) => Ok(RuntimeValue::String(b.justification)),
                    _ => Ok(RuntimeValue::String("Grounded certainty (epistemic ground truth)".to_string())),
                }
            }
            Expr::OracleCall { contract, method, prompt_arg, .. } => {
                let prompt_val = self.eval_expr(prompt_arg)?;
                let prompt_str = match &prompt_val {
                    RuntimeValue::String(s) => s.clone(),
                    other => format!("{}", other),
                };

                let contract_def = self.program.items.iter().find_map(|item| {
                    if let Item::Contract(c) = item {
                        if &c.name == contract {
                            return Some(c.clone());
                        }
                    }
                    None
                }).ok_or_else(|| anyhow!("Contract '{}' not found", contract))?;

                let belief = self.oracle.query(&contract_def, method, &prompt_str)?;
                Ok(RuntimeValue::Belief(belief))
            }
            Expr::Verify { target, guard_name, fallback } => {
                let val = self.eval_expr(target)?;
                let (inner_val, _, _) = val.unwrap_belief();

                let guard_def = self.program.items.iter().find_map(|item| {
                    if let Item::Guard(g) = item {
                        if &g.name == guard_name {
                            return Some(g.clone());
                        }
                    }
                    None
                }).ok_or_else(|| anyhow!("Guard '{}' not found", guard_name))?;

                // Evaluate guard predicate
                self.push_scope();
                self.define_var(&guard_def.param_name, inner_val.clone());
                let guard_result = self.eval_expr(&guard_def.body)?;
                self.pop_scope();

                if guard_result.is_truthy() {
                    // VERIFICATION SUCCEEDED: The belief is epistemically promoted to Certain T
                    Ok(inner_val.clone())
                } else if let Some(fb) = fallback {
                    // Fallback to safe alternative
                    self.eval_expr(fb)
                } else {
                    Err(anyhow!(
                        "Epistemic verification failed: value '{}' did not satisfy guard '{}'",
                        inner_val,
                        guard_name
                    ))
                }
            }
            Expr::Consensus { count, threshold, oracle_call } => {
                let mut votes: HashMap<String, (usize, f64, String, RuntimeValue)> = HashMap::new();
                let mut _total_confidence = 0.0;

                for _ in 0..*count {
                    let val = self.eval_expr(oracle_call)?;
                    if let RuntimeValue::Belief(b) = val {
                        let repr = format!("{}", b.value);
                        _total_confidence += b.confidence;
                        let entry = votes.entry(repr).or_insert((0, 0.0, b.justification.clone(), *b.value));
                        entry.0 += 1;
                        entry.1 += b.confidence;
                    }
                }

                // Find majority vote
                let mut best_vote = None;
                for (repr, (cnt, conf_sum, just, val)) in votes {
                    let agreement_ratio = cnt as f64 / *count as f64;
                    if agreement_ratio >= *threshold {
                        let avg_conf = conf_sum / cnt as f64;
                        best_vote = Some((repr, agreement_ratio, avg_conf, just, val));
                        break;
                    }
                }

                if let Some((_, agreement, avg_conf, just, val)) = best_vote {
                    Ok(RuntimeValue::Belief(BeliefValue {
                        value: Box::new(val),
                        confidence: avg_conf * agreement,
                        justification: format!("Consensus achieved ({:.0}% agreement): {}", agreement * 100.0, just),
                        model_origin: format!("consensus({}/{})", count, count),
                    }))
                } else {
                    Err(anyhow!(
                        "Epistemic consensus failed: No semantic branch achieved the required agreement threshold of {:.2}",
                        threshold
                    ))
                }
            }
            Expr::Fork { target, cases, fallback } => {
                let target_val = self.eval_expr(target)?;
                let (resolved_target, _, _) = target_val.unwrap_belief();
                let target_str = format!("{}", resolved_target);

                // SPECULATIVE SEMANTIC EXECUTION:
                // In a production execution, branches execute in isolated transactional sandboxes.
                // Here, we simulate speculative evaluation: we find the matching case, commit its state changes,
                // and record the rollback of unchosen speculative branches.

                let mut matched_case = None;
                let mut rolled_back = Vec::new();

                for case in cases {
                    let matches_pattern = target_str.contains(&case.pattern_name)
                        || target_str == case.pattern_name;

                    if matches_pattern && matched_case.is_none() {
                        matched_case = Some(case);
                    } else {
                        rolled_back.push(case.pattern_name.clone());
                    }
                }

                self.traces.push(SpeculativeForkTrace {
                    target_value: target_str.clone(),
                    chosen_branch: matched_case.map(|c| c.pattern_name.clone()).unwrap_or_else(|| "fallback".to_string()),
                    rolled_back_branches: rolled_back,
                });

                if let Some(case) = matched_case {
                    self.push_scope();
                    if let Some(binding) = &case.binding {
                        self.define_var(binding, resolved_target.clone());
                    }
                    for stmt in &case.body {
                        if let Some(ret) = self.eval_stmt(stmt)? {
                            self.pop_scope();
                            return Ok(ret);
                        }
                    }
                    self.pop_scope();
                } else if let Some(fb_stmts) = fallback {
                    self.push_scope();
                    for stmt in fb_stmts {
                        if let Some(ret) = self.eval_stmt(stmt)? {
                            self.pop_scope();
                            return Ok(ret);
                        }
                    }
                    self.pop_scope();
                }

                Ok(RuntimeValue::Unit)
            }
        }
    }

    fn eval_binary_op(&self, left: &RuntimeValue, op: BinaryOp, right: &RuntimeValue) -> Result<RuntimeValue> {
        let (l, _, _) = left.unwrap_belief();
        let (r, _, _) = right.unwrap_belief();

        match (l, r) {
            (RuntimeValue::Int(a), RuntimeValue::Int(b)) => match op {
                BinaryOp::Add => Ok(RuntimeValue::Int(a + b)),
                BinaryOp::Sub => Ok(RuntimeValue::Int(a - b)),
                BinaryOp::Mul => Ok(RuntimeValue::Int(a * b)),
                BinaryOp::Div => {
                    if *b == 0 { return Err(anyhow!("Division by zero")); }
                    Ok(RuntimeValue::Int(a / b))
                }
                BinaryOp::Mod => Ok(RuntimeValue::Int(a % b)),
                BinaryOp::Eq => Ok(RuntimeValue::Bool(a == b)),
                BinaryOp::Neq => Ok(RuntimeValue::Bool(a != b)),
                BinaryOp::Lt => Ok(RuntimeValue::Bool(a < b)),
                BinaryOp::Lte => Ok(RuntimeValue::Bool(a <= b)),
                BinaryOp::Gt => Ok(RuntimeValue::Bool(a > b)),
                BinaryOp::Gte => Ok(RuntimeValue::Bool(a >= b)),
                _ => Err(anyhow!("Invalid operator {:?} for integers", op)),
            },
            (RuntimeValue::Float(a), RuntimeValue::Float(b)) => match op {
                BinaryOp::Add => Ok(RuntimeValue::Float(a + b)),
                BinaryOp::Sub => Ok(RuntimeValue::Float(a - b)),
                BinaryOp::Mul => Ok(RuntimeValue::Float(a * b)),
                BinaryOp::Div => Ok(RuntimeValue::Float(a / b)),
                BinaryOp::Eq => Ok(RuntimeValue::Bool((a - b).abs() < f64::EPSILON)),
                BinaryOp::Neq => Ok(RuntimeValue::Bool((a - b).abs() >= f64::EPSILON)),
                BinaryOp::Lt => Ok(RuntimeValue::Bool(a < b)),
                BinaryOp::Lte => Ok(RuntimeValue::Bool(a <= b)),
                BinaryOp::Gt => Ok(RuntimeValue::Bool(a > b)),
                BinaryOp::Gte => Ok(RuntimeValue::Bool(a >= b)),
                _ => Err(anyhow!("Invalid operator {:?} for floats", op)),
            },
            (RuntimeValue::String(a), RuntimeValue::String(b)) => match op {
                BinaryOp::Add => Ok(RuntimeValue::String(format!("{}{}", a, b))),
                BinaryOp::Eq => Ok(RuntimeValue::Bool(a == b)),
                BinaryOp::Neq => Ok(RuntimeValue::Bool(a != b)),
                _ => Err(anyhow!("Invalid operator {:?} for strings", op)),
            },
            (RuntimeValue::Bool(a), RuntimeValue::Bool(b)) => match op {
                BinaryOp::And => Ok(RuntimeValue::Bool(*a && *b)),
                BinaryOp::Or => Ok(RuntimeValue::Bool(*a || *b)),
                BinaryOp::Eq => Ok(RuntimeValue::Bool(a == b)),
                BinaryOp::Neq => Ok(RuntimeValue::Bool(a != b)),
                _ => Err(anyhow!("Invalid operator {:?} for booleans", op)),
            },
            _ => Err(anyhow!("Type mismatch in binary operation: cannot apply {:?} between {} and {}", op, l, r)),
        }
    }
}
