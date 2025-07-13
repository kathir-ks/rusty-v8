// Converted from V8 C++ source files:
// Header: pipelines.h
// Implementation: pipelines.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod pipelines {
    use std::borrow::Cow;
    use std::cell::RefCell;
    use std::collections::{HashMap, HashSet};
    use std::ffi;
    use std::fmt;
    use std::io;
    use std::mem;
    use std::ops::{Deref, DerefMut};
    use std::rc::Rc;
    use std::sync::{Arc, Mutex, RwLock};

    use crate::codegen::optimized_compilation_info::OptimizedCompilationInfo;
    use crate::compiler::backend::register_allocator_verifier::RegisterAllocatorVerifier;
    use crate::compiler::basic_block_instrumentor::BasicBlockCallGraphProfiler;
    use crate::compiler::pipeline_statistics::PipelineStatistics;
    use crate::compiler::turbofan_graph_visualizer::PrintTurboshaftGraph;
    use crate::compiler::turboshaft::block_instrumentation_phase::BlockInstrumentationPhase;
    use crate::compiler::turboshaft::build_graph_phase::BuildGraphPhase;
    use crate::compiler::turboshaft::code_elimination_and_simplification_phase::CodeEliminationAndSimplificationPhase;
    use crate::compiler::turboshaft::debug_feature_lowering_phase::DebugFeatureLoweringPhase;
    use crate::compiler::turboshaft::decompression_optimization_phase::DecompressionOptimizationPhase;
    use crate::compiler::turboshaft::instruction_selection_phase::InstructionSelectionPhase;
    use crate::compiler::turboshaft::loop_peeling_phase::LoopPeelingPhase;
    use crate::compiler::turboshaft::loop_unrolling_phase::LoopUnrollingPhase;
    use crate::compiler::turboshaft::machine_lowering_phase::MachineLoweringPhase;
    use crate::compiler::turboshaft::maglev_graph_building_phase::MaglevGraphBuildingPhase;
    use crate::compiler::turboshaft::optimize_phase::OptimizePhase;
    use crate::compiler::turboshaft::phase::Phase;
    use crate::compiler::turboshaft::register_allocation_phase::*;
    use crate::compiler::turboshaft::sidetable::SideTable;
    use crate::compiler::turboshaft::store_store_elimination_phase::StoreStoreEliminationPhase;
    use crate::compiler::turboshaft::tracing::Tracing;
    use crate::compiler::turboshaft::type_assertions_phase::TypeAssertionsPhase;
    use crate::compiler::turboshaft::typed_optimizations_phase::TypedOptimizationsPhase;
    use crate::execution::isolate::Isolate;
    use crate::objects::code::Code;
    use crate::objects::shared::SharedFunctionInfo;
    use std::any::Any;

    use crate::base;
    use crate::compiler::common::ProfileDataFromFile;
    use crate::compiler::pipeline_data::PipelineData;
    use crate::compiler::turboshaft::csa_optimize_phase::*;
    use crate::compiler::turboshaft::instruction_selection_normalization_reducer::InstructionSelectionNormalizationReducer;
    use crate::compiler::turboshaft::load_store_simplification_reducer::LoadStoreSimplificationReducer;
    use crate::execution::frames::Frame;
    use crate::execution::vm_state::ExternalCallbackScope;
    use crate::handles::handles::*;
    use crate::handles::maybe_handles::MaybeHandle;
    use crate::handles::maybe_handles::MaybeIndirectHandle;
    use crate::objects::heap_object::HeapObject;
    use crate::objects::string::String;
    use crate::zone::zone::Zone;
    use crate::zone::zone_handle::ZoneHandle;

    use crate::compiler::turboshaft::graph::Graph;
    use crate::compiler::turboshaft::graph_builder::Linkage;
    use crate::compiler::turboshaft::instruction_selection_phase::UnparkedScopeIfNeeded;
    use crate::compiler::turboshaft::instruction_selection_phase::TurboJsonFile;
    use crate::utils::code_tracer::CodeTracer;
    use crate::utils::source_position_table::SourcePositionTable;

    use crate::compiler::register_configuration::RegisterConfiguration;
    use crate::compiler::turboshaft::code_generator::CodeGenerator;
    use crate::compiler::turboshaft::instruction::Instruction;
    use crate::compiler::turboshaft::register_allocation_data::RegisterAllocationData;
    use crate::execution::isolate::Address;
    use crate::flags::FlagList;
    use crate::flags;
    use crate::objects::descriptor::Descriptor;
    use std::io::Write;

    const K_TEMP_ZONE_NAME: &str = "temp-zone";

    pub struct SimplificationAndNormalizationPhase;

    impl SimplificationAndNormalizationPhase {
        const PHASE_NAME: &'static str = "SimplificationAndNormalization";
        const RUNTIME_CALL_COUNTER_ID: i32 = 0;
        const COUNTER_MODE: i32 = 0;
    }

    impl SimplificationAndNormalizationPhase {
        pub fn Run(data: *mut PipelineData, temp_zone: *mut Zone) {
            copying_phase::CopyingPhase::<
                LoadStoreSimplificationReducer,
                InstructionSelectionNormalizationReducer,
            >::Run(data, temp_zone);
        }
    }

    pub mod copying_phase {
        use super::*;
        pub struct CopyingPhase<R1, R2> {
            _phantom: std::marker::PhantomData<(R1, R2)>,
        }
        impl<R1, R2> CopyingPhase<R1, R2> {
            pub fn Run(data: *mut PipelineData, temp_zone: *mut Zone) {}
        }
    }
    
    pub mod assemble_code_phase {
        use super::*;
        pub struct AssembleCodePhase {}
        impl AssembleCodePhase {
            pub fn Run(data: *mut PipelineData) {}
        }
    }

    pub mod finalize_code_phase {
        use super::*;
        pub struct FinalizeCodePhase {}
        impl FinalizeCodePhase {
            pub fn Run(data: *mut PipelineData) {}
        }
    }

    pub mod special_rpo_scheduling_phase {
        use super::*;
        pub struct SpecialRPOSchedulingPhase {}
        impl SpecialRPOSchedulingPhase {
            pub fn Run(data: *mut PipelineData) {}
        }
    }

    pub struct Pipeline {
        data_: *mut PipelineData,
    }

    impl Pipeline {
        pub fn new(data: *mut PipelineData) -> Self {
            Pipeline { data_ }
        }

        pub fn data(&self) -> *mut PipelineData {
            self.data_
        }

        pub fn begin_phase_kind(&self, phase_kind_name: &str) {
            unsafe {
                if let Some(statistics) = (*self.data_).pipeline_statistics() {
                    statistics.begin_phase_kind(phase_kind_name);
                }
            }
        }

        pub fn end_phase_kind(&self) {
            unsafe {
                if let Some(statistics) = (*self.data_).pipeline_statistics() {
                    statistics.end_phase_kind();
                }
            }
        }

        pub fn run<PhaseT: TurboshaftPhase>(&self, args: ()) -> Result<(), String> {
            let phase_name = PhaseT::phase_name();
            unsafe {
                let mut phase_scope = PhaseScope::new(
                    (*self.data_).pipeline_statistics(),
                    PhaseT::phase_name(),
                );
                let mut temp_zone = ZoneWithName::<PhaseT::kPhaseName>::new((*self.data_).zone_stats(), PhaseT::phase_name());
                let mut origin_scope = NodeOriginTable::PhaseScope::new(
                    (*self.data_).node_origins(),
                    PhaseT::phase_name(),
                );

                let mut phase = PhaseT::new();
                let result = phase.run(self.data_, &mut temp_zone, args);
                if result.is_ok() && PhaseT::produces_printable_graph() {
                    self.print_graph(&mut temp_zone, PhaseT::phase_name());
                }
                return result;
            }
        }

        fn print_graph(&self, zone: &mut ZoneWithName<&str>, phase_name: &str) {
            unsafe {
                let mut code_tracer: *mut CodeTracer = std::ptr::null_mut();
                if (*self.data_).info().trace_turbo_graph() {
                    code_tracer = (*self.data_).get_code_tracer();
                    assert!(!code_tracer.is_null());
                }

                PrintTurboshaftGraph(self.data_, zone.zone_mut(), code_tracer, phase_name);
            }
        }

        fn trace_sequence(&self, phase_name: &str) {
            unsafe {
                if self.info().trace_turbo_json() {
                    let scope = UnparkedScopeIfNeeded::new((*self.data_).broker());
                    let allow_deref = AllowHandleDereference {};
                    let mut json_of = TurboJsonFile::new(self.info(), std::ios_base::app);
                    let instruction_sequence_as_json =
                        InstructionSequenceAsJSON::new((*self.data_).sequence());
                    let register_allocation_data_as_json = RegisterAllocationDataAsJSON::new(
                        *(*self.data_).register_allocation_data(),
                        *(*self.data_).sequence(),
                    );

                    write!(
                        json_of,
                        "{{\"name\":\"{}\",\"type\":\"sequence\",\"blocks\":{},\"register_allocation\":{}}}",
                        phase_name, instruction_sequence_as_json, register_allocation_data_as_json
                    )
                    .expect("Failed to write to TurboJsonFile");
                }
                if self.info().trace_turbo_graph() {
                    let scope = UnparkedScopeIfNeeded::new((*self.data_).broker());
                    let allow_deref = AllowHandleDereference {};
                    let mut tracing_scope = CodeTracer::StreamScope::new((*self.data_).get_code_tracer());
                    unsafe {
                        write!((*tracing_scope.stream()), "----- Instruction sequence {} -----\n", phase_name).expect("").unwrap();
                        write!((*tracing_scope.stream()), "{}", *(*self.data_).sequence()).expect("").unwrap();
                    }
                }
            }
        }

        pub fn create_graph_with_maglev(&self, linkage: *mut Linkage) -> bool {
            unsafe {
                let unparked_scope = UnparkedScopeIfNeeded::new((*self.data_).broker());

                self.begin_phase_kind("V8.TFGraphCreation");
                let tracing_scope = Tracing::Scope::new(self.info());
                let bailout = self.run::<MaglevGraphBuildingPhase>((linkage));
                self.end_phase_kind();

                if let Err(_reason) = bailout {
                    (*self.data_).info().abort_optimization(BailoutReason::Generic);
                    return false;
                }

                return true;
            }
        }

        pub fn create_graph_from_turbofan(
            &self,
            turbofan_data: *mut compiler::TFPipelineData,
            linkage: *mut Linkage,
        ) -> bool {
            unsafe {
                assert!(!flags::FLAG_disable_optimizing_compilers || flags::FLAG_turboshaft);

                let scope = UnparkedScopeIfNeeded::new(
                    (*self.data_).broker(),
                    flags::FLAG_turboshaft_trace_reduction || flags::FLAG_turboshaft_trace_emitted,
                );

                let tracing_scope = Tracing::Scope::new(self.info());

                if let Err(_bailout) = self.run::<BuildGraphPhase>((turbofan_data, linkage)) {
                    self.info().abort_optimization(BailoutReason::Generic);
                    return false;
                }

                return true;
            }
        }

        pub fn optimize_turboshaft_graph(&self, _linkage: *mut Linkage) -> bool {
            unsafe {
                let scope = UnparkedScopeIfNeeded::new(
                    (*self.data_).broker(),
                    flags::FLAG_turboshaft_trace_reduction || flags::FLAG_turboshaft_trace_emitted,
                );

                let tracing_scope = Tracing::Scope::new(self.info());

                self.begin_phase_kind("V8.TurboshaftOptimize");

                if flags::FLAG_turboshaft_wasm_in_js_inlining {
                    self.run::<wasm_in_js_inlining_phase::WasmInJSInliningPhase>(());
                }

                self.run::<MachineLoweringPhase>(());

                if flags::FLAG_turboshaft_loop_unrolling {
                    self.run::<LoopUnrollingPhase>(());
                }

                if flags::FLAG_turbo_store_elimination {
                    self.run::<StoreStoreEliminationPhase>(());
                }

                self.run::<OptimizePhase>(());

                if flags::FLAG_turboshaft_typed_optimizations {
                    self.run::<TypedOptimizationsPhase>(());
                }

                if flags::FLAG_turboshaft_assert_types {
                    self.run::<TypeAssertionsPhase>(());
                }

                self.run::<CodeEliminationAndSimplificationPhase>(());

                if flags::FLAG_turboshaft_enable_debug_features {
                    self.run::<DebugFeatureLoweringPhase>(());
                }

                return true;
            }
        }

        pub fn run_simplification_and_normalization_phase(&self) {
            self.run::<SimplificationAndNormalizationPhase>(());
        }

        pub fn prepare_for_instruction_selection(&self, profile: *const ProfileDataFromFile) {
            unsafe {
                if (*self.data_).pipeline_kind() == TurboshaftPipelineKind::kCSA
                    || (*self.data_).pipeline_kind() == TurboshaftPipelineKind::kTSABuiltin
                {
                    if !profile.is_null() {
                        //self.run::<ProfileApplicationPhase>(profile);
                    }

                    if flags::FLAG_reorder_builtins
                        && Builtins::is_builtin_id(self.info().builtin())
                    {
                        let unparked_scope = UnparkedScopeIfNeeded::new((*self.data_).broker());
                        BasicBlockCallGraphProfiler::store_call_graph(
                            self.info(),
                            (*self.data_).graph(),
                        );
                    }

                    if flags::FLAG_turbo_profiling {
                        let unparked_scope = UnparkedScopeIfNeeded::new((*self.data_).broker());
                        let allow_handle_dereference = AllowHandleDereference {};

                        let block_count = (*self.data_).graph().block_count();
                        // let mut profiler_data = BasicBlockProfiler::get().new_data(block_count);
                        // profiler_data.set_function_name(self.info().get_debug_name());

                        // if flags::FLAG_turbo_profiling_verbose {
                        //     let mut os = std::ostringstream::new();
                        //     os << (*self.data_).graph();
                        //     // profiler_data.set_schedule(os.str());
                        // }
                        // (*self.data_).info().set_profiler_data(profiler_data);
                        // self.run::<BlockInstrumentationPhase>(());
                    } else {
                        let mut temp_zone =
                            ZoneWithName::<K_TEMP_ZONE_NAME>::new((*self.data_).zone_stats(), K_TEMP_ZONE_NAME);
                            copying_phase::CopyingPhase::<
                                LoadStoreSimplificationReducer,
                                InstructionSelectionNormalizationReducer,
                            >::Run(self.data_, temp_zone.zone_mut());
                    }
                }

                self.run::<DecompressionOptimizationPhase>(());
                self.run::<SpecialRPOSchedulingPhase>(());
            }
        }

        pub fn select_instructions(&self, linkage: *mut Linkage) -> Result<bool, String> {
            unsafe {
                let call_descriptor = (*linkage).get_incoming_descriptor();

                if (*self.data_).frame().is_null() {
                    (*self.data_).initialize_frame_data(call_descriptor);
                }

                let mut code_tracer: *mut CodeTracer = std::ptr::null_mut();
                if self.info().trace_turbo_graph() {
                    code_tracer = (*self.data_).get_code_tracer();
                }

                match self.run::<InstructionSelectionPhase>((call_descriptor, linkage, code_tracer)) {
                    Ok(_) => Ok(true),
                    Err(_bailout) => {
                        (*self.data_).info().abort_optimization(BailoutReason::Generic);
                        self.end_phase_kind();
                        Ok(false)
                    }
                }
            }
        }

        pub fn allocate_registers(&self, call_descriptor: *mut CallDescriptor) -> bool {
            unsafe {
                self.begin_phase_kind("V8.TFRegisterAllocation");

                let run_verifier = flags::FLAG_turbo_verify_allocation;
                let mut verifier_zone: Option<Zone> = None;
                let mut verifier: *mut RegisterAllocatorVerifier = std::ptr::null_mut();

                if run_verifier {
                    let allocator = (*self.data_).allocator();
                    assert!(!allocator.is_null());

                    verifier_zone = Some(Zone::new_for_compiler(
                        (*self.data_).allocator(),
                        "RegisterAllocatorVerifierZone",
                    ));
                    if let Some(zone) = verifier_zone.as_mut() {
                        verifier = zone.new::<RegisterAllocatorVerifier>((
                            zone,
                            RegisterConfiguration::default(),
                            (*self.data_).sequence(),
                            (*self.data_).frame(),
                        ));
                    }
                }
                (*self.data_).initialize_register_component(
                    RegisterConfiguration::default(),
                    call_descriptor,
                );
                self.run::<MeetRegisterConstraintsPhase>(());
                self.run::<ResolvePhisPhase>(());
                self.run::<BuildLiveRangesPhase>(());
                self.run::<BuildLiveRangeBundlesPhase>(());

                self.trace_sequence("before register allocation");
                
                if let Some(zone) = verifier_zone.as_ref() {
                    
                }

                if self.info().trace_turbo_json() && !self.may_have_unverifiable_graph() {
                    let tcf = TurboCfgFile::new(self.info().isolate());
                    //tcf << AsC1VRegisterAllocationData("PreAllocation", self.data_.register_allocation_data());
                }

                self.run::<AllocateGeneralRegistersPhase<LinearScanAllocator>>(());
                if (*(*self.data_).sequence()).has_fp_virtual_registers() {
                    self.run::<AllocateFPRegistersPhase<LinearScanAllocator>>(());
                }

                if (*(*self.data_).sequence()).has_simd128_virtual_registers()
                    && k_fp_aliasing() == AliasingKind::kIndependent
                {
                    self.run::<AllocateSimd128RegistersPhase<LinearScanAllocator>>(());
                }

                self.run::<DecideSpillingModePhase>(());
                self.run::<AssignSpillSlotsPhase>(());
                self.run::<CommitAssignmentPhase>(());

                if !verifier.is_null() {
                    //(*verifier).verify_assignment("Immediately after CommitAssignmentPhase.");
                }

                self.run::<ConnectRangesPhase>(());
                self.run::<ResolveControlFlowPhase>(());
                self.run::<PopulateReferenceMapsPhase>(());

                if flags::FLAG_turbo_move_optimization {
                    self.run::<OptimizeMovesPhase>(());
                }
                self.trace_sequence("after register allocation");

                if !verifier.is_null() {
                    //(*verifier).verify_assignment("End of regalloc pipeline.");
                    //(*verifier).verify_gap_moves();
                }

                if self.info().trace_turbo_json() && !self.may_have_unverifiable_graph() {
                    let tcf = TurboCfgFile::new(self.info().isolate());
                    //tcf << AsC1VRegisterAllocationData("CodeGen", self.data_.register_allocation_data());
                }
                (*self.data_).clear_register_component();
                self.end_phase_kind();

                return true;
            }
        }

        pub fn may_have_unverifiable_graph(&self) -> bool {
            true
        }

        fn verify_generated_code_is_idempotent(&self) {}

        fn allocate_registers_with_config(
            &self,
            config: *const RegisterConfiguration,
            call_descriptor: *mut CallDescriptor,
            run_verifier: bool,
        ) {
            unsafe {
                let mut verifier_zone: Option<Zone> = None;
                let mut verifier: *mut RegisterAllocatorVerifier = std::ptr::null_mut();

                if run_verifier {
                    let allocator = (*self.data_).allocator();
                    assert!(!allocator.is_null());
                    verifier_zone = Some(Zone::new_for_compiler(
                        (*self.data_).allocator(),
                        "RegisterAllocatorVerifierZone",
                    ));
                    if let Some(zone) = verifier_zone.as_mut() {
                        verifier = zone.new::<RegisterAllocatorVerifier>((
                            zone,
                            config,
                            (*self.data_).sequence(),
                            (*self.data_).frame(),
                        ));
                    }
                }
                (*self.data_).initialize_register_component(config, call_descriptor);
                self.run::<MeetRegisterConstraintsPhase>(());
                self.run::<ResolvePhisPhase>(());
                self.run::<BuildLiveRangesPhase>(());
                self.run::<BuildLiveRangeBundlesPhase>(());

                self.trace_sequence("before register allocation");
                
                if let Some(zone) = verifier_zone.as_ref() {
                }

                if self.info().trace_turbo_json() && !self.may_have_unverifiable_graph() {
                    let tcf = TurboCfgFile::new(self.info().isolate());
                    //tcf << AsC1VRegisterAllocationData("PreAllocation", self.data_.register_allocation_data());
                }

                self.run::<AllocateGeneralRegistersPhase<LinearScanAllocator>>(());

                if (*(*self.data_).sequence()).has_fp_virtual_registers() {
                    self.run::<AllocateFPRegistersPhase<LinearScanAllocator>>(());
                }

                if (*(*self.data_).sequence()).has_simd128_virtual_registers()
                    && k_fp_aliasing() == AliasingKind::kIndependent
                {
                    self.run::<AllocateSimd128RegistersPhase<LinearScanAllocator>>(());
                }

                self.run::<DecideSpillingModePhase>(());
                self.run::<AssignSpillSlotsPhase>(());
                self.run::<CommitAssignmentPhase>(());

                if !verifier.is_null() {
                    //(*verifier).verify_assignment("Immediately after CommitAssignmentPhase.");
                }

                self.run::<ConnectRangesPhase>(());
                self.run::<ResolveControlFlowPhase>(());
                self.run::<PopulateReferenceMapsPhase>(());

                if flags::FLAG_turbo_move_optimization {
                    self.run::<OptimizeMovesPhase>(());
                }

                self.trace_sequence("after register allocation");

                if !verifier.is_null() {
                    //(*verifier).verify_assignment("End of regalloc pipeline.");
                    //(*verifier).verify_gap_moves();
                }

                if self.info().trace_turbo_json() && !self.may_have_unverifiable_graph() {
                    let tcf = TurboCfgFile::new(self.info().isolate());
                    //tcf << AsC1VRegisterAllocationData("CodeGen", self.data_.register_allocation_data());
                }

                (*self.data_).clear_register_component();
            }
        }

        pub fn assemble_code(&self, linkage: *mut Linkage) {
            self.begin_phase_kind("V8.TFCodeGeneration");
            unsafe {
                (*self.data_).initialize_code_generator(linkage);

                let unparked_scope = UnparkedScopeIfNeeded::new((*self.data_).broker());

                self.run::<assemble_code_phase::AssembleCodePhase>(());
                if self.info().trace_turbo_json() {
                    let mut json_of = TurboJsonFile::new(self.info(), std::ios_base::app);
                    //json_of << "{\"name\":\"code generation\"" << ", \"type\":\"instructions\""
                    //      << InstructionStartsAsJSON{&data()->code_generator()->instr_starts()}
                    //      << TurbolizerCodeOffsetsInfoAsJSON{
                    //             &data()->code_generator()->offsets_info()};
                    //json_of << "},\n";
                }

                (*self.data_).clear_instruction_component();
                self.end_phase_kind();
            }
        }

        pub fn generate_code(&self, call_descriptor: *mut CallDescriptor) -> MaybeHandle<Code> {
            unsafe {
                let mut linkage = Linkage::new(call_descriptor);
                self.prepare_for_instruction_selection(std::ptr::null());
                if !self.select_instructions(&mut linkage).unwrap() {
                    return MaybeHandle::empty();
                }
                self.allocate_registers((*linkage).get_incoming_descriptor());
                self.assemble_code(&mut linkage);
                return self.finalize_code(true);
            }
        }

        pub fn generate_code_with_linkage(
            &self,
            linkage: *mut Linkage,
            osr_helper: std::shared::ptr::SharedPtr<OsrHelper>,
            jump_optimization_info: *mut JumpOptimizationInfo,
            profile: *const ProfileDataFromFile,
            initial_graph_hash: i32,
        ) -> bool {
            unsafe {
                (*self.data_).initialize_codegen_component(osr_helper, jump_optimization_info);
                self.prepare_for_instruction_selection(profile);
                if !self.select_instructions(linkage).unwrap() {
                    return false;
                }
                self.allocate_registers((*linkage).get_incoming_descriptor());
                self.assemble_code(linkage);

                if let Some(info) = (*self.data_).info().profiler_data() {
                    //(*info).set_hash(initial_graph_hash);
                }

                if !jump_optimization_info.is_null() && (*jump_optimization_info).is_optimizable() {
                    (*self.data_).clear_codegen_component();
                    (*jump_optimization_info).set_optimizing();
                    (*self.data_).initialize_codegen_component(osr_helper, jump_optimization_info);
                    if !self.select_instructions(linkage).unwrap() {
                        return false;
                    }
                    self.allocate_registers((*linkage).get_incoming_descriptor());
                    self.assemble_code(linkage);
                }
                return true;
            }
        }

        pub fn info(&self) -> *mut OptimizedCompilationInfo {
            self.data_unsafe().info()
        }

        fn data_unsafe(&self) -> &PipelineData {
            unsafe { &*self.data_ }
        }

        pub fn finalize_code(&self, retire_broker: bool) -> MaybeIndirectHandle<Code> {
            self.begin_phase_kind("V8.TFFinalizeCode");
            unsafe {
                if !(*self.data_).broker().is_null() && retire_broker {
                    //(*self.data_).broker().retire();
                }
                self.run::<finalize_code_phase::FinalizeCodePhase>(());

                let maybe_code = (*self.data_).code();
                let mut code: IndirectHandle<Code> = IndirectHandle::empty();
                if !maybe_code.to_handle(&mut code) {
                    return maybe_code;
                }
                (*self.data_).info().set_code(code);
                self.print_code((*self.data_).isolate(), code, (*self.data_).info());

                if self.info().trace_turbo_json() {
                    let mut json_of = TurboJsonFile::new(self.info(), std::ios_base::app);
                    json_of.write_all(b"{\"name\":\"disassembly\",\"type\":\"disassembly\"}")
                        .unwrap();

                    //json_of << BlockStartsAsJSON{&data_->code_generator()->block_starts()}
                    //      << "\"data\":\"";
                    // CodeTracer::StreamScope tracing_scope(data_->GetCodeTracer());
                    // tracing_scope.stream()
                    //     << "---------------------------------------------------\n"
                    //     << "Finished compiling method " << info()->GetDebugName().get()
                    //     << " using TurboFan" << std::endl;
                }
                self.end_phase_kind();
                return maybe_code;
            }
        }

        fn print_code(&self, isolate: *mut Isolate, code: IndirectHandle<Code>, info: *mut OptimizedCompilationInfo) {
            
        }
        pub fn commit_dependencies(&self, code: Handle<Code>) -> bool {
            unsafe {
                (*self.data_).depedencies().is_null() || (*(*self.data_).depedencies()).commit(code)
            }
        }
    }
    unsafe impl Send for Pipeline {}
    unsafe impl Sync for Pipeline {}

    pub trait TurboshaftPhase {
        const kPhaseName: &'static str;
        fn phase_name() -> &'static str;
        fn k_runtime_call_counter_id() -> i32;
        fn k_counter_mode() -> i32;
        fn produces_printable_graph() -> bool;
        fn new() -> Self;
        fn run(&mut self, data: *mut PipelineData, temp_zone: &mut ZoneWithName<&str>, args: ()) -> Result<(), String>;
    }

    pub struct PhaseScope {
        statistics: *mut PipelineStatistics,
        phase_kind_name: &'static str,
    }

    impl PhaseScope {
        pub fn new(statistics: *mut PipelineStatistics, phase_kind_name: &'static str) -> Self {
            unsafe {
                if !statistics.is_null() {
                    (*statistics).begin_phase_kind(phase_kind_name);
                }
            }
            PhaseScope {
                statistics,
                phase_kind_name,
            }
        }
    }

    impl Drop for PhaseScope {
        fn drop(&mut self) {
            unsafe {
                if !self.statistics.is_null() {
                    (*self.statistics).end_phase_kind();
                }
            }
        }
    }

    pub struct NodeOriginTable {
        _phantom: std::marker::PhantomData<()>,
    }

    impl NodeOriginTable {
        pub struct PhaseScope {
            node_origins: *mut NodeOriginTable,
            phase_name: &'static str,
        }

        impl PhaseScope {
            pub fn new(node_origins: *mut NodeOriginTable, phase_name: &'static str) -> Self {
                PhaseScope {
                    node_origins,
                    phase_name,
                }
            }
        }
    }

    pub struct ZoneWithName<const NAME: &'static str> {
        zone: Zone,
    }

    impl<const NAME: &'static str> ZoneWithName<NAME> {
        pub fn new(zone_stats: *mut ZoneStats, name: &'static str) -> Self {
            let accounting_allocator = unsafe { (*zone_stats).allocator() };
            ZoneWithName {
                zone: Zone::new_for_compiler(accounting_allocator, name),
            }
        }
        pub fn zone_mut(&mut self) -> &mut Zone{
            &mut self.zone
        }
        pub fn zone(&self) -> &Zone {
            &self.zone
        }
    }

    pub struct JumpOptimizationInfo {}

    impl JumpOptimizationInfo {
        pub fn is_optimizable(&self) -> bool {
            false
        }
        pub fn set_optimizing(&mut self) {}
        pub fn is_collecting(&self) -> bool {
            false
        }
    }

    pub mod compiler {
        pub struct TFPipelineData {}
    }

    pub mod wasm_in_js_inlining_phase {
        use super::*;
        pub struct WasmInJSInliningPhase {}
        impl WasmInJSInliningPhase {
            pub fn Run(_data: *mut PipelineData, _temp_zone: &mut ZoneWithName<&str>) {}
        }
    }

    pub struct OsrHelper {}

    pub struct Builtins {}

    impl Builtins {
        pub fn is_builtin_id(_id: i32) -> bool {
            false
        }
    }

    pub struct BasicBlockProfiler {}

    impl BasicBlockProfiler {
        pub fn get() -> &'static BasicBlockProfiler {
            static BASIC_BLOCK_PROFILER: BasicBlockProfiler = BasicBlockProfiler {};
            &BASIC_BLOCK_PROFILER
        }
        pub fn new_data(&self, _block_count: usize) -> *mut BasicBlockProfilerData {
            std::ptr::null_mut()
        }
    }

    pub struct BasicBlockProfilerData {}

    impl BasicBlockProfilerData {
        pub fn set_function_name(&mut self, _debug_name: Handle<String>) {}
        pub fn set_schedule(&mut self, _os: std::string::String) {}
        pub fn set_hash(&self, _initial_graph_hash: i32) {}
    }

    pub enum BailoutReason {
        Generic,
    }

    pub enum TurboshaftPipelineKind {
        kCSA,
        kTSABuiltin,
    }

    pub struct AliasingKind {}

    impl AliasingKind {
        const kIndependent: i32 = 0;
    }

    pub fn k_fp_aliasing() -> i32 {
        0
    }

    pub struct InstructionSequenceAsJSON<'a> {
        sequence: *mut InstructionSequence,
        _marker
