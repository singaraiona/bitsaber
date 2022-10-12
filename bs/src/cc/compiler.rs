use crate::parse::parser::{Expr, Function, Prototype};
use llvm::core::*;
use llvm::execution_engine::*;
use llvm::prelude::LLVMValueRef;
use llvm::target::*;
use llvm::*;
use rand::prelude::*;
use std::borrow::Borrow;
use std::collections::HashMap;
/// Defines the `Expr` compiler.
pub struct Compiler {
    context: *mut LLVMContext,
    builder: *mut LLVMBuilder,
    module: *mut LLVMModule,
    function: Function,
}

impl Compiler {
    fn compile_fn(&mut self, name: &str) -> Result<LLVMValueRef, &'static str> {
        unsafe {
            // get a type for sum function

            let i64t = LLVMInt64TypeInContext(self.context);
            let mut argts = [i64t, i64t, i64t];
            let function_type = LLVMFunctionType(i64t, argts.as_mut_ptr(), argts.len() as u32, 0);

            // add it to our module
            let function =
                LLVMAddFunction(self.module, b"sum\0".as_ptr() as *const _, function_type);

            // Create a basic block in the function and set our builder to generate
            // code in it.
            let bb = LLVMAppendBasicBlockInContext(
                self.context,
                function,
                b"entry\0".as_ptr() as *const _,
            );

            LLVMPositionBuilderAtEnd(self.builder, bb);

            // get the function's arguments
            let x = LLVMGetParam(function, 0);
            let y = LLVMGetParam(function, 1);
            let z = LLVMGetParam(function, 2);

            let mut rng = rand::thread_rng();
            let rnd: u8 = rng.gen();

            let sum = LLVMBuildAdd(self.builder, x, y, name.as_ptr() as *const _);

            let sum = if rnd > 100 {
                LLVMBuildAdd(self.builder, sum, z, b"sum.2\0".as_ptr() as *const _)
            } else {
                println!("build mul");
                LLVMBuildMul(self.builder, sum, z, b"sum.2\0".as_ptr() as *const _)
            };

            // Emit a `ret void` into the function
            LLVMBuildRet(self.builder, sum);

            Ok(function)
        }
    }

    pub fn compile(
        context: *mut LLVMContext,
        builder: *mut LLVMBuilder,
        module: *mut LLVMModule,
        function: Function,
        name: &str,
    ) -> Result<LLVMValueRef, &'static str> {
        let mut compiler = Compiler {
            context,
            builder,
            module,
            function,
        };

        compiler.compile_fn(name)
    }
}
