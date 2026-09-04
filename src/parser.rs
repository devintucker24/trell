use crate::ast::*;
use crate::diagnostics::Diagnostic;
use crate::lexer::{Token, TokenKind};
use crate::span::Span;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse_program(&mut self) -> Result<Program, Diagnostic> {
        let start = self.span();
        let cap = if self.at(TokenKind::Cap) {
            Some(self.parse_cap()?)
        } else {
            None
        };

        let mut inputs = Vec::new();
        while self.at(TokenKind::In) {
            inputs.push(self.parse_input()?);
        }

        let mut body = Vec::new();
        while !self.at(TokenKind::Eof) {
            body.push(self.parse_stmt()?);
        }

        let end = self.previous_span();
        Ok(Program {
            span: start.merge(end),
            cap,
            inputs,
            body,
        })
    }

    fn parse_cap(&mut self) -> Result<CapBlock, Diagnostic> {
        let start = self.span();
        self.expect(TokenKind::Cap)?;
        let name = if self.is_ident_like() && !self.at(TokenKind::LBrace) {
            Some(self.parse_ident()?)
        } else {
            None
        };
        self.expect(TokenKind::LBrace)?;
        let mut items = Vec::new();
        while !self.at(TokenKind::RBrace) && !self.at(TokenKind::Eof) {
            items.push(self.parse_cap_item()?);
        }
        self.expect(TokenKind::RBrace)?;
        Ok(CapBlock {
            span: start.merge(self.previous_span()),
            name,
            items,
        })
    }

    fn parse_cap_item(&mut self) -> Result<CapItem, Diagnostic> {
        let start = self.span();
        match self.kind() {
            TokenKind::Allow => {
                self.advance();
                let name = self.parse_ident()?;
                let mut paths = Vec::new();
                while matches!(self.kind(), TokenKind::Text(_)) {
                    paths.push(self.parse_string()?);
                }
                Ok(CapItem::Allow {
                    span: start.merge(self.previous_span()),
                    name,
                    paths,
                })
            }
            TokenKind::Deny => {
                self.advance();
                let name = self.parse_ident()?;
                Ok(CapItem::Deny {
                    span: start.merge(self.previous_span()),
                    name,
                })
            }
            TokenKind::Need => {
                self.advance();
                self.expect(TokenKind::Approve)?;
                self.expect(TokenKind::On)?;
                let effect = self.parse_ident()?;
                Ok(CapItem::NeedApprove {
                    span: start.merge(self.previous_span()),
                    effect,
                })
            }
            TokenKind::Budget => {
                self.advance();
                let name = self.parse_ident()?;
                let amount = self.parse_int_literal()?;
                Ok(CapItem::Budget {
                    span: start.merge(self.previous_span()),
                    name,
                    amount,
                })
            }
            TokenKind::Spawn => {
                self.advance();
                let limit = self.parse_int_literal()?;
                Ok(CapItem::SpawnLimit {
                    span: start.merge(self.previous_span()),
                    limit,
                })
            }
            _ => Err(self.error(format!(
                "Expected a capability statement (allow, deny, need, budget, spawn), found {}",
                self.kind().as_str()
            ))),
        }
    }

    fn parse_input(&mut self) -> Result<InputDecl, Diagnostic> {
        let start = self.span();
        self.expect(TokenKind::In)?;
        let name = self.parse_ident()?;
        self.expect(TokenKind::Colon)?;
        let ty = self.parse_type()?;
        Ok(InputDecl {
            span: start.merge(self.previous_span()),
            name,
            ty,
        })
    }

    fn parse_stmt(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.span();
        match self.kind() {
            TokenKind::Let => {
                self.advance();
                let name = self.parse_ident()?;
                self.expect(TokenKind::Assign)?;
                let value = self.parse_expr()?;
                Ok(Stmt::Let {
                    span: start.merge(value.span),
                    name,
                    value,
                })
            }
            TokenKind::Return => {
                self.advance();
                let value = self.parse_expr()?;
                Ok(Stmt::Return {
                    span: start.merge(value.span),
                    value,
                })
            }
            TokenKind::Approve => {
                self.advance();
                let message = self.parse_expr()?;
                Ok(Stmt::Approve {
                    span: start.merge(message.span),
                    message,
                })
            }
            TokenKind::Send => {
                self.advance();
                let value = self.parse_expr()?;
                Ok(Stmt::Send {
                    span: start.merge(value.span),
                    value,
                })
            }
            _ => {
                let value = self.parse_expr()?;
                Ok(Stmt::Expr {
                    span: value.span,
                    value,
                })
            }
        }
    }

    fn parse_block(&mut self) -> Result<Block, Diagnostic> {
        let start = self.span();
        self.expect(TokenKind::LBrace)?;
        let mut stmts = Vec::new();
        while !self.at(TokenKind::RBrace) && !self.at(TokenKind::Eof) {
            stmts.push(self.parse_stmt()?);
        }
        self.expect(TokenKind::RBrace)?;
        Ok(Block {
            span: start.merge(self.previous_span()),
            stmts,
        })
    }

    fn parse_expr(&mut self) -> Result<Expr, Diagnostic> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.parse_and()?;
        while self.at(TokenKind::OrOr) {
            self.advance();
            let right = self.parse_and()?;
            expr = binary(BinOp::Or, expr, right);
        }
        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.parse_equality()?;
        while self.at(TokenKind::AndAnd) {
            self.advance();
            let right = self.parse_equality()?;
            expr = binary(BinOp::And, expr, right);
        }
        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.parse_comparison()?;
        loop {
            let op = match self.kind() {
                TokenKind::EqEq => BinOp::Eq,
                TokenKind::NotEq => BinOp::Ne,
                _ => break,
            };
            self.advance();
            let right = self.parse_comparison()?;
            expr = binary(op, expr, right);
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, Diagnostic> {
        let expr = self.parse_addition()?;
        let op = match self.kind() {
            TokenKind::Lt => BinOp::Lt,
            TokenKind::LtEq => BinOp::Le,
            TokenKind::Gt => BinOp::Gt,
            TokenKind::GtEq => BinOp::Ge,
            _ => return Ok(expr),
        };
        self.advance();
        let right = self.parse_addition()?;
        Ok(binary(op, expr, right))
    }

    fn parse_addition(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.parse_multiplication()?;
        loop {
            let op = match self.kind() {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplication()?;
            expr = binary(op, expr, right);
        }
        Ok(expr)
    }

    fn parse_multiplication(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.parse_unary()?;
        loop {
            let op = match self.kind() {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            expr = binary(op, expr, right);
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, Diagnostic> {
        let start = self.span();
        if self.at(TokenKind::Minus) {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr {
                span: start.merge(expr.span),
                kind: ExprKind::Unary {
                    op: UnOp::Neg,
                    expr: Box::new(expr),
                },
            });
        }
        if self.at(TokenKind::Not) {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr {
                span: start.merge(expr.span),
                kind: ExprKind::Unary {
                    op: UnOp::Not,
                    expr: Box::new(expr),
                },
            });
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.parse_primary()?;
        while self.at(TokenKind::Dot) {
            self.advance();
            let field = self.parse_ident()?;
            expr = Expr {
                span: expr.span.merge(field.span),
                kind: ExprKind::Field {
                    base: Box::new(expr),
                    field,
                },
            };
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, Diagnostic> {
        let start = self.span();
        match self.kind().clone() {
            TokenKind::Int(value) => {
                self.advance();
                Ok(Expr {
                    span: start,
                    kind: ExprKind::Int(value),
                })
            }
            TokenKind::Text(value) => {
                self.advance();
                Ok(Expr {
                    span: start,
                    kind: ExprKind::Text(value),
                })
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr {
                    span: start,
                    kind: ExprKind::Bool(true),
                })
            }
            TokenKind::False => {
                self.advance();
                Ok(Expr {
                    span: start,
                    kind: ExprKind::Bool(false),
                })
            }
            TokenKind::Ident(name) => {
                self.advance();
                Ok(Expr {
                    span: start,
                    kind: ExprKind::Ident(name),
                })
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Ok(expr)
            }
            TokenKind::LBrace => self.parse_record(),
            TokenKind::If => self.parse_if(),
            TokenKind::Ask => self.parse_ask(),
            TokenKind::Read => {
                self.advance();
                let path = self.parse_expr()?;
                Ok(Expr {
                    span: start.merge(path.span),
                    kind: ExprKind::Read {
                        path: Box::new(path),
                    },
                })
            }
            TokenKind::Write => {
                self.advance();
                let path = self.parse_unary()?;
                let body = self.parse_expr()?;
                Ok(Expr {
                    span: start.merge(body.span),
                    kind: ExprKind::Write {
                        path: Box::new(path),
                        body: Box::new(body),
                    },
                })
            }
            TokenKind::Spawn => {
                self.advance();
                let source = self.parse_expr()?;
                Ok(Expr {
                    span: start.merge(source.span),
                    kind: ExprKind::Spawn {
                        source: Box::new(source),
                    },
                })
            }
            other => Err(self.error(format!("Expected an expression, found {}", other.as_str()))),
        }
    }

    fn parse_if(&mut self) -> Result<Expr, Diagnostic> {
        let start = self.span();
        self.expect(TokenKind::If)?;
        let cond = self.parse_expr()?;
        let then_block = self.parse_block()?;
        let else_block = if self.at(TokenKind::Else) {
            self.advance();
            if self.at(TokenKind::If) {
                let nested = self.parse_if()?;
                Some(Block {
                    span: nested.span,
                    stmts: vec![Stmt::Expr {
                        span: nested.span,
                        value: nested,
                    }],
                })
            } else {
                Some(self.parse_block()?)
            }
        } else {
            None
        };
        Ok(Expr {
            span: start.merge(self.previous_span()),
            kind: ExprKind::If {
                cond: Box::new(cond),
                then_block,
                else_block,
            },
        })
    }

    fn parse_ask(&mut self) -> Result<Expr, Diagnostic> {
        let start = self.span();
        self.expect(TokenKind::Ask)?;
        let prompt = self.parse_string()?;
        let using = if self.at(TokenKind::Using) {
            self.advance();
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };
        self.expect(TokenKind::As)?;
        let schema = self.parse_schema()?;
        Ok(Expr {
            span: start.merge(schema.span),
            kind: ExprKind::Ask {
                prompt,
                using,
                schema,
            },
        })
    }

    fn parse_record(&mut self) -> Result<Expr, Diagnostic> {
        let start = self.span();
        self.expect(TokenKind::LBrace)?;
        let mut fields = Vec::new();
        while !self.at(TokenKind::RBrace) && !self.at(TokenKind::Eof) {
            let name = self.parse_ident()?;
            self.expect(TokenKind::Colon)?;
            let value = self.parse_expr()?;
            fields.push((name, value));
            if self.at(TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        self.expect(TokenKind::RBrace)?;
        Ok(Expr {
            span: start.merge(self.previous_span()),
            kind: ExprKind::Record { fields },
        })
    }

    fn parse_schema(&mut self) -> Result<Schema, Diagnostic> {
        let start = self.span();
        self.expect(TokenKind::LBrace)?;
        let mut fields = Vec::new();
        while !self.at(TokenKind::RBrace) && !self.at(TokenKind::Eof) {
            let name = self.parse_ident()?;
            self.expect(TokenKind::Colon)?;
            let ty = self.parse_type()?;
            fields.push((name, ty));
            if self.at(TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        self.expect(TokenKind::RBrace)?;
        Ok(Schema {
            span: start.merge(self.previous_span()),
            fields,
        })
    }

    fn parse_type(&mut self) -> Result<Type, Diagnostic> {
        match self.kind() {
            TokenKind::IntType => {
                self.advance();
                Ok(Type::Int)
            }
            TokenKind::TextType => {
                self.advance();
                Ok(Type::Text)
            }
            TokenKind::BoolType => {
                self.advance();
                Ok(Type::Bool)
            }
            TokenKind::Enum => {
                self.advance();
                self.expect(TokenKind::LParen)?;
                let mut variants = Vec::new();
                loop {
                    variants.push(self.parse_ident()?);
                    if self.at(TokenKind::Comma) {
                        self.advance();
                    } else {
                        break;
                    }
                }
                self.expect(TokenKind::RParen)?;
                if variants.is_empty() {
                    return Err(self.error("enum must have at least one variant".to_string()));
                }
                Ok(Type::Enum { variants })
            }
            TokenKind::LBrace => {
                let schema = self.parse_schema()?;
                Ok(Type::Record(schema))
            }
            _ => Err(self.error(format!(
                "Expected a type (int, text, bool, enum(...), or a record), found {}",
                self.kind().as_str()
            ))),
        }
    }

    fn parse_ident(&mut self) -> Result<Ident, Diagnostic> {
        let span = self.span();
        if let Some(name) = self.kind().ident_name() {
            let name = name.to_string();
            self.advance();
            Ok(Ident { span, name })
        } else {
            Err(self.error(format!("Expected a name, found {}", self.kind().as_str())))
        }
    }

    fn parse_string(&mut self) -> Result<StringLit, Diagnostic> {
        let span = self.span();
        match self.kind().clone() {
            TokenKind::Text(value) => {
                self.advance();
                Ok(StringLit { span, value })
            }
            _ => Err(self.error(format!(
                "Expected a string literal, found {}",
                self.kind().as_str()
            ))),
        }
    }

    fn parse_int_literal(&mut self) -> Result<i64, Diagnostic> {
        match self.kind().clone() {
            TokenKind::Int(value) => {
                self.advance();
                Ok(value)
            }
            _ => Err(self.error(format!(
                "Expected an integer, found {}",
                self.kind().as_str()
            ))),
        }
    }

    fn is_ident_like(&self) -> bool {
        self.kind().ident_name().is_some()
    }

    fn at(&self, kind: TokenKind) -> bool {
        std::mem::discriminant(self.kind()) == std::mem::discriminant(&kind)
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), Diagnostic> {
        if self.at(kind.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(format!(
                "Expected {}, found {}",
                kind.as_str(),
                self.kind().as_str()
            )))
        }
    }

    fn kind(&self) -> &TokenKind {
        &self.tokens[self.current].kind
    }

    fn span(&self) -> Span {
        self.tokens[self.current].span
    }

    fn previous_span(&self) -> Span {
        if self.current == 0 {
            self.span()
        } else {
            self.tokens[self.current - 1].span
        }
    }

    fn advance(&mut self) {
        if !matches!(self.kind(), TokenKind::Eof) {
            self.current += 1;
        }
    }

    fn error(&self, message: String) -> Diagnostic {
        Diagnostic::error(message, self.span())
    }
}

fn binary(op: BinOp, left: Expr, right: Expr) -> Expr {
    Expr {
        span: left.span.merge(right.span),
        kind: ExprKind::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        },
    }
}

pub fn parse(source: &str) -> Result<Program, Diagnostic> {
    let tokens = crate::lexer::lex(source)?;
    Parser::new(tokens).parse_program()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_arithmetic() {
        let program = parse("20 + 22 * 2").unwrap();
        assert!(program.cap.is_none());
        assert_eq!(program.body.len(), 1);
    }

    #[test]
    fn parses_workflow() {
        let source = r#"
cap review {
  allow read "src/**"
  allow ask
  deny write
  need approve on write
  budget tokens 8000
  spawn 0
}

in diff: text

let review = ask "risk?" using diff as { risk: enum(low, medium, high), reason: text }
if review.risk == high {
  approve "blocked"
}
send review
"#;
        let program = parse(source).unwrap();
        assert!(program.cap.is_some());
        assert_eq!(program.inputs.len(), 1);
        assert_eq!(program.body.len(), 3);
    }

    #[test]
    fn parses_if_else_if() {
        let program = parse("if x { 1 } else if y { 2 } else { 3 }").unwrap();
        assert_eq!(program.body.len(), 1);
    }
}
