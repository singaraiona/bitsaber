pub mod i64_value;
use llvm::context::Context;
use llvm::values::Value as LLVMValue;
use std::fmt;
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

fn into_llvm_struct<'a>(tag: i64, val: i64, context: &'a Context) -> LLVMValue<'a> {
    let ret_struct = context
        .struct_type(
            &[context.i64_type().into(), context.i64_type().into()],
            false,
        )
        .const_value(
            &[
                context.i64_type().const_value(tag).into(),
                context.i64_type().const_value(val).into(),
            ],
            true,
        );

    ret_struct.into()
}

impl Value {
    pub fn into_llvm_value<'a>(self, context: &'a Context) -> LLVMValue<'a> {
        match self {
            Value::Null => into_llvm_struct(0, 0, context),
            Value::I64(v) => into_llvm_struct(1, v, context),
            Value::F64(v) => into_llvm_struct(2, v as i64, context),
            Value::VecI64(v) => {
                let v: i64 = unsafe { std::mem::transmute(v) };
                into_llvm_struct(3, v, context)
            }
            _ => unimplemented!(),
        }
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
        write!(f, "{:?} {}", self, std::mem::size_of_val(self))
    }
}
