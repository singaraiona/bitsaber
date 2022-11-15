use crate::base::bs_ops::Op;
use crate::base::Type as BSType;
use crate::result::*;
use BSType::*;
use Op::*;

// Basic type inference table for binary ops/functions
pub static OPS_TABLE: [(Op, BSType, BSType, BSType); 9] = [
    (Add, Int64, Int64, Int64),
    (Add, Float64, Float64, Float64),
    (Sub, Int64, Int64, Int64),
    (Sub, Float64, Float64, Float64),
    (Mul, Int64, Int64, Int64),
    (Mul, Float64, Float64, Float64),
    (Div, Int64, Int64, Int64),
    (Div, Float64, Float64, Float64),
    (Rem, Int64, Int64, Int64),
];

pub fn infer_type(op: Op, lhs: BSType, rhs: BSType) -> BSResult<BSType> {
    match OPS_TABLE
        .iter()
        .find(|(op_, lhs_, rhs_, _)| op == *op_ && lhs == *lhs_ && rhs == *rhs_)
        .map(|(_, _, _, ret)| *ret)
    {
        Some(ty) => ok(ty),
        None => compile_error(&format!("Invalid types for op: {:?} {} {}", op, lhs, rhs)),
    }
}
