use super::ValueRef;
use crate::basic_block::BasicBlock;
use crate::values::Value;
use crate::values::ValueIntrinsics;
use llvm_sys::core::LLVMAddIncoming;
use llvm_sys::core::LLVMCountIncoming;
use llvm_sys::core::LLVMGetIncomingBlock;
use llvm_sys::core::LLVMGetIncomingValue;
use llvm_sys::prelude::*;
use std::ffi::CStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct PhiValue<'a> {
    val: ValueRef<'a>,
}

impl<'a> PhiValue<'a> {
    pub(crate) fn new(llvm_value: LLVMValueRef) -> Self {
        Self {
            val: ValueRef::new(llvm_value),
        }
    }

    pub fn add_incoming(self, incoming: &[(Value<'a>, BasicBlock<'a>)]) {
        let (mut values, mut basic_blocks): (Vec<LLVMValueRef>, Vec<LLVMBasicBlockRef>) = {
            incoming
                .iter()
                .map(|&(v, bb)| (v.as_llvm_value_ref(), bb.basic_block))
                .unzip()
        };

        unsafe {
            LLVMAddIncoming(
                self.as_llvm_value_ref(),
                values.as_mut_ptr(),
                basic_blocks.as_mut_ptr(),
                incoming.len() as u32,
            );
        }
    }

    pub fn count_incoming(self) -> u32 {
        unsafe { LLVMCountIncoming(self.as_llvm_value_ref()) }
    }

    pub fn get_incoming(self, index: u32) -> Option<(Value<'a>, BasicBlock<'a>)> {
        if index >= self.count_incoming() {
            return None;
        }

        let basic_block = unsafe {
            BasicBlock::new(LLVMGetIncomingBlock(self.as_llvm_value_ref(), index))
                .expect("Invalid BasicBlock")
        };
        let value = unsafe { Value::new(LLVMGetIncomingValue(self.as_llvm_value_ref(), index)) };

        Some((value, basic_block))
    }
}

impl ValueIntrinsics for PhiValue<'_> {
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
