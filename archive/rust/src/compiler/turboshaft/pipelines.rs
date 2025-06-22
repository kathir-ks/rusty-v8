// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod pipelines {
    use std::marker::PhantomData;

    // Placeholder for crates corresponding to C++ includes.
    // Replace with actual crate imports.
    // For example:
    // use some_crate::SomeType;
    // use another_crate::*;

    // Placeholder types. Replace with actual Rust equivalents.
    pub type OptimizedCompilationInfo = usize; // Replace with actual type
    pub type PipelineStatistics = usize; // Replace with actual type
    pub type BailoutReason = usize; // Replace with actual type
    pub type TFPipelineData = usize; // Replace with actual type
    pub type Linkage = usize; // Replace with actual type
    pub type Code = usize; // Replace with actual type
    pub type CallDescriptor = usize; // Replace with actual type
    pub type RegisterConfiguration = usize; // Replace with actual type
    pub type JumpOptimizationInfo = usize; // Replace with actual type
    pub type InstructionSequence = usize; // Replace with actual type
    pub type OptimizedCompilationInfoHandle = usize; // Replace with actual type
    pub type TurboJsonFile = usize; // Replace with actual type
    pub type OsrHelper = usize; // Replace with actual type
    pub type ProfileDataFromFile = usize; // Replace with actual type
    pub type FrameData = usize; // Replace with actual type
    pub type TurboshaftGraph = usize; // Replace with actual type
    pub type Isolate = usize; // Replace with actual type
    pub type SharedInfo = usize; // Replace with actual type
    pub type Dependencies = usize; // Replace with actual type
    pub type CodeGenerator = usize; // Replace with actual type

    pub const K_TEMP_ZONE_NAME: &str = "temp-zone";

    pub struct SimplificationAndNormalizationPhase;

    impl SimplificationAndNormalizationPhase {
        //DECL_TURBOSHAFT_PHASE_CONSTANTS(SimplificationAndNormalization) // Macro

        pub fn run(data: &mut PipelineData, temp_zone: &mut Zone) {
            todo!()
        }
    }

    pub struct PipelineData {
        // Replace with actual fields.
        info: OptimizedCompilationInfo,
        pipeline_statistics: Option<PipelineStatistics>,
        broker: usize, // Replace with actual type
        zone_stats: usize, // Replace with actual type
        node_origins: usize, // Replace with actual type
        runtime_call_stats: usize, // Replace with actual type
        sequence: usize, // Replace with actual type
        register_allocation_data: usize, // Replace with actual type
        frame: Option<FrameData>,
        jump_optimization_info: Option<JumpOptimizationInfo>,
        graph: TurboshaftGraph,
        code: Option<Code>,
        depedencies: Option<Dependencies>,
        isolate: Isolate, // Replace with actual type
        code_generator: Option<CodeGenerator>,
        source_position_output: String,
        pipeline_kind: TurboshaftPipelineKind,
    }

    impl PipelineData {
        pub fn info(&self) -> OptimizedCompilationInfo {
            self.info
        }

        pub fn pipeline_statistics(&self) -> Option<PipelineStatistics> {
            self.pipeline_statistics
        }

        pub fn broker(&self) -> usize {
            self.broker
        }

        pub fn sequence(&self) -> usize {
            self.sequence
        }
        pub fn register_allocation_data(&self) -> usize {
            self.register_allocation_data
        }

        pub fn frame(&self) -> Option<&FrameData> {
            self.frame.as_ref()
        }
        pub fn initialize_frame_data(&mut self, call_descriptor: CallDescriptor){
            todo!()
        }

        pub fn jump_optimization_info(&self) -> Option<&JumpOptimizationInfo>{
            self.jump_optimization_info.as_ref()
        }

        pub fn graph(&self) -> &TurboshaftGraph {
            &self.graph
        }
        pub fn code(&self) -> Option<Code> {
            self.code
        }
        pub fn code_generator(&self) -> Option<&CodeGenerator> {
            self.code_generator.as_ref()
        }
        pub fn clear_instruction_component(&mut self) {
            todo!()
        }
        pub fn isolate(&self) -> Isolate {
            self.isolate
        }
        pub fn depedencies(&self) -> Option<&Dependencies> {
            self.depedencies.as_ref()
        }
        pub fn pipeline_kind(&self) -> TurboshaftPipelineKind {
            self.pipeline_kind
        }

        pub fn set_source_position_output(&mut self, output: String) {
            self.source_position_output = output;
        }
        pub fn source_position_output(&self) -> String {
            self.source_position_output.clone()
        }

        pub fn get_code_tracer(&self) -> usize { // Replace with actual type
            todo!()
        }

    }

    pub struct Pipeline<'a> {
        data_: &'a mut PipelineData,
    }

    impl<'a> Pipeline<'a> {
        pub fn new(data: &'a mut PipelineData) -> Self {
            Pipeline { data_: data }
        }

        pub fn data(&self) -> &PipelineData {
            self.data_
        }

        pub fn begin_phase_kind(&mut self, phase_kind_name: &str) {
            if let Some(statistics) = self.data_.pipeline_statistics() {
                //statistics.begin_phase_kind(phase_kind_name);
                todo!()
            }
        }

        pub fn end_phase_kind(&mut self) {
            if let Some(statistics) = self.data_.pipeline_statistics() {
                //statistics.end_phase_kind();
                todo!()
            }
        }

        pub fn run<Phase: TurboshaftPhase>(&mut self, args: Vec<usize>) -> Result<(), BailoutReason>
        where
            Phase: Default,
        {
            // Setup run scope.
            //PhaseScope phase_scope(self.data_.pipeline_statistics(), Phase::phase_name());
            //ZoneWithName<Phase::kPhaseName> temp_zone(self.data_.zone_stats(), Phase::phase_name());
            //NodeOriginTable::PhaseScope origin_scope(self.data_.node_origins(), Phase::phase_name());
            //#ifdef V8_RUNTIME_CALL_STATS
            //RuntimeCallTimerScope runtime_call_timer_scope(self.data_.runtime_call_stats(), Phase::kRuntimeCallCounterId, Phase::kCounterMode);
            //#endif

            let mut phase: Phase = Phase::default();
            //using result_t =
            //    decltype(phase.Run(self.data_, temp_zone, std::forward<Args>(args)...));
            //if constexpr (std::is_same_v<result_t, void>) {

            //TODO: Proper args handling.
            let temp_zone = &mut Zone::default();
            phase.run(self.data_, temp_zone)?;
            if Phase::produces_printable_graph() {
                self.print_graph(temp_zone, Phase::phase_name());
            }

            Ok(())

            // } else {
            //   let result = phase.Run(self.data_, temp_zone, std::forward<Args>(args)...);
            //   if constexpr (produces_printable_graph<Phase>::value) {
            //     PrintGraph(temp_zone, Phase::phase_name());
            //   }
            //   return result;
            // }
            //UNREACHABLE();
        }

        pub fn print_graph(&mut self, zone: &mut Zone, phase_name: &str) {
            let mut code_tracer: Option<usize> = None; // Replace with actual type
            if true { //if self.data_.info().trace_turbo_graph() {
                // NOTE: We must not call `GetCodeTracer` if tracing is not enabled,
                // because it may not yet be initialized then and doing so from the
                // background thread is not threadsafe.
                //code_tracer = Some(self.data_.get_code_tracer());
                //DCHECK_NOT_NULL(code_tracer);
                todo!()
            }
            //PrintTurboshaftGraph(self.data_, zone, code_tracer, phase_name);
            todo!()
        }

        pub fn trace_sequence(&mut self, phase_name: &str) {
            //if self.info().trace_turbo_json() {
            //  UnparkedScopeIfNeeded scope(self.data().broker());
            //  AllowHandleDereference allow_deref;
            //  TurboJsonFile json_of(self.info(), std::ios_base::app);
            //  json_of
            //      << "{\"name\":\"" << phase_name << "\",\"type\":\"sequence\""
            //      << ",\"blocks\":" << InstructionSequenceAsJSON{self.data().sequence()}
            //      << ",\"register_allocation\":{"
            //      << RegisterAllocationDataAsJSON{*(self.data().register_allocation_data()),
            //                                      *(self.data().sequence())}
            //      << "}},\n";
            //}
            //if self.info().trace_turbo_graph() {
            //  UnparkedScopeIfNeeded scope(self.data().broker());
            //  AllowHandleDereference allow_deref;
            //  CodeTracer::StreamScope tracing_scope(self.data().GetCodeTracer());
            //  tracing_scope.stream()
            //      << "----- Instruction sequence " << phase_name << " -----\n"
            //      << *self.data().sequence();
            //}
            todo!()
        }

        pub fn create_graph_with_maglev(&mut self, linkage: &mut Linkage) -> Result<bool, BailoutReason> {
            //UnparkedScopeIfNeeded unparked_scope(self.data_.broker());

            self.begin_phase_kind("V8.TFGraphCreation");
            //turboshaft::Tracing::Scope tracing_scope(self.info());
            //TODO: Handle this.
            //let bailout: Option<BailoutReason> = self.run::<turboshaft::MaglevGraphBuildingPhase>(linkage);
            self.end_phase_kind();

            //if bailout.is_some() {
            //    self.data_.info().AbortOptimization(bailout.unwrap());
            //    return Ok(false);
            //}

            Ok(true)
        }

        pub fn create_graph_from_turbofan(&mut self, turbofan_data: &mut TFPipelineData, linkage: &mut Linkage) -> Result<bool, BailoutReason> {
            //CHECK_IMPLIES(!v8_flags.disable_optimizing_compilers, v8_flags.turboshaft);

            //UnparkedScopeIfNeeded scope(self.data_.broker(),
            //                            v8_flags.turboshaft_trace_reduction ||
            //                                v8_flags.turboshaft_trace_emitted);

            //turboshaft::Tracing::Scope tracing_scope(self.info());

            //if let Some(bailout) = self.run::<turboshaft::BuildGraphPhase>(turbofan_data, linkage) {
            //    self.info().AbortOptimization(bailout);
            //    return Ok(false);
            //}

            Ok(true)
        }

        pub fn optimize_turboshaft_graph(&mut self, linkage: &mut Linkage) -> bool {
            //UnparkedScopeIfNeeded scope(self.data_.broker(),
            //                            v8_flags.turboshaft_trace_reduction ||
            //                                v8_flags.turboshaft_trace_emitted);

            //turboshaft::Tracing::Scope tracing_scope(self.info());

            self.begin_phase_kind("V8.TurboshaftOptimize");

            //#ifdef V8_ENABLE_WEBASSEMBLY
            //// TODO(dlehmann,353475584): Once the Wasm-in-JS TS inlining MVP is feature-
            //// complete and cleaned-up, move its reducer into the beginning of the
            //// `MachineLoweringPhase` since we can reuse the `DataViewLoweringReducer`
            //// there and avoid a separate phase.
            //if (v8_flags.turboshaft_wasm_in_js_inlining) {
            //    self.run::<turboshaft::WasmInJSInliningPhase>();
            //}
            //#endif  // !V8_ENABLE_WEBASSEMBLY

            //self.run::<turboshaft::MachineLoweringPhase>();

            //if (v8_flags.turboshaft_loop_unrolling) {
            //    self.run::<turboshaft::LoopUnrollingPhase>();
            //}

            //if (v8_flags.turbo_store_elimination) {
            //    self.run::<turboshaft::StoreStoreEliminationPhase>();
            //}

            //self.run::<turboshaft::OptimizePhase>();

            //if (v8_flags.turboshaft_typed_optimizations) {
            //    self.run::<turboshaft::TypedOptimizationsPhase>();
            //}

            //if (v8_flags.turboshaft_assert_types) {
            //    self.run::<turboshaft::TypeAssertionsPhase>();
            //}

            //// Perform dead code elimination, reduce stack checks, simplify loads on
            //// platforms where required, ...
            //self.run::<turboshaft::CodeEliminationAndSimplificationPhase>();

            //#ifdef V8_ENABLE_DEBUG_CODE
            //if (V8_UNLIKELY(v8_flags.turboshaft_enable_debug_features)) {
            //    // This phase has to run very late to allow all previous phases to use
            //    // debug features.
            //    self.run::<turboshaft::DebugFeatureLoweringPhase>();
            //}
            //#endif  // V8_ENABLE_DEBUG_CODE
            todo!()
            //true
        }

        pub fn run_simplification_and_normalization_phase(&mut self) {
            let _ = self.run::<SimplificationAndNormalizationPhase>(Vec::new());
        }

        pub fn prepare_for_instruction_selection(&mut self, profile: Option<&ProfileDataFromFile>) {
            //if (V8_UNLIKELY(self.data().pipeline_kind() == TurboshaftPipelineKind::kCSA ||
            //                self.data().pipeline_kind() ==
            //                    TurboshaftPipelineKind::kTSABuiltin)) {
            //    if (profile) {
            //        self.run::<ProfileApplicationPhase>(profile);
            //    }

            //    if (v8_flags.reorder_builtins &&
            //        Builtins::IsBuiltinId(self.info().builtin())) {
            //        UnparkedScopeIfNeeded unparked_scope(self.data().broker());
            //        BasicBlockCallGraphProfiler::StoreCallGraph(self.info(), self.data().graph());
            //    }

            //    if (v8_flags.turbo_profiling) {
            //        UnparkedScopeIfNeeded unparked_scope(self.data().broker());

            //        // Basic block profiling disables concurrent compilation, so handle
            //        // deref is fine.
            //        AllowHandleDereference allow_handle_dereference;
            //        const size_t block_count = self.data().graph().block_count();
            //        BasicBlockProfilerData* profiler_data =
            //            BasicBlockProfiler::Get()->NewData(block_count);

            //        // Set the function name.
            //        profiler_data->SetFunctionName(self.info().GetDebugName());
            //        // Capture the schedule string before instrumentation.
            //        if (v8_flags.turbo_profiling_verbose) {
            //            std::ostringstream os;
            //            os << self.data().graph();
            //            profiler_data->SetSchedule(os);
            //        }

            //        self.info().set_profiler_data(profiler_data);

            //        self.run::<BlockInstrumentationPhase>();
            //    } else {
            //        // We run an empty copying phase to make sure that we have the same
            //        // control flow as when taking the profile.
            //        ZoneWithName<kTempZoneName> temp_zone(self.data().zone_stats(),
            //                                              kTempZoneName);
            //        CopyingPhase<>::Run(self.data(), temp_zone);
            //    }
            //}

            //// DecompressionOptimization has to run as the last phase because it
            //// constructs an (slightly) invalid graph that mixes Tagged and Compressed
            //// representations.
            //self.run::<DecompressionOptimizationPhase>();

            //self.run::<SpecialRPOSchedulingPhase>();
            todo!()
        }

        pub fn select_instructions(&mut self, linkage: &mut Linkage) -> Result<bool, BailoutReason> {
            let call_descriptor = linkage.get_incoming_descriptor();

            // Depending on which code path led us to this function, the frame may or
            // may not have been initialized. If it hasn't yet, initialize it now.
            if self.data_.frame().is_none() {
                self.data_.initialize_frame_data(call_descriptor);
            }

            // Select and schedule instructions covering the scheduled graph.
            let mut code_tracer: Option<usize> = None; // Replace with actual type
            //if self.info().trace_turbo_graph() {
            //    // NOTE: We must not call `GetCodeTracer` if tracing is not enabled,
            //    // because it may not yet be initialized then and doing so from the
            //    // background thread is not threadsafe.
            //    code_tracer = Some(self.data_.GetCodeTracer());
            //}

            //if let Some(bailout) = self.run::<InstructionSelectionPhase>(
            //    call_descriptor, linkage, code_tracer) {
            //    self.data_.info().AbortOptimization(bailout);
            //    self.end_phase_kind();
            //    return Ok(false);
            //}

            Ok(true)

            // TODO(nicohartmann@): We might need to provide this.
            // if (self.info().trace_turbo_json()) {
            //   UnparkedScopeIfNeeded scope(turbofan_data.broker());
            //   AllowHandleDereference allow_deref;
            //   TurboCfgFile tcf(isolate());
            //   tcf << AsC1V("CodeGen", turbofan_data.schedule(),
            //                turbofan_data.source_positions(),
            //                turbofan_data.sequence());

            //   std::ostringstream source_position_output;
            //   // Output source position information before the graph is deleted.
            //   if (self.data_.source_positions() != nullptr) {
            //     self.data_.source_positions().PrintJson(source_position_output);
            //   } else {
            //     source_position_output << "{}";
            //   }
            //   source_position_output << ",\n\"nodeOrigins\" : ";
            //   self.data_.node_origins().PrintJson(source_position_output);
            //   self.data_.set_source_position_output(source_position_output.str());
            // }
        }

        pub fn allocate_registers(&mut self, call_descriptor: &mut CallDescriptor) -> bool {
            self.begin_phase_kind("V8.TFRegisterAllocation");

            //let run_verifier = v8_flags.turbo_verify_allocation;

            //// Allocate registers.
            let config = RegisterConfiguration::default();
            //std::unique_ptr<const RegisterConfiguration> restricted_config;
            //if (call_descriptor.HasRestrictedAllocatableRegisters()) {
            //  RegList registers = call_descriptor.AllocatableRegisters();
            //  DCHECK_LT(0, registers.Count());
            //  restricted_config.reset(
            //      RegisterConfiguration::RestrictGeneralRegisters(registers));
            //  config = restricted_config.get();
            //}
            self.allocate_registers_impl(&config, call_descriptor, false);

            //// Verify the instruction sequence has the same hash in two stages.
            self.verify_generated_code_is_idempotent();

            //self.run::<FrameElisionPhase>();

            //// TODO(mtrofin): move this off to the register allocator.
            let generate_frame_at_start = false;
            //    self.data_.sequence().instruction_blocks().front().must_construct_frame();
            //// Optimimize jumps.
            //if (v8_flags.turbo_jt) {
            //  self.run::<JumpThreadingPhase>(generate_frame_at_start);
            //}

            self.end_phase_kind();

            true
        }

        pub fn may_have_unverifiable_graph(&self) -> bool {
            // TODO(nicohartmann): Are there any graph which are still verifiable?
            true
        }

        pub fn verify_generated_code_is_idempotent(&mut self) {
            let jump_opt = match self.data_.jump_optimization_info() {
                Some(info) => info,
                None => return,
            };

            //let code = self.data_.sequence();
            //let instruction_blocks = code.InstructionBlockCount();
            //let virtual_registers = code.VirtualRegisterCount();
            //let mut hash_code =
            //    base::hash_combine(instruction_blocks, virtual_registers);
            //for Instruction* instr : *code {
            //    hash_code = base::hash_combine(hash_code, instr.opcode(),
            //                                 instr.InputCount(), instr.OutputCount());
            //}
            //for (int i = 0; i < virtual_registers; i++) {
            //    hash_code = base::hash_combine(hash_code, code.GetRepresentation(i));
            //}
            //if jump_opt.is_collecting() {
            //    jump_opt.hash_code = hash_code;
            //} else {
            //    CHECK_EQ(hash_code, jump_opt.hash_code);
            //}
            todo!()
        }

        fn allocate_registers_impl(&mut self, config: &RegisterConfiguration, call_descriptor: &mut CallDescriptor, run_verifier: bool) {
            todo!()
        }

        pub fn assemble_code(&mut self, linkage: &mut Linkage) {
            self.begin_phase_kind("V8.TFCodeGeneration");
            self.data_.initialize_code_generator(linkage);

            //UnparkedScopeIfNeeded unparked_scope(self.data_.broker());

            //self.run::<AssembleCodePhase>();
            //if self.info().trace_turbo_json() {
            //    TurboJsonFile json_of(self.info(), std::ios_base::app);
            //    json_of
            //        << "{\"name\":\"code generation\"" << ", \"type\":\"instructions\""
            //        << InstructionStartsAsJSON{&self.data_.code_generator().instr_starts()}
            //        << TurbolizerCodeOffsetsInfoAsJSON{
            //               &self.data_.code_generator().offsets_info()};
            //    json_of << "},\n";
            //}

            self.data_.clear_instruction_component();
            self.end_phase_kind();
            todo!()
        }

        pub fn generate_code(&mut self, call_descriptor: &mut CallDescriptor) -> Result<Option<Code>, BailoutReason> {
            let mut linkage = Linkage::default(); //Linkage(call_descriptor);
            self.prepare_for_instruction_selection(None);
            if !self.select_instructions(&mut linkage)? {
                return Ok(None);
            }
            self.allocate_registers(call_descriptor);
            self.assemble_code(&mut linkage);
            Ok(Some(self.finalize_code(true)?))
        }

        pub fn generate_code_extended(
            &mut self,
            linkage: &mut Linkage,
            osr_helper: Option<&OsrHelper>,
            jump_optimization_info: Option<&JumpOptimizationInfo>,
            profile: Option<&ProfileDataFromFile>,
            initial_graph_hash: i32,
        ) -> bool {
            todo!()
        }

        pub fn info(&self) -> &OptimizedCompilationInfo {
            &self.data_.info
        }

        pub fn finalize_code(&mut self, retire_broker: bool) -> Result<Code, BailoutReason> {
            self.begin_phase_kind("V8.TFFinalizeCode");
            //if self.data_.broker() && retire_broker {
            //    self.data_.broker().Retire();
            //}
            //self.run::<FinalizeCodePhase>();

            //let maybe_code = self.data_.code();
            //let code = match maybe_code {
            //    Some(code) => code,
            //    None => return maybe_code,
            //};

            //self.data_.info().SetCode(code);
            //PrintCode(self.data_.isolate(), code, self.data_.info());

            //// Functions with many inline candidates are sensitive to correct call
            //// frequency feedback and should therefore not be tiered up early.
            //if (v8_flags.profile_guided_optimization &&
            //    self.info().could_not_inline_all_candidates() &&
            //    self.info().shared_info().cached_tiering_decision() !=
            //        CachedTieringDecision::kDelayMaglev) {
            //    self.info().shared_info().set_cached_tiering_decision(
            //        CachedTieringDecision::kNormal);
            //}

            //if self.info().trace_turbo_json() {
            //    TurboJsonFile json_of(self.info(), std::ios_base::app);

            //    json_of << "{\"name\":\"disassembly\",\"type\":\"disassembly\""
            //        << BlockStartsAsJSON{&self.data_.code_generator().block_starts()}
            //        << TurbolizerCodeOffsetsInfoAsJSON{
            //               &self.data_.code_generator().offsets_info()};
            //    json_of << "\"data\":\"";
            //#ifdef ENABLE_DISASSEMBLER
            //    std::stringstream disassembly_stream;
            //    code.Disassemble(nullptr, disassembly_stream, self.data_.isolate());
            //    std::string disassembly_string(disassembly_stream.str());
            //    for (const auto& c : disassembly_string) {
            //        json_of << AsEscapedUC16ForJSON(c);
            //    }
            //#endif  // ENABLE_DISASSEMBLER
            //    json_of << "\"}\n],\n";
            //    json_of << "\"nodePositions\":";
            //    // TODO(nicohartmann): We should try to always provide source positions.
            //    json_of << (self.data_.source_position_output().empty()
            //                  ? "{}"
            //                  : self.data_.source_position_output())
            //        << ",\n";
            //    JsonPrintAllSourceWithPositions(json_of, self.data_.info(), self.data_.isolate());
            //    if self.info().has_bytecode_array() {
            //        json_of << ",\n";
            //        JsonPrintAllBytecodeSources(json_of, self.info());
            //    }
            //    json_of << "\n}";
            //}
            //if self.info().trace_turbo_json() || self.info().trace_turbo_graph() {
            //    CodeTracer::StreamScope tracing_scope(self.data_.GetCodeTracer());
            //    tracing_scope.stream()
            //        << "---------------------------------------------------\n"
            //        << "Finished compiling method " << self.info().GetDebugName().get()
            //        << " using TurboFan" << std::endl;
            //}
            self.end_phase_kind();
            //code
            todo!()
        }

        pub fn commit_dependencies(&self, code: Code) -> bool {
            true //self.data_.depedencies().is_none() ||
                 //  self.data_.depedencies().Commit(code);
        }

        // #[cfg(debug_assertions)]
        // fn is_builtin_pipeline(&self) -> bool {
        //     false
        // }
    }

    pub trait TurboshaftPhase : Default{
        const PHASE_NAME: &'static str;
        fn phase_name() -> &'static str {
            Self::PHASE_NAME
        }
        fn produces_printable_graph() -> bool {
            false
        }
        fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) -> Result<(), BailoutReason>;
    }

    impl TurboshaftPhase for SimplificationAndNormalizationPhase{
        const PHASE_NAME: &'static str = "SimplificationAndNormalization";

        fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) -> Result<(), BailoutReason> {
            SimplificationAndNormalizationPhase::run(data, temp_zone);
            Ok(())
        }
    }

    pub struct Zone {

    }

    impl Default for Zone {
        fn default() -> Self {
            Self {}
        }
    }

    pub struct MaglevGraphBuildingPhase;

    impl Default for MaglevGraphBuildingPhase {
        fn default() -> Self {
            Self {}
        }
    }

    impl TurboshaftPhase for MaglevGraphBuildingPhase {
        const PHASE_NAME: &'static str = "MaglevGraphBuildingPhase";
        fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) -> Result<(), BailoutReason> {
            todo!()
        }
    }

    pub struct FinalizeCodePhase;
    impl Default for FinalizeCodePhase {
        fn default() -> Self {
            Self {}
        }
    }

    impl TurboshaftPhase for FinalizeCodePhase {
        const PHASE_NAME: &'static str = "FinalizeCodePhase";
        fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) -> Result<(), BailoutReason> {
            todo!()
        }
    }
    #[derive(PartialEq, Eq)]
    pub enum TurboshaftPipelineKind {
        kNormal,
        kCSA,
        kTSABuiltin,
    }
    pub struct BuildGraphPhase;

    impl Default for BuildGraphPhase {
        fn default() -> Self {
            Self {}
        }
    }

    impl TurboshaftPhase for BuildGraphPhase {
        const PHASE_NAME: &'static str = "BuildGraphPhase";
        fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) -> Result<(), BailoutReason> {
            todo!()
        }
    }

    pub struct WasmInJSInliningPhase;
    impl Default for WasmInJSInliningPhase {
        fn default() -> Self {
            Self {}
        }
    }

    impl TurboshaftPhase for WasmInJSInliningPhase {
        const PHASE_NAME: &'static str = "WasmInJSInliningPhase";
        fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) -> Result<(), BailoutReason> {
            todo!()
        }
    }
    pub struct MachineLoweringPhase;
    impl Default for MachineLoweringPhase {
        fn default() -> Self {
            Self {}
        }
    }

    impl TurboshaftPhase for MachineLoweringPhase {
        const PHASE_NAME: &'static str = "MachineLoweringPhase";
        fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) -> Result<(), BailoutReason> {
            todo!()
        }
    }

    pub struct LoopUnrollingPhase;
    impl Default for LoopUnrollingPhase {
        fn default() -> Self {
            Self {}
        }
    }

    impl TurboshaftPhase for LoopUnrollingPhase {
        const PHASE_NAME: &'static str = "LoopUnrollingPhase";
        fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) -> Result<(), BailoutReason> {
            todo!()
        }
    }

    pub struct StoreStoreEliminationPhase;
    impl Default for StoreStoreEliminationPhase {
        fn default() -> Self {
            Self {}
        }
    }

    impl TurboshaftPhase for StoreStoreEliminationPhase {
        const PHASE_NAME: &'static str = "StoreStoreEliminationPhase";
        fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) -> Result<(), BailoutReason> {
            todo!()
        }
    }
    pub struct OptimizePhase;
    impl Default for OptimizePhase {
        fn default() -> Self {
            Self {}
        }
    }

    impl TurboshaftPhase for OptimizePhase {
        const PHASE_NAME: &'static str = "OptimizePhase";
        fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) -> Result<(), BailoutReason> {
            todo!()
        }
    }
    pub struct TypedOptimizationsPhase;
    impl Default for TypedOptimizationsPhase {
        fn default() -> Self {
            Self {}
        }
    }

    impl TurboshaftPhase for TypedOptimizationsPhase {
        const PHASE_NAME: &'static str = "TypedOptimizationsPhase";
        fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) -> Result<(), BailoutReason> {
            todo!()
        }
    }
    pub struct TypeAssertionsPhase;
    impl Default for TypeAssertionsPhase {
        fn default() -> Self {
            Self {}
        }
    }

    impl TurboshaftPhase for TypeAssertionsPhase {
        const PHASE_NAME: &'static str = "TypeAssertionsPhase";
        fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) -> Result<(), BailoutReason> {
            todo!()
        }
    }
    pub struct CodeEliminationAndSimplificationPhase;
    impl Default for CodeEliminationAndSimplificationPhase {
        fn default() -> Self {
            Self {}
        }
    }

    impl TurboshaftPhase for CodeEliminationAndSimplificationPhase {
        const PHASE_NAME: &'