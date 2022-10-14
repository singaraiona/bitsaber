use super::{Type, TypeRef};
use crate::values::i64_value::*;
use llvm_sys::core::LLVMConstInt;
use llvm_sys::prelude::LLVMTypeRef;

pub struct I64Type<'a> {
    pub(crate) ty: TypeRef<'a>,
}

impl<'a> I64Type<'a> {
    pub(crate) fn new(llvm_type: LLVMTypeRef) -> Self {
        Self {
            ty: TypeRef::new(llvm_type),
        }
    }

    pub fn const_value(self, value: i64) -> I64Value<'a> {
        unsafe {
            I64Value::new(LLVMConstInt(
                self.ty.llvm_type,
                value as u64,
                value.is_negative() as i32,
            ))
        }
    }

    // pub fn const_array(self, values: &[I64Value<'a>]) -> ArrayValue<'a> {
    //     let mut values: Vec<LLVMValueRef> = values.iter().map(|v| v.val.llvm_value).collect();
    //     unsafe {
    //         ArrayValue::new(LLVMConstArray(
    //             self.as_type_ref(),
    //             values.as_mut_ptr(),
    //             values.len() as u32,
    //         ))
    //     }
    // }
}
