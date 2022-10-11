use crate::cc::compiler::*;
use crate::parse::parser::*;
use std::collections::HashMap;
use std::fmt;

#[no_mangle]
pub extern "C" fn printd(x: f64) -> f64 {
    println!("{}", x);
    x
}

pub struct Runtime {
    builder: Builder<'static>,
    modules: HashMap<String, Module<'static>>,
    execution_engine: ExecutionEngine<'static>,
    fpm: PassManager<FunctionValue<'static>>,
    context: Box<Context>,
}

impl Runtime {
    pub fn new() -> Self {
        // Target::initialize_native(&InitializationConfig::default()).unwrap();

        let context = Box::new(Context::create());
        let main_module: Module<'static> =
            unsafe { std::mem::transmute(context.create_module("main")) };
        let builder: Builder<'static> = unsafe { std::mem::transmute(context.create_builder()) };
        let fpm = PassManager::create(&main_module);

        fpm.add_instruction_combining_pass();
        fpm.add_reassociate_pass();
        fpm.add_gvn_pass();
        fpm.add_cfg_simplification_pass();
        fpm.add_basic_alias_analysis_pass();
        fpm.add_promote_memory_to_register_pass();
        fpm.add_instruction_combining_pass();
        fpm.add_reassociate_pass();
        fpm.initialize();

        let execution_engine = main_module
            .create_jit_execution_engine(OptimizationLevel::None)
            .expect("failed to create execution engine");

        let mut modules = HashMap::new();
        modules.insert("main".to_string(), main_module);

        Self {
            builder,
            modules,
            execution_engine,
            fpm,
            context,
        }
    }

    pub fn get_module(&self, name: &str) -> &Module<'static> {
        &self.modules[name]
    }

    pub fn parse_eval(&mut self, input: String) -> Result<Box<dyn fmt::Display>, String> {
        unsafe {
            let context: &Context = std::mem::transmute(self.context.as_ref());
            let builder = &mut self.builder;
            let fpm = &mut self.fpm;

            let modname = input.clone();
            let module = context.create_module(modname.trim());
            self.execution_engine
                .add_module(&module)
                .expect("unable to add new module");

            // Build precedence map
            let mut prec = HashMap::with_capacity(6);

            prec.insert('=', 2);
            prec.insert('<', 10);
            prec.insert('+', 20);
            prec.insert('-', 20);
            prec.insert('*', 40);
            prec.insert('/', 40);

            let parsed_fn = Parser::new(input, &mut prec).parse()?;

            if parsed_fn.is_anon {
                let compiled_fn = Compiler::compile(context, builder, fpm, &module, &parsed_fn)?;
                self.modules.insert(modname, module);
                let float_type = context.f64_type();
                let res = self.execution_engine.run_function(compiled_fn, &[]);
                // compiled_fn.delete();
                Ok(Box::new(res.as_float(&float_type)))
            } else {
                Ok(Box::new("".to_string()))
            }
        }
    }
}
