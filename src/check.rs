use crate::ast::*;
use crate::diagnostics::Diagnostic;
use crate::span::Span;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

pub const KNOWN_EFFECTS: &[&str] = &["read", "write", "ask", "send", "spawn", "net", "git"];
pub const KNOWN_BUDGETS: &[&str] = &["tokens", "cents"];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EffectUse {
    pub effect: String,
    pub detail: String,
    pub tainted: bool,
    pub needs_approve: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Caps {
    pub name: Option<String>,
    pub allowed: BTreeMap<String, Vec<String>>,
    pub denied: BTreeSet<String>,
    pub need_approve: BTreeSet<String>,
    pub budget_tokens: i64,
    pub budget_cents: i64,
    pub spawn_limit: i64,
}

impl Default for Caps {
    fn default() -> Self {
        Self {
            name: None,
            allowed: BTreeMap::new(),
            denied: BTreeSet::new(),
            need_approve: BTreeSet::new(),
            budget_tokens: 0,
            budget_cents: 0,
            spawn_limit: 0,
        }
    }
}

impl Caps {
    pub fn is_allowed(&self, effect: &str) -> bool {
        if self.denied.contains(effect) {
            return false;
        }
        self.allowed.contains_key(effect)
    }

    pub fn paths_for(&self, effect: &str) -> &[String] {
        self.allowed
            .get(effect)
            .map(|p| p.as_slice())
            .unwrap_or(&[])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TypeInfo {
    ty: TypeKey,
    tainted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TypeKey {
    Int,
    Text,
    Bool,
    Unit,
    Enum(Vec<String>),
    Record(Vec<(String, Box<TypeKey>)>),
}

impl TypeKey {
    fn from_ast(ty: &Type) -> Self {
        match ty {
            Type::Int => TypeKey::Int,
            Type::Text => TypeKey::Text,
            Type::Bool => TypeKey::Bool,
            Type::Unit => TypeKey::Unit,
            Type::Enum { variants } => {
                TypeKey::Enum(variants.iter().map(|v| v.name.clone()).collect())
            }
            Type::Record(schema) => TypeKey::Record(
                schema
                    .fields
                    .iter()
                    .map(|(n, t)| (n.name.clone(), Box::new(TypeKey::from_ast(t))))
                    .collect(),
            ),
        }
    }

    fn name(&self) -> String {
        match self {
            TypeKey::Int => "int".into(),
            TypeKey::Text => "text".into(),
            TypeKey::Bool => "bool".into(),
            TypeKey::Unit => "unit".into(),
            TypeKey::Enum(variants) => format!("enum({})", variants.join(", ")),
            TypeKey::Record(fields) => {
                let inner: Vec<_> = fields
                    .iter()
                    .map(|(n, t)| format!("{n}: {}", t.name()))
                    .collect();
                format!("{{ {} }}", inner.join(", "))
            }
        }
    }

    fn same(&self, other: &TypeKey) -> bool {
        match (self, other) {
            (TypeKey::Int, TypeKey::Int)
            | (TypeKey::Text, TypeKey::Text)
            | (TypeKey::Bool, TypeKey::Bool)
            | (TypeKey::Unit, TypeKey::Unit) => true,
            (TypeKey::Enum(a), TypeKey::Enum(b)) => a == b,
            (TypeKey::Record(a), TypeKey::Record(b)) => {
                a.len() == b.len()
                    && a.iter()
                        .zip(b.iter())
                        .all(|((n1, t1), (n2, t2))| n1 == n2 && t1.same(t2))
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CheckedProgram {
    pub program: Program,
    pub caps: Caps,
    pub effects: Vec<EffectUse>,
    pub warnings: Vec<Diagnostic>,
}

pub fn check(program: Program) -> Result<CheckedProgram, Vec<Diagnostic>> {
    let mut checker = Checker {
        caps: Caps::default(),
        bindings: BTreeMap::new(),
        errors: Vec::new(),
        warnings: Vec::new(),
        effects: Vec::new(),
        enum_variants: BTreeMap::new(),
    };

    if let Some(cap) = &program.cap {
        checker.collect_caps(cap);
    }

    for input in &program.inputs {
        let ty = TypeKey::from_ast(&input.ty);
        checker.register_enum_variants(&ty);
        checker
            .bindings
            .insert(input.name.name.clone(), TypeInfo { ty, tainted: false });
    }

    let mut approved = false;
    for stmt in &program.body {
        checker.check_stmt(stmt, &mut approved);
    }

    if checker.errors.is_empty() {
        Ok(CheckedProgram {
            program,
            caps: checker.caps,
            effects: checker.effects,
            warnings: checker.warnings,
        })
    } else {
        let mut all = checker.errors;
        all.extend(checker.warnings);
        Err(all)
    }
}

struct Checker {
    caps: Caps,
    bindings: BTreeMap<String, TypeInfo>,
    errors: Vec<Diagnostic>,
    warnings: Vec<Diagnostic>,
    effects: Vec<EffectUse>,
    enum_variants: BTreeMap<String, TypeKey>,
}

impl Checker {
    fn collect_caps(&mut self, cap: &CapBlock) {
        self.caps.name = cap.name.as_ref().map(|n| n.name.clone());
        for item in &cap.items {
            match item {
                CapItem::Allow { name, paths, span } => {
                    self.check_effect_name(&name.name, *span);
                    if self.caps.denied.contains(&name.name) {
                        self.errors.push(
                            Diagnostic::error(
                                format!(
                                    "Cannot allow `{}` because it was already denied",
                                    name.name
                                ),
                                *span,
                            )
                            .note(
                                "Trell fails closed: deny wins, and order does not reopen a door",
                            ),
                        );
                    }
                    let path_values = paths.iter().map(|p| p.value.clone()).collect();
                    self.caps.allowed.insert(name.name.clone(), path_values);
                }
                CapItem::Deny { name, span } => {
                    self.check_effect_name(&name.name, *span);
                    self.caps.denied.insert(name.name.clone());
                    self.caps.allowed.remove(&name.name);
                }
                CapItem::NeedApprove { effect, span } => {
                    self.check_effect_name(&effect.name, *span);
                    self.caps.need_approve.insert(effect.name.clone());
                }
                CapItem::Budget { name, amount, span } => {
                    if !KNOWN_BUDGETS.contains(&name.name.as_str()) {
                        self.errors.push(
                            Diagnostic::error(format!("Unknown budget `{}`", name.name), *span)
                                .note("Known budgets: tokens, cents"),
                        );
                    }
                    if *amount < 0 {
                        self.errors.push(Diagnostic::error(
                            "Budget amounts must be non-negative",
                            *span,
                        ));
                    }
                    match name.name.as_str() {
                        "tokens" => self.caps.budget_tokens = *amount,
                        "cents" => self.caps.budget_cents = *amount,
                        _ => {}
                    }
                }
                CapItem::SpawnLimit { limit, span } => {
                    if *limit < 0 {
                        self.errors
                            .push(Diagnostic::error("Spawn limit must be non-negative", *span));
                    }
                    self.caps.spawn_limit = *limit;
                    if *limit > 0 {
                        self.caps
                            .allowed
                            .entry("spawn".into())
                            .or_insert_with(Vec::new);
                    } else {
                        self.caps.denied.insert("spawn".into());
                    }
                }
            }
        }
    }

    fn check_effect_name(&mut self, name: &str, span: Span) {
        if !KNOWN_EFFECTS.contains(&name) {
            self.errors.push(
                Diagnostic::error(format!("Unknown capability `{name}`"), span)
                    .note(format!("Known capabilities: {}", KNOWN_EFFECTS.join(", "))),
            );
        }
    }

    fn register_enum_variants(&mut self, ty: &TypeKey) {
        match ty {
            TypeKey::Enum(variants) => {
                for variant in variants {
                    self.enum_variants.insert(variant.clone(), ty.clone());
                }
            }
            TypeKey::Record(fields) => {
                for (_, field_ty) in fields {
                    self.register_enum_variants(field_ty);
                }
            }
            _ => {}
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt, approved: &mut bool) {
        match stmt {
            Stmt::Let { name, value, .. } => {
                if self.bindings.contains_key(&name.name) {
                    self.errors.push(Diagnostic::error(
                        format!("`{}` is already defined", name.name),
                        name.span,
                    ));
                }
                let info = self.check_expr(value, *approved);
                self.register_enum_variants(&info.ty);
                self.bindings.insert(name.name.clone(), info);
            }
            Stmt::Return { value, .. } => {
                self.check_expr(value, *approved);
            }
            Stmt::Approve { message, span } => {
                let info = self.check_expr(message, *approved);
                if !info.ty.same(&TypeKey::Text) {
                    self.errors.push(Diagnostic::error(
                        format!("`approve` message must be text, found {}", info.ty.name()),
                        message.span,
                    ));
                }
                *approved = true;
                self.effects.push(EffectUse {
                    effect: "approve".into(),
                    detail: "human gate".into(),
                    tainted: info.tainted,
                    needs_approve: false,
                });
                let _ = span;
            }
            Stmt::Send { value, span } => {
                self.require_effect("send", *span, *approved, false, "result");
                self.check_expr(value, *approved);
            }
            Stmt::Expr { value, .. } => {
                self.check_expr(value, *approved);
            }
        }
    }

    fn check_block(&mut self, block: &Block, approved: bool) -> (TypeInfo, bool) {
        let saved = self.bindings.clone();
        let mut local_approved = approved;
        let mut last = TypeInfo {
            ty: TypeKey::Unit,
            tainted: false,
        };
        for stmt in &block.stmts {
            match stmt {
                Stmt::Let { name, value, .. } => {
                    let info = self.check_expr(value, local_approved);
                    self.register_enum_variants(&info.ty);
                    self.bindings.insert(name.name.clone(), info.clone());
                    last = TypeInfo {
                        ty: TypeKey::Unit,
                        tainted: false,
                    };
                    let _ = info;
                }
                Stmt::Return { value, .. } => {
                    last = self.check_expr(value, local_approved);
                }
                Stmt::Approve { message, span } => {
                    let info = self.check_expr(message, local_approved);
                    if !info.ty.same(&TypeKey::Text) {
                        self.errors.push(Diagnostic::error(
                            format!("`approve` message must be text, found {}", info.ty.name()),
                            message.span,
                        ));
                    }
                    local_approved = true;
                    self.effects.push(EffectUse {
                        effect: "approve".into(),
                        detail: "human gate".into(),
                        tainted: info.tainted,
                        needs_approve: false,
                    });
                    last = TypeInfo {
                        ty: TypeKey::Unit,
                        tainted: false,
                    };
                    let _ = span;
                }
                Stmt::Send { value, span } => {
                    self.require_effect("send", *span, local_approved, false, "result");
                    last = self.check_expr(value, local_approved);
                }
                Stmt::Expr { value, .. } => {
                    last = self.check_expr(value, local_approved);
                }
            }
        }
        self.bindings = saved;
        (last, local_approved)
    }

    fn check_expr(&mut self, expr: &Expr, approved: bool) -> TypeInfo {
        match &expr.kind {
            ExprKind::Int(_) => TypeInfo {
                ty: TypeKey::Int,
                tainted: false,
            },
            ExprKind::Text(_) => TypeInfo {
                ty: TypeKey::Text,
                tainted: false,
            },
            ExprKind::Bool(_) => TypeInfo {
                ty: TypeKey::Bool,
                tainted: false,
            },
            ExprKind::Ident(name) => {
                if let Some(info) = self.bindings.get(name) {
                    info.clone()
                } else if let Some(ty) = self.enum_variants.get(name) {
                    TypeInfo {
                        ty: ty.clone(),
                        tainted: false,
                    }
                } else if let Some(ty) = self.enum_type_for_variant(name) {
                    TypeInfo { ty, tainted: false }
                } else {
                    self.errors.push(
                        Diagnostic::error(format!("Unknown name `{name}`"), expr.span)
                            .note("Declare it with `let`, `in`, or use a known enum variant"),
                    );
                    TypeInfo {
                        ty: TypeKey::Unit,
                        tainted: false,
                    }
                }
            }
            ExprKind::Field { base, field } => {
                let info = self.check_expr(base, approved);
                match &info.ty {
                    TypeKey::Record(fields) => {
                        if let Some((_, ty)) = fields.iter().find(|(n, _)| n == &field.name) {
                            TypeInfo {
                                ty: *ty.clone(),
                                tainted: info.tainted,
                            }
                        } else {
                            self.errors.push(Diagnostic::error(
                                format!("No field `{}` on {}", field.name, info.ty.name()),
                                field.span,
                            ));
                            TypeInfo {
                                ty: TypeKey::Unit,
                                tainted: info.tainted,
                            }
                        }
                    }
                    other => {
                        self.errors.push(Diagnostic::error(
                            format!("Cannot access field `{}` on {}", field.name, other.name()),
                            field.span,
                        ));
                        TypeInfo {
                            ty: TypeKey::Unit,
                            tainted: info.tainted,
                        }
                    }
                }
            }
            ExprKind::Unary { op, expr: inner } => {
                let info = self.check_expr(inner, approved);
                match op {
                    UnOp::Neg => {
                        self.expect_ty(&info.ty, &TypeKey::Int, inner.span, "unary `-`");
                        TypeInfo {
                            ty: TypeKey::Int,
                            tainted: info.tainted,
                        }
                    }
                    UnOp::Not => {
                        self.expect_ty(&info.ty, &TypeKey::Bool, inner.span, "`!`");
                        TypeInfo {
                            ty: TypeKey::Bool,
                            tainted: info.tainted,
                        }
                    }
                }
            }
            ExprKind::Binary { op, left, right } => self.check_binary(*op, left, right, approved),
            ExprKind::If {
                cond,
                then_block,
                else_block,
            } => {
                let cond_info = self.check_expr(cond, approved);
                if !cond_info.ty.same(&TypeKey::Bool) && !cond_info.ty.same(&TypeKey::Int) {
                    self.errors.push(Diagnostic::error(
                        format!(
                            "`if` condition must be bool or int, found {}",
                            cond_info.ty.name()
                        ),
                        cond.span,
                    ));
                }
                let (then_info, _then_approved) = self.check_block(then_block, approved);
                if let Some(else_block) = else_block {
                    let (else_info, _else_approved) = self.check_block(else_block, approved);
                    if !then_info.ty.same(&else_info.ty)
                        && !then_info.ty.same(&TypeKey::Unit)
                        && !else_info.ty.same(&TypeKey::Unit)
                    {
                        self.errors.push(Diagnostic::error(
                            format!(
                                "`if` branches have different types: {} vs {}",
                                then_info.ty.name(),
                                else_info.ty.name()
                            ),
                            expr.span,
                        ));
                    }
                    let ty = if then_info.ty.same(&TypeKey::Unit) {
                        else_info.ty
                    } else {
                        then_info.ty
                    };
                    TypeInfo {
                        ty,
                        tainted: then_info.tainted || else_info.tainted,
                    }
                } else {
                    TypeInfo {
                        ty: TypeKey::Unit,
                        tainted: then_info.tainted,
                    }
                }
            }
            ExprKind::Ask {
                prompt,
                using,
                schema,
            } => {
                self.require_effect("ask", expr.span, approved, true, &prompt.value);
                if self.caps.budget_tokens <= 0 {
                    self.errors.push(
                        Diagnostic::error(
                            "`ask` requires a token budget",
                            expr.span,
                        )
                        .note("Add `budget tokens N` to the cap block. Trell will not spend without a ceiling."),
                    );
                }
                if let Some(input) = using {
                    self.check_expr(input, approved);
                }
                let ty = TypeKey::from_ast(&Type::Record(schema.clone()));
                self.register_enum_variants(&ty);
                TypeInfo { ty, tainted: true }
            }
            ExprKind::Read { path } => {
                self.require_effect("read", expr.span, approved, true, "path");
                let info = self.check_expr(path, approved);
                self.expect_ty(&info.ty, &TypeKey::Text, path.span, "`read` path");
                if let ExprKind::Text(value) = &path.kind {
                    self.check_path_grant("read", value, path.span);
                }
                TypeInfo {
                    ty: TypeKey::Text,
                    tainted: true,
                }
            }
            ExprKind::Write { path, body } => {
                self.require_effect("write", expr.span, approved, true, "path");
                let path_info = self.check_expr(path, approved);
                self.expect_ty(&path_info.ty, &TypeKey::Text, path.span, "`write` path");
                self.check_expr(body, approved);
                if let ExprKind::Text(value) = &path.kind {
                    self.check_path_grant("write", value, path.span);
                }
                TypeInfo {
                    ty: TypeKey::Unit,
                    tainted: false,
                }
            }
            ExprKind::Spawn { source } => {
                self.require_effect("spawn", expr.span, approved, true, "child program");
                if self.caps.spawn_limit <= 0 {
                    self.errors.push(
                        Diagnostic::error("`spawn` is not granted (spawn limit is 0)", expr.span)
                            .note("Raise `spawn N` in the cap block, or remove this spawn"),
                    );
                }
                let info = self.check_expr(source, approved);
                if info.tainted {
                    self.errors.push(
                        Diagnostic::error(
                            "Cannot `spawn` a tainted value (model or tool output)",
                            source.span,
                        )
                        .note("Prompt injection is information flow. Spawn source must be clean, reviewed Trell — not `ask` or `read` output."),
                    );
                }
                TypeInfo {
                    ty: TypeKey::Unit,
                    tainted: false,
                }
            }
            ExprKind::Record { fields } => {
                let mut typed = Vec::new();
                let mut tainted = false;
                for (name, value) in fields {
                    let info = self.check_expr(value, approved);
                    tainted |= info.tainted;
                    typed.push((name.name.clone(), Box::new(info.ty)));
                }
                TypeInfo {
                    ty: TypeKey::Record(typed),
                    tainted,
                }
            }
        }
    }

    fn check_binary(&mut self, op: BinOp, left: &Expr, right: &Expr, approved: bool) -> TypeInfo {
        let l = self.check_expr(left, approved);
        let r = self.check_expr(right, approved);
        let tainted = l.tainted || r.tainted;
        match op {
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                if op == BinOp::Add && l.ty.same(&TypeKey::Text) && r.ty.same(&TypeKey::Text) {
                    return TypeInfo {
                        ty: TypeKey::Text,
                        tainted,
                    };
                }
                self.expect_ty(&l.ty, &TypeKey::Int, left.span, op.as_str());
                self.expect_ty(&r.ty, &TypeKey::Int, right.span, op.as_str());
                TypeInfo {
                    ty: TypeKey::Int,
                    tainted,
                }
            }
            BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                self.expect_ty(&l.ty, &TypeKey::Int, left.span, op.as_str());
                self.expect_ty(&r.ty, &TypeKey::Int, right.span, op.as_str());
                TypeInfo {
                    ty: TypeKey::Bool,
                    tainted,
                }
            }
            BinOp::And | BinOp::Or => {
                self.expect_ty(&l.ty, &TypeKey::Bool, left.span, op.as_str());
                self.expect_ty(&r.ty, &TypeKey::Bool, right.span, op.as_str());
                TypeInfo {
                    ty: TypeKey::Bool,
                    tainted,
                }
            }
            BinOp::Eq | BinOp::Ne => {
                if !l.ty.same(&r.ty) {
                    // Allow comparing an enum field to a bare variant name.
                    if let (TypeKey::Enum(variants), TypeKey::Enum(_)) = (&l.ty, &r.ty) {
                        let _ = variants;
                    } else if let TypeKey::Enum(variants) = &l.ty {
                        if let ExprKind::Ident(name) = &right.kind {
                            if variants.iter().any(|v| v == name) {
                                return TypeInfo {
                                    ty: TypeKey::Bool,
                                    tainted,
                                };
                            }
                        }
                    }
                    self.errors.push(Diagnostic::error(
                        format!(
                            "Cannot compare {} and {} with {}",
                            l.ty.name(),
                            r.ty.name(),
                            op.as_str()
                        ),
                        left.span.merge(right.span),
                    ));
                }
                TypeInfo {
                    ty: TypeKey::Bool,
                    tainted,
                }
            }
        }
    }

    fn require_effect(
        &mut self,
        effect: &str,
        span: Span,
        approved: bool,
        tainted: bool,
        detail: &str,
    ) {
        if !self.caps.is_allowed(effect) {
            self.errors.push(
                Diagnostic::error(format!("Effect `{effect}` is not allowed"), span)
                    .note(format!(
                        "Grant it with `allow {effect}` in the cap block, or remove this effect"
                    ))
                    .note("Trell fails closed: missing permission is a compile error"),
            );
        }
        let needs = self.caps.need_approve.contains(effect);
        if needs && !approved {
            self.errors.push(
                Diagnostic::error(
                    format!("Effect `{effect}` requires `approve` first"),
                    span,
                )
                .note(format!(
                    "The cap block says `need approve on {effect}`. Put an `approve` on every path that reaches this effect."
                )),
            );
        }
        self.effects.push(EffectUse {
            effect: effect.into(),
            detail: detail.into(),
            tainted,
            needs_approve: needs,
        });
    }

    fn check_path_grant(&mut self, effect: &str, path: &str, span: Span) {
        if path.contains("..") {
            self.errors.push(
                Diagnostic::error(format!("Path `{path}` must not contain `..`"), span).note(
                    "Trell paths are capability-relative. Parent directory escape is denied.",
                ),
            );
            return;
        }
        if path.starts_with('/') {
            self.errors.push(
                Diagnostic::error(format!("Path `{path}` must be relative"), span)
                    .note("Absolute paths are ambient authority. Grant a glob instead."),
            );
            return;
        }
        let globs = self.caps.paths_for(effect);
        if globs.is_empty() {
            return;
        }
        if !globs.iter().any(|glob| path_matches(glob, path)) {
            self.errors.push(
                Diagnostic::error(format!("Path `{path}` is not granted by `{effect}`"), span)
                    .note(format!("Allowed globs: {}", globs.join(", "))),
            );
        }
    }

    fn expect_ty(&mut self, got: &TypeKey, want: &TypeKey, span: Span, what: &str) {
        if !got.same(want) {
            self.errors.push(Diagnostic::error(
                format!("{what} expected {}, found {}", want.name(), got.name()),
                span,
            ));
        }
    }

    fn enum_type_for_variant(&self, name: &str) -> Option<TypeKey> {
        for info in self.bindings.values() {
            if let Some(found) = find_enum_with_variant(&info.ty, name) {
                return Some(found);
            }
        }
        None
    }
}

fn find_enum_with_variant(ty: &TypeKey, name: &str) -> Option<TypeKey> {
    match ty {
        TypeKey::Enum(variants) if variants.iter().any(|v| v == name) => Some(ty.clone()),
        TypeKey::Record(fields) => fields
            .iter()
            .find_map(|(_, field_ty)| find_enum_with_variant(field_ty, name)),
        _ => None,
    }
}

pub fn path_matches(glob: &str, path: &str) -> bool {
    glob_match(glob, path)
}

fn glob_match(glob: &str, path: &str) -> bool {
    let glob_parts: Vec<&str> = glob.split('/').collect();
    let path_parts: Vec<&str> = path.split('/').collect();
    match_parts(&glob_parts, &path_parts)
}

fn match_parts(glob: &[&str], path: &[&str]) -> bool {
    match (glob.first().copied(), path.first().copied()) {
        (None, None) => true,
        (Some("**"), _) => {
            if glob.len() == 1 {
                return true;
            }
            if match_parts(&glob[1..], path) {
                return true;
            }
            if path.is_empty() {
                return false;
            }
            match_parts(glob, &path[1..])
        }
        (Some(g), Some(p)) => {
            if g == "*" || g == p || segment_star(g, p) {
                match_parts(&glob[1..], &path[1..])
            } else {
                false
            }
        }
        _ => false,
    }
}

fn segment_star(glob: &str, path: &str) -> bool {
    if !glob.contains('*') {
        return glob == path;
    }
    let parts: Vec<&str> = glob.split('*').collect();
    if parts.len() == 2 {
        path.starts_with(parts[0]) && path.ends_with(parts[1]) && path.len() >= glob.len() - 1
    } else {
        glob == path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn check_src(src: &str) -> Result<CheckedProgram, Vec<Diagnostic>> {
        check(parse(src).unwrap())
    }

    #[test]
    fn arithmetic_needs_no_cap() {
        assert!(check_src("20 + 22 * 2").is_ok());
    }

    #[test]
    fn ask_without_allow_fails() {
        let err = check_src(r#"ask "hi" as { ok: bool }"#).unwrap_err();
        assert!(err.iter().any(|d| d.message.contains("ask")));
    }

    #[test]
    fn write_without_approve_fails() {
        let src = r#"
cap demo {
  allow write "notes/**"
  need approve on write
}
write "notes/out.md" "hello"
"#;
        let err = check_src(src).unwrap_err();
        assert!(err.iter().any(|d| d.message.contains("approve")));
    }

    #[test]
    fn write_after_approve_ok() {
        let src = r#"
cap demo {
  allow write "notes/**"
  need approve on write
}
approve "human said yes"
write "notes/out.md" "hello"
"#;
        assert!(check_src(src).is_ok());
    }

    #[test]
    fn tainted_spawn_fails() {
        let src = r#"
cap demo {
  allow ask
  budget tokens 100
  spawn 1
}
let code = ask "write a child" as { body: text }
spawn code.body
"#;
        let err = check_src(src).unwrap_err();
        assert!(err.iter().any(|d| d.message.contains("tainted")));
    }

    #[test]
    fn glob_matching() {
        assert!(path_matches("src/**", "src/main.rs"));
        assert!(path_matches("src/**", "src/a/b.rs"));
        assert!(!path_matches("src/**", "docs/a.rs"));
        assert!(path_matches("*.md", "README.md"));
        assert!(!path_matches("*.md", "docs/README.md"));
    }
}
