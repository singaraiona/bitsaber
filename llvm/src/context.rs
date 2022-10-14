use crate::basic_block::BasicBlock;
use crate::builder::Builder;
use crate::module::Module;
use crate::types::fn_type::FnType;
use crate::types::i64_type::*;
use crate::types::*;
use crate::utils::to_c_str;
use crate::values::fn_value::FnValue;
use llvm_sys::core::*;
use llvm_sys::execution_engine::*;
use llvm_sys::prelude::LLVMContextRef;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::target::*;

pub struct Context {
    llvm_context: LLVMContextRef,
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.llvm_context);
        }
    }
}

impl Context {
    pub fn new() -> Result<Context, &'static str> {
        unsafe {
            let r = LLVM_InitializeNativeTarget();
            if r != 0 {
                return Err("Context: could not initialize native target");
            }
            let r = LLVM_InitializeNativeAsmPrinter();
            if r != 0 {
                return Err("Context: could not initialize native asm printer");
            }
            LLVMLinkInMCJIT();

            let llvm_context = LLVMContextCreate();
            if llvm_context.is_null() {
                return Err("Context: could not create LLVM context");
            }
            Ok(Context { llvm_context })
        }
    }

    pub fn create_builder<'a>(&self) -> Result<Builder<'a>, &'static str> {
        unsafe {
            let llvm_builder = LLVMCreateBuilderInContext(self.llvm_context);
            if llvm_builder.is_null() {
                return Err("Context: could not create LLVM builder");
            }
            Ok(Builder::new(llvm_builder))
        }
    }

    pub fn create_module<'a>(&self, name: &str) -> Result<Module<'a>, &'static str> {
        let c_string = to_c_str(name);

        unsafe {
            let llvm_module =
                LLVMModuleCreateWithNameInContext(c_string.as_ptr(), self.llvm_context);

            if llvm_module.is_null() {
                return Err("Context: could not create LLVM module");
            }
            Ok(Module::new(llvm_module))
        }
    }

    pub fn i64_type<'a>(&self) -> I64Type<'a> {
        unsafe { I64Type::new(LLVMInt64TypeInContext(self.llvm_context)) }
    }

    pub fn fn_i64_type<'a>(&self, param_types: &[Type<'a>], is_var_args: bool) -> FnType<'a> {
        let mut param_types: Vec<LLVMTypeRef> =
            param_types.iter().map(|ty| ty.as_llvm_type_ref()).collect();

        unsafe {
            FnType::new(LLVMFunctionType(
                self.i64_type().ty.into(),
                param_types.as_mut_ptr(),
                param_types.len() as u32,
                is_var_args as i32,
            ))
        }
    }

    pub fn append_basic_block<'a>(&self, function: FnValue<'a>, name: &str) -> BasicBlock<'a> {
        let c_string = to_c_str(name);

        unsafe {
            BasicBlock::new(LLVMAppendBasicBlockInContext(
                self.llvm_context,
                function.val.as_llvm_value_ref(),
                c_string.as_ptr(),
            ))
            .expect("Appending basic block should never fail")
        }
    }

    pub fn insert_basic_block_after<'ctx>(
        &self,
        basic_block: BasicBlock<'ctx>,
        name: &str,
    ) -> BasicBlock<'ctx> {
        match basic_block.get_next_basic_block() {
            Some(next_basic_block) => self.prepend_basic_block(next_basic_block, name),
            None => {
                let parent_fn = basic_block.get_parent().unwrap();

                self.append_basic_block(parent_fn, name)
            }
        }
    }

    pub fn prepend_basic_block<'ctx>(
        &self,
        basic_block: BasicBlock<'ctx>,
        name: &str,
    ) -> BasicBlock<'ctx> {
        let c_string = to_c_str(name);

        unsafe {
            BasicBlock::new(LLVMInsertBasicBlockInContext(
                self.llvm_context,
                basic_block.basic_block,
                c_string.as_ptr(),
            ))
            .expect("Prepending basic block should never fail")
        }
    }
}
