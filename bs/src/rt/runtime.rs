use crate::cc::compiler::*;
use crate::parse::parser::*;
use std::collections::HashMap;

#[no_mangle]
pub extern "C" fn printd(x: f64) -> f64 {
    println!("{}", x);
    x
}

// Adding the functions above to a global array,
// so Rust compiler won't remove them.
#[used]
static EXTERNAL_FNS: [extern "C" fn(f64) -> f64; 1] = [printd];

struct ModuleExecutionEngine {
    module: Module<'static>,
    executor: Box<dyn FnMut(FunctionValue) -> Result<String, String>>,
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
                Ok(f) => unsafe { Ok(format!("{}", f.call())) },
                Err(err) => Err(format!("EE get function `{}`: {:?}", name, err)),
            }
        };

        Self {
            module,
            executor: Box::new(executor),
        }
    }

    fn execute(&mut self, name: &str) -> Result<String, String> {
        let main = self.module.get_function(name).unwrap();
        (self.executor)(main)
    }
}

pub struct Runtime {
    builder: Builder<'static>,
    modules: HashMap<&'static str, ModuleExecutionEngine>,
    fpm: PassManager<FunctionValue<'static>>,
    prec: HashMap<char, i32>,
    context: Box<Context>,
    parsed_exprs: Vec<Function>,
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
            context,
            builder,
            modules,
            fpm,
            prec,
            parsed_exprs: Default::default(),
        }
    }

    pub fn get_module(&self, name: &str) -> &Module<'static> {
        &self.modules[name].module
    }

    pub fn parse_eval(&mut self, input: String) -> Result<String, String> {
        self.modules.entry("_main_").or_insert_with(|| {
            let module: Module<'static> =
                unsafe { std::mem::transmute(self.context.create_module("_main_")) };
            ModuleExecutionEngine::new(module)
        });


        let module = self.context.create_module("tmp");
        let parsed_fn = Parser::new(input, &mut self.prec).parse()?;
        let ctx = unsafe { std::mem::transmute(self.context.as_ref()) };
        let module = self.modules[name].module;
        let builder = &mut self.builder;
        let fpm = &mut self.fpm;

        // recompile every previously parsed function into the new module
        for prev in &self.parsed_exprs {
            Compiler::compile(ctx, builder, fpm, &module, prev)
                .expect("Cannot re-add previously compiled function.");
        }

        let compiled_fn = Compiler::compile(ctx, builder, fpm, &module, &parsed_fn)?;

        if parsed_fn.is_anon {
            let ee = module
                .create_jit_execution_engine(OptimizationLevel::Aggressive)
                .unwrap();
            let name = compiled_fn.get_name().to_str().unwrap();
            let maybe_fn = unsafe { ee.get_function::<unsafe extern "C" fn() -> f64>(name) };
            match maybe_fn {
                Ok(f) => unsafe { Ok(format!("{}", f.call())) },
                Err(err) => Err(format!("EE get function `{}`: {:?}", name, err)),
            }
        } else {
            self.parsed_exprs.push(parsed_fn);
            Ok("".to_string())
        }
    }
}
