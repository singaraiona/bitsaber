use crate::base::Value as BsValue;
use crate::parse::ast::{Expr, Function, Prototype};
use llvm::builder::Builder;
use llvm::context::Context;
use llvm::module::Module;
use llvm::values::fn_value::FnValue;
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

    fn compile_fn(&mut self) -> Result<FnValue<'a>, &'static str> {
        let mut arg_types = [
            self.context.i64_type().into(),
            self.context.i64_type().into(),
        ];
        let ret_type = self.context.i64_type();
        let fn_type = self.context.fn_type(ret_type.into(), &mut arg_types, false);
        let function = self
            .module
            .add_function(&self.function.prototype.name, fn_type);

        let entry = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(entry);

        let mut params_iter = function.get_params_iter();

        let x = params_iter.next().unwrap();
        let y = params_iter.next().unwrap();
        let sum = self.builder.build_i64_add(x.into(), y.into(), "tmpsum");

        let ret_struct = self
            .context
            .struct_type(
                &[
                    self.context.i64_type().into(),
                    self.context.i64_type().into(),
                ],
                false,
            )
            .const_value(
                &[
                    self.context.i64_type().const_value(123).into(),
                    self.context.i64_type().const_value(456).into(),
                ],
                false,
            );

        self.builder.build_return(ret_struct.into());
        Ok(function)
    }

    pub fn compile(&mut self) -> Result<FnValue<'a>, &'static str> {
        self.compile_fn()
    }
}
