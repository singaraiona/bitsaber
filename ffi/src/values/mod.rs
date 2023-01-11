pub mod f64_value;
pub mod fn_value;
pub mod i64_value;

use crate::types::Type;
use std::fmt;
use std::mem::{forget, transmute};
use std::ops::Deref;
use std::rc::Rc;

pub mod prelude {
    pub use super::f64_value::F64Value;
    pub use super::fn_value::FnValue;
    pub use super::i64_value::I64Value;
}

use prelude::*;

pub const NULL_VALUE: i64 = std::i64::MAX;
type Discriminant = i64;

#[derive(Debug, Clone)]
#[repr(transparent)]
struct OpaqueValue(Discriminant);

impl Deref for OpaqueValue {
    type Target = Discriminant;

    fn deref(&self) -> &Self::Target { &self.0 }
}

pub struct Value {
    ty: Type,
    val: OpaqueValue,
}

impl From<()> for Value {
    fn from(_: ()) -> Self { Value { ty: Type::Null, val: OpaqueValue(NULL_VALUE) } }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self { Value { ty: Type::Bool, val: OpaqueValue(value as _) } }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self { Value { ty: Type::Int64, val: OpaqueValue(value) } }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self { Value { ty: Type::Float64, val: OpaqueValue(unsafe { transmute(value) }) } }
}

impl From<Vec<i64>> for Value {
    fn from(value: Vec<i64>) -> Self {
        Value { ty: Type::VecInt64, val: OpaqueValue(unsafe { transmute(Rc::new(value)) }) }
    }
}

impl From<Vec<f64>> for Value {
    fn from(value: Vec<f64>) -> Self {
        Value { ty: Type::VecFloat64, val: OpaqueValue(unsafe { transmute(Rc::new(value)) }) }
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value { ty: Type::List, val: OpaqueValue(unsafe { transmute(Rc::new(value)) }) }
    }
}

impl From<FnValue> for Value {
    fn from(value: FnValue) -> Self {
        Value { ty: Type::Fn(value.get_type().clone()), val: OpaqueValue(unsafe { transmute(Rc::new(value)) }) }
    }
}

impl Into<bool> for Value {
    fn into(self) -> bool { *self.val != 0 }
}

impl Into<i64> for Value {
    fn into(self) -> i64 { *self.val }
}

impl Into<f64> for Value {
    fn into(self) -> f64 { unsafe { transmute(*self.val) } }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.ty {
            Type::Null => write!(f, "null"),
            Type::Bool => write!(f, "{}", *self.val != 0),
            Type::Int64 => write!(f, "{}", *self.val),
            Type::Float64 => write!(f, "{:.2}", *self.val as f64),
            Type::VecInt64 => unsafe {
                let v: Rc<Vec<i64>> = transmute(*self.val);
                let res = write!(f, "{:?}", v);
                forget(v);
                res
            },
            // VecFloat64 => write!(f, "{:?}", v),
            // Value::List(v) => write!(f, "{:?}", v),
            // // Value::Fn(_) => write!(f, "{:?}", self.get_type()),
            _ => write!(f, "{:?}", "ty"),
        }
    }
}

impl Value {
    pub fn from_raw_parts(ty: Type, val: i64) -> Self { Value { ty, val: OpaqueValue(val) } }

    pub fn get_type(&self) -> &Type { &self.ty }

    pub fn is_null(&self) -> bool { self.ty == Type::Null }

    pub fn get_infered_type(&self) -> Type {
        match &self.ty {
            Type::Fn(f) => f.ret.as_ref().clone(),
            _ => self.ty.clone(),
        }
    }

    pub fn as_raw(&self) -> i64 { *self.val }

    pub fn as_ptr(&self) -> *const () { *self.val as *const () }
}
