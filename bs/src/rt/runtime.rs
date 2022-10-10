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
        Target::initialize_native(&InitializationConfig::default()).unwrap();

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
            .create_jit_execution_engine(OptimizationLevel::Aggressive)
            .expect("failed to create jit engine");

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
        let parsed_fn = Parser::new(input).parse()?;
        let context: &Context = unsafe { std::mem::transmute(self.context.as_ref()) };
        let module = context.create_module("repl");
        self.execution_engine.add_module(&module).unwrap();
        let builder = &mut self.builder;
        let fpm = &mut self.fpm;

        let compiled_fn = Compiler::compile(context, builder, fpm, &module, &parsed_fn)?;

        if parsed_fn.is_anon {
            let name = compiled_fn.get_name().to_str().unwrap();

            let maybe_fn = unsafe {
                self.execution_engine
                    .get_function::<unsafe extern "C" fn() -> f64>(name)
            };

            let res = match maybe_fn {
                Ok(f) => unsafe { Ok(Box::new(f.call()) as _) },
                Err(err) => Err(format!(
                    "Execution engine get function `{}`: {:?}",
                    name, err
                )),
            };

            self.execution_engine.remove_module(&module).unwrap();

            res
        } else {
            Ok(Box::new("".to_string()))
        }
    }
}
