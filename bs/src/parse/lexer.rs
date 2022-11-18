use crate::base::binary::Op;
use crate::parse::span::Span;
use crate::result::*;
use std::iter::Peekable;
use std::ops::DerefMut;
use std::str::Chars;

/// Represents a primitive syntax token.
#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    Comment(&'a str),
    Tag(&'a str),
    Int64(i64),
    Float64(f64),
    Op(Op),
    LeftParen,
    RightParen,
    LeftSquare,
    RightSquare,
    LeftBrace,
    RightBrace,
    DoubleQuote,
    SingleQuote,
    Dollar,
    Ampersand,
    Percent,
    Comma,
    Colon,
    SemiColon,
    Period,
    Excl,
    Equal,
    Less,
    Greater,
    Minus,
    Plus,
    Asterisk,
    Slash,
    BackSlash,
    BackTick,
    Circ,
    Underscore,
    EOF,
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
                && chars
                    .peek()
                    .map(|c| !c.is_whitespace())
                    .unwrap_or_else(|| false) =>
            {
                ok(Token::Op(Op::Sub))
            }
            '-' if seen_whitespaces
                && chars
                    .peek()
                    .map(|c| c.is_whitespace())
                    .unwrap_or_else(|| false) =>
            {
                ok(Token::Op(Op::Sub))
            }
            '-' | '.' | '0'..='9' => {
                // Parse number literal
                let mut is_float = false;
                loop {
                    let ch = match chars.peek() {
                        Some(ch) => *ch,
                        None => return ok(Token::EOF),
                    };

                    if ch == '.' {
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

                ok(Token::Tag(&src[self.span.label_start..self.span.label_end]))
            }

            '(' => ok(Token::LeftParen),
            ')' => ok(Token::RightParen),
            '[' => ok(Token::LeftSquare),
            ']' => ok(Token::RightSquare),
            '{' => ok(Token::LeftBrace),
            '}' => ok(Token::RightBrace),
            ',' => ok(Token::Comma),
            ';' => ok(Token::SemiColon),
            '=' => ok(Token::Equal),
            '>' => ok(Token::Greater),
            '<' => ok(Token::Less),
            '!' => ok(Token::Excl),
            '+' => ok(Token::Plus),
            '*' => ok(Token::Asterisk),
            '/' => ok(Token::Slash),
            '\\' => ok(Token::BackSlash),
            '^' => ok(Token::Circ),
            '_' => ok(Token::Underscore),
            ':' => ok(Token::Colon),
            '.' => ok(Token::Period),
            '$' => ok(Token::Dollar),
            '&' => ok(Token::Ampersand),
            '%' => ok(Token::Percent),
            '\'' => ok(Token::SingleQuote),
            '"' => ok(Token::DoubleQuote),
            '`' => ok(Token::BackTick),
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
