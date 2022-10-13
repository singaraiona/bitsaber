use crate::parse::ast::*;
use crate::parse::lexer::{Lexer, Token};
use crate::result::*;
use std::collections::HashMap;
use Token::*;

pub struct Parser<'a> {
    input: &'a str,
    lexer: Lexer<'a>,
    pos: usize,
    curr: Token,
}

#[allow(unused_must_use)]
#[allow(unused_must_use)]
impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(input);

        Parser {
            input,
            lexer,
            pos: 0,
            curr: Token::Start,
        }
    }

    /// Returns the current `Token`, or an error that
    /// indicates that the end of the file has been unexpectedly reached if it is the case.
    fn current(&self) -> BSResult<Token> {
        ok(self.curr.clone())
    }

    /// Advances the position, and returns an empty `Result` whose error
    /// indicates that the end of the file has been unexpectedly reached.
    /// This allows to use the `self.advance()?;` syntax.
    fn advance(&mut self) -> BSResult<()> {
        // let npos = self.pos + 1;

        // self.pos = npos;

        // if npos < self.tokens.len() {
        //     Ok(())
        // } else {
        //     Err("Unexpected end of file.")
        // }

        todo!()
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

    /// Parses any expression.
    fn parse_expr(&mut self) -> BSResult<Expr> {
        // match self.parse_unary_expr() {
        //     Ok(left) => self.parse_binary_expr(0, left),
        //     err => err,
        // }

        todo!()
    }

    /// Parses a literal number.
    fn parse_nb_expr(&mut self) -> BSResult<Expr> {
        // Simply convert Token::Number to Expr::Number
        // match self.curr() {
        //     Number(nb) => {
        //         self.advance();
        //         Ok(Expr::Number(nb))
        //     }
        //     _ => Err("Expected number literal."),
        // }

        todo!()
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

    /// Parses an unary expression.
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

        todo!()
    }

    /// Parses a binary expression, given its left-hand expression.
    fn parse_binary_expr(&mut self, prec: i32, mut left: Expr) -> BSResult<Expr> {
        // loop {
        //     let curr_prec = self.get_tok_precedence();

        //     if curr_prec < prec || self.at_end() {
        //         return Ok(left);
        //     }

        //     let op = match self.curr() {
        //         Op(op) => op,
        //         _ => return Err("Invalid operator."),
        //     };

        //     self.advance()?;

        //     let mut right = self.parse_unary_expr()?;

        //     let next_prec = self.get_tok_precedence();

        //     if curr_prec < next_prec {
        //         right = self.parse_binary_expr(curr_prec + 1, right)?;
        //     }

        //     left = Expr::Binary {
        //         op,
        //         left: Box::new(left),
        //         right: Box::new(right),
        //     };
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

    /// Parses a primary expression (an identifier, a number or a parenthesized expression).
    fn parse_primary(&mut self) -> BSResult<Expr> {
        // match self.curr() {
        //     Ident(_) => self.parse_id_expr(),
        //     Number(_) => self.parse_nb_expr(),
        //     LParen => self.parse_paren_expr(),
        //     If => self.parse_conditional_expr(),
        //     For => self.parse_for_expr(),
        //     Var => self.parse_var_expr(),
        //     _ => Err("Unknown expression."),
        // }

        todo!()
    }

    /// Parses a top-level expression and makes an anonymous function out of it,
    /// for easier compilation.
    fn parse_toplevel_expr(&mut self) -> BSResult<Function> {
        // match self.parse_expr() {
        //     Ok(expr) => Ok(Function {
        //         prototype: Prototype {
        //             name: "anonymous".to_string(),
        //             args: vec![],
        //             is_op: false,
        //             prec: 0,
        //         },
        //         body: Some(expr),
        //         is_anon: true,
        //     }),

        //     Err(err) => Err(err),
        // }

        ok(Function {
            prototype: Prototype {
                name: "anonymous".to_string(),
                args: vec![],
                is_op: false,
                prec: 0,
            },
            body: Some(Expr::Number(0.0)),
            is_anon: true,
        })
    }

    /// Parses the content of the parser.
    pub fn parse(&mut self) -> BSResult<Function> {
        let result = match self.current()? {
            Def => self.parse_def(),
            Extern => self.parse_extern(),
            _ => self.parse_toplevel_expr(),
        };

        match result {
            BSResult::Ok(result) => {
                if !self.at_end() {
                    parse_error("Unexpected token after parsed expression.")
                } else {
                    ok(result)
                }
            }

            err => err,
        }
    }
}
