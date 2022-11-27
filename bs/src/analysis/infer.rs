use crate::parse::ast::BinaryOp;
use crate::parse::span::Span;
use crate::result::*;
use ffi::Type as BSType;
use std::collections::HashMap;
use BSType::*;
use BinaryOp::*;

lazy_static! {
    static ref OPS_TABLE: HashMap<(BinaryOp, BSType, BSType), BSType> = {
        let mut m = HashMap::new();
        m.insert((Equal, Bool, Bool), Bool);
        m.insert((NotEqual, Bool, Bool), Bool);
        m.insert((Or, Bool, Bool), Bool);
        m.insert((And, Bool, Bool), Bool);
        m.insert((Xor, Bool, Bool), Bool);

        m.insert((Add, Int64, Int64), Int64);
        m.insert((Sub, Int64, Int64), Int64);
        m.insert((Mul, Int64, Int64), Int64);
        m.insert((Div, Int64, Int64), Int64);
        m.insert((Rem, Int64, Int64), Int64);
        m.insert((Or, Int64, Int64), Int64);
        m.insert((And, Int64, Int64), Int64);
        m.insert((Xor, Int64, Int64), Int64);
        m.insert((Equal, Int64, Int64), Bool);
        m.insert((Less, Int64, Int64), Bool);
        m.insert((Greater, Int64, Int64), Bool);
        m.insert((LessOrEqual, Int64, Int64), Bool);
        m.insert((GreaterOrEqual, Int64, Int64), Bool);
        m.insert((NotEqual, Int64, Int64), Bool);

        m.insert((Add, Float64, Float64), Float64);
        m.insert((Sub, Float64, Float64), Float64);
        m.insert((Mul, Float64, Float64), Float64);
        m.insert((Div, Float64, Float64), Float64);
        m.insert((Or, Float64, Float64), Float64);
        m.insert((And, Float64, Float64), Float64);
        m.insert((Xor, Float64, Float64), Float64);
        m.insert((Equal, Float64, Float64), Bool);
        m.insert((Less, Float64, Float64), Bool);
        m.insert((Greater, Float64, Float64), Bool);
        m.insert((LessOrEqual, Float64, Float64), Bool);
        m.insert((GreaterOrEqual, Float64, Float64), Bool);
        m.insert((NotEqual, Float64, Float64), Bool);

        m
    };
}

pub fn infer_type(op: BinaryOp, lhs: BSType, rhs: BSType, span: Option<Span>) -> BSResult<BSType> {
    match OPS_TABLE.get(&(op, lhs, rhs)) {
        Some(&ty) => ok(ty),
        None => compile_error(
            "Type inference error".to_string(),
            format!("No such op: '{}' for types: {} {}", op, lhs, rhs),
            span,
        ),
    }
}
