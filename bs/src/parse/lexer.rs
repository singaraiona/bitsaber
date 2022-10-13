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
    I64(i64),
    F64(f64),
    Op(char),
    Then,
    Unary,
    Var,
}

/// Defines a lexer which transforms an input `String` into
/// a `Token` stream.
pub struct Lexer<'a> {
    input: &'a str,
    chars: Box<Peekable<Chars<'a>>>,
    pos: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new `Lexer`, given its source `input`.
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input,
            chars: Box::new(input.chars().peekable()),
            pos: 0,
        }
    }

    /// Lexes and returns the next `Token` from the source code.
    pub fn next(&mut self) -> BSResult<Token<'a>> {
        let chars = self.chars.deref_mut();
        let src = self.input;

        let mut pos = self.pos;

        // Skip whitespaces
        loop {
            // Note: the following lines are in their own scope to
            // limit how long 'chars' is borrowed, and in order to allow
            // it to be borrowed again in the loop by 'chars.next()'.
            {
                let ch = chars.peek();

                if ch.is_none() {
                    self.pos = pos;
                    return ok(Token::EOF);
                }

                if !ch.unwrap().is_whitespace() {
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
            pos: pos,
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
                        pos: start,
                    })?;

                    ok(Token::F64(v))
                } else {
                    let s = &src[start..pos];
                    let v = s.parse::<i64>().map_err(|_| BSError::ParseError {
                        msg: "Invalid integer literal",
                        pos: start,
                    })?;

                    ok(Token::I64(v))
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

            op => {
                // Parse operator
                ok(Token::Op(op))
            }
        };

        // Update stored position, and return
        self.pos = pos;

        result
    }

    pub fn pos(&self) -> usize {
        self.pos
    }
}
