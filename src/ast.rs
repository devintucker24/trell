// Palimpsest Abstract Syntax Tree

use crate::types::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// `trust legal above policy above user above rumor`
    Trust(Vec<String>),

    /// `about acme.alice:` followed by an indented block
    About { prefix: Vec<String>, body: Vec<Stmt> },

    /// `alice.city is "Berlin" from relocation_ticket on 2026-08-15`
    Fact {
        path: Vec<String>,
        value: Expr,
        facets: Facets,
        line: usize,
    },

    /// `when db_outage:` followed by an indented block
    Episode {
        id: String,
        happened: Option<Expr>,
        involved: Vec<Expr>,
        details: Vec<(String, Expr)>,
        summary: Option<Expr>,
    },

    /// `forget everything from phishing_email`
    ForgetSource(Expr),
    /// `forget when db_outage`
    ForgetEpisode(String),
    /// `forget alice.city`
    ForgetPath(Vec<String>),

    /// `let x = what is alice.city`
    Let { name: String, expr: Expr },

    /// A bare expression at statement position prints its own value.
    Show(Expr),

    /// `expect what is alice.city is "Berlin"`
    Expect { left: Expr, right: Expr, line: usize },

    /// `now is 2026-09-04`
    NowIs(Expr),
    /// `later by 30 days`
    LaterBy(Expr),
}

/// The trailing prepositional phrases that qualify a fact.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Facets {
    /// `as policy`
    pub authority: Option<String>,
    /// `from hr_handbook`
    pub source: Option<Expr>,
    /// `unverified` / `verified`
    pub verified: Option<bool>,
    /// `on 2026-08-15` / `since 2026-08-15`
    pub asserted_at: Option<Expr>,
    /// `for 90 days`
    pub ttl: Option<Expr>,
    /// `until 2027-01-01`
    pub until: Option<Expr>,
    /// `because db_outage`
    pub because: Option<String>,
}

/// Conditions a query places on whatever it resolves. A query that cannot meet
/// them refuses rather than downgrading.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Demands {
    /// `verified` — refuse a belief with no trustworthy provenance.
    pub verified: bool,
    /// `fresh` — refuse a belief that has outlived its lifetime.
    pub fresh: bool,
    /// `trusted <authority>` — refuse anything below that rank.
    pub min_authority: Option<String>,
}

impl Demands {
    pub fn any(&self) -> bool {
        self.verified || self.fresh || self.min_authority.is_some()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Value),
    Variable(String),

    /// `what is alice.city` / `what was alice.city on 2026-04-01`
    Ask {
        path: Vec<String>,
        as_of: Option<Box<Expr>>,
        demands: Demands,
    },

    /// `why alice.city` — the full layered history of a name.
    Why(Vec<String>),

    /// `conflicts` — every defeated override recorded so far.
    Conflicts,

    /// `episodes` — the episodic log.
    Episodes,

    /// `check` — a health report over the whole belief store.
    Check,

    List(Vec<Expr>),
    Record(Vec<(String, Expr)>),

    Binary { op: BinOp, left: Box<Expr>, right: Box<Expr> },
    Unary { op: UnOp, expr: Box<Expr> },
    Field { expr: Box<Expr>, field: String },
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
