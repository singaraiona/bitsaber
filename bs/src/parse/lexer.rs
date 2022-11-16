use crate::base::bs_ops::Op;
use crate::parse::span::Span;
use crate::result::*;
use std::iter::Peekable;
use std::ops::DerefMut;
use std::str::Chars;

/// Represents a primitive syntax token.
#[derive(Debug, Clone)]
pub enum Token<'a> {
    Binary,
    Comma,
    Comment,
    Def,
    Else,
    EOF,
    Extern,
    For,
    Ident(&'a str),
    If,
    In,
    LParen,
    RParen,
    LBox,
    RBox,
    LBrace,
    RBrace,
    Int64(i64),
    Float64(f64),
    Op(Op),
    Then,
    Unary,
    Var,
    Dot,
}

/// Defines a lexer which transforms an input `String` into
/// a `Token` stream.
pub struct Lexer<'a> {
    input: &'a str,
    chars: Box<Peekable<Chars<'a>>>,
    pos: usize,
    line_start: usize,
    line_number: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new `Lexer`, given its source `input`.
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input,
            chars: Box::new(input.chars().peekable()),
            pos: 0,
            line_start: 0,
            line_number: 1,
        }
    }

    /// Lexes and returns the next `Token` from the source code.
    pub fn next(&mut self) -> BSResult<Token<'a>> {
        let chars = self.chars.deref_mut();
        let src = self.input;
        let mut pos = self.pos;

        // Skip whitespaces
        loop {
            {
                let ch = chars.peek();

                if ch.is_none() {
                    self.pos = pos;
                    return ok(Token::EOF);
                }

                let c = ch.unwrap();

                if *c == '\n' {
                    self.line_start = pos + 1;
                    self.line_number += 1;
                }

                if !c.is_whitespace() {
                    break;
                }
            }

            chars.next();
            pos += 1;
        }

        let start = pos;
        let next = chars.next();

        if next.is_none() {
            return ok(Token::EOF);
        }

        pos += 1;

        let next_c = next.ok_or_else(|| BSError::ParseError {
            msg: "Unexpected EOF",
            span: None,
        })?;

        // Actually get the next token.
        let result = match next_c {
            '(' => ok(Token::LParen),
            ')' => ok(Token::RParen),
            '[' => ok(Token::LBox),
            ']' => ok(Token::RBox),
            '{' => ok(Token::LBrace),
            '}' => ok(Token::RBrace),

            ',' => ok(Token::Comma),

            '#' => {
                // Comment
                loop {
                    let ch = chars.next();
                    pos += 1;
                    if ch == Some('\n') {
                        break;
                    }
                }

                ok(Token::Comment)
            }

            '-' if self.pos + 1 == pos
                && chars
                    .peek()
                    .map(|c| c.is_digit(10) || c.is_whitespace())
                    .unwrap_or_else(|| false) =>
            {
                ok(Token::Op(Op::Sub))
            }

            '-' if self.pos + 1 != pos
                && chars
                    .peek()
                    .map(|c| c.is_whitespace())
                    .unwrap_or_else(|| false) =>
            {
                ok(Token::Op(Op::Sub))
            }

            '+' | '*' | '/' | '&' | '%' | '|' | '&' | '^' => {
                // Parse operator
                ok(Token::Op(Op::try_from(&src[start..pos]).map_err(|e| {
                    BSError::ParseError {
                        msg: "Invalid binary op",
                        span: Some(self.span()),
                    }
                })?))
            }

            '.' if chars
                .peek()
                .map(|c| !c.is_digit(10))
                .unwrap_or_else(|| false) =>
            {
                ok(Token::Dot)
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
                    pos += 1;
                }

                if is_float {
                    let s = &src[start..pos];
                    let v = s.parse::<f64>().map_err(|_| BSError::ParseError {
                        msg: "Invalid float literal",
                        span: Some(self.span()),
                    })?;

                    ok(Token::Float64(v))
                } else {
                    let s = &src[start..pos];
                    let v = s.parse::<i64>().map_err(|_| BSError::ParseError {
                        msg: "Invalid integer literal",
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
                    pos += 1;
                }

                match &src[start..pos] {
                    "def" => ok(Token::Def),
                    "extern" => ok(Token::Extern),
                    "if" => ok(Token::If),
                    "then" => ok(Token::Then),
                    "else" => ok(Token::Else),
                    "for" => ok(Token::For),
                    "in" => ok(Token::In),
                    "unary" => ok(Token::Unary),
                    "binary" => ok(Token::Binary),
                    "var" => ok(Token::Var),

                    ident => ok(Token::Ident(ident)),
                }
            }

            c => parse_error("Unexpected character", Some(self.span())),
        };

        // Update stored position, and return
        self.pos = pos;

        result
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn span(&self) -> Span {
        let line_start = self.line_start;
        let mut line_end = line_start;

        for c in self.input[line_start..].chars() {
            if c == '\n' {
                break;
            }

            line_end += 1;
        }

        Span::new(self.line_number, line_start, line_end, 0, 1)
    }
}
