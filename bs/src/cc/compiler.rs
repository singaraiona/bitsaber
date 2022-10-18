use crate::base::Value as BsValue;
use crate::llvm::values::ptr_value::PtrValue;
use crate::llvm::values::ValueIntrinsics;
use crate::parse::ast::{Expr, Function, Prototype};
use crate::result::*;
use llvm::builder::Builder;
use llvm::context::Context;
use llvm::module::Module;
use llvm::types::Type;
use llvm::values::fn_value::FnValue;
use llvm::values::Value;
use std::collections::HashMap;

pub struct Compiler<'a, 'b> {
    context: &'a mut Context,
    builder: &'a mut Builder<'b>,
    module: &'a mut Module<'b>,
    function: Function,
    variables: HashMap<String, PtrValue<'b>>,
    fn_value: Option<FnValue<'b>>,
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
            variables: HashMap::new(),
            fn_value: None,
        }
    }

    fn compile_expr(&self, expr: &Expr) -> BSResult<BsValue> {
        match expr {
            Expr::Null => ok(BsValue::Null),
            Expr::I64(v) => ok(BsValue::I64(*v)),
            Expr::F64(v) => ok(BsValue::F64(*v)),
            Expr::VecI64(v) => ok(BsValue::from(v.clone())),
            Expr::VecF64(v) => ok(BsValue::from(v.clone())),
            _ => compile_error("Compiler: unknown expression"),
        }
    }

    fn compile_prototype(&self) -> BSResult<FnValue<'b>> {
        let proto = &self.function.prototype;
        let args_types = std::iter::repeat(BsValue::llvm_type(self.context))
            .take(proto.args.len())
            .map(|f| f.into())
            .collect::<Vec<Type<'_>>>();
        let args_types = args_types.as_slice();

        let fn_type = self
            .context
            .fn_type(BsValue::llvm_type(self.context), args_types, false);
        let fn_val = self.module.add_function(proto.name.as_str(), fn_type);

        // set arguments names
        for (i, arg) in fn_val.get_params_iter().enumerate() {
            arg.set_name(proto.args[i].as_str());
        }

        // finally return built prototype
        ok(fn_val)
    }

    /// Creates a new stack allocation instruction in the entry block of the function.
    fn create_entry_block_alloca(&self, name: &str) -> PtrValue<'b> {
        let builder = self
            .context
            .create_builder()
            .expect("unable to create builder");

        let entry = self.fn_value.unwrap().get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry),
        }

        builder.build_alloca(self.context.f64_type().into(), name)
    }

    fn compile_fn(&mut self) -> BSResult<FnValue<'b>> {
        let function = self.compile_prototype()?;
        self.fn_value = Some(function);

        // got external function, returning only compiled prototype
        if self.function.body.is_none() {
            return ok(function);
        }

        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        // build variables map
        self.variables.reserve(self.function.prototype.args.len());

        for (i, arg) in function.get_params_iter().enumerate() {
            let arg_name = self.function.prototype.args[i].as_str();
            let alloca = self.create_entry_block_alloca(arg_name);
            self.builder.build_store(alloca, arg);
            self.variables
                .insert(self.function.prototype.args[i].clone(), alloca);
        }

        // compile body
        let body = self.compile_expr(self.function.body.as_ref().unwrap())?;

        self.builder
            .build_return(body.into_llvm_value(self.context));

        // return the whole thing after verification and optimization
        match function.verify() {
            Ok(_) => {
                // self.fpm.run_on(&function);
                ok(function)
            }
            Err(e) => {
                function.delete();
                compile_error(&format!("Compiler: function verification failed: {}", e))
            }
        }
    }

    pub fn compile(&mut self) -> BSResult<FnValue<'b>> {
        self.compile_fn()
    }
}
