use crate::base::Value as BSValue;
use crate::compile::builder::Builder;
use crate::compile::compiler::Compiler;
use crate::compile::context::Context;
use crate::compile::execution_engine::ExecutionEngine;
use crate::compile::module::Module;
use crate::compile::types::Type;
use crate::compile::values::Value;
use crate::parse::parser::*;
use crate::result::*;
use std::collections::HashMap;
use std::mem;

#[no_mangle]
pub extern "C" fn printd(x: f64) -> f64 {
    println!("{}", x);
    x
}

struct FunctionProto<'a> {
    name: String,
    return_type: Type<'a>,
    args: Vec<Type<'a>>,
}

pub struct Runtime<'a> {
    context: Context,
    module: Module<'a>,
    builder: Builder<'a>,
    functions: HashMap<String, FunctionProto<'a>>,
}

// Public methods
impl<'a> Runtime<'a> {
    pub fn new() -> BSResult<Self> {
        let context = Context::new().map_err(|e| BSError::RuntimeError(e.to_string()))?;
        let module = context
            .create_module("main")
            .map_err(|e| BSError::RuntimeError(e.to_string()))?;
        let builder = context
            .create_builder()
            .map_err(|e| BSError::RuntimeError(e.to_string()))?;

        ok(Self {
            context,
            module,
            builder,
            functions: HashMap::new(),
        })
    }

    pub fn call_function(
        &self,
        fn_name: &str,
        ee: &ExecutionEngine,
    ) -> Result<Value<'a>, &'static str> {
        // let addr = self.get_function_address(fn_name)?;

        // Ok(FunctionValue::new(function))
        todo!()
    }

    pub fn parse_eval(&mut self, input: String) -> BSResult<BSValue> {
        unsafe {
            let mut module = self
                .context
                .create_module("repl")
                .map_err(|e| BSError::RuntimeError(e.to_string()))?;

            let execution_engine = module
                .create_mcjit_execution_engine()
                .map_err(|e| BSError::RuntimeError(e.to_string()))?;

            let parsed_fn = Parser::new(input.as_str()).parse()?;

            let mut compiler =
                Compiler::new(&mut self.context, &mut self.builder, &mut module, parsed_fn);
            let compiled_fn = compiler.compile()?;
            let ret_type = compiled_fn.get_return_type();

            let addr = execution_engine
                .get_function_address("anonymous")
                .map_err(|e| BSError::RuntimeError(e.to_string()))?;

            match ret_type {
                Type::I64(_) => {
                    let f: extern "C" fn() -> i64 = mem::transmute(addr);
                    let ret = f();
                    ok(BSValue::I64(ret))
                }

                Type::F64(_) => {
                    let f: extern "C" fn() -> f64 = mem::transmute(addr);
                    let ret = f();
                    ok(BSValue::F64(ret))
                }

                Type::Struct(_) => {
                    println!("struct");
                    todo!()
                }
                Type::Fn(_) => {
                    println!("struct");
                    todo!()
                }
                t => {
                    todo!()
                }
            }
        }
    }
}
