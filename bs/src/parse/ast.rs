use crate::analysis::infer;
use crate::base::Type as BSType;
use crate::base::Type;
use crate::parse::span::Span;
use crate::result::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum BinaryOp {
    Add = 0,
    Sub,
    Mul,
    Div,
    Rem,
    Or,
    And,
    Xor,
    Equal,
    Less,
    Greater,
    LessOrEqual,
    GreaterOrEqual,
    NotEqual,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
            BinaryOp::Rem => write!(f, "%"),
            BinaryOp::Or => write!(f, "|"),
            BinaryOp::And => write!(f, "&"),
            BinaryOp::Xor => write!(f, "^"),
            BinaryOp::Equal => write!(f, "=="),
            BinaryOp::Less => write!(f, "<"),
            BinaryOp::Greater => write!(f, ">"),
            BinaryOp::LessOrEqual => write!(f, "<="),
            BinaryOp::GreaterOrEqual => write!(f, ">="),
            BinaryOp::NotEqual => write!(f, "!="),
        }
    }
}

/// Defines a primitive expression.
#[derive(Debug, Clone)]
pub enum ExprBody {
    Null,

    Binary {
        op: BinaryOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },

    Dot {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },

    Call {
        name: String,
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

    Bool(bool),

    Int64(i64),

    Float64(f64),

    Variable(String),
}

#[derive(Debug, Clone)]
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
                format!("{:?}", self),
                self.span,
            ),
        }
    }

    fn infer_type(&mut self, variables: &mut HashMap<String, BSType>) -> BSResult<BSType> {
        use ExprBody::*;

        if let Some(ty) = self.expr_type {
            return ok(ty);
        }

        match &mut self.body {
            Null => {
                self.expr_type = Some(BSType::Null);
                ok(BSType::Null)
            }
            Bool(_) => {
                self.expr_type = Some(BSType::Bool);
                ok(BSType::Bool)
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
                let body_ty = body.infer_type(variables)?;
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
                let lhs_type = lhs.infer_type(variables)?;
                let rhs_type = rhs.infer_type(variables)?;
                let res_type = infer::infer_type(*op, lhs_type, rhs_type, self.span)?;
                self.expr_type = Some(res_type);
                ok(res_type)
            }

            // TODO: infer type for call
            Call { name, args: _ } => {
                self.expr_type = Some(BSType::Int64);
                ok(BSType::Int64)
            }

            e => compile_error(
                format!("Cannot infer type for {:?}", e),
                "Unknown ambiguous type for expression".to_string(),
                self.span,
            ),
        }
    }
}

pub fn infer_types(exprs: &mut [Expr], variables: &mut HashMap<String, Type>) -> BSResult<BSType> {
    let mut res_ty = BSType::Null;
    for e in exprs {
        res_ty = e.infer_type(variables)?;
    }
    ok(res_ty)
}

pub struct Function {
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub body: Vec<Expr>,
    pub topl: bool,
}
