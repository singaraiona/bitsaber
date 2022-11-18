use crate::analysis::infer;
use crate::base::binary::Op;
use crate::base::Type as BSType;
use crate::parse::span::Span;
use crate::result::*;
use std::collections::HashMap;

/// Defines a primitive expression.
#[derive(Debug)]
pub enum ExprBody {
    Null,

    Binary {
        op: Op,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },

    Dot {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },

    Call {
        fn_name: String,
        args: Vec<Expr>,
    },

    Conditional {
        cond: Box<Expr>,
        consequence: Box<Expr>,
        alternative: Box<Expr>,
    },

    For {
        var_name: String,
        start: Box<Expr>,
        end: Box<Expr>,
        step: Option<Box<Expr>>,
        body: Box<Expr>,
    },

    Assign {
        variable: String,
        body: Box<Expr>,
    },

    VecInt64(Vec<i64>),

    VecFloat64(Vec<f64>),

    Int64(i64),

    Float64(f64),

    Variable(String),
}

#[derive(Debug)]
pub struct Expr {
    pub body: ExprBody,
    pub expr_type: Option<BSType>,
    pub span: Option<Span>,
}

impl Expr {
    pub fn new(body: ExprBody, span: Option<Span>) -> Expr {
        Expr {
            body,
            expr_type: None,
            span,
        }
    }

    pub fn get_type(&self) -> BSResult<BSType> {
        match &self.expr_type {
            Some(t) => ok(t.clone()),
            None => compile_error(
                "Unknown expression type".to_string(),
                "".to_string(),
                self.span,
            ),
        }
    }

    fn _infer_type(&mut self, variables: &mut HashMap<String, BSType>) -> BSResult<BSType> {
        use ExprBody::*;

        if let Some(ty) = self.expr_type {
            return ok(ty);
        }

        match &mut self.body {
            Null => {
                self.expr_type = Some(BSType::Null);
                ok(BSType::Null)
            }
            Int64(_) => {
                self.expr_type = Some(BSType::Int64);
                ok(BSType::Int64)
            }
            Float64(_) => {
                self.expr_type = Some(BSType::Float64);
                ok(BSType::Float64)
            }
            VecInt64(_) => {
                self.expr_type = Some(BSType::VecInt64);
                ok(BSType::VecInt64)
            }
            VecFloat64(_) => {
                self.expr_type = Some(BSType::VecFloat64);
                ok(BSType::VecFloat64)
            }
            Assign { variable, body } => {
                let body_ty = body._infer_type(variables)?;
                self.expr_type = Some(body_ty.clone());
                variables.insert(variable.clone(), body_ty);
                ok(body_ty)
            }
            Variable(name) => match variables.get(name) {
                Some(ty) => {
                    self.expr_type = Some(ty.clone());
                    ok(ty.clone())
                }
                None => compile_error(
                    format!("Unknown variable: {}", name),
                    "".to_string(),
                    self.span,
                ),
            },
            Binary {
                op,
                ref mut lhs,
                ref mut rhs,
            } => {
                let lhs_type = lhs._infer_type(variables)?;
                let rhs_type = rhs._infer_type(variables)?;
                let res_type = infer::infer_type(*op, lhs_type, rhs_type, self.span)?;
                self.expr_type = Some(res_type);
                ok(res_type)
            }

            e => compile_error(
                format!("Cannot infer type for {:?}", e),
                "Unknown ambiguous type for expression".to_string(),
                self.span,
            ),
        }
    }

    pub fn infer_type(&mut self) -> BSResult<BSType> {
        let mut variables = HashMap::new();
        self._infer_type(&mut variables)
    }
}

pub fn infer_types(exprs: &mut [Expr]) -> BSResult<BSType> {
    let mut variables = HashMap::new();
    let mut res_ty = BSType::Null;
    for e in exprs {
        res_ty = e._infer_type(&mut variables)?;
    }
    ok(res_ty)
}

/// Defines the prototype (name and parameters) of a function.
#[derive(Debug)]
pub struct Prototype {
    pub name: String,
    pub args: Vec<String>,
    pub is_op: bool,
    pub prec: usize,
}

/// Defines a user-defined or external function.
#[derive(Debug)]
pub struct Function {
    pub prototype: Prototype,
    pub body: Vec<Expr>,
    pub is_anon: bool,
    pub span: Option<Span>,
}
