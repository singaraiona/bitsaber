use super::ValueRef;
use crate::values::i64_value::I64Value;
use crate::values::ValueIntrinsics;
use llvm_sys::core::LLVMConstPtrToInt;
use llvm_sys::core::LLVMGetTypeContext;
use llvm_sys::core::LLVMInt64TypeInContext;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::prelude::LLVMValueRef;
use std::ffi::CStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct PtrValue<'a> {
    val: ValueRef<'a>,
}

impl<'a> PtrValue<'a> {
    pub(crate) fn new(llvm_value: LLVMValueRef) -> Self { Self { val: ValueRef::new(llvm_value) } }

    pub fn const_to_i64(self) -> I64Value<'a> {
        unsafe {
            let context = LLVMGetTypeContext(self.get_llvm_type_ref());
            let ty = LLVMInt64TypeInContext(context);
            I64Value::new(LLVMConstPtrToInt(self.as_llvm_value_ref(), ty))
        }
    }
}

impl ValueIntrinsics for PtrValue<'_> {
    fn as_llvm_value_ref(&self) -> LLVMValueRef { self.val.as_llvm_value_ref() }

    fn set_name(&mut self, name: &str) { self.val.set_name(name) }

    fn get_name(&self) -> &CStr { self.val.get_name() }

    fn get_llvm_type_ref(&self) -> LLVMTypeRef { self.val.get_llvm_type_ref() }
}
