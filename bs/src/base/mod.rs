pub mod i64_value;
use crate::compile::context::Context;
use crate::compile::types::struct_type::StructType as LLVMStructType;
use crate::compile::types::Type as LLVMType;
use crate::compile::values::Value as LLVMValue;
use std::fmt;
use std::mem::transmute;
use std::rc::Rc;

#[derive(Debug, Clone)]
#[repr(C, align(16))]
pub enum Value {
    Null,
    I64(i64),
    F64(f64),
    VecI64(Rc<Vec<i64>>),
    VecF64(Rc<Vec<f64>>),
    List(Rc<Vec<Value>>),
    // Table(Rc<Vec<(Value, Value)>>),
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
                Value::I64(v) => Self::into_llvm_struct(1, v, context),
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
        Value::I64(value)
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
