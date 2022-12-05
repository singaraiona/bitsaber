use super::ValueRef;
use crate::context::Context;
use crate::values::i64_value::I64Value;
use crate::values::ValueIntrinsics;
use llvm_sys::core::LLVMConstExtractElement;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::prelude::LLVMValueRef;
use std::ffi::CStr;
use std::mem::transmute;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct StructValue<'a> {
    val: ValueRef<'a>,
}

impl<'a> StructValue<'a> {
    pub(crate) fn new(llvm_value: LLVMValueRef) -> Self { Self { val: ValueRef::new(llvm_value) } }

    pub fn get_constant(self, context: &'a Context) -> (i64, i64) {
        unsafe {
            let tag = context.i64_type().const_value(0);
            let val = context.i64_type().const_value(1);
            let t = LLVMConstExtractElement(self.val.as_llvm_value_ref(), tag.as_llvm_value_ref());
            let v = LLVMConstExtractElement(self.val.as_llvm_value_ref(), val.as_llvm_value_ref());
            let tag = I64Value::new(t).get_constant();
            let val = I64Value::new(v).get_constant();
            (tag, val)
        }
    }
}

impl ValueIntrinsics for StructValue<'_> {
    fn as_llvm_value_ref(&self) -> LLVMValueRef { self.val.as_llvm_value_ref() }

    fn set_name(self, name: &str) { self.val.set_name(name) }

    fn get_name(&self) -> &CStr { self.val.get_name() }

    fn get_llvm_type_ref(&self) -> LLVMTypeRef { self.val.get_llvm_type_ref() }
}
