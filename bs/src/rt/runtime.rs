use crate::base::Type as BSType;
use crate::base::Value as BSValue;
use crate::cc::compiler::Compiler;
use crate::parse::ast::Function;
use crate::parse::parser::*;
use crate::result::*;
use llvm::builder::Builder;
use llvm::context::Context;
use llvm::execution_engine::ExecutionEngine;
use llvm::module::Module;
use std::collections::HashMap;
use std::mem;

#[no_mangle]
pub extern "C" fn printd(x: f64) -> f64 {
    println!("{}", x);
    x
}

pub(crate) struct RuntimeModule<'a> {
    module: Module<'a>,
    engine: ExecutionEngine<'a>,
    globals: HashMap<String, (BSValue, BSType)>,
}

impl<'a> RuntimeModule<'a> {
    pub fn new(name: String, context: &Context) -> BSResult<Self> {
        let module = context
            .create_module(name.as_str())
            .map_err(|e| BSError::RuntimeError(e.to_string()))?;
        let engine = module
            .create_mcjit_execution_engine()
            .map_err(|e| BSError::RuntimeError(e.to_string()))?;

        ok(Self {
            module,
            engine,
            globals: HashMap::new(),
        })
    }

    // pub fn add_function(&mut self, name: &str, ty: BSType, func: fn() -> f64) {
    //     let context = self.module.get_context();
    //     let func_ty = ty.to_llvm(&context);
    //     let func = self.module.add_function(name, func_ty);
    //     self.engine.add_global_mapping(func, func as *mut _);
    //     self.globals.insert(name.to_string(), func);
    // }

    pub fn add_global(&mut self, name: &str, ty: BSType, value: BSValue) {
        self.globals.insert(name.to_string(), (value, ty));
    }

    // pub fn get_function(&self, name: &str) -> Option<Value<'a>> {
    //     self.globals.get(name).cloned()
    // }
}

pub struct Runtime<'a> {
    context: Context,
    modules: HashMap<String, RuntimeModule<'a>>,
    builder: Builder<'a>,
    previous_functions: Vec<Function>,
}

// Public methods
impl<'a> Runtime<'a> {
    pub fn new() -> BSResult<Self> {
        let context = Context::new().map_err(|e| BSError::RuntimeError(e.to_string()))?;
        let modules = HashMap::new();
        let builder = context
            .create_builder()
            .map_err(|e| BSError::RuntimeError(e.to_string()))?;

        ok(Self {
            context,
            modules,
            builder,
            previous_functions: Vec::new(),
        })
    }

    pub fn parse_eval(&mut self, input: &str) -> BSResult<BSValue> {
        unsafe {
            let repl_module = RuntimeModule::new("repl".into(), &self.context)?;
            self.modules.insert("repl".into(), repl_module);

            let runtime_module = &mut self.modules.get_mut("repl").unwrap();
            let module = &mut runtime_module.module;
            let engine = &mut runtime_module.engine;

            // recompile every previously parsed function into the new module
            for f in &self.previous_functions {
                Compiler::new(&mut self.context, &mut self.builder, module, f.clone()).compile()?;
            }

            let parsed_fns = Parser::new(input).parse()?;

            let mut top_level_fn = None;

            for f in parsed_fns {
                let is_top_level = f.topl;
                let (compiled_fn, ret_ty) =
                    Compiler::new(&mut self.context, &mut self.builder, module, f.clone())
                        .compile()?;

                if is_top_level {
                    top_level_fn = Some((compiled_fn, ret_ty));
                } else {
                    self.previous_functions.push(f);
                }
            }

            match top_level_fn {
                Some((_, ty)) => {
                    let addr = engine
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
