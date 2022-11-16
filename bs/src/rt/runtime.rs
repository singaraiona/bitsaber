use crate::base::Type as BSType;
use crate::base::Value as BSValue;
use crate::cc::compiler::Compiler;
use crate::parse::parser::*;
use crate::result::*;
use llvm::builder::Builder;
use llvm::context::Context;
use llvm::execution_engine::ExecutionEngine;
use llvm::module::Module;
use llvm::types::Type;
use llvm::values::Value;
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

    pub fn parse_eval(&mut self, input: &str) -> BSResult<BSValue> {
        unsafe {
            let parsed_fn = Parser::new(input).parse()?;

            let mut module = self
                .context
                .create_module("repl")
                .map_err(|e| BSError::RuntimeError(e.to_string()))?;

            let execution_engine = module
                .create_mcjit_execution_engine()
                .map_err(|e| BSError::RuntimeError(e.to_string()))?;

            let mut compiler =
                Compiler::new(&mut self.context, &mut self.builder, &mut module, parsed_fn);

            let (compiled_fn, ret_ty) = compiler.compile()?;

            let addr = execution_engine
                .get_function_address("anonymous")
                .map_err(|e| BSError::RuntimeError(e.to_string()))?;

            match ret_ty {
                BSType::Int64 => {
                    let f: extern "C" fn() -> i64 = mem::transmute(addr);
                    ok(BSValue::Int64(f().into()))
                }
                BSType::Float64 => {
                    let f: extern "C" fn() -> f64 = mem::transmute(addr);
                    ok(BSValue::Float64(f().into()))
                }
                _ => {
                    let f: extern "C" fn() -> BSValue = mem::transmute(addr);
                    ok(f())
                }
            }
        }
    }
}
