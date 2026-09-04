// Palimpsest Abstract Syntax Tree (AST)

use crate::types::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// authority Legal > Compliance > Policy > User > Unverified;
    AuthorityDecl(Vec<String>),

    /// scope user.alice { ... }
    Scope {
        prefix: Vec<String>,
        body: Vec<Stmt>,
    },

    /// assert user.location = "Berlin" @ authority(User), source("chat_08");
    Assert {
        path: Vec<String>,
        value: Expr,
        modifiers: AssertModifiers,
    },

    /// episode db_failure { at: "...", actors: [...], context: { ... }, summary: "..." }
    Episode {
        id: String,
        at: Expr,
        actors: Vec<Expr>,
        context: Vec<(String, Expr)>,
        summary: Expr,
    },

    /// retract source "phishing_email";
    RetractSource(Expr),

    /// retract belief user.location;
    RetractBelief(Vec<String>),

    /// retract episode db_failure;
    RetractEpisode(String),

    /// let x = recall user.location;
    Let {
        name: String,
        expr: Expr,
    },

    /// print expr;
    Print(Expr),

    /// assert_eq left, right;
    AssertEq {
        left: Expr,
        right: Expr,
    },

    /// set_time "2026-09-04T12:00:00Z";
    SetTime(Expr),

    /// advance_time 48h;
    AdvanceTime(Expr),

    /// Expression statement
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AssertModifiers {
    pub authority: Option<String>,
    pub source: Option<Expr>,
    pub verified: Option<bool>,
    pub at: Option<Expr>,
    pub ttl: Option<Expr>,
    pub valid_until: Option<Expr>,
    pub grounded_in: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Value),
    Variable(String),
    Path(Vec<String>),
    Recall {
        path: Vec<String>,
        as_of: Option<Box<Expr>>,
        fresh: bool,
        verified_only: bool,
        min_authority: Option<String>,
    },
    History(Vec<String>),
    Audit(Vec<String>),
    Conflicts,
    Episodes,
    List(Vec<Expr>),
    Record(Vec<(String, Expr)>),
    BinaryOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnOp,
        expr: Box<Expr>,
    },
    FieldAccess {
        expr: Box<Expr>,
        field: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Not,
    Neg,
}
