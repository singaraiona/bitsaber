use crate::compile::utils::to_c_str;
use llvm_sys::execution_engine::*;
use std::marker::PhantomData;

pub struct ExecutionEngine<'a> {
    llvm_execution_engine: LLVMExecutionEngineRef,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Drop for ExecutionEngine<'a> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeExecutionEngine(self.llvm_execution_engine);
        }
    }
}

impl<'a> ExecutionEngine<'a> {
    pub(crate) fn new(
        llvm_execution_engine: LLVMExecutionEngineRef,
    ) -> Result<ExecutionEngine<'a>, &'static str> {
        Ok(ExecutionEngine {
            llvm_execution_engine,
            _phantom: PhantomData,
        })
    }

    pub fn get_function_address(&self, fn_name: &str) -> Result<usize, &'static str> {
        let c_string = to_c_str(fn_name);
        let address =
            unsafe { LLVMGetFunctionAddress(self.llvm_execution_engine, c_string.as_ptr()) };

        if address == 0 {
            return Err("ExecutionEngine: could not get function address");
        }

        Ok(address as usize)
    }
}
