use anyhow::{anyhow, Result};

use crate::ast::{AxisDecl, CmpOp, Cond, EchoTarget, GrainExpr, Item, Program, Step, Stmt};
use crate::lexer::{Located, Token};

pub struct Parser {
    tokens: Vec<Located>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Located>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        let mut items = Vec::new();
        while !self.check(&Token::Eof) {
            items.push(self.parse_item()?);
        }
        Ok(Program { items })
    }

    fn parse_item(&mut self) -> Result<Item> {
        match self.current_token() {
            Token::Axes => {
                self.advance();
                self.expect(&Token::LBrace, "Expected '{' after axes")?;
                let mut axes = Vec::new();
                while !self.check(&Token::RBrace) && !self.check(&Token::Eof) {
                    axes.push(self.parse_axis()?);
                }
                self.expect(&Token::RBrace, "Expected '}' to close axes")?;
                Ok(Item::Axes(axes))
            }
            Token::Offer => {
                self.advance();
                self.expect(&Token::At, "Expected 'at' after offer")?;
                let mut coords = Vec::new();
                coords.push(self.parse_coord()?);
                while self.match_token(&Token::Comma) {
                    coords.push(self.parse_coord()?);
                }
                self.expect(&Token::Colon, "Expected ':' after offer coordinates")?;
                let text = self.expect_string("Expected offer text")?;
                Ok(Item::Offer { coords, text })
            }
            Token::Path => {
                self.advance();
                let name = self.expect_ident("Expected path name")?;
                self.expect(&Token::LBrace, "Expected '{' after path name")?;
                let mut steps = Vec::new();
                while !self.check(&Token::RBrace) && !self.check(&Token::Eof) {
                    steps.push(self.parse_step()?);
                }
                if steps.is_empty() {
                    return Err(self.error("Path must contain at least one step"));
                }
                self.expect(&Token::RBrace, "Expected '}' to close path")?;
                Ok(Item::Path { name, steps })
            }
            _ => Ok(Item::Stmt(self.parse_stmt()?)),
        }
    }

    fn parse_axis(&mut self) -> Result<AxisDecl> {
        let name = self.expect_ident("Expected axis name")?;
        self.expect(&Token::Colon, "Expected ':' after axis name")?;
        let low = self.expect_string("Expected low pole string")?;
        self.expect(&Token::Arrow, "Expected '<->' between poles")?;
        let high = self.expect_string("Expected high pole string")?;
        Ok(AxisDecl { name, low, high })
    }

    fn parse_coord(&mut self) -> Result<(String, f64)> {
        let name = self.expect_ident("Expected axis name in offer")?;
        self.expect(&Token::Eq, "Expected '=' in offer coordinate")?;
        let value = self.expect_number("Expected number in offer coordinate")?;
        Ok((name, value))
    }

    fn parse_stmt(&mut self) -> Result<Stmt> {
        match self.current_token() {
            Token::Grain => {
                self.advance();
                let name = self.expect_ident("Expected grain name")?;
                self.expect(&Token::Eq, "Expected '=' after grain name")?;
                let expr = self.parse_grain_expr()?;
                Ok(Stmt::Grain { name, expr })
            }
            Token::Speak => {
                self.advance();
                Ok(Stmt::Speak(self.parse_grain_expr()?))
            }
            Token::Echo => {
                self.advance();
                if self.match_token(&Token::Space) {
                    Ok(Stmt::Echo(EchoTarget::Space))
                } else {
                    Ok(Stmt::Echo(EchoTarget::Grain(self.parse_grain_expr()?)))
                }
            }
            Token::When => {
                self.advance();
                let cond = self.parse_cond()?;
                self.expect(&Token::LBrace, "Expected '{' after when condition")?;
                let then_body = self.parse_block()?;
                let else_body = if self.match_token(&Token::Else) {
                    self.expect(&Token::LBrace, "Expected '{' after else")?;
                    self.parse_block()?
                } else {
                    Vec::new()
                };
                Ok(Stmt::When {
                    cond,
                    then_body,
                    else_body,
                })
            }
            _ => Err(self.error("Expected grain, speak, echo, or when")),
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>> {
        let mut stmts = Vec::new();
        while !self.check(&Token::RBrace) && !self.check(&Token::Eof) {
            stmts.push(self.parse_stmt()?);
        }
        self.expect(&Token::RBrace, "Expected '}' to close block")?;
        Ok(stmts)
    }

    fn parse_cond(&mut self) -> Result<Cond> {
        if let Token::Ident(name) = self.current_token().clone() {
            if self.peek_is(&Token::Dot) {
                self.advance();
                self.advance();
                let axis = self.expect_ident("Expected axis name after '.'")?;
                let op = match self.current_token() {
                    Token::Gt => CmpOp::Gt,
                    Token::Lt => CmpOp::Lt,
                    Token::Ge => CmpOp::Ge,
                    Token::Le => CmpOp::Le,
                    _ => return Err(self.error("Expected comparison after axis access")),
                };
                self.advance();
                let value = self.expect_number("Expected number in comparison")?;
                return Ok(Cond::Axis {
                    grain: name,
                    axis,
                    op,
                    value,
                });
            }
        }

        let left = self.parse_grain_expr()?;
        self.expect(&Token::Tilde, "Expected '~' in resonance condition")?;
        match self.current_token().clone() {
            Token::String(phrase) => {
                self.advance();
                Ok(Cond::ResonatePhrase {
                    grain: left,
                    phrase,
                })
            }
            _ => {
                let right = self.parse_grain_expr()?;
                Ok(Cond::ResonateGrains { left, right })
            }
        }
    }

    fn parse_grain_expr(&mut self) -> Result<GrainExpr> {
        let base = self.parse_grain_atom()?;
        let mut steps = Vec::new();
        while self.starts_step() {
            steps.push(self.parse_step()?);
        }
        if steps.is_empty() {
            Ok(base)
        } else {
            Ok(GrainExpr::Pipeline {
                base: Box::new(base),
                steps,
            })
        }
    }

    fn starts_step(&self) -> bool {
        matches!(
            self.current_token(),
            Token::Along | Token::Keeping | Token::Via | Token::Without | Token::With
        )
    }

    fn parse_step(&mut self) -> Result<Step> {
        match self.current_token() {
            Token::Along => {
                self.advance();
                let axis = self.expect_ident("Expected axis name after along")?;
                self.expect(&Token::Toward, "Expected 'toward' after axis")?;
                let toward = self.expect_number("Expected target after toward")?;
                let by = if self.match_token(&Token::By) {
                    Some(self.expect_number("Expected rate after by")?)
                } else {
                    None
                };
                Ok(Step::Along { axis, toward, by })
            }
            Token::Keeping => {
                self.advance();
                let mut names = vec![self.expect_ident("Expected axis name after keeping")?];
                while self.match_token(&Token::Comma) {
                    names.push(self.expect_ident("Expected axis name after comma")?);
                }
                Ok(Step::Keeping(names))
            }
            Token::Via => {
                self.advance();
                Ok(Step::Via(
                    self.expect_ident("Expected path name after via")?,
                ))
            }
            Token::Without => {
                self.advance();
                Ok(Step::Without(Box::new(self.parse_grain_atom()?)))
            }
            Token::With => {
                self.advance();
                self.expect(&Token::Shadow, "Expected 'shadow' after with")?;
                Ok(Step::WithShadow)
            }
            _ => Err(self.error("Expected a grain step")),
        }
    }

    fn parse_grain_atom(&mut self) -> Result<GrainExpr> {
        match self.current_token().clone() {
            Token::Feel => {
                self.advance();
                Ok(GrainExpr::Feel(
                    self.expect_string("Expected text after feel")?,
                ))
            }
            Token::Shadow => {
                self.advance();
                self.expect(&Token::Of, "Expected 'of' after shadow")?;
                Ok(GrainExpr::ShadowOf(Box::new(self.parse_grain_atom()?)))
            }
            Token::Blend => {
                self.advance();
                let left = self.parse_grain_atom()?;
                self.expect(&Token::With, "Expected 'with' after blend")?;
                let right = self.parse_grain_atom()?;
                self.expect(&Token::By, "Expected 'by' after blend")?;
                let by = self.expect_number("Expected blend amount")?;
                Ok(GrainExpr::Blend {
                    left: Box::new(left),
                    right: Box::new(right),
                    by,
                })
            }
            Token::Ident(name) => {
                self.advance();
                Ok(GrainExpr::Name(name))
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_grain_expr()?;
                self.expect(&Token::RParen, "Expected ')'")?;
                Ok(expr)
            }
            _ => Err(self.error("Expected a grain (feel, name, shadow, blend, or '(')")),
        }
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.current].token
    }

    fn peek_is(&self, token: &Token) -> bool {
        self.tokens
            .get(self.current + 1)
            .map(|located| located.token == *token)
            .unwrap_or(false)
    }

    fn check(&self, token: &Token) -> bool {
        self.current_token() == token
    }

    fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn advance(&mut self) {
        if !matches!(self.current_token(), Token::Eof) {
            self.current += 1;
        }
    }

    fn expect(&mut self, token: &Token, message: &str) -> Result<()> {
        if self.check(token) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(message))
        }
    }

    fn expect_ident(&mut self, message: &str) -> Result<String> {
        match self.current_token().clone() {
            Token::Ident(name) => {
                self.advance();
                Ok(name)
            }
            _ => Err(self.error(message)),
        }
    }

    fn expect_string(&mut self, message: &str) -> Result<String> {
        match self.current_token().clone() {
            Token::String(text) => {
                self.advance();
                Ok(text)
            }
            _ => Err(self.error(message)),
        }
    }

    fn expect_number(&mut self, message: &str) -> Result<f64> {
        match self.current_token().clone() {
            Token::Number(value) => {
                self.advance();
                Ok(value)
            }
            _ => Err(self.error(message)),
        }
    }

    fn error(&self, message: &str) -> anyhow::Error {
        let located = &self.tokens[self.current];
        anyhow!(
            "{message} (found {:?} at line {} column {})",
            located.token,
            located.line,
            located.column
        )
    }
}

pub fn parse(source: &str) -> Result<Program> {
    let tokens = crate::lexer::lex(source)?;
    Parser::new(tokens).parse_program()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_small_program() {
        let source = r#"
            axes {
              warmth: "ice" <-> "ember"
            }
            grain scene = feel "a quiet room"
            speak scene along warmth toward 0.9
        "#;
        let program = parse(source).unwrap();
        assert_eq!(program.items.len(), 3);
    }
}
