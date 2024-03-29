use super::{Value, ValueRef};
use crate::basic_block::BasicBlock;
use crate::types::Type;
use crate::values::ValueIntrinsics;
use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyFunction};
use llvm_sys::core::*;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMTypeKind;
use std::ffi::CStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct FnValue<'a> {
    val: ValueRef<'a>,
}

impl<'a> FnValue<'a> {
    pub(crate) fn new(llvm_value: LLVMValueRef) -> Self {
        unsafe { assert!(!LLVMIsAFunction(llvm_value).is_null()) }

        Self { val: ValueRef::new(llvm_value) }
    }

    pub fn get_param_count(&self) -> usize { unsafe { LLVMCountParams(self.val.as_llvm_value_ref()) as usize } }

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
        (0..self.get_param_count()).map(move |i| self.get_param(i)).flatten()
    }

    pub fn get_first_basic_block(self) -> Option<BasicBlock<'a>> {
        unsafe { BasicBlock::new(LLVMGetFirstBasicBlock(self.as_llvm_value_ref())) }
    }

    pub fn get_last_basic_block(self) -> Option<BasicBlock<'a>> {
        unsafe { BasicBlock::new(LLVMGetLastBasicBlock(self.as_llvm_value_ref())) }
    }

    pub fn verify(self) -> Result<(), String> {
        let code =
            unsafe { LLVMVerifyFunction(self.as_llvm_value_ref(), LLVMVerifierFailureAction::LLVMPrintMessageAction) };

        if code != 0 {
            println!();
            return Err(format!("Function verification failed with code: {:?}", code));
        }

        Ok(())
    }

    pub fn delete(self) { unsafe { LLVMDeleteFunction(self.as_llvm_value_ref()) } }

    // pub fn get_return_type(&self) -> Type<'a> { unsafe { Type::new(LLVMGetReturnType(self.val.get_llvm_type_ref())) } }

    // pub fn set_linkage(self, linkage: Linkage) {
    //     unsafe { LLVMSetLinkage(self.as_value_ref(), linkage.into()) }
    // }
}

impl ValueIntrinsics for FnValue<'_> {
    fn as_llvm_value_ref(&self) -> LLVMValueRef { self.val.as_llvm_value_ref() }

    fn set_name(&mut self, name: &str) { self.val.set_name(name) }

    fn get_name(&self) -> &CStr { self.val.get_name() }

    fn get_llvm_type_ref(&self) -> LLVMTypeRef {
        unsafe {
            let llvm_ty_ref = self.val.get_llvm_type_ref();
            LLVMInt64Type()
            // match LLVMGetTypeKind(llvm_ty_ref) {
            //     LLVMTypeKind::LLVMFunctionTypeKind => llvm_ty_ref,
            //     LLVMTypeKind::LLVMPointerTypeKind => LLVMGetElementType(llvm_ty_ref),
            //     _ => unreachable!(),
            // }
        }
    }
}
