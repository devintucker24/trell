// Palimpsest Parser
//
// A recursive-descent parser over a token stream in which every bare word is
// undifferentiated. Keywords are recognised positionally, which is what lets a
// belief be named `summary` or `context` without escaping.

use crate::ast::*;
use crate::error::PalimpsestError;
use crate::lexer::{duration_unit_secs, Token, TokenKind};
use crate::time::{Duration, Timestamp};
use crate::types::Value;

/// Words that introduce a statement or query and therefore cannot begin a
/// belief path.
const RESERVED: &[&str] = &[
    "trust", "about", "when", "forget", "let", "now", "later", "show", "expect", "what", "why",
    "check", "conflicts", "episodes", "true", "false", "not", "nothing", "and", "or",
];

pub fn is_reserved(word: &str) -> bool {
    RESERVED.contains(&word)
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    // ---- token helpers -------------------------------------------------

    fn peek(&self) -> &Token {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    fn kind(&self) -> &TokenKind {
        &self.peek().kind
    }

    fn line(&self) -> usize {
        self.peek().line
    }

    fn at_end(&self) -> bool {
        matches!(self.kind(), TokenKind::Eof)
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens[self.pos.min(self.tokens.len() - 1)].clone();
        if !self.at_end() {
            self.pos += 1;
        }
        tok
    }

    fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(self.kind()) == std::mem::discriminant(kind)
    }

    fn eat(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: &TokenKind, what: &str) -> Result<Token, PalimpsestError> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(self.error(what))
        }
    }

    fn error(&self, expected: &str) -> PalimpsestError {
        let tok = self.peek();
        PalimpsestError::ParseError {
            line: tok.line,
            column: tok.column,
            message: format!("Expected {}, found {}", expected, tok.kind.describe()),
        }
    }

    /// Returns the current token's text if it is a word.
    fn word(&self) -> Option<&str> {
        match self.kind() {
            TokenKind::Word(w) => Some(w.as_str()),
            _ => None,
        }
    }

    fn word_is(&self, expected: &str) -> bool {
        self.word().map(|w| w.eq_ignore_ascii_case(expected)).unwrap_or(false)
    }

    fn eat_word(&mut self, expected: &str) -> bool {
        if self.word_is(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect_word(&mut self, expected: &str) -> Result<(), PalimpsestError> {
        if self.eat_word(expected) {
            Ok(())
        } else {
            Err(self.error(&format!("`{}`", expected)))
        }
    }

    fn take_word(&mut self, what: &str) -> Result<String, PalimpsestError> {
        match self.kind().clone() {
            TokenKind::Word(w) => {
                self.advance();
                Ok(w)
            }
            _ => Err(self.error(what)),
        }
    }

    /// A name that may be written bare or quoted, such as a source identifier.
    fn take_name(&mut self, what: &str) -> Result<String, PalimpsestError> {
        match self.kind().clone() {
            TokenKind::Word(w) => {
                self.advance();
                Ok(w)
            }
            TokenKind::Str(s) => {
                self.advance();
                Ok(s)
            }
            _ => Err(self.error(what)),
        }
    }

    fn skip_separators(&mut self) {
        while matches!(self.kind(), TokenKind::Newline | TokenKind::Semi) {
            self.advance();
        }
    }

    /// Consumes the end of a statement, tolerating a trailing `?` or `;`.
    fn end_statement(&mut self) -> Result<(), PalimpsestError> {
        self.eat(&TokenKind::Question);
        if matches!(self.kind(), TokenKind::Newline | TokenKind::Semi) {
            self.advance();
            return Ok(());
        }
        if matches!(self.kind(), TokenKind::Eof | TokenKind::Dedent | TokenKind::RBrace) {
            return Ok(());
        }
        Err(self.error("the end of the line"))
    }

    // ---- program -------------------------------------------------------

    pub fn parse_program(&mut self) -> Result<Program, PalimpsestError> {
        let mut statements = Vec::new();
        self.skip_separators();
        while !self.at_end() {
            statements.push(self.parse_statement()?);
            self.skip_separators();
        }
        Ok(Program { statements })
    }

    /// Parses either a `:`-introduced indented block or a `{ }` block.
    fn parse_block(&mut self) -> Result<Vec<Stmt>, PalimpsestError> {
        let mut body = Vec::new();

        if self.eat(&TokenKind::Colon) {
            self.expect(&TokenKind::Newline, "a line break after `:`")?;
            self.expect(&TokenKind::Indent, "an indented block after `:`")?;
            self.skip_separators();
            while !self.check(&TokenKind::Dedent) && !self.at_end() {
                body.push(self.parse_statement()?);
                self.skip_separators();
            }
            self.expect(&TokenKind::Dedent, "the end of the indented block")?;
            return Ok(body);
        }

        if self.eat(&TokenKind::LBrace) {
            self.skip_separators();
            while !self.check(&TokenKind::RBrace) && !self.at_end() {
                body.push(self.parse_statement()?);
                self.skip_separators();
            }
            self.expect(&TokenKind::RBrace, "`}`")?;
            return Ok(body);
        }

        Err(self.error("`:` followed by an indented block"))
    }

    fn parse_statement(&mut self) -> Result<Stmt, PalimpsestError> {
        if self.check(&TokenKind::Indent) {
            let tok = self.peek();
            return Err(PalimpsestError::ParseError {
                line: tok.line,
                column: tok.column,
                message: "This line is indented but the line above it does not open a block. Blocks are opened by `about <name>:` or `when <name>:`.".into(),
            });
        }

        if let Some(w) = self.word() {
            let w = w.to_ascii_lowercase();
            match w.as_str() {
                "trust" => return self.parse_trust(),
                "about" => return self.parse_about(),
                "when" => return self.parse_episode(),
                "forget" => return self.parse_forget(),
                "let" => return self.parse_let(),
                "now" => return self.parse_now(),
                "later" => return self.parse_later(),
                "show" => return self.parse_show(),
                "expect" => return self.parse_expect(),
                _ => {}
            }
        }

        // Anything that is not a reserved opener and parses as `path is value`
        // is a fact; otherwise it is a query printed for its value.
        if !self.word().map(is_reserved).unwrap_or(true) {
            let save = self.pos;
            if let Ok(path) = self.parse_path() {
                if self.word_is("is") || self.word_is("are") || self.check(&TokenKind::Eq) {
                    return self.parse_fact(path);
                }
            }
            self.pos = save;
        }

        let expr = self.parse_expr()?;
        self.end_statement()?;
        Ok(Stmt::Show(expr))
    }

    // ---- statements ----------------------------------------------------

    /// `trust legal above policy above user above rumor`
    fn parse_trust(&mut self) -> Result<Stmt, PalimpsestError> {
        self.advance();
        let mut tiers = vec![self.take_word("an authority name")?];
        while self.eat_word("above") {
            tiers.push(self.take_word("an authority name")?);
        }
        self.end_statement()?;
        Ok(Stmt::Trust(tiers))
    }

    /// `about acme.alice:` followed by a block
    fn parse_about(&mut self) -> Result<Stmt, PalimpsestError> {
        self.advance();
        let prefix = self.parse_path()?;
        let body = self.parse_block()?;
        Ok(Stmt::About { prefix, body })
    }

    /// `when db_outage:` followed by episode fields
    fn parse_episode(&mut self) -> Result<Stmt, PalimpsestError> {
        self.advance();
        let id = self.take_name("an episode name")?;

        self.expect(&TokenKind::Colon, "`:` after the episode name")?;
        self.expect(&TokenKind::Newline, "a line break after `:`")?;
        self.expect(&TokenKind::Indent, "an indented block of episode details")?;
        self.skip_separators();

        let mut happened = None;
        let mut involved = Vec::new();
        let mut details = Vec::new();
        let mut summary = None;

        while !self.check(&TokenKind::Dedent) && !self.at_end() {
            let field = self.take_word("an episode field")?.to_ascii_lowercase();
            match field.as_str() {
                "happened" => {
                    self.eat_word("on");
                    happened = Some(self.parse_time_expr()?);
                }
                "involved" => loop {
                    let who = self.take_name("a participant")?;
                    involved.push(Expr::Literal(Value::String(who)));
                    if !self.eat(&TokenKind::Comma) {
                        break;
                    }
                },
                "details" => loop {
                    let key = self.take_word("a detail name")?;
                    self.expect_word("is")?;
                    let val = self.parse_expr()?;
                    details.push((key, val));
                    if !self.eat(&TokenKind::Comma) {
                        break;
                    }
                },
                "summary" => {
                    summary = Some(self.parse_expr()?);
                }
                other => {
                    return Err(PalimpsestError::ParseError {
                        line: self.line(),
                        column: 1,
                        message: format!(
                            "`{}` is not an episode field; use happened, involved, details or summary",
                            other
                        ),
                    })
                }
            }
            self.skip_separators();
        }

        self.expect(&TokenKind::Dedent, "the end of the episode block")?;

        Ok(Stmt::Episode {
            id,
            happened,
            involved,
            details,
            summary,
        })
    }

    /// `forget everything from X` / `forget when X` / `forget a.path`
    fn parse_forget(&mut self) -> Result<Stmt, PalimpsestError> {
        self.advance();

        if self.eat_word("everything") {
            self.expect_word("from")?;
            let src = self.take_name("a source name")?;
            self.end_statement()?;
            return Ok(Stmt::ForgetSource(Expr::Literal(Value::String(src))));
        }

        if self.eat_word("from") {
            let src = self.take_name("a source name")?;
            self.end_statement()?;
            return Ok(Stmt::ForgetSource(Expr::Literal(Value::String(src))));
        }

        if self.eat_word("when") {
            let id = self.take_name("an episode name")?;
            self.end_statement()?;
            return Ok(Stmt::ForgetEpisode(id));
        }

        let path = self.parse_path()?;
        self.end_statement()?;
        Ok(Stmt::ForgetPath(path))
    }

    fn parse_let(&mut self) -> Result<Stmt, PalimpsestError> {
        self.advance();
        let name = self.take_word("a name to bind")?;
        if !self.eat(&TokenKind::Eq) && !self.eat_word("is") {
            return Err(self.error("`=` or `is`"));
        }
        let expr = self.parse_expr()?;
        self.end_statement()?;
        Ok(Stmt::Let { name, expr })
    }

    fn parse_now(&mut self) -> Result<Stmt, PalimpsestError> {
        self.advance();
        self.expect_word("is")?;
        let expr = self.parse_time_expr()?;
        self.end_statement()?;
        Ok(Stmt::NowIs(expr))
    }

    fn parse_later(&mut self) -> Result<Stmt, PalimpsestError> {
        self.advance();
        self.expect_word("by")?;
        let expr = self.parse_duration_expr()?;
        self.end_statement()?;
        Ok(Stmt::LaterBy(expr))
    }

    fn parse_show(&mut self) -> Result<Stmt, PalimpsestError> {
        self.advance();
        let expr = self.parse_expr()?;
        self.end_statement()?;
        Ok(Stmt::Show(expr))
    }

    fn parse_expect(&mut self) -> Result<Stmt, PalimpsestError> {
        let line = self.line();
        self.advance();
        let left = self.parse_expr()?;
        if !self.eat_word("is") && !self.eat(&TokenKind::EqEq) && !self.eat(&TokenKind::Eq) {
            return Err(self.error("`is` between the two values being compared"));
        }
        let right = self.parse_expr()?;
        self.end_statement()?;
        Ok(Stmt::Expect { left, right, line })
    }

    /// `alice.city is "Berlin" from relocation_ticket on 2026-08-15`
    fn parse_fact(&mut self, path: Vec<String>) -> Result<Stmt, PalimpsestError> {
        let line = self.line();
        if !self.eat_word("is") && !self.eat_word("are") && !self.eat(&TokenKind::Eq) {
            return Err(self.error("`is`"));
        }
        let value = self.parse_expr()?;
        let facets = self.parse_facets()?;
        self.end_statement()?;
        Ok(Stmt::Fact {
            path,
            value,
            facets,
            line,
        })
    }

    /// The trailing prepositional phrases on a fact, in any order.
    fn parse_facets(&mut self) -> Result<Facets, PalimpsestError> {
        let mut f = Facets::default();

        loop {
            let Some(w) = self.word().map(|w| w.to_ascii_lowercase()) else {
                break;
            };

            match w.as_str() {
                "from" => {
                    self.advance();
                    let src = self.take_name("a source name")?;
                    f.source = Some(Expr::Literal(Value::String(src)));
                }
                "as" => {
                    self.advance();
                    f.authority = Some(self.take_word("an authority name")?);
                }
                "on" | "since" | "at" => {
                    self.advance();
                    f.asserted_at = Some(self.parse_time_expr()?);
                }
                "for" | "lasting" => {
                    self.advance();
                    f.ttl = Some(self.parse_duration_expr()?);
                }
                "until" => {
                    self.advance();
                    f.until = Some(self.parse_time_expr()?);
                }
                "because" | "during" => {
                    self.advance();
                    f.because = Some(self.take_name("an episode name")?);
                }
                "unverified" => {
                    self.advance();
                    f.verified = Some(false);
                }
                "verified" => {
                    self.advance();
                    f.verified = Some(true);
                }
                _ => break,
            }

            self.eat(&TokenKind::Comma);
        }

        Ok(f)
    }

    // ---- shared fragments ----------------------------------------------

    fn parse_path(&mut self) -> Result<Vec<String>, PalimpsestError> {
        let mut segments = vec![self.take_word("a belief name")?];
        while self.check(&TokenKind::Dot) {
            self.advance();
            segments.push(self.take_word("a name after `.`")?);
        }
        Ok(segments)
    }

    fn parse_time_expr(&mut self) -> Result<Expr, PalimpsestError> {
        match self.kind().clone() {
            TokenKind::Date(text) => {
                self.advance();
                let ts = Timestamp::parse_iso(&text)
                    .map_err(|e| PalimpsestError::ParseError {
                        line: self.line(),
                        column: 1,
                        message: e,
                    })?;
                Ok(Expr::Literal(Value::Timestamp(ts)))
            }
            TokenKind::Str(_) | TokenKind::Int(_) => self.parse_expr(),
            _ => Err(self.error("a date such as 2026-08-15")),
        }
    }

    fn parse_duration_expr(&mut self) -> Result<Expr, PalimpsestError> {
        match self.kind().clone() {
            TokenKind::Dur(secs) => {
                self.advance();
                Ok(Expr::Literal(Value::Duration(Duration::from_secs(secs))))
            }
            // A spaced form such as `30 days`.
            TokenKind::Int(n) => {
                let save = self.pos;
                self.advance();
                if let Some(unit) = self.word().map(|w| w.to_string()) {
                    if let Some(mult) = duration_unit_secs(&unit) {
                        self.advance();
                        let secs = (n.max(0) as u64) * mult;
                        return Ok(Expr::Literal(Value::Duration(Duration::from_secs(secs))));
                    }
                }
                self.pos = save;
                Err(self.error("a length of time such as `30 days` or `90d`"))
            }
            _ => Err(self.error("a length of time such as `30 days` or `90d`")),
        }
    }

    // ---- expressions ----------------------------------------------------

    pub fn parse_expr(&mut self) -> Result<Expr, PalimpsestError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, PalimpsestError> {
        let mut left = self.parse_and()?;
        loop {
            if self.check(&TokenKind::Pipe2) || self.word_is("or") {
                self.advance();
                let right = self.parse_and()?;
                left = Expr::Binary {
                    op: BinOp::Or,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                return Ok(left);
            }
        }
    }

    fn parse_and(&mut self) -> Result<Expr, PalimpsestError> {
        let mut left = self.parse_equality()?;
        loop {
            if self.check(&TokenKind::Amp2) || self.word_is("and") {
                self.advance();
                let right = self.parse_equality()?;
                left = Expr::Binary {
                    op: BinOp::And,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                return Ok(left);
            }
        }
    }

    fn parse_equality(&mut self) -> Result<Expr, PalimpsestError> {
        let mut left = self.parse_comparison()?;
        loop {
            let op = match self.kind() {
                TokenKind::EqEq => BinOp::Eq,
                TokenKind::BangEq => BinOp::NotEq,
                _ => return Ok(left),
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
    }

    fn parse_comparison(&mut self) -> Result<Expr, PalimpsestError> {
        let mut left = self.parse_term()?;
        loop {
            let op = match self.kind() {
                TokenKind::Lt => BinOp::Lt,
                TokenKind::LtEq => BinOp::LtEq,
                TokenKind::Gt => BinOp::Gt,
                TokenKind::GtEq => BinOp::GtEq,
                _ => return Ok(left),
            };
            self.advance();
            let right = self.parse_term()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
    }

    fn parse_term(&mut self) -> Result<Expr, PalimpsestError> {
        let mut left = self.parse_factor()?;
        loop {
            let op = match self.kind() {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => return Ok(left),
            };
            self.advance();
            let right = self.parse_factor()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
    }

    fn parse_factor(&mut self) -> Result<Expr, PalimpsestError> {
        let mut left = self.parse_unary()?;
        loop {
            let op = match self.kind() {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                _ => return Ok(left),
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, PalimpsestError> {
        if self.check(&TokenKind::Bang) || self.word_is("not") {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnOp::Not,
                expr: Box::new(expr),
            });
        }
        if self.check(&TokenKind::Minus) {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnOp::Neg,
                expr: Box::new(expr),
            });
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, PalimpsestError> {
        match self.kind().clone() {
            TokenKind::Str(s) => {
                self.advance();
                Ok(Expr::Literal(Value::String(s)))
            }
            TokenKind::Int(n) => {
                self.advance();
                // `30 days` in value position is still a duration.
                if let Some(unit) = self.word().map(|w| w.to_string()) {
                    if let Some(mult) = duration_unit_secs(&unit) {
                        self.advance();
                        let secs = (n.max(0) as u64) * mult;
                        return Ok(Expr::Literal(Value::Duration(Duration::from_secs(secs))));
                    }
                }
                Ok(Expr::Literal(Value::Int(n)))
            }
            TokenKind::Float(f) => {
                self.advance();
                Ok(Expr::Literal(Value::Float(f)))
            }
            TokenKind::Dur(secs) => {
                self.advance();
                Ok(Expr::Literal(Value::Duration(Duration::from_secs(secs))))
            }
            TokenKind::Date(text) => {
                self.advance();
                let ts = Timestamp::parse_iso(&text).map_err(|e| PalimpsestError::ParseError {
                    line: self.line(),
                    column: 1,
                    message: e,
                })?;
                Ok(Expr::Literal(Value::Timestamp(ts)))
            }
            TokenKind::LParen => {
                self.advance();
                let inner = self.parse_expr()?;
                self.expect(&TokenKind::RParen, "`)`")?;
                Ok(inner)
            }
            TokenKind::LBracket => {
                self.advance();
                let mut items = Vec::new();
                while !self.check(&TokenKind::RBracket) && !self.at_end() {
                    items.push(self.parse_expr()?);
                    if !self.eat(&TokenKind::Comma) {
                        break;
                    }
                }
                self.expect(&TokenKind::RBracket, "`]`")?;
                Ok(Expr::List(items))
            }
            TokenKind::LBrace => {
                self.advance();
                let mut fields = Vec::new();
                while !self.check(&TokenKind::RBrace) && !self.at_end() {
                    let key = self.take_name("a field name")?;
                    if !self.eat(&TokenKind::Colon) && !self.eat_word("is") {
                        return Err(self.error("`:` after the field name"));
                    }
                    fields.push((key, self.parse_expr()?));
                    if !self.eat(&TokenKind::Comma) {
                        break;
                    }
                }
                self.expect(&TokenKind::RBrace, "`}`")?;
                Ok(Expr::Record(fields))
            }
            TokenKind::Word(raw) => {
                let w = raw.to_ascii_lowercase();
                match w.as_str() {
                    "what" => self.parse_ask(),
                    "why" => {
                        self.advance();
                        let path = self.parse_path()?;
                        Ok(Expr::Why(path))
                    }
                    "check" => {
                        self.advance();
                        Ok(Expr::Check)
                    }
                    "conflicts" => {
                        self.advance();
                        Ok(Expr::Conflicts)
                    }
                    "episodes" => {
                        self.advance();
                        Ok(Expr::Episodes)
                    }
                    "true" => {
                        self.advance();
                        Ok(Expr::Literal(Value::Bool(true)))
                    }
                    "false" => {
                        self.advance();
                        Ok(Expr::Literal(Value::Bool(false)))
                    }
                    "nothing" | "null" | "none" => {
                        self.advance();
                        Ok(Expr::Literal(Value::Null))
                    }
                    _ => {
                        let path = self.parse_path()?;
                        if path.len() == 1 {
                            Ok(Expr::Variable(path.into_iter().next().unwrap()))
                        } else {
                            Ok(Expr::Ask {
                                path,
                                as_of: None,
                                demands: Demands::default(),
                            })
                        }
                    }
                }
            }
            _ => Err(self.error("a value")),
        }
    }

    /// `what is [verified] [fresh] [trusted <tier>] <path>`
    /// `what was <path> on <date>`
    fn parse_ask(&mut self) -> Result<Expr, PalimpsestError> {
        self.advance();

        let past_tense = if self.eat_word("was") {
            true
        } else if self.eat_word("is") {
            false
        } else {
            return Err(self.error("`is` or `was` after `what`"));
        };

        let mut demands = Demands::default();
        loop {
            if self.eat_word("verified") {
                demands.verified = true;
                continue;
            }
            if self.eat_word("fresh") {
                demands.fresh = true;
                continue;
            }
            if self.word_is("trusted") {
                self.advance();
                demands.min_authority = Some(self.take_word("an authority name")?);
                continue;
            }
            break;
        }

        let path = self.parse_path()?;

        let mut as_of = None;
        if self.word_is("on") || self.word_is("at") {
            self.advance();
            as_of = Some(Box::new(self.parse_time_expr()?));
        } else if self.word_is("as") {
            // `as of <date>`
            let save = self.pos;
            self.advance();
            if self.eat_word("of") {
                as_of = Some(Box::new(self.parse_time_expr()?));
            } else {
                self.pos = save;
            }
        }

        if past_tense && as_of.is_none() {
            return Err(self.error("`on <date>` after `what was`"));
        }

        Ok(Expr::Ask {
            path,
            as_of,
            demands,
        })
    }
}
