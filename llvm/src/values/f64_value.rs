use super::ValueRef;
use crate::values::ValueIntrinsics;
use llvm_sys::core::LLVMConstRealGetDouble;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::prelude::LLVMValueRef;
use std::ffi::CStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct F64Value<'a> {
    val: ValueRef<'a>,
}

impl<'a> F64Value<'a> {
    pub(crate) fn new(llvm_value: LLVMValueRef) -> Self { Self { val: ValueRef::new(llvm_value) } }

    pub fn get_constant(self) -> f64 {
        let mut lossy = 0;
        let constant = unsafe { LLVMConstRealGetDouble(self.val.as_llvm_value_ref(), &mut lossy) };
        constant
    }
}

impl Into<f64> for F64Value<'_> {
    fn into(self) -> f64 { self.get_constant() }
}

impl ValueIntrinsics for F64Value<'_> {
    fn as_llvm_value_ref(&self) -> LLVMValueRef { self.val.as_llvm_value_ref() }
    fn set_name(&mut self, name: &str) { self.val.set_name(name) }

    fn get_name(&self) -> &CStr { self.val.get_name() }

    fn get_llvm_type_ref(&self) -> LLVMTypeRef { self.val.get_llvm_type_ref() }
}
