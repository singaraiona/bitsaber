use ffi::Type as BSType;
use ffi::Value as BSValue;
use ffi::NULL_VALUE;
use llvm::context::Context;
use llvm::types::prelude::StructType as LLVMStructType;
use llvm::types::Type as LLVMType;
use llvm::values::prelude::*;
use llvm::values::Value as LLVMValue;
use std::mem::transmute;

fn into_llvm_struct<'a>(tag: i64, val: i64, context: &'a Context) -> LLVMValue<'a> {
    let ret_struct = llvm_struct_type(context).const_value(
        &[
            context.i64_type().const_value(tag).into(),
            context.i64_type().const_value(val).into(),
        ],
        true,
    );

    ret_struct.into()
}

fn llvm_struct_type<'a>(context: &'a Context) -> LLVMStructType<'a> {
    context.struct_type(
        &[context.i64_type().into(), context.i64_type().into()],
        true,
    )
}

pub fn llvm_value_from_bs_value<'a>(bs_value: BSValue, context: &'a Context) -> LLVMValue<'a> {
    unsafe {
        let tag = bs_value.get_type() as u64 as i64;
        match bs_value {
            BSValue::Null => context.i64_type().const_value(NULL_VALUE).into(),
            BSValue::Bool(b) => context.i1_type().const_value(b).into(),
            BSValue::Int64(v) => context.i64_type().const_value(v.into()).into(),
            BSValue::Float64(v) => context.f64_type().const_value(v.into()).into(),
            BSValue::VecInt64(v) => into_llvm_struct(tag, transmute::<_, i64>(v), context),
            BSValue::VecFloat64(v) => into_llvm_struct(tag, transmute::<_, i64>(v), context),

            _ => unimplemented!(),
        }
    }
}

pub fn bs_value_from_llvm_value(value: LLVMValue, ty: BSType) -> BSValue {
    match ty {
        BSType::Null => BSValue::Null,
        BSType::Bool => {
            let val: I1Value<'_> = value.into();
            BSValue::Bool(val.get_constant().into())
        }
        BSType::Int64 => {
            let val: I64Value<'_> = value.into();
            BSValue::Int64(val.get_constant().into())
        }
        BSType::Float64 => {
            let val: F64Value<'_> = value.into();
            BSValue::Float64(val.into())
        }
        _ => todo!(), // _ => transmute::<LLVMValue, BSValue>(value),
    }
}

pub fn llvm_type_from_bs_type<'a>(bs_type: BSType, context: &'a Context) -> LLVMType<'a> {
    match bs_type {
        BSType::Null => context.i64_type().into(),
        BSType::Bool => context.i1_type().into(),
        BSType::Int64 => context.i64_type().into(),
        BSType::Float64 => context.f64_type().into(),
        BSType::VecInt64 => context
            .struct_type(
                &[context.i64_type().into(), context.i64_type().into()],
                true,
            )
            .into(),
        BSType::VecFloat64 => context
            .struct_type(
                &[context.i64_type().into(), context.i64_type().into()],
                true,
            )
            .into(),
        _ => unimplemented!(),
    }
}
