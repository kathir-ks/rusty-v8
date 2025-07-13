// Converted from V8 C++ source files:
// Header: phase.h
// Implementation: phase.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::fmt;
use std::io::Write;
use std::mem;
use std::optional::Optional;
use std::rc::Rc;
use std::string::String;
use std::sync::{Arc, Mutex, RwLock};
use std::vec::Vec;

pub mod base {
    pub mod contextual;
    pub mod template_meta_programming {
        pub mod functional;
    }
}
pub mod codegen {
    pub mod assembler;
    pub mod optimized_compilation_info;
}
pub mod common {
    pub mod globals;
}
pub mod compiler {
    pub mod access_info;
    pub mod backend {
        pub mod instruction;
    }
    pub mod compilation_dependencies;
    pub mod compiler_source_position_table;
    pub mod node_origin_table;
    pub mod osr;
    pub mod phase;
    pub mod turboshaft {
        pub mod builtin_compiler;
        pub mod graph;
        pub mod sidetable;
        pub mod zone_with_name;
    }
    pub mod js_heap_broker;
    pub mod turbofan_graph_visualizer;
    pub mod turboshaft {
        pub mod graph_visualizer;
    }
    pub mod pipeline_data_inl;
    pub mod representation_change;
}
pub mod diagnostics {
    pub mod code_tracer;
}
pub mod execution {
    pub mod isolate;
    pub mod simulator_base;
}
pub mod logging {
    pub mod runtime_call_stats;
}
pub mod utils {
    pub mod ostreams;
}
pub mod wasm {
    pub mod wasm_engine;
}
pub mod zone {
    pub mod accounting_allocator;
    pub mod zone;
}

use crate::base::contextual::*;
use crate::base::template_meta_programming::functional::*;
use crate::codegen::assembler::*;
use crate::codegen::optimized_compilation_info::*;
use crate::common::globals::*;
use crate::compiler::access_info::*;
use crate::compiler::backend::instruction::*;
use crate::compiler::compilation_dependencies::*;
use crate::compiler::compiler_source_position_table::*;
use crate::compiler::js_heap_broker::JSHeapBroker;
use crate::compiler::node_origin_table::*;
use crate::compiler::osr::*;
use crate::compiler::phase::*;
use crate::compiler::pipeline_data_inl::*;
use crate::compiler::turbofan_graph_visualizer::*;
use crate::compiler::turboshaft::builtin_compiler::*;
use crate::compiler::turboshaft::graph::*;
use crate::compiler::turboshaft::graph_visualizer::*;
use crate::compiler::turboshaft::sidetable::*;
use crate::compiler::turboshaft::zone_with_name::*;
use crate::diagnostics::code_tracer::*;
use crate::execution::isolate::*;
use crate::execution::simulator_base::*;
use crate::logging::runtime_call_stats::*;
use crate::utils::ostreams::*;
use crate::wasm::wasm_engine::*;
use crate::zone::accounting_allocator::*;
use crate::zone::zone::*;

pub struct V8_EXPORT_PRIVATE {}

pub struct ByteArrayHeader {}

pub struct OptimizedCompilationInfo {}

pub struct Isolate {}

impl Isolate {
    pub fn allocator(&self) -> *mut AccountingAllocator {
        // Returning a dummy pointer since the actual allocator requires more context.
        std::ptr::null_mut()
    }
    pub fn GetCodeTracer(&self) -> *mut CodeTracer {
        std::ptr::null_mut()
    }
}

pub struct AssemblerOptions {
    pub is_wasm: bool,
}

impl AssemblerOptions {
    pub fn new() -> Self {
        AssemblerOptions { is_wasm: false }
    }
}

pub struct Code {}

pub struct MaybeIndirectHandle<T> {
    data: Option<Box<T>>,
}

impl<T> MaybeIndirectHandle<T> {
    pub fn new(data: Option<Box<T>>) -> Self {
        MaybeIndirectHandle { data }
    }
    pub fn is_null(&self) -> bool {
        self.data.is_none()
    }
}

pub struct SourcePositionTable {}

pub struct NodeOriginTable {}

pub struct Frame {}

pub struct CompilationDependencies {}

pub struct JumpOptimizationInfo {}

pub struct RuntimeCallStats {}

pub struct InstructionSequence {
    instruction_blocks: Vec<InstructionBlock>,
}

impl InstructionSequence {
    pub fn InstructionBlocksFor(zone: *mut Zone, graph: &Graph) -> Vec<InstructionBlock> {
        Vec::new()
    }

    pub fn instruction_blocks(&mut self) -> &mut Vec<InstructionBlock> {
        &mut self.instruction_blocks
    }
}

pub struct InstructionBlock {}

impl InstructionBlock {
    pub fn mark_needs_frame(&mut self) {}
}

pub struct RegisterAllocationData {}

pub struct RegisterConfiguration {}

pub struct Linkage {}

pub struct CodeGenerator {}

impl CodeGenerator {
    pub fn new(
        zone: &ZoneWithName<kCodegenZoneName>,
        frame: *mut Frame,
        linkage: *mut Linkage,
        sequence: *mut InstructionSequence,
        info: *mut OptimizedCompilationInfo,
        isolate: *mut Isolate,
        osr_helper: std::optional<OsrHelper>,
        start_source_position: i32,
        jump_optimization_info: *mut JumpOptimizationInfo,
        assembler_options: AssemblerOptions,
        is_builtin: bool,
        max_unoptimized_frame_height: usize,
        max_pushed_argument_count: usize,
        debug_name: *const i8,
    ) -> Self {
        CodeGenerator {}
    }
}

pub struct CallDescriptor {}

impl CallDescriptor {
    pub fn RequiresFrameAsIncoming(&self) -> bool {
        false
    }
    pub fn CalleeSavedFPRegisters(&self) -> Vec<i32> {
        Vec::new()
    }
    pub fn CalculateFixedFrameSize(&self, code_kind: i32) -> i32 {
        0
    }
}

pub struct BytecodeHandlerData {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TurboshaftPipelineKind {
    kJS,
    kWasm,
    kCSA,
    kTSABuiltin,
    kJSToWasm,
}

#[allow(non_camel_case_types)]
pub struct kGraphZoneName {}
#[allow(non_camel_case_types)]
pub struct kCompilationZoneName {}
#[allow(non_camel_case_types)]
pub struct kCodegenZoneName {}
#[allow(non_camel_case_types)]
pub struct kInstructionZoneName {}
#[allow(non_camel_case_types)]
pub struct kRegisterAllocationZoneName {}

pub struct ZoneWithNamePointer<T, ZoneName> {
    ptr: *mut T,
}

impl<T, ZoneName> ZoneWithNamePointer<T, ZoneName> {
    pub fn new(ptr: *mut T) -> Self {
        ZoneWithNamePointer { ptr }
    }
}

pub struct ZoneWithName<ZoneName> {
    name: std::marker::PhantomData<ZoneName>,
}

impl<ZoneName> ZoneWithName<ZoneName> {
    pub fn new() -> Self {
        ZoneWithName {
            name: std::marker::PhantomData,
        }
    }
    pub fn New<T>(&self, _zone: &Zone) -> *mut T {
        // Dummy implementation.  In real code, this would allocate memory
        // from the zone.
        std::ptr::null_mut()
    }
}

pub struct UnparkedScopeIfNeeded {}
pub struct AllowHandleDereference {}

pub struct OsrHelper {}

impl OsrHelper {
    pub fn SetupFrame(&self, frame: *mut Frame) {}
}

pub struct WasmRevecAnalyzer {}

pub struct WasmShuffleAnalyzer {}

const kNoSourcePosition: i32 = -1;

mod detail {
    use super::*;

    pub struct BuiltinComponent {
        pub call_descriptor: *const CallDescriptor,
        pub bytecode_handler_data: std::option::Option<BytecodeHandlerData>,
    }

    impl BuiltinComponent {
        pub fn new(
            call_descriptor: *const CallDescriptor,
            bytecode_handler_data: std::option::Option<BytecodeHandlerData>,
        ) -> Self {
            BuiltinComponent {
                call_descriptor,
                bytecode_handler_data,
            }
        }
    }

    pub struct GraphComponent {
        pub zone: ZoneWithName<kGraphZoneName>,
        pub graph: *mut Graph,
        pub source_positions: ZoneWithNamePointer<SourcePositionTable, kGraphZoneName>,
        pub node_origins: ZoneWithNamePointer<NodeOriginTable, kGraphZoneName>,
        pub graph_has_special_rpo: bool,
        pub graph_has_lowered_fast_api_calls: bool,
    }

    impl GraphComponent {
        pub fn new(zone_stats: *mut ZoneStats) -> Self {
            let zone = ZoneWithName::<kGraphZoneName>::new();
            GraphComponent {
                zone,
                graph: std::ptr::null_mut(),
                source_positions: ZoneWithNamePointer::new(std::ptr::null_mut()),
                node_origins: ZoneWithNamePointer::new(std::ptr::null_mut()),
                graph_has_special_rpo: false,
                graph_has_lowered_fast_api_calls: false,
            }
        }
    }

    pub struct CodegenComponent {
        pub zone: ZoneWithName<kCodegenZoneName>,
        pub frame: *mut Frame,
        pub code_generator: std::option::Option<std::unique_ptr<CodeGenerator>>,
        pub dependencies: *mut CompilationDependencies,
        pub osr_helper: std::shared_ptr::SharedPtr<OsrHelper>,
        pub jump_optimization_info: *mut JumpOptimizationInfo,
        pub max_unoptimized_frame_height: usize,
        pub max_pushed_argument_count: usize,
    }

    impl CodegenComponent {
        pub fn new(zone_stats: *mut ZoneStats) -> Self {
            CodegenComponent {
                zone: ZoneWithName::<kCodegenZoneName>::new(),
                frame: std::ptr::null_mut(),
                code_generator: std::option::Option::None,
                dependencies: std::ptr::null_mut(),
                osr_helper: std::shared_ptr::SharedPtr::new(),
                jump_optimization_info: std::ptr::null_mut(),
                max_unoptimized_frame_height: 0,
                max_pushed_argument_count: 0,
            }
        }
    }

    pub struct InstructionComponent {
        pub zone: ZoneWithName<kInstructionZoneName>,
        pub sequence: ZoneWithNamePointer<InstructionSequence, kInstructionZoneName>,
    }

    impl InstructionComponent {
        pub fn new(zone_stats: *mut ZoneStats) -> Self {
            InstructionComponent {
                zone: ZoneWithName::<kInstructionZoneName>::new(),
                sequence: ZoneWithNamePointer::new(std::ptr::null_mut()),
            }
        }
    }

    pub struct RegisterComponent {
        pub zone: ZoneWithName<kRegisterAllocationZoneName>,
        pub allocation_data: *mut RegisterAllocationData,
    }

    impl RegisterComponent {
        pub fn new(zone_stats: *mut ZoneStats) -> Self {
            RegisterComponent {
                zone: ZoneWithName::<kRegisterAllocationZoneName>::new(),
                allocation_data: std::ptr::null_mut(),
            }
        }
    }
}

pub struct PipelineData {
    zone_stats_: *mut ZoneStats,
    compilation_zone_: ZoneWithName<kCompilationZoneName>,
    pipeline_kind_: TurboshaftPipelineKind,
    isolate_: *mut Isolate,
    info_: *mut OptimizedCompilationInfo,
    debug_name_: std::unique_ptr<[i8]>,
    broker_: std::shared_ptr::SharedPtr<JSHeapBroker>,
    pipeline_statistics_: *mut TurbofanPipelineStatistics,
    dependencies_: *mut CompilationDependencies,
    start_source_position_: i32,
    assembler_options_: AssemblerOptions,
    code_: MaybeIndirectHandle<Code>,
    source_position_output_: String,
    runtime_call_stats_: *mut RuntimeCallStats,

    builtin_component_: std::option::Option<detail::BuiltinComponent>,
    graph_component_: std::option::Option<detail::GraphComponent>,
    codegen_component_: std::option::Option<detail::CodegenComponent>,
    instruction_component_: std::option::Option<detail::InstructionComponent>,
    register_component_: std::option::Option<detail::RegisterComponent>,

    wasm_module_sig_: *const wasm::FunctionSig,
    wasm_canonical_sig_: *const wasm::CanonicalSig,
    wasm_module_: *const wasm::WasmModule,
    wasm_shared_: bool,
    wasm_shuffle_analyzer_: *mut WasmShuffleAnalyzer,

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
            compilation_zone_: ZoneWithName::<kCompilationZoneName>::new(),
            pipeline_kind_: pipeline_kind,
            isolate_: isolate,
            info_: info,
            debug_name_: std::unique_ptr::<[i8]>::new(),
            broker_: std::shared_ptr::SharedPtr::new(),
            pipeline_statistics_: std::ptr::null_mut(),
            dependencies_: std::ptr::null_mut(),
            start_source_position_: start_source_position,
            assembler_options_: assembler_options,
            code_: MaybeIndirectHandle::new(std::option::Option::None),
            source_position_output_: String::new(),
            runtime_call_stats_: std::ptr::null_mut(),
            builtin_component_: std::option::Option::None,
            graph_component_: std::option::Option::None,
            codegen_component_: std::option::Option::None,
            instruction_component_: std::option::Option::None,
            register_component_: std::option::Option::None,
            wasm_module_sig_: std::ptr::null(),
            wasm_canonical_sig_: std::ptr::null(),
            wasm_module_: std::ptr::null(),
            wasm_shared_: false,
            wasm_shuffle_analyzer_: std::ptr::null_mut(),
            wasm_revec_analyzer_: std::ptr::null_mut(),
        }
    }

    pub fn InitializeBrokerAndDependencies(
        &mut self,
        broker: std::shared_ptr::SharedPtr<JSHeapBroker>,
        dependencies: *mut CompilationDependencies,
    ) {
        self.broker_ = broker;
        self.dependencies_ = dependencies;
    }

    pub fn InitializeBuiltinComponent(
        &mut self,
        call_descriptor: *const CallDescriptor,
        bytecode_handler_data: std::option::Option<BytecodeHandlerData>,
    ) {
        self.builtin_component_ = std::option::Option::Some(detail::BuiltinComponent::new(
            call_descriptor,
            bytecode_handler_data,
        ));
    }

    pub fn InitializeGraphComponent(&mut self, source_positions: *mut SourcePositionTable) {
        self.graph_component_ = std::option::Option::Some(detail::GraphComponent::new(self.zone_stats_));
        let graph_component = self.graph_component_.as_mut().unwrap();
        graph_component.graph = graph_component.zone.New::<Graph>(&Zone {});
        graph_component.source_positions =
            ZoneWithNamePointer::new(source_positions);
        if !self.info_.is_null() && unsafe { (*self.info_).trace_turbo_json() } {
            graph_component.node_origins = graph_component.zone.New::<NodeOriginTable>(&Zone {});
        }
    }

    pub fn InitializeGraphComponentWithGraphZone(
        &mut self,
        graph_zone: ZoneWithName<kGraphZoneName>,
        source_positions: ZoneWithNamePointer<SourcePositionTable, kGraphZoneName>,
        node_origins: ZoneWithNamePointer<NodeOriginTable, kGraphZoneName>,
    ) {
        self.graph_component_ = std::option::Option::Some(detail::GraphComponent {
            zone: graph_zone,
            graph: std::ptr::null_mut(),
            source_positions,
            node_origins,
            graph_has_special_rpo: false,
            graph_has_lowered_fast_api_calls: false,
        });

        let graph_component = self.graph_component_.as_mut().unwrap();
        graph_component.graph = graph_component.zone.New::<Graph>(&Zone {});
        if !graph_component.node_origins.ptr.is_null() && !self.info_.is_null() && unsafe { (*self.info_).trace_turbo_json() } {
            graph_component.node_origins = graph_component.zone.New::<NodeOriginTable>(&Zone {});
        }
    }

    pub fn ClearGraphComponent(&mut self) {
        self.graph_component_ = std::option::Option::None;
    }

    pub fn InitializeCodegenComponent(
        &mut self,
        osr_helper: std::shared_ptr::SharedPtr<OsrHelper>,
        jump_optimization_info: *mut JumpOptimizationInfo,
    ) {
        self.codegen_component_ = std::option::Option::Some(detail::CodegenComponent::new(self.zone_stats_));
        let codegen_component = self.codegen_component_.as_mut().unwrap();
        codegen_component.osr_helper = osr_helper;
        codegen_component.jump_optimization_info = jump_optimization_info;
    }

    pub fn ClearCodegenComponent(&mut self) {
        self.codegen_component_ = std::option::Option::None;
    }

    pub fn InitializeCodeGenerator(&mut self, linkage: *mut Linkage) {
        let cg = self.codegen_component_.as_mut().unwrap();
        let assembler_options = self.assembler_options_.clone();
        let osr_helper = if cg.osr_helper.is_null() {
            std::option::Option::None
        } else {
            std::option::Option::Some(unsafe { cg.osr_helper.as_ref().clone() })
        };

        cg.code_generator = std::option::Option::Some(std::unique_ptr::new(CodeGenerator::new(
            &cg.zone,
            cg.frame,
            linkage,
            self.sequence(),
            self.info_,
            self.isolate_,
            osr_helper,
            self.start_source_position_,
            cg.jump_optimization_info,
            assembler_options,
            false,
            cg.max_unoptimized_frame_height,
            cg.max_pushed_argument_count,
            std::ptr::null(),
        )));
    }

    pub fn InitializeInstructionComponent(&mut self, call_descriptor: *const CallDescriptor) {
        self.instruction_component_ = std::option::Option::Some(detail::InstructionComponent::new(self.zone_stats()));
        let instruction_component = self.instruction_component_.as_mut().unwrap();
        let zone_ptr = self.zone_stats_;
        let instruction_blocks = InstructionSequence::InstructionBlocksFor(unsafe { *zone_ptr }.zone, &self.graph());
        let zone = &unsafe { &*zone_ptr }.zone;
        let instruction_sequence = zone.New::<InstructionSequence>(zone);
        instruction_component.sequence = ZoneWithNamePointer::new(instruction_sequence);

        if !call_descriptor.is_null() && unsafe { (*call_descriptor).RequiresFrameAsIncoming() } {
            unsafe {
                (*instruction_component.sequence.ptr).instruction_blocks()[0].mark_needs_frame();
            }
        }
    }

    pub fn InitializeInstructionComponentWithSequence(&mut self, sequence: *mut InstructionSequence) {
        self.instruction_component_ = std::option::Option::Some(detail::InstructionComponent {
            zone: ZoneWithName::<kInstructionZoneName>::new(),
            sequence: ZoneWithNamePointer::new(sequence),
        });
    }

    pub fn ClearInstructionComponent(&mut self) {
        self.instruction_component_ = std::option::Option::None;
    }

    pub fn InitializeRegisterComponent(
        &mut self,
        config: *const RegisterConfiguration,
        call_descriptor: *mut CallDescriptor,
    ) {
        self.register_component_ = std::option::Option::Some(detail::RegisterComponent::new(self.zone_stats()));
        let register_component = self.register_component_.as_mut().unwrap();
        register_component.allocation_data = unsafe { (*self.zone_stats_).zone }.New::<RegisterAllocationData>(&Zone {});
    }

    pub fn ClearRegisterComponent(&mut self) {
        self.register_component_ = std::option::Option::None;
    }

    pub fn allocator(&self) -> *mut AccountingAllocator {
        if !self.isolate_.is_null() {
            unsafe { (*self.isolate_).allocator() }
        } else {
            std::ptr::null_mut()
        }
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
    pub fn debug_name(&self) -> *const i8 {
        self.debug_name_.as_ptr() as *const i8
    }
    pub fn broker(&self) -> *mut JSHeapBroker {
        if self.broker_.is_null() {
            std::ptr::null_mut()
        } else {
            unsafe { self.broker_.as_ptr() as *mut JSHeapBroker }
        }
    }
    pub fn depedencies(&self) -> *mut CompilationDependencies {
        self.dependencies_
    }
    pub fn assembler_options(&self) -> &AssemblerOptions {
        &self.assembler_options_
    }
    pub fn jump_optimization_info(&self) -> *mut JumpOptimizationInfo {
        if self.codegen_component_.is_none() {
            std::ptr::null_mut()
        } else {
            self.codegen_component_.as_ref().unwrap().jump_optimization_info
        }
    }
    pub fn builtin_call_descriptor(&self) -> *const CallDescriptor {
        self.builtin_component_.as_ref().unwrap().call_descriptor
    }
    pub fn bytecode_handler_data(&mut self) -> &mut std::option::Option<BytecodeHandlerData> {
        &mut self.builtin_component_.as_mut().unwrap().bytecode_handler_data
    }

    pub fn has_graph(&self) -> bool {
        self.graph_component_.is_some()
    }
    pub fn graph_zone(&mut self) -> &mut ZoneWithName<kGraphZoneName> {
        &mut self.graph_component_.as_mut().unwrap().zone
    }
    pub fn graph(&self) -> &Graph {
        unsafe { &*self.graph_component_.as_ref().unwrap().graph }
    }
    pub fn source_positions(&self) -> ZoneWithNamePointer<SourcePositionTable, kGraphZoneName> {
        self.graph_component_.as_ref().unwrap().source_positions
    }
    pub fn node_origins(&self) -> ZoneWithNamePointer<NodeOriginTable, kGraphZoneName> {
        if self.graph_component_.is_none() {
            ZoneWithNamePointer { ptr: std::ptr::null_mut() }
        } else {
            self.graph_component_.as_ref().unwrap().node_origins
        }
    }
    pub fn register_allocation_data(&self) -> *mut RegisterAllocationData {
        self.register_component_.as_ref().unwrap().allocation_data
    }
    pub fn register_allocation_zone(&mut self) -> &mut ZoneWithName<kRegisterAllocationZoneName> {
        &mut self.register_component_.as_mut().unwrap().zone
    }
    pub fn code_generator(&self) -> *mut CodeGenerator {
        if let Some(ref codegen_component) = self.codegen_component_ {
            if let Some(ref code_generator) = codegen_component.code_generator {
                return code_generator.as_ptr() as *mut CodeGenerator;
            }
        }
        std::ptr::null_mut()
    }
    pub fn set_code(&mut self, code: MaybeIndirectHandle<Code>) {
        self.code_ = code;
    }
    pub fn code(&self) -> MaybeIndirectHandle<Code> {
        self.code_.clone()
    }
    pub fn sequence(&self) -> *mut InstructionSequence {
        self.instruction_component_.as_ref().unwrap().sequence.ptr
    }
    pub fn frame(&self) -> *mut Frame {
        self.codegen_component_.as_ref().unwrap().frame
    }
    pub fn GetCodeTracer(&self) -> *mut CodeTracer {
        if !self.info_.is_null() {
            unsafe { (*self.isolate_).GetCodeTracer() }
        } else {
            std::ptr::null_mut()
        }
    }
    pub fn max_unoptimized_frame_height(&mut self) -> &mut usize {
        &mut self.codegen_component_.as_mut().unwrap().max_unoptimized_frame_height
    }
    pub fn max_pushed_argument_count(&mut self) -> &mut usize {
        &mut self.codegen_component_.as_mut().unwrap().max_pushed_argument_count
    }
    pub fn runtime_call_stats(&self) -> *mut RuntimeCallStats {
        self.runtime_call_stats_
    }
    pub fn set_runtime_call_stats(&mut self, stats: *mut RuntimeCallStats) {
        self.runtime_call_stats_ = stats;
    }

    pub fn compilation_zone(&mut self) -> &mut ZoneWithName<kCompilationZoneName> {
        &mut self.compilation_zone_
    }

    pub fn pipeline_statistics(&self) -> *mut TurbofanPipelineStatistics {
        self.pipeline_statistics_
    }
    pub fn set_pipeline_statistics(&mut self, pipeline_statistics: *mut TurbofanPipelineStatistics) {
        self.pipeline_statistics_ = pipeline_statistics;
    }

    pub fn wasm_module_sig(&self) -> *const wasm::FunctionSig {
        self.wasm_module_sig_
    }

    pub fn wasm_canonical_sig(&self) -> *const wasm::CanonicalSig {
        self.wasm_canonical_sig_
    }

    pub fn wasm_module(&self) -> *const wasm::WasmModule {
        self.wasm_module_
    }

    pub fn wasm_shared(&self) -> bool {
        self.wasm_shared_
    }

    pub fn SetIsWasmFunction(&mut self, module: *const wasm::WasmModule, sig: *const wasm::FunctionSig, shared: bool) {
        self.wasm_module_ = module;
        self.wasm_module_sig_ = sig;
        self.wasm_shared_ = shared;
    }

    pub fn SetIsWasmWrapper(&mut self, sig: *const wasm::CanonicalSig) {
        self.wasm_canonical_sig_ = sig;
    }

    pub fn wasm_revec_analyzer(&self) -> *mut WasmRevecAnalyzer {
        self.wasm_revec_analyzer_
    }

    pub fn set_wasm_revec_analyzer(&mut self, wasm_revec_analyzer: *mut WasmRevecAnalyzer) {
        self.wasm_revec_analyzer_ = wasm_revec_analyzer;
    }

    pub fn clear_wasm_revec_analyzer(&mut self) {
        self.wasm_revec_analyzer_ = std::ptr::null_mut();
    }

    pub fn wasm_shuffle_analyzer(&self) -> *mut WasmShuffleAnalyzer {
        self.wasm_shuffle_analyzer_
    }

    pub fn set_wasm_shuffle_analyzer(&mut self, wasm_shuffle_analyzer: *mut WasmShuffleAnalyzer) {
        self.wasm_shuffle_analyzer_ = wasm_shuffle_analyzer;
    }

    pub fn clear_wasm_shuffle_analyzer(&mut self) {
        self.wasm_shuffle_analyzer_ = std::ptr::null_mut();
    }

    pub fn is_wasm(&self) -> bool {
        self.pipeline_kind() == TurboshaftPipelineKind::kWasm || self.pipeline_kind() == TurboshaftPipelineKind::kJSToWasm
    }
    pub fn is_js_to_wasm(&self) -> bool {
        self.pipeline_kind() == TurboshaftPipelineKind::kJSToWasm
    }

    pub fn InitializeFrameData(&mut self, call_descriptor: *mut CallDescriptor) {
        let cg = self.codegen_component_.as_mut().unwrap();
        let mut fixed_frame_size = 0;
        if !call_descriptor.is_null() {
            fixed_frame_size = unsafe { (*call_descriptor).CalculateFixedFrameSize(unsafe { (*self.info_).code_kind() }) };
        }
        cg.frame = cg.zone.New::<Frame>(&Zone {});
        if !cg.osr_helper.is_null() {
            unsafe { cg.osr_helper.as_ref().SetupFrame(cg.frame) };
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

pub struct TurbofanPipelineStatistics {}

pub struct ZoneStats {
    zone: Zone,
}

impl ZoneStats {
    pub fn new() -> Self {
        ZoneStats { zone: Zone::new() }
    }
}

pub mod std {
    pub mod unique_ptr {
        pub struct unique_ptr<T> {
            ptr: *mut T,
        }

        impl<T> unique_ptr<T> {
            pub fn new() -> Self {
                unique_ptr { ptr: std::ptr::null_mut() }
            }
            pub fn as_ptr(&self) -> *mut T {
                self.ptr
            }
        }
    }

    pub mod shared_ptr {
        pub struct SharedPtr<T> {
            ptr: *mut T,
        }

        impl<T> SharedPtr<T> {
            pub fn new() -> Self {
                SharedPtr { ptr: std::ptr::null_mut() }
            }
            pub fn as_ptr(&self) -> *mut T {
                self.ptr
            }
            pub fn is_null(&self) -> bool {
                self.ptr.is_null()
            }
            pub fn as_ref(&self) -> &T {
                unsafe { &*self.ptr }
            }
            pub fn clone(&self) -> Self {
                SharedPtr { ptr: self.ptr }
            }
        }
    }
}

pub mod wasm {
    pub struct FunctionSig {}
    pub struct CanonicalSig {}
    pub struct WasmModule {}

    pub fn GetWasmEngine() -> std::option::Option<WasmEngine> {
        std::option::Option::None
    }

    pub struct WasmEngine {}

    impl WasmEngine {
        pub fn allocator(&self) -> *mut super::AccountingAllocator {
            std::ptr::null_mut()
        }
        pub fn GetCodeTracer(&self) -> *mut super::CodeTracer {
            std::ptr::null_mut()
        }
    }
}

pub fn PrintTurboshaftGraph(
    data: *mut PipelineData,
    temp_zone: *mut Zone,
    code_tracer: *mut CodeTracer,
    phase_name: *const i8,
) {
    let data = unsafe { &mut *data };
    if !data.info().is_null() && unsafe { (*data.info()).trace_turbo_json() } {
        let mut scope = UnparkedScopeIfNeeded {};
        let mut allow_deref = AllowHandleDereference {};
        let graph = data.graph();

        let mut json_of = TurboJsonFile::new(data.info(), std::ios_base::app);
        PrintTurboshaftGraphForTurbolizer(
            &mut json_of,
            graph,
            unsafe { std::ffi
