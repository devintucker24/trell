use anyhow::{anyhow, Result};

use crate::ast::*;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn check(&self, expected: &Token) -> bool {
        self.peek() == expected
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    fn consume(&mut self, expected: &Token, err_msg: &str) -> Result<&Token> {
        if self.check(expected) {
            Ok(self.advance())
        } else {
            Err(anyhow!("Expected {:?}, found {:?}: {}", expected, self.peek(), err_msg))
        }
    }

    fn consume_ident(&mut self, err_msg: &str) -> Result<String> {
        match self.peek() {
            Token::Ident(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            other => Err(anyhow!("Expected identifier, found {:?}: {}", other, err_msg)),
        }
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        // Backwards compatibility check: If file starts with an expression directly
        // rather than contract/struct/guard/fn declarations, wrap it into `fn main() { print(<expr>); }`
        if !matches!(self.peek(), Token::Contract | Token::Struct | Token::Guard | Token::Fn | Token::Eof) {
            let expr = self.parse_expr()?;
            return Ok(Program {
                items: vec![Item::Function(FunctionDef {
                    name: "main".to_string(),
                    params: Vec::new(),
                    return_type: Type::Unit,
                    body: vec![Stmt::Print(expr)],
                })],
            });
        }

        let mut items = Vec::new();

        while !self.is_at_end() {
            match self.peek() {
                Token::Contract => items.push(Item::Contract(self.parse_contract()?)),
                Token::Struct => items.push(Item::Struct(self.parse_struct()?)),
                Token::Guard => items.push(Item::Guard(self.parse_guard()?)),
                Token::Fn => items.push(Item::Function(self.parse_function()?)),
                other => return Err(anyhow!("Unexpected token at top level: {:?}", other)),
            }
        }

        Ok(Program { items })
    }

    fn parse_contract(&mut self) -> Result<ModelContract> {
        self.consume(&Token::Contract, "Expected 'contract'")?;
        let name = self.consume_ident("Expected contract name")?;
        self.consume(&Token::LeftBrace, "Expected '{' after contract name")?;

        let mut model_kind = String::from("reasoning");
        let mut temperature = None;
        let mut max_tokens = None;
        let mut min_confidence = None;

        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            match self.peek() {
                Token::Model => {
                    self.advance();
                    self.consume(&Token::Colon, "Expected ':' after 'model'")?;
                    model_kind = self.consume_ident("Expected model kind")?;
                    if self.check(&Token::Semi) {
                        self.advance();
                    }
                }
                Token::Temperature => {
                    self.advance();
                    self.consume(&Token::Colon, "Expected ':' after 'temperature'")?;
                    match self.peek() {
                        Token::Float(val) => {
                            temperature = Some(*val);
                            self.advance();
                        }
                        Token::Int(val) => {
                            temperature = Some(*val as f64);
                            self.advance();
                        }
                        other => return Err(anyhow!("Expected float or int for temperature, found {:?}", other)),
                    }
                    if self.check(&Token::Semi) {
                        self.advance();
                    }
                }
                Token::Budget => {
                    self.advance();
                    self.consume(&Token::Colon, "Expected ':' after 'budget'")?;
                    match self.peek() {
                        Token::Int(val) => {
                            max_tokens = Some(*val as u64);
                            self.advance();
                        }
                        other => return Err(anyhow!("Expected integer for token budget, found {:?}", other)),
                    }
                    if self.check(&Token::Semi) {
                        self.advance();
                    }
                }
                Token::Invariant => {
                    self.advance();
                    self.consume(&Token::Colon, "Expected ':' after 'invariant'")?;
                    // Format: confidence >= 0.85
                    if self.check(&Token::Confidence) {
                        self.advance();
                        self.consume(&Token::GreaterEqual, "Expected '>=' after confidence")?;
                        match self.peek() {
                            Token::Float(val) => {
                                min_confidence = Some(*val);
                                self.advance();
                            }
                            Token::Int(val) => {
                                min_confidence = Some(*val as f64);
                                self.advance();
                            }
                            other => return Err(anyhow!("Expected number for min confidence, found {:?}", other)),
                        }
                    } else {
                        return Err(anyhow!("Unsupported contract invariant form"));
                    }
                    if self.check(&Token::Semi) {
                        self.advance();
                    }
                }
                other => return Err(anyhow!("Unexpected token inside contract definition: {:?}", other)),
            }
        }

        self.consume(&Token::RightBrace, "Expected '}' closing contract")?;
        Ok(ModelContract {
            name,
            model_kind,
            temperature,
            max_tokens,
            min_confidence,
        })
    }

    fn parse_struct(&mut self) -> Result<StructDef> {
        self.consume(&Token::Struct, "Expected 'struct'")?;
        let name = self.consume_ident("Expected struct name")?;
        self.consume(&Token::LeftBrace, "Expected '{' after struct name")?;

        let mut fields = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let field_name = self.consume_ident("Expected struct field name")?;
            self.consume(&Token::Colon, "Expected ':' after field name")?;
            let ty = self.parse_type()?;
            fields.push(StructField { name: field_name, ty });
            if self.check(&Token::Comma) {
                self.advance();
            }
        }

        self.consume(&Token::RightBrace, "Expected '}' after struct fields")?;
        Ok(StructDef { name, fields })
    }

    fn parse_guard(&mut self) -> Result<GuardDef> {
        self.consume(&Token::Guard, "Expected 'guard'")?;
        let name = self.consume_ident("Expected guard name")?;
        self.consume(&Token::LeftParen, "Expected '(' after guard name")?;
        let param_name = self.consume_ident("Expected guard parameter name")?;
        self.consume(&Token::Colon, "Expected ':' after guard parameter name")?;
        let param_type = self.parse_type()?;
        self.consume(&Token::RightParen, "Expected ')' closing guard parameter list")?;

        self.consume(&Token::LeftBrace, "Expected '{' opening guard body")?;
        let body = self.parse_expr()?;
        self.consume(&Token::RightBrace, "Expected '}' closing guard body")?;

        Ok(GuardDef {
            name,
            param_name,
            param_type,
            body,
        })
    }

    fn parse_function(&mut self) -> Result<FunctionDef> {
        self.consume(&Token::Fn, "Expected 'fn'")?;
        let name = self.consume_ident("Expected function name")?;
        self.consume(&Token::LeftParen, "Expected '(' after function name")?;

        let mut params = Vec::new();
        while !self.check(&Token::RightParen) && !self.is_at_end() {
            let param_name = self.consume_ident("Expected parameter name")?;
            self.consume(&Token::Colon, "Expected ':' after parameter name")?;
            let ty = self.parse_type()?;
            params.push(Param { name: param_name, ty });
            if self.check(&Token::Comma) {
                self.advance();
            }
        }

        self.consume(&Token::RightParen, "Expected ')' after parameter list")?;

        let return_type = if self.check(&Token::ThinArrow) {
            self.advance();
            self.parse_type()?
        } else {
            Type::Unit
        };

        self.consume(&Token::LeftBrace, "Expected '{' opening function body")?;
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.parse_stmt()?);
        }
        self.consume(&Token::RightBrace, "Expected '}' closing function body")?;

        Ok(FunctionDef {
            name,
            params,
            return_type,
            body,
        })
    }

    pub fn parse_type(&mut self) -> Result<Type> {
        if self.check(&Token::Certain) {
            self.advance();
            match self.peek() {
                Token::TypeInt => { self.advance(); Ok(Type::Certain(PrimitiveType::Int)) }
                Token::TypeFloat => { self.advance(); Ok(Type::Certain(PrimitiveType::Float)) }
                Token::TypeBool => { self.advance(); Ok(Type::Certain(PrimitiveType::Bool)) }
                Token::TypeString => { self.advance(); Ok(Type::Certain(PrimitiveType::String)) }
                Token::TypeJson => { self.advance(); Ok(Type::Certain(PrimitiveType::Json)) }
                Token::Ident(name) => {
                    let name = name.clone();
                    self.advance();
                    Ok(Type::CertainCustom(name))
                }
                other => Err(anyhow!("Expected primitive type or struct name after 'certain', found {:?}", other)),
            }
        } else if self.check(&Token::Belief) {
            self.advance();
            self.consume(&Token::LessThan, "Expected '<' after 'belief'")?;
            let inner_ty = match self.peek() {
                Token::TypeInt => { self.advance(); Type::Belief(PrimitiveType::Int) }
                Token::TypeFloat => { self.advance(); Type::Belief(PrimitiveType::Float) }
                Token::TypeBool => { self.advance(); Type::Belief(PrimitiveType::Bool) }
                Token::TypeString => { self.advance(); Type::Belief(PrimitiveType::String) }
                Token::TypeJson => { self.advance(); Type::Belief(PrimitiveType::Json) }
                Token::Ident(name) => {
                    let name = name.clone();
                    self.advance();
                    Type::BeliefCustom(name)
                }
                other => return Err(anyhow!("Expected inner type inside 'belief<...>', found {:?}", other)),
            };
            self.consume(&Token::GreaterThan, "Expected '>' closing 'belief<...>'")?;
            Ok(inner_ty)
        } else {
            // Default bare type: treat as Certain
            match self.peek() {
                Token::TypeInt => { self.advance(); Ok(Type::Certain(PrimitiveType::Int)) }
                Token::TypeFloat => { self.advance(); Ok(Type::Certain(PrimitiveType::Float)) }
                Token::TypeBool => { self.advance(); Ok(Type::Certain(PrimitiveType::Bool)) }
                Token::TypeString => { self.advance(); Ok(Type::Certain(PrimitiveType::String)) }
                Token::TypeJson => { self.advance(); Ok(Type::Certain(PrimitiveType::Json)) }
                Token::Ident(name) => {
                    let name = name.clone();
                    self.advance();
                    Ok(Type::CertainCustom(name))
                }
                other => Err(anyhow!("Expected type, found {:?}", other)),
            }
        }
    }

    fn parse_stmt(&mut self) -> Result<Stmt> {
        match self.peek() {
            Token::Let => {
                self.advance();
                let name = self.consume_ident("Expected variable name after 'let'")?;
                let ty = if self.check(&Token::Colon) {
                    self.advance();
                    Some(self.parse_type()?)
                } else {
                    None
                };
                self.consume(&Token::Equal, "Expected '=' in variable declaration")?;
                let value = self.parse_expr()?;
                self.consume(&Token::Semi, "Expected ';' after let statement")?;
                Ok(Stmt::Let { name, ty, value })
            }
            Token::Print => {
                self.advance();
                let expr = self.parse_expr()?;
                self.consume(&Token::Semi, "Expected ';' after print statement")?;
                Ok(Stmt::Print(expr))
            }
            Token::Assert => {
                self.advance();
                let condition = self.parse_expr()?;
                let message = if self.check(&Token::Comma) {
                    self.advance();
                    match self.peek() {
                        Token::StringLit(msg) => {
                            let msg = msg.clone();
                            self.advance();
                            Some(msg)
                        }
                        other => return Err(anyhow!("Expected string literal error message in assert, found {:?}", other)),
                    }
                } else {
                    None
                };
                self.consume(&Token::Semi, "Expected ';' after assert statement")?;
                Ok(Stmt::Assert { condition, message })
            }
            Token::Return => {
                self.advance();
                if self.check(&Token::Semi) {
                    self.advance();
                    Ok(Stmt::Return(None))
                } else {
                    let expr = self.parse_expr()?;
                    self.consume(&Token::Semi, "Expected ';' after return value")?;
                    Ok(Stmt::Return(Some(expr)))
                }
            }
            _ => {
                // Could be assignment or expr statement
                let expr = self.parse_expr()?;
                if self.check(&Token::Equal) {
                    self.advance();
                    let val = self.parse_expr()?;
                    self.consume(&Token::Semi, "Expected ';' after assignment")?;
                    if let Expr::Ident(id) = expr {
                        Ok(Stmt::Assign { target: id, value: val })
                    } else {
                        Err(anyhow!("Invalid assignment target: {:?}", expr))
                    }
                } else {
                    if self.check(&Token::Semi) {
                        self.advance();
                    }
                    Ok(Stmt::Expr(expr))
                }
            }
        }
    }

    pub fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expr> {
        let mut left = self.parse_logical_and()?;
        while self.check(&Token::Or) {
            self.advance();
            let right = self.parse_logical_and()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<Expr> {
        let mut left = self.parse_equality()?;
        while self.check(&Token::And) {
            self.advance();
            let right = self.parse_equality()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr> {
        let mut left = self.parse_comparison()?;
        while matches!(self.peek(), Token::EqualEqual | Token::NotEqual) {
            let op = match self.advance() {
                Token::EqualEqual => BinaryOp::Eq,
                Token::NotEqual => BinaryOp::Neq,
                _ => unreachable!(),
            };
            let right = self.parse_comparison()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr> {
        let mut left = self.parse_term()?;
        while matches!(self.peek(), Token::LessThan | Token::LessEqual | Token::GreaterThan | Token::GreaterEqual) {
            let op = match self.advance() {
                Token::LessThan => BinaryOp::Lt,
                Token::LessEqual => BinaryOp::Lte,
                Token::GreaterThan => BinaryOp::Gt,
                Token::GreaterEqual => BinaryOp::Gte,
                _ => unreachable!(),
            };
            let right = self.parse_term()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr> {
        let mut left = self.parse_factor()?;
        while matches!(self.peek(), Token::Plus | Token::Minus) {
            let op = match self.advance() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_factor()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr> {
        let mut left = self.parse_unary()?;
        while matches!(self.peek(), Token::Star | Token::Slash | Token::Percent) {
            let op = match self.advance() {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                Token::Percent => BinaryOp::Mod,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr> {
        if self.check(&Token::Bang) {
            self.advance();
            let operand = self.parse_unary()?;
            Ok(Expr::UnaryNot(Box::new(operand)))
        } else {
            self.parse_postfix()
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.check(&Token::Dot) {
                self.advance();
                let field = self.consume_ident("Expected field name or method after '.'")?;
                if self.check(&Token::LeftParen) {
                    // Method call: expr.method(...) -> could be lowered or represented as Call with self
                    self.advance();
                    let mut args = vec![expr];
                    while !self.check(&Token::RightParen) && !self.is_at_end() {
                        args.push(self.parse_expr()?);
                        if self.check(&Token::Comma) {
                            self.advance();
                        }
                    }
                    self.consume(&Token::RightParen, "Expected ')' after method arguments")?;
                    expr = Expr::Call {
                        function: field,
                        args,
                    };
                } else {
                    expr = Expr::FieldAccess {
                        target: Box::new(expr),
                        field,
                    };
                }
            } else if self.check(&Token::LeftParen) {
                // Function call: expr(...)
                if let Expr::Ident(name) = expr {
                    self.advance();
                    let mut args = Vec::new();
                    while !self.check(&Token::RightParen) && !self.is_at_end() {
                        args.push(self.parse_expr()?);
                        if self.check(&Token::Comma) {
                            self.advance();
                        }
                    }
                    self.consume(&Token::RightParen, "Expected ')' after function arguments")?;
                    expr = Expr::Call {
                        function: name,
                        args,
                    };
                } else {
                    return Err(anyhow!("Cannot call non-identifier expression"));
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        match self.peek().clone() {
            Token::Int(n) => {
                self.advance();
                Ok(Expr::Lit(Literal::Int(n)))
            }
            Token::Float(f) => {
                self.advance();
                Ok(Expr::Lit(Literal::Float(f)))
            }
            Token::StringLit(s) => {
                self.advance();
                Ok(Expr::Lit(Literal::String(s)))
            }
            Token::Bool(b) => {
                self.advance();
                Ok(Expr::Lit(Literal::Bool(b)))
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.consume(&Token::RightParen, "Expected ')' closing parenthesized expression")?;
                Ok(expr)
            }
            Token::LeftBrace => {
                // Block expression: { stmt; stmt; expr }
                self.advance();
                let mut stmts = Vec::new();
                let mut tail = None;
                while !self.check(&Token::RightBrace) && !self.is_at_end() {
                    // Check if it's an expr without trailing semicolon
                    // Try parsing stmt
                    stmts.push(self.parse_stmt()?);
                }
                // Check if the last stmt was an Expr, which can be the block's tail
                if let Some(Stmt::Expr(expr)) = stmts.last() {
                    tail = Some(Box::new(expr.clone()));
                }
                self.consume(&Token::RightBrace, "Expected '}' closing block")?;
                Ok(Expr::Block(stmts, tail))
            }
            Token::Confidence => {
                self.advance();
                self.consume(&Token::LeftParen, "Expected '(' after confidence")?;
                let target = self.parse_expr()?;
                self.consume(&Token::RightParen, "Expected ')' closing confidence(...)")?;
                Ok(Expr::Confidence(Box::new(target)))
            }
            Token::Justification => {
                self.advance();
                self.consume(&Token::LeftParen, "Expected '(' after justification")?;
                let target = self.parse_expr()?;
                self.consume(&Token::RightParen, "Expected ')' closing justification(...)")?;
                Ok(Expr::Justification(Box::new(target)))
            }
            Token::Verify => {
                // verify <target> with <guard_name> [fallback <expr>]
                self.advance();
                let target = self.parse_expr()?;
                self.consume(&Token::With, "Expected 'with' in verify expression")?;
                let guard_name = self.consume_ident("Expected guard name after 'with'")?;
                let fallback = if self.check(&Token::Fallback) {
                    self.advance();
                    Some(Box::new(self.parse_expr()?))
                } else {
                    None
                };
                Ok(Expr::Verify {
                    target: Box::new(target),
                    guard_name,
                    fallback,
                })
            }
            Token::Consensus => {
                // consensus(n, threshold) { oracle<Contract>.method(prompt) }
                self.advance();
                self.consume(&Token::LeftParen, "Expected '(' after 'consensus'")?;
                let count = match self.peek() {
                    Token::Int(n) => {
                        let c = *n as usize;
                        self.advance();
                        c
                    }
                    other => return Err(anyhow!("Expected integer count for consensus, found {:?}", other)),
                };
                self.consume(&Token::Comma, "Expected ',' in consensus(n, threshold)")?;
                let threshold = match self.peek() {
                    Token::Float(f) => {
                        let t = *f;
                        self.advance();
                        t
                    }
                    Token::Int(n) => {
                        let t = *n as f64;
                        self.advance();
                        t
                    }
                    other => return Err(anyhow!("Expected float threshold for consensus, found {:?}", other)),
                };
                self.consume(&Token::RightParen, "Expected ')' after consensus parameters")?;
                self.consume(&Token::LeftBrace, "Expected '{' enclosing consensus oracle call")?;
                let oracle_call = self.parse_expr()?;
                self.consume(&Token::RightBrace, "Expected '}' enclosing consensus oracle call")?;
                Ok(Expr::Consensus {
                    count,
                    threshold,
                    oracle_call: Box::new(oracle_call),
                })
            }
            Token::Oracle => {
                // oracle<ContractName> : Type . method ( prompt_arg )
                // Or: oracle<ContractName>.method(prompt_arg) : Type
                self.advance();
                self.consume(&Token::LessThan, "Expected '<' after 'oracle'")?;
                let contract = self.consume_ident("Expected contract name")?;
                self.consume(&Token::GreaterThan, "Expected '>' after contract name")?;

                let target_type = if self.check(&Token::Colon) {
                    self.advance();
                    self.parse_type()?
                } else {
                    Type::Belief(PrimitiveType::String)
                };

                self.consume(&Token::Dot, "Expected '.' after oracle<...>")?;
                let method = self.consume_ident("Expected oracle method name (e.g. assess, judge, query)")?;
                self.consume(&Token::LeftParen, "Expected '(' after oracle method")?;
                let prompt_arg = self.parse_expr()?;
                self.consume(&Token::RightParen, "Expected ')' after oracle prompt argument")?;

                Ok(Expr::OracleCall {
                    contract,
                    method,
                    prompt_arg: Box::new(prompt_arg),
                    target_type,
                })
            }
            Token::Fork => {
                // fork target { case Pattern => { stmts } ... fallback => { stmts } } collapse
                self.advance();
                let target = self.parse_expr()?;
                self.consume(&Token::LeftBrace, "Expected '{' opening fork block")?;

                let mut cases = Vec::new();
                let mut fallback = None;

                while !self.check(&Token::RightBrace) && !self.is_at_end() {
                    if self.check(&Token::Case) {
                        self.advance();
                        let pattern_name = self.consume_ident("Expected case pattern name")?;
                        let binding = if self.check(&Token::LeftParen) {
                            self.advance();
                            let b = self.consume_ident("Expected case variable binding")?;
                            self.consume(&Token::RightParen, "Expected ')' after case binding")?;
                            Some(b)
                        } else {
                            None
                        };
                        self.consume(&Token::FatArrow, "Expected '=>' after case pattern")?;
                        self.consume(&Token::LeftBrace, "Expected '{' opening case body")?;
                        let mut body = Vec::new();
                        while !self.check(&Token::RightBrace) && !self.is_at_end() {
                            body.push(self.parse_stmt()?);
                        }
                        self.consume(&Token::RightBrace, "Expected '}' closing case body")?;
                        cases.push(ForkCase {
                            pattern_name,
                            binding,
                            body,
                        });
                    } else if self.check(&Token::Fallback) {
                        self.advance();
                        self.consume(&Token::FatArrow, "Expected '=>' after fallback")?;
                        self.consume(&Token::LeftBrace, "Expected '{' opening fallback body")?;
                        let mut body = Vec::new();
                        while !self.check(&Token::RightBrace) && !self.is_at_end() {
                            body.push(self.parse_stmt()?);
                        }
                        self.consume(&Token::RightBrace, "Expected '}' closing fallback body")?;
                        fallback = Some(body);
                    } else {
                        return Err(anyhow!("Expected 'case' or 'fallback' inside fork, found {:?}", self.peek()));
                    }
                }

                self.consume(&Token::RightBrace, "Expected '}' closing fork block")?;
                self.consume(&Token::Collapse, "Expected 'collapse' keyword after fork block")?;

                Ok(Expr::Fork {
                    target: Box::new(target),
                    cases,
                    fallback,
                })
            }
            Token::Ident(name) => {
                self.advance();
                // Check if this is a struct initialization: StructName { field: expr, ... }
                if self.check(&Token::LeftBrace) {
                    // Peek ahead to see if it looks like field: value
                    let is_struct_init = if self.current + 2 < self.tokens.len() {
                        matches!((&self.tokens[self.current + 1], &self.tokens[self.current + 2]), (Token::Ident(_), Token::Colon))
                    } else {
                        false
                    };

                    if is_struct_init {
                        self.advance(); // consume '{'
                        let mut fields = Vec::new();
                        while !self.check(&Token::RightBrace) && !self.is_at_end() {
                            let f_name = self.consume_ident("Expected field name in struct init")?;
                            self.consume(&Token::Colon, "Expected ':' after struct field name")?;
                            let f_val = self.parse_expr()?;
                            fields.push((f_name, f_val));
                            if self.check(&Token::Comma) {
                                self.advance();
                            }
                        }
                        self.consume(&Token::RightBrace, "Expected '}' closing struct init")?;
                        return Ok(Expr::StructInit { name, fields });
                    }
                }

                Ok(Expr::Ident(name))
            }
            other => Err(anyhow!("Unexpected token parsing primary expression: {:?}", other)),
        }
    }
}
