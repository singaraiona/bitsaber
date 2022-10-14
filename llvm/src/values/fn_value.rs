use super::ValueRef;
use llvm_sys::prelude::LLVMValueRef;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct FnValue<'a> {
    pub(crate) val: ValueRef<'a>,
}

impl<'a> FnValue<'a> {
    pub(crate) fn new(llvm_value: LLVMValueRef) -> Self {
        Self {
            val: ValueRef::new(llvm_value),
        }
    }

    // pub fn set_linkage(self, linkage: Linkage) {
    //     unsafe { LLVMSetLinkage(self.as_value_ref(), linkage.into()) }
    // }
}
