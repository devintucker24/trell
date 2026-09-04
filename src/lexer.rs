use crate::diagnostics::Diagnostic;
use crate::span::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Int(i64),
    Text(String),
    Ident(String),

    Cap,
    Allow,
    Deny,
    Need,
    Approve,
    On,
    Budget,
    Spawn,
    In,
    Let,
    If,
    Else,
    Return,
    Send,
    Ask,
    Using,
    As,
    Enum,
    True,
    False,
    Read,
    Write,
    IntType,
    TextType,
    BoolType,

    Plus,
    Minus,
    Star,
    Slash,
    EqEq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    AndAnd,
    OrOr,
    Not,
    Assign,
    Dot,
    Comma,
    Colon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Eof,
}

impl TokenKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenKind::Int(_) => "integer",
            TokenKind::Text(_) => "string",
            TokenKind::Ident(_) => "identifier",
            TokenKind::Cap => "cap",
            TokenKind::Allow => "allow",
            TokenKind::Deny => "deny",
            TokenKind::Need => "need",
            TokenKind::Approve => "approve",
            TokenKind::On => "on",
            TokenKind::Budget => "budget",
            TokenKind::Spawn => "spawn",
            TokenKind::In => "in",
            TokenKind::Let => "let",
            TokenKind::If => "if",
            TokenKind::Else => "else",
            TokenKind::Return => "return",
            TokenKind::Send => "send",
            TokenKind::Ask => "ask",
            TokenKind::Using => "using",
            TokenKind::As => "as",
            TokenKind::Enum => "enum",
            TokenKind::True => "true",
            TokenKind::False => "false",
            TokenKind::Read => "read",
            TokenKind::Write => "write",
            TokenKind::IntType => "int",
            TokenKind::TextType => "text",
            TokenKind::BoolType => "bool",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            TokenKind::EqEq => "==",
            TokenKind::NotEq => "!=",
            TokenKind::Lt => "<",
            TokenKind::Gt => ">",
            TokenKind::LtEq => "<=",
            TokenKind::GtEq => ">=",
            TokenKind::AndAnd => "&&",
            TokenKind::OrOr => "||",
            TokenKind::Not => "!",
            TokenKind::Assign => "=",
            TokenKind::Dot => ".",
            TokenKind::Comma => ",",
            TokenKind::Colon => ":",
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::LBrace => "{",
            TokenKind::RBrace => "}",
            TokenKind::Eof => "end of file",
        }
    }

    pub fn ident_name(&self) -> Option<&str> {
        match self {
            TokenKind::Ident(name) => Some(name),
            TokenKind::Cap => Some("cap"),
            TokenKind::Allow => Some("allow"),
            TokenKind::Deny => Some("deny"),
            TokenKind::Need => Some("need"),
            TokenKind::Approve => Some("approve"),
            TokenKind::On => Some("on"),
            TokenKind::Budget => Some("budget"),
            TokenKind::Spawn => Some("spawn"),
            TokenKind::In => Some("in"),
            TokenKind::Let => Some("let"),
            TokenKind::If => Some("if"),
            TokenKind::Else => Some("else"),
            TokenKind::Return => Some("return"),
            TokenKind::Send => Some("send"),
            TokenKind::Ask => Some("ask"),
            TokenKind::Using => Some("using"),
            TokenKind::As => Some("as"),
            TokenKind::Enum => Some("enum"),
            TokenKind::Read => Some("read"),
            TokenKind::Write => Some("write"),
            TokenKind::IntType => Some("int"),
            TokenKind::TextType => Some("text"),
            TokenKind::BoolType => Some("bool"),
            TokenKind::True => Some("true"),
            TokenKind::False => Some("false"),
            _ => None,
        }
    }
}

pub const KEYWORDS: &[(&str, TokenKind)] = &[
    ("cap", TokenKind::Cap),
    ("allow", TokenKind::Allow),
    ("deny", TokenKind::Deny),
    ("need", TokenKind::Need),
    ("approve", TokenKind::Approve),
    ("on", TokenKind::On),
    ("budget", TokenKind::Budget),
    ("spawn", TokenKind::Spawn),
    ("in", TokenKind::In),
    ("let", TokenKind::Let),
    ("if", TokenKind::If),
    ("else", TokenKind::Else),
    ("return", TokenKind::Return),
    ("send", TokenKind::Send),
    ("ask", TokenKind::Ask),
    ("using", TokenKind::Using),
    ("as", TokenKind::As),
    ("enum", TokenKind::Enum),
    ("true", TokenKind::True),
    ("false", TokenKind::False),
    ("read", TokenKind::Read),
    ("write", TokenKind::Write),
    ("int", TokenKind::IntType),
    ("text", TokenKind::TextType),
    ("bool", TokenKind::BoolType),
];

pub fn lex(source: &str) -> Result<Vec<Token>, Diagnostic> {
    let mut tokens = Vec::new();
    let bytes = source.as_bytes();
    let mut i = 0usize;

    while i < bytes.len() {
        let start = i;
        let ch = source[i..].chars().next().unwrap();
        let width = ch.len_utf8();

        if ch.is_whitespace() {
            i += width;
            continue;
        }

        if ch == '/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            i += 2;
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }

        let kind = match ch {
            '+' => {
                i += 1;
                TokenKind::Plus
            }
            '*' => {
                i += 1;
                TokenKind::Star
            }
            '/' => {
                i += 1;
                TokenKind::Slash
            }
            '(' => {
                i += 1;
                TokenKind::LParen
            }
            ')' => {
                i += 1;
                TokenKind::RParen
            }
            '{' => {
                i += 1;
                TokenKind::LBrace
            }
            '}' => {
                i += 1;
                TokenKind::RBrace
            }
            '.' => {
                i += 1;
                TokenKind::Dot
            }
            ',' => {
                i += 1;
                TokenKind::Comma
            }
            ':' => {
                i += 1;
                TokenKind::Colon
            }
            '-' => {
                i += 1;
                TokenKind::Minus
            }
            '!' => {
                i += 1;
                if i < bytes.len() && bytes[i] == b'=' {
                    i += 1;
                    TokenKind::NotEq
                } else {
                    TokenKind::Not
                }
            }
            '=' => {
                i += 1;
                if i < bytes.len() && bytes[i] == b'=' {
                    i += 1;
                    TokenKind::EqEq
                } else {
                    TokenKind::Assign
                }
            }
            '<' => {
                i += 1;
                if i < bytes.len() && bytes[i] == b'=' {
                    i += 1;
                    TokenKind::LtEq
                } else {
                    TokenKind::Lt
                }
            }
            '>' => {
                i += 1;
                if i < bytes.len() && bytes[i] == b'=' {
                    i += 1;
                    TokenKind::GtEq
                } else {
                    TokenKind::Gt
                }
            }
            '&' => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'&' {
                    i += 2;
                    TokenKind::AndAnd
                } else {
                    return Err(Diagnostic::error(
                        "Unexpected character '&' (did you mean '&&'?)",
                        Span::new(start, start + 1),
                    ));
                }
            }
            '|' => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'|' {
                    i += 2;
                    TokenKind::OrOr
                } else {
                    return Err(Diagnostic::error(
                        "Unexpected character '|' (did you mean '||'?)",
                        Span::new(start, start + 1),
                    ));
                }
            }
            '"' => {
                i += 1;
                let mut value = String::new();
                loop {
                    if i >= bytes.len() {
                        return Err(Diagnostic::error(
                            "Unterminated string literal",
                            Span::new(start, i),
                        ));
                    }
                    let next = source[i..].chars().next().unwrap();
                    let next_width = next.len_utf8();
                    match next {
                        '"' => {
                            i += 1;
                            break;
                        }
                        '\\' => {
                            i += 1;
                            if i >= bytes.len() {
                                return Err(Diagnostic::error(
                                    "Unterminated string escape",
                                    Span::new(start, i),
                                ));
                            }
                            let escaped = source[i..].chars().next().unwrap();
                            let escaped_width = escaped.len_utf8();
                            let mapped = match escaped {
                                'n' => '\n',
                                't' => '\t',
                                '"' => '"',
                                '\\' => '\\',
                                other => {
                                    return Err(Diagnostic::error(
                                        format!("Unknown string escape '\\{other}'"),
                                        Span::new(i - 1, i + escaped_width),
                                    ));
                                }
                            };
                            value.push(mapped);
                            i += escaped_width;
                        }
                        '\n' => {
                            return Err(Diagnostic::error(
                                "Unterminated string literal",
                                Span::new(start, i),
                            ));
                        }
                        other => {
                            value.push(other);
                            i += next_width;
                        }
                    }
                }
                TokenKind::Text(value)
            }
            digit if digit.is_ascii_digit() => {
                let mut end = i + 1;
                while end < bytes.len() && bytes[end].is_ascii_digit() {
                    end += 1;
                }
                let number = &source[i..end];
                i = end;
                let value = number.parse::<i64>().map_err(|_| {
                    Diagnostic::error(
                        format!("Integer literal is too large: {number}"),
                        Span::new(start, end),
                    )
                })?;
                TokenKind::Int(value)
            }
            letter if letter.is_ascii_alphabetic() || letter == '_' => {
                let mut end = i + width;
                while end < bytes.len() {
                    let next = bytes[end];
                    if next.is_ascii_alphanumeric() || next == b'_' {
                        end += 1;
                    } else {
                        break;
                    }
                }
                let name = &source[i..end];
                i = end;
                keyword_or_ident(name)
            }
            unexpected => {
                return Err(Diagnostic::error(
                    format!("Unexpected character in Trell source: '{unexpected}'"),
                    Span::new(start, start + width),
                ));
            }
        };

        tokens.push(Token {
            kind,
            span: Span::new(start, i),
        });
    }

    tokens.push(Token {
        kind: TokenKind::Eof,
        span: Span::new(source.len(), source.len()),
    });
    Ok(tokens)
}

fn keyword_or_ident(name: &str) -> TokenKind {
    for (keyword, kind) in KEYWORDS {
        if *keyword == name {
            return kind.clone();
        }
    }
    TokenKind::Ident(name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_arithmetic() {
        let tokens = lex("20 + 22 * 2").unwrap();
        assert!(matches!(tokens[0].kind, TokenKind::Int(20)));
        assert!(matches!(tokens[1].kind, TokenKind::Plus));
        assert!(matches!(tokens[2].kind, TokenKind::Int(22)));
        assert!(matches!(tokens[3].kind, TokenKind::Star));
        assert!(matches!(tokens[4].kind, TokenKind::Int(2)));
        assert!(matches!(tokens[5].kind, TokenKind::Eof));
    }

    #[test]
    fn lexes_keywords_and_comments() {
        let tokens = lex("cap demo { allow ask // hi\n }").unwrap();
        assert!(matches!(tokens[0].kind, TokenKind::Cap));
        assert!(matches!(tokens[1].kind, TokenKind::Ident(ref n) if n == "demo"));
        assert!(matches!(tokens[2].kind, TokenKind::LBrace));
        assert!(matches!(tokens[3].kind, TokenKind::Allow));
        assert!(matches!(tokens[4].kind, TokenKind::Ask));
    }

    #[test]
    fn rejects_unknown_escape() {
        let err = lex("\"\\q\"").unwrap_err();
        assert!(err.message.contains("Unknown string escape"));
    }
}
