use super::{Type, TypeRef};
use crate::types::TypeIntrinsics;
use crate::values::bool_value::*;
use llvm_sys::core::LLVMConstInt;
use llvm_sys::prelude::LLVMTypeRef;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct BoolType<'a> {
    ty: TypeRef<'a>,
}

impl<'a> BoolType<'a> {
    pub(crate) fn new(llvm_type: LLVMTypeRef) -> Self {
        Self {
            ty: TypeRef::new(llvm_type),
        }
    }

    pub fn const_value(self, value: bool) -> BoolValue<'a> {
        unsafe { BoolValue::new(LLVMConstInt(self.ty.llvm_type, value as u64, 0)) }
    }
}

impl<'a> TypeIntrinsics for BoolType<'a> {
    fn as_llvm_type_ref(&self) -> LLVMTypeRef {
        self.ty.as_llvm_type_ref()
    }
}
