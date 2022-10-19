use crate::parse::ast::*;
use crate::parse::lexer::{Lexer, Token};
use crate::result::*;
use std::collections::HashMap;
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

    /// Parses the prototype of a function, whether external or user-defined.
    fn parse_prototype(&mut self) -> BSResult<Prototype> {
        // let (id, is_operator, precedence) = match self.curr() {
        //     Ident(id) => {
        //         self.advance()?;

        //         (id, false, 0)
        //     }

        //     Binary => {
        //         self.advance()?;

        //         let op = match self.curr() {
        //             Op(ch) => ch,
        //             _ => return Err("Expected operator in custom operator declaration."),
        //         };

        //         self.advance()?;

        //         let mut name = String::from("binary");

        //         name.push(op);

        //         let prec = if let Number(prec) = self.curr() {
        //             self.advance()?;

        //             prec as usize
        //         } else {
        //             0
        //         };

        //         self.prec.insert(op, prec as i32);

        //         (name, true, prec)
        //     }

        //     Unary => {
        //         self.advance()?;

        //         let op = match self.curr() {
        //             Op(ch) => ch,
        //             _ => return Err("Expected operator in custom operator declaration."),
        //         };

        //         let mut name = String::from("unary");

        //         name.push(op);

        //         self.advance()?;

        //         (name, true, 0)
        //     }

        //     _ => return Err("Expected identifier in prototype declaration."),
        // };

        // match self.curr() {
        //     LParen => (),
        //     _ => return Err("Expected '(' character in prototype declaration."),
        // }

        // self.advance()?;

        // if let RParen = self.curr() {
        //     self.advance();

        //     return Ok(Prototype {
        //         name: id,
        //         args: vec![],
        //         is_op: is_operator,
        //         prec: precedence,
        //     });
        // }

        // let mut args = vec![];

        // loop {
        //     match self.curr() {
        //         Ident(name) => args.push(name),
        //         _ => return Err("Expected identifier in parameter declaration."),
        //     }

        //     self.advance()?;

        //     match self.curr() {
        //         RParen => {
        //             self.advance();
        //             break;
        //         }
        //         Comma => {
        //             self.advance();
        //         }
        //         _ => return Err("Expected ',' or ')' character in prototype declaration."),
        //     }
        // }

        // Ok(Prototype {
        //     name: id,
        //     args,
        //     is_op: is_operator,
        //     prec: precedence,
        // })

        todo!()
    }

    /// Parses a user-defined function.
    fn parse_def(&mut self) -> BSResult<Function> {
        // Eat 'def' keyword
        // self.pos += 1;

        // // Parse signature of function
        // let proto = self.parse_prototype()?;

        // // Parse body of function
        // let body = self.parse_expr()?;

        // // Return new function
        // Ok(Function {
        //     prototype: proto,
        //     body: Some(body),
        //     is_anon: false,
        // })

        todo!()
    }

    /// Parses an external function declaration.
    fn parse_extern(&mut self) -> BSResult<Function> {
        // Eat 'extern' keyword
        // self.pos += 1;

        // // Parse signature of extern function
        // let proto = self.parse_prototype()?;

        // Ok(Function {
        //     prototype: proto,
        //     body: None,
        //     is_anon: false,
        // })

        todo!()
    }

    /// Parses a literal number.
    fn parse_nb_expr(&mut self) -> BSResult<Expr> {
        let r = match self.curr {
            I64(v) => ok(Expr::I64(v)),
            F64(v) => ok(Expr::F64(v)),
            _ => parse_error("Expected number literal.", self.lexer.pos()),
        };

        match r {
            BSResult::Ok(_) => {
                self.advance()?;
                r
            }
            _ => r,
        }
    }

    /// Parses an expression enclosed in parenthesis.
    fn parse_paren_expr(&mut self) -> BSResult<Expr> {
        // match self.current()? {
        //     LParen => (),
        //     _ => return Err("Expected '(' character at start of parenthesized expression."),
        // }

        // self.advance()?;

        // let expr = self.parse_expr()?;

        // match self.current()? {
        //     RParen => (),
        //     _ => return Err("Expected ')' character at end of parenthesized expression."),
        // }

        // self.advance();

        // Ok(expr)

        todo!()
    }

    /// Parses an expression that starts with an identifier (either a variable or a function call).
    fn parse_id_expr(&mut self) -> BSResult<Expr> {
        // let id = match self.curr() {
        //     Ident(id) => id,
        //     _ => return Err("Expected identifier."),
        // };

        // if self.advance().is_err() {
        //     return Ok(Expr::Variable(id));
        // }

        // match self.curr() {
        //     LParen => {
        //         self.advance()?;

        //         if let RParen = self.curr() {
        //             return Ok(Expr::Call {
        //                 fn_name: id,
        //                 args: vec![],
        //             });
        //         }

        //         let mut args = vec![];

        //         loop {
        //             args.push(self.parse_expr()?);

        //             match self.current()? {
        //                 Comma => (),
        //                 RParen => break,
        //                 _ => return Err("Expected ',' character in function call."),
        //             }

        //             self.advance()?;
        //         }

        //         self.advance();

        //         Ok(Expr::Call { fn_name: id, args })
        //     }

        //     _ => Ok(Expr::Variable(id)),
        // }

        todo!()
    }

    /// Parses a conditional if..then..else expression.
    fn parse_conditional_expr(&mut self) -> BSResult<Expr> {
        // eat 'if' token
        // self.advance()?;

        // let cond = self.parse_expr()?;

        // // eat 'then' token
        // match self.current() {
        //     Ok(Then) => self.advance()?,
        //     _ => return Err("Expected 'then' keyword."),
        // }

        // let then = self.parse_expr()?;

        // // eat 'else' token
        // match self.current() {
        //     Ok(Else) => self.advance()?,
        //     _ => return Err("Expected 'else' keyword."),
        // }

        // let otherwise = self.parse_expr()?;

        // Ok(Expr::Conditional {
        //     cond: Box::new(cond),
        //     consequence: Box::new(then),
        //     alternative: Box::new(otherwise),
        // })

        todo!()
    }

    /// Parses a loop for..in.. expression.
    fn parse_for_expr(&mut self) -> BSResult<Expr> {
        // eat 'for' token
        // self.advance()?;

        // let name = match self.curr() {
        //     Ident(n) => n,
        //     _ => return Err("Expected identifier in for loop."),
        // };

        // // eat identifier
        // self.advance()?;

        // // eat '=' token
        // match self.curr() {
        //     Op('=') => self.advance()?,
        //     _ => return Err("Expected '=' character in for loop."),
        // }

        // let start = self.parse_expr()?;

        // // eat ',' token
        // match self.current()? {
        //     Comma => self.advance()?,
        //     _ => return Err("Expected ',' character in for loop."),
        // }

        // let end = self.parse_expr()?;

        // // parse (optional) step expression
        // let step = match self.current()? {
        //     Comma => {
        //         self.advance()?;

        //         Some(self.parse_expr()?)
        //     }

        //     _ => None,
        // };

        // // eat 'in' token
        // match self.current()? {
        //     In => self.advance()?,
        //     _ => return Err("Expected 'in' keyword in for loop."),
        // }

        // let body = self.parse_expr()?;

        // Ok(Expr::For {
        //     var_name: name,
        //     start: Box::new(start),
        //     end: Box::new(end),
        //     step: step.map(Box::new),
        //     body: Box::new(body),
        // })

        todo!()
    }

    /// Parses a var..in expression.
    fn parse_var_expr(&mut self) -> BSResult<Expr> {
        // eat 'var' token
        // self.advance()?;

        // let mut variables = Vec::new();

        // // parse variables
        // loop {
        //     let name = match self.curr() {
        //         Ident(name) => name,
        //         _ => return Err("Expected identifier in 'var..in' declaration."),
        //     };

        //     self.advance()?;

        //     // read (optional) initializer
        //     let initializer = match self.curr() {
        //         Op('=') => Some({
        //             self.advance()?;
        //             self.parse_expr()?
        //         }),

        //         _ => None,
        //     };

        //     variables.push((name, initializer));

        //     match self.curr() {
        //         Comma => {
        //             self.advance()?;
        //         }
        //         In => {
        //             self.advance()?;
        //             break;
        //         }
        //         _ => return Err("Expected comma or 'in' keyword in variable declaration."),
        //     }
        // }

        // // parse body
        // let body = self.parse_expr()?;

        // Ok(Expr::VarIn {
        //     variables,
        //     body: Box::new(body),
        // })

        todo!()
    }

    fn parse_vec_literal(&mut self) -> BSResult<Expr> {
        let mut vec_i64 = vec![];
        let mut vec_f64 = vec![];

        loop {
            self.advance()?;
            match &self.curr {
                I64(v) => {
                    if vec_f64.len() == 0 {
                        vec_i64.push(*v);
                    } else {
                        vec_f64.push(*v as f64);
                    }
                }
                F64(v) => {
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
                        "Expected int or float in vector literal.",
                        self.lexer.pos(),
                    )
                }
            }
        }

        if vec_i64.len() == 0 {
            ok(Expr::VecF64(vec_f64))
        } else {
            ok(Expr::VecI64(vec_i64))
        }
    }

    fn parse_unary_expr(&mut self) -> BSResult<Expr> {
        // let op = match self.current()? {
        //     Op(ch) => {
        //         self.advance()?;
        //         ch
        //     }
        //     _ => return self.parse_primary(),
        // };

        // let mut name = String::from("unary");

        // name.push(op);

        // Ok(Expr::Call {
        //     fn_name: name,
        //     args: vec![self.parse_unary_expr()?],
        // })

        match self.curr {
            I64(_) => self.parse_nb_expr(),
            F64(_) => self.parse_nb_expr(),
            LBox => self.parse_vec_literal(),
            LParen => self.parse_paren_expr(),
            _ => parse_error(
                "Expected int, float, vector or parenthesized expression.",
                self.lexer.pos(),
            ),
        }
    }

    /// Parses a binary expression, given its left-hand expression.
    fn parse_binary_expr(&mut self, mut lhs: Expr) -> BSResult<Expr> {
        // loop {
        let op = match self.curr {
            Op(op) => op,
            _ => return parse_error("Invalid operator.", self.lexer.pos()),
        };

        self.advance()?;

        let mut rhs = self.parse_unary_expr()?;

        self.advance()?;

        ok(Expr::Binary {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
        // }
    }

    fn parse_expr(&mut self) -> BSResult<Expr> {
        match self.parse_unary_expr() {
            BSResult::Ok(left) => self.parse_binary_expr(left),
            err => err,
        }
    }

    /// Parses a top-level expression and makes an anonymous function out of it,
    /// for easier compilation.
    fn parse_toplevel_expr(&mut self) -> BSResult<Function> {
        let expr = match self.curr {
            EOF => ok(Expr::Null),
            Ident(_) => self.parse_id_expr(),
            If => self.parse_conditional_expr(),
            For => self.parse_for_expr(),
            Var => self.parse_var_expr(),
            // Def => self.parse_def(),
            // Extern => self.parse_extern(),
            _ => self.parse_expr(),
        }?;

        self.advance()?;

        ok(Function {
            prototype: Prototype {
                name: "anonymous".to_string(),
                args: vec![],
                is_op: false,
                prec: 0,
            },
            body: Some(expr),
            is_anon: true,
        })
    }

    /// Parses the content of the parser.
    pub fn parse(&mut self) -> BSResult<Function> {
        self.advance()?;

        match self.parse_toplevel_expr() {
            BSResult::Ok(expr) => {
                if !self.at_end() {
                    parse_error(
                        "Unexpected token after parsed expression.",
                        self.lexer.pos(),
                    )
                } else {
                    ok(expr)
                }
            }

            err => err,
        }
    }
}
