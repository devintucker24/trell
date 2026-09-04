use crate::span::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub span: Span,
    pub cap: Option<CapBlock>,
    pub inputs: Vec<InputDecl>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapBlock {
    pub span: Span,
    pub name: Option<Ident>,
    pub items: Vec<CapItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapItem {
    Allow {
        span: Span,
        name: Ident,
        paths: Vec<StringLit>,
    },
    Deny {
        span: Span,
        name: Ident,
    },
    NeedApprove {
        span: Span,
        effect: Ident,
    },
    Budget {
        span: Span,
        name: Ident,
        amount: i64,
    },
    SpawnLimit {
        span: Span,
        limit: i64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputDecl {
    pub span: Span,
    pub name: Ident,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Let {
        span: Span,
        name: Ident,
        value: Expr,
    },
    Return {
        span: Span,
        value: Expr,
    },
    Approve {
        span: Span,
        message: Expr,
    },
    Send {
        span: Span,
        value: Expr,
    },
    Expr {
        span: Span,
        value: Expr,
    },
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Stmt::Let { span, .. }
            | Stmt::Return { span, .. }
            | Stmt::Approve { span, .. }
            | Stmt::Send { span, .. }
            | Stmt::Expr { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub span: Span,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr {
    pub span: Span,
    pub kind: ExprKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprKind {
    Int(i64),
    Text(String),
    Bool(bool),
    Ident(String),
    Field {
        base: Box<Expr>,
        field: Ident,
    },
    Unary {
        op: UnOp,
        expr: Box<Expr>,
    },
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    If {
        cond: Box<Expr>,
        then_block: Block,
        else_block: Option<Block>,
    },
    Ask {
        prompt: StringLit,
        using: Option<Box<Expr>>,
        schema: Schema,
    },
    Read {
        path: Box<Expr>,
    },
    Write {
        path: Box<Expr>,
        body: Box<Expr>,
    },
    Spawn {
        source: Box<Expr>,
    },
    Record {
        fields: Vec<(Ident, Expr)>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
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
    And,
    Or,
}

impl BinOp {
    pub fn as_str(self) -> &'static str {
        match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Eq => "==",
            BinOp::Ne => "!=",
            BinOp::Lt => "<",
            BinOp::Le => "<=",
            BinOp::Gt => ">",
            BinOp::Ge => ">=",
            BinOp::And => "&&",
            BinOp::Or => "||",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schema {
    pub span: Span,
    pub fields: Vec<(Ident, Type)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Text,
    Bool,
    Unit,
    Enum { variants: Vec<Ident> },
    Record(Schema),
}

impl Type {
    pub fn name(&self) -> String {
        match self {
            Type::Int => "int".into(),
            Type::Text => "text".into(),
            Type::Bool => "bool".into(),
            Type::Unit => "unit".into(),
            Type::Enum { variants } => {
                let names: Vec<_> = variants.iter().map(|v| v.name.as_str()).collect();
                format!("enum({})", names.join(", "))
            }
            Type::Record(schema) => {
                let fields: Vec<_> = schema
                    .fields
                    .iter()
                    .map(|(n, t)| format!("{}: {}", n.name, t.name()))
                    .collect();
                format!("{{ {} }}", fields.join(", "))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident {
    pub span: Span,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLit {
    pub span: Span,
    pub value: String,
}

impl Program {
    pub fn is_pure_compute(&self) -> bool {
        if self.cap.is_some() {
            return false;
        }
        if !self
            .inputs
            .iter()
            .all(|input| matches!(input.ty, Type::Int))
        {
            return false;
        }
        self.body.iter().all(stmt_is_compute)
    }
}

fn stmt_is_compute(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Let { value, .. } | Stmt::Return { value, .. } | Stmt::Expr { value, .. } => {
            expr_is_compute(value)
        }
        Stmt::Approve { .. } | Stmt::Send { .. } => false,
    }
}

fn expr_is_compute(expr: &Expr) -> bool {
    match &expr.kind {
        ExprKind::Int(_) | ExprKind::Bool(_) | ExprKind::Ident(_) => true,
        ExprKind::Text(_)
        | ExprKind::Ask { .. }
        | ExprKind::Read { .. }
        | ExprKind::Write { .. }
        | ExprKind::Spawn { .. }
        | ExprKind::Record { .. }
        | ExprKind::Field { .. } => false,
        ExprKind::Unary { expr, .. } => expr_is_compute(expr),
        ExprKind::Binary { left, right, .. } => expr_is_compute(left) && expr_is_compute(right),
        ExprKind::If {
            cond,
            then_block,
            else_block,
        } => {
            expr_is_compute(cond)
                && then_block.stmts.iter().all(stmt_is_compute)
                && else_block
                    .as_ref()
                    .map(|b| b.stmts.iter().all(stmt_is_compute))
                    .unwrap_or(true)
        }
    }
}
