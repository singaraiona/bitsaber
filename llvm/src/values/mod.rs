use libc::c_char;
use llvm_sys::core::LLVMGetIntTypeWidth;
use llvm_sys::core::LLVMGetTypeKind;
use llvm_sys::core::LLVMTypeOf;
use llvm_sys::core::{LLVMGetValueName2, LLVMSetValueName2};
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMTypeKind;
use prelude::*;
use std::ffi::CStr;
use std::fmt;
use std::marker::PhantomData;

pub mod f64_value;
pub mod fn_value;
pub mod i1_value;
pub mod i64_value;
pub mod instruction_value;
pub mod phi_value;
pub mod ptr_value;
pub mod struct_value;

pub mod prelude {
    pub use super::f64_value::F64Value;
    pub use super::fn_value::FnValue;
    pub use super::i1_value::I1Value;
    pub use super::i64_value::I64Value;
    pub use super::instruction_value::InstructionValue;
    pub use super::phi_value::PhiValue;
    pub use super::ptr_value::PtrValue;
    pub use super::struct_value::StructValue;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ValueRef<'a> {
    llvm_value: LLVMValueRef,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> ValueRef<'a> {
    pub(crate) fn new(llvm_value: LLVMValueRef) -> Self {
        debug_assert!(!llvm_value.is_null());

        Self { llvm_value, _phantom: PhantomData }
    }
}

impl Into<LLVMValueRef> for ValueRef<'_> {
    fn into(self) -> LLVMValueRef { self.llvm_value }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Value<'a> {
    Null(I64Value<'a>),
    Bool(I1Value<'a>),
    Int64(I64Value<'a>),
    Float64(F64Value<'a>),
    Fn(FnValue<'a>),
    Struct(StructValue<'a>),
    Instruction(InstructionValue<'a>),
    Ptr(PtrValue<'a>),
    Phi(PhiValue<'a>),
}

impl<'a> From<I1Value<'a>> for Value<'a> {
    fn from(val: I1Value<'a>) -> Self { Value::Bool(val) }
}

impl<'a> From<I64Value<'a>> for Value<'a> {
    fn from(val: I64Value<'a>) -> Self { Self::Int64(val) }
}

impl<'a> From<F64Value<'a>> for Value<'a> {
    fn from(val: F64Value<'a>) -> Self { Self::Float64(val) }
}

impl<'a> From<FnValue<'a>> for Value<'a> {
    fn from(val: FnValue<'a>) -> Self { Self::Fn(val) }
}

impl<'a> From<StructValue<'a>> for Value<'a> {
    fn from(val: StructValue<'a>) -> Self { Self::Struct(val) }
}

impl<'a> From<InstructionValue<'a>> for Value<'a> {
    fn from(val: InstructionValue<'a>) -> Self { Self::Instruction(val) }
}

impl<'a> From<PtrValue<'a>> for Value<'a> {
    fn from(val: PtrValue<'a>) -> Self { Self::Ptr(val) }
}

impl<'a> From<PhiValue<'a>> for Value<'a> {
    fn from(val: PhiValue<'a>) -> Self { Self::Phi(val) }
}

impl<'a> Into<I1Value<'a>> for Value<'a> {
    fn into(self) -> I1Value<'a> {
        match self {
            Value::Bool(val) => val,
            _ => panic!("Expected I8Value"),
        }
    }
}

impl<'a> Into<I64Value<'a>> for Value<'a> {
    fn into(self) -> I64Value<'a> {
        match self {
            Self::Int64(val) => val,
            _ => panic!("Expected I64Value"),
        }
    }
}

impl<'a> Into<F64Value<'a>> for Value<'a> {
    fn into(self) -> F64Value<'a> {
        match self {
            Self::Float64(val) => val,
            _ => panic!("Expected F64Value"),
        }
    }
}

impl<'a> Into<FnValue<'a>> for Value<'a> {
    fn into(self) -> FnValue<'a> {
        match self {
            Self::Fn(val) => val,
            _ => panic!("Expected FnValue"),
        }
    }
}

impl<'a> Into<StructValue<'a>> for Value<'a> {
    fn into(self) -> StructValue<'a> {
        match self {
            Self::Struct(val) => val,
            _ => panic!("Expected StructValue"),
        }
    }
}

impl<'a> Into<InstructionValue<'a>> for Value<'a> {
    fn into(self) -> InstructionValue<'a> {
        match self {
            Self::Instruction(val) => val,
            _ => panic!("Expected InstructionValue"),
        }
    }
}

impl<'a> Into<PtrValue<'a>> for Value<'a> {
    fn into(self) -> PtrValue<'a> {
        match self {
            Self::Ptr(val) => val,
            _ => panic!("Expected PtrValue"),
        }
    }
}

impl<'a> Into<PhiValue<'a>> for Value<'a> {
    fn into(self) -> PhiValue<'a> {
        match self {
            Self::Phi(val) => val,
            _ => panic!("Expected PhiValue"),
        }
    }
}

impl<'a> Value<'a> {
    pub(crate) fn new(llvm_value: LLVMValueRef) -> Self {
        unsafe {
            let llvm_type = LLVMTypeOf(llvm_value);
            let kind = LLVMGetTypeKind(llvm_type);
            // println!("KIND: {:?}", kind);
            match kind {
                LLVMTypeKind::LLVMFloatTypeKind | LLVMTypeKind::LLVMDoubleTypeKind | LLVMTypeKind::LLVMHalfTypeKind => {
                    Value::Float64(F64Value::new(llvm_value))
                }
                LLVMTypeKind::LLVMIntegerTypeKind => {
                    let width = LLVMGetIntTypeWidth(LLVMTypeOf(llvm_value));

                    match width {
                        1 => Value::Bool(I1Value::new(llvm_value)),
                        _ => Value::Int64(I64Value::new(llvm_value)),
                    }
                }
                LLVMTypeKind::LLVMStructTypeKind => Value::Struct(StructValue::new(llvm_value)),
                LLVMTypeKind::LLVMFunctionTypeKind => Value::Fn(FnValue::new(llvm_value)),
                LLVMTypeKind::LLVMPointerTypeKind => Value::Ptr(PtrValue::new(llvm_value)),
                kind => panic!("Unknown value type: {:?}", kind),
            }
        }
    }
}

pub trait ValueIntrinsics {
    fn as_llvm_value_ref(&self) -> LLVMValueRef;
    fn set_name(self, name: &str);
    fn get_name(&self) -> &CStr;
    fn get_llvm_type_ref(&self) -> LLVMTypeRef;
}

impl ValueIntrinsics for ValueRef<'_> {
    fn as_llvm_value_ref(&self) -> LLVMValueRef { self.llvm_value }
    fn set_name(self, name: &str) {
        unsafe { LLVMSetValueName2(self.llvm_value, name.as_ptr() as *const c_char, name.len()) }
    }

    fn get_name(&self) -> &CStr {
        let ptr = unsafe {
            let mut len = 0;
            LLVMGetValueName2(self.llvm_value, &mut len)
        };
        unsafe { CStr::from_ptr(ptr) }
    }

    fn get_llvm_type_ref(&self) -> LLVMTypeRef { unsafe { LLVMTypeOf(self.llvm_value) } }
}

impl ValueIntrinsics for Value<'_> {
    fn as_llvm_value_ref(&self) -> LLVMValueRef {
        match self {
            Value::Null(v) => v.as_llvm_value_ref(),
            Value::Bool(val) => val.as_llvm_value_ref(),
            Value::Int64(v) => v.as_llvm_value_ref(),
            Value::Float64(v) => v.as_llvm_value_ref(),
            Value::Fn(v) => v.as_llvm_value_ref(),
            Value::Instruction(v) => v.as_llvm_value_ref(),
            Value::Struct(v) => v.as_llvm_value_ref(),
            Value::Ptr(v) => v.as_llvm_value_ref(),
            Value::Phi(v) => v.as_llvm_value_ref(),
        }
    }
    fn set_name(self, name: &str) {
        match self {
            Value::Bool(val) => val.set_name(name),
            Value::Int64(v) => v.set_name(name),
            Value::Float64(v) => v.set_name(name),
            Value::Fn(v) => v.set_name(name),
            Value::Instruction(v) => v.set_name(name),
            Value::Struct(v) => v.set_name(name),
            Value::Ptr(v) => v.set_name(name),
            Value::Null(v) => v.set_name(name),
            Value::Phi(v) => v.set_name(name),
        }
    }

    fn get_name(&self) -> &CStr {
        match self {
            Value::Null(v) => v.get_name(),
            Value::Bool(val) => val.get_name(),
            Value::Int64(v) => v.get_name(),
            Value::Float64(v) => v.get_name(),
            Value::Fn(v) => v.get_name(),
            Value::Instruction(v) => v.get_name(),
            Value::Struct(v) => v.get_name(),
            Value::Ptr(v) => v.get_name(),
            Value::Phi(v) => v.get_name(),
        }
    }
    fn get_llvm_type_ref(&self) -> LLVMTypeRef {
        match self {
            Value::Null(v) => v.get_llvm_type_ref(),
            Value::Bool(val) => val.get_llvm_type_ref(),
            Value::Int64(v) => v.get_llvm_type_ref(),
            Value::Float64(v) => v.get_llvm_type_ref(),
            Value::Fn(v) => v.get_llvm_type_ref(),
            Value::Instruction(v) => v.get_llvm_type_ref(),
            Value::Struct(v) => v.get_llvm_type_ref(),
            Value::Ptr(v) => v.get_llvm_type_ref(),
            Value::Phi(v) => v.get_llvm_type_ref(),
        }
    }
}

impl fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null(v) => write!(f, "{:?}", v),
            Value::Bool(val) => write!(f, "{:?}", val),
            Value::Int64(v) => write!(f, "{:?}", v),
            Value::Float64(v) => write!(f, "{:?}", v),
            Value::Fn(v) => write!(f, "{:?}", v),
            Value::Instruction(v) => write!(f, "{:?}", v),
            Value::Struct(v) => write!(f, "{:?}", v),
            Value::Ptr(v) => write!(f, "{:?}", v),
            Value::Phi(v) => write!(f, "{:?}", v),
        }
    }
}
