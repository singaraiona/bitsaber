use crate::ops::binary;
use crate::parse::span::Span;
use crate::result::*;
use ffi::Type as BSType;
use ffi::Value as BSValue;
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

    Binary { op: BinaryOp, lhs: Box<Expr>, rhs: Box<Expr> },

    Dot { lhs: Box<Expr>, rhs: Box<Expr> },

    Call { name: String, args: Vec<Expr> },

    Cond { cond: Box<Expr>, cons: Vec<Expr>, altr: Vec<Expr> },

    For { var_name: String, start: Box<Expr>, end: Box<Expr>, step: Option<Box<Expr>>, body: Box<Expr> },

    Assign { name: String, body: Box<Expr>, global: bool },

    Iterator { res_type: BSType, count: usize },

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
    pub fn new(body: ExprBody, span: Option<Span>) -> Expr { Expr { body, expr_type: None, span } }

    pub fn get_type(&self) -> BSResult<BSType> {
        match &self.expr_type {
            Some(t) => ok(t.clone()),
            None => compile_error("Unknown expression type".to_string(), format!("{:?}", self), self.span),
        }
    }

    pub fn infer_type(
        &mut self,
        globals: &HashMap<String, (BSValue, BSType)>,
        variables: &mut HashMap<String, BSType>,
    ) -> BSResult<BSType> {
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
            Assign { name, body, global } => {
                let body_ty = body.infer_type(globals, variables)?;
                self.expr_type = Some(body_ty.clone());
                variables.insert(name.clone(), body_ty);
                ok(body_ty)
            }
            Variable(name) => match variables.get(name) {
                Some(ty) => {
                    self.expr_type = Some(ty.clone());
                    ok(ty.clone())
                }
                None => match globals.get(name) {
                    Some((_, ty)) => {
                        self.expr_type = Some(ty.clone());
                        ok(ty.clone())
                    }
                    None => compile_error("Unknown variable".to_string(), name.clone(), self.span),
                },
            },
            Binary { op, ref mut lhs, ref mut rhs } => {
                let lhs_type = lhs.infer_type(globals, variables)?;
                let rhs_type = rhs.infer_type(globals, variables)?;
                let res_type = binary::infer_type(*op, lhs_type, rhs_type, self.span)?;
                self.expr_type = Some(res_type);
                ok(res_type)
            }

            Call { name, args } => {
                infer_types(args, globals, variables)?;
                match globals.get(name) {
                    Some((_, ty)) => {
                        self.expr_type = Some(ty.clone());
                        ok(ty.clone())
                    }
                    None => compile_error("Unknown variable".to_string(), name.clone(), self.span),
                }
            }

            Cond { cond, cons, altr } => {
                let cond_type = cond.infer_type(globals, variables)?;
                if cond_type != BSType::Bool {
                    return compile_error(
                        "Condition must be a bool type".to_string(),
                        format!("Found {:?} here", cond_type),
                        self.span,
                    );
                }

                let cons_type = infer_types(cons, globals, variables)?;
                let altr_type = infer_types(altr, globals, variables)?;

                if cons_type != altr_type {
                    return compile_error(
                        "Both branches of condition must have the same type".to_string(),
                        format!("Found {:?} in the true branch and {:?} in the false branch", cons_type, altr_type),
                        self.span,
                    );
                }

                self.expr_type = Some(cons_type);
                ok(cons_type)
            }

            Iterator { res_type, count } => {
                self.expr_type = Some(res_type.clone());
                ok(res_type.clone())
            }

            e => compile_error(
                format!("Cannot infer type for {:?}", e),
                "Unknown or ambiguous type for expression".to_string(),
                self.span,
            ),
        }
    }
}

pub fn infer_types(
    exprs: &mut [Expr],
    globals: &HashMap<String, (BSValue, BSType)>,
    variables: &mut HashMap<String, BSType>,
) -> BSResult<BSType> {
    let mut res_ty = BSType::Null;
    for e in exprs {
        res_ty = e.infer_type(globals, variables)?;
    }
    ok(res_ty)
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<(String, BSType)>,
    pub body: Vec<Expr>,
    pub topl: bool,
}
