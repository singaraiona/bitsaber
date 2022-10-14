use llvm_sys::prelude::LLVMTypeRef;
use std::marker::PhantomData;

pub mod f64_type;
pub mod fn_type;
pub mod i64_type;
use f64_type::F64Type;
use fn_type::FnType;
use i64_type::I64Type;

pub(crate) struct TypeRef<'a> {
    llvm_type: LLVMTypeRef,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> TypeRef<'a> {
    pub(crate) fn new(llvm_type: LLVMTypeRef) -> TypeRef<'a> {
        debug_assert!(!llvm_type.is_null());

        TypeRef {
            llvm_type,
            _phantom: PhantomData,
        }
    }

    pub fn as_llvm_type_ref(&self) -> LLVMTypeRef {
        self.llvm_type
    }
}

impl Into<LLVMTypeRef> for TypeRef<'_> {
    fn into(self) -> LLVMTypeRef {
        self.llvm_type
    }
}

pub enum Type<'a> {
    I64(I64Type<'a>),
    F64(F64Type<'a>),
    Fn(FnType<'a>),
}

impl<'a> From<I64Type<'a>> for Type<'a> {
    fn from(ty: I64Type<'a>) -> Self {
        Self::I64(ty)
    }
}

impl<'a> From<F64Type<'a>> for Type<'a> {
    fn from(ty: F64Type<'a>) -> Self {
        Self::F64(ty)
    }
}

impl<'a> From<FnType<'a>> for Type<'a> {
    fn from(ty: FnType<'a>) -> Self {
        Self::Fn(ty)
    }
}

impl<'a> Type<'a> {
    pub(crate) fn new(llvm_type: LLVMTypeRef) -> Type<'a> {
        match unsafe { llvm_sys::core::LLVMGetTypeKind(llvm_type) } {
            llvm_sys::LLVMTypeKind::LLVMIntegerTypeKind => Type::I64(I64Type::new(llvm_type)),
            llvm_sys::LLVMTypeKind::LLVMFloatTypeKind => Type::F64(F64Type::new(llvm_type)),
            llvm_sys::LLVMTypeKind::LLVMFunctionTypeKind => Type::Fn(FnType::new(llvm_type)),
            _ => panic!("Unknown type"),
        }
    }

    pub fn as_llvm_type_ref(&self) -> LLVMTypeRef {
        match self {
            Type::I64(t) => t.ty.llvm_type,
            Type::F64(t) => t.ty.llvm_type,
            Type::Fn(t) => t.ty.llvm_type,
        }
    }
}
