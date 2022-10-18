use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

pub mod f64_value;
pub mod fn_value;
pub mod i64_value;
pub mod instruction_value;
pub mod ptr_value;
pub mod struct_value;

use f64_value::F64Value;
use fn_value::FnValue;
use i64_value::I64Value;
use instruction_value::InstructionValue;
use libc::c_char;
use llvm_sys::core::LLVMGetTypeKind;
use llvm_sys::core::LLVMTypeOf;
use llvm_sys::core::{LLVMGetValueName2, LLVMSetValueName2};
use llvm_sys::LLVMTypeKind;
use ptr_value::PtrValue;
use std::ffi::CStr;
use std::fmt;
use struct_value::StructValue;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ValueRef<'a> {
    llvm_value: LLVMValueRef,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> ValueRef<'a> {
    pub(crate) fn new(llvm_value: LLVMValueRef) -> Self {
        debug_assert!(!llvm_value.is_null());

        Self {
            llvm_value,
            _phantom: PhantomData,
        }
    }
}

impl Into<LLVMValueRef> for ValueRef<'_> {
    fn into(self) -> LLVMValueRef {
        self.llvm_value
    }
}

pub enum Value<'a> {
    I64(I64Value<'a>),
    F64(F64Value<'a>),
    Fn(FnValue<'a>),
    // VecI64(Vec<I64Value<'a>>),
    // VecF64(Vec<F64Value<'a>>),
    // List(Vec<Value<'a>>),
    Struct(StructValue<'a>),
    Instruction(InstructionValue<'a>),
    Ptr(PtrValue<'a>),
}

impl<'a> From<I64Value<'a>> for Value<'a> {
    fn from(val: I64Value<'a>) -> Self {
        Self::I64(val)
    }
}

impl<'a> From<F64Value<'a>> for Value<'a> {
    fn from(val: F64Value<'a>) -> Self {
        Self::F64(val)
    }
}

impl<'a> From<FnValue<'a>> for Value<'a> {
    fn from(val: FnValue<'a>) -> Self {
        Self::Fn(val)
    }
}

impl<'a> From<StructValue<'a>> for Value<'a> {
    fn from(val: StructValue<'a>) -> Self {
        Self::Struct(val)
    }
}

impl<'a> From<InstructionValue<'a>> for Value<'a> {
    fn from(val: InstructionValue<'a>) -> Self {
        Self::Instruction(val)
    }
}

impl<'a> From<PtrValue<'a>> for Value<'a> {
    fn from(val: PtrValue<'a>) -> Self {
        Self::Ptr(val)
    }
}

impl<'a> Into<I64Value<'a>> for Value<'a> {
    fn into(self) -> I64Value<'a> {
        match self {
            Self::I64(val) => val,
            _ => panic!("Expected I64Value"),
        }
    }
}

impl<'a> Into<F64Value<'a>> for Value<'a> {
    fn into(self) -> F64Value<'a> {
        match self {
            Self::F64(val) => val,
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

impl<'a> Value<'a> {
    pub(crate) fn new(llvm_value: LLVMValueRef) -> Self {
        unsafe {
            match LLVMGetTypeKind(LLVMTypeOf(llvm_value)) {
                LLVMTypeKind::LLVMFloatTypeKind
                | LLVMTypeKind::LLVMFP128TypeKind
                | LLVMTypeKind::LLVMDoubleTypeKind
                | LLVMTypeKind::LLVMHalfTypeKind
                | LLVMTypeKind::LLVMX86_FP80TypeKind
                | LLVMTypeKind::LLVMPPC_FP128TypeKind => Value::F64(F64Value::new(llvm_value)),
                LLVMTypeKind::LLVMIntegerTypeKind => Value::I64(I64Value::new(llvm_value)),
                LLVMTypeKind::LLVMStructTypeKind => Value::Struct(StructValue::new(llvm_value)),
                LLVMTypeKind::LLVMFunctionTypeKind => Value::Fn(FnValue::new(llvm_value)),
                LLVMTypeKind::LLVMPointerTypeKind => Value::Ptr(PtrValue::new(llvm_value)),
                kind => panic!("Unknown value: {:?}", kind),
            }
        }
    }
}

pub trait ValueIntrinsics {
    fn as_llvm_value_ref(&self) -> LLVMValueRef;
    fn set_name(self, name: &str);
    fn get_name(&self) -> &CStr;
}

impl ValueIntrinsics for ValueRef<'_> {
    fn as_llvm_value_ref(&self) -> LLVMValueRef {
        self.llvm_value
    }
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
}

impl ValueIntrinsics for Value<'_> {
    fn as_llvm_value_ref(&self) -> LLVMValueRef {
        match self {
            Value::I64(v) => v.as_llvm_value_ref(),
            Value::F64(v) => v.as_llvm_value_ref(),
            Value::Fn(v) => v.as_llvm_value_ref(),
            Value::Instruction(v) => v.as_llvm_value_ref(),
            Value::Struct(v) => v.as_llvm_value_ref(),
            Value::Ptr(v) => v.as_llvm_value_ref(),
        }
    }
    fn set_name(self, name: &str) {
        match self {
            Value::I64(v) => v.set_name(name),
            Value::F64(v) => v.set_name(name),
            Value::Fn(v) => v.set_name(name),
            Value::Instruction(v) => v.set_name(name),
            Value::Struct(v) => v.set_name(name),
            Value::Ptr(v) => v.set_name(name),
        }
    }

    fn get_name(&self) -> &CStr {
        match self {
            Value::I64(v) => v.get_name(),
            Value::F64(v) => v.get_name(),
            Value::Fn(v) => v.get_name(),
            Value::Instruction(v) => v.get_name(),
            Value::Struct(v) => v.get_name(),
            Value::Ptr(v) => v.get_name(),
        }
    }
}

impl fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::I64(v) => write!(f, "{:?}", v),
            Value::F64(v) => write!(f, "{:?}", v),
            Value::Fn(v) => write!(f, "{:?}", v),
            Value::Instruction(v) => write!(f, "{:?}", v),
            Value::Struct(v) => write!(f, "{:?}", v),
            Value::Ptr(v) => write!(f, "{:?}", v),
        }
    }
}
