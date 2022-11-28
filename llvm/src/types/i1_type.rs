use super::TypeRef;
use crate::types::TypeIntrinsics;
use crate::values::i1_value::*;
use llvm_sys::core::LLVMConstInt;
use llvm_sys::prelude::LLVMTypeRef;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct I1Type<'a> {
    ty: TypeRef<'a>,
}

impl<'a> I1Type<'a> {
    pub(crate) fn new(llvm_type: LLVMTypeRef) -> Self {
        Self {
            ty: TypeRef::new(llvm_type),
        }
    }

    pub fn const_value(self, value: bool) -> I1Value<'a> {
        unsafe { I1Value::new(LLVMConstInt(self.ty.llvm_type, value as u64, 0)) }
    }
}

impl<'a> TypeIntrinsics for I1Type<'a> {
    fn as_llvm_type_ref(&self) -> LLVMTypeRef {
        self.ty.as_llvm_type_ref()
    }
}
