use super::TypeRef;
use crate::types::TypeIntrinsics;
use crate::values::vec_value::*;
use crate::values::Value;
use crate::values::ValueIntrinsics;
use llvm_sys::core::LLVMConstVector;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::prelude::LLVMValueRef;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct VecType<'a> {
    ty: TypeRef<'a>,
}

impl<'a> VecType<'a> {
    pub(crate) fn new(llvm_type: LLVMTypeRef) -> Self {
        assert!(!llvm_type.is_null());

        Self { ty: TypeRef::new(llvm_type) }
    }

    pub fn const_vector(&self, values: &[Value<'a>]) -> VecValue<'a> {
        let mut values: Vec<LLVMValueRef> = values.iter().map(|val| val.as_llvm_value_ref()).collect();
        unsafe { VecValue::new(LLVMConstVector(values.as_mut_ptr(), values.len() as u32)) }
    }
}

impl<'a> TypeIntrinsics for VecType<'a> {
    fn as_llvm_type_ref(&self) -> LLVMTypeRef { self.ty.as_llvm_type_ref() }
}
