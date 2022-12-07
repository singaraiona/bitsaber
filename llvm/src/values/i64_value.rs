use super::ValueRef;
use crate::types::ptr_type::*;
use crate::types::TypeIntrinsics;
use crate::values::ptr_value::*;
use crate::values::ValueIntrinsics;
use llvm_sys::core::LLVMConstIntGetSExtValue;
use llvm_sys::core::LLVMConstIntToPtr;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::prelude::LLVMValueRef;
use std::ffi::CStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct I64Value<'a> {
    val: ValueRef<'a>,
}

impl<'a> I64Value<'a> {
    pub(crate) fn new(llvm_value: LLVMValueRef) -> Self { Self { val: ValueRef::new(llvm_value) } }

    pub fn get_constant(self) -> i64 { unsafe { LLVMConstIntGetSExtValue(self.as_llvm_value_ref()) } }

    pub fn to_ptr(self, ty: PtrType) -> PtrValue<'a> {
        unsafe { PtrValue::new(LLVMConstIntToPtr(self.as_llvm_value_ref(), ty.as_llvm_type_ref())) }
    }
}

impl Into<i64> for I64Value<'_> {
    fn into(self) -> i64 { self.get_constant() }
}

impl ValueIntrinsics for I64Value<'_> {
    fn as_llvm_value_ref(&self) -> LLVMValueRef { self.val.as_llvm_value_ref() }
    fn set_name(self, name: &str) { self.val.set_name(name) }

    fn get_name(&self) -> &CStr { self.val.get_name() }

    fn get_llvm_type_ref(&self) -> LLVMTypeRef { self.val.get_llvm_type_ref() }
}
