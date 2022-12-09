use crate::basic_block::BasicBlock;
use crate::enums::*;
use crate::types::{prelude::*, Type, TypeIntrinsics};
use crate::utils::to_c_str;
use crate::values::instruction_value::InstructionValue;
use crate::values::ptr_value::PtrValue;
use crate::values::{prelude::*, Value, ValueIntrinsics};
use llvm_sys::core::*;
use llvm_sys::prelude::LLVMBuilderRef;
use std::marker::PhantomData;

pub struct Builder<'a> {
    llvm_builder: LLVMBuilderRef,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Builder<'a> {
    pub(crate) fn new(llvm_builder: LLVMBuilderRef) -> Builder<'a> { Builder { llvm_builder, _phantom: PhantomData } }

    pub fn position_at(&self, basic_block: BasicBlock<'a>, instruction: &InstructionValue<'a>) {
        unsafe { LLVMPositionBuilder(self.llvm_builder, basic_block.basic_block, instruction.as_llvm_value_ref()) }
    }

    pub fn position_before(&self, instruction: &InstructionValue<'a>) {
        unsafe { LLVMPositionBuilderBefore(self.llvm_builder, instruction.as_llvm_value_ref()) }
    }

    pub fn position_at_end(&self, basic_block: BasicBlock<'a>) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.llvm_builder, basic_block.basic_block);
        }
    }

    pub fn build_unconditional_branch(&self, destination_block: BasicBlock<'a>) -> Value<'a> {
        let value = unsafe { LLVMBuildBr(self.llvm_builder, destination_block.basic_block) };
        InstructionValue::new(value).into()
    }

    pub fn build_conditional_branch(
        &self,
        cmp: Value<'a>,
        then_block: BasicBlock<'a>,
        else_block: BasicBlock<'a>,
    ) -> Value<'a> {
        let value = unsafe {
            LLVMBuildCondBr(self.llvm_builder, cmp.as_llvm_value_ref(), then_block.basic_block, else_block.basic_block)
        };
        InstructionValue::new(value).into()
    }

    // pub fn build_indirect_branch<BV: BasicValue<'ctx>>(
    //     &self,
    //     address: BV,
    //     destinations: &[BasicBlock<'ctx>],
    // ) -> InstructionValue<'ctx> {
    //     let value = unsafe {
    //         LLVMBuildIndirectBr(
    //             self.builder,
    //             address.as_value_ref(),
    //             destinations.len() as u32,
    //         )
    //     };

    //     for destination in destinations {
    //         unsafe { LLVMAddDestination(value, destination.basic_block) }
    //     }

    //     unsafe { InstructionValue::new(value) }
    // }

    pub fn get_insert_block(&self) -> Option<BasicBlock<'a>> {
        unsafe { BasicBlock::new(LLVMGetInsertBlock(self.llvm_builder)) }
    }

    pub fn build_alloca(&self, ty: Type<'a>, name: &str) -> Value<'a> {
        let c_string = to_c_str(name);
        let value = unsafe { LLVMBuildAlloca(self.llvm_builder, ty.as_llvm_type_ref(), c_string.as_ptr()) };
        PtrValue::new(value).into()
    }

    pub fn build_phi(&self, ty: Type<'a>, name: &str) -> Value<'a> {
        let c_string = to_c_str(name);
        let value = unsafe { LLVMBuildPhi(self.llvm_builder, ty.as_llvm_type_ref(), c_string.as_ptr()) };
        PhiValue::new(value).into()
    }

    pub fn build_return(&self, value: Value<'a>) -> Value<'a> {
        let value = unsafe { LLVMBuildRet(self.llvm_builder, value.as_llvm_value_ref()) };
        InstructionValue::new(value).into()
    }

    pub fn build_aggregate_return(&self, values: &[Value<'a>]) -> Value<'a> {
        unsafe {
            let mut args: Vec<_> = values.iter().map(|val| val.as_llvm_value_ref()).collect();
            let value = LLVMBuildAggregateRet(self.llvm_builder, args.as_mut_ptr(), args.len() as u32);
            InstructionValue::new(value).into()
        }
    }

    pub fn build_store(&self, ptr: Value<'a>, value: Value<'a>) -> Value<'a> {
        let ptr_value: PtrValue = ptr.into();
        let value =
            unsafe { LLVMBuildStore(self.llvm_builder, value.as_llvm_value_ref(), ptr_value.as_llvm_value_ref()) };

        InstructionValue::new(value).into()
    }

    pub fn build_load(&self, ptr: Value<'a>, name: &str) -> Value<'a> {
        let c_string = to_c_str(name);
        let value = unsafe {
            LLVMBuildLoad2(
                self.llvm_builder,
                LLVMGetElementType(ptr.get_llvm_type_ref()),
                ptr.as_llvm_value_ref(),
                c_string.as_ptr(),
            )
        };

        Value::new(value)
    }

    pub fn build_call(&self, function: Value<'a>, args: &[Value<'a>], name: &str) -> Value<'a> {
        let fn_value: FnValue<'_> = function.into();
        let c_string = to_c_str(name);
        let mut args: Vec<_> = args.iter().map(|val| val.as_llvm_value_ref()).collect();
        let value = unsafe {
            LLVMBuildCall2(
                self.llvm_builder,
                fn_value.get_llvm_type_ref(),
                fn_value.as_llvm_value_ref(),
                args.as_mut_ptr(),
                args.len() as u32,
                c_string.as_ptr(),
            )

            // LLVMBuildCall(
            //     self.llvm_builder,
            //     fn_value.as_llvm_value_ref(),
            //     args.as_mut_ptr(),
            //     args.len() as u32,
            //     c_string.as_ptr(),
            // )
        };

        // unsafe { println!("build_call: {:?}", LLVMGetTypeKind(LLVMTypeOf(value))) };

        Value::new(value)
    }

    pub unsafe fn build_gep(&self, ptr: Value<'a>, indexes: &[Value<'a>], name: &str) -> Value<'a> {
        let c_string = to_c_str(name);

        let mut index_values: Vec<_> = indexes.iter().map(|val| val.as_llvm_value_ref()).collect();

        let value = LLVMBuildGEP(
            self.llvm_builder,
            ptr.as_llvm_value_ref(),
            index_values.as_mut_ptr(),
            index_values.len() as u32,
            c_string.as_ptr(),
        );

        Value::new(value)
    }

    // -- OPS

    pub fn build_int_add(&self, lhs: Value<'a>, rhs: Value<'a>, name: &str) -> Value<'a> {
        unsafe {
            let c_string = to_c_str(name);
            Value::new(LLVMBuildAdd(
                self.llvm_builder,
                lhs.as_llvm_value_ref(),
                rhs.as_llvm_value_ref(),
                c_string.as_ptr(),
            ))
        }
    }

    pub fn build_float_add(&self, lhs: Value<'a>, rhs: Value<'a>, name: &str) -> Value<'a> {
        unsafe {
            let c_string = to_c_str(name);
            Value::new(LLVMBuildFAdd(
                self.llvm_builder,
                lhs.as_llvm_value_ref(),
                rhs.as_llvm_value_ref(),
                c_string.as_ptr(),
            ))
        }
    }

    pub fn build_int_sub(&self, lhs: Value<'a>, rhs: Value<'a>, name: &str) -> Value<'a> {
        unsafe {
            let c_string = to_c_str(name);
            Value::new(LLVMBuildSub(
                self.llvm_builder,
                lhs.as_llvm_value_ref(),
                rhs.as_llvm_value_ref(),
                c_string.as_ptr(),
            ))
        }
    }

    pub fn build_float_sub(&self, lhs: Value<'a>, rhs: Value<'a>, name: &str) -> Value<'a> {
        unsafe {
            let c_string = to_c_str(name);
            Value::new(LLVMBuildFSub(
                self.llvm_builder,
                lhs.as_llvm_value_ref(),
                rhs.as_llvm_value_ref(),
                c_string.as_ptr(),
            ))
        }
    }

    pub fn build_int_mul(&self, lhs: Value<'a>, rhs: Value<'a>, name: &str) -> Value<'a> {
        unsafe {
            let c_string = to_c_str(name);
            Value::new(LLVMBuildMul(
                self.llvm_builder,
                lhs.as_llvm_value_ref(),
                rhs.as_llvm_value_ref(),
                c_string.as_ptr(),
            ))
        }
    }

    pub fn build_float_mul(&self, lhs: Value<'a>, rhs: Value<'a>, name: &str) -> Value<'a> {
        unsafe {
            let c_string = to_c_str(name);
            Value::new(LLVMBuildFMul(
                self.llvm_builder,
                lhs.as_llvm_value_ref(),
                rhs.as_llvm_value_ref(),
                c_string.as_ptr(),
            ))
        }
    }

    pub fn build_int_div(&self, lhs: Value<'a>, rhs: Value<'a>, name: &str) -> Value<'a> {
        unsafe {
            let c_string = to_c_str(name);
            Value::new(LLVMBuildSDiv(
                self.llvm_builder,
                lhs.as_llvm_value_ref(),
                rhs.as_llvm_value_ref(),
                c_string.as_ptr(),
            ))
        }
    }

    pub fn build_float_div(&self, lhs: Value<'a>, rhs: Value<'a>, name: &str) -> Value<'a> {
        unsafe {
            let c_string = to_c_str(name);
            Value::new(LLVMBuildFDiv(
                self.llvm_builder,
                lhs.as_llvm_value_ref(),
                rhs.as_llvm_value_ref(),
                c_string.as_ptr(),
            ))
        }
    }

    pub fn build_rem(&self, lhs: Value<'a>, rhs: Value<'a>, name: &str) -> Value<'a> {
        unsafe {
            let c_string = to_c_str(name);
            Value::new(LLVMBuildSRem(
                self.llvm_builder,
                lhs.as_llvm_value_ref(),
                rhs.as_llvm_value_ref(),
                c_string.as_ptr(),
            ))
        }
    }

    pub fn build_and(&self, lhs: Value<'a>, rhs: Value<'a>, name: &str) -> Value<'a> {
        unsafe {
            let c_string = to_c_str(name);
            Value::new(LLVMBuildAnd(
                self.llvm_builder,
                lhs.as_llvm_value_ref(),
                rhs.as_llvm_value_ref(),
                c_string.as_ptr(),
            ))
        }
    }

    pub fn build_or(&self, lhs: Value<'a>, rhs: Value<'a>, name: &str) -> Value<'a> {
        unsafe {
            let c_string = to_c_str(name);
            Value::new(LLVMBuildOr(
                self.llvm_builder,
                lhs.as_llvm_value_ref(),
                rhs.as_llvm_value_ref(),
                c_string.as_ptr(),
            ))
        }
    }

    pub fn build_xor(&self, lhs: Value<'a>, rhs: Value<'a>, name: &str) -> Value<'a> {
        unsafe {
            let c_string = to_c_str(name);
            Value::new(LLVMBuildXor(
                self.llvm_builder,
                lhs.as_llvm_value_ref(),
                rhs.as_llvm_value_ref(),
                c_string.as_ptr(),
            ))
        }
    }

    pub fn build_shl(&self, lhs: Value<'a>, rhs: Value<'a>, name: &str) -> Value<'a> {
        unsafe {
            let c_string = to_c_str(name);
            Value::new(LLVMBuildShl(
                self.llvm_builder,
                lhs.as_llvm_value_ref(),
                rhs.as_llvm_value_ref(),
                c_string.as_ptr(),
            ))
        }
    }

    pub fn build_lshr(&self, lhs: Value<'a>, rhs: Value<'a>, name: &str) -> Value<'a> {
        unsafe {
            let c_string = to_c_str(name);
            Value::new(LLVMBuildLShr(
                self.llvm_builder,
                lhs.as_llvm_value_ref(),
                rhs.as_llvm_value_ref(),
                c_string.as_ptr(),
            ))
        }
    }

    pub fn build_ashr(&self, lhs: Value<'a>, rhs: Value<'a>, name: &str) -> Value<'a> {
        unsafe {
            let c_string = to_c_str(name);
            Value::new(LLVMBuildAShr(
                self.llvm_builder,
                lhs.as_llvm_value_ref(),
                rhs.as_llvm_value_ref(),
                c_string.as_ptr(),
            ))
        }
    }

    pub fn build_neg(&self, val: Value<'a>, name: &str) -> Value<'a> {
        unsafe {
            let c_string = to_c_str(name);
            Value::new(LLVMBuildNeg(self.llvm_builder, val.as_llvm_value_ref(), c_string.as_ptr()))
        }
    }

    pub fn build_not(&self, val: Value<'a>, name: &str) -> Value<'a> {
        unsafe {
            let c_string = to_c_str(name);
            Value::new(LLVMBuildNot(self.llvm_builder, val.as_llvm_value_ref(), c_string.as_ptr()))
        }
    }

    pub fn build_int_compare(&self, predicate: IntPredicate, lhs: Value<'a>, rhs: Value<'a>, name: &str) -> Value<'a> {
        unsafe {
            let c_string = to_c_str(name);
            Value::new(LLVMBuildICmp(
                self.llvm_builder,
                predicate.into(),
                lhs.as_llvm_value_ref(),
                rhs.as_llvm_value_ref(),
                c_string.as_ptr(),
            ))
        }
    }

    pub fn build_float_compare(
        &self,
        predicate: FloatPredicate,
        lhs: Value<'a>,
        rhs: Value<'a>,
        name: &str,
    ) -> Value<'a> {
        unsafe {
            let c_string = to_c_str(name);
            Value::new(LLVMBuildFCmp(
                self.llvm_builder,
                predicate.into(),
                lhs.as_llvm_value_ref(),
                rhs.as_llvm_value_ref(),
                c_string.as_ptr(),
            ))
        }
    }

    pub fn build_extract_value(&self, value: Value<'a>, index: u32, name: &str) -> Option<Value<'a>> {
        unsafe {
            // let size = value.get_type().count_fields();

            // if index >= size {
            //     return None;
            // }

            // TODO: check if index is valid

            let c_string = to_c_str(name);
            let value = LLVMBuildExtractValue(self.llvm_builder, value.as_llvm_value_ref(), index, c_string.as_ptr());

            Some(Value::new(value))
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
