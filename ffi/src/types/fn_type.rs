use super::Type;
use crate::values::fn_value::FnValue;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FnType {
    pub args: Vec<Type>,
    pub ret: Box<Type>,
}

impl FnType {
    pub fn new(args: Vec<Type>, ret: Type) -> Self { Self { args, ret: Box::new(ret) } }

    pub fn const_value(&self, ptr: *const ()) -> FnValue { FnValue::new(self.clone(), ptr) }
}
