#[macro_use]
extern crate lazy_static;

use std::fmt;
use std::rc::Rc;

pub const NULL_VALUE: i64 = std::i64::MAX;

pub mod external;
pub mod f64_value;
pub mod i64_value;

pub mod prelude {
    pub use super::f64_value::F64Value;
    pub use super::i64_value::I64Value;
}

use prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u64)]
pub enum Type {
    Null = 0,
    Bool,
    Int64,
    Float64,
    VecInt64,
    VecFloat64,
    List,
}

#[derive(Debug, Clone)]
pub struct FnType {
    pub args: Vec<Type>,
    pub ret: Type,
}

impl TryFrom<&str> for Type {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Null" => Ok(Type::Null),
            "Bool" => Ok(Type::Bool),
            "Int64" => Ok(Type::Int64),
            "Float64" => Ok(Type::Float64),
            "Int64[]" => Ok(Type::VecInt64),
            "Float64[]" => Ok(Type::VecFloat64),
            "[]" => Ok(Type::List),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Null => write!(f, "Null"),
            Type::Bool => write!(f, "Bool"),
            Type::Int64 => write!(f, "Int64"),
            Type::Float64 => write!(f, "Float64"),
            Type::VecInt64 => write!(f, "Int64[]"),
            Type::VecFloat64 => write!(f, "Float64[]"),
            Type::List => write!(f, "[]"),
        }
    }
}

impl Type {
    pub fn is_scalar(&self) -> bool {
        match self {
            Type::Null | Type::Int64 | Type::Float64 | Type::Bool => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C, align(16))]
pub enum Value {
    Null,
    Bool(bool),
    Int64(I64Value),
    Float64(f64),
    VecInt64(Rc<Vec<i64>>),
    VecFloat64(Rc<Vec<f64>>),
    List(Rc<Vec<Value>>),
}

impl Value {
    pub fn get_type(&self) -> Type {
        match self {
            Value::Null => Type::Null,
            Value::Bool(_) => Type::Bool,
            Value::Int64(_) => Type::Int64,
            Value::Float64(_) => Type::Float64,
            Value::VecInt64(_) => Type::VecInt64,
            Value::VecFloat64(_) => Type::VecFloat64,
            Value::List(_) => Type::List,
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self { Value::Bool(value) }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self { Value::Int64(value.into()) }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self { Value::Float64(value) }
}

impl From<Vec<i64>> for Value {
    fn from(value: Vec<i64>) -> Self { Value::VecInt64(Rc::new(value)) }
}

impl From<Vec<f64>> for Value {
    fn from(value: Vec<f64>) -> Self { Value::VecFloat64(Rc::new(value)) }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self { Value::List(Rc::new(value)) }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int64(v) => write!(f, "{}", v),
            Value::Float64(v) => write!(f, "{:.2}", v),
            Value::VecInt64(v) => write!(f, "{:?}", v),
            Value::VecFloat64(v) => write!(f, "{:?}", v),
            Value::List(v) => write!(f, "{:?}", v),
        }
    }
}
