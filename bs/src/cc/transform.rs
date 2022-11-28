use ffi::Type as BSType;
use ffi::Value as BSValue;
use llvm::context::Context;
use llvm::types::prelude::StructType as LLVMStructType;
use llvm::types::Type as LLVMType;
use llvm::values::Value as LLVMValue;
use std::mem::transmute;
use std::rc::Rc;

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
            BSValue::Null => into_llvm_struct(tag, 0, context),
            BSValue::Bool(b) => into_llvm_struct(tag, b as i64, context),
            BSValue::Int64(v) => context.i64_type().const_value(v.into()).into(),
            BSValue::Float64(v) => into_llvm_struct(tag, transmute(v), context),
            BSValue::VecInt64(v) => into_llvm_struct(tag, transmute::<_, i64>(v), context),
            BSValue::VecFloat64(v) => into_llvm_struct(tag, transmute::<_, i64>(v), context),

            _ => unimplemented!(),
        }
    }
}

pub fn bs_value_from_llvm_value(value: LLVMValue) -> BSValue {
    // let struct_val: StructValue<'_> = value.into();
    unsafe {
        // let tag = value
        //     .get_struct_element_value(0)
        //     .unwrap()
        //     .into_int_value()
        //     .into();
        // let val = value
        //     .get_struct_element_value(1)
        //     .unwrap()
        //     .into_int_value()
        //     .into();

        let tag = 2;
        let val = 666;

        match transmute::<u64, BSType>(tag as u64) {
            BSType::Null => BSValue::Null,
            BSType::Bool => BSValue::Bool(val != 0),
            BSType::Int64 => BSValue::Int64(val.into()),
            BSType::Float64 => BSValue::Float64(transmute(val)),
            BSType::VecInt64 => BSValue::VecInt64(transmute::<_, Rc<Vec<i64>>>(val)),
            _ => unreachable!(),
        }
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
