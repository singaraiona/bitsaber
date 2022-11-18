use crate::base::binary::Op;
use crate::base::Type as BSType;
use crate::parse::span::Span;
use crate::result::*;
use BSType::*;
use Op::*;

// Basic type inference table for binary ops/functions
pub static OPS_TABLE: [(Op, BSType, BSType, BSType); 16] = [
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
];

pub fn infer_type(op: Op, lhs: BSType, rhs: BSType, span: Option<Span>) -> BSResult<BSType> {
    match OPS_TABLE
        .iter()
        .find(|(op_, lhs_, rhs_, _)| op == *op_ && lhs == *lhs_ && rhs == *rhs_)
        .map(|(_, _, _, ret)| *ret)
    {
        Some(ty) => ok(ty),
        None => compile_error(
            "Type inference error".to_string(),
            format!("No such op: {:?} for types: {} {}", op, lhs, rhs),
            span,
        ),
    }
}
