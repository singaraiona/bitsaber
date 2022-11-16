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
        let mut label_end = self.span.label_end;

        // Skip whitespaces
        loop {
            {
                let ch = chars.peek();

                if ch.is_none() {
                    self.span.label_end = label_end;
                    return ok(Token::EOF);
                }

                let c = ch.unwrap();

                if *c == '\n' {
                    self.span.line_end = label_end.wrapping_sub(1);
                    self.span.line_start = label_end + 1;
                    self.span.line_number += 1;
                } else if !c.is_whitespace() {
                    break;
                }
            }

            chars.next();
            label_end += 1;
        }

        self.span.label_start = label_end;
        let next = chars.next();

        if next.is_none() {
            return ok(Token::EOF);
        }

        label_end += 1;

        let next_c = next.ok_or_else(|| BSError::ParseError {
            msg: "Unexpected EOF",
            desc: "Expected a character",
            span: Some(self.span),
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
                    if ch == Some('\n') {
                        break;
                    }
                    label_end += 1;
                }

                ok(Token::Comment)
            }

            '-' if self.span.label_end + 1 == label_end
                && chars
                    .peek()
                    .map(|c| c.is_digit(10) || c.is_whitespace())
                    .unwrap_or_else(|| false) =>
            {
                ok(Token::Op(Op::Sub))
            }

            '-' if self.span.label_end + 1 != label_end
                && chars
                    .peek()
                    .map(|c| c.is_whitespace())
                    .unwrap_or_else(|| false) =>
            {
                ok(Token::Op(Op::Sub))
            }

            '+' | '*' | '/' | '&' | '%' | '|' | '&' | '^' => {
                // Parse operator
                ok(Token::Op(
                    Op::try_from(&src[self.span.label_start..label_end]).map_err(|e| {
                        BSError::ParseError {
                            msg: "Invalid binary op",
                            desc: "Expected one of: +, -, *, /, %, &, |, ^",
                            span: Some(self.span()),
                        }
                    })?,
                ))
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
                    label_end += 1;
                }

                if is_float {
                    let s = &src[self.span.label_start..label_end];
                    let v = s.parse::<f64>().map_err(|_| BSError::ParseError {
                        msg: "Invalid float literal",
                        desc: "Expected a valid float literal",
                        span: Some(self.span()),
                    })?;

                    ok(Token::Float64(v))
                } else {
                    let s = &src[self.span.label_start..label_end];
                    let v = s.parse::<i64>().map_err(|_| BSError::ParseError {
                        msg: "Invalid integer literal",
                        desc: "Expected a valid integer literal",
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
                    label_end += 1;
                }

                match &src[self.span.label_start..label_end] {
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

            c => parse_error("Unexpected character", "", Some(self.span())),
        };

        // Update stored position, and return
        self.span.label_end = label_end;

        result
    }

    pub fn pos(&self) -> usize {
        self.span.label_end
    }

    pub fn span(&mut self) -> Span {
        // update span line end
        if let Some(end) = self.chars.deref_mut().position(|c| c == '\n') {
            self.span.line_end = self.span.line_start + end;
        }

        self.span.line_end = std::cmp::max(self.span.line_end, self.span.label_end);
        self.span
    }
}
