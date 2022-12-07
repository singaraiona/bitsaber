use crate::parse::ast::*;
use crate::parse::lexer::{Lexer, Token};
use crate::parse::span::Span;
use crate::result::*;
use ffi::Type as BSType;
use Token::*;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr: Token<'a>,
    top_level: bool,
}

#[allow(unused_must_use)]
#[allow(unused_must_use)]
impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(input);

        Parser { lexer, curr: Token::EOF, top_level: true }
    }

    fn span(&mut self) -> Option<Span> { Some(self.lexer.span()) }

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
            Token::EOF | Token::RightBrace => true,
            _ => false,
        }
    }

    fn at_term(&self) -> bool {
        match self.curr {
            Token::EOF | Token::RightParen | Token::RightBrace | Token::RightSquare => true,
            _ => false,
        }
    }

    fn expect(&mut self, expected: Token<'a>) -> BSResult<Token> {
        if self.curr == expected {
            let tok = self.curr.clone();
            self.advance()?;
            ok(tok)
        } else {
            parse_error("Invalid syntax", format!("Expected '{}' here", expected), Some(self.lexer.span()))
        }
    }

    fn parse_type(&mut self) -> BSResult<BSType> {
        match self.curr {
            Token::Ident(name) => match BSType::try_from(name) {
                Ok(ty) => {
                    self.advance()?;
                    ok(ty)
                }
                Err(_) => parse_error("Invalid type", format!("'{}' is not a valid type", name), self.span()),
            },
            _ => parse_error("Invalid syntax", "Expected type name here".into(), self.span()),
        }
    }

    /// Parses an expression that starts with an identifier (either a variable or a function call).
    fn parse_ident_expr(&mut self) -> BSResult<Expr> {
        let name = match self.curr {
            Ident(name) => name,
            _ => {
                return parse_error(
                    "Expected identifier",
                    "Expected identifier here".to_string(),
                    Some(self.lexer.span()),
                )
            }
        };

        let span = self.lexer.span();

        self.advance()?;

        match self.curr {
            LeftParen => {
                self.advance()?;
                let mut args = vec![];
                self.top_level = false;

                while self.curr != RightParen {
                    let arg = self.parse_expr()?;
                    args.push(arg);

                    if self.curr == Comma {
                        self.advance()?;
                        continue;
                    } else {
                        break;
                    }
                }

                self.expect(RightParen)?;
                self.top_level = true;

                ok(Expr::new(ExprBody::Call { name: name.to_string(), args }, Some(span)))
            }
            _ => ok(Expr::new(ExprBody::Variable(name.to_string()), Some(span))),
        }
    }

    fn parse_cond_expr(&mut self) -> BSResult<Expr> {
        self.advance()?;

        let cond = self.parse_expr()?;
        self.expect(LeftBrace)?;

        let then = self.parse_exprs()?;
        self.expect(RightBrace)?;

        let mut els = vec![];
        if self.curr == Else {
            self.advance()?;
            self.expect(LeftBrace)?;
            els = self.parse_exprs()?;
            self.expect(RightBrace)?;
        }

        ok(Expr::new(ExprBody::Cond { cond: Box::new(cond), cons: then, altr: els }, Some(self.lexer.span())))
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
                RightSquare => break,
                _ => {
                    return parse_error(
                        "Invalid number literal",
                        "Expected int or float in vector literal here".to_string(),
                        self.span(),
                    )
                }
            }
        }

        self.advance()?;

        if vec_i64.is_empty() {
            ok(Expr::new(ExprBody::VecFloat64(vec_f64), self.span()))
        } else {
            ok(Expr::new(ExprBody::VecInt64(vec_i64), self.span()))
        }
    }

    fn parse_dot_expr(&mut self) -> BSResult<Expr> {
        match self.curr {
            Period => {
                self.advance()?;
                self.expect(LeftParen)?;
                self.expect(RightParen)?;
                ok(Expr::new(ExprBody::Null, self.span()))
            }
            _ => {
                return parse_error(
                    "Expected combinator",
                    "Consider using one of map, filter, fold, zip.. etc. here".to_string(),
                    self.span(),
                );
            }
        }
    }

    fn parse_unary_expr(&mut self) -> BSResult<Expr> {
        match self.curr {
            Null => {
                self.advance()?;
                ok(Expr::new(ExprBody::Null, self.span()))
            }
            Bool(v) => {
                self.advance()?;
                ok(Expr::new(ExprBody::Bool(v), self.span()))
            }
            Int64(v) => {
                self.advance()?;
                ok(Expr::new(ExprBody::Int64(v), self.span()))
            }
            Float64(v) => {
                self.advance()?;
                ok(Expr::new(ExprBody::Float64(v), self.span()))
            }
            LeftSquare => self.parse_vec_literal(),
            Ident(_) => self.parse_ident_expr(),
            If => self.parse_cond_expr(),
            LeftParen => {
                self.advance()?;
                let expr = self.parse_expr()?;
                self.expect(RightParen)?;
                ok(expr)
            }
            _ => parse_error(
                "Invalid expression",
                "Expected int, float, vector or parenthesized expression here".to_string(),
                self.span(),
            ),
        }
    }

    fn parse_binary_expr(&mut self, lhs: Expr) -> BSResult<Expr> {
        if self.at_end() {
            return ok(lhs);
        }

        let span = Some(self.lexer.span());

        match self.curr {
            Plus | Minus | Asterisk | Slash | Ampersand | Equal | Less | Greater | LessOrEqual | GreaterOrEqual
            | NotEqual => {
                let op = match self.curr {
                    Plus => BinaryOp::Add,
                    Minus => BinaryOp::Sub,
                    Asterisk => BinaryOp::Mul,
                    Slash => BinaryOp::Div,
                    Ampersand => BinaryOp::And,
                    Equal => BinaryOp::Equal,
                    Less => BinaryOp::Less,
                    Greater => BinaryOp::Greater,
                    LessOrEqual => BinaryOp::LessOrEqual,
                    GreaterOrEqual => BinaryOp::GreaterOrEqual,
                    NotEqual => BinaryOp::NotEqual,

                    _ => {
                        return parse_error(
                            "Invalid binary operator",
                            format!("Expected one of: {}, {}, {}, {}' here", Plus, Minus, Asterisk, Slash),
                            self.span(),
                        )
                    }
                };
                self.advance()?;
                let rhs = self.parse_expr()?;
                ok(Expr::new(ExprBody::Binary { op, lhs: Box::new(lhs), rhs: Box::new(rhs) }, span))
            }

            Assign => {
                self.advance()?;
                let span = self.span();

                match lhs.body {
                    ExprBody::Variable(name) => {
                        let rhs = self.parse_expr()?;
                        ok(Expr::new(ExprBody::Assign { name, body: Box::new(rhs), global: self.top_level }, span))
                    }

                    _ => parse_error(
                        "Invalid assignment",
                        "Expected variable on the left hand side of the assignment".to_string(),
                        span,
                    ),
                }
            }

            Period => {
                self.advance()?;
                let rhs = self.parse_dot_expr()?;
                ok(Expr::new(ExprBody::Dot { lhs: Box::new(lhs), rhs: Box::new(rhs) }, Some(self.lexer.span())))
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

    fn parse_exprs(&mut self) -> BSResult<Vec<Expr>> {
        let mut exprs = vec![];

        while !self.at_end() {
            let e = match self.curr {
                Comment(_) => {
                    self.advance()?;
                    continue;
                }
                Period => self.parse_dot_expr(),
                _ => self.parse_expr(),
            }?;

            exprs.push(e);

            if self.curr == SemiColon {
                self.advance()?;
                if self.at_term() {
                    exprs.push(Expr::new(ExprBody::Null, Some(self.lexer.span())));
                }
                continue;
            }
        }

        if exprs.is_empty() {
            exprs.push(Expr::new(ExprBody::Null, Some(self.lexer.span())));
        }

        ok(exprs)
    }

    fn parse_function_proto(&mut self) -> BSResult<Function> {
        self.advance()?;
        let name = match self.curr {
            Token::Ident(name) => {
                self.advance()?; // eat ident
                ok(name)
            }
            _ => parse_error("Invalid syntax", "Expected identifier".into(), self.span()),
        }?;

        self.expect(Bar)?;
        let mut args = vec![];
        while self.curr != Bar {
            let arg_name = match self.curr {
                Token::Ident(name) => name,
                _ => return parse_error("Invalid syntax", "Expected identifier here".to_string(), self.span()),
            };
            self.advance()?;
            self.expect(Colon)?;
            let ty = self.parse_type()?;
            args.push((arg_name.to_string(), ty));
            if self.curr == Comma {
                self.advance()?;
            }
        }

        self.expect(Bar)?;

        ok(Function { name: name.into(), args, body: vec![], topl: false })
    }

    fn parse_function_body(&mut self, proto: Function) -> BSResult<Function> {
        self.expect(Token::LeftBrace)?;
        let body = self.parse_exprs()?;
        self.expect(Token::RightBrace)?;

        ok(Function { name: proto.name, args: proto.args, body, topl: false })
    }

    pub fn parse_module(&mut self) -> BSResult<Vec<Function>> {
        let mut functions = vec![];

        while !self.at_end() {
            let e = match self.curr {
                Comment(_) => {
                    self.advance()?;
                    continue;
                }
                Def => {
                    self.top_level = false;
                    let proto = self.parse_function_proto()?;
                    let func = self.parse_function_body(proto)?;
                    self.top_level = true;
                    ok(func)
                }
                Extern => self.parse_function_proto(),
                _ => {
                    let body = self.parse_exprs()?;
                    ok(Function { name: "top-level".into(), args: vec![], body: body, topl: true })
                }
            }?;

            functions.push(e);
        }

        ok(functions)
    }

    pub fn parse(&mut self) -> BSResult<Vec<Function>> {
        self.advance()?;

        match self.parse_module() {
            BSResult::Ok(expr) => {
                if !self.at_end() {
                    parse_error("Unexpected token after parsed expression.", "".to_string(), Some(self.lexer.span()))
                } else {
                    ok(expr)
                }
            }

            err => err,
        }
    }
}
