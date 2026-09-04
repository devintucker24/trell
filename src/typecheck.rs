use std::collections::HashMap;
use anyhow::{anyhow, Result};

use crate::ast::*;

pub struct TypeChecker {
    contracts: HashMap<String, ModelContract>,
    structs: HashMap<String, StructDef>,
    guards: HashMap<String, GuardDef>,
    functions: HashMap<String, (Vec<Param>, Type)>,
    scopes: Vec<HashMap<String, Type>>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            contracts: HashMap::new(),
            structs: HashMap::new(),
            guards: HashMap::new(),
            functions: HashMap::new(),
            scopes: vec![HashMap::new()],
        }
    }

    pub fn check_program(&mut self, program: &Program) -> Result<()> {
        // First pass: collect declarations
        for item in &program.items {
            match item {
                Item::Contract(c) => {
                    self.contracts.insert(c.name.clone(), c.clone());
                }
                Item::Struct(s) => {
                    self.structs.insert(s.name.clone(), s.clone());
                }
                Item::Guard(g) => {
                    self.guards.insert(g.name.clone(), g.clone());
                }
                Item::Function(f) => {
                    self.functions.insert(f.name.clone(), (f.params.clone(), f.return_type.clone()));
                }
            }
        }

        // Second pass: check guards
        for item in &program.items {
            if let Item::Guard(g) = item {
                self.push_scope();
                self.define_var(&g.param_name, g.param_type.clone());
                let body_ty = self.check_expr(&g.body)?;
                self.pop_scope();
                if body_ty != Type::Certain(PrimitiveType::Bool) {
                    return Err(anyhow!(
                        "Guard '{}' must return a certain bool, found {:?}",
                        g.name,
                        body_ty
                    ));
                }
            }
        }

        // Third pass: check functions
        for item in &program.items {
            if let Item::Function(f) = item {
                self.push_scope();
                for param in &f.params {
                    self.define_var(&param.name, param.ty.clone());
                }
                let num_stmts = f.body.len();
                for (idx, stmt) in f.body.iter().enumerate() {
                    let is_last = idx == num_stmts - 1;
                    if is_last {
                        if let Stmt::Expr(expr) = stmt {
                            let expr_ty = self.check_expr(expr)?;
                            if !self.is_assignable(&f.return_type, &expr_ty) {
                                return Err(anyhow!(
                                    "Function '{}' trailing expression type {:?} does not match return type {:?}",
                                    f.name,
                                    expr_ty,
                                    f.return_type
                                ));
                            }
                            continue;
                        }
                    }
                    self.check_stmt(stmt, &f.return_type)?;
                }
                self.pop_scope();
            }
        }

        Ok(())
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn define_var(&mut self, name: &str, ty: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), ty);
        }
    }

    fn lookup_var(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }

    fn check_stmt(&mut self, stmt: &Stmt, expected_return: &Type) -> Result<()> {
        match stmt {
            Stmt::Let { name, ty, value } => {
                let inferred_ty = self.check_expr(value)?;
                if let Some(declared_ty) = ty {
                    if !self.is_assignable(declared_ty, &inferred_ty) {
                        return Err(anyhow!(
                            "Type mismatch in 'let {}': declared as {:?}, but assigned value has type {:?}",
                            name,
                            declared_ty,
                            inferred_ty
                        ));
                    }
                    self.define_var(name, declared_ty.clone());
                } else {
                    self.define_var(name, inferred_ty);
                }
            }
            Stmt::Assign { target, value } => {
                let target_ty = self.lookup_var(target).ok_or_else(|| {
                    anyhow!("Undefined variable '{}' in assignment", target)
                })?;
                let val_ty = self.check_expr(value)?;
                if !self.is_assignable(&target_ty, &val_ty) {
                    return Err(anyhow!(
                        "Cannot assign type {:?} to variable '{}' of type {:?}",
                        val_ty,
                        target,
                        target_ty
                    ));
                }
            }
            Stmt::Print(expr) => {
                self.check_expr(expr)?;
            }
            Stmt::Assert { condition, .. } => {
                let cond_ty = self.check_expr(condition)?;
                if cond_ty != Type::Certain(PrimitiveType::Bool) {
                    return Err(anyhow!("Assert condition must be 'certain bool', found {:?}", cond_ty));
                }
            }
            Stmt::Return(opt_expr) => {
                let ret_ty = if let Some(expr) = opt_expr {
                    self.check_expr(expr)?
                } else {
                    Type::Unit
                };
                if !self.is_assignable(expected_return, &ret_ty) {
                    return Err(anyhow!(
                        "Function return type mismatch: expected {:?}, returning {:?}",
                        expected_return,
                        ret_ty
                    ));
                }
            }
            Stmt::Expr(expr) => {
                self.check_expr(expr)?;
            }
        }
        Ok(())
    }

    pub fn check_expr(&mut self, expr: &Expr) -> Result<Type> {
        match expr {
            Expr::Lit(lit) => Ok(match lit {
                Literal::Int(_) => Type::Certain(PrimitiveType::Int),
                Literal::Float(_) => Type::Certain(PrimitiveType::Float),
                Literal::Bool(_) => Type::Certain(PrimitiveType::Bool),
                Literal::String(_) => Type::Certain(PrimitiveType::String),
            }),
            Expr::Ident(name) => {
                self.lookup_var(name).ok_or_else(|| anyhow!("Undefined identifier '{}'", name))
            }
            Expr::Binary { left, op, right } => {
                let left_ty = self.check_expr(left)?;
                let right_ty = self.check_expr(right)?;

                match op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                        // Arithmetic operations require certain numeric types
                        if left_ty == Type::Certain(PrimitiveType::Int) && right_ty == Type::Certain(PrimitiveType::Int) {
                            Ok(Type::Certain(PrimitiveType::Int))
                        } else if (left_ty == Type::Certain(PrimitiveType::Float) || left_ty == Type::Certain(PrimitiveType::Int))
                            && (right_ty == Type::Certain(PrimitiveType::Float) || right_ty == Type::Certain(PrimitiveType::Int))
                        {
                            Ok(Type::Certain(PrimitiveType::Float))
                        } else if *op == BinaryOp::Add && left_ty == Type::Certain(PrimitiveType::String) && right_ty == Type::Certain(PrimitiveType::String) {
                            Ok(Type::Certain(PrimitiveType::String))
                        } else {
                            Err(anyhow!(
                                "Arithmetic operator {:?} not supported between {:?} and {:?}",
                                op,
                                left_ty,
                                right_ty
                            ))
                        }
                    }
                    BinaryOp::Eq | BinaryOp::Neq => {
                        if left_ty == right_ty {
                            Ok(Type::Certain(PrimitiveType::Bool))
                        } else {
                            Err(anyhow!(
                                "Cannot compare equality between different types {:?} and {:?}",
                                left_ty,
                                right_ty
                            ))
                        }
                    }
                    BinaryOp::Lt | BinaryOp::Lte | BinaryOp::Gt | BinaryOp::Gte => {
                        // Comparison on numbers
                        Ok(Type::Certain(PrimitiveType::Bool))
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        if left_ty == Type::Certain(PrimitiveType::Bool) && right_ty == Type::Certain(PrimitiveType::Bool) {
                            Ok(Type::Certain(PrimitiveType::Bool))
                        } else {
                            Err(anyhow!("Logical operators require 'certain bool' operands"))
                        }
                    }
                }
            }
            Expr::UnaryNot(operand) => {
                let ty = self.check_expr(operand)?;
                if ty == Type::Certain(PrimitiveType::Bool) {
                    Ok(Type::Certain(PrimitiveType::Bool))
                } else {
                    Err(anyhow!("Unary '!' requires 'certain bool', found {:?}", ty))
                }
            }
            Expr::Block(stmts, tail) => {
                self.push_scope();
                for stmt in stmts {
                    self.check_stmt(stmt, &Type::Unit)?;
                }
                let ret = if let Some(t) = tail {
                    self.check_expr(t)?
                } else {
                    Type::Unit
                };
                self.pop_scope();
                Ok(ret)
            }
            Expr::Call { function, args } => {
                let (params, return_type) = self.functions.get(function).cloned().ok_or_else(|| {
                    anyhow!("Undefined function '{}'", function)
                })?;
                if params.len() != args.len() {
                    return Err(anyhow!(
                        "Function '{}' expects {} arguments, received {}",
                        function,
                        params.len(),
                        args.len()
                    ));
                }
                for (param, arg) in params.iter().zip(args.iter()) {
                    let arg_ty = self.check_expr(arg)?;
                    if !self.is_assignable(&param.ty, &arg_ty) {
                        return Err(anyhow!(
                            "Argument type mismatch for function '{}' parameter '{}': expected {:?}, received {:?}",
                            function,
                            param.name,
                            param.ty,
                            arg_ty
                        ));
                    }
                }
                Ok(return_type)
            }
            Expr::FieldAccess { target, field } => {
                let target_ty = self.check_expr(target)?;
                match target_ty {
                    Type::CertainCustom(name) | Type::BeliefCustom(name) => {
                        let s = self.structs.get(&name).ok_or_else(|| {
                            anyhow!("Undefined struct type '{}'", name)
                        })?;
                        for f in &s.fields {
                            if &f.name == field {
                                return Ok(f.ty.clone());
                            }
                        }
                        Err(anyhow!("Struct '{}' has no field '{}'", name, field))
                    }
                    other => Err(anyhow!("Field access not supported on type {:?}", other)),
                }
            }
            Expr::StructInit { name, fields } => {
                let s = self.structs.get(name).cloned().ok_or_else(|| {
                    anyhow!("Undefined struct '{}' in initializer", name)
                })?;
                for (f_name, f_expr) in fields {
                    let expected_f = s.fields.iter().find(|f| &f.name == f_name).ok_or_else(|| {
                        anyhow!("Unknown field '{}' in struct '{}'", f_name, name)
                    })?;
                    let val_ty = self.check_expr(f_expr)?;
                    if !self.is_assignable(&expected_f.ty, &val_ty) {
                        return Err(anyhow!(
                            "Field '{}' of struct '{}' expects {:?}, received {:?}",
                            f_name,
                            name,
                            expected_f.ty,
                            val_ty
                        ));
                    }
                }
                Ok(Type::CertainCustom(name.clone()))
            }
            Expr::OracleCall { contract, target_type, prompt_arg, .. } => {
                if !self.contracts.contains_key(contract) {
                    return Err(anyhow!("Undefined model contract '{}' in oracle call", contract));
                }
                self.check_expr(prompt_arg)?;
                // The oracle call ALWAYS returns a belief!
                match target_type {
                    Type::Certain(p) => Ok(Type::Belief(p.clone())),
                    Type::CertainCustom(c) => Ok(Type::BeliefCustom(c.clone())),
                    Type::Belief(_) | Type::BeliefCustom(_) => Ok(target_type.clone()),
                    Type::Unit => Ok(Type::Unit),
                }
            }
            Expr::Verify { target, guard_name, fallback } => {
                let target_ty = self.check_expr(target)?;
                let guard = self.guards.get(guard_name).cloned().ok_or_else(|| {
                    anyhow!("Undefined guard '{}' in verify expression", guard_name)
                })?;

                // Target inner type must match guard param type
                let inner_ty = match &target_ty {
                    Type::Belief(p) => Type::Certain(p.clone()),
                    Type::BeliefCustom(c) => Type::CertainCustom(c.clone()),
                    Type::Certain(_) | Type::CertainCustom(_) => target_ty.clone(),
                    Type::Unit => Type::Unit,
                };

                if !self.is_assignable(&guard.param_type, &inner_ty) {
                    return Err(anyhow!(
                        "Guard '{}' expects param type {:?}, but target reduces to {:?}",
                        guard_name,
                        guard.param_type,
                        inner_ty
                    ));
                }

                if let Some(fb) = fallback {
                    let fb_ty = self.check_expr(fb)?;
                    if !self.is_assignable(&inner_ty, &fb_ty) {
                        return Err(anyhow!(
                            "Verify fallback type {:?} does not match verified return type {:?}",
                            fb_ty,
                            inner_ty
                        ));
                    }
                }

                // Crucial epistemic transition: verify converts a belief<T> into a certain T!
                Ok(inner_ty)
            }
            Expr::Consensus { oracle_call, .. } => {
                let call_ty = self.check_expr(oracle_call)?;
                // Consensus yields a belief with enhanced empirical confidence
                Ok(call_ty)
            }
            Expr::Fork { target, cases, fallback } => {
                let target_ty = self.check_expr(target)?;
                self.push_scope();

                for case in cases {
                    if let Some(binding) = &case.binding {
                        self.define_var(binding, target_ty.clone());
                    }
                    for stmt in &case.body {
                        self.check_stmt(stmt, &Type::Unit)?;
                    }
                }

                if let Some(fb_stmts) = fallback {
                    for stmt in fb_stmts {
                        self.check_stmt(stmt, &Type::Unit)?;
                    }
                }

                self.pop_scope();
                Ok(Type::Unit)
            }
            Expr::Confidence(target) => {
                let ty = self.check_expr(target)?;
                match ty {
                    Type::Belief(_) | Type::BeliefCustom(_) => Ok(Type::Certain(PrimitiveType::Float)),
                    other => Err(anyhow!("'confidence(...)' only applies to belief types, found {:?}", other)),
                }
            }
            Expr::Justification(target) => {
                let ty = self.check_expr(target)?;
                match ty {
                    Type::Belief(_) | Type::BeliefCustom(_) => Ok(Type::Certain(PrimitiveType::String)),
                    other => Err(anyhow!("'justification(...)' only applies to belief types, found {:?}", other)),
                }
            }
        }
    }

    /// Check if `actual` can be assigned where `expected` is required.
    /// CRITICAL EPISTEMIC RULE:
    /// belief<T> can NEVER be assigned to certain T without explicit verify!
    /// certain T can be assigned to belief<T> (a certain fact is a belief with 1.0 confidence).
    pub fn is_assignable(&self, expected: &Type, actual: &Type) -> bool {
        if expected == actual {
            return true;
        }

        match (expected, actual) {
            // Epistemic promotion: Certain T can satisfy Belief<T>
            (Type::Belief(p1), Type::Certain(p2)) if p1 == p2 => true,
            (Type::BeliefCustom(c1), Type::CertainCustom(c2)) if c1 == c2 => true,
            // Float <- Int coercion
            (Type::Certain(PrimitiveType::Float), Type::Certain(PrimitiveType::Int)) => true,
            (Type::Belief(PrimitiveType::Float), Type::Belief(PrimitiveType::Int)) => true,
            // EPISTEMIC SOUNDNESS: Belief -> Certain is strictly FORBIDDEN!
            (Type::Certain(_), Type::Belief(_)) => false,
            (Type::CertainCustom(_), Type::BeliefCustom(_)) => false,
            _ => false,
        }
    }
}
