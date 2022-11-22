use crate::parse::span::Span;
use crate::result::*;
use std::fmt;
use std::iter::Peekable;
use std::ops::DerefMut;
use std::str::Chars;

/// Represents a primitive syntax token.
#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    Comment(&'a str), // #asdfasdf
    Ident(&'a str),   // asdfasdf
    Bool(bool),       // true, false
    Int64(i64),       // 123
    Float64(f64),     // 123.123
    LeftParen,        // (
    RightParen,       // )
    LeftSquare,       // [
    RightSquare,      // ]
    LeftBrace,        // {
    RightBrace,       // }
    DoubleQuote,      // "
    SingleQuote,      // '
    Dollar,           // $
    Ampersand,        // &
    Percent,          // %
    Comma,            // ,
    Colon,            // :
    SemiColon,        // ;
    Period,           // .
    Excl,             // !
    Assign,           // =
    Equal,            // ==
    Less,             // <
    Greater,          // >
    LessOrEqual,      // <=
    GreaterOrEqual,   // >=
    NotEqual,         // !=
    Minus,            // -
    Plus,             // +
    Asterisk,         // *
    Slash,            // /
    BackSlash,        // \
    BackTick,         // `
    Circ,             // ^
    Underscore,       // _
    Def,              // def
    EOF,              // end of input
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Comment(s) => write!(f, "#{}", s),
            Token::Ident(s) => write!(f, "{}", s),
            Token::Bool(b) => write!(f, "{}", b),
            Token::Int64(i) => write!(f, "{}", i),
            Token::Float64(v) => write!(f, "{}", v),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftSquare => write!(f, "["),
            Token::RightSquare => write!(f, "]"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::DoubleQuote => write!(f, "\""),
            Token::SingleQuote => write!(f, "'"),
            Token::Dollar => write!(f, "$"),
            Token::Ampersand => write!(f, "&"),
            Token::Percent => write!(f, "%"),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::SemiColon => write!(f, ";"),
            Token::Period => write!(f, "."),
            Token::Excl => write!(f, "!"),
            Token::Assign => write!(f, "="),
            Token::Equal => write!(f, "="),
            Token::Less => write!(f, "<"),
            Token::Greater => write!(f, ">"),
            Token::LessOrEqual => write!(f, "<="),
            Token::GreaterOrEqual => write!(f, ">="),
            Token::NotEqual => write!(f, "!="),
            Token::Minus => write!(f, "-"),
            Token::Plus => write!(f, "+"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::BackSlash => write!(f, "\\"),
            Token::BackTick => write!(f, "`"),
            Token::Circ => write!(f, "^"),
            Token::Underscore => write!(f, "_"),
            Token::Def => write!(f, "def"),
            Token::EOF => write!(f, "EOF"),
        }
    }
}

/// Defines a lexer which transforms an input `String` into
/// a `Token` stream.
pub struct Lexer<'a> {
    input: &'a str,
    chars: Box<Peekable<Chars<'a>>>,
    span: Span,
}

impl<'a> Lexer<'a> {
    /// Creates a new `Lexer`, given its source `input`.
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input,
            chars: Box::new(input.chars().peekable()),
            span: Default::default(),
        }
    }

    /// Lexes and returns the next `Token` from the source code.
    pub fn next(&mut self) -> BSResult<Token<'a>> {
        let chars = self.chars.deref_mut();
        let src = self.input;
        let mut seen_whitespaces = false;

        // Skip whitespaces
        loop {
            {
                let ch = chars.peek();

                if ch.is_none() {
                    return ok(Token::EOF);
                }

                let c = ch.unwrap();

                if !c.is_whitespace() {
                    if *c == '\n' {
                        self.span.line_end = self.span.label_end.saturating_sub(1);
                        self.span.line_start = self.span.label_end + 1;
                        self.span.line_number += 1;
                    }
                    break;
                }

                seen_whitespaces = true;
            }

            chars.next();
            self.span.label_end += 1;
        }

        self.span.label_start = self.span.label_end;
        let next = chars.next();

        if next.is_none() {
            return ok(Token::EOF);
        }

        self.span.label_end += 1;

        let next_c = next.ok_or_else(|| BSError::ParseError {
            msg: "Unexpected EOF",
            desc: "Expected a character".to_string(),
            span: Some(self.span),
        })?;

        match next_c {
            '(' => ok(Token::LeftParen),
            ')' => ok(Token::RightParen),
            '[' => ok(Token::LeftSquare),
            ']' => ok(Token::RightSquare),
            '{' => ok(Token::LeftBrace),
            '}' => ok(Token::RightBrace),
            ',' => ok(Token::Comma),
            ';' => ok(Token::SemiColon),
            '+' => ok(Token::Plus),
            '*' => ok(Token::Asterisk),
            '/' => ok(Token::Slash),
            '\\' => ok(Token::BackSlash),
            '^' => ok(Token::Circ),
            ':' => ok(Token::Colon),
            '$' => ok(Token::Dollar),
            '&' => ok(Token::Ampersand),
            '%' => ok(Token::Percent),
            '\'' => ok(Token::SingleQuote),
            '"' => ok(Token::DoubleQuote),
            '`' => ok(Token::BackTick),

            '#' => {
                // Comment
                loop {
                    let ch = chars.next();
                    if ch == Some('\n') {
                        break;
                    }
                    self.span.label_end += 1;
                }

                ok(Token::Comment(
                    &src[self.span.label_start..self.span.label_end],
                ))
            }

            '-' if !seen_whitespaces
                && self.span.label_start + 1 != self.span.label_end
                && chars
                    .peek()
                    .map(|c| !c.is_whitespace())
                    .unwrap_or_else(|| false) =>
            {
                ok(Token::Minus)
            }

            '-' if seen_whitespaces
                && chars
                    .peek()
                    .map(|c| c.is_whitespace())
                    .unwrap_or_else(|| false) =>
            {
                ok(Token::Minus)
            }

            '-' | '0'..='9' => {
                // Parse number literal
                let mut is_float = false;
                loop {
                    let ch = match chars.peek() {
                        Some(ch) => *ch,
                        None => return ok(Token::EOF),
                    };

                    if ch == '.' {
                        if is_float {
                            break;
                        }
                        is_float = true;
                    } else if ch == '-' {
                        break;
                    } else if !ch.is_digit(10) {
                        break;
                    }

                    chars.next();
                    self.span.label_end += 1;
                }

                if is_float {
                    let s = &src[self.span.label_start..self.span.label_end];
                    let v = s.parse::<f64>().map_err(|_| BSError::ParseError {
                        msg: "Invalid float literal",
                        desc: "Expected a valid float literal".to_string(),
                        span: Some(self.span()),
                    })?;

                    ok(Token::Float64(v))
                } else {
                    let s = &src[self.span.label_start..self.span.label_end];
                    let v = s.parse::<i64>().map_err(|_| BSError::ParseError {
                        msg: "Invalid integer literal",
                        desc: "Expected a valid integer literal".to_string(),
                        span: Some(self.span()),
                    })?;

                    ok(Token::Int64(v))
                }
            }

            '=' => {
                if chars.peek().map(|c| *c == '=').unwrap_or_else(|| false) {
                    chars.next();
                    self.span.label_end += 1;
                    ok(Token::Equal)
                } else {
                    ok(Token::Assign)
                }
            }

            '<' => {
                if chars.peek().map(|c| *c == '=').unwrap_or_else(|| false) {
                    chars.next();
                    self.span.label_end += 1;
                    ok(Token::LessOrEqual)
                } else {
                    ok(Token::Less)
                }
            }

            '>' => {
                if chars.peek().map(|c| *c == '=').unwrap_or_else(|| false) {
                    chars.next();
                    self.span.label_end += 1;
                    ok(Token::GreaterOrEqual)
                } else {
                    ok(Token::Greater)
                }
            }

            '!' => {
                if chars.peek().map(|c| *c == '=').unwrap_or_else(|| false) {
                    chars.next();
                    self.span.label_end += 1;
                    ok(Token::NotEqual)
                } else {
                    ok(Token::Excl)
                }
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                // Parse identifier
                loop {
                    let ch = match chars.peek() {
                        Some(ch) => *ch,
                        None => return ok(Token::EOF),
                    };

                    // A word-like identifier only contains underscores and alphanumeric characters.
                    if ch != '_' && !ch.is_alphanumeric() {
                        break;
                    }

                    chars.next();
                    self.span.label_end += 1;
                }

                match &src[self.span.label_start..self.span.label_end] {
                    "true" => ok(Token::Bool(true)),
                    "false" => ok(Token::Bool(false)),
                    "def" => ok(Token::Def),
                    ident => ok(Token::Ident(ident)),
                }
            }

            '.' => ok(Token::Period),

            _ => parse_error("Unexpected character", "".to_string(), Some(self.span())),
        }
    }

    pub fn span(&mut self) -> Span {
        // update span line end
        for c in self.input[self.span.line_end..].chars() {
            self.span.line_end += 1;
            if c == '\n' {
                break;
            }
        }

        self.span
    }
}
