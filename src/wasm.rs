use crate::ast::*;
use crate::diagnostics::Diagnostic;
use crate::span::Span;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct WasmProgram {
    pub params: Vec<String>,
    pub locals: Vec<String>,
    pub ops: Vec<Op>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    I64Const(i64),
    LocalGet(u32),
    LocalSet(u32),
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Eqz,
    And,
    Or,
    If,
    Else,
    End,
    Drop,
}

pub fn compile(program: &Program) -> Result<WasmProgram, Diagnostic> {
    if !program.is_pure_compute() {
        return Err(Diagnostic::error(
            "Wasm compile is for pure integer compute (no cap, ask, text, or tools)",
            program.span,
        )
        .note("Use `trell run --mock` for agent workflows. Wasm is the sandbox for untrusted arithmetic."));
    }

    let mut params = Vec::new();
    let mut locals = Vec::new();
    for input in &program.inputs {
        params.push(input.name.name.clone());
    }

    collect_lets(&program.body, &mut locals);

    let mut names: Vec<String> = params
        .iter()
        .cloned()
        .chain(locals.iter().cloned())
        .collect();
    let mut compiler = Compiler {
        names: names
            .drain(..)
            .enumerate()
            .map(|(i, n)| (n, i as u32))
            .collect(),
        ops: Vec::new(),
    };

    compiler.body(&program.body)?;
    if compiler.ops.is_empty() {
        compiler.ops.push(Op::I64Const(0));
    }

    Ok(WasmProgram {
        params,
        locals,
        ops: compiler.ops,
    })
}

fn collect_lets(stmts: &[Stmt], locals: &mut Vec<String>) {
    for stmt in stmts {
        match stmt {
            Stmt::Let { name, .. } => {
                if !locals.contains(&name.name) {
                    locals.push(name.name.clone());
                }
            }
            Stmt::Expr { value, .. } | Stmt::Return { value, .. } => {
                collect_lets_expr(value, locals);
            }
            Stmt::Approve { .. } | Stmt::Send { .. } => {}
        }
    }
}

fn collect_lets_expr(expr: &Expr, locals: &mut Vec<String>) {
    if let ExprKind::If {
        then_block,
        else_block,
        ..
    } = &expr.kind
    {
        collect_lets(&then_block.stmts, locals);
        if let Some(else_block) = else_block {
            collect_lets(&else_block.stmts, locals);
        }
    }
}

struct Compiler {
    names: BTreeMap<String, u32>,
    ops: Vec<Op>,
}

impl Compiler {
    fn body(&mut self, stmts: &[Stmt]) -> Result<(), Diagnostic> {
        let mut produced = false;
        for stmt in stmts {
            match stmt {
                Stmt::Let { name, value, span } => {
                    self.expr(value)?;
                    let idx = self.local(name, *span)?;
                    self.ops.push(Op::LocalSet(idx));
                    produced = false;
                }
                Stmt::Return { value, .. } | Stmt::Expr { value, .. } => {
                    if produced {
                        self.ops.push(Op::Drop);
                    }
                    self.expr(value)?;
                    produced = true;
                }
                Stmt::Approve { span, .. } | Stmt::Send { span, .. } => {
                    return Err(Diagnostic::error(
                        "Effects cannot be compiled to import-free Wasm",
                        *span,
                    ));
                }
            }
        }
        if !produced {
            self.ops.push(Op::I64Const(0));
        }
        Ok(())
    }

    fn expr(&mut self, expr: &Expr) -> Result<(), Diagnostic> {
        match &expr.kind {
            ExprKind::Int(v) => {
                self.ops.push(Op::I64Const(*v));
                Ok(())
            }
            ExprKind::Bool(v) => {
                self.ops.push(Op::I64Const(if *v { 1 } else { 0 }));
                Ok(())
            }
            ExprKind::Ident(name) => {
                let idx = self.local_name(name, expr.span)?;
                self.ops.push(Op::LocalGet(idx));
                Ok(())
            }
            ExprKind::Unary { op, expr: inner } => {
                match op {
                    UnOp::Neg => {
                        self.ops.push(Op::I64Const(0));
                        self.expr(inner)?;
                        self.ops.push(Op::Sub);
                    }
                    UnOp::Not => {
                        self.expr(inner)?;
                        self.ops.push(Op::Eqz);
                        self.ops.push(Op::If);
                        self.ops.push(Op::I64Const(1));
                        self.ops.push(Op::Else);
                        self.ops.push(Op::I64Const(0));
                        self.ops.push(Op::End);
                    }
                }
                Ok(())
            }
            ExprKind::Binary { op, left, right } => {
                self.expr(left)?;
                self.expr(right)?;
                let inst = match op {
                    BinOp::Add => Op::Add,
                    BinOp::Sub => Op::Sub,
                    BinOp::Mul => Op::Mul,
                    BinOp::Div => Op::Div,
                    BinOp::Eq => Op::Eq,
                    BinOp::Ne => Op::Ne,
                    BinOp::Lt => Op::Lt,
                    BinOp::Le => Op::Le,
                    BinOp::Gt => Op::Gt,
                    BinOp::Ge => Op::Ge,
                    BinOp::And => Op::And,
                    BinOp::Or => Op::Or,
                };
                self.ops.push(inst);
                Ok(())
            }
            ExprKind::If {
                cond,
                then_block,
                else_block,
            } => {
                self.expr(cond)?;
                self.ops.push(Op::If);
                self.body(&then_block.stmts)?;
                self.ops.push(Op::Else);
                if let Some(else_block) = else_block {
                    self.body(&else_block.stmts)?;
                } else {
                    self.ops.push(Op::I64Const(0));
                }
                self.ops.push(Op::End);
                Ok(())
            }
            _ => Err(Diagnostic::error(
                "This expression cannot be compiled to integer Wasm",
                expr.span,
            )),
        }
    }

    fn local(&self, ident: &Ident, span: Span) -> Result<u32, Diagnostic> {
        self.local_name(&ident.name, span)
    }

    fn local_name(&self, name: &str, span: Span) -> Result<u32, Diagnostic> {
        self.names.get(name).copied().ok_or_else(|| {
            Diagnostic::error(format!("Unknown name `{name}` in Wasm compile"), span)
        })
    }
}

pub fn interpret(module: &WasmProgram, args: &[i64]) -> Result<i64, String> {
    if args.len() != module.params.len() {
        return Err(format!(
            "expected {} argument(s), got {}",
            module.params.len(),
            args.len()
        ));
    }
    let mut locals = vec![0i64; module.params.len() + module.locals.len()];
    for (i, arg) in args.iter().enumerate() {
        locals[i] = *arg;
    }

    let mut stack: Vec<i64> = Vec::new();
    let mut ip = 0usize;
    let ops = &module.ops;
    while ip < ops.len() {
        match &ops[ip] {
            Op::I64Const(v) => stack.push(*v),
            Op::LocalGet(i) => stack.push(locals[*i as usize]),
            Op::LocalSet(i) => {
                let v = stack.pop().ok_or("stack underflow")?;
                locals[*i as usize] = v;
            }
            Op::Add => bin(&mut stack, |a, b| a.wrapping_add(b))?,
            Op::Sub => bin(&mut stack, |a, b| a.wrapping_sub(b))?,
            Op::Mul => bin(&mut stack, |a, b| a.wrapping_mul(b))?,
            Op::Div => {
                let b = stack.pop().ok_or("stack underflow")?;
                let a = stack.pop().ok_or("stack underflow")?;
                if b == 0 {
                    return Err("division by zero".into());
                }
                stack.push(a / b);
            }
            Op::Eq => cmp(&mut stack, |a, b| a == b)?,
            Op::Ne => cmp(&mut stack, |a, b| a != b)?,
            Op::Lt => cmp(&mut stack, |a, b| a < b)?,
            Op::Le => cmp(&mut stack, |a, b| a <= b)?,
            Op::Gt => cmp(&mut stack, |a, b| a > b)?,
            Op::Ge => cmp(&mut stack, |a, b| a >= b)?,
            Op::Eqz => {
                let a = stack.pop().ok_or("stack underflow")?;
                stack.push(if a == 0 { 1 } else { 0 });
            }
            Op::And => {
                let b = stack.pop().ok_or("stack underflow")?;
                let a = stack.pop().ok_or("stack underflow")?;
                stack.push(if a != 0 && b != 0 { 1 } else { 0 });
            }
            Op::Or => {
                let b = stack.pop().ok_or("stack underflow")?;
                let a = stack.pop().ok_or("stack underflow")?;
                stack.push(if a != 0 || b != 0 { 1 } else { 0 });
            }
            Op::If => {
                let cond = stack.pop().ok_or("stack underflow")?;
                if cond == 0 {
                    ip = skip_to_else_or_end(ops, ip)?;
                }
            }
            Op::Else => {
                ip = skip_to_matching_end(ops, ip)?;
            }
            Op::End => {}
            Op::Drop => {
                stack.pop().ok_or("stack underflow")?;
            }
        }
        ip += 1;
    }
    stack.pop().ok_or_else(|| "no result on stack".into())
}

fn bin(stack: &mut Vec<i64>, f: impl FnOnce(i64, i64) -> i64) -> Result<(), String> {
    let b = stack.pop().ok_or("stack underflow")?;
    let a = stack.pop().ok_or("stack underflow")?;
    stack.push(f(a, b));
    Ok(())
}

fn cmp(stack: &mut Vec<i64>, f: impl FnOnce(i64, i64) -> bool) -> Result<(), String> {
    let b = stack.pop().ok_or("stack underflow")?;
    let a = stack.pop().ok_or("stack underflow")?;
    stack.push(if f(a, b) { 1 } else { 0 });
    Ok(())
}

fn skip_to_else_or_end(ops: &[Op], from: usize) -> Result<usize, String> {
    let mut depth = 0i32;
    let mut i = from + 1;
    while i < ops.len() {
        match ops[i] {
            Op::If => depth += 1,
            Op::Else if depth == 0 => return Ok(i),
            Op::End if depth == 0 => return Ok(i),
            Op::End => depth -= 1,
            _ => {}
        }
        i += 1;
    }
    Err("unmatched if".into())
}

fn skip_to_matching_end(ops: &[Op], from: usize) -> Result<usize, String> {
    let mut depth = 0i32;
    let mut i = from + 1;
    while i < ops.len() {
        match ops[i] {
            Op::If => depth += 1,
            Op::End if depth == 0 => return Ok(i),
            Op::End => depth -= 1,
            _ => {}
        }
        i += 1;
    }
    Err("unmatched else".into())
}

pub fn to_wat(module: &WasmProgram) -> String {
    let mut out = String::from("(module\n  (func (export \"eval\")");
    for _ in &module.params {
        out.push_str(" (param i64)");
    }
    out.push_str(" (result i64)\n");
    for _ in &module.locals {
        out.push_str("    (local i64)\n");
    }
    for op in &module.ops {
        match op {
            Op::I64Const(v) => out.push_str(&format!("    i64.const {v}\n")),
            Op::LocalGet(i) => out.push_str(&format!("    local.get {i}\n")),
            Op::LocalSet(i) => out.push_str(&format!("    local.set {i}\n")),
            Op::Add => out.push_str("    i64.add\n"),
            Op::Sub => out.push_str("    i64.sub\n"),
            Op::Mul => out.push_str("    i64.mul\n"),
            Op::Div => out.push_str("    i64.div_s\n"),
            Op::Eq => out.push_str("    i64.eq\n    i64.extend_i32_u\n"),
            Op::Ne => out.push_str("    i64.ne\n    i64.extend_i32_u\n"),
            Op::Lt => out.push_str("    i64.lt_s\n    i64.extend_i32_u\n"),
            Op::Le => out.push_str("    i64.le_s\n    i64.extend_i32_u\n"),
            Op::Gt => out.push_str("    i64.gt_s\n    i64.extend_i32_u\n"),
            Op::Ge => out.push_str("    i64.ge_s\n    i64.extend_i32_u\n"),
            Op::Eqz => out.push_str("    i64.eqz\n    i64.extend_i32_u\n"),
            Op::And | Op::Or => {
                // These operate on 0/1 i64 truth values.
                if matches!(op, Op::And) {
                    out.push_str("    i64.and\n");
                } else {
                    out.push_str("    i64.or\n");
                }
            }
            Op::If => out.push_str("    i64.eqz\n    i32.eqz\n    if (result i64)\n"),
            Op::Else => out.push_str("    else\n"),
            Op::End => out.push_str("    end\n"),
            Op::Drop => out.push_str("    drop\n"),
        }
    }
    out.push_str("  )\n)\n");
    out
}

pub fn encode_wasm(module: &WasmProgram) -> Vec<u8> {
    let mut w = Vec::new();
    w.extend_from_slice(&[0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]);

    // Type section: (param i64)* -> i64
    let mut types = Vec::new();
    types.push(0x60);
    encode_vec_len(&mut types, module.params.len());
    for _ in &module.params {
        types.push(0x7e); // i64
    }
    types.push(0x01);
    types.push(0x7e);
    section(&mut w, 1, {
        let mut body = vec![0x01];
        body.extend(types);
        body
    });

    // Function section
    section(&mut w, 3, vec![0x01, 0x00]);

    // Export section
    let mut export = Vec::new();
    export.push(0x01); // one export
    encode_name(&mut export, "eval");
    export.push(0x00); // func
    export.push(0x00); // func 0
    section(&mut w, 7, export);

    // Code section
    let mut code = Vec::new();
    let mut locals_decl = Vec::new();
    if module.locals.is_empty() {
        locals_decl.push(0x00);
    } else {
        locals_decl.push(0x01);
        encode_uleb(&mut locals_decl, module.locals.len() as u32);
        locals_decl.push(0x7e);
    }
    let mut body = locals_decl;
    encode_ops(&mut body, &module.ops);
    body.push(0x0b); // end

    encode_vec_len(&mut code, 1);
    encode_vec_len(&mut code, body.len());
    code.extend(body);
    section(&mut w, 10, code);
    w
}

fn encode_ops(out: &mut Vec<u8>, ops: &[Op]) {
    for op in ops {
        match op {
            Op::I64Const(v) => {
                out.push(0x42);
                encode_sleb(out, *v);
            }
            Op::LocalGet(i) => {
                out.push(0x20);
                encode_uleb(out, *i);
            }
            Op::LocalSet(i) => {
                out.push(0x21);
                encode_uleb(out, *i);
            }
            Op::Add => out.push(0x7c),
            Op::Sub => out.push(0x7d),
            Op::Mul => out.push(0x7e),
            Op::Div => out.push(0x7f), // div_s
            Op::Eq => {
                out.push(0x51); // i64.eq -> i32
                out.push(0xad); // i64.extend_i32_u
            }
            Op::Ne => {
                out.push(0x52);
                out.push(0xad);
            }
            Op::Lt => {
                out.push(0x53);
                out.push(0xad);
            }
            Op::Le => {
                out.push(0x55);
                out.push(0xad);
            }
            Op::Gt => {
                out.push(0x57);
                out.push(0xad);
            }
            Op::Ge => {
                out.push(0x59);
                out.push(0xad);
            }
            Op::Eqz => {
                out.push(0x50);
                out.push(0xad);
            }
            Op::And => out.push(0x83),
            Op::Or => out.push(0x84),
            Op::If => {
                // cond is i64; wasm if wants i32
                out.push(0x50); // i64.eqz -> i32
                out.push(0x45); // i32.eqz  (invert: nonzero -> 1)
                out.push(0x04); // if
                out.push(0x7e); // result i64
            }
            Op::Else => out.push(0x05),
            Op::End => out.push(0x0b),
            Op::Drop => out.push(0x1a),
        }
    }
}

fn section(w: &mut Vec<u8>, id: u8, body: Vec<u8>) {
    w.push(id);
    encode_uleb(w, body.len() as u32);
    w.extend(body);
}

fn encode_name(out: &mut Vec<u8>, name: &str) {
    encode_uleb(out, name.len() as u32);
    out.extend(name.as_bytes());
}

fn encode_vec_len(out: &mut Vec<u8>, len: usize) {
    encode_uleb(out, len as u32);
}

fn encode_uleb(out: &mut Vec<u8>, mut value: u32) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
}

fn encode_sleb(out: &mut Vec<u8>, mut value: i64) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        let done = (value == 0 && byte & 0x40 == 0) || (value == -1 && byte & 0x40 != 0);
        if !done {
            byte |= 0x80;
        }
        out.push(byte);
        if done {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn eval_wasm(src: &str) -> i64 {
        let program = parse(src).unwrap();
        let wasm = compile(&program).unwrap();
        interpret(&wasm, &[]).unwrap()
    }

    #[test]
    fn wasm_arithmetic() {
        assert_eq!(eval_wasm("20 + 22 * 2"), 64);
        assert_eq!(eval_wasm("(20 + 22) * 2"), 84);
        assert_eq!(eval_wasm("100 - 10 * 3"), 70);
        assert_eq!(eval_wasm("100 / 4 + 3"), 28);
        assert_eq!(eval_wasm("42"), 42);
    }

    #[test]
    fn wasm_binary_has_magic() {
        let program = parse("1 + 2").unwrap();
        let wasm = compile(&program).unwrap();
        let bytes = encode_wasm(&wasm);
        assert_eq!(&bytes[..4], b"\0asm");
        assert!(bytes.len() > 8);
    }

    #[test]
    fn wasm_if() {
        assert_eq!(eval_wasm("if 1 { 9 } else { 8 }"), 9);
        assert_eq!(eval_wasm("if 0 { 9 } else { 8 }"), 8);
    }
}
