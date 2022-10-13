use crate::cc::compiler::*;
use crate::parse::parser::*;
use crate::result::*;
use crate::value::*;
use llvm::core::*;
use llvm::execution_engine::*;
use llvm::target::*;
use llvm::*;
use std::collections::HashMap;
use std::mem;

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

// Public methods
impl Runtime {
    fn create_execution_engine(&mut self) -> BSResult<()> {
        unsafe {
            let mut out = mem::zeroed();
            LLVMCreateExecutionEngineForModule(&mut self.execution_engine, self.module, &mut out);
            BSResult::Ok(())
        }
    }

    pub fn new() -> Self {
        unsafe {
            let context = LLVMContextCreate();
            let module = LLVMModuleCreateWithNameInContext(b"main\0".as_ptr() as *const _, context);
            let builder = LLVMCreateBuilderInContext(context);

            LLVMLinkInMCJIT();
            LLVM_InitializeNativeTarget();
            LLVM_InitializeNativeAsmPrinter();

            Self {
                context,
                module,
                builder,
                execution_engine: mem::MaybeUninit::zeroed().assume_init(),
            }
        }
    }

    pub fn parse_eval(&mut self, input: String) -> BSResult<Value> {
        unsafe {
            self.create_execution_engine();

            let parsed_fn = Parser::new(input.as_str()).parse()?;
            // let compiled_fn =
            //     Compiler::compile(self.context, self.builder, self.module, parsed_fn, name)
            //         .unwrap();

            // let mut len = 0;
            // let ptr = LLVMGetValueName2(compiled_fn, &mut len);
            // let compiled_name = std::ffi::CStr::from_ptr(ptr);

            // let addr = LLVMGetFunctionAddress(self.execution_engine, compiled_name.as_ptr());

            // let f: extern "C" fn(u64) -> u64 = mem::transmute(addr);
            // let res = f(2);

            // LLVMFreeMachineCodeForFunction(self.execution_engine, compiled_fn);
            // LLVMDeleteFunction(compiled_fn);

            // BSResult::Ok((res as i64).into())

            println!("{:?}", parsed_fn);
            BSResult::Ok((0_i64).into())
        }
    }
}
