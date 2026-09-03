use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Integer(u64),
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
    Eof,
}

pub fn lex(source: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut characters = source.chars().peekable();

    while let Some(character) = characters.next() {
        match character {
            whitespace if whitespace.is_whitespace() => {}

            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),

            digit if digit.is_ascii_digit() => {
                let mut number = String::from(digit);

                while let Some(next) = characters.peek() {
                    if next.is_ascii_digit() {
                        number.push(*next);
                        characters.next();
                    } else {
                        break;
                    }
                }

                let value = number
                    .parse::<u64>()
                    .map_err(|_| anyhow!("Integer literal is too large: {number}"))?;

                tokens.push(Token::Integer(value));
            }

            unexpected => {
                return Err(anyhow!("Unexpected character in Trell source: '{unexpected}'"));
            }
        }
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}
