use crate::builtins;
use crate::cc::compiler::Compiler;
use crate::parse::ast::Function;
use crate::parse::parser::*;
use crate::result::*;
use ffi::external;
use ffi::Type as BSType;
use ffi::Value as BSValue;
use llvm::builder::Builder;
use llvm::context::Context;
use llvm::execution_engine::ExecutionEngine;
use llvm::llvm_sys::support::LLVMAddSymbol;
use llvm::module::Module;
use llvm::utils::to_c_str;
use std::collections::HashMap;
use std::mem;
use std::mem::transmute;

static mut RUNTIME: Option<*mut Runtime> = None;

pub fn set_runtime(runtime: *mut Runtime<'static>) { unsafe { RUNTIME = Some(runtime) } }

pub fn get_runtime<'a>() -> Option<&'a mut Runtime<'static>> { unsafe { RUNTIME.as_mut().map(|r| &mut **r) } }

pub struct RuntimeModule<'a> {
    pub(crate) module: Module<'a>,
    pub(crate) engine: ExecutionEngine<'a>,
    pub(crate) globals: HashMap<String, (BSType, BSValue)>,
}

impl<'a> RuntimeModule<'a> {
    pub fn new(name: String, context: &Context) -> BSResult<Self> {
        let module = context
            .create_module(name.as_str())
            .map_err(|e| BSError::RuntimeError(e.to_string()))?;
        let engine = module
            .create_mcjit_execution_engine()
            .map_err(|e| BSError::RuntimeError(e.to_string()))?;

        ok(Self { module, engine, globals: HashMap::new() })
    }

    pub fn recreate_module(&mut self, name: String, context: &Context) -> BSResult<()> {
        self.module = context
            .create_module(name.as_str())
            .map_err(|e| BSError::RuntimeError(e.to_string()))?;
        self.engine = self
            .module
            .create_mcjit_execution_engine()
            .map_err(|e| BSError::RuntimeError(e.to_string()))?;
        ok(())
    }

    pub fn add_global(&mut self, name: &str, ty: BSType, value: BSValue) {
        self.globals.insert(name.to_string(), (ty, value));
    }

    pub fn get_global(&self, name: &str) -> Option<(BSType, *const u8)> {
        match self.globals.get(name) {
            Some((ty, value)) => Some((*ty, value as *const BSValue as *const u8)),
            None => None,
        }
    }
}

pub struct Runtime<'a> {
    modules: HashMap<String, RuntimeModule<'a>>,
    builder: Builder<'a>,
    previous_functions: Vec<Function>,
    context: Context,
}

// Public methods
impl<'a> Runtime<'a> {
    pub fn new() -> BSResult<Box<Self>> {
        let context = Context::new().map_err(|e| BSError::RuntimeError(e.to_string()))?;
        let modules = HashMap::new();
        let builder = context
            .create_builder()
            .map_err(|e| BSError::RuntimeError(e.to_string()))?;

        // Initialize builtins
        builtins::init();

        // add external symbols
        external::with(|map| {
            for (name, ext) in map {
                Self::add_symbol(name, ext.addr);
            }
        });

        unsafe {
            let rt = Box::new(transmute(Self { context, modules, builder, previous_functions: Vec::new() }));
            let ptr = Box::into_raw(rt);
            set_runtime(ptr);
            ok(Box::from_raw(ptr))
        }
    }

    pub fn add_symbol(name: &str, addr: i64) {
        unsafe {
            LLVMAddSymbol(to_c_str(name).as_ptr(), addr as _);
        }
    }

    pub fn parse_eval(&mut self, input: &str) -> BSResult<BSValue> {
        unsafe {
            let repl_module = self
                .modules
                .entry("repl".into())
                .or_insert(RuntimeModule::new("repl".into(), &self.context)?);

            repl_module.recreate_module("repl".into(), &self.context)?;

            // add external symbols
            external::with(|map| {
                for (name, ext) in map {
                    let args = ext.args.iter().map(|t| ("".into(), *t)).collect::<Vec<_>>();
                    let f = Function { name: name.clone(), args: args, body: vec![], topl: false };
                    let cc = Compiler::new("repl", &mut self.context, &mut self.builder, &mut self.modules, f);

                    cc.compile_prototype(ext.ret)
                        .expect("failed to compile external function");

                    self.modules
                        .get_mut("repl".into())
                        .unwrap()
                        .add_global(name, ext.ret, BSValue::Null);
                }
            });

            // recompile every previously parsed function into the new module
            for f in &self.previous_functions {
                Compiler::new("repl", &mut self.context, &mut self.builder, &mut self.modules, f.clone()).compile()?;
            }

            let parsed_fns = Parser::new(input).parse()?;

            let mut top_level_fn = None;

            for f in parsed_fns {
                let is_top_level = f.topl;
                let (compiled_fn, ret_ty) =
                    Compiler::new("repl", &mut self.context, &mut self.builder, &mut self.modules, f.clone())
                        .compile()?;

                if is_top_level {
                    top_level_fn = Some((compiled_fn, ret_ty));
                } else {
                    self.previous_functions.push(f);
                }
            }

            let runtime_module = &mut self.modules.get_mut("repl").unwrap();
            let engine = &mut runtime_module.engine;

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

    pub fn get_module(&self, name: &str) -> Option<&RuntimeModule> { self.modules.get(name) }
}
