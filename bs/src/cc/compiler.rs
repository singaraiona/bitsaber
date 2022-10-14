use crate::parse::ast::{Expr, Function, Prototype};
use llvm::builder::Builder;
use llvm::context::Context;
use llvm::module::Module;
use rand::prelude::*;
use std::borrow::Borrow;
use std::collections::HashMap;

pub struct Compiler<'a, 'b> {
    context: &'a mut Context,
    builder: &'a mut Builder<'b>,
    module: &'a mut Module<'b>,
    function: Function,
}

impl<'a, 'b> Compiler<'a, 'b> {
    pub(crate) fn new(
        context: &'a mut Context,
        builder: &'a mut Builder<'b>,
        module: &'a mut Module<'b>,
        function: Function,
    ) -> Self {
        Compiler {
            context,
            builder,
            module,
            function,
        }
    }

    fn compile_fn(&mut self) -> Result<i64, &'static str> {
        unsafe {
            let mut arg_types = [self.context.i64_type().into()];
            let fn_type = self.context.fn_i64_type(&mut arg_types, false);

            // add it to our module
            let function = self
                .module
                .add_function(&self.function.prototype.name, fn_type);
            // let function = LLVMAddFunction(self.module, name.as_ptr() as *const _, function_type);

            // // Create a basic block in the function and set our builder to generate
            // // code in it.
            // let bb = LLVMAppendBasicBlockInContext(
            //     self.context,
            //     function,
            //     b"entry\0".as_ptr() as *const _,
            // );

            // LLVMPositionBuilderAtEnd(self.builder, bb);

            let mut rng = rand::thread_rng();
            let rnd: i64 = rng.gen_range(0..8);

            // // get the function's arguments

            // let x = LLVMGetParam(function, 0);
            let y = self.context.i64_type().const_value(rnd);
            // let sum = LLVMBuildAdd(self.builder, x, y, b"tmpsum\0".as_ptr() as *const _);

            // // Emit a `ret void` into the function
            // LLVMBuildRet(self.builder, sum);

            // Ok(function)

            todo!()
        }
    }

    pub fn compile(&mut self) -> Result<i64, &'static str> {
        self.compile_fn()
    }
}
