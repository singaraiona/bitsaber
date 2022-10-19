use llvm_sys::core::*;
use llvm_sys::prelude::LLVMPassManagerRef;
use llvm_sys::transforms::aggressive_instcombine::*;
use llvm_sys::transforms::coroutines::*;
use llvm_sys::transforms::ipo::*;
use llvm_sys::transforms::pass_manager_builder::*;
use llvm_sys::transforms::scalar::*;
use llvm_sys::transforms::util::*;
use llvm_sys::transforms::vectorize::*;
use std::marker::PhantomData;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum OptimizationLevel {
    None = 0,
    Less = 1,
    Default = 2,
    Aggressive = 3,
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        OptimizationLevel::Default
    }
}

#[derive(Debug)]
pub struct PassManager<'a> {
    llvm_pass_manager: LLVMPassManagerRef,
    _phantom: PhantomData<&'a ()>,
}

pub struct PassManagerBuilder<'a> {
    pass_manager: PassManager<'a>,
}

impl<'a> PassManagerBuilder<'a> {
    // return true means some pass modified the module, not an error occurred
    pub fn initialize(&self) -> bool {
        unsafe { LLVMInitializeFunctionPassManager(self.pass_manager) == 1 }
    }

    pub fn finalize(&self) -> bool {
        unsafe { LLVMFinalizeFunctionPassManager(self.pass_manager) == 1 }
    }

    pub fn new() -> Self {
        let pass_manager = unsafe {
            PassManager {
                llvm_pass_manager: LLVMCreatePassManager(),
                _phantom: PhantomData,
            }
        };

        PassManagerBuilder { pass_manager }
    }

    pub fn set_optimization_level(&self, opt_level: OptimizationLevel) {
        unsafe { LLVMPassManagerBuilderSetOptLevel(self.pass_manager_builder, opt_level as u32) }
    }

    pub fn set_size_level(&self, size_level: u32) {
        unsafe { LLVMPassManagerBuilderSetSizeLevel(self.pass_manager_builder, size_level) }
    }

    pub fn set_disable_unit_at_a_time(&self, disable: bool) {
        unsafe {
            LLVMPassManagerBuilderSetDisableUnitAtATime(self.pass_manager_builder, disable as i32)
        }
    }

    pub fn set_disable_unroll_loops(&self, disable: bool) {
        unsafe {
            LLVMPassManagerBuilderSetDisableUnrollLoops(self.pass_manager_builder, disable as i32)
        }
    }

    pub fn set_disable_simplify_lib_calls(&self, disable: bool) {
        unsafe {
            LLVMPassManagerBuilderSetDisableSimplifyLibCalls(
                self.pass_manager_builder,
                disable as i32,
            )
        }
    }

    pub fn set_inliner_with_threshold(&self, threshold: u32) {
        unsafe {
            LLVMPassManagerBuilderUseInlinerWithThreshold(self.pass_manager_builder, threshold)
        }
    }

    pub fn run_on(&self, input: &T) -> bool {
        unsafe { input.run_in_pass_manager(self) }
    }

    pub fn add_argument_promotion_pass(&self) {
        unsafe { LLVMAddArgumentPromotionPass(self.pass_manager) }
    }

    pub fn add_constant_merge_pass(&self) {
        unsafe { LLVMAddConstantMergePass(self.pass_manager) }
    }

    pub fn add_merge_functions_pass(&self) {
        unsafe { LLVMAddMergeFunctionsPass(self.pass_manager) }
    }

    pub fn add_dead_arg_elimination_pass(&self) {
        unsafe { LLVMAddDeadArgEliminationPass(self.pass_manager) }
    }

    pub fn add_function_attrs_pass(&self) {
        unsafe { LLVMAddFunctionAttrsPass(self.pass_manager) }
    }

    pub fn add_function_inlining_pass(&self) {
        unsafe { LLVMAddFunctionInliningPass(self.pass_manager) }
    }

    /// A custom inliner that handles only functions that are marked as “always inline”.
    pub fn add_always_inliner_pass(&self) {
        unsafe { LLVMAddAlwaysInlinerPass(self.pass_manager) }
    }

    pub fn add_global_dce_pass(&self) {
        unsafe { LLVMAddGlobalDCEPass(self.pass_manager) }
    }

    pub fn add_global_optimizer_pass(&self) {
        unsafe { LLVMAddGlobalOptimizerPass(self.pass_manager) }
    }

    pub fn add_ip_constant_propagation_pass(&self) {
        unsafe { LLVMAddIPConstantPropagationPass(self.pass_manager) }
    }

    pub fn add_prune_eh_pass(&self) {
        unsafe { LLVMAddPruneEHPass(self.pass_manager) }
    }

    pub fn add_ipsccp_pass(&self) {
        unsafe { LLVMAddIPSCCPPass(self.pass_manager) }
    }

    pub fn add_internalize_pass(&self, all_but_main: bool) {
        unsafe { LLVMAddInternalizePass(self.pass_manager, all_but_main as u32) }
    }

    pub fn add_strip_dead_prototypes_pass(&self) {
        unsafe { LLVMAddStripDeadPrototypesPass(self.pass_manager) }
    }

    pub fn add_strip_symbol_pass(&self) {
        unsafe { LLVMAddStripSymbolsPass(self.pass_manager) }
    }

    pub fn add_loop_vectorize_pass(&self) {
        unsafe { LLVMAddLoopVectorizePass(self.pass_manager) }
    }

    pub fn add_slp_vectorize_pass(&self) {
        unsafe { LLVMAddSLPVectorizePass(self.pass_manager) }
    }

    pub fn add_aggressive_dce_pass(&self) {
        unsafe { LLVMAddAggressiveDCEPass(self.pass_manager) }
    }

    pub fn add_bit_tracking_dce_pass(&self) {
        unsafe { LLVMAddBitTrackingDCEPass(self.pass_manager) }
    }

    pub fn add_alignment_from_assumptions_pass(&self) {
        unsafe { LLVMAddAlignmentFromAssumptionsPass(self.pass_manager) }
    }

    pub fn add_cfg_simplification_pass(&self) {
        unsafe { LLVMAddCFGSimplificationPass(self.pass_manager) }
    }

    pub fn add_dead_store_elimination_pass(&self) {
        unsafe { LLVMAddDeadStoreEliminationPass(self.pass_manager) }
    }

    pub fn add_scalarizer_pass(&self) {
        unsafe { LLVMAddScalarizerPass(self.pass_manager) }
    }

    pub fn add_merged_load_store_motion_pass(&self) {
        unsafe { LLVMAddMergedLoadStoreMotionPass(self.pass_manager) }
    }

    pub fn add_gvn_pass(&self) {
        unsafe { LLVMAddGVNPass(self.pass_manager) }
    }

    pub fn add_new_gvn_pass(&self) {
        unsafe { LLVMAddNewGVNPass(self.pass_manager) }
    }

    pub fn add_ind_var_simplify_pass(&self) {
        unsafe { LLVMAddIndVarSimplifyPass(self.pass_manager) }
    }

    pub fn add_instruction_combining_pass(&self) {
        unsafe { LLVMAddInstructionCombiningPass(self.pass_manager) }
    }

    pub fn add_jump_threading_pass(&self) {
        unsafe { LLVMAddJumpThreadingPass(self.pass_manager) }
    }

    pub fn add_licm_pass(&self) {
        unsafe { LLVMAddLICMPass(self.pass_manager) }
    }

    pub fn add_loop_deletion_pass(&self) {
        unsafe { LLVMAddLoopDeletionPass(self.pass_manager) }
    }

    pub fn add_loop_idiom_pass(&self) {
        unsafe { LLVMAddLoopIdiomPass(self.pass_manager) }
    }

    pub fn add_loop_rotate_pass(&self) {
        unsafe { LLVMAddLoopRotatePass(self.pass_manager) }
    }

    pub fn add_loop_reroll_pass(&self) {
        unsafe { LLVMAddLoopRerollPass(self.pass_manager) }
    }

    pub fn add_loop_unroll_pass(&self) {
        unsafe { LLVMAddLoopUnrollPass(self.pass_manager) }
    }

    pub fn add_loop_unswitch_pass(&self) {
        unsafe { LLVMAddLoopUnswitchPass(self.pass_manager) }
    }

    pub fn add_memcpy_optimize_pass(&self) {
        unsafe { LLVMAddMemCpyOptPass(self.pass_manager) }
    }

    pub fn add_partially_inline_lib_calls_pass(&self) {
        unsafe { LLVMAddPartiallyInlineLibCallsPass(self.pass_manager) }
    }

    pub fn add_lower_switch_pass(&self) {
        unsafe { LLVMAddLowerSwitchPass(self.pass_manager) }
    }

    pub fn add_promote_memory_to_register_pass(&self) {
        unsafe { LLVMAddPromoteMemoryToRegisterPass(self.pass_manager) }
    }

    pub fn add_reassociate_pass(&self) {
        unsafe { LLVMAddReassociatePass(self.pass_manager) }
    }

    pub fn add_sccp_pass(&self) {
        unsafe { LLVMAddSCCPPass(self.pass_manager) }
    }

    pub fn add_scalar_repl_aggregates_pass(&self) {
        unsafe { LLVMAddScalarReplAggregatesPass(self.pass_manager) }
    }

    pub fn add_scalar_repl_aggregates_pass_ssa(&self) {
        unsafe { LLVMAddScalarReplAggregatesPassSSA(self.pass_manager) }
    }

    pub fn add_scalar_repl_aggregates_pass_with_threshold(&self, threshold: i32) {
        unsafe { LLVMAddScalarReplAggregatesPassWithThreshold(self.pass_manager, threshold) }
    }

    pub fn add_simplify_lib_calls_pass(&self) {
        unsafe { LLVMAddSimplifyLibCallsPass(self.pass_manager) }
    }

    pub fn add_tail_call_elimination_pass(&self) {
        unsafe { LLVMAddTailCallEliminationPass(self.pass_manager) }
    }

    pub fn add_instruction_simplify_pass(&self) {
        unsafe { LLVMAddInstructionSimplifyPass(self.pass_manager) }
    }

    pub fn add_demote_memory_to_register_pass(&self) {
        unsafe { LLVMAddDemoteMemoryToRegisterPass(self.pass_manager) }
    }

    pub fn add_verifier_pass(&self) {
        unsafe { LLVMAddVerifierPass(self.pass_manager) }
    }

    pub fn add_correlated_value_propagation_pass(&self) {
        unsafe { LLVMAddCorrelatedValuePropagationPass(self.pass_manager) }
    }

    pub fn add_early_cse_pass(&self) {
        unsafe { LLVMAddEarlyCSEPass(self.pass_manager) }
    }

    pub fn add_early_cse_mem_ssa_pass(&self) {
        unsafe { LLVMAddEarlyCSEMemSSAPass(self.pass_manager) }
    }

    pub fn add_lower_expect_intrinsic_pass(&self) {
        unsafe { LLVMAddLowerExpectIntrinsicPass(self.pass_manager) }
    }

    pub fn add_type_based_alias_analysis_pass(&self) {
        unsafe { LLVMAddTypeBasedAliasAnalysisPass(self.pass_manager) }
    }

    pub fn add_scoped_no_alias_aa_pass(&self) {
        unsafe { LLVMAddScopedNoAliasAAPass(self.pass_manager) }
    }

    pub fn add_basic_alias_analysis_pass(&self) {
        unsafe { LLVMAddBasicAliasAnalysisPass(self.pass_manager) }
    }

    pub fn add_aggressive_inst_combiner_pass(&self) {
        unsafe { LLVMAddAggressiveInstCombinerPass(self.pass_manager) }
    }

    pub fn add_loop_unroll_and_jam_pass(&self) {
        unsafe { LLVMAddLoopUnrollAndJamPass(self.pass_manager) }
    }

    pub fn add_coroutine_early_pass(&self) {
        unsafe { LLVMAddCoroEarlyPass(self.pass_manager) }
    }

    pub fn add_coroutine_split_pass(&self) {
        unsafe { LLVMAddCoroSplitPass(self.pass_manager) }
    }

    pub fn add_coroutine_elide_pass(&self) {
        unsafe { LLVMAddCoroElidePass(self.pass_manager) }
    }

    pub fn add_coroutine_cleanup_pass(&self) {
        unsafe { LLVMAddCoroCleanupPass(self.pass_manager) }
    }
}

impl<T> Drop for PassManager<T> {
    fn drop(&mut self) {
        unsafe { LLVMDisposePassManager(self.pass_manager) }
    }
}
