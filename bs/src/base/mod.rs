use llvm::context::Context;
use llvm::types::struct_type::StructType as LLVMStructType;
use llvm::types::Type as LLVMType;
use llvm::types::TypeIntrinsics;
use llvm::values::Value as LLVMValue;
use std::fmt;
use std::mem::transmute;
use std::rc::Rc;

pub mod f64_value;
pub mod i64_value;

pub mod prelude {
    pub use super::f64_value::F64Value;
    pub use super::i64_value::I64Value;
}

use prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Null,
    I64,
    F64,
    VecI64,
    VecF64,
    List,
}

impl From<LLVMType<'_>> for Type {
    fn from(llvm_type: LLVMType) -> Self {
        match llvm_type {
            LLVMType::Null => Self::Null,
            LLVMType::I64(_) => Self::I64,
            LLVMType::F64(_) => Self::F64,
            _ => Self::Null,
        }
    }
}

// impl Into<LLVMType<'_>> for Type {
//     fn into(self) -> LLVMType {
//         match self {
//             Self::Null => LLVMType::Null,
//             Self::I64 => LLVMType::I64(LLVMType::new_i64()),
//             Self::F64 => LLVMType::F64(LLVMType::new_f64()),
//             _ => LLVMType::Null,
//         }
//     }
// }

impl Type {
    pub fn into_llvm_type<'a>(self, ctx: &'a Context) -> LLVMType<'a> {
        match self {
            Type::Null => LLVMType::Null,
            Type::I64 => ctx.i64_type().into(),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C, align(16))]
pub enum Value {
    Null,
    I64(I64Value),
    F64(f64),
    VecI64(Rc<Vec<i64>>),
    VecF64(Rc<Vec<f64>>),
    List(Rc<Vec<Value>>),
}

impl Value {
    fn into_llvm_struct<'a>(tag: i64, val: i64, context: &'a Context) -> LLVMValue<'a> {
        let ret_struct = Value::llvm_struct_type(context).const_value(
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
            false,
        )
    }

    pub fn into_llvm_value<'a>(self, context: &'a Context) -> LLVMValue<'a> {
        unsafe {
            match self {
                Value::Null => Self::into_llvm_struct(0, 0, context),
                Value::I64(v) => Self::into_llvm_struct(1, v.into(), context),
                Value::F64(v) => Self::into_llvm_struct(2, transmute(v), context),
                Value::VecI64(v) => Self::into_llvm_struct(3, transmute::<_, i64>(v), context),
                Value::VecF64(v) => Self::into_llvm_struct(4, transmute::<_, i64>(v), context),
                _ => unimplemented!(),
            }
        }
    }

    pub fn llvm_type<'a>(context: &'a Context) -> LLVMType<'a> {
        Self::llvm_struct_type(context).into()
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::I64(value.into())
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::F64(value)
    }
}

impl From<Vec<i64>> for Value {
    fn from(value: Vec<i64>) -> Self {
        Value::VecI64(Rc::new(value))
    }
}

impl From<Vec<f64>> for Value {
    fn from(value: Vec<f64>) -> Self {
        Value::VecF64(Rc::new(value))
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::List(Rc::new(value))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "Null"),
            Value::I64(v) => write!(f, "{}", v),
            Value::F64(v) => write!(f, "{:.2}", v),
            Value::VecI64(v) => write!(f, "{:?}", v),
            Value::VecF64(v) => write!(f, "{:?}", v),
            Value::List(v) => write!(f, "{:?}", v),
        }
    }
}
