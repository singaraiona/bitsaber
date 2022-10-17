use super::{Type, TypeRef};
use crate::types::AsLLVMTypeRef;
use crate::values::struct_value::*;
use crate::values::AsLLVMValueRef;
use crate::values::Value;
use llvm_sys::core::LLVMConstStruct;
use llvm_sys::core::LLVMConstStructInContext;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::prelude::LLVMValueRef;

pub struct StructType<'a> {
    pub(crate) ty: TypeRef<'a>,
}

impl<'a> StructType<'a> {
    pub(crate) fn new(llvm_type: LLVMTypeRef) -> Self {
        Self {
            ty: TypeRef::new(llvm_type),
        }
    }

    pub fn const_value<'ctx>(&self, values: &[Value<'a>], packed: bool) -> StructValue<'ctx> {
        let mut args: Vec<LLVMValueRef> =
            values.iter().map(|val| val.as_llvm_value_ref()).collect();
        unsafe {
            StructValue::new(LLVMConstStruct(
                args.as_mut_ptr(),
                args.len() as u32,
                packed as i32,
            ))
        }
    }
}

impl<'a> AsLLVMTypeRef for StructType<'a> {
    fn as_llvm_type_ref(&self) -> LLVMTypeRef {
        self.ty.as_llvm_type_ref()
    }
}
