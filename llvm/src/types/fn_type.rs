use super::{Type, TypeRef};
use crate::types::TypeIntrinsics;
use crate::values::i64_value::*;
use llvm_sys::core::LLVMConstInt;
use llvm_sys::prelude::LLVMTypeRef;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct FnType<'a> {
    ty: TypeRef<'a>,
}

impl<'a> FnType<'a> {
    pub(crate) fn new(llvm_type: LLVMTypeRef) -> Self {
        Self {
            ty: TypeRef::new(llvm_type),
        }
    }
}

impl<'a> TypeIntrinsics for FnType<'a> {
    fn as_llvm_type_ref(&self) -> LLVMTypeRef {
        self.ty.as_llvm_type_ref()
    }
}
