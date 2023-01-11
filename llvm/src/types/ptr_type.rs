use super::{Type, TypeRef};
use crate::types::TypeIntrinsics;
// use llvm_sys::core::LLVMGetElementType;
use crate::values::prelude::PtrValue;
use llvm_sys::core::LLVMConstInt;

use llvm_sys::{
    core::{LLVMConstIntToPtr, LLVMGetTypeContext, LLVMInt64TypeInContext},
    prelude::LLVMTypeRef,
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct PtrType<'a> {
    ty: TypeRef<'a>,
}

impl<'a> PtrType<'a> {
    pub(crate) fn new(llvm_type: LLVMTypeRef) -> Self { Self { ty: TypeRef::new(llvm_type) } }

    pub fn const_value(self, ptr: *const ()) -> PtrValue<'a> {
        unsafe {
            let context = LLVMGetTypeContext(self.as_llvm_type_ref());
            let ty = LLVMInt64TypeInContext(context);
            let ptr = LLVMConstInt(ty, ptr as _, 0);
            PtrValue::new(LLVMConstIntToPtr(ptr, self.as_llvm_type_ref()))
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

    // pub fn get_element_type(self) -> Type<'a> { unsafe { Type::new(LLVMGetElementType(self.ty.llvm_type)) } }
}

impl<'a> TypeIntrinsics for PtrType<'a> {
    fn as_llvm_type_ref(&self) -> LLVMTypeRef { self.ty.as_llvm_type_ref() }
}
