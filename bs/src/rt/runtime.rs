use crate::cc::compiler::*;
use crate::parse::parser::*;
use llvm::core::*;
use llvm::execution_engine::*;
use llvm::target::*;
use llvm::*;
use std::collections::HashMap;
use std::{fmt, mem};

#[no_mangle]
pub extern "C" fn printd(x: f64) -> f64 {
    println!("{}", x);
    x
}

pub struct Runtime {
    context: *mut LLVMContext,
    module: *mut LLVMModule,
    builder: *mut LLVMBuilder,
    execution_engine: *mut LLVMOpaqueExecutionEngine,
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.builder);
            LLVMDisposeExecutionEngine(self.execution_engine);
            LLVMContextDispose(self.context);
        }
    }
}

impl Runtime {
    pub fn new() -> Self {
        unsafe {
            let context = LLVMContextCreate();
            let module = LLVMModuleCreateWithNameInContext(b"main\0".as_ptr() as *const _, context);
            let builder = LLVMCreateBuilderInContext(context);

            LLVMDumpModule(module);

            let mut execution_engine = mem::MaybeUninit::zeroed().assume_init();
            let mut out = mem::zeroed();

            LLVMLinkInMCJIT();
            LLVM_InitializeNativeTarget();
            LLVM_InitializeNativeAsmPrinter();
            LLVMCreateExecutionEngineForModule(&mut execution_engine, module, &mut out);

            Self {
                context,
                module,
                builder,
                execution_engine,
            }
        }
    }

    pub fn parse_eval(&mut self, input: String) -> Result<Box<dyn fmt::Display>, String> {
        unsafe {
            let name = "sum\0";

            // Build precedence map
            let mut prec = HashMap::with_capacity(6);

            prec.insert('=', 2);
            prec.insert('<', 10);
            prec.insert('+', 20);
            prec.insert('-', 20);
            prec.insert('*', 40);
            prec.insert('/', 40);

            let parsed_fn = Parser::new(input, &mut prec).parse().unwrap();
            let compiled_fn =
                Compiler::compile(self.context, self.builder, self.module, parsed_fn, name)
                    .unwrap();

            let addr = LLVMGetFunctionAddress(self.execution_engine, name.as_ptr() as *const _);
            // LLVMRecompileAndRelinkFunction(self.execution_engine, compiled_fn);
            // let addr = LLVMGetFunctionAddress(self.execution_engine, name.as_ptr() as *const _);

            let mut len = 0;
            let ptr = LLVMGetValueName2(compiled_fn, &mut len);
            let compiled_name = std::ffi::CStr::from_ptr(ptr);

            let f: extern "C" fn(u64) -> u64 = mem::transmute(addr);
            let res = f(2);

            println!("COMPILED: {:?}", compiled_fn);

            LLVMFreeMachineCodeForFunction(self.execution_engine, compiled_fn);
            LLVMDeleteFunction(compiled_fn);
            // LLVMInstructionEraseFromParent(compiled_fn);

            Ok(Box::new(format!(
                "addr: {} name: {:?} res: {}",
                addr, compiled_name, res
            )))
        }
    }
}
