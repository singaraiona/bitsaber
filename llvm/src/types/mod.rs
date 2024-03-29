use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::LLVMTypeKind;
use std::marker::PhantomData;

pub mod f64_type;
pub mod fn_type;
pub mod i1_type;
pub mod i64_type;
pub mod ptr_type;
pub mod struct_type;
pub mod vec_type;
pub mod void_type;

pub mod prelude {
    pub use super::f64_type::F64Type;
    pub use super::fn_type::FnType;
    pub use super::i1_type::I1Type;
    pub use super::i64_type::I64Type;
    pub use super::ptr_type::PtrType;
    pub use super::struct_type::StructType;
    pub use super::vec_type::VecType;
    pub use super::void_type::VoidType;
}

use prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct TypeRef<'a> {
    llvm_type: LLVMTypeRef,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> TypeRef<'a> {
    pub(crate) fn new(llvm_type: LLVMTypeRef) -> TypeRef<'a> {
        debug_assert!(!llvm_type.is_null());

        TypeRef { llvm_type, _phantom: PhantomData }
    }
}

impl Into<LLVMTypeRef> for TypeRef<'_> {
    fn into(self) -> LLVMTypeRef { self.llvm_type }
}

#[derive(Clone, Debug)]
pub enum Type<'a> {
    Null(VoidType<'a>),
    Bool(I1Type<'a>),
    Int64(I64Type<'a>),
    Float64(F64Type<'a>),
    Ptr(PtrType<'a>),
    Fn(FnType<'a>),
    Struct(StructType<'a>),
    Vec(VecType<'a>),
}

impl<'a> From<I64Type<'a>> for Type<'a> {
    fn from(ty: I64Type<'a>) -> Self { Self::Int64(ty) }
}

impl<'a> From<I1Type<'a>> for Type<'a> {
    fn from(ty: I1Type<'a>) -> Self { Self::Bool(ty) }
}

impl<'a> From<F64Type<'a>> for Type<'a> {
    fn from(ty: F64Type<'a>) -> Self { Self::Float64(ty) }
}

impl<'a> From<FnType<'a>> for Type<'a> {
    fn from(ty: FnType<'a>) -> Self { Self::Fn(ty) }
}

impl<'a> From<StructType<'a>> for Type<'a> {
    fn from(ty: StructType<'a>) -> Self { Self::Struct(ty) }
}

impl<'a> From<PtrType<'a>> for Type<'a> {
    fn from(ty: PtrType<'a>) -> Self { Self::Ptr(ty) }
}

impl<'a> From<VoidType<'a>> for Type<'a> {
    fn from(ty: VoidType<'a>) -> Self { Self::Null(ty) }
}

impl<'a> From<VecType<'a>> for Type<'a> {
    fn from(ty: VecType<'a>) -> Self { Self::Vec(ty) }
}

impl<'a> Into<PtrType<'a>> for Type<'a> {
    fn into(self) -> PtrType<'a> {
        match self {
            Type::Ptr(t) => t,
            _ => panic!("Cannot convert {:?} to PtrType", self),
        }
    }
}

impl<'a> Type<'a> {
    pub fn new(llvm_type: LLVMTypeRef) -> Type<'a> {
        match unsafe { llvm_sys::core::LLVMGetTypeKind(llvm_type) } {
            llvm_sys::LLVMTypeKind::LLVMVoidTypeKind => Type::Null(VoidType::new(llvm_type)),
            llvm_sys::LLVMTypeKind::LLVMIntegerTypeKind => unsafe {
                let int_width = llvm_sys::core::LLVMGetIntTypeWidth(llvm_type);
                match int_width {
                    1 => Type::Bool(I1Type::new(llvm_type)),
                    64 => Type::Int64(I64Type::new(llvm_type)),
                    _ => panic!("Unknown integer width: {}", int_width),
                }
            },
            llvm_sys::LLVMTypeKind::LLVMFloatTypeKind | llvm_sys::LLVMTypeKind::LLVMDoubleTypeKind => {
                Type::Float64(F64Type::new(llvm_type))
            }
            llvm_sys::LLVMTypeKind::LLVMFunctionTypeKind => Type::Fn(FnType::new(llvm_type)),
            llvm_sys::LLVMTypeKind::LLVMStructTypeKind => Type::Struct(StructType::new(llvm_type)),
            llvm_sys::LLVMTypeKind::LLVMPointerTypeKind => Type::Ptr(PtrType::new(llvm_type)),
            llvm_sys::LLVMTypeKind::LLVMVectorTypeKind => Type::Vec(VecType::new(llvm_type)),
            kind => panic!("Unknown type kind: {:?}", kind),
        }
    }
}

pub trait TypeIntrinsics {
    fn as_llvm_type_ref(&self) -> LLVMTypeRef;
    fn get_llvm_type_kind(&self) -> LLVMTypeKind { unsafe { llvm_sys::core::LLVMGetTypeKind(self.as_llvm_type_ref()) } }
}

impl<'a> TypeIntrinsics for TypeRef<'a> {
    fn as_llvm_type_ref(&self) -> LLVMTypeRef { self.llvm_type }
}

impl<'a> TypeIntrinsics for Type<'a> {
    fn as_llvm_type_ref(&self) -> LLVMTypeRef {
        match self {
            Type::Null(t) => t.as_llvm_type_ref(),
            Type::Bool(t) => t.as_llvm_type_ref(),
            Type::Int64(t) => t.as_llvm_type_ref(),
            Type::Float64(t) => t.as_llvm_type_ref(),
            Type::Fn(t) => t.as_llvm_type_ref(),
            Type::Struct(t) => t.as_llvm_type_ref(),
            Type::Ptr(t) => t.as_llvm_type_ref(),
            Type::Vec(t) => t.as_llvm_type_ref(),
        }
    }
}
