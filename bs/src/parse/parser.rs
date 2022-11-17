use crate::parse::ast::*;
use crate::parse::lexer::{Lexer, Token};
use crate::result::*;
use Token::*;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr: Token<'a>,
}

#[allow(unused_must_use)]
#[allow(unused_must_use)]
impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(input);

        Parser {
            lexer,
            curr: Token::EOF,
        }
    }

    /// Advances the position, and returns an empty `Result` whose error
    /// indicates that the end of the file has been unexpectedly reached.
    /// This allows to use the `self.advance()?;` syntax.
    fn advance(&mut self) -> BSResult<()> {
        let token = self.lexer.next()?;
        self.curr = token;
        ok(())
    }

    /// Returns a value indicating whether or not the `Parser`
    /// has reached the end of the input.
    fn at_end(&self) -> bool {
        match self.curr {
            Token::EOF => true,
            _ => false,
        }
    }

    /// Parses a literal number.
    fn parse_nb_expr(&mut self) -> BSResult<Expr> {
        let r = match self.curr {
            Int64(v) => ok(Expr::new(ExprBody::Int64(v), Some(self.lexer.span()))),
            Float64(v) => ok(Expr::new(ExprBody::Float64(v), Some(self.lexer.span()))),
            _ => parse_error(
                "Invalid literal",
                "Expected number literal here",
                Some(self.lexer.span()),
            ),
        };

        match r {
            BSResult::Ok(_) => {
                self.advance()?;
                r
            }
            _ => r,
        }
    }

    /// Parses an expression that starts with an identifier (either a variable or a function call).
    fn parse_id_expr(&mut self) -> BSResult<Expr> {
        let id = match self.curr {
            Ident(id) => id,
            _ => {
                return parse_error(
                    "Expected identifier",
                    "Expected identifier here",
                    Some(self.lexer.span()),
                )
            }
        };

        let span = self.lexer.span();
        self.advance()?;

        match self.curr {
            Assign => {
                self.advance()?;
                let rhs = self.parse_expr()?;

                ok(Expr::new(
                    ExprBody::Assign {
                        variable: id.to_string(),
                        body: Box::new(rhs),
                    },
                    Some(span),
                ))
            }
            _ => ok(Expr::new(ExprBody::Variable(id.to_string()), Some(span))),
        }
    }

    fn parse_vec_literal(&mut self) -> BSResult<Expr> {
        let mut vec_i64 = vec![];
        let mut vec_f64 = vec![];

        loop {
            self.advance()?;

            match &self.curr {
                Int64(v) => {
                    if vec_f64.len() == 0 {
                        vec_i64.push(*v);
                    } else {
                        vec_f64.push(*v as f64);
                    }
                }
                Float64(v) => {
                    if vec_i64.len() == 0 {
                        vec_f64.push(*v);
                    } else {
                        for v in vec_i64.drain(..) {
                            vec_f64.push(v as f64);
                        }
                        vec_f64.push(*v as f64);
                    }
                }
                Comma => {}
                RBox => break,
                _ => {
                    return parse_error(
                        "Invalid number literal",
                        "Expected int or float in vector literal.",
                        Some(self.lexer.span()),
                    )
                }
            }
        }

        self.advance()?;

        if vec_i64.len() == 0 {
            ok(Expr::new(
                ExprBody::VecFloat64(vec_f64),
                Some(self.lexer.span()),
            ))
        } else {
            ok(Expr::new(
                ExprBody::VecInt64(vec_i64),
                Some(self.lexer.span()),
            ))
        }
    }

    fn parse_dot_expr(&mut self) -> BSResult<Expr> {
        todo!()
    }

    fn parse_unary_expr(&mut self) -> BSResult<Expr> {
        match self.curr {
            Int64(_) => self.parse_nb_expr(),
            Float64(_) => self.parse_nb_expr(),
            LBox => self.parse_vec_literal(),
            Ident(_) => self.parse_id_expr(),
            _ => parse_error(
                "Invalid expression",
                "Expected int, float, vector or parenthesized expression here",
                Some(self.lexer.span()),
            ),
        }
    }

    fn parse_binary_expr(&mut self, lhs: Expr) -> BSResult<Expr> {
        if self.at_end() {
            return ok(lhs);
        }
        let span = Some(self.lexer.span());
        match self.curr {
            Op(op) => {
                self.advance()?;
                let rhs = self.parse_unary_expr()?;
                ok(Expr::new(
                    ExprBody::Binary {
                        op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    },
                    span,
                ))
            }
            Dot => {
                self.advance()?;
                let rhs = self.parse_dot_expr()?;
                ok(Expr::new(
                    ExprBody::Dot {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    },
                    Some(self.lexer.span()),
                ))
            }

            _ => ok(lhs),
        }
    }

    fn parse_expr(&mut self) -> BSResult<Expr> {
        match self.parse_unary_expr() {
            BSResult::Ok(left) => self.parse_binary_expr(left),
            err => err,
        }
    }

    fn parse_program(&mut self) -> BSResult<Function> {
        let mut body = vec![];

        while !self.at_end() {
            let e = match self.curr {
                Dot => self.parse_dot_expr(),
                _ => self.parse_expr(),
            }?;

            body.push(e);

            if self.curr == Semi {
                self.advance()?;
                continue;
            }
        }

        ok(Function {
            prototype: Prototype {
                name: "anonymous".to_string(),
                args: vec![],
                is_op: false,
                prec: 0,
            },
            body: body,
            is_anon: true,
            span: Some(self.lexer.span()),
        })
    }

    pub fn parse(&mut self) -> BSResult<Function> {
        self.advance()?;
        match self.parse_program() {
            BSResult::Ok(expr) => {
                if !self.at_end() {
                    parse_error(
                        "Unexpected token after parsed expression.",
                        "",
                        Some(self.lexer.span()),
                    )
                } else {
                    ok(expr)
                }
            }

            err => err,
        }
    }
}
