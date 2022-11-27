use crate::base::Type as BSType;
use crate::base::Value as BSValue;
use crate::cc::compiler::Compiler;
use crate::parse::ast::Expr;
use crate::parse::ast::ExprBody;
use crate::parse::ast::Function;
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
    previous_functions: Vec<Function>,
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
            previous_functions: Vec::new(),
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
            let mut module = self
                .context
                .create_module("repl")
                .map_err(|e| BSError::RuntimeError(e.to_string()))?;

            // recompile every previously parsed function into the new module
            for f in &self.previous_functions {
                Compiler::new(&mut self.context, &mut self.builder, &mut module, f.clone())
                    .compile()?;
            }

            let parsed_fns = Parser::new(input).parse()?;

            let execution_engine = module
                .create_mcjit_execution_engine()
                .map_err(|e| BSError::RuntimeError(e.to_string()))?;

            let mut top_level_fn = None;

            for f in parsed_fns {
                let is_top_level = f.topl;
                let (compiled_fn, ret_ty) =
                    Compiler::new(&mut self.context, &mut self.builder, &mut module, f.clone())
                        .compile()?;

                if is_top_level {
                    top_level_fn = Some((compiled_fn, ret_ty));
                } else {
                    self.previous_functions.push(f);
                }
            }

            match top_level_fn {
                Some((_, ty)) => {
                    let addr = execution_engine
                        .get_function_address("top-level")
                        .map_err(|e| BSError::RuntimeError(e.to_string()))?;

                    match ty {
                        BSType::Null => {
                            let f: extern "C" fn() -> i64 = mem::transmute(addr);
                            let _ = f();
                            ok(BSValue::Null)
                        }
                        BSType::Bool => {
                            let f: extern "C" fn() -> bool = mem::transmute(addr);
                            let result = f();
                            ok(BSValue::Bool(result))
                        }
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

                None => ok(BSValue::Null),
            }
        }
    }
}
