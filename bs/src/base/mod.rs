use llvm::context::Context;
use llvm::types::struct_type::StructType as LLVMStructType;
use llvm::types::Type as LLVMType;
use llvm::values::Value as LLVMValue;
use std::fmt;
use std::mem::transmute;
use std::rc::Rc;

pub const NULL_VALUE: i64 = std::i64::MAX;

pub mod binary;
pub mod f64_value;
pub mod i64_value;
pub mod infer;

pub mod prelude {
    pub use super::f64_value::F64Value;
    pub use super::i64_value::I64Value;
}

use prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u64)]
pub enum Type {
    Null = 0,
    Int64,
    Float64,
    VecInt64,
    VecFloat64,
    List,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Null => write!(f, "Null"),
            Type::Int64 => write!(f, "Int64"),
            Type::Float64 => write!(f, "Float64"),
            Type::VecInt64 => write!(f, "Int64[]"),
            Type::VecFloat64 => write!(f, "Float64[]"),
            Type::List => write!(f, "[]"),
        }
    }
}

impl Type {
    pub fn into_llvm_type<'a>(self, context: &'a Context) -> LLVMType<'a> {
        match self {
            Type::Null => context.i64_type().into(),
            Type::Int64 => context.i64_type().into(),
            Type::Float64 => context.f64_type().into(),
            Type::VecInt64 => context
                .struct_type(
                    &[context.i64_type().into(), context.i64_type().into()],
                    true,
                )
                .into(),
            Type::VecFloat64 => context
                .struct_type(
                    &[context.i64_type().into(), context.i64_type().into()],
                    true,
                )
                .into(),
            _ => unimplemented!(),
        }
    }

    pub fn is_scalar(&self) -> bool {
        match self {
            Type::Null => false,
            Type::Int64 => true,
            Type::Float64 => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C, align(16))]
pub enum Value {
    Null,
    Int64(I64Value),
    Float64(f64),
    VecInt64(Rc<Vec<i64>>),
    VecFloat64(Rc<Vec<f64>>),
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
            true,
        )
    }

    pub fn bs_type(&self) -> Type {
        match self {
            Value::Null => Type::Null,
            Value::Int64(_) => Type::Int64,
            Value::Float64(_) => Type::Float64,
            Value::VecInt64(_) => Type::VecInt64,
            Value::VecFloat64(_) => Type::VecFloat64,
            Value::List(_) => Type::List,
        }
    }

    pub fn into_llvm_value<'a>(self, context: &'a Context) -> LLVMValue<'a> {
        unsafe {
            let tag = self.bs_type() as u64 as i64;
            match self {
                Value::Null => Self::into_llvm_struct(tag, 0, context),
                Value::Int64(v) => Self::into_llvm_struct(tag, v.into(), context),
                Value::Float64(v) => Self::into_llvm_struct(tag, transmute(v), context),
                Value::VecInt64(v) => Self::into_llvm_struct(tag, transmute::<_, i64>(v), context),
                Value::VecFloat64(v) => {
                    Self::into_llvm_struct(tag, transmute::<_, i64>(v), context)
                }

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
        Value::Int64(value.into())
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float64(value)
    }
}

impl From<Vec<i64>> for Value {
    fn from(value: Vec<i64>) -> Self {
        Value::VecInt64(Rc::new(value))
    }
}

impl From<Vec<f64>> for Value {
    fn from(value: Vec<f64>) -> Self {
        Value::VecFloat64(Rc::new(value))
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
            Value::Int64(v) => write!(f, "{}: {}", v, self.bs_type()),
            Value::Float64(v) => write!(f, "{:.2}: {}", v, self.bs_type()),
            Value::VecInt64(v) => write!(f, "{:?}: {}", v, self.bs_type()),
            Value::VecFloat64(v) => write!(f, "{:?}: {}", v, self.bs_type()),
            Value::List(v) => write!(f, "{:?}: {}", v, self.bs_type()),
        }
    }
}
