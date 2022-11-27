use crate::base::Type as BSType;
use crate::base::Value as BSValue;
use crate::base::NULL_VALUE;
use crate::llvm::enums::*;
use crate::llvm::values::ValueIntrinsics;
use crate::parse::ast::BinaryOp;
use crate::parse::ast::ExprBody;
use crate::parse::ast::{infer_types, Expr, Function};
use crate::parse::span::Span;
use crate::result::*;
use crate::rt::runtime::RuntimeModule;
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
    modules: &'a mut HashMap<String, RuntimeModule<'b>>,
    function: Function,
    variables: HashMap<String, Value<'b>>,
    fn_value_opt: Option<FnValue<'b>>,
}

impl<'a, 'b> Compiler<'a, 'b> {
    pub fn new(
        context: &'a mut Context,
        builder: &'a mut Builder<'b>,
        modules: &'a mut HashMap<String, RuntimeModule<'b>>,
        function: Function,
    ) -> Self {
        Compiler {
            context,
            builder,
            modules,
            function,
            variables: HashMap::new(),
            fn_value_opt: None,
        }
    }

    fn module(&mut self) -> &mut Module<'b> {
        &mut self.modules.get_mut("repl").unwrap().module
    }

    pub fn compile_binary_op(
        &self,
        op: BinaryOp,
        lhs: (Value<'a>, BSType),
        rhs: (Value<'a>, BSType),
        span: Option<Span>,
    ) -> BSResult<Value<'a>> {
        let (lhs, lhs_type) = lhs;
        let (rhs, rhs_type) = rhs;

        use BSType::*;
        use BinaryOp::*;
        use FloatPredicate as FP;
        use IntPredicate as IP;

        let result = match (op, lhs_type, rhs_type) {
            (Add, Int64, Int64) => self.builder.build_int_add(lhs, rhs, "addtmp"),
            (Add, Float64, Float64) => self.builder.build_float_add(lhs, rhs, "addtmp"),
            (Div, Int64, Int64) => self.builder.build_int_div(lhs, rhs, "divtmp"),
            (Div, Float64, Float64) => self.builder.build_float_div(lhs, rhs, "divtmp"),
            (Sub, Int64, Int64) => self.builder.build_int_sub(lhs, rhs, "subtmp"),
            (Sub, Float64, Float64) => self.builder.build_float_sub(lhs, rhs, "subtmp"),
            (Mul, Int64, Int64) => self.builder.build_int_mul(lhs, rhs, "multmp"),
            (Mul, Float64, Float64) => self.builder.build_float_mul(lhs, rhs, "multmp"),
            (Rem, Int64, Int64) => self.builder.build_rem(lhs, rhs, "remtmp"),
            (Rem, Float64, Float64) => self.builder.build_rem(lhs, rhs, "remtmp"),
            (Or, Int64, Int64) => self.builder.build_or(lhs, rhs, "ortmp"),
            (Or, Float64, Float64) => self.builder.build_or(lhs, rhs, "ortmp"),
            (And, Int64, Int64) => self.builder.build_and(lhs, rhs, "andtmp"),
            (And, Float64, Float64) => self.builder.build_and(lhs, rhs, "andtmp"),
            (Xor, Int64, Int64) => self.builder.build_xor(lhs, rhs, "xortmp"),
            (Xor, Float64, Float64) => self.builder.build_xor(lhs, rhs, "xortmp"),
            // (Shl, Int64, Int64) => self.builder.build_shl(lhs, rhs, "shltmp"),
            // (Shl, Float64, Float64) => self.builder.build_shl(lhs, rhs, "shltmp"),
            // (Shr, Int64, Int64) => self.builder.build_shr(lhs, rhs, "shrtmp"),
            // (Shr, Float64, Float64) => self.builder.build_shr(lhs, rhs, "shrtmp"),
            (Equal, Int64, Int64) => self.builder.build_int_compare(IP::EQ, lhs, rhs, "eqtmp"),
            (Equal, Float64, Float64) => {
                self.builder.build_float_compare(FP::UEQ, lhs, rhs, "eqtmp")
            }
            (Less, Int64, Int64) => self.builder.build_int_compare(IP::SLT, lhs, rhs, "lttmp"),
            (Less, Float64, Float64) => {
                self.builder.build_float_compare(FP::ULT, lhs, rhs, "lttmp")
            }
            (LessOrEqual, Int64, Int64) => {
                self.builder.build_int_compare(IP::SLE, lhs, rhs, "letmp")
            }
            (LessOrEqual, Float64, Float64) => {
                self.builder.build_float_compare(FP::ULE, lhs, rhs, "letmp")
            }
            (Greater, Int64, Int64) => self.builder.build_int_compare(IP::SGT, lhs, rhs, "gttmp"),
            (Greater, Float64, Float64) => {
                self.builder.build_float_compare(FP::UGT, lhs, rhs, "gttmp")
            }
            (GreaterOrEqual, Int64, Int64) => {
                self.builder.build_int_compare(IP::SGE, lhs, rhs, "getmp")
            }
            (GreaterOrEqual, Float64, Float64) => {
                self.builder.build_float_compare(FP::UGE, lhs, rhs, "getmp")
            }
            (NotEqual, Int64, Int64) => self.builder.build_int_compare(IP::NE, lhs, rhs, "neqtmp"),
            (NotEqual, Float64, Float64) => {
                self.builder
                    .build_float_compare(FP::UNE, lhs, rhs, "neqtmp")
            }
            (op, _, _) => {
                return compile_error(
                    format!("Unsupported binary op: '{}'", op),
                    "Refer to a supported binary operations".to_string(),
                    span,
                )
            }
        };

        ok(result)
    }

    fn compile_expr(&mut self, expr: &Expr) -> BSResult<Value<'a>> {
        match &expr.body {
            ExprBody::Null => ok(self.context.i64_type().const_value(NULL_VALUE).into()),
            ExprBody::Bool(b) => ok(self.context.i1_type().const_value(*b).into()),
            ExprBody::Int64(v) => ok(self.context.i64_type().const_value(*v).into()),
            ExprBody::Float64(v) => ok(self.context.f64_type().const_value(*v).into()),
            ExprBody::VecInt64(v) => unsafe {
                ok(std::mem::transmute(
                    BSValue::from(v.clone()).into_llvm_value(&self.context),
                ))
            },
            ExprBody::VecFloat64(v) => unsafe {
                ok(std::mem::transmute(
                    BSValue::from(v.clone()).into_llvm_value(&self.context),
                ))
            },
            ExprBody::Variable(ref name) => match self.variables.get(name.as_str()) {
                Some(v) => ok(self.builder.build_load(*v, name.as_str())),
                None => compile_error(
                    format!("Undefined symbol '{}'", name),
                    "Symbol not found".to_string(),
                    expr.span,
                ),
            },
            ExprBody::Binary { op, lhs, rhs } => {
                let lhs_e = self.compile_expr(&lhs)?;
                let rhs_e = self.compile_expr(&rhs)?;
                self.compile_binary_op(
                    *op,
                    (lhs_e, lhs.get_type()?),
                    (rhs_e, rhs.get_type()?),
                    expr.span,
                )
            }

            ExprBody::Assign {
                variable,
                body,
                global,
            } => {
                let initializer_ty = body.get_type()?;
                let body = self.compile_expr(&body)?;

                if *global {
                    self.modules.get_mut("repl").unwrap().add_global(
                        variable,
                        initializer_ty,
                        BSValue::from(body),
                    );
                } else {
                    let alloca = self.create_entry_block_alloca(
                        variable,
                        initializer_ty.into_llvm_type(&self.context),
                    );
                    self.builder.build_store(alloca, body);
                    self.variables.insert(variable.clone(), alloca);
                }
                ok(body)
            }

            ExprBody::Call { name, args } => {
                let mut call_args = vec![];

                for arg in args {
                    let arg = self.compile_expr(arg)?;
                    call_args.push(arg);
                }

                let function = self.module().get_function(name.as_str()).ok_or_else(|| {
                    BSError::CompileError {
                        msg: format!("Undefined function '{}'", name),
                        desc: "Function not found".to_string(),
                        span: expr.span,
                    }
                })?;

                ok(self
                    .builder
                    .build_call(function.into(), &call_args, "calltmp"))
            }

            e => compile_error(
                format!("Compiler: unknown expression: {:?}", e),
                "".to_string(),
                expr.span,
            ),
        }
    }

    fn fn_value(&self) -> FnValue<'b> {
        self.fn_value_opt.unwrap()
    }

    /// Creates a new stack allocation instruction in the entry block of the function.
    fn create_entry_block_alloca(&self, name: &str, ty: Type<'_>) -> Value<'b> {
        let builder = self
            .context
            .create_builder()
            .expect("unable to create builder");

        let entry = self.fn_value().get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry),
        }
        let ini = unsafe { std::mem::transmute(ty) };
        builder.build_alloca(ini, name)
    }

    fn compile_prototype(&self, ret_type: BSType) -> BSResult<FnValue<'b>> {
        let module = &self.modules.get("repl").unwrap().module;
        let proto = &self.function;
        let args_types = proto
            .args
            .iter()
            .map(|(_, ty)| ty.into_llvm_type(&self.context))
            .collect::<Vec<Type<'_>>>();

        let args_types = args_types.as_slice();

        let fn_type =
            self.context
                .fn_type(ret_type.into_llvm_type(&self.context), args_types, false);
        let fn_val = module.add_function(proto.name.as_str(), fn_type);

        // set arguments names
        for (i, arg) in fn_val.get_params_iter().enumerate() {
            arg.set_name(proto.args[i].0.as_str());
        }

        // finally return built prototype
        ok(fn_val)
    }

    fn compile_fn(&mut self) -> BSResult<(FnValue<'b>, BSType)> {
        let mut args_variables = HashMap::new();
        for (a, t) in self.function.args.iter() {
            args_variables.insert(a.clone(), *t);
        }
        let globals = &self.modules.get("repl").unwrap().globals;
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
            self.variables
                .insert(self.function.args[i].0.clone(), alloca.into());
        }

        // compile body
        let body = {
            // TODO: Fix this hack
            let body: &mut Vec<_> = unsafe { std::mem::transmute(&mut self.function.body) };
            body.into_iter()
                .map(|e| self.compile_expr(&e))
                .last()
                .unwrap()?
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
                ok((function, ret_ty))
            }
            Err(e) => {
                function.delete();
                compile_error("Compile function failed".to_string(), e.to_string(), None)
            }
        }
    }

    pub fn compile(&mut self) -> BSResult<(FnValue<'b>, BSType)> {
        self.compile_fn()
    }
}
