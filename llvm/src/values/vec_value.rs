use super::ValueRef;
use crate::values::ValueIntrinsics;
use llvm_sys::core::LLVMConstRealGetDouble;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::prelude::LLVMValueRef;
use std::ffi::CStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct VecValue<'a> {
    val: ValueRef<'a>,
}

impl<'a> VecValue<'a> {
    pub(crate) fn new(llvm_value: LLVMValueRef) -> Self { Self { val: ValueRef::new(llvm_value) } }
}

impl ValueIntrinsics for VecValue<'_> {
    fn as_llvm_value_ref(&self) -> LLVMValueRef { self.val.as_llvm_value_ref() }

    fn set_name(&mut self, name: &str) { self.val.set_name(name) }

    fn get_name(&self) -> &CStr { self.val.get_name() }

    fn get_llvm_type_ref(&self) -> LLVMTypeRef { self.val.get_llvm_type_ref() }
}
