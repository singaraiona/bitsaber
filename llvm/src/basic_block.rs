use crate::values::fn_value::FnValue;
use llvm_sys::core::*;
use llvm_sys::prelude::{LLVMBasicBlockRef, LLVMValueRef};
use std::marker::PhantomData;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct BasicBlock<'a> {
    pub(crate) basic_block: LLVMBasicBlockRef,
    _marker: PhantomData<&'a ()>,
}

impl<'a> BasicBlock<'a> {
    pub(crate) unsafe fn new(basic_block: LLVMBasicBlockRef) -> Option<Self> {
        if basic_block.is_null() {
            return None;
        }

        // NOTE: There is a LLVMBasicBlockAsValue but it might be the same as casting
        assert!(!LLVMIsABasicBlock(basic_block as LLVMValueRef).is_null());

        Some(BasicBlock {
            basic_block,
            _marker: PhantomData,
        })
    }

    pub fn get_parent(self) -> Option<FnValue<'a>> {
        // TODO!!!!!
        unsafe { Some(FnValue::new(LLVMGetBasicBlockParent(self.basic_block))) }
    }

    pub fn get_previous_basic_block(self) -> Option<BasicBlock<'a>> {
        self.get_parent()?;
        unsafe { BasicBlock::new(LLVMGetPreviousBasicBlock(self.basic_block)) }
    }

    pub fn get_next_basic_block(self) -> Option<BasicBlock<'a>> {
        self.get_parent()?;
        unsafe { BasicBlock::new(LLVMGetNextBasicBlock(self.basic_block)) }
    }
}