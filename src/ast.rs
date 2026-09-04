#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Axes(Vec<AxisDecl>),
    Offer {
        coords: Vec<(String, f64)>,
        text: String,
    },
    Path {
        name: String,
        steps: Vec<Step>,
    },
    Stmt(Stmt),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AxisDecl {
    pub name: String,
    pub low: String,
    pub high: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Grain {
        name: String,
        expr: GrainExpr,
    },
    Speak(GrainExpr),
    Echo(EchoTarget),
    When {
        cond: Cond,
        then_body: Vec<Stmt>,
        else_body: Vec<Stmt>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum EchoTarget {
    Grain(GrainExpr),
    Space,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Cond {
    ResonatePhrase {
        grain: GrainExpr,
        phrase: String,
    },
    ResonateGrains {
        left: GrainExpr,
        right: GrainExpr,
    },
    Axis {
        grain: String,
        axis: String,
        op: CmpOp,
        value: f64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp {
    Gt,
    Lt,
    Ge,
    Le,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GrainExpr {
    Feel(String),
    Name(String),
    ShadowOf(Box<GrainExpr>),
    Blend {
        left: Box<GrainExpr>,
        right: Box<GrainExpr>,
        by: f64,
    },
    Pipeline {
        base: Box<GrainExpr>,
        steps: Vec<Step>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Step {
    Along {
        axis: String,
        toward: f64,
        by: Option<f64>,
    },
    Keeping(Vec<String>),
    Via(String),
    Without(Box<GrainExpr>),
    WithShadow,
}

impl GrainExpr {
    pub fn label(&self) -> String {
        match self {
            GrainExpr::Feel(_) => "felt".into(),
            GrainExpr::Name(name) => name.clone(),
            GrainExpr::ShadowOf(inner) => format!("shadow of {}", inner.label()),
            GrainExpr::Blend { .. } => "blend".into(),
            GrainExpr::Pipeline { base, .. } => base.label(),
        }
    }
}
