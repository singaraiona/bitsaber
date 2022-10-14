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
}
