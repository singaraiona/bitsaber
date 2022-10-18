use super::{Type, TypeRef};
use crate::types::TypeIntrinsics;
use crate::values::f64_value::*;
use llvm_sys::core::LLVMConstReal;
use llvm_sys::prelude::LLVMTypeRef;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct F64Type<'a> {
    ty: TypeRef<'a>,
}

impl<'a> F64Type<'a> {
    pub(crate) fn new(llvm_type: LLVMTypeRef) -> Self {
        Self {
            ty: TypeRef::new(llvm_type),
        }
    }

    pub fn const_value(self, value: f64) -> F64Value<'a> {
        unsafe { F64Value::new(LLVMConstReal(self.ty.llvm_type, value)) }
    }

    // pub fn fn_type(self, param_types: &[Type<'a>], is_var_args: bool) -> FunctionType<'a> {
    //     // self.ty.fn_type(param_types, is_var_args)
    //     todo!();
    // }

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

impl<'a> TypeIntrinsics for F64Type<'a> {
    fn as_llvm_type_ref(&self) -> LLVMTypeRef {
        self.ty.as_llvm_type_ref()
    }
}
