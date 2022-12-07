use crate::execution_engine::ExecutionEngine;
use crate::types::fn_type::FnType;
use crate::types::Type;
use crate::types::TypeIntrinsics;
use crate::utils::to_c_str;
use crate::values::fn_value::FnValue;
use crate::values::Value;
use crate::values::ValueIntrinsics;
use llvm_sys::core::LLVMAddFunction;
use llvm_sys::core::LLVMAddGlobal;
use llvm_sys::core::LLVMAddGlobalInAddressSpace;
use llvm_sys::core::LLVMDumpModule;
use llvm_sys::core::LLVMGetNamedFunction;
use llvm_sys::core::LLVMGetNamedGlobal;
use llvm_sys::core::LLVMSetInitializer;
use llvm_sys::execution_engine::LLVMCreateExecutionEngineForModule;
use llvm_sys::prelude::LLVMModuleRef;
use std::marker::PhantomData;
use std::mem;

pub struct Module<'a> {
    llvm_module: LLVMModuleRef,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Module<'a> {
    pub(crate) fn new(llvm_module: LLVMModuleRef) -> Module<'a> { Module { llvm_module, _phantom: PhantomData } }
}

impl<'a> Module<'a> {
    pub fn create_mcjit_execution_engine(&self) -> Result<ExecutionEngine<'a>, &'static str> {
        unsafe {
            let mut execution_engine = mem::MaybeUninit::zeroed().assume_init();
            let mut out = mem::zeroed();
            let r = LLVMCreateExecutionEngineForModule(&mut execution_engine, self.llvm_module, &mut out);
            if r != 0 {
                return Err("Module: could not create MCJIT execution engine");
            }

            ExecutionEngine::new(execution_engine)
        }
    }

    pub fn add_function(
        &self,
        name: &str,
        ty: FnType<'_>,
        // linkage: Option<Linkage>,
    ) -> FnValue<'a> {
        let c_string = to_c_str(name);
        let fn_value =
            unsafe { FnValue::new(LLVMAddFunction(self.llvm_module, c_string.as_ptr(), ty.as_llvm_type_ref())) };

        // if let Some(linkage) = linkage {
        //     fn_value.set_linkage(linkage)
        // }

        fn_value
    }

    pub fn get_function(&self, name: &str) -> Option<FnValue<'a>> {
        let c_string = to_c_str(name);
        let val = unsafe { LLVMGetNamedFunction(self.llvm_module, c_string.as_ptr()) };
        if val.is_null() {
            None
        } else {
            Some(FnValue::new(val))
        }
    }

    pub fn add_global(&self, name: &str, ty: Type<'a>) -> Value {
        unsafe {
            let c_string = to_c_str(name);
            let global = LLVMAddGlobalInAddressSpace(self.llvm_module, ty.as_llvm_type_ref(), c_string.as_ptr(), 0);
            Value::new(global)
        }
    }

    pub fn get_global(&self, name: &str) -> Option<Value<'a>> {
        let c_string = to_c_str(name);
        let val = unsafe { LLVMGetNamedGlobal(self.llvm_module, c_string.as_ptr()) };
        if val.is_null() {
            None
        } else {
            Some(Value::new(val))
        }
    }

    pub fn dump(&self) {
        unsafe {
            LLVMDumpModule(self.llvm_module);
        }
    }
}
