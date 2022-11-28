use crate::llvm::enums::*;
use crate::parse::ast::BinaryOp;
use crate::parse::span::Span;
use crate::result::*;
use ffi::Type as BSType;
use llvm::builder::Builder;
use llvm::values::Value;
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

pub fn compile<'a, 'b>(
    builder: &'a Builder<'b>,
    op: BinaryOp,
    lhs: (Value<'b>, BSType),
    rhs: (Value<'b>, BSType),
    span: Option<Span>,
) -> BSResult<Value<'b>> {
    let (lhs, lhs_type) = lhs;
    let (rhs, rhs_type) = rhs;

    use BSType::*;
    use BinaryOp::*;
    use FloatPredicate as FP;
    use IntPredicate as IP;

    let result = match (op, lhs_type, rhs_type) {
        (Add, Int64, Int64) => builder.build_int_add(lhs, rhs, "addtmp"),
        (Add, Float64, Float64) => builder.build_float_add(lhs, rhs, "addtmp"),
        (Div, Int64, Int64) => builder.build_int_div(lhs, rhs, "divtmp"),
        (Div, Float64, Float64) => builder.build_float_div(lhs, rhs, "divtmp"),
        (Sub, Int64, Int64) => builder.build_int_sub(lhs, rhs, "subtmp"),
        (Sub, Float64, Float64) => builder.build_float_sub(lhs, rhs, "subtmp"),
        (Mul, Int64, Int64) => builder.build_int_mul(lhs, rhs, "multmp"),
        (Mul, Float64, Float64) => builder.build_float_mul(lhs, rhs, "multmp"),
        (Rem, Int64, Int64) => builder.build_rem(lhs, rhs, "remtmp"),
        (Rem, Float64, Float64) => builder.build_rem(lhs, rhs, "remtmp"),
        (Or, Int64, Int64) => builder.build_or(lhs, rhs, "ortmp"),
        (Or, Float64, Float64) => builder.build_or(lhs, rhs, "ortmp"),
        (And, Int64, Int64) => builder.build_and(lhs, rhs, "andtmp"),
        (And, Float64, Float64) => builder.build_and(lhs, rhs, "andtmp"),
        (Xor, Int64, Int64) => builder.build_xor(lhs, rhs, "xortmp"),
        (Xor, Float64, Float64) => builder.build_xor(lhs, rhs, "xortmp"),
        // (Shl, Int64, Int64) => self.builder.build_shl(lhs, rhs, "shltmp"),
        // (Shl, Float64, Float64) => self.builder.build_shl(lhs, rhs, "shltmp"),
        // (Shr, Int64, Int64) => self.builder.build_shr(lhs, rhs, "shrtmp"),
        // (Shr, Float64, Float64) => self.builder.build_shr(lhs, rhs, "shrtmp"),
        (Equal, Bool, Bool) => builder.build_int_compare(IP::EQ, lhs, rhs, "eqtmp"),
        (Equal, Int64, Int64) => builder.build_int_compare(IP::EQ, lhs, rhs, "eqtmp"),
        (Equal, Float64, Float64) => builder.build_float_compare(FP::UEQ, lhs, rhs, "eqtmp"),
        (Less, Bool, Bool) => builder.build_int_compare(IP::SLT, lhs, rhs, "lttmp"),
        (Less, Int64, Int64) => builder.build_int_compare(IP::SLT, lhs, rhs, "lttmp"),
        (Less, Float64, Float64) => builder.build_float_compare(FP::ULT, lhs, rhs, "lttmp"),
        (LessOrEqual, Bool, Bool) => builder.build_int_compare(IP::SLE, lhs, rhs, "letmp"),
        (LessOrEqual, Int64, Int64) => builder.build_int_compare(IP::SLE, lhs, rhs, "letmp"),
        (LessOrEqual, Float64, Float64) => builder.build_float_compare(FP::ULE, lhs, rhs, "letmp"),
        (Greater, Bool, Bool) => builder.build_int_compare(IP::SGT, lhs, rhs, "gttmp"),
        (Greater, Int64, Int64) => builder.build_int_compare(IP::SGT, lhs, rhs, "gttmp"),
        (Greater, Float64, Float64) => builder.build_float_compare(FP::UGT, lhs, rhs, "gttmp"),
        (GreaterOrEqual, Bool, Bool) => builder.build_int_compare(IP::SGE, lhs, rhs, "getmp"),
        (GreaterOrEqual, Int64, Int64) => builder.build_int_compare(IP::SGE, lhs, rhs, "getmp"),
        (GreaterOrEqual, Float64, Float64) => {
            builder.build_float_compare(FP::UGE, lhs, rhs, "getmp")
        }
        (NotEqual, Bool, Bool) => builder.build_int_compare(IP::NE, lhs, rhs, "neqtmp"),
        (NotEqual, Int64, Int64) => builder.build_int_compare(IP::NE, lhs, rhs, "neqtmp"),
        (NotEqual, Float64, Float64) => builder.build_float_compare(FP::UNE, lhs, rhs, "neqtmp"),
        (op, _, _) => {
            return compile_error(
                format!("Unsupported binary op: '{}'", op),
                "Refer to a supported binary operations".to_string(),
                span,
            )
        }
    };

    ok(result)
}
