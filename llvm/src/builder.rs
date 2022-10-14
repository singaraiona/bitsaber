//
use llvm_sys::core::*;
use llvm_sys::prelude::LLVMBuilderRef;
use std::marker::PhantomData;

pub struct Builder<'a> {
    llvm_builder: LLVMBuilderRef,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Builder<'a> {
    pub(crate) fn new(llvm_builder: LLVMBuilderRef) -> Builder<'a> {
        Builder {
            llvm_builder,
            _phantom: PhantomData,
        }
    }
}

impl<'a> Drop for Builder<'a> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.llvm_builder);
        }
    }
}
