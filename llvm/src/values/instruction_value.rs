use super::ValueRef;
use crate::values::AsLLVMValueRef;
use llvm_sys::prelude::LLVMValueRef;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct InstructionValue<'a> {
    val: ValueRef<'a>,
}

impl<'a> InstructionValue<'a> {
    pub(crate) fn new(llvm_value: LLVMValueRef) -> Self {
        Self {
            val: ValueRef::new(llvm_value),
        }
    }
}

impl AsLLVMValueRef<'_> for InstructionValue<'_> {
    fn as_llvm_value_ref(&self) -> LLVMValueRef {
        self.val.as_llvm_value_ref()
    }
}
