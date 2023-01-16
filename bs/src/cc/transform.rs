use ffi::types::Type as BSType;
use ffi::values::Value as BSValue;
use ffi::values::NULL_VALUE;
use llvm::context::Context;
use llvm::types::Type as LLVMType;
use llvm::values::prelude::*;
use llvm::values::Value as LLVMValue;

pub fn llvm_value_from_bs_value<'a>(bs_value: BSValue, context: &'a Context) -> LLVMValue<'a> {
    match bs_value.get_type() {
        BSType::Null => context.i64_type().const_value(NULL_VALUE).into(),
        BSType::Bool => context.i1_type().const_value(bs_value.into()).into(),
        BSType::Int64 => context.i64_type().const_value(bs_value.into()).into(),
        BSType::Float64 => context.f64_type().const_value(bs_value.into()).into(),
        BSType::VecInt64 => context
            .ptr_type(context.i64_type().into())
            .const_value(bs_value.as_raw() as _)
            .into(),
        // BSType::VecFloat64(v) => into_llvm_struct(tag, transmute::<_, i64>(v), context),
        _ => unimplemented!(),
    }
}

pub fn bs_value_from_llvm_value(value: LLVMValue, ty: BSType, context: &Context) -> BSValue {
    match ty {
        BSType::Null => BSValue::from(()),
        BSType::Bool => {
            let val: I1Value<'_> = value.into();
            let val: bool = val.get_constant().into();
            BSValue::from(val)
        }
        BSType::Int64 => {
            let val: I64Value<'_> = value.into();
            let val: i64 = val.get_constant().into();
            BSValue::from(val)
        }
        BSType::Float64 => {
            let val: F64Value<'_> = value.into();
            let val: f64 = val.get_constant().into();
            BSValue::from(val)
        }
        BSType::VecInt64 => {
            let val: PtrValue<'_> = value.into();
            BSValue::from_raw_parts(ty, val.const_to_i64().into())
        }
        _ => todo!(),
    }
}

pub fn llvm_type_from_bs_type<'a>(bs_type: BSType, context: &'a Context) -> LLVMType<'a> {
    match bs_type {
        BSType::Null => context.i64_type().into(),
        BSType::Bool => context.i1_type().into(),
        BSType::Int64 => context.i64_type().into(),
        BSType::Float64 => context.f64_type().into(),
        BSType::VecInt64 => context.ptr_type(context.i64_type().into()).into(),
        // BSType::VecFloat64 => context
        //     .struct_type(&[context.i64_type().into(), context.i64_type().into()], false)
        //     .into(),
        _ => unimplemented!(),
    }
}
