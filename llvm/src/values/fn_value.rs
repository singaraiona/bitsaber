use super::{Value, ValueRef};
use crate::values::AsLLVMValueRef;
use llvm_sys::core::LLVMCountParams;
use llvm_sys::core::LLVMGetParam;
use llvm_sys::prelude::LLVMValueRef;

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

    // pub fn get_name(&self) -> &str {
    //     self.val.get_name()
    // }

    // pub fn set_linkage(self, linkage: Linkage) {
    //     unsafe { LLVMSetLinkage(self.as_value_ref(), linkage.into()) }
    // }
}

impl AsLLVMValueRef<'_> for FnValue<'_> {
    fn as_llvm_value_ref(&self) -> LLVMValueRef {
        self.val.as_llvm_value_ref()
    }
}
