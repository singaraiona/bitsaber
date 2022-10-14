use llvm_sys::prelude::LLVMValueRef;
use std::marker::PhantomData;

pub mod f64_value;
pub mod fn_value;
pub mod i64_value;
pub mod instruction_value;
use f64_value::F64Value;
use fn_value::FnValue;
use i64_value::I64Value;
use instruction_value::InstructionValue;

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

    pub fn as_llvm_value_ref(&self) -> LLVMValueRef {
        self.llvm_value
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
    Instruction(InstructionValue<'a>),
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

impl<'a> From<InstructionValue<'a>> for Value<'a> {
    fn from(val: InstructionValue<'a>) -> Self {
        Self::Instruction(val)
    }
}

impl<'a> Value<'a> {
    pub(crate) fn new(llvm_value: LLVMValueRef) -> Self {
        match unsafe { llvm_sys::core::LLVMGetValueKind(llvm_value) } {
            llvm_sys::LLVMValueKind::LLVMConstantIntValueKind => {
                Self::I64(I64Value::new(llvm_value))
            }
            llvm_sys::LLVMValueKind::LLVMConstantFPValueKind => {
                Self::F64(F64Value::new(llvm_value))
            }
            llvm_sys::LLVMValueKind::LLVMFunctionValueKind => Self::Fn(FnValue::new(llvm_value)),
            llvm_sys::LLVMValueKind::LLVMInstructionValueKind => {
                Self::Instruction(InstructionValue::new(llvm_value))
            }
            _ => panic!("Unknown value"),
        }
    }

    pub fn as_llvm_value_ref(self) -> LLVMValueRef {
        match self {
            Value::I64(v) => v.val.as_llvm_value_ref(),
            Value::F64(v) => v.val.as_llvm_value_ref(),
            Value::Fn(v) => v.val.as_llvm_value_ref(),
            Value::Instruction(v) => v.val.as_llvm_value_ref(),
        }
    }
}
