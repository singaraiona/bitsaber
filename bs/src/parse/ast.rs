use crate::base::bs_ops::Op;

/// Defines a primitive expression.
#[derive(Debug)]
pub enum Expr {
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

    VecInt64(Vec<i64>),

    VecFloat64(Vec<f64>),

    Int64(i64),

    Float64(f64),

    Variable(String),

    VarIn {
        variables: Vec<(String, Option<Expr>)>,
        body: Box<Expr>,
    },
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
    pub body: Option<Expr>,
    pub is_anon: bool,
}
