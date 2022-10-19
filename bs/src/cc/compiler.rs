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
        }
    }

    fn compile_expr(&self, expr: &Expr) -> BSResult<Value<'b>> {
        match expr {
            // Expr::Null => ok(Value::Null),
            Expr::I64(v) => ok(self.context.i64_type().const_value(*v).into()),
            Expr::F64(v) => ok(self.context.f64_type().const_value(*v).into()),
            // Expr::VecI64(v) => ok(BsValue::from(v.clone())),
            // Expr::VecF64(v) => ok(BsValue::from(v.clone())),
            Expr::Binary { op, lhs, rhs } => {
                let lhs = self.compile_expr(lhs)?;
                let rhs = self.compile_expr(rhs)?;

                match op {
                    '+' => ok(self.builder.build_add(lhs, rhs, "tmpadd")),
                    _ => todo!(),
                }
            }
            _ => compile_error("Compiler: unknown expression"),
        }
    }

    fn compile_prototype(&self, ret_type: Type<'a>) -> BSResult<FnValue<'b>> {
        let proto = &self.function.prototype;
        // let args_types = std::iter::repeat(BsValue::llvm_type(self.context))
        //     .take(proto.args.len())
        //     .map(|f| f.into())
        //     .collect::<Vec<Type<'_>>>();
        // let args_types = args_types.as_slice();

        let args_types = &[];

        let fn_type = self.context.fn_type(ret_type, args_types, false);
        let fn_val = self.module.add_function(proto.name.as_str(), fn_type);

        // set arguments names
        // for (i, arg) in fn_val.get_params_iter().enumerate() {
        //     arg.set_name(proto.args[i].as_str());
        // }

        // finally return built prototype
        ok(fn_val)
    }

    /// Creates a new stack allocation instruction in the entry block of the function.
    fn create_entry_block_alloca(&self, name: &str, fn_value: FnValue<'_>) -> Value<'b> {
        let builder = self
            .context
            .create_builder()
            .expect("unable to create builder");

        let entry = fn_value.get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry),
        }

        builder.build_alloca(self.context.f64_type().into(), name)
    }

    fn compile_fn(&mut self) -> BSResult<FnValue<'b>> {
        // compile body
        let body = self.compile_expr(self.function.body.as_ref().unwrap())?;
        let function = self.compile_prototype(Type::new(body.get_llvm_type_ref()))?;

        let variables = &mut self.variables;

        // got external function, returning only compiled prototype
        if self.function.body.is_none() {
            return ok(function);
        }

        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        // build variables map
        variables.reserve(self.function.prototype.args.len());

        for (i, arg) in function.get_params_iter().enumerate() {
            let arg_name = self.function.prototype.args[i].as_str();
            let alloca = self.create_entry_block_alloca(arg_name, function);
            self.builder.build_store(alloca, arg);
            self.variables
                .insert(self.function.prototype.args[i].clone(), alloca.into());
        }

        self.builder.build_return(body);

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
