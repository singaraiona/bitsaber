use super::ValueRef;
use crate::values::ValueIntrinsics;
use llvm_sys::core::LLVMConstIntGetSExtValue;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::prelude::LLVMValueRef;
use std::ffi::CStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct I1Value<'a> {
    val: ValueRef<'a>,
}

impl<'a> I1Value<'a> {
    pub(crate) fn new(llvm_value: LLVMValueRef) -> Self {
        Self {
            val: ValueRef::new(llvm_value),
        }
    }

    pub fn get_constant(self) -> bool {
        unsafe { LLVMConstIntGetSExtValue(self.as_llvm_value_ref()) != 0 }
    }
}

impl ValueIntrinsics for I1Value<'_> {
    fn as_llvm_value_ref(&self) -> LLVMValueRef {
        self.val.as_llvm_value_ref()
    }
    fn set_name(self, name: &str) {
        self.val.set_name(name)
    }

    fn get_name(&self) -> &CStr {
        self.val.get_name()
    }

    fn get_llvm_type_ref(&self) -> LLVMTypeRef {
        self.val.get_llvm_type_ref()
    }
}

impl Into<bool> for I1Value<'_> {
    fn into(self) -> bool {
        self.get_constant()
    }
}
