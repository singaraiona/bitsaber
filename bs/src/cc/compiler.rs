use super::transform::*;

use crate::llvm::values::ValueIntrinsics;
use crate::ops::binary;
use crate::parse::ast::ExprBody;
use crate::parse::ast::{infer_types, Expr, Function};
use crate::result::*;
use crate::rt::runtime::RuntimeModule;
use ffi::Type as BSType;
use ffi::Value as BSValue;
use ffi::NULL_VALUE;
use llvm::builder::Builder;
use llvm::context::Context;
use llvm::module::Module;
use llvm::types::Type;
use llvm::values::fn_value::FnValue;
use llvm::values::prelude::PhiValue;
use llvm::values::Value;
use std::collections::HashMap;
use std::mem::transmute;

pub struct Compiler<'a, 'b> {
    module: &'a str,
    context: &'a mut Context,
    builder: &'a mut Builder<'b>,
    modules: &'a mut HashMap<String, RuntimeModule<'b>>,
    function: Function,
    variables: HashMap<String, Value<'b>>,
    fn_value_opt: Option<FnValue<'b>>,
}

impl<'a, 'b> Compiler<'a, 'b> {
    pub fn new(
        module: &'a str,
        context: &'a mut Context,
        builder: &'a mut Builder<'b>,
        modules: &'a mut HashMap<String, RuntimeModule<'b>>,
        function: Function,
    ) -> Self {
        Compiler { module, context, builder, modules, function, variables: HashMap::new(), fn_value_opt: None }
    }

    fn module(&mut self) -> &mut Module<'b> { &mut self.modules.get_mut(self.module).unwrap().module }

    fn compile_expr(&mut self, expr: &Expr) -> BSResult<Value<'a>> {
        match &expr.body {
            ExprBody::Null => ok(self.context.i64_type().const_value(NULL_VALUE).into()),
            ExprBody::Bool(b) => ok(self.context.i1_type().const_value(*b).into()),
            ExprBody::Int64(v) => ok(self.context.i64_type().const_value(*v).into()),
            ExprBody::Float64(v) => ok(self.context.f64_type().const_value(*v).into()),
            ExprBody::VecInt64(v) => unsafe {
                ok(transmute(llvm_value_from_bs_value(BSValue::from(v.clone()), self.context)))
            },
            ExprBody::VecFloat64(v) => unsafe {
                ok(transmute(llvm_value_from_bs_value(BSValue::from(v.clone()), self.context)))
            },
            ExprBody::Variable(ref name) => match self.variables.get(name.as_str()) {
                Some(&ptr) => ok(self.builder.build_load(ptr, name.as_str())),
                None => match self.modules.get(self.module).unwrap().get_global(name.as_str()) {
                    Some((ty, ptr)) => {
                        let ptr_ty = self.context.ptr_type(llvm_type_from_bs_type(ty, self.context).into());
                        let ptr = self.context.i64_type().const_value(ptr as _).to_ptr(ptr_ty);
                        ok(self.builder.build_load(ptr.into(), name.as_str()))
                    }
                    None => compile_error(
                        format!("Undefined variable: '{}'", name),
                        "Define the variable before using it".to_string(),
                        expr.span,
                    ),
                },
            },
            ExprBody::Binary { op, lhs, rhs } => {
                let lhs_e = self.compile_expr(&lhs)?;
                let rhs_e = self.compile_expr(&rhs)?;
                binary::compile(self.builder, *op, (lhs_e, lhs.get_type()?), (rhs_e, rhs.get_type()?), expr.span)
            }

            ExprBody::Assign { name, body, global } => {
                let ty = body.get_type()?;
                let body = self.compile_expr(&body)?;

                if *global {
                    self.modules.get_mut(self.module).unwrap().add_global(
                        name,
                        ty,
                        bs_value_from_llvm_value(body.clone(), ty, self.context),
                    );
                } else {
                    let ptr = self.create_entry_block_alloca(name, llvm_type_from_bs_type(ty, &self.context));
                    self.builder.build_store(ptr, body);
                    self.variables.insert(name.clone(), ptr.clone());
                }
                ok(body)
            }

            ExprBody::Call { name, args } => {
                let mut call_args = vec![];

                for arg in args {
                    let arg = self.compile_expr(arg)?;
                    call_args.push(arg);
                }

                let function = self
                    .module()
                    .get_function(name.as_str())
                    .ok_or_else(|| BSError::CompileError {
                        msg: format!("Undefined function '{}'", name),
                        desc: "Function not found".to_string(),
                        span: expr.span,
                    })?;

                ok(self.builder.build_call(function.into(), &call_args, "calltmp"))
            }

            ExprBody::Cond { cond, cons, altr } => {
                let parent = self.fn_value();
                let cond = self.compile_expr(cond)?;

                // build branches
                let then_bb = self.context.append_basic_block(parent, "then");
                let else_bb = self.context.append_basic_block(parent, "else");
                let cont_bb = self.context.append_basic_block(parent, "ifcont");

                self.builder.build_conditional_branch(cond, then_bb, else_bb);

                // build then block
                self.builder.position_at_end(then_bb);
                let then_val = cons.iter().map(|e| self.compile_expr(&e)).last().unwrap()?;

                self.builder.build_unconditional_branch(cont_bb);

                let then_bb = self.builder.get_insert_block().unwrap();

                // build else block
                self.builder.position_at_end(else_bb);
                let else_val = altr
                    .iter()
                    .map(|e| self.compile_expr(&e))
                    .last()
                    .unwrap_or_else(|| ok(self.context.i64_type().const_value(NULL_VALUE).into()))?;
                self.builder.build_unconditional_branch(cont_bb);

                let else_bb = self.builder.get_insert_block().unwrap();

                // emit merge block
                self.builder.position_at_end(cont_bb);

                let ty = cons.last().map(|l| l.get_type()).unwrap_or_else(|| ok(BSType::Null))?;

                // TODO: get rid of unsafe here
                let ty = unsafe { transmute(llvm_type_from_bs_type(ty, self.context)) };

                let phi = self.builder.build_phi(ty, "iftmp");

                let phi: PhiValue<'_> = phi.into();
                phi.add_incoming(&[(then_val, then_bb), (else_val, else_bb)]);

                ok(phi.into())
            }

            e => compile_error(format!("Compiler: unknown expression: {:?}", e), "".to_string(), expr.span),
        }
    }

    fn fn_value(&self) -> FnValue<'b> { self.fn_value_opt.unwrap() }

    /// Creates a new stack allocation instruction in the entry block of the function.
    fn create_entry_block_alloca(&self, name: &str, ty: Type<'_>) -> Value<'b> {
        let builder = self.context.create_builder().expect("unable to create builder");

        let entry = self.fn_value().get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry),
        }
        let ini = unsafe { std::mem::transmute(ty) };
        builder.build_alloca(ini, name)
    }

    pub fn compile_prototype(&self, ret_type: BSType) -> BSResult<FnValue<'b>> {
        let module = &self.modules.get(self.module).unwrap().module;
        let proto = &self.function;
        let args_types = proto
            .args
            .iter()
            .map(|(_, ty)| llvm_type_from_bs_type(*ty, &self.context))
            .collect::<Vec<Type<'_>>>();

        let args_types = args_types.as_slice();

        let fn_type = self
            .context
            .fn_type(llvm_type_from_bs_type(ret_type, &self.context), args_types, false);
        let fn_val = module.add_function(proto.name.as_str(), fn_type);

        // set arguments names
        for (i, arg) in fn_val.get_params_iter().enumerate() {
            arg.set_name(proto.args[i].0.as_str());
        }

        // finally return built prototype
        ok(fn_val)
    }

    pub fn compile_fn(&mut self) -> BSResult<(FnValue<'b>, BSType)> {
        let mut args_variables = HashMap::new();
        for (a, t) in self.function.args.iter() {
            args_variables.insert(a.clone(), *t);
        }

        let globals = &self.modules.get(self.module).unwrap().globals;
        let ret_ty = infer_types(&mut self.function.body, globals, &mut args_variables)?;
        let function = self.compile_prototype(ret_ty)?;

        // got external function, returning only compiled prototype
        if self.function.body.is_empty() {
            return ok((function, ret_ty));
        }

        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        self.fn_value_opt = Some(function);

        for (i, arg) in function.get_params_iter().enumerate() {
            let ty = arg.get_llvm_type_ref();
            let arg_name = self.function.args[i].0.as_str();
            let alloca = self.create_entry_block_alloca(arg_name, Type::new(ty));
            self.builder.build_store(alloca, arg);
            self.variables.insert(self.function.args[i].0.clone(), alloca.into());
        }

        // compile body
        let body = {
            // TODO: Fix this hack
            let body: &mut Vec<_> = unsafe { std::mem::transmute(&mut self.function.body) };
            body.into_iter().map(|e| self.compile_expr(&e)).last().unwrap()?
        };

        // build return instruction according to return type
        if ret_ty.is_scalar() {
            self.builder.build_return(body);
        } else {
            self.builder.build_aggregate_return(&[body]);
        }

        // return the whole thing after verification and optimization
        match function.verify() {
            Ok(_) => {
                // self.fpm.run_on(&function);
                self.modules.get_mut(self.module).unwrap().add_global(
                    self.function.name.as_str(),
                    ret_ty,
                    BSValue::Null,
                );
                ok((function, ret_ty))
            }
            Err(e) => {
                function.delete();
                compile_error("Compile function failed".to_string(), e.to_string(), None)
            }
        }
    }

    pub fn compile(&mut self) -> BSResult<(FnValue<'b>, BSType)> { self.compile_fn() }
}
