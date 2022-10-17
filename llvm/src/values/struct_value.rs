use super::ValueRef;
use crate::values::AsLLVMValueRef;
use llvm_sys::prelude::LLVMValueRef;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct StructValue<'a> {
    val: ValueRef<'a>,
}

impl<'a> StructValue<'a> {
    pub(crate) fn new(llvm_value: LLVMValueRef) -> Self {
        Self {
            val: ValueRef::new(llvm_value),
        }
    }
}

impl AsLLVMValueRef<'_> for StructValue<'_> {
    fn as_llvm_value_ref(&self) -> LLVMValueRef {
        self.val.as_llvm_value_ref()
    }
}
