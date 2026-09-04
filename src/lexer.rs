// Palimpsest Lexer

use crate::error::PalimpsestError;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Authority,
    Scope,
    Assert,
    Episode,
    Retract,
    Source,
    Belief,
    Let,
    Print,
    AssertEq,
    SetTime,
    AdvanceTime,
    Recall,
    AsOf,
    Fresh,
    Verified,
    Unverified,
    MinAuthority,
    History,
    Audit,
    Conflicts,
    Episodes,
    At,
    Ttl,
    ValidUntil,
    GroundedIn,
    Actors,
    Context,
    Summary,
    True,
    False,
    Null,

    // Identifiers & Literals
    Ident(String),
    StringLit(String),
    IntLit(i64),
    FloatLit(f64),
    DurationLit(String),

    // Symbols
    Dot,
    Comma,
    Colon,
    Semicolon,
    AtSign,
    Equals,
    EqEq,
    BangEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    AmpAmp,
    PipePipe,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer<'a> {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    _source: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            _source: source,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.peek() {
            self.pos += 1;
            if ch == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            Some(ch)
        } else {
            None
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, PalimpsestError> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek() {
            // Whitespace
            if ch.is_whitespace() {
                self.advance();
                continue;
            }

            // Single line comment: // ...
            if ch == '/' && self.peek_next() == Some('/') {
                self.advance(); // consume first /
                self.advance(); // consume second /
                while let Some(c) = self.peek() {
                    if c == '\n' {
                        break;
                    }
                    self.advance();
                }
                continue;
            }

            let start_line = self.line;
            let start_col = self.col;

            // Strings
            if ch == '"' {
                let s = self.lex_string()?;
                tokens.push(Token {
                    kind: TokenKind::StringLit(s),
                    line: start_line,
                    column: start_col,
                });
                continue;
            }

            // Numbers or Durations
            if ch.is_ascii_digit() {
                let token = self.lex_number_or_duration(start_line, start_col)?;
                tokens.push(token);
                continue;
            }

            // Identifiers / Keywords
            if ch.is_ascii_alphabetic() || ch == '_' {
                let ident = self.lex_ident();
                let kind = match ident.as_str() {
                    "authority" => TokenKind::Authority,
                    "scope" => TokenKind::Scope,
                    "assert" => TokenKind::Assert,
                    "episode" => TokenKind::Episode,
                    "retract" => TokenKind::Retract,
                    "source" => TokenKind::Source,
                    "belief" => TokenKind::Belief,
                    "let" => TokenKind::Let,
                    "print" => TokenKind::Print,
                    "assert_eq" => TokenKind::AssertEq,
                    "set_time" => TokenKind::SetTime,
                    "advance_time" => TokenKind::AdvanceTime,
                    "recall" => TokenKind::Recall,
                    "as_of" => TokenKind::AsOf,
                    "fresh" => TokenKind::Fresh,
                    "verified" => TokenKind::Verified,
                    "unverified" => TokenKind::Unverified,
                    "min_authority" => TokenKind::MinAuthority,
                    "history" => TokenKind::History,
                    "audit" => TokenKind::Audit,
                    "conflicts" => TokenKind::Conflicts,
                    "episodes" => TokenKind::Episodes,
                    "at" => TokenKind::At,
                    "ttl" => TokenKind::Ttl,
                    "valid_until" => TokenKind::ValidUntil,
                    "grounded_in" => TokenKind::GroundedIn,
                    "actors" => TokenKind::Actors,
                    "context" => TokenKind::Context,
                    "summary" => TokenKind::Summary,
                    "true" => TokenKind::True,
                    "false" => TokenKind::False,
                    "null" => TokenKind::Null,
                    _ => TokenKind::Ident(ident),
                };
                tokens.push(Token {
                    kind,
                    line: start_line,
                    column: start_col,
                });
                continue;
            }

            // Symbols
            self.advance();
            let kind = match ch {
                '.' => TokenKind::Dot,
                ',' => TokenKind::Comma,
                ':' => TokenKind::Colon,
                ';' => TokenKind::Semicolon,
                '@' => TokenKind::AtSign,
                '(' => TokenKind::LeftParen,
                ')' => TokenKind::RightParen,
                '{' => TokenKind::LeftBrace,
                '}' => TokenKind::RightBrace,
                '[' => TokenKind::LeftBracket,
                ']' => TokenKind::RightBracket,
                '+' => TokenKind::Plus,
                '-' => TokenKind::Minus,
                '*' => TokenKind::Star,
                '/' => TokenKind::Slash,
                '=' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        TokenKind::EqEq
                    } else {
                        TokenKind::Equals
                    }
                }
                '!' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        TokenKind::BangEq
                    } else {
                        TokenKind::Bang
                    }
                }
                '<' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        TokenKind::LtEq
                    } else {
                        TokenKind::Lt
                    }
                }
                '>' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        TokenKind::GtEq
                    } else {
                        TokenKind::Gt
                    }
                }
                '&' => {
                    if self.peek() == Some('&') {
                        self.advance();
                        TokenKind::AmpAmp
                    } else {
                        return Err(PalimpsestError::ParseError {
                            line: start_line,
                            column: start_col,
                            message: "Unexpected character '&', expected '&&'".to_string(),
                        });
                    }
                }
                '|' => {
                    if self.peek() == Some('|') {
                        self.advance();
                        TokenKind::PipePipe
                    } else {
                        return Err(PalimpsestError::ParseError {
                            line: start_line,
                            column: start_col,
                            message: "Unexpected character '|', expected '||'".to_string(),
                        });
                    }
                }
                other => {
                    return Err(PalimpsestError::ParseError {
                        line: start_line,
                        column: start_col,
                        message: format!("Unexpected character: '{}'", other),
                    });
                }
            };

            tokens.push(Token {
                kind,
                line: start_line,
                column: start_col,
            });
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            line: self.line,
            column: self.col,
        });

        Ok(tokens)
    }

    fn lex_string(&mut self) -> Result<String, PalimpsestError> {
        let start_line = self.line;
        let start_col = self.col;
        self.advance(); // consume opening quote

        let mut s = String::new();
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance(); // consume closing quote
                return Ok(s);
            }
            if ch == '\\' {
                self.advance();
                match self.advance() {
                    Some('n') => s.push('\n'),
                    Some('t') => s.push('\t'),
                    Some('r') => s.push('\r'),
                    Some('\\') => s.push('\\'),
                    Some('"') => s.push('"'),
                    Some(c) => s.push(c),
                    None => {
                        return Err(PalimpsestError::ParseError {
                            line: start_line,
                            column: start_col,
                            message: "Unterminated escape sequence in string".to_string(),
                        });
                    }
                }
            } else {
                s.push(ch);
                self.advance();
            }
        }

        Err(PalimpsestError::ParseError {
            line: start_line,
            column: start_col,
            message: "Unterminated string literal".to_string(),
        })
    }

    fn lex_ident(&mut self) -> String {
        let mut s = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                s.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        s
    }

    fn lex_number_or_duration(&mut self, line: usize, col: usize) -> Result<Token, PalimpsestError> {
        let mut num_str = String::new();
        let mut is_float = false;

        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else if ch == '.' && self.peek_next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                is_float = true;
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Check if immediately followed by duration suffix: s, m, h, d, w, y, ms
        if let Some(ch) = self.peek() {
            if ch.is_ascii_alphabetic() {
                let unit = self.lex_ident();
                let dur_str = format!("{}{}", num_str, unit);
                return Ok(Token {
                    kind: TokenKind::DurationLit(dur_str),
                    line,
                    column: col,
                });
            }
        }

        if is_float {
            let f = num_str.parse::<f64>().map_err(|e| PalimpsestError::ParseError {
                line,
                column: col,
                message: format!("Invalid float literal: {}", e),
            })?;
            Ok(Token {
                kind: TokenKind::FloatLit(f),
                line,
                column: col,
            })
        } else {
            let i = num_str.parse::<i64>().map_err(|e| PalimpsestError::ParseError {
                line,
                column: col,
                message: format!("Invalid integer literal: {}", e),
            })?;
            Ok(Token {
                kind: TokenKind::IntLit(i),
                line,
                column: col,
            })
        }
    }
}
