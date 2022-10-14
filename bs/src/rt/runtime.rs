use crate::cc::compiler::Compiler;
use crate::parse::parser::*;
use crate::result::*;
use llvm::builder::Builder;
use llvm::context::Context;
use llvm::execution_engine::ExecutionEngine;
use llvm::module::Module;
use std::mem;

#[no_mangle]
pub extern "C" fn printd(x: f64) -> f64 {
    println!("{}", x);
    x
}

pub struct Runtime<'a> {
    context: Context,
    module: Module<'a>,
    builder: Builder<'a>,
}

// Public methods
impl<'a> Runtime<'a> {
    pub fn new() -> BSResult<Self> {
        unsafe {
            let context = Context::new().map_err(|e| runtime_error(e.to_string()))?;
            let module = context
                .create_module("main")
                .map_err(|e| runtime_error(e.to_string()))?;
            let builder = context
                .create_builder()
                .map_err(|e| runtime_error(e.to_string()))?;

            ok(Self {
                context,
                module,
                builder,
            })
        }
    }

    pub fn parse_eval(&mut self, input: String) -> BSResult<i64> {
        unsafe {
            let mut module = self
                .context
                .create_module("repl")
                .map_err(|e| runtime_error(e.to_string()))?;

            let execution_engine = module
                .create_mcjit_execution_engine()
                .map_err(|e| runtime_error(e.to_string()))?;

            let parsed_fn = Parser::new(input.as_str()).parse()?;

            let mut compiler =
                Compiler::new(&mut self.context, &mut self.builder, &mut module, parsed_fn);
            let compiled_fn = compiler.compile().unwrap();

            // // let mut len = 0;
            // // let ptr = LLVMGetValueName2(compiled_fn, &mut len);
            // // let compiled_name = std::ffi::CStr::from_ptr(ptr);

            // // let addr = LLVMGetFunctionAddress(self.execution_engine, compiled_name.as_ptr());

            // // let f: extern "C" fn(u64) -> u64 = mem::transmute(addr);
            // // let res = f(2);

            // // LLVMFreeMachineCodeForFunction(self.execution_engine, compiled_fn);
            // // LLVMDeleteFunction(compiled_fn);

            // // BSResult::Ok((res as i64).into())

            // println!("{:?}", parsed_fn);
            BSResult::Ok((0_i64).into())
        }
    }
}
