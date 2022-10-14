use super::{Type, TypeRef};
use crate::values::i64_value::*;
use llvm_sys::core::LLVMConstInt;
use llvm_sys::prelude::LLVMTypeRef;

pub struct FnType<'a> {
    pub(crate) ty: TypeRef<'a>,
}

impl<'a> FnType<'a> {
    pub(crate) fn new(llvm_type: LLVMTypeRef) -> Self {
        Self {
            ty: TypeRef::new(llvm_type),
        }
    }
}
