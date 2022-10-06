use crate::cc::compiler::*;
use crate::parse::parser::*;
use std::collections::HashMap;
use std::fmt;

#[no_mangle]
pub extern "C" fn printd(x: f64) -> f64 {
    println!("{}", x);
    x
}

struct ModuleExecutionEngine {
    module: Module<'static>,
    executor: Box<dyn FnMut(FunctionValue) -> Result<Box<dyn fmt::Display>, String>>,
}

impl ModuleExecutionEngine {
    fn new(module: Module<'static>) -> Self {
        let ee = module
            .create_jit_execution_engine(OptimizationLevel::Aggressive)
            .unwrap();

        let executor = move |compiled_fn: FunctionValue| {
            let name = compiled_fn.get_name().to_str().unwrap();
            let maybe_fn = unsafe { ee.get_function::<unsafe extern "C" fn() -> f64>(name) };
            match maybe_fn {
                Ok(f) => unsafe { Ok(Box::new(f.call()) as _) },
                Err(err) => Err(format!(
                    "Execution engine get function `{}`: {:?}",
                    name, err
                )),
            }
        };

        Self {
            module,
            executor: Box::new(executor),
        }
    }

    fn execute(&mut self, name: &str) -> Result<Box<dyn fmt::Display>, String> {
        if let Some(main) = self.module.get_function(name) {
            (self.executor)(main)
        } else {
            Err(format!("Function `{}` not found", name))
        }
    }
}

pub struct Runtime {
    builder: Builder<'static>,
    modules: HashMap<&'static str, ModuleExecutionEngine>,
    fpm: PassManager<FunctionValue<'static>>,
    prec: HashMap<char, i32>,
    context: Box<Context>,
}

impl Runtime {
    pub fn new() -> Self {
        let context = Box::new(Context::create());
        let main_module: Module<'static> =
            unsafe { std::mem::transmute(context.create_module("_main_")) };
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

        // Build precedence map
        let mut prec = HashMap::with_capacity(6);

        prec.insert('=', 2);
        prec.insert('<', 10);
        prec.insert('+', 20);
        prec.insert('-', 20);
        prec.insert('*', 40);
        prec.insert('/', 40);

        let mut modules = HashMap::new();
        modules.insert("_main_", ModuleExecutionEngine::new(main_module));

        Self {
            builder,
            modules,
            fpm,
            prec,
            context,
        }
    }

    pub fn get_module(&self, name: &str) -> &Module<'static> {
        &self.modules[name].module
    }

    pub fn parse_eval(&mut self, input: String) -> Result<Box<dyn fmt::Display>, String> {
        let pseudonim = input.clone();
        let parsed_fn = Parser::new(input, &mut self.prec).parse(pseudonim.trim())?;
        let ctx = unsafe { std::mem::transmute(self.context.as_ref()) };
        let module_executor = &mut self.modules.get_mut("_main_").unwrap();
        let module = &module_executor.module;
        let builder = &mut self.builder;
        let fpm = &mut self.fpm;

        let compiled_fn = Compiler::compile(ctx, builder, fpm, module, &parsed_fn)?;

        if parsed_fn.is_anon {
            let name = compiled_fn.get_name().to_str().unwrap();
            module_executor.execute(name)
        } else {
            Ok(Box::new("".to_string()))
        }
    }
}
