use crate::base::Type as BSType;
use crate::parse::ast::BinaryOp;
use crate::parse::span::Span;
use crate::result::*;
use BSType::*;
use BinaryOp::*;

// Basic type inference table for binary ops/functions
pub static OPS_TABLE: [(BinaryOp, BSType, BSType, BSType); 28] = [
    (Add, Int64, Int64, Int64),
    (Add, Float64, Float64, Float64),
    (Sub, Int64, Int64, Int64),
    (Sub, Float64, Float64, Float64),
    (Mul, Int64, Int64, Int64),
    (Mul, Float64, Float64, Float64),
    (Div, Int64, Int64, Int64),
    (Div, Float64, Float64, Float64),
    (Rem, Int64, Int64, Int64),
    (Rem, Float64, Float64, Float64),
    (Or, Int64, Int64, Int64),
    (Or, Float64, Float64, Float64),
    (And, Int64, Int64, Int64),
    (And, Float64, Float64, Float64),
    (Xor, Int64, Int64, Int64),
    (Xor, Float64, Float64, Float64),
    (Equal, Int64, Int64, Bool),
    (Equal, Float64, Float64, Bool),
    (Less, Int64, Int64, Bool),
    (Less, Float64, Float64, Bool),
    (Greater, Int64, Int64, Bool),
    (Greater, Float64, Float64, Bool),
    (LessOrEqual, Int64, Int64, Bool),
    (LessOrEqual, Float64, Float64, Bool),
    (GreaterOrEqual, Int64, Int64, Bool),
    (GreaterOrEqual, Float64, Float64, Bool),
    (NotEqual, Int64, Int64, Bool),
    (NotEqual, Float64, Float64, Bool),
];

pub fn infer_type(op: BinaryOp, lhs: BSType, rhs: BSType, span: Option<Span>) -> BSResult<BSType> {
    match OPS_TABLE
        .iter()
        .find(|(op_, lhs_, rhs_, _)| op == *op_ && lhs == *lhs_ && rhs == *rhs_)
        .map(|(_, _, _, ret)| *ret)
    {
        Some(ty) => ok(ty),
        None => compile_error(
            "Type inference error".to_string(),
            format!("No such op: {} for types: {} {}", op, lhs, rhs),
            span,
        ),
    }
}
