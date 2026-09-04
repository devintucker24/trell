// Palimpsest Parser

use crate::ast::*;
use crate::error::PalimpsestError;
use crate::lexer::{Token, TokenKind};
use crate::time::Duration;
use crate::types::Value;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    fn peek_kind(&self) -> &TokenKind {
        &self.peek().kind
    }

    #[allow(dead_code)]
    fn peek_next_kind(&self) -> &TokenKind {
        if self.pos + 1 < self.tokens.len() {
            &self.tokens[self.pos + 1].kind
        } else {
            &TokenKind::Eof
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek_kind() == &TokenKind::Eof
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.pos += 1;
        }
        &self.tokens[self.pos - 1]
    }

    fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(self.peek_kind()) == std::mem::discriminant(kind)
    }

    fn match_token(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: &TokenKind, msg: &str) -> Result<Token, PalimpsestError> {
        if self.check(kind) {
            Ok(self.advance().clone())
        } else {
            let tok = self.peek();
            Err(PalimpsestError::ParseError {
                line: tok.line,
                column: tok.column,
                message: format!("{}, found {:?}", msg, tok.kind),
            })
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, PalimpsestError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        Ok(Program { statements })
    }

    pub fn parse_statement(&mut self) -> Result<Stmt, PalimpsestError> {
        match self.peek_kind() {
            TokenKind::Authority => self.parse_authority_decl(),
            TokenKind::Scope => self.parse_scope(),
            TokenKind::Assert => self.parse_assert(),
            TokenKind::Episode => self.parse_episode(),
            TokenKind::Retract => self.parse_retract(),
            TokenKind::Let => self.parse_let(),
            TokenKind::Print => self.parse_print(),
            TokenKind::AssertEq => self.parse_assert_eq(),
            TokenKind::SetTime => self.parse_set_time(),
            TokenKind::AdvanceTime => self.parse_advance_time(),
            _ => {
                let expr = self.parse_expr()?;
                self.match_token(&TokenKind::Semicolon);
                Ok(Stmt::Expr(expr))
            }
        }
    }

    // authority Legal > Compliance > Policy > User > Unverified;
    fn parse_authority_decl(&mut self) -> Result<Stmt, PalimpsestError> {
        self.advance(); // consume 'authority'
        let mut tiers = Vec::new();

        loop {
            let tok = self.peek().clone();
            let name = match tok.kind {
                TokenKind::Ident(s) => {
                    self.advance();
                    s
                }
                _ => {
                    return Err(PalimpsestError::ParseError {
                        line: tok.line,
                        column: tok.column,
                        message: "Expected authority identifier".to_string(),
                    });
                }
            };
            tiers.push(name);

            if self.match_token(&TokenKind::Gt) {
                continue;
            } else {
                break;
            }
        }

        self.expect(&TokenKind::Semicolon, "Expected ';' after authority declaration")?;
        Ok(Stmt::AuthorityDecl(tiers))
    }

    // scope enterprise.acme { ... }
    fn parse_scope(&mut self) -> Result<Stmt, PalimpsestError> {
        self.advance(); // consume 'scope'
        let prefix = self.parse_path()?;
        self.expect(&TokenKind::LeftBrace, "Expected '{' to start scope block")?;

        let mut body = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            body.push(self.parse_statement()?);
        }

        self.expect(&TokenKind::RightBrace, "Expected '}' to end scope block")?;
        Ok(Stmt::Scope { prefix, body })
    }

    // assert user.location = "Berlin" @ authority(User), source("chat");
    fn parse_assert(&mut self) -> Result<Stmt, PalimpsestError> {
        self.advance(); // consume 'assert'
        let path = self.parse_path()?;

        self.expect(&TokenKind::Equals, "Expected '=' in assert statement")?;
        let value = self.parse_expr()?;

        let mut modifiers = AssertModifiers::default();

        if self.match_token(&TokenKind::AtSign) {
            loop {
                let tok = self.peek().clone();
                match tok.kind {
                    TokenKind::Authority => {
                        self.advance();
                        self.expect(&TokenKind::LeftParen, "Expected '(' after 'authority'")?;
                        let auth_tok = self.peek().clone();
                        let auth_name = match auth_tok.kind {
                            TokenKind::Ident(s) => {
                                self.advance();
                                s
                            }
                            _ => {
                                return Err(PalimpsestError::ParseError {
                                    line: auth_tok.line,
                                    column: auth_tok.column,
                                    message: "Expected authority identifier in authority(...)".to_string(),
                                });
                            }
                        };
                        self.expect(&TokenKind::RightParen, "Expected ')' after authority identifier")?;
                        modifiers.authority = Some(auth_name);
                    }
                    TokenKind::Source => {
                        self.advance();
                        self.expect(&TokenKind::LeftParen, "Expected '(' after 'source'")?;
                        let src_expr = self.parse_expr()?;
                        self.expect(&TokenKind::RightParen, "Expected ')' after source expression")?;
                        modifiers.source = Some(src_expr);
                    }
                    TokenKind::Verified => {
                        self.advance();
                        modifiers.verified = Some(true);
                    }
                    TokenKind::Unverified => {
                        self.advance();
                        modifiers.verified = Some(false);
                    }
                    TokenKind::At => {
                        self.advance();
                        self.expect(&TokenKind::LeftParen, "Expected '(' after 'at'")?;
                        let at_expr = self.parse_expr()?;
                        self.expect(&TokenKind::RightParen, "Expected ')' after at expression")?;
                        modifiers.at = Some(at_expr);
                    }
                    TokenKind::Ttl => {
                        self.advance();
                        self.expect(&TokenKind::LeftParen, "Expected '(' after 'ttl'")?;
                        let ttl_expr = self.parse_expr()?;
                        self.expect(&TokenKind::RightParen, "Expected ')' after ttl expression")?;
                        modifiers.ttl = Some(ttl_expr);
                    }
                    TokenKind::ValidUntil => {
                        self.advance();
                        self.expect(&TokenKind::LeftParen, "Expected '(' after 'valid_until'")?;
                        let vu_expr = self.parse_expr()?;
                        self.expect(&TokenKind::RightParen, "Expected ')' after valid_until expression")?;
                        modifiers.valid_until = Some(vu_expr);
                    }
                    TokenKind::GroundedIn => {
                        self.advance();
                        self.expect(&TokenKind::LeftParen, "Expected '(' after 'grounded_in'")?;
                        let ep_tok = self.peek().clone();
                        let ep_id = match ep_tok.kind {
                            TokenKind::Ident(s) | TokenKind::StringLit(s) => {
                                self.advance();
                                s
                            }
                            _ => {
                                return Err(PalimpsestError::ParseError {
                                    line: ep_tok.line,
                                    column: ep_tok.column,
                                    message: "Expected episode ID in grounded_in(...)".to_string(),
                                });
                            }
                        };
                        self.expect(&TokenKind::RightParen, "Expected ')' after episode ID")?;
                        modifiers.grounded_in = Some(ep_id);
                    }
                    _ => {
                        return Err(PalimpsestError::ParseError {
                            line: tok.line,
                            column: tok.column,
                            message: format!("Unknown assertion modifier: {:?}", tok.kind),
                        });
                    }
                }

                if self.match_token(&TokenKind::Comma) {
                    continue;
                } else {
                    break;
                }
            }
        }

        self.expect(&TokenKind::Semicolon, "Expected ';' after assert statement")?;
        Ok(Stmt::Assert { path, value, modifiers })
    }

    // episode ident { at: ..., actors: [...], context: { ... }, summary: ... }
    fn parse_episode(&mut self) -> Result<Stmt, PalimpsestError> {
        self.advance(); // consume 'episode'
        let id_tok = self.peek().clone();
        let id = match id_tok.kind {
            TokenKind::Ident(s) | TokenKind::StringLit(s) => {
                self.advance();
                s
            }
            _ => {
                return Err(PalimpsestError::ParseError {
                    line: id_tok.line,
                    column: id_tok.column,
                    message: "Expected episode identifier".to_string(),
                });
            }
        };

        self.expect(&TokenKind::LeftBrace, "Expected '{' to start episode block")?;

        let mut at_expr = None;
        let mut actors_vec = Vec::new();
        let mut context_vec = Vec::new();
        let mut summary_expr = None;

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            let field_tok = self.peek().clone();
            let field_name = match field_tok.kind {
                TokenKind::At => {
                    self.advance();
                    "at"
                }
                TokenKind::Actors => {
                    self.advance();
                    "actors"
                }
                TokenKind::Context => {
                    self.advance();
                    "context"
                }
                TokenKind::Summary => {
                    self.advance();
                    "summary"
                }
                TokenKind::Ident(ref s) => {
                    let s_clone = s.as_str();
                    match s_clone {
                        "at" | "actors" | "context" | "summary" => {
                            self.advance();
                            s_clone
                        }
                        _ => {
                            return Err(PalimpsestError::ParseError {
                                line: field_tok.line,
                                column: field_tok.column,
                                message: format!("Unknown episode field: {}", s),
                            });
                        }
                    }
                }
                _ => {
                    return Err(PalimpsestError::ParseError {
                        line: field_tok.line,
                        column: field_tok.column,
                        message: "Expected episode field (at, actors, context, summary)".to_string(),
                    });
                }
            };

            self.expect(&TokenKind::Colon, "Expected ':' after episode field name")?;

            match field_name {
                "at" => {
                    at_expr = Some(self.parse_expr()?);
                }
                "actors" => {
                    self.expect(&TokenKind::LeftBracket, "Expected '[' for actors list")?;
                    while !self.check(&TokenKind::RightBracket) && !self.is_at_end() {
                        actors_vec.push(self.parse_expr()?);
                        if !self.match_token(&TokenKind::Comma) {
                            break;
                        }
                    }
                    self.expect(&TokenKind::RightBracket, "Expected ']' after actors list")?;
                }
                "context" => {
                    self.expect(&TokenKind::LeftBrace, "Expected '{' for context record")?;
                    while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                        let k_tok = self.peek().clone();
                        let k = match k_tok.kind {
                            TokenKind::Ident(s) | TokenKind::StringLit(s) => {
                                self.advance();
                                s
                            }
                            _ => {
                                return Err(PalimpsestError::ParseError {
                                    line: k_tok.line,
                                    column: k_tok.column,
                                    message: "Expected key in context record".to_string(),
                                });
                            }
                        };
                        self.expect(&TokenKind::Colon, "Expected ':' after context key")?;
                        let v = self.parse_expr()?;
                        context_vec.push((k, v));
                        if !self.match_token(&TokenKind::Comma) {
                            break;
                        }
                    }
                    self.expect(&TokenKind::RightBrace, "Expected '}' after context record")?;
                }
                "summary" => {
                    summary_expr = Some(self.parse_expr()?);
                }
                _ => unreachable!(),
            }

            self.match_token(&TokenKind::Comma);
            self.match_token(&TokenKind::Semicolon);
        }

        self.expect(&TokenKind::RightBrace, "Expected '}' to end episode block")?;

        let at = at_expr.ok_or_else(|| PalimpsestError::ParseError {
            line: id_tok.line,
            column: id_tok.column,
            message: "Episode missing required field 'at'".to_string(),
        })?;

        let summary = summary_expr.unwrap_or_else(|| Expr::Literal(Value::String("".to_string())));

        Ok(Stmt::Episode {
            id,
            at,
            actors: actors_vec,
            context: context_vec,
            summary,
        })
    }

    // retract source "xyz"; / retract belief a.b; / retract episode ident;
    fn parse_retract(&mut self) -> Result<Stmt, PalimpsestError> {
        self.advance(); // consume 'retract'
        let tok = self.peek().clone();
        match tok.kind {
            TokenKind::Source => {
                self.advance();
                let src_expr = self.parse_expr()?;
                self.expect(&TokenKind::Semicolon, "Expected ';' after retract source")?;
                Ok(Stmt::RetractSource(src_expr))
            }
            TokenKind::Belief => {
                self.advance();
                let path = self.parse_path()?;
                self.expect(&TokenKind::Semicolon, "Expected ';' after retract belief")?;
                Ok(Stmt::RetractBelief(path))
            }
            TokenKind::Episode => {
                self.advance();
                let ep_tok = self.peek().clone();
                let ep_id = match ep_tok.kind {
                    TokenKind::Ident(s) | TokenKind::StringLit(s) => {
                        self.advance();
                        s
                    }
                    _ => {
                        return Err(PalimpsestError::ParseError {
                            line: ep_tok.line,
                            column: ep_tok.column,
                            message: "Expected episode ID after retract episode".to_string(),
                        });
                    }
                };
                self.expect(&TokenKind::Semicolon, "Expected ';' after retract episode")?;
                Ok(Stmt::RetractEpisode(ep_id))
            }
            _ => Err(PalimpsestError::ParseError {
                line: tok.line,
                column: tok.column,
                message: "Expected 'source', 'belief', or 'episode' after 'retract'".to_string(),
            }),
        }
    }

    fn parse_let(&mut self) -> Result<Stmt, PalimpsestError> {
        self.advance(); // consume 'let'
        let id_tok = self.peek().clone();
        let name = match id_tok.kind {
            TokenKind::Ident(s) => {
                self.advance();
                s
            }
            _ => {
                return Err(PalimpsestError::ParseError {
                    line: id_tok.line,
                    column: id_tok.column,
                    message: "Expected variable name after 'let'".to_string(),
                });
            }
        };

        self.expect(&TokenKind::Equals, "Expected '=' in let binding")?;
        let expr = self.parse_expr()?;
        self.expect(&TokenKind::Semicolon, "Expected ';' after let statement")?;
        Ok(Stmt::Let { name, expr })
    }

    fn parse_print(&mut self) -> Result<Stmt, PalimpsestError> {
        self.advance(); // consume 'print'
        let expr = self.parse_expr()?;
        self.expect(&TokenKind::Semicolon, "Expected ';' after print statement")?;
        Ok(Stmt::Print(expr))
    }

    fn parse_assert_eq(&mut self) -> Result<Stmt, PalimpsestError> {
        self.advance(); // consume 'assert_eq'
        let left = self.parse_expr()?;
        self.expect(&TokenKind::Comma, "Expected ',' between assert_eq expressions")?;
        let right = self.parse_expr()?;
        self.expect(&TokenKind::Semicolon, "Expected ';' after assert_eq statement")?;
        Ok(Stmt::AssertEq { left, right })
    }

    fn parse_set_time(&mut self) -> Result<Stmt, PalimpsestError> {
        self.advance(); // consume 'set_time'
        let expr = self.parse_expr()?;
        self.expect(&TokenKind::Semicolon, "Expected ';' after set_time statement")?;
        Ok(Stmt::SetTime(expr))
    }

    fn parse_advance_time(&mut self) -> Result<Stmt, PalimpsestError> {
        self.advance(); // consume 'advance_time'
        let expr = self.parse_expr()?;
        self.expect(&TokenKind::Semicolon, "Expected ';' after advance_time statement")?;
        Ok(Stmt::AdvanceTime(expr))
    }

    // Helper: parses dotted path, e.g. user.alice.location
    fn parse_path(&mut self) -> Result<Vec<String>, PalimpsestError> {
        let mut segments = Vec::new();
        loop {
            let tok = self.peek().clone();
            let seg = match tok.kind {
                TokenKind::Ident(s) => {
                    self.advance();
                    s
                }
                _ => {
                    return Err(PalimpsestError::ParseError {
                        line: tok.line,
                        column: tok.column,
                        message: "Expected identifier in path".to_string(),
                    });
                }
            };
            segments.push(seg);

            if self.match_token(&TokenKind::Dot) {
                continue;
            } else {
                break;
            }
        }
        Ok(segments)
    }

    // Expression parser using precedence climbing
    pub fn parse_expr(&mut self) -> Result<Expr, PalimpsestError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, PalimpsestError> {
        let mut expr = self.parse_and()?;
        while self.match_token(&TokenKind::PipePipe) {
            let right = self.parse_and()?;
            expr = Expr::BinaryOp {
                op: BinOp::Or,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expr, PalimpsestError> {
        let mut expr = self.parse_equality()?;
        while self.match_token(&TokenKind::AmpAmp) {
            let right = self.parse_equality()?;
            expr = Expr::BinaryOp {
                op: BinOp::And,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr, PalimpsestError> {
        let mut expr = self.parse_comparison()?;
        while let Some(op) = match self.peek_kind() {
            TokenKind::EqEq => Some(BinOp::Eq),
            TokenKind::BangEq => Some(BinOp::NotEq),
            _ => None,
        } {
            self.advance();
            let right = self.parse_comparison()?;
            expr = Expr::BinaryOp {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, PalimpsestError> {
        let mut expr = self.parse_term()?;
        while let Some(op) = match self.peek_kind() {
            TokenKind::Lt => Some(BinOp::Lt),
            TokenKind::LtEq => Some(BinOp::LtEq),
            TokenKind::Gt => Some(BinOp::Gt),
            TokenKind::GtEq => Some(BinOp::GtEq),
            _ => None,
        } {
            self.advance();
            let right = self.parse_term()?;
            expr = Expr::BinaryOp {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, PalimpsestError> {
        let mut expr = self.parse_factor()?;
        while let Some(op) = match self.peek_kind() {
            TokenKind::Plus => Some(BinOp::Add),
            TokenKind::Minus => Some(BinOp::Sub),
            _ => None,
        } {
            self.advance();
            let right = self.parse_factor()?;
            expr = Expr::BinaryOp {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, PalimpsestError> {
        let mut expr = self.parse_unary()?;
        while let Some(op) = match self.peek_kind() {
            TokenKind::Star => Some(BinOp::Mul),
            TokenKind::Slash => Some(BinOp::Div),
            _ => None,
        } {
            self.advance();
            let right = self.parse_unary()?;
            expr = Expr::BinaryOp {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, PalimpsestError> {
        if self.match_token(&TokenKind::Bang) {
            let expr = self.parse_unary()?;
            return Ok(Expr::UnaryOp {
                op: UnOp::Not,
                expr: Box::new(expr),
            });
        }
        if self.match_token(&TokenKind::Minus) {
            let expr = self.parse_unary()?;
            return Ok(Expr::UnaryOp {
                op: UnOp::Neg,
                expr: Box::new(expr),
            });
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expr, PalimpsestError> {
        let mut expr = self.parse_primary()?;

        while self.match_token(&TokenKind::Dot) {
            let tok = self.peek().clone();
            let field = match tok.kind {
                TokenKind::Ident(s) => {
                    self.advance();
                    s
                }
                _ => {
                    return Err(PalimpsestError::ParseError {
                        line: tok.line,
                        column: tok.column,
                        message: "Expected field identifier after '.'".to_string(),
                    });
                }
            };
            expr = Expr::FieldAccess {
                expr: Box::new(expr),
                field,
            };
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, PalimpsestError> {
        let tok = self.peek().clone();
        match &tok.kind {
            TokenKind::StringLit(s) => {
                let val = s.clone();
                self.advance();
                Ok(Expr::Literal(Value::String(val)))
            }
            TokenKind::IntLit(n) => {
                let val = *n;
                self.advance();
                Ok(Expr::Literal(Value::Int(val)))
            }
            TokenKind::FloatLit(f) => {
                let val = *f;
                self.advance();
                Ok(Expr::Literal(Value::Float(val)))
            }
            TokenKind::DurationLit(d_str) => {
                let d = Duration::parse_str(d_str).map_err(|e| PalimpsestError::ParseError {
                    line: tok.line,
                    column: tok.column,
                    message: e,
                })?;
                self.advance();
                Ok(Expr::Literal(Value::Duration(d)))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr::Literal(Value::Bool(true)))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expr::Literal(Value::Bool(false)))
            }
            TokenKind::Null => {
                self.advance();
                Ok(Expr::Literal(Value::Null))
            }
            TokenKind::Conflicts => {
                self.advance();
                Ok(Expr::Conflicts)
            }
            TokenKind::Episodes => {
                self.advance();
                Ok(Expr::Episodes)
            }
            TokenKind::Recall => self.parse_recall_expr(),
            TokenKind::History => {
                self.advance();
                let path = self.parse_path()?;
                Ok(Expr::History(path))
            }
            TokenKind::Audit => {
                self.advance();
                let path = self.parse_path()?;
                Ok(Expr::Audit(path))
            }
            TokenKind::LeftParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&TokenKind::RightParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            TokenKind::LeftBracket => {
                self.advance();
                let mut items = Vec::new();
                while !self.check(&TokenKind::RightBracket) && !self.is_at_end() {
                    items.push(self.parse_expr()?);
                    if !self.match_token(&TokenKind::Comma) {
                        break;
                    }
                }
                self.expect(&TokenKind::RightBracket, "Expected ']' after list")?;
                Ok(Expr::List(items))
            }
            TokenKind::LeftBrace => {
                self.advance();
                let mut entries = Vec::new();
                while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                    let k_tok = self.peek().clone();
                    let k = match k_tok.kind {
                        TokenKind::Ident(s) | TokenKind::StringLit(s) => {
                            self.advance();
                            s
                        }
                        _ => {
                            return Err(PalimpsestError::ParseError {
                                line: k_tok.line,
                                column: k_tok.column,
                                message: "Expected key in record literal".to_string(),
                            });
                        }
                    };
                    self.expect(&TokenKind::Colon, "Expected ':' after record key")?;
                    let v = self.parse_expr()?;
                    entries.push((k, v));
                    if !self.match_token(&TokenKind::Comma) {
                        break;
                    }
                }
                self.expect(&TokenKind::RightBrace, "Expected '}' after record literal")?;
                Ok(Expr::Record(entries))
            }
            TokenKind::Ident(name) => {
                let first_name = name.clone();
                self.advance();

                // If followed by dot, it's a dotted path!
                if self.check(&TokenKind::Dot) {
                    let mut path = vec![first_name];
                    while self.match_token(&TokenKind::Dot) {
                        let next_tok = self.peek().clone();
                        match next_tok.kind {
                            TokenKind::Ident(s) => {
                                self.advance();
                                path.push(s);
                            }
                            _ => {
                                return Err(PalimpsestError::ParseError {
                                    line: next_tok.line,
                                    column: next_tok.column,
                                    message: "Expected identifier in path after '.'".to_string(),
                                });
                            }
                        }
                    }
                    Ok(Expr::Path(path))
                } else {
                    // Single identifier: could be variable or 1-element path
                    Ok(Expr::Variable(first_name))
                }
            }
            _ => Err(PalimpsestError::ParseError {
                line: tok.line,
                column: tok.column,
                message: format!("Unexpected token in expression: {:?}", tok.kind),
            }),
        }
    }

    // recall [as_of(expr)] [fresh] [verified] [min_authority(Ident)] path
    fn parse_recall_expr(&mut self) -> Result<Expr, PalimpsestError> {
        self.advance(); // consume 'recall'

        let mut as_of = None;
        let mut fresh = false;
        let mut verified_only = false;
        let mut min_authority = None;

        // Parse optional recall modifiers
        loop {
            if self.check(&TokenKind::AsOf) {
                self.advance();
                self.expect(&TokenKind::LeftParen, "Expected '(' after as_of")?;
                let expr = self.parse_expr()?;
                self.expect(&TokenKind::RightParen, "Expected ')' after as_of expression")?;
                as_of = Some(Box::new(expr));
                continue;
            }
            if self.check(&TokenKind::Fresh) {
                self.advance();
                fresh = true;
                continue;
            }
            if self.check(&TokenKind::Verified) {
                self.advance();
                verified_only = true;
                continue;
            }
            if self.check(&TokenKind::MinAuthority) {
                self.advance();
                self.expect(&TokenKind::LeftParen, "Expected '(' after min_authority")?;
                let auth_tok = self.peek().clone();
                let auth_name = match auth_tok.kind {
                    TokenKind::Ident(s) => {
                        self.advance();
                        s
                    }
                    _ => {
                        return Err(PalimpsestError::ParseError {
                            line: auth_tok.line,
                            column: auth_tok.column,
                            message: "Expected authority identifier in min_authority(...)".to_string(),
                        });
                    }
                };
                self.expect(&TokenKind::RightParen, "Expected ')' after authority identifier")?;
                min_authority = Some(auth_name);
                continue;
            }
            break;
        }

        let path = self.parse_path()?;
        Ok(Expr::Recall {
            path,
            as_of,
            fresh,
            verified_only,
            min_authority,
        })
    }
}
