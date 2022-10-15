use super::{Value, ValueRef};
use llvm_sys::core::LLVMCountParams;
use llvm_sys::core::LLVMGetParam;
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

    pub fn get_param(&self, index: usize) -> Option<Value<'a>> {
        let param = unsafe { LLVMGetParam(self.val.as_llvm_value_ref(), index as u32) };
        if param.is_null() {
            None
        } else {
            Some(Value::new(param))
        }
    }

    pub fn get_param_count(&self) -> usize {
        unsafe { LLVMCountParams(self.val.as_llvm_value_ref()) as usize }
    }

    // pub fn get_name(&self) -> &str {
    //     self.val.get_name()
    // }

    // pub fn set_linkage(self, linkage: Linkage) {
    //     unsafe { LLVMSetLinkage(self.as_value_ref(), linkage.into()) }
    // }
}
