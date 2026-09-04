use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimitiveType {
    Int,
    Float,
    Bool,
    String,
    Json,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
    Certain(PrimitiveType),
    CertainCustom(String),
    Belief(PrimitiveType),
    BeliefCustom(String),
    Unit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelContract {
    pub name: String,
    pub model_kind: String, // e.g. "reasoning", "fast", "multimodal"
    pub temperature: Option<f64>,
    pub max_tokens: Option<u64>,
    pub min_confidence: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuardDef {
    pub name: String,
    pub param_name: String,
    pub param_type: Type,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Type,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForkCase {
    pub pattern_name: String,
    pub binding: Option<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stmt {
    Let {
        name: String,
        ty: Option<Type>,
        value: Expr,
    },
    Assign {
        target: String,
        value: Expr,
    },
    Return(Option<Expr>),
    Expr(Expr),
    Print(Expr),
    Assert {
        condition: Expr,
        message: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    Lit(Literal),
    Ident(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    UnaryNot(Box<Expr>),
    Block(Vec<Stmt>, Option<Box<Expr>>),
    Call {
        function: String,
        args: Vec<Expr>,
    },
    FieldAccess {
        target: Box<Expr>,
        field: String,
    },
    StructInit {
        name: String,
        fields: Vec<(String, Expr)>,
    },
    // Epistemic Oracle Deliberation
    // e.g.: oracle<DiagnosticOracle>.assess("Patient exhibits acute abdominal pain")
    OracleCall {
        contract: String,
        method: String,
        prompt_arg: Box<Expr>,
        target_type: Type,
    },
    // Epistemic reduction / invariant verification
    // e.g.: verify candidate with IsApprovedHash
    Verify {
        target: Box<Expr>,
        guard_name: String,
        fallback: Option<Box<Expr>>,
    },
    // Epistemic consensus across n samples
    // e.g.: consensus(3, 0.7) { oracle<Auditor>.judge(tx) }
    Consensus {
        count: usize,
        threshold: f64,
        oracle_call: Box<Expr>,
    },
    // Speculative semantic execution fork
    // fork belief_expr { case A => ... case B => ... fallback => ... } collapse
    Fork {
        target: Box<Expr>,
        cases: Vec<ForkCase>,
        fallback: Option<Vec<Stmt>>,
    },
    // Extract metadata from a belief
    Confidence(Box<Expr>),
    Justification(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Contract(ModelContract),
    Struct(StructDef),
    Guard(GuardDef),
    Function(FunctionDef),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub items: Vec<Item>,
}
