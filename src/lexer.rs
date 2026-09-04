// Palimpsest Lexer
//
// Produces a token stream for the prose-shaped surface syntax. Two properties
// matter here and shape everything below:
//
//   1. Every bare word lexes as `Word`. Keywords are resolved positionally by
//      the parser, so a belief may legitimately be named `context`, `summary`,
//      or `from` without colliding with the grammar.
//   2. Layout is significant. Indentation produces Indent/Dedent tokens so a
//      block can be written with a colon instead of braces.

use crate::error::PalimpsestError;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Word(String),
    Str(String),
    Int(i64),
    Float(f64),
    /// An ISO-8601 calendar date, optionally with a time component.
    Date(String),
    /// A duration written without a space, such as `90d` or `300s`.
    Dur(u64),

    Dot,
    Comma,
    Colon,
    Semi,
    Question,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    Eq,
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
    Amp2,
    Pipe2,

    Newline,
    Indent,
    Dedent,
    Eof,
}

impl TokenKind {
    /// Human-facing rendering used in parse diagnostics.
    pub fn describe(&self) -> String {
        match self {
            TokenKind::Word(w) => format!("word `{}`", w),
            TokenKind::Str(s) => format!("text \"{}\"", s),
            TokenKind::Int(n) => format!("number {}", n),
            TokenKind::Float(n) => format!("number {}", n),
            TokenKind::Date(d) => format!("date {}", d),
            TokenKind::Dur(d) => format!("duration {}s", d),
            TokenKind::Dot => "`.`".into(),
            TokenKind::Comma => "`,`".into(),
            TokenKind::Colon => "`:`".into(),
            TokenKind::Semi => "`;`".into(),
            TokenKind::Question => "`?`".into(),
            TokenKind::LParen => "`(`".into(),
            TokenKind::RParen => "`)`".into(),
            TokenKind::LBrace => "`{`".into(),
            TokenKind::RBrace => "`}`".into(),
            TokenKind::LBracket => "`[`".into(),
            TokenKind::RBracket => "`]`".into(),
            TokenKind::Eq => "`=`".into(),
            TokenKind::EqEq => "`==`".into(),
            TokenKind::BangEq => "`!=`".into(),
            TokenKind::Lt => "`<`".into(),
            TokenKind::LtEq => "`<=`".into(),
            TokenKind::Gt => "`>`".into(),
            TokenKind::GtEq => "`>=`".into(),
            TokenKind::Plus => "`+`".into(),
            TokenKind::Minus => "`-`".into(),
            TokenKind::Star => "`*`".into(),
            TokenKind::Slash => "`/`".into(),
            TokenKind::Bang => "`!`".into(),
            TokenKind::Amp2 => "`&&`".into(),
            TokenKind::Pipe2 => "`||`".into(),
            TokenKind::Newline => "end of line".into(),
            TokenKind::Indent => "indented block".into(),
            TokenKind::Dedent => "end of indented block".into(),
            TokenKind::Eof => "end of file".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    indents: Vec<usize>,
    /// Depth of `(` and `[` nesting. Layout is ignored while inside them.
    bracket_depth: usize,
    at_line_start: bool,
    /// Whether any real content has been seen, used to set the baseline indent.
    seen_content: bool,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            indents: vec![0],
            bracket_depth: 0,
            at_line_start: true,
            seen_content: false,
            tokens: Vec::new(),
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_at(&self, offset: usize) -> Option<char> {
        self.chars.get(self.pos + offset).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(ch)
    }

    fn push(&mut self, kind: TokenKind, line: usize, column: usize) {
        self.tokens.push(Token { kind, line, column });
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, PalimpsestError> {
        while self.pos < self.chars.len() {
            if self.at_line_start && self.bracket_depth == 0 && self.handle_line_start()? {
                continue;
            }

            let Some(ch) = self.peek() else { break };

            if ch == '\n' {
                // Recorded before consuming, so the terminator belongs to the
                // line it ends rather than the one it starts.
                let (l, c) = (self.line, self.col);
                self.advance();
                if self.bracket_depth == 0 {
                    // Collapse runs of blank lines into a single terminator.
                    if !matches!(
                        self.tokens.last().map(|t| &t.kind),
                        None | Some(TokenKind::Newline) | Some(TokenKind::Indent)
                    ) {
                        self.push(TokenKind::Newline, l, c);
                    }
                    self.at_line_start = true;
                }
                continue;
            }

            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
                continue;
            }

            if self.at_comment() {
                self.skip_comment();
                continue;
            }

            self.lex_token()?;
        }

        let (l, c) = (self.line, self.col);
        if !matches!(
            self.tokens.last().map(|t| &t.kind),
            None | Some(TokenKind::Newline)
        ) {
            self.push(TokenKind::Newline, l, c);
        }
        while self.indents.len() > 1 {
            self.indents.pop();
            self.push(TokenKind::Dedent, l, c);
        }
        self.push(TokenKind::Eof, l, c);

        Ok(self.tokens)
    }

    fn at_comment(&self) -> bool {
        match self.peek() {
            Some('#') => true,
            Some('/') => self.peek_at(1) == Some('/'),
            _ => false,
        }
    }

    fn skip_comment(&mut self) {
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    /// Measures leading whitespace and emits Indent/Dedent. Returns true when
    /// the line held nothing but whitespace or a comment and should be skipped.
    fn handle_line_start(&mut self) -> Result<bool, PalimpsestError> {
        let mut width = 0usize;
        let mut scan = self.pos;
        while let Some(&c) = self.chars.get(scan) {
            match c {
                ' ' => width += 1,
                // A tab advances to the next multiple of 4, matching how the
                // same source looks in an editor with default settings.
                '\t' => width += 4 - (width % 4),
                '\r' => {}
                _ => break,
            }
            scan += 1;
        }

        // Blank line, comment-only line, or trailing whitespace at EOF: layout
        // is not meaningful, so consume the indentation and move on.
        let next = self.chars.get(scan).copied();
        let comment_only = matches!(next, Some('#'))
            || (next == Some('/') && self.chars.get(scan + 1).copied() == Some('/'));
        if next.is_none() || next == Some('\n') || comment_only {
            while self.pos < scan {
                self.advance();
            }
            if comment_only {
                self.skip_comment();
            }
            self.at_line_start = false;
            if self.peek() == Some('\n') {
                self.advance();
                self.at_line_start = true;
            }
            return Ok(true);
        }

        while self.pos < scan {
            self.advance();
        }
        self.at_line_start = false;

        // Whatever the first real line is indented by becomes the baseline, so
        // a uniformly indented program — a fenced block inside a markdown list
        // item, or a Rust raw string in a test — is not read as one big block.
        if !self.seen_content {
            self.seen_content = true;
            self.indents[0] = width;
            return Ok(false);
        }

        let current = *self.indents.last().unwrap();
        let (l, c) = (self.line, self.col);

        match width.cmp(&current) {
            std::cmp::Ordering::Greater => {
                self.indents.push(width);
                self.push(TokenKind::Indent, l, c);
            }
            std::cmp::Ordering::Less => {
                while *self.indents.last().unwrap() > width {
                    self.indents.pop();
                    self.push(TokenKind::Dedent, l, c);
                }
                if *self.indents.last().unwrap() != width {
                    return Err(PalimpsestError::ParseError {
                        line: l,
                        column: c,
                        message: format!(
                            "Indentation of {} spaces does not line up with any enclosing block",
                            width
                        ),
                    });
                }
            }
            std::cmp::Ordering::Equal => {}
        }

        Ok(false)
    }

    fn lex_token(&mut self) -> Result<(), PalimpsestError> {
        let (line, col) = (self.line, self.col);
        let ch = self.peek().unwrap();

        if ch == '"' {
            let s = self.lex_string(line, col)?;
            self.push(TokenKind::Str(s), line, col);
            return Ok(());
        }

        if ch.is_ascii_digit() {
            return self.lex_number(line, col);
        }

        if ch.is_ascii_alphabetic() || ch == '_' {
            let word = self.lex_word();
            self.push(TokenKind::Word(word), line, col);
            return Ok(());
        }

        self.advance();
        let kind = match ch {
            '.' => TokenKind::Dot,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semi,
            '?' => TokenKind::Question,
            '(' => {
                self.bracket_depth += 1;
                TokenKind::LParen
            }
            ')' => {
                self.bracket_depth = self.bracket_depth.saturating_sub(1);
                TokenKind::RParen
            }
            '[' => {
                self.bracket_depth += 1;
                TokenKind::LBracket
            }
            ']' => {
                self.bracket_depth = self.bracket_depth.saturating_sub(1);
                TokenKind::RBracket
            }
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '=' => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::EqEq
                } else {
                    TokenKind::Eq
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
                    TokenKind::Amp2
                } else {
                    return Err(PalimpsestError::ParseError {
                        line,
                        column: col,
                        message: "Stray `&`; write `and` to combine two conditions".into(),
                    });
                }
            }
            '|' => {
                if self.peek() == Some('|') {
                    self.advance();
                    TokenKind::Pipe2
                } else {
                    return Err(PalimpsestError::ParseError {
                        line,
                        column: col,
                        message: "Stray `|`; write `or` to combine two conditions".into(),
                    });
                }
            }
            other => {
                return Err(PalimpsestError::ParseError {
                    line,
                    column: col,
                    message: format!("Unexpected character `{}`", other),
                });
            }
        };

        self.push(kind, line, col);
        Ok(())
    }

    fn lex_string(&mut self, line: usize, col: usize) -> Result<String, PalimpsestError> {
        self.advance();
        let mut out = String::new();
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance();
                return Ok(out);
            }
            if ch == '\\' {
                self.advance();
                match self.advance() {
                    Some('n') => out.push('\n'),
                    Some('t') => out.push('\t'),
                    Some('r') => out.push('\r'),
                    Some('\\') => out.push('\\'),
                    Some('"') => out.push('"'),
                    Some(c) => out.push(c),
                    None => break,
                }
                continue;
            }
            out.push(ch);
            self.advance();
        }

        Err(PalimpsestError::ParseError {
            line,
            column: col,
            message: "Text is missing its closing quote".into(),
        })
    }

    fn lex_word(&mut self) -> String {
        let mut out = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                // A trailing `-` belongs to the next token unless a word
                // character follows it, so `a-b` is one word but `a - b` is not.
                if ch == '-'
                    && !self
                        .peek_at(1)
                        .map(|c| c.is_ascii_alphanumeric() || c == '_')
                        .unwrap_or(false)
                {
                    break;
                }
                out.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        out
    }

    /// Dates, durations, integers and floats all begin with a digit, so this
    /// disambiguates by looking ahead before consuming anything.
    fn lex_number(&mut self, line: usize, col: usize) -> Result<(), PalimpsestError> {
        if let Some(len) = self.match_date() {
            let text: String = self.chars[self.pos..self.pos + len].iter().collect();
            for _ in 0..len {
                self.advance();
            }
            self.push(TokenKind::Date(text), line, col);
            return Ok(());
        }

        let mut digits = String::new();
        let mut is_float = false;
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                digits.push(ch);
                self.advance();
            } else if ch == '.'
                && self
                    .peek_at(1)
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false)
                && !is_float
            {
                is_float = true;
                digits.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // `90d` is a duration; `90 days` is handled by the parser instead.
        if !is_float {
            if let Some(ch) = self.peek() {
                if ch.is_ascii_alphabetic() {
                    let save_pos = self.pos;
                    let save_line = self.line;
                    let save_col = self.col;
                    let unit = self.lex_word();
                    match duration_unit_secs(&unit) {
                        Some(mult) => {
                            let n: u64 = digits.parse().map_err(|_| PalimpsestError::ParseError {
                                line,
                                column: col,
                                message: format!("Number {} is too large", digits),
                            })?;
                            self.push(TokenKind::Dur(n * mult), line, col);
                            return Ok(());
                        }
                        None => {
                            self.pos = save_pos;
                            self.line = save_line;
                            self.col = save_col;
                        }
                    }
                }
            }
        }

        if is_float {
            let f = digits.parse::<f64>().map_err(|_| PalimpsestError::ParseError {
                line,
                column: col,
                message: format!("`{}` is not a valid number", digits),
            })?;
            self.push(TokenKind::Float(f), line, col);
        } else {
            let n = digits.parse::<i64>().map_err(|_| PalimpsestError::ParseError {
                line,
                column: col,
                message: format!("`{}` is not a valid whole number", digits),
            })?;
            self.push(TokenKind::Int(n), line, col);
        }

        Ok(())
    }

    /// Returns the character length of an ISO-8601 date at the cursor.
    fn match_date(&self) -> Option<usize> {
        let digit = |o: usize| self.peek_at(o).map(|c| c.is_ascii_digit()).unwrap_or(false);
        if !(digit(0) && digit(1) && digit(2) && digit(3)) {
            return None;
        }
        if self.peek_at(4) != Some('-') || !(digit(5) && digit(6)) {
            return None;
        }
        if self.peek_at(7) != Some('-') || !(digit(8) && digit(9)) {
            return None;
        }

        let mut len = 10;
        if self.peek_at(len) == Some('T') {
            let mut probe = len + 1;
            let d = |o: usize| self.peek_at(o).map(|c| c.is_ascii_digit()).unwrap_or(false);
            if d(probe) && d(probe + 1) && self.peek_at(probe + 2) == Some(':') && d(probe + 3) && d(probe + 4) {
                probe += 5;
                if self.peek_at(probe) == Some(':') && d(probe + 1) && d(probe + 2) {
                    probe += 3;
                }
                if self.peek_at(probe) == Some('Z') {
                    probe += 1;
                }
                len = probe;
            }
        }

        Some(len)
    }
}

pub fn duration_unit_secs(unit: &str) -> Option<u64> {
    match unit.to_ascii_lowercase().as_str() {
        "s" | "sec" | "secs" | "second" | "seconds" => Some(1),
        "m" | "min" | "mins" | "minute" | "minutes" => Some(60),
        "h" | "hr" | "hrs" | "hour" | "hours" => Some(3600),
        "d" | "day" | "days" => Some(86_400),
        "w" | "week" | "weeks" => Some(604_800),
        "mo" | "month" | "months" => Some(2_592_000),
        "y" | "yr" | "year" | "years" => Some(31_536_000),
        _ => None,
    }
}
