// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod phase {
    use std::any::Any;
    use std::borrow::Cow;
    use std::cell::{RefCell, UnsafeCell};
    use std::fmt;
    use std::marker::PhantomData;
    use std::mem;
    use std::ops::{Deref, DerefMut};
    use std::rc::Rc;
    use std::sync::Arc;

    //use crate::base::contextual::Contextual; // Assuming a Rust equivalent
    //use crate::base::template_meta_programming::functional; // Assuming a Rust equivalent
    //use crate::codegen::assembler::AssemblerOptions; // Assuming a Rust equivalent
    //use crate::codegen::optimized_compilation_info::OptimizedCompilationInfo; // Assuming a Rust equivalent
    //use crate::common::globals::kNoSourcePosition; // Assuming a Rust equivalent
    //use crate::compiler::access_info::AccessInfo; // Assuming a Rust equivalent
    //use crate::compiler::backend::instruction::InstructionSequence; // Assuming a Rust equivalent
    //use crate::compiler::compilation_dependencies::CompilationDependencies; // Assuming a Rust equivalent
    //use crate::compiler::compiler_source_position_table::SourcePositionTable; // Assuming a Rust equivalent
    //use crate::compiler::node_origin_table::NodeOriginTable; // Assuming a Rust equivalent
    //use crate::compiler::osr::OsrHelper; // Assuming a Rust equivalent
    //use crate::compiler::phase::PhaseKind; // Assuming a Rust equivalent
    //use crate::compiler::turboshaft::builtin_compiler::BytecodeHandlerData; // Assuming a Rust equivalent
    //use crate::compiler::turboshaft::graph::Graph; // Assuming a Rust equivalent
    //use crate::compiler::turboshaft::sidetable::SideTable; // Assuming a Rust equivalent
    //use crate::compiler::turboshaft::zone_with_name::ZoneWithName; // Assuming a Rust equivalent
    //use crate::logging::runtime_call_stats::RuntimeCallStats; // Assuming a Rust equivalent
    //use crate::zone::accounting_allocator::AccountingAllocator; // Assuming a Rust equivalent
    //use crate::zone::zone::Zone; // Assuming a Rust equivalent
    //use crate::wasm; // Assuming a Rust equivalent

    // Placeholder enums
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum PhaseKind {
        kTurboshaft,
        kTurbofan,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct AssemblerOptions {
        pub is_wasm: bool,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct BytecodeHandlerData {}

    pub type MaybeIndirectHandle<T> = *mut T;

    pub struct ZoneStats {}

    pub struct CompilationDependencies {}

    pub struct OptimizedCompilationInfo {}

    impl OptimizedCompilationInfo {
        pub fn GetDebugName(&self) -> std::unique_ptr<char[]> {
            std::unique_ptr::default()
        }
        pub fn trace_turbo_json(&self) -> bool {
            false
        }
        pub fn code_kind(&self) -> i32 {
            0
        }
        pub fn IsWasm(&self) -> bool {
            false
        }
        pub fn IsWasmBuiltin(&self) -> bool {
            false
        }
        pub fn builtin(&self) -> i32 {
            0
        }
    }

    // Placeholder for StringLiteral used in C++ templates
    #[derive(Debug, Copy, Clone)]
    pub struct StringLiteral(pub &'static str);

    // Placeholder for ZoneName
    pub const ZONE_NAME: &str = "PlaceholderZoneName";
    pub const kCompilationZoneName: StringLiteral = StringLiteral("CompilationZone");
    pub const kGraphZoneName: StringLiteral = StringLiteral("GraphZone");
    pub const kCodegenZoneName: StringLiteral = StringLiteral("CodegenZone");
    pub const kInstructionZoneName: StringLiteral = StringLiteral("InstructionZone");
    pub const kRegisterAllocationZoneName: StringLiteral =
        StringLiteral("RegisterAllocationZone");

    // Placeholder structs

    pub struct SourcePositionTable {}
    pub struct NodeOriginTable {}
    pub struct InstructionSequence {}
    pub struct Frame {}
    pub struct CodeGenerator {}
    pub struct CompilationDependency {}
    pub struct OsrHelper {}
    pub struct JumpOptimizationInfo {}
    pub struct Linkage {}
    pub struct RegisterConfiguration {}
    pub struct RegisterAllocationData {}
    pub struct Code {}
    pub struct InstructionBlocks {}
    pub struct CodeTracer {}
    pub struct TurbofanPipelineStatistics {}
    pub struct WasmRevecAnalyzer {}
    pub struct WasmShuffleAnalyzer {}
    pub struct CallDescriptor {}

    // Dummy impls

    impl Frame {
        pub fn new(_fixed_frame_size: i32, _zone: &ZoneWithName<kCodegenZoneName>) -> Self {
            Self {}
        }
    }
    impl InstructionSequence {
        pub fn InstructionBlocksFor(_zone: &Zone, _graph: &Graph) -> *mut InstructionBlocks {
            std::ptr::null_mut()
        }
        pub fn instruction_blocks(&self) -> *mut InstructionBlocks {
            std::ptr::null_mut()
        }
    }
    impl InstructionBlocks {
        pub fn mark_needs_frame(&mut self) {}
    }

    impl CodeGenerator {
        pub fn new(
            _zone: &ZoneWithName<kCodegenZoneName>,
            _frame: &Frame,
            _linkage: &Linkage,
            _sequence: *mut InstructionSequence,
            _info: &OptimizedCompilationInfo,
            _isolate: *mut Isolate,
            _osr_helper: std::option::Option<OsrHelper>,
            _start_source_position: i32,
            _jump_optimization_info: *mut JumpOptimizationInfo,
            _assembler_options: AssemblerOptions,
            _builtin: i32,
            _max_unoptimized_frame_height: usize,
            _max_pushed_argument_count: usize,
            _debug_name: *const char,
        ) -> Self {
            Self {}
        }
    }

    // TODO: Fix lifetimes, probably Rc or Arc
    #[derive(Debug)]
    pub struct ZoneWithName<const NAME: &'static str> {
        name: String,
    }

    impl<const NAME: &'static str> ZoneWithName<NAME> {
        pub fn new(zone_stats: *mut ZoneStats, name: &str) -> Self {
            ZoneWithName {
                name: name.to_string(),
            }
        }

        pub fn New<T>(&self, _t: Zone) -> *mut T {
            // Returning a dangling pointer, this is incomplete
            unsafe {
                let layout = std::alloc::Layout::new::<T>();
                let ptr = std::alloc::alloc(layout) as *mut T;
                // Use placement new or initialize directly
                //std::ptr::write(ptr, Default::default());
                ptr
            }
        }
    }

    pub struct ZoneWithNamePointer<T, const NAME: &'static str>(*mut T);
    impl<T, const NAME: &'static str> ZoneWithNamePointer<T, NAME> {
        pub fn new(ptr: *mut T) -> Self {
            ZoneWithNamePointer(ptr)
        }
    }

    impl<T, const NAME: &'static str> Deref for ZoneWithNamePointer<T, NAME> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            unsafe { &*self.0 }
        }
    }

    impl<T, const NAME: &'static str> DerefMut for ZoneWithNamePointer<T, NAME> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { &mut *self.0 }
        }
    }
    // Macro placeholders

    macro_rules! decl_turboshaft_phase_constants_impl {
        ($Name:ident, $CallStatsName:ident) => {
            //decl_pipeline_phase_constants_helper!($CallStatsName, PhaseKind::kTurboshaft, RuntimeCallStats::kThreadSpecific);
            const K_PHASE_NAME: &'static str = concat!("V8.TF", stringify!($CallStatsName));
            fn assert_turboshaft_phase() {
                //static_assert!(TurboshaftPhase<$Name##Phase>);
            }
        };
    }

    macro_rules! decl_turboshaft_phase_constants {
        ($Name:ident) => {
            decl_turboshaft_phase_constants_impl!($Name, Turboshaft##$Name);
        };
    }

    macro_rules! decl_turboshaft_phase_constants_with_legacy_name {
        ($Name:ident) => {
            decl_turboshaft_phase_constants_impl!($Name, $Name);
        };
    }

    macro_rules! decl_turboshaft_main_thread_pipeline_phase_constants_with_legacy_name {
        ($Name:ident) => {
            //decl_pipeline_phase_constants_helper!($Name, PhaseKind::kTurboshaft, RuntimeCallStats::kExact);
            const K_PHASE_NAME: &'static str = concat!("V8.TF", stringify!($Name));
            fn assert_turboshaft_phase() {
                //static_assert!(TurboshaftPhase<$Name##Phase>);
            }
        };
    }
    use std::unique_ptr;

    pub use decl_turboshaft_main_thread_pipeline_phase_constants_with_legacy_name;
    pub use decl_turboshaft_phase_constants;
    pub use decl_turboshaft_phase_constants_with_legacy_name;

    // Turboshaft phase trait definition
    pub trait TurboshaftPhase {
        const K_KIND: PhaseKind;
        fn run(data: &mut PipelineData, zone: &ZoneWithName<kCompilationZoneName>);
    }

    // Placeholder struct for Graph
    #[derive(Debug)]
    pub struct Graph {}

    pub struct AccountingAllocator {}

    impl AccountingAllocator {
        pub fn new() -> Self {
            AccountingAllocator {}
        }
    }

    pub struct Zone {
        accounting_allocator: AccountingAllocator,
    }

    impl Zone {
        pub fn new() -> Self {
            Zone {
                accounting_allocator: AccountingAllocator::new(),
            }
        }
    }

    // Placeholder structs
    pub struct Isolate {}
    impl Isolate {
        pub fn new() -> Self {
            Isolate {}
        }
    }

    // Pipeline Data Definition

    #[derive(Debug)]
    pub struct PipelineData {
        zone_stats_: *mut ZoneStats,
        compilation_zone_: ZoneWithName<kCompilationZoneName>,
        pipeline_kind_: TurboshaftPipelineKind,
        isolate_: *mut Isolate,
        info_: *mut OptimizedCompilationInfo,
        debug_name_: std::unique_ptr<char[]>,
        start_source_position_: i32,
        assembler_options_: AssemblerOptions,
        broker_: std::shared_ptr::SharedPtr<JSHeapBroker>,
        pipeline_statistics_: *mut TurbofanPipelineStatistics,
        dependencies_: *mut CompilationDependencies,
        code_: MaybeIndirectHandle<Code>,
        source_position_output_: String,
        runtime_call_stats_: *mut RuntimeCallStats,

        builtin_component_: Option<BuiltinComponent>,
        graph_component_: Option<GraphComponent>,
        codegen_component_: Option<CodegenComponent>,
        instruction_component_: Option<InstructionComponent>,
        register_component_: Option<RegisterComponent>,

        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        wasm_module_sig_: *const wasm::FunctionSig,
        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        wasm_canonical_sig_: *const wasm::CanonicalSig,
        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        wasm_module_: *const wasm::WasmModule,
        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        wasm_shared_: bool,
        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        wasm_shuffle_analyzer_: *mut WasmShuffleAnalyzer,

        #[cfg(all(V8_ENABLE_WEBASSEMBLY, V8_ENABLE_WASM_SIMD256_REVEC))]
        wasm_revec_analyzer_: *mut WasmRevecAnalyzer,
    }

    impl PipelineData {
        pub fn new(
            zone_stats: *mut ZoneStats,
            pipeline_kind: TurboshaftPipelineKind,
            isolate: *mut Isolate,
            info: *mut OptimizedCompilationInfo,
            assembler_options: AssemblerOptions,
            start_source_position: i32,
        ) -> Self {
            PipelineData {
                zone_stats_: zone_stats,
                compilation_zone_: ZoneWithName::new(zone_stats, "compilation_zone"),
                pipeline_kind_: pipeline_kind,
                isolate_: isolate,
                info_: info,
                debug_name_: OptimizedCompilationInfo::default().GetDebugName(),
                start_source_position_: start_source_position,
                assembler_options_: assembler_options,
                broker_: std::shared_ptr::SharedPtr::null(),
                pipeline_statistics_: std::ptr::null_mut(),
                dependencies_: std::ptr::null_mut(),
                code_: std::ptr::null_mut(),
                source_position_output_: String::new(),
                runtime_call_stats_: std::ptr::null_mut(),

                builtin_component_: None,
                graph_component_: None,
                codegen_component_: None,
                instruction_component_: None,
                register_component_: None,

                #[cfg(V8_ENABLE_WEBASSEMBLY)]
                wasm_module_sig_: std::ptr::null(),
                #[cfg(V8_ENABLE_WEBASSEMBLY)]
                wasm_canonical_sig_: std::ptr::null(),
                #[cfg(V8_ENABLE_WEBASSEMBLY)]
                wasm_module_: std::ptr::null(),
                #[cfg(V8_ENABLE_WEBASSEMBLY)]
                wasm_shared_: false,
                #[cfg(V8_ENABLE_WEBASSEMBLY)]
                wasm_shuffle_analyzer_: std::ptr::null_mut(),

                #[cfg(all(V8_ENABLE_WEBASSEMBLY, V8_ENABLE_WASM_SIMD256_REVEC))]
                wasm_revec_analyzer_: std::ptr::null_mut(),
            }
        }

        pub fn InitializeBrokerAndDependencies(
            &mut self,
            broker: std::shared_ptr::SharedPtr<JSHeapBroker>,
            dependencies: *mut CompilationDependencies,
        ) {
            assert!(self.broker_.is_null());
            assert!(self.dependencies_ == std::ptr::null_mut());
            assert!(!broker.is_null());
            assert!(dependencies != std::ptr::null_mut());
            self.broker_ = broker;
            self.dependencies_ = dependencies;
        }

        pub fn InitializeBuiltinComponent(
            &mut self,
            call_descriptor: *const CallDescriptor,
            bytecode_handler_data: Option<BytecodeHandlerData>,
        ) {
            assert!(self.builtin_component_.is_none());
            self.builtin_component_ = Some(BuiltinComponent::new(
                call_descriptor,
                bytecode_handler_data.map(|data| data),
            ));
        }

        pub fn InitializeGraphComponent(&mut self, source_positions: *mut SourcePositionTable) {
            assert!(self.graph_component_.is_none());
            self.graph_component_ = Some(GraphComponent::new(self.zone_stats_));
            let graph_component = self.graph_component_.as_mut().unwrap();
            //let zone = &mut graph_component.zone;
            //graph_component.graph = graph_component.zone.New::<Graph>(Zone::new());
            graph_component.source_positions =
                GraphComponent::Pointer::new(source_positions);
            if !self.info_.is_null() && unsafe { (*self.info_).trace_turbo_json() } {
                //graph_component.node_origins = graph_component.zone.New::<NodeOriginTable>(Zone::new());
                graph_component.node_origins = GraphComponent::Pointer::new(std::ptr::null_mut());
            }
        }

        pub fn InitializeGraphComponentWithGraphZone(
            &mut self,
            graph_zone: ZoneWithName<kGraphZoneName>,
            source_positions: ZoneWithNamePointer<SourcePositionTable, kGraphZoneName>,
            node_origins: ZoneWithNamePointer<NodeOriginTable, kGraphZoneName>,
        ) {
            assert!(self.graph_component_.is_none());
            self.graph_component_ = Some(GraphComponent::new(self.zone_stats_));
            let graph_component = self.graph_component_.as_mut().unwrap();
            //let zone = &mut graph_component.zone;
            //graph_component.graph = zone.New::<Graph>(Zone::new());
            graph_component.source_positions = source_positions;
            graph_component.node_origins = node_origins;
            if graph_component.node_origins.0.is_null()
                && !self.info_.is_null()
                && unsafe { (*self.info_).trace_turbo_json() }
            {
                //graph_component.node_origins = zone.New::<NodeOriginTable>(zone);
                graph_component.node_origins = GraphComponent::Pointer::new(std::ptr::null_mut());
            }
        }

        pub fn ClearGraphComponent(&mut self) {
            assert!(self.graph_component_.is_some());
            self.graph_component_ = None;
        }

        pub fn InitializeCodegenComponent(
            &mut self,
            osr_helper: std::shared_ptr::SharedPtr<OsrHelper>,
            jump_optimization_info: *mut JumpOptimizationInfo,
        ) {
            assert!(self.codegen_component_.is_none());
            self.codegen_component_ = Some(CodegenComponent::new(self.zone_stats_));
            let codegen_component = self.codegen_component_.as_mut().unwrap();
            codegen_component.osr_helper = osr_helper;
            codegen_component.jump_optimization_info = jump_optimization_info;
        }

        pub fn ClearCodegenComponent(&mut self) {
            assert!(self.codegen_component_.is_some());
            self.codegen_component_ = None;
        }

        pub fn InitializeCodeGenerator(&mut self, linkage: *mut Linkage) {
            assert!(self.codegen_component_.is_some());
            let cg = self.codegen_component_.as_mut().unwrap();
            assert!(cg.code_generator.is_none());

            #[cfg(V8_ENABLE_WEBASSEMBLY)]
            assert_eq!(
                self.assembler_options_.is_wasm,
                unsafe { (*self.info_).IsWasm() } || unsafe { (*self.info_).IsWasmBuiltin() }
            );

            let osr_helper = if !cg.osr_helper.is_null() {
                Some(unsafe { *cg.osr_helper })
            } else {
                None
            };

            cg.code_generator = Some(Box::new(CodeGenerator::new(
                &cg.zone,
                cg.frame,
                linkage,
                self.sequence(),
                unsafe { &*self.info_ },
                self.isolate_,
                osr_helper,
                self.start_source_position_,
                cg.jump_optimization_info,
                self.assembler_options_,
                unsafe { (*self.info_).builtin() },
                cg.max_unoptimized_frame_height,
                cg.max_pushed_argument_count,
                std::ptr::null(),
            )));
        }

        pub fn InitializeInstructionComponent(&mut self, call_descriptor: *const CallDescriptor) {
            assert!(self.instruction_component_.is_none());
            self.instruction_component_ = Some(InstructionComponent::new(self.zone_stats()));
            let instruction_component = self.instruction_component_.as_mut().unwrap();
            let zone = &instruction_component.zone;
            // let instruction_blocks =
            //     InstructionSequence::InstructionBlocksFor(zone, &self.graph());
            //instruction_component.sequence = zone.New::<InstructionSequence>(isolate, zone, instruction_blocks);
            instruction_component.sequence =
                InstructionComponent::Pointer::new(std::ptr::null_mut());
            if !call_descriptor.is_null()
                && unsafe { (*call_descriptor).RequiresFrameAsIncoming() }
            {
                // instruction_component.sequence.instruction_blocks()[0]
                //    .mark_needs_frame();
            } else {
                assert!(unsafe { (*call_descriptor).CalleeSavedFPRegisters().is_empty() });
            }
        }

        pub fn InitializeInstructionComponentWithSequence(
            &mut self,
            sequence: *mut InstructionSequence,
        ) {
            assert!(self.instruction_component_.is_none());
            self.instruction_component_ = Some(InstructionComponent::new(self.zone_stats()));
            let instruction_component = self.instruction_component_.as_mut().unwrap();
            instruction_component.sequence =
                InstructionComponent::Pointer::new(sequence);
        }

        pub fn ClearInstructionComponent(&mut self) {
            assert!(self.instruction_component_.is_some());
            self.instruction_component_ = None;
        }

        pub fn InitializeRegisterComponent(
            &mut self,
            config: *const RegisterConfiguration,
            call_descriptor: *mut CallDescriptor,
        ) {
            // TODO: Implement this.  Not implemented in the C++ version either.
        }

        pub fn ClearRegisterComponent(&mut self) {
            assert!(self.register_component_.is_some());
            self.register_component_ = None;
        }

        pub fn allocator(&self) -> *mut AccountingAllocator {
            // TODO: Implement this
            std::ptr::null_mut()
        }
        pub fn zone_stats(&self) -> *mut ZoneStats {
            self.zone_stats_
        }
        pub fn pipeline_kind(&self) -> TurboshaftPipelineKind {
            self.pipeline_kind_
        }
        pub fn isolate(&self) -> *mut Isolate {
            self.isolate_
        }
        pub fn info(&self) -> *mut OptimizedCompilationInfo {
            self.info_
        }
        pub fn debug_name(&self) -> *const char {
            // TODO: Implement this
            std::ptr::null()
        }
        pub fn broker(&self) -> *mut JSHeapBroker {
            // TODO: Implement this
            std::ptr::null_mut()
        }
        pub fn depedencies(&self) -> *mut CompilationDependencies {
            self.dependencies_
        }
        pub fn assembler_options(&self) -> &AssemblerOptions {
            &self.assembler_options_
        }
        pub fn jump_optimization_info(&mut self) -> *mut JumpOptimizationInfo {
            if self.codegen_component_.is_none() {
                return std::ptr::null_mut();
            }
            self.codegen_component_.as_mut().unwrap().jump_optimization_info
        }
        pub fn builtin_call_descriptor(&self) -> *const CallDescriptor {
            assert!(self.builtin_component_.is_some());
            self.builtin_component_.as_ref().unwrap().call_descriptor
        }
        pub fn bytecode_handler_data(&mut self) -> Option<&mut BytecodeHandlerData> {
            assert!(self.builtin_component_.is_some());
            self.builtin_component_
                .as_mut()
                .unwrap()
                .bytecode_handler_data
                .as_mut()
        }

        pub fn has_graph(&self) -> bool {
            if self.graph_component_.is_some() {
                assert!(!self.graph_component_.as_ref().unwrap().graph.is_null());
            }
            self.graph_component_.is_some()
        }
        pub fn graph_zone(&mut self) -> &mut ZoneWithName<kGraphZoneName> {
            self.graph_component_.as_mut().unwrap().zone.as_mut()
        }
        pub fn graph(&self) -> &Graph {
            unsafe { &*self.graph_component_.as_ref().unwrap().graph }
        }
        pub fn source_positions(&self) -> GraphComponent::Pointer<SourcePositionTable> {
            self.graph_component_.as_ref().unwrap().source_positions
        }
        pub fn node_origins(&self) -> GraphComponent::Pointer<NodeOriginTable> {
            if self.graph_component_.is_none() {
                return GraphComponent::Pointer::new(std::ptr::null_mut());
            }
            self.graph_component_.as_ref().unwrap().node_origins
        }
        pub fn register_allocation_data(&self) -> *mut RegisterAllocationData {
            // TODO: Implement this
            std::ptr::null_mut()
        }
        pub fn register_allocation_zone(&mut self) -> *mut ZoneWithName<kRegisterAllocationZoneName> {
            // TODO: Implement this
            std::ptr::null_mut()
        }
        pub fn code_generator(&self) -> Option<&CodeGenerator> {
            self.codegen_component_
                .as_ref()
                .and_then(|cg| cg.code_generator.as_deref())
        }
        pub fn set_code(&mut self, code: MaybeIndirectHandle<Code>) {
            assert!(self.code_.is_null());
            self.code_ = code;
        }
        pub fn code(&self) -> MaybeIndirectHandle<Code> {
            self.code_
        }
        pub fn sequence(&self) -> *mut InstructionSequence {
            self.instruction_component_.as_ref().unwrap().sequence
        }
        pub fn frame(&self) -> *mut Frame {
            self.codegen_component_.as_ref().unwrap().frame
        }
        pub fn GetCodeTracer(&self) -> *mut CodeTracer {
            // TODO: Implement this
            std::ptr::null_mut()
        }
        pub fn max_unoptimized_frame_height(&mut self) -> &mut usize {
            &mut self.codegen_component_.as_mut().unwrap().max_unoptimized_frame_height
        }
        pub fn max_pushed_argument_count(&mut self) -> &mut usize {
            &mut self
                .codegen_component_
                .as_mut()
                .unwrap()
                .max_pushed_argument_count
        }
        pub fn runtime_call_stats(&self) -> *mut RuntimeCallStats {
            self.runtime_call_stats_
        }
        pub fn set_runtime_call_stats(&mut self, stats: *mut RuntimeCallStats) {
            self.runtime_call_stats_ = stats;
        }

        // The {compilation_zone} outlives the entire compilation pipeline. It is
        // shared between all phases (including code gen where the graph zone is gone
        // already).
        pub fn compilation_zone(&mut self) -> &mut ZoneWithName<kCompilationZoneName> {
            &mut self.compilation_zone_
        }

        pub fn pipeline_statistics(&self) -> *mut TurbofanPipelineStatistics {
            self.pipeline_statistics_
        }
        pub fn set_pipeline_statistics(&mut self, pipeline_statistics: *mut TurbofanPipelineStatistics) {
            self.pipeline_statistics_ = pipeline_statistics;
        }

        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        pub fn wasm_module_sig(&self) -> *const wasm::FunctionSig {
            self.wasm_module_sig_
        }

        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        pub fn wasm_canonical_sig(&self) -> *const wasm::CanonicalSig {
            self.wasm_canonical_sig_
        }

        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        pub fn wasm_module(&self) -> *const wasm::WasmModule {
            self.wasm_module_
        }

        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        pub fn wasm_shared(&self) -> bool {
            self.wasm_shared_
        }

        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        pub fn SetIsWasmFunction(&mut self, module: *const wasm::WasmModule, sig: *const wasm::FunctionSig, shared: bool) {
            self.wasm_module_ = module;
            self.wasm_module_sig_ = sig;
            self.wasm_shared_ = shared;
            assert!(self.pipeline_kind() == TurboshaftPipelineKind::kWasm ||
                     self.pipeline_kind() == TurboshaftPipelineKind::kJSToWasm);
        }

        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        pub fn SetIsWasmWrapper(&mut self, sig: *const wasm::CanonicalSig) {
            self.wasm_canonical_sig_ = sig;
            assert!(self.pipeline_kind() == TurboshaftPipelineKind::kWasm ||
                     self.pipeline_kind() == TurboshaftPipelineKind::kJSToWasm);
        }

        #[cfg(all(V8_ENABLE_WEBASSEMBLY, V8_ENABLE_WASM_SIMD256_REVEC))]
        pub fn wasm_revec_analyzer(&self) -> *mut WasmRevecAnalyzer {
            assert!(!self.wasm_revec_analyzer_.is_null());
            self.wasm_revec_analyzer_
        }

        #[cfg(all(V8_ENABLE_WEBASSEMBLY, V8_ENABLE_WASM_SIMD256_REVEC))]
        pub fn set_wasm_revec_analyzer(&mut self, wasm_revec_analyzer: *mut WasmRevecAnalyzer) {
            assert!(self.wasm_revec_analyzer_.is_null());
            self.wasm_revec_analyzer_ = wasm_revec_analyzer;
        }

        #[cfg(all(V8_ENABLE_WEBASSEMBLY, V8_ENABLE_WASM_SIMD256_REVEC))]
        pub fn clear_wasm_revec_analyzer(&mut self) {
            self.wasm_revec_analyzer_ = std::ptr::null_mut();
        }

        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        pub fn wasm_shuffle_analyzer(&self) -> *mut WasmShuffleAnalyzer {
            assert!(!self.wasm_shuffle_analyzer_.is_null());
            self.wasm_shuffle_analyzer_
        }

        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        pub fn set_wasm_shuffle_analyzer(&mut self, wasm_shuffle_analyzer: *mut WasmShuffleAnalyzer) {
            assert!(self.wasm_shuffle_analyzer_.is_null());
            self.wasm_shuffle_analyzer_ = wasm_shuffle_analyzer;
        }

        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        pub fn clear_wasm_shuffle_analyzer(&mut self) {
            self.wasm_shuffle_analyzer_ = std::ptr::null_mut();
        }

        pub fn is_wasm(&self) -> bool {
            self.pipeline_kind() == TurboshaftPipelineKind::kWasm ||
                self.pipeline_kind() == TurboshaftPipelineKind::kJSToWasm
        }

        pub fn is_js_to_wasm(&self) -> bool {
            self.pipeline_kind() == TurboshaftPipelineKind::kJSToWasm
        }

        pub fn InitializeFrameData(&mut self, call_descriptor: *mut CallDescriptor) {
            assert!(self.codegen_component_.is_some());
            assert!(self.codegen_component_.as_ref().unwrap().frame.is_null());

            let mut fixed_frame_size = 0;
            if !call_descriptor.is_null() {
                unsafe {
                    fixed_frame_size = (*call_descriptor).CalculateFixedFrameSize((*self.info_).code_kind());
                }
            }

            let codegen_component = self.codegen_component_.as_mut().unwrap();
            codegen_component.frame = codegen_component.zone.New::<Frame>(Frame::new(fixed_frame_size, &codegen_component.zone));

            if !codegen_component.osr_helper.is_null() {
                //codegen_component.osr_helper.SetupFrame(codegen_component.frame);
            }
        }

        pub fn set_source_position_output(&mut self, source_position_output: String) {
            self.source_position_output_ = source_position_output;
        }
        pub fn source_position_output(&self) -> String {
            self.source_position_output_.clone()
        }

        pub fn graph_has_special_rpo(&self) -> bool {
            self.graph_component_.as_ref().unwrap().graph_has_special_rpo
        }
        pub fn set_graph_has_special_rpo(&mut self) {
            self.graph_component_.as_mut().unwrap().graph_has_special_rpo = true;
        }
        pub fn graph_has_lowered_fast_api_calls(&self) -> bool {
            self.graph_component_.as_ref().unwrap().graph_has_lowered_fast_api_calls
        }
        pub fn set_graph_has_lowered_fast_api_calls(&mut self) {
            self.graph_component_.as_mut().unwrap().graph_has_lowered_fast_api_calls = true;
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub enum TurboshaftPipelineKind {
        kJS,
        kWasm,
        kCSA,
        kTSABuiltin,
        kJSToWasm,
    }

    // Component structs

    #[derive(Debug)]
    