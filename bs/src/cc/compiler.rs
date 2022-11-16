use crate::base::binary::Op;
use crate::base::infer::infer_type;
use crate::base::Type as BSType;
use crate::base::Value as BsValue;
use crate::llvm::values::ptr_value::PtrValue;
use crate::parse::ast::ExprBody;
use crate::parse::ast::{Expr, Function, Prototype};
use crate::parse::span::Span;
use crate::result::*;
use llvm::builder::Builder;
use llvm::context::Context;
use llvm::module::Module;
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

    pub fn compile_binary_op(
        &self,
        op: Op,
        lhs: (Value<'a>, BSType),
        rhs: (Value<'a>, BSType),
        span: Option<Span>,
    ) -> BSResult<(Value<'a>, BSType)> {
        let (lhs, lhs_type) = lhs;
        let (rhs, rhs_type) = rhs;
        let result_type = infer_type(op, lhs_type, rhs_type, span)?;

        use BSType::*;
        use Op::*;

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
            op => {
                return compile_error(
                    format!("Unsupported binary op: {:?}", op),
                    "Refer to a supported binary operations".to_string(),
                    span,
                )
            }
        };

        ok((result, result_type))
    }

    fn compile_expr(&self, expr: &Expr) -> BSResult<(Value<'a>, BSType)> {
        match &expr.body {
            // Expr::Null => ok((Value::Null, BSType::Null)),
            ExprBody::Int64(v) => ok((
                self.context.i64_type().const_value(*v).into(),
                BSType::Int64,
            )),
            ExprBody::Float64(v) => ok((
                self.context.f64_type().const_value(*v).into(),
                BSType::Float64,
            )),
            ExprBody::VecInt64(v) => {
                let bsval = unsafe {
                    std::mem::transmute(BsValue::from(v.clone()).into_llvm_value(&self.context))
                };
                ok((bsval, BSType::VecInt64))
            }
            ExprBody::VecFloat64(v) => {
                let bsval = unsafe {
                    std::mem::transmute(BsValue::from(v.clone()).into_llvm_value(&self.context))
                };
                ok((bsval, BSType::VecFloat64))
            }
            // Expr::VecFloat64(v) => ok(BsValue::from(v.clone())),
            ExprBody::Binary { op, lhs, rhs } => {
                let lhs = self.compile_expr(&lhs)?;
                let rhs = self.compile_expr(&rhs)?;
                self.compile_binary_op(*op, lhs, rhs, expr.span)
            }

            ExprBody::Assign { variable, body } => {
                let body = self.compile_expr(&body)?;

                // let alloca = self.create_entry_block_alloca(variable);
                // self.builder.build_store(alloca, initial_val);

                // let variable = self.variables.get(variable).unwrap();
                // self.builder.build_store(*variable, body.0);

                ok(body)
            }

            e => compile_error(
                format!("Compiler: unknown expression: {:?}", e),
                "".to_string(),
                expr.span,
            ),
        }
    }

    fn compile_prototype(&self, ret_type: BSType) -> BSResult<(FnValue<'b>, BSType)> {
        let proto = &self.function.prototype;
        // let args_types = std::iter::repeat(BsValue::llvm_type(self.context))
        //     .take(proto.args.len())
        //     .map(|f| f.into())
        //     .collect::<Vec<Type<'_>>>();
        // let args_types = args_types.as_slice();

        let args_types = &[];

        let fn_type =
            self.context
                .fn_type(ret_type.into_llvm_type(&self.context), args_types, false);
        let fn_val = self.module.add_function(proto.name.as_str(), fn_type);

        // println!("{:?}", fn_val.get_return_type());

        // set arguments names
        // for (i, arg) in fn_val.get_params_iter().enumerate() {
        //     arg.set_name(proto.args[i].as_str());
        // }

        // finally return built prototype
        ok((fn_val, ret_type))
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

    fn compile_fn(&mut self) -> BSResult<(FnValue<'b>, BSType)> {
        // compile body
        let (body, ret_ty) = self.compile_expr(self.function.body.as_ref().unwrap())?;
        let (function, ret_ty) = self.compile_prototype(ret_ty)?;

        let variables = &mut self.variables;

        // got external function, returning only compiled prototype
        if self.function.body.is_none() {
            return ok((function, ret_ty));
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
                compile_error(
                    "Compile function failed".to_string(),
                    e.to_string(),
                    self.function.span,
                )
            }
        }
    }

    pub fn compile(&mut self) -> BSResult<(FnValue<'b>, BSType)> {
        self.compile_fn()
    }
}
