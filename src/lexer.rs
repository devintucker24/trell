use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Number(f64),
    String(String),
    Axes,
    Offer,
    At,
    Feel,
    Grain,
    Along,
    Toward,
    Keeping,
    Blend,
    With,
    By,
    Without,
    Speak,
    Shadow,
    Of,
    When,
    Else,
    Path,
    Via,
    Echo,
    Space,
    Arrow, // <->
    LBrace,
    RBrace,
    LParen,
    RParen,
    Colon,
    Comma,
    Eq,
    Dot,
    Tilde,
    Gt,
    Lt,
    Ge,
    Le,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Located {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}

pub fn lex(source: &str) -> Result<Vec<Located>> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();
    let mut line = 1usize;
    let mut column = 1usize;

    fn bump(line: &mut usize, column: &mut usize, c: char) {
        if c == '\n' {
            *line += 1;
            *column = 1;
        } else {
            *column += 1;
        }
    }

    while let Some(&c) = chars.peek() {
        let start_line = line;
        let start_column = column;

        match c {
            ' ' | '\t' | '\r' => {
                chars.next();
                bump(&mut line, &mut column, c);
            }
            '\n' => {
                chars.next();
                bump(&mut line, &mut column, c);
            }
            '/' => {
                chars.next();
                bump(&mut line, &mut column, '/');
                match chars.peek() {
                    Some('/') => {
                        chars.next();
                        bump(&mut line, &mut column, '/');
                        while let Some(&next) = chars.peek() {
                            chars.next();
                            bump(&mut line, &mut column, next);
                            if next == '\n' {
                                break;
                            }
                        }
                    }
                    Some(_) => {
                        return Err(anyhow!(
                            "Unexpected '/' at line {start_line} column {start_column}"
                        ));
                    }
                    None => {
                        return Err(anyhow!(
                            "Unexpected '/' at line {start_line} column {start_column}"
                        ));
                    }
                }
            }
            '"' => {
                chars.next();
                bump(&mut line, &mut column, '"');
                let mut text = String::new();
                loop {
                    match chars.next() {
                        Some('"') => {
                            bump(&mut line, &mut column, '"');
                            break;
                        }
                        Some('\\') => {
                            bump(&mut line, &mut column, '\\');
                            match chars.next() {
                                Some(escaped) => {
                                    bump(&mut line, &mut column, escaped);
                                    match escaped {
                                        'n' => text.push('\n'),
                                        't' => text.push('\t'),
                                        '"' => text.push('"'),
                                        '\\' => text.push('\\'),
                                        other => text.push(other),
                                    }
                                }
                                None => {
                                    return Err(anyhow!(
                                        "Unterminated string at line {start_line} column {start_column}"
                                    ));
                                }
                            }
                        }
                        Some(ch) => {
                            bump(&mut line, &mut column, ch);
                            text.push(ch);
                        }
                        None => {
                            return Err(anyhow!(
                                "Unterminated string at line {start_line} column {start_column}"
                            ));
                        }
                    }
                }
                tokens.push(Located {
                    token: Token::String(text),
                    line: start_line,
                    column: start_column,
                });
            }
            '{' => {
                chars.next();
                bump(&mut line, &mut column, c);
                tokens.push(Located {
                    token: Token::LBrace,
                    line: start_line,
                    column: start_column,
                });
            }
            '}' => {
                chars.next();
                bump(&mut line, &mut column, c);
                tokens.push(Located {
                    token: Token::RBrace,
                    line: start_line,
                    column: start_column,
                });
            }
            '(' => {
                chars.next();
                bump(&mut line, &mut column, c);
                tokens.push(Located {
                    token: Token::LParen,
                    line: start_line,
                    column: start_column,
                });
            }
            ')' => {
                chars.next();
                bump(&mut line, &mut column, c);
                tokens.push(Located {
                    token: Token::RParen,
                    line: start_line,
                    column: start_column,
                });
            }
            ':' => {
                chars.next();
                bump(&mut line, &mut column, c);
                tokens.push(Located {
                    token: Token::Colon,
                    line: start_line,
                    column: start_column,
                });
            }
            ',' => {
                chars.next();
                bump(&mut line, &mut column, c);
                tokens.push(Located {
                    token: Token::Comma,
                    line: start_line,
                    column: start_column,
                });
            }
            '=' => {
                chars.next();
                bump(&mut line, &mut column, c);
                tokens.push(Located {
                    token: Token::Eq,
                    line: start_line,
                    column: start_column,
                });
            }
            '.' => {
                chars.next();
                bump(&mut line, &mut column, c);
                tokens.push(Located {
                    token: Token::Dot,
                    line: start_line,
                    column: start_column,
                });
            }
            '~' => {
                chars.next();
                bump(&mut line, &mut column, c);
                tokens.push(Located {
                    token: Token::Tilde,
                    line: start_line,
                    column: start_column,
                });
            }
            '<' => {
                chars.next();
                bump(&mut line, &mut column, c);
                if chars.peek() == Some(&'-') {
                    chars.next();
                    bump(&mut line, &mut column, '-');
                    if chars.peek() == Some(&'>') {
                        chars.next();
                        bump(&mut line, &mut column, '>');
                        tokens.push(Located {
                            token: Token::Arrow,
                            line: start_line,
                            column: start_column,
                        });
                    } else {
                        return Err(anyhow!(
                            "Expected '<->' at line {start_line} column {start_column}"
                        ));
                    }
                } else if chars.peek() == Some(&'=') {
                    chars.next();
                    bump(&mut line, &mut column, '=');
                    tokens.push(Located {
                        token: Token::Le,
                        line: start_line,
                        column: start_column,
                    });
                } else {
                    tokens.push(Located {
                        token: Token::Lt,
                        line: start_line,
                        column: start_column,
                    });
                }
            }
            '>' => {
                chars.next();
                bump(&mut line, &mut column, c);
                if chars.peek() == Some(&'=') {
                    chars.next();
                    bump(&mut line, &mut column, '=');
                    tokens.push(Located {
                        token: Token::Ge,
                        line: start_line,
                        column: start_column,
                    });
                } else {
                    tokens.push(Located {
                        token: Token::Gt,
                        line: start_line,
                        column: start_column,
                    });
                }
            }
            c if c.is_ascii_digit() => {
                let mut raw = String::new();
                while let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() || next == '.' {
                        raw.push(next);
                        chars.next();
                        bump(&mut line, &mut column, next);
                    } else {
                        break;
                    }
                }
                let value: f64 = raw.parse().map_err(|_| {
                    anyhow!("Invalid number '{raw}' at line {start_line} column {start_column}")
                })?;
                tokens.push(Located {
                    token: Token::Number(value),
                    line: start_line,
                    column: start_column,
                });
            }
            c if is_ident_start(c) => {
                let mut ident = String::new();
                while let Some(&next) = chars.peek() {
                    if is_ident_continue(next) {
                        ident.push(next);
                        chars.next();
                        bump(&mut line, &mut column, next);
                    } else {
                        break;
                    }
                }
                let token = keyword_or_ident(&ident);
                tokens.push(Located {
                    token,
                    line: start_line,
                    column: start_column,
                });
            }
            unexpected => {
                return Err(anyhow!(
                    "Unexpected character '{unexpected}' at line {start_line} column {start_column}"
                ));
            }
        }
    }

    tokens.push(Located {
        token: Token::Eof,
        line,
        column,
    });
    Ok(tokens)
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-'
}

fn keyword_or_ident(ident: &str) -> Token {
    match ident {
        "axes" => Token::Axes,
        "offer" => Token::Offer,
        "at" => Token::At,
        "feel" => Token::Feel,
        "grain" => Token::Grain,
        "along" => Token::Along,
        "toward" => Token::Toward,
        "keeping" => Token::Keeping,
        "blend" => Token::Blend,
        "with" => Token::With,
        "by" => Token::By,
        "without" => Token::Without,
        "speak" => Token::Speak,
        "shadow" => Token::Shadow,
        "of" => Token::Of,
        "when" => Token::When,
        "else" => Token::Else,
        "path" => Token::Path,
        "via" => Token::Via,
        "echo" => Token::Echo,
        "space" => Token::Space,
        _ => Token::Ident(ident.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_axis_arrow_and_string() {
        let tokens = lex(r#"warmth: "ice" <-> "ember""#).unwrap();
        assert!(matches!(tokens[0].token, Token::Ident(_)));
        assert!(matches!(tokens[1].token, Token::Colon));
        assert!(matches!(tokens[2].token, Token::String(_)));
        assert!(matches!(tokens[3].token, Token::Arrow));
    }
}
