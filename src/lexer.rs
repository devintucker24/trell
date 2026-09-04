use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Contract,
    Guard,
    Fn,
    Let,
    Return,
    Print,
    Assert,
    Fork,
    Case,
    Fallback,
    Collapse,
    Verify,
    With,
    Consensus,
    Oracle,
    Confidence,
    Justification,
    Certain,
    Belief,
    Struct,
    Model,
    Invariant,
    Temperature,
    Budget,

    // Natural Trell keywords
    End,
    When,
    Is,
    Else,
    Ask,
    Action,
    Quorum,
    Require,
    In,

    // Types
    TypeInt,
    TypeFloat,
    TypeBool,
    TypeString,
    TypeJson,

    // Literals
    Int(i64),
    Float(f64),
    StringLit(String),
    Bool(bool),
    Ident(String),

    // Symbols & Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    FatArrow, // =>
    ThinArrow, // ->
    Bang,     // !
    And,      // &&
    Or,       // ||
    Dot,
    Comma,
    Colon,
    Semi,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Eof,
}

pub struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            line: 1,
            col: 1,
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.next()?;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(ch)
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while let Some(&ch) = self.peek() {
            match ch {
                c if c.is_whitespace() => {
                    self.advance();
                }
                '/' => {
                    self.advance();
                    if let Some(&'/') = self.peek() {
                        // Line comment
                        self.advance();
                        while let Some(&c) = self.peek() {
                            self.advance();
                            if c == '\n' {
                                break;
                            }
                        }
                    } else {
                        tokens.push(Token::Slash);
                    }
                }
                '+' => { self.advance(); tokens.push(Token::Plus); }
                '-' => {
                    self.advance();
                    if let Some(&'>') = self.peek() {
                        self.advance();
                        tokens.push(Token::ThinArrow);
                    } else {
                        tokens.push(Token::Minus);
                    }
                }
                '*' => { self.advance(); tokens.push(Token::Star); }
                '%' => { self.advance(); tokens.push(Token::Percent); }
                '=' => {
                    self.advance();
                    if let Some(&'=') = self.peek() {
                        self.advance();
                        tokens.push(Token::EqualEqual);
                    } else if let Some(&'>') = self.peek() {
                        self.advance();
                        tokens.push(Token::FatArrow);
                    } else {
                        tokens.push(Token::Equal);
                    }
                }
                '!' => {
                    self.advance();
                    if let Some(&'=') = self.peek() {
                        self.advance();
                        tokens.push(Token::NotEqual);
                    } else {
                        tokens.push(Token::Bang);
                    }
                }
                '<' => {
                    self.advance();
                    if let Some(&'=') = self.peek() {
                        self.advance();
                        tokens.push(Token::LessEqual);
                    } else {
                        tokens.push(Token::LessThan);
                    }
                }
                '>' => {
                    self.advance();
                    if let Some(&'=') = self.peek() {
                        self.advance();
                        tokens.push(Token::GreaterEqual);
                    } else {
                        tokens.push(Token::GreaterThan);
                    }
                }
                '&' => {
                    self.advance();
                    if let Some(&'&') = self.peek() {
                        self.advance();
                        tokens.push(Token::And);
                    } else {
                        return Err(anyhow!("Unexpected single '&' at line {}, col {}", self.line, self.col));
                    }
                }
                '|' => {
                    self.advance();
                    if let Some(&'|') = self.peek() {
                        self.advance();
                        tokens.push(Token::Or);
                    } else {
                        return Err(anyhow!("Unexpected single '|' at line {}, col {}", self.line, self.col));
                    }
                }
                '.' => { self.advance(); tokens.push(Token::Dot); }
                ',' => { self.advance(); tokens.push(Token::Comma); }
                ':' => { self.advance(); tokens.push(Token::Colon); }
                ';' => { self.advance(); tokens.push(Token::Semi); }
                '(' => { self.advance(); tokens.push(Token::LeftParen); }
                ')' => { self.advance(); tokens.push(Token::RightParen); }
                '{' => { self.advance(); tokens.push(Token::LeftBrace); }
                '}' => { self.advance(); tokens.push(Token::RightBrace); }
                '[' => { self.advance(); tokens.push(Token::LeftBracket); }
                ']' => { self.advance(); tokens.push(Token::RightBracket); }
                '"' => {
                    tokens.push(self.lex_string()?);
                }
                c if c.is_ascii_digit() => {
                    tokens.push(self.lex_number()?);
                }
                c if c.is_ascii_alphabetic() || c == '_' => {
                    tokens.push(self.lex_identifier_or_keyword());
                }
                unexpected => {
                    return Err(anyhow!("Unexpected character '{}' at line {}, col {}", unexpected, self.line, self.col));
                }
            }
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }

    fn lex_string(&mut self) -> Result<Token> {
        self.advance(); // consume opening quote
        let mut s = String::new();
        while let Some(&c) = self.peek() {
            if c == '"' {
                self.advance();
                return Ok(Token::StringLit(s));
            } else if c == '\\' {
                self.advance();
                match self.advance() {
                    Some('n') => s.push('\n'),
                    Some('t') => s.push('\t'),
                    Some('r') => s.push('\r'),
                    Some('"') => s.push('"'),
                    Some('\\') => s.push('\\'),
                    Some(other) => s.push(other),
                    None => return Err(anyhow!("Unterminated escape sequence in string literal")),
                }
            } else {
                s.push(c);
                self.advance();
            }
        }
        Err(anyhow!("Unterminated string literal at line {}", self.line))
    }

    fn lex_number(&mut self) -> Result<Token> {
        let mut s = String::new();
        let mut is_float = false;

        while let Some(&c) = self.peek() {
            if c.is_ascii_digit() {
                s.push(c);
                self.advance();
            } else if c == '.' && !is_float {
                // Check if followed by a digit (avoid dot in method calls)
                // We peek ahead by taking an iterator clone or checking
                s.push(c);
                self.advance();
                if let Some(&next_c) = self.peek() {
                    if next_c.is_ascii_digit() {
                        is_float = true;
                    } else {
                        // Undo dot or treat as invalid
                        return Err(anyhow!("Invalid floating point literal at line {}", self.line));
                    }
                } else {
                    return Err(anyhow!("Invalid floating point literal at line {}", self.line));
                }
            } else {
                break;
            }
        }

        if is_float {
            let val = s.parse::<f64>().map_err(|e| anyhow!("Failed to parse float: {e}"))?;
            Ok(Token::Float(val))
        } else {
            let val = s.parse::<i64>().map_err(|e| anyhow!("Failed to parse integer: {e}"))?;
            Ok(Token::Int(val))
        }
    }

    fn lex_identifier_or_keyword(&mut self) -> Token {
        let mut s = String::new();
        while let Some(&c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }

        match s.as_str() {
            "contract" => Token::Contract,
            "guard" => Token::Guard,
            "fn" => Token::Fn,
            "let" => Token::Let,
            "return" => Token::Return,
            "print" => Token::Print,
            "assert" => Token::Assert,
            "fork" => Token::Fork,
            "case" => Token::Case,
            "fallback" => Token::Fallback,
            "collapse" => Token::Collapse,
            "verify" => Token::Verify,
            "with" => Token::With,
            "consensus" => Token::Consensus,
            "oracle" => Token::Oracle,
            "confidence" => Token::Confidence,
            "justification" => Token::Justification,
            "certain" => Token::Certain,
            "belief" => Token::Belief,
            "struct" => Token::Struct,
            "model" => Token::Model,
            "invariant" => Token::Invariant,
            "temperature" => Token::Temperature,
            "budget" => Token::Budget,
            "end" => Token::End,
            "when" => Token::When,
            "is" => Token::Is,
            "else" => Token::Else,
            "ask" => Token::Ask,
            "action" => Token::Action,
            "quorum" => Token::Quorum,
            "require" => Token::Require,
            "in" => Token::In,
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Bang,
            "int" => Token::TypeInt,
            "float" => Token::TypeFloat,
            "bool" => Token::TypeBool,
            "string" => Token::TypeString,
            "json" => Token::TypeJson,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            _ => Token::Ident(s),
        }
    }
}

pub fn lex(source: &str) -> Result<Vec<Token>> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}
