use anyhow::{anyhow, Result};

use crate::ast::{BinaryOperator, Expr};
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse_program(&mut self) -> Result<Expr> {
        let expression = self.parse_addition()?;

        if !matches!(self.current_token(), Token::Eof) {
            return Err(anyhow!(
                "Expected the end of the Trell expression, found {:?}",
                self.current_token()
            ));
        }

        Ok(expression)
    }

    fn parse_addition(&mut self) -> Result<Expr> {
        let mut expression = self.parse_multiplication()?;

        loop {
            let operator = match self.current_token() {
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Subtract,
                _ => break,
            };

            self.advance();
            let right = self.parse_multiplication()?;

            expression = Expr::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn parse_multiplication(&mut self) -> Result<Expr> {
        let mut expression = self.parse_primary()?;

        loop {
            let operator = match self.current_token() {
                Token::Star => BinaryOperator::Multiply,
                Token::Slash => BinaryOperator::Divide,
                _ => break,
            };

            self.advance();
            let right = self.parse_primary()?;

            expression = Expr::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        match self.current_token().clone() {
            Token::Integer(value) => {
                self.advance();
                Ok(Expr::Integer(value))
            }

            Token::LeftParen => {
                self.advance();
                let expression = self.parse_addition()?;

                match self.current_token() {
                    Token::RightParen => {
                        self.advance();
                        Ok(expression)
                    }
                    token => Err(anyhow!("Expected ')', found {:?}", token)),
                }
            }

            token => Err(anyhow!("Expected an integer or '(', found {:?}", token)),
        }
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) {
        if !matches!(self.current_token(), Token::Eof) {
            self.current += 1;
        }
    }
}
