use crate::ast::*;
use crate::check::CheckedProgram;
use crate::diagnostics::Diagnostic;
use crate::span::Span;
use crate::value::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Host {
    pub inputs: BTreeMap<String, Value>,
    pub auto_approve: bool,
    pub files: BTreeMap<String, String>,
    pub ask_replies: Vec<Value>,
}

impl Default for Host {
    fn default() -> Self {
        Self {
            inputs: BTreeMap::new(),
            auto_approve: true,
            files: BTreeMap::new(),
            ask_replies: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RunResult {
    pub value: Value,
    pub sends: Vec<Value>,
    pub writes: Vec<(String, String)>,
    pub approvals: Vec<String>,
    pub asks: usize,
}

pub fn run(checked: &CheckedProgram, mut host: Host) -> Result<RunResult, Diagnostic> {
    let mut env = BTreeMap::new();
    for input in &checked.program.inputs {
        if let Some(value) = host.inputs.get(&input.name.name) {
            env.insert(input.name.name.clone(), value.clone());
        } else {
            env.insert(input.name.name.clone(), default_for_type(&input.ty));
        }
    }

    let mut result = RunResult {
        value: Value::Unit,
        sends: Vec::new(),
        writes: Vec::new(),
        approvals: Vec::new(),
        asks: 0,
    };

    eval_stmts(
        &checked.program.body,
        &mut env,
        &mut host,
        &mut result,
        None,
    )?;
    Ok(result)
}

fn eval_stmts(
    stmts: &[Stmt],
    env: &mut BTreeMap<String, Value>,
    host: &mut Host,
    result: &mut RunResult,
    mut return_slot: Option<&mut Value>,
) -> Result<Value, Diagnostic> {
    let mut last = Value::Unit;
    for stmt in stmts {
        match stmt {
            Stmt::Let { name, value, .. } => {
                let v = eval_expr(value, env, host, result)?;
                env.insert(name.name.clone(), v);
                last = Value::Unit;
            }
            Stmt::Return { value, .. } => {
                let v = eval_expr(value, env, host, result)?;
                if let Some(slot) = return_slot.as_mut() {
                    **slot = v.clone();
                }
                result.value = v.clone();
                return Ok(v);
            }
            Stmt::Approve { message, span } => {
                let v = eval_expr(message, env, host, result)?;
                let text = v.to_string();
                if !host.auto_approve {
                    return Err(Diagnostic::error(
                        format!("Would wait for human approval: {text}"),
                        *span,
                    )
                    .note("Re-run with --auto-approve to mock the human gate"));
                }
                result.approvals.push(text);
                last = Value::Unit;
            }
            Stmt::Send { value, .. } => {
                let v = eval_expr(value, env, host, result)?;
                result.sends.push(v.clone());
                last = v;
            }
            Stmt::Expr { value, .. } => {
                last = eval_expr(value, env, host, result)?;
            }
        }
    }
    result.value = last.clone();
    Ok(last)
}

fn eval_expr(
    expr: &Expr,
    env: &mut BTreeMap<String, Value>,
    host: &mut Host,
    result: &mut RunResult,
) -> Result<Value, Diagnostic> {
    match &expr.kind {
        ExprKind::Int(v) => Ok(Value::Int(*v)),
        ExprKind::Text(v) => Ok(Value::Text(v.clone())),
        ExprKind::Bool(v) => Ok(Value::Bool(*v)),
        ExprKind::Ident(name) => {
            if let Some(value) = env.get(name) {
                Ok(value.clone())
            } else {
                // Bare names that survived `trell check` are enum variants.
                Ok(Value::Enum(name.clone()))
            }
        }
        ExprKind::Field { base, field } => {
            let value = eval_expr(base, env, host, result)?;
            match value {
                Value::Record(fields) => fields.get(&field.name).cloned().ok_or_else(|| {
                    Diagnostic::error(format!("No field `{}`", field.name), field.span)
                }),
                other => Err(Diagnostic::error(
                    format!("Cannot access field on {}", other.type_name()),
                    field.span,
                )),
            }
        }
        ExprKind::Unary { op, expr: inner } => {
            let value = eval_expr(inner, env, host, result)?;
            match op {
                UnOp::Neg => match value {
                    Value::Int(v) => Ok(Value::Int(v.wrapping_neg())),
                    _ => Err(type_err("int", &value, inner.span)),
                },
                UnOp::Not => Ok(Value::Bool(!value.truthy())),
            }
        }
        ExprKind::Binary { op, left, right } => {
            if matches!(op, BinOp::And) {
                let l = eval_expr(left, env, host, result)?;
                if !l.truthy() {
                    return Ok(Value::Bool(false));
                }
                let r = eval_expr(right, env, host, result)?;
                return Ok(Value::Bool(r.truthy()));
            }
            if matches!(op, BinOp::Or) {
                let l = eval_expr(left, env, host, result)?;
                if l.truthy() {
                    return Ok(Value::Bool(true));
                }
                let r = eval_expr(right, env, host, result)?;
                return Ok(Value::Bool(r.truthy()));
            }
            let l = eval_expr(left, env, host, result)?;
            let r = eval_expr(right, env, host, result)?;
            eval_binary(*op, l, r, expr.span)
        }
        ExprKind::If {
            cond,
            then_block,
            else_block,
        } => {
            let cond_value = eval_expr(cond, env, host, result)?;
            if cond_value.truthy() {
                eval_block(then_block, env, host, result)
            } else if let Some(else_block) = else_block {
                eval_block(else_block, env, host, result)
            } else {
                Ok(Value::Unit)
            }
        }
        ExprKind::Ask { schema, .. } => {
            result.asks += 1;
            if let Some(reply) = host.ask_replies.first().cloned() {
                host.ask_replies.remove(0);
                Ok(reply)
            } else {
                Ok(mock_schema(schema))
            }
        }
        ExprKind::Read { path } => {
            let path_value = eval_expr(path, env, host, result)?;
            let path = path_value
                .as_text()
                .ok_or_else(|| type_err("text", &path_value, path.span))?
                .to_string();
            if let Some(content) = host.files.get(&path) {
                Ok(Value::Text(content.clone()))
            } else {
                Ok(Value::Text(format!("<mock file: {path}>")))
            }
        }
        ExprKind::Write { path, body } => {
            let path_value = eval_expr(path, env, host, result)?;
            let body_value = eval_expr(body, env, host, result)?;
            let path = path_value
                .as_text()
                .ok_or_else(|| type_err("text", &path_value, path.span))?
                .to_string();
            result.writes.push((path.clone(), body_value.to_string()));
            host.files.insert(path, body_value.to_string());
            Ok(Value::Unit)
        }
        ExprKind::Spawn { .. } => Err(Diagnostic::error(
            "`spawn` is not executed in the mock runner yet (checked, not launched)",
            expr.span,
        )
        .note("The checker already enforced spawn ceilings and taint. A Wasmtime host is the next runtime.")),
        ExprKind::Record { fields } => {
            let mut record = BTreeMap::new();
            for (name, value) in fields {
                record.insert(name.name.clone(), eval_expr(value, env, host, result)?);
            }
            Ok(Value::Record(record))
        }
    }
}

fn eval_block(
    block: &Block,
    env: &BTreeMap<String, Value>,
    host: &mut Host,
    result: &mut RunResult,
) -> Result<Value, Diagnostic> {
    let mut local = env.clone();
    eval_stmts(&block.stmts, &mut local, host, result, None)
}

fn eval_binary(op: BinOp, left: Value, right: Value, span: Span) -> Result<Value, Diagnostic> {
    match op {
        BinOp::Add => match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a.wrapping_add(b))),
            (Value::Text(a), Value::Text(b)) => Ok(Value::Text(format!("{a}{b}"))),
            (l, r) => Err(Diagnostic::error(
                format!("Cannot add {} and {}", l.type_name(), r.type_name()),
                span,
            )),
        },
        BinOp::Sub => ints(left, right, span, |a, b| a.wrapping_sub(b)),
        BinOp::Mul => ints(left, right, span, |a, b| a.wrapping_mul(b)),
        BinOp::Div => match (left, right) {
            (Value::Int(_), Value::Int(0)) => Err(Diagnostic::error("Division by zero", span)),
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a / b)),
            (l, r) => Err(Diagnostic::error(
                format!("Cannot divide {} and {}", l.type_name(), r.type_name()),
                span,
            )),
        },
        BinOp::Eq => Ok(Value::Bool(eq(&left, &right))),
        BinOp::Ne => Ok(Value::Bool(!eq(&left, &right))),
        BinOp::Lt => cmp(left, right, span, |a, b| a < b),
        BinOp::Le => cmp(left, right, span, |a, b| a <= b),
        BinOp::Gt => cmp(left, right, span, |a, b| a > b),
        BinOp::Ge => cmp(left, right, span, |a, b| a >= b),
        BinOp::And | BinOp::Or => unreachable!("short-circuit ops handled above"),
    }
}

fn ints(
    left: Value,
    right: Value,
    span: Span,
    f: impl FnOnce(i64, i64) -> i64,
) -> Result<Value, Diagnostic> {
    match (left, right) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(f(a, b))),
        (l, r) => Err(Diagnostic::error(
            format!(
                "Expected int operands, found {} and {}",
                l.type_name(),
                r.type_name()
            ),
            span,
        )),
    }
}

fn cmp(
    left: Value,
    right: Value,
    span: Span,
    f: impl FnOnce(i64, i64) -> bool,
) -> Result<Value, Diagnostic> {
    match (left, right) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(f(a, b))),
        (l, r) => Err(Diagnostic::error(
            format!("Cannot compare {} and {}", l.type_name(), r.type_name()),
            span,
        )),
    }
}

fn eq(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Int(a), Value::Int(b)) => a == b,
        (Value::Text(a), Value::Text(b)) => a == b,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::Enum(a), Value::Enum(b)) => a == b,
        (Value::Enum(a), Value::Text(b)) | (Value::Text(b), Value::Enum(a)) => a == b,
        (Value::Unit, Value::Unit) => true,
        (Value::Record(a), Value::Record(b)) => a == b,
        _ => false,
    }
}

fn type_err(expected: &str, got: &Value, span: Span) -> Diagnostic {
    Diagnostic::error(
        format!("Expected {expected}, found {}", got.type_name()),
        span,
    )
}

fn default_for_type(ty: &Type) -> Value {
    match ty {
        Type::Int => Value::Int(0),
        Type::Text => Value::Text(String::new()),
        Type::Bool => Value::Bool(false),
        Type::Unit => Value::Unit,
        Type::Enum { variants } => Value::Enum(
            variants
                .first()
                .map(|v| v.name.clone())
                .unwrap_or_else(|| "unknown".into()),
        ),
        Type::Record(schema) => {
            let mut fields = BTreeMap::new();
            for (name, field_ty) in &schema.fields {
                fields.insert(name.name.clone(), default_for_type(field_ty));
            }
            Value::Record(fields)
        }
    }
}

fn mock_schema(schema: &Schema) -> Value {
    default_for_type(&Type::Record(schema.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check::check;
    use crate::parser::parse;

    fn eval_src(src: &str) -> Value {
        let checked = check(parse(src).unwrap()).unwrap();
        run(&checked, Host::default()).unwrap().value
    }

    #[test]
    fn arithmetic() {
        assert_eq!(eval_src("20 + 22 * 2"), Value::Int(64));
        assert_eq!(eval_src("(20 + 22) * 2"), Value::Int(84));
        assert_eq!(eval_src("100 - 10 * 3"), Value::Int(70));
        assert_eq!(eval_src("100 / 4 + 3"), Value::Int(28));
        assert_eq!(eval_src("42"), Value::Int(42));
    }

    #[test]
    fn if_and_let() {
        assert_eq!(
            eval_src("let x = 3\nif x > 2 { 10 } else { 1 }"),
            Value::Int(10)
        );
    }
}
