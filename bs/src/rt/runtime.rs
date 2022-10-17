use crate::base::Value as BsValue;
use crate::cc::compiler::Compiler;
use crate::parse::parser::*;
use crate::result::*;
use llvm::builder::Builder;
use llvm::context::Context;
use llvm::execution_engine::ExecutionEngine;
use llvm::module::Module;
use llvm::values::Value;
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

    pub fn parse_eval(&mut self, input: String) -> BSResult<BsValue> {
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

            let addr = execution_engine
                .get_function_address("anonymous")
                .map_err(|e| runtime_error(e.to_string()))?;

            let f: extern "C" fn(u64, u64) -> BsValue = mem::transmute(addr);
            let res = f(12, 55);

            BSResult::Ok(res)
        }
    }
}
