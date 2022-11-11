use super::{Value, ValueRef};
use crate::basic_block::BasicBlock;
use crate::types::{Type, TypeIntrinsics};
use crate::values::ValueIntrinsics;
use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyFunction};
use llvm_sys::core::*;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::prelude::LLVMValueRef;
use std::ffi::CStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct FnValue<'a> {
    val: ValueRef<'a>,
}

impl<'a> FnValue<'a> {
    pub(crate) fn new(llvm_value: LLVMValueRef) -> Self {
        Self {
            val: ValueRef::new(llvm_value),
        }
    }

    pub fn get_param_count(&self) -> usize {
        unsafe { LLVMCountParams(self.val.as_llvm_value_ref()) as usize }
    }

    pub fn get_param(&self, index: usize) -> Option<Value<'a>> {
        let param = unsafe { LLVMGetParam(self.val.as_llvm_value_ref(), index as u32) };
        if param.is_null() {
            None
        } else {
            Some(Value::new(param))
        }
    }

    pub fn get_params(&self) -> Vec<Value<'a>> {
        let mut params = Vec::new();
        for i in 0..self.get_param_count() {
            params.push(self.get_param(i).unwrap());
        }
        params
    }

    pub fn get_params_iter(&'a self) -> impl Iterator<Item = Value<'a>> + 'a {
        (0..self.get_param_count())
            .map(move |i| self.get_param(i))
            .flatten()
    }

    pub fn get_first_basic_block(self) -> Option<BasicBlock<'a>> {
        unsafe { BasicBlock::new(LLVMGetFirstBasicBlock(self.as_llvm_value_ref())) }
    }

    pub fn get_last_basic_block(self) -> Option<BasicBlock<'a>> {
        unsafe { BasicBlock::new(LLVMGetLastBasicBlock(self.as_llvm_value_ref())) }
    }

    pub fn verify(self) -> Result<(), String> {
        let code = unsafe {
            LLVMVerifyFunction(
                self.as_llvm_value_ref(),
                LLVMVerifierFailureAction::LLVMPrintMessageAction,
            )
        };

        if code != 0 {
            return Err(format!("Function is broken: {:?}", code));
        }

        Ok(())
    }

    pub fn delete(self) {
        unsafe { LLVMDeleteFunction(self.as_llvm_value_ref()) }
    }

    pub fn get_return_type(&self) -> Type<'a> {
        unsafe {
            let ptr_ty = self.val.get_llvm_type_ref();
            let fn_ty = LLVMGetElementType(ptr_ty);

            println!("fn_ty: {:?}", llvm_sys::core::LLVMGetTypeKind(fn_ty));
            let tp = LLVMGetReturnType(ptr_ty);
            Type::new(tp)
        }
    }

    // pub fn set_linkage(self, linkage: Linkage) {
    //     unsafe { LLVMSetLinkage(self.as_value_ref(), linkage.into()) }
    // }
}

impl ValueIntrinsics for FnValue<'_> {
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
