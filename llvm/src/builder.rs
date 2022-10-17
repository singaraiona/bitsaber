use crate::basic_block::BasicBlock;
use crate::utils::to_c_str;
use crate::values::i64_value::I64Value;
use crate::values::instruction_value::InstructionValue;
use crate::values::AsLLVMValueRef;
use crate::values::Value;
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

    pub fn position_at(&self, basic_block: BasicBlock<'a>, instruction: &InstructionValue<'a>) {
        unsafe {
            LLVMPositionBuilder(
                self.llvm_builder,
                basic_block.basic_block,
                instruction.as_llvm_value_ref(),
            )
        }
    }

    pub fn position_before(&self, instruction: &InstructionValue<'a>) {
        unsafe { LLVMPositionBuilderBefore(self.llvm_builder, instruction.as_llvm_value_ref()) }
    }

    pub fn position_at_end(&self, basic_block: BasicBlock<'a>) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.llvm_builder, basic_block.basic_block);
        }
    }

    pub fn build_i64_add(&self, lhs: I64Value<'_>, rhs: I64Value<'_>, name: &str) -> I64Value<'a> {
        unsafe {
            let c_string = to_c_str(name);
            I64Value::new(LLVMBuildAdd(
                self.llvm_builder,
                lhs.as_llvm_value_ref(),
                rhs.as_llvm_value_ref(),
                c_string.as_ptr(),
            ))
        }
    }

    pub fn build_return(&self, value: Value<'a>) -> InstructionValue<'a> {
        let value = unsafe { LLVMBuildRet(self.llvm_builder, value.as_llvm_value_ref()) };
        InstructionValue::new(value)
    }
}

impl<'a> Drop for Builder<'a> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.llvm_builder);
        }
    }
}
