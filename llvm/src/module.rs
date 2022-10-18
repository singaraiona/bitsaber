use crate::execution_engine::ExecutionEngine;
use crate::types::fn_type::FnType;
use crate::types::Type;
use crate::types::TypeIntrinsics;
use crate::utils::to_c_str;
use crate::values::fn_value::FnValue;
use crate::values::*;
use llvm_sys::core::LLVMAddFunction;
use llvm_sys::execution_engine::LLVMCreateExecutionEngineForModule;
use llvm_sys::prelude::LLVMModuleRef;
use std::marker::PhantomData;
use std::mem;

pub struct Module<'a> {
    llvm_module: LLVMModuleRef,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Module<'a> {
    pub(crate) fn new(llvm_module: LLVMModuleRef) -> Module<'a> {
        Module {
            llvm_module,
            _phantom: PhantomData,
        }
    }
}

impl<'a> Module<'a> {
    pub fn create_mcjit_execution_engine(&self) -> Result<ExecutionEngine<'a>, &'static str> {
        unsafe {
            let mut execution_engine = mem::MaybeUninit::zeroed().assume_init();
            let mut out = mem::zeroed();
            let r = LLVMCreateExecutionEngineForModule(
                &mut execution_engine,
                self.llvm_module,
                &mut out,
            );
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
        let fn_value = unsafe {
            FnValue::new(LLVMAddFunction(
                self.llvm_module,
                c_string.as_ptr(),
                ty.as_llvm_type_ref(),
            ))
        };

        // if let Some(linkage) = linkage {
        //     fn_value.set_linkage(linkage)
        // }

        fn_value
    }
}
