// Converted from V8 C++ source files:
// Header: pipeline-data-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod compiler {
use std::sync::{Arc, Mutex};
#[allow(dead_code)]
use crate::builtins::profile_data_reader::ProfileDataFromFile;
use crate::codegen::assembler::AssemblerOptions;
use crate::codegen::optimized_compilation_info::OptimizedCompilationInfo;
use crate::common::globals::kNoSourcePosition;
use crate::compiler::backend::code_generator::CodeGenerator;
use crate::compiler::backend::instruction_selector::InstructionSelector;
use crate::compiler::backend::instruction::InstructionSequence;
use crate::compiler::backend::register_allocator::RegisterAllocationData;
use crate::compiler::common_operator::CommonOperatorBuilder;
use crate::compiler::compilation_dependencies::CompilationDependencies;
use crate::compiler::compiler_source_position_table::SourcePositionTable;
use crate::compiler::js_context_specialization::OuterContext;
use crate::compiler::js_heap_broker::JSHeapBroker;
use crate::compiler::js_operator::JSOperatorBuilder;
use crate::compiler::machine_graph::MachineGraph;
use crate::compiler::machine_operator::MachineOperatorBuilder;
use crate::compiler::node_observer::ObserveNodeManager;
use crate::compiler::node_origin_table::NodeOriginTable;
use crate::compiler::pipeline_statistics::TurbofanPipelineStatistics;
use crate::compiler::schedule::Schedule;
use crate::compiler::simplified_operator::SimplifiedOperatorBuilder;
use crate::compiler::turbofan_typer::Typer;
use crate::execution::isolate::Isolate;
use crate::handles::handles_inl::MaybeIndirectHandle;
use crate::objects::objects_inl::Context;
use crate::objects::objects_inl::JSGlobalObject;
use crate::zone::zone_containers::Zone;
use crate::compiler::js_graph::JSGraph;
use crate::codegen::register_configuration::RegisterConfiguration;
use crate::codegen::call_descriptor::CallDescriptor;
use crate::deoptimizer::osr::OsrHelper;
use crate::handles::handles_inl::DirectHandle;
use crate::handles::handles_inl::kHandleTableCapacity;
use crate::utils::allocation::AccountingAllocator;
use crate::wasm::wasm_engine::WasmEngine;

use std::cell::RefCell;
use crate::common::globals::Address;
use crate::objects::fixed_array::FixedArray;
use std::rc::Rc;

#[cfg(v8_enable_webassembly)]
use crate::wasm::wasm_module::WasmModule;
#[cfg(v8_enable_webassembly)]
use crate::compiler::js_wasm_calls_sidetable::JsWasmCallsSidetable;
use crate::codegen::frame::Frame;
use crate::codegen::linkage::Linkage;
use crate::utils::code_tracer::CodeTracer;
use crate::runtime::runtime_call_stats::RuntimeCallStats;
use crate::compiler::jump_optimization::JumpOptimizationInfo;

const kGraphZoneName: &str = "GraphZone";
const kInstructionZoneName: &str = "InstructionZone";
const kCodegenZoneName: &str = "CodegenZone";
const kRegisterAllocationZoneName: &str = "RegisterAllocationZone";
const kCompressGraphZone: bool = true;

thread_local! {
    static CURRENT_PHASE: RefCell<Option<String>> = RefCell::new(None);
}

struct PhaseScope<'a> {
    pipeline_statistics: Option<&'a mut TurbofanPipelineStatistics>,
    phase_name: String,
}

impl<'a> PhaseScope<'a> {
    fn new(pipeline_statistics: Option<&'a mut TurbofanPipelineStatistics>, phase_name: &str) -> Self {
        if let Some(stats) = pipeline_statistics {
            CURRENT_PHASE.with(|p| {
                *p.borrow_mut() = Some(phase_name.to_string());
            });
            stats.begin_phase_kind(phase_name);
        }
        PhaseScope {
            pipeline_statistics,
            phase_name: phase_name.to_string(),
        }
    }
}

impl<'a> Drop for PhaseScope<'a> {
    fn drop(&mut self) {
        if let Some(stats) = self.pipeline_statistics {
            stats.end_phase_kind();
            CURRENT_PHASE.with(|p| {
                *p.borrow_mut() = None;
            });
        }
    }
}

fn get_module_context(info: &mut OptimizedCompilationInfo) -> Result<OuterContext, String> {
    let mut current = info.closure().context();
    let mut distance: usize = 0;
    while !current.is_native_context() {
        if current.is_module_context() {
            return Ok(OuterContext::new(
                info.canonical_handle(&current), distance));
        }
        current = current.previous();
        distance += 1;
    }
    Err("Nothing".to_string())
}

pub struct ZoneStats {
}

impl ZoneStats {
    pub fn new() -> Self {
        ZoneStats {}
    }
}

pub struct TFPipelineData {
    isolate_: *mut Isolate,
    allocator_: *mut AccountingAllocator,
    info_: *mut OptimizedCompilationInfo,
    debug_name_: std::ffi::CString,
    may_have_unverifiable_graph_: bool,
    zone_stats_: *mut ZoneStats,
    pipeline_statistics_: *mut TurbofanPipelineStatistics,
    verify_graph_: bool,
    start_source_position_: i32,
    osr_helper_: Option<std::rc::Rc<OsrHelper>>,
    code_: MaybeIndirectHandle<Code>,
    code_generator_: *mut CodeGenerator,
    typer_: *mut Typer,
    typer_flags_: u32,

    graph_zone_: Box<Zone>,
    graph_: *mut TFGraph,
    source_positions_: *mut SourcePositionTable,
    node_origins_: *mut NodeOriginTable,
    simplified_: *mut SimplifiedOperatorBuilder,
    machine_: *mut MachineOperatorBuilder,
    common_: *mut CommonOperatorBuilder,
    javascript_: *mut JSOperatorBuilder,
    jsgraph_: *mut JSGraph,
    mcgraph_: *mut MachineGraph,
    schedule_: *mut Schedule,
    observe_node_manager_: *mut ObserveNodeManager,
    ts_data_: Option<Box<PipelineData>>,

    instruction_zone_scope_: ZoneStatsScope,
    instruction_zone_: *mut Zone,
    sequence_: *mut InstructionSequence,

    codegen_zone_scope_: ZoneStatsScope,
    codegen_zone_: *mut Zone,
    dependencies_: *mut CompilationDependencies,
    broker_: Option<Arc<JSHeapBroker>>,
    frame_: *mut Frame,

    register_allocation_zone_scope_: ZoneStatsScope,
    register_allocation_zone_: *mut Zone,
    register_allocation_data_: *mut RegisterAllocationData,

    source_position_output_: String,

    jump_optimization_info_: *mut JumpOptimizationInfo,
    assembler_options_: AssemblerOptions,
    specialization_context_: Result<OuterContext, String>,

    max_unoptimized_frame_height_: usize,
    max_pushed_argument_count_: usize,

    runtime_call_stats_: *mut RuntimeCallStats,
    profile_data_: *const ProfileDataFromFile,

    #[cfg(v8_enable_webassembly)]
    wasm_module_for_inlining_: *const WasmModule,
    #[cfg(v8_enable_webassembly)]
    js_wasm_calls_sidetable_: *mut JsWasmCallsSidetable,
}

impl TFPipelineData {
    pub fn new(zone_stats: *mut ZoneStats, isolate: *mut Isolate,
               info: *mut OptimizedCompilationInfo,
               pipeline_statistics: *mut TurbofanPipelineStatistics) -> Self {
        unsafe {
            let debug_name = std::ffi::CString::new((*info).get_debug_name()).unwrap();
            let allocator = (*isolate).allocator();
            let graph_zone = Box::new(Zone::new());
            let instruction_zone_scope = ZoneStatsScope::new();
            let instruction_zone = instruction_zone_scope.zone();
            let codegen_zone_scope = ZoneStatsScope::new();
            let codegen_zone = codegen_zone_scope.zone();
            let broker = Arc::new(JSHeapBroker::new(isolate, (*info).zone(),
                                                     (*info).trace_heap_broker(),
                                                     (*info).code_kind()));
            let register_allocation_zone_scope = ZoneStatsScope::new();
            let register_allocation_zone = register_allocation_zone_scope.zone();
            let assembler_options = AssemblerOptions::default(isolate);

            let mut pipeline_data = TFPipelineData {
                isolate_: isolate,
                allocator_: allocator,
                info_: info,
                debug_name_: debug_name,
                may_have_unverifiable_graph_: true,
                zone_stats_: zone_stats,
                pipeline_statistics_: pipeline_statistics,
                verify_graph_: false,
                start_source_position_: kNoSourcePosition,
                osr_helper_: None,
                code_: MaybeIndirectHandle::empty(),
                code_generator_: std::ptr::null_mut(),
                typer_: std::ptr::null_mut(),
                typer_flags_: 0,

                graph_zone_: graph_zone,
                graph_: std::ptr::null_mut(),
                source_positions_: std::ptr::null_mut(),
                node_origins_: std::ptr::null_mut(),
                simplified_: std::ptr::null_mut(),
                machine_: std::ptr::null_mut(),
                common_: std::ptr::null_mut(),
                javascript_: std::ptr::null_mut(),
                jsgraph_: std::ptr::null_mut(),
                mcgraph_: std::ptr::null_mut(),
                schedule_: std::ptr::null_mut(),
                observe_node_manager_: std::ptr::null_mut(),
                ts_data_: None,

                instruction_zone_scope_: instruction_zone_scope,
                instruction_zone_: instruction_zone,
                sequence_: std::ptr::null_mut(),

                codegen_zone_scope_: codegen_zone_scope,
                codegen_zone_: codegen_zone,
                dependencies_: std::ptr::null_mut(),
                broker_: Some(broker),
                frame_: std::ptr::null_mut(),

                register_allocation_zone_scope_: register_allocation_zone_scope,
                register_allocation_zone_: register_allocation_zone,
                register_allocation_data_: std::ptr::null_mut(),

                source_position_output_: String::new(),

                jump_optimization_info_: std::ptr::null_mut(),
                assembler_options_: assembler_options,
                specialization_context_: Err("".to_string()),

                max_unoptimized_frame_height_: 0,
                max_pushed_argument_count_: 0,

                runtime_call_stats_: std::ptr::null_mut(),
                profile_data_: std::ptr::null(),

                #[cfg(v8_enable_webassembly)]
                wasm_module_for_inlining_: std::ptr::null(),
                #[cfg(v8_enable_webassembly)]
                js_wasm_calls_sidetable_: std::ptr::null_mut(),
            };

            let graph = pipeline_data.graph_zone_.allocate(std::mem::size_of::<TFGraph>()) as *mut TFGraph;
            pipeline_data.graph_ = graph;
            let source_positions = pipeline_data.graph_zone_.allocate(std::mem::size_of::<SourcePositionTable>()) as *mut SourcePositionTable;
            pipeline_data.source_positions_ = source_positions;
            if (*info).trace_turbo_json() {
                let node_origins = pipeline_data.graph_zone_.allocate(std::mem::size_of::<NodeOriginTable>()) as *mut NodeOriginTable;
                pipeline_data.node_origins_ = node_origins;
            }
            #[cfg(v8_enable_webassembly)]
            {
                let js_wasm_calls_sidetable = pipeline_data.graph_zone_.allocate(std::mem::size_of::<JsWasmCallsSidetable>()) as *mut JsWasmCallsSidetable;
                pipeline_data.js_wasm_calls_sidetable_ = js_wasm_calls_sidetable;
            }
            let simplified = pipeline_data.graph_zone_.allocate(std::mem::size_of::<SimplifiedOperatorBuilder>()) as *mut SimplifiedOperatorBuilder;
            pipeline_data.simplified_ = simplified;
            let machine = pipeline_data.graph_zone_.allocate(std::mem::size_of::<MachineOperatorBuilder>()) as *mut MachineOperatorBuilder;
            pipeline_data.machine_ = machine;
            let common = pipeline_data.graph_zone_.allocate(std::mem::size_of::<CommonOperatorBuilder>()) as *mut CommonOperatorBuilder;
            pipeline_data.common_ = common;
            let javascript = pipeline_data.graph_zone_.allocate(std::mem::size_of::<JSOperatorBuilder>()) as *mut JSOperatorBuilder;
            pipeline_data.javascript_ = javascript;
            let jsgraph = pipeline_data.graph_zone_.allocate(std::mem::size_of::<JSGraph>()) as *mut JSGraph;
            pipeline_data.jsgraph_ = jsgraph;
            if (*info).node_observer() != std::ptr::null_mut() {
                let observe_node_manager = pipeline_data.graph_zone_.allocate(std::mem::size_of::<ObserveNodeManager>()) as *mut ObserveNodeManager;
                pipeline_data.observe_node_manager_ = observe_node_manager;
            }
            let dependencies = (*info).zone().allocate(std::mem::size_of::<CompilationDependencies>()) as *mut CompilationDependencies;
            pipeline_data.dependencies_ = dependencies;

            PhaseScope::new(Some(&mut *pipeline_statistics), "V8.TFInitPipelineData");

            pipeline_data
        }
    }

    #[cfg(v8_enable_webassembly)]
    pub fn new_wasm(zone_stats: *mut ZoneStats, wasm_engine: *mut WasmEngine,
                    info: *mut OptimizedCompilationInfo, mcgraph: *mut MachineGraph,
                    pipeline_statistics: *mut TurbofanPipelineStatistics,
                    source_positions: *mut SourcePositionTable,
                    node_origins: *mut NodeOriginTable,
                    assembler_options: &AssemblerOptions) -> Self {
        unsafe {
            let debug_name = std::ffi::CString::new((*info).get_debug_name()).unwrap();
            let allocator = (*wasm_engine).allocator();
            let graph = (*mcgraph).graph();
            let machine = (*mcgraph).machine();
            let common = (*mcgraph).common();
            let graph_zone = Box::new(Zone::new());
            let instruction_zone_scope = ZoneStatsScope::new();
            let instruction_zone = instruction_zone_scope.zone();
            let codegen_zone_scope = ZoneStatsScope::new();
            let codegen_zone = codegen_zone_scope.zone();
            let register_allocation_zone_scope = ZoneStatsScope::new();
            let register_allocation_zone = register_allocation_zone_scope.zone();

            let mut pipeline_data = TFPipelineData {
                isolate_: std::ptr::null_mut(),
                allocator_: allocator,
                info_: info,
                debug_name_: debug_name,
                may_have_unverifiable_graph_: true,
                zone_stats_: zone_stats,
                pipeline_statistics_: pipeline_statistics,
                verify_graph_: false,
                start_source_position_: kNoSourcePosition,
                osr_helper_: None,
                code_: MaybeIndirectHandle::empty(),
                code_generator_: std::ptr::null_mut(),
                typer_: std::ptr::null_mut(),
                typer_flags_: 0,

                graph_zone_: graph_zone,
                graph_: graph,
                source_positions_: source_positions,
                node_origins_: node_origins,
                simplified_: std::ptr::null_mut(),
                machine_: machine,
                common_: common,
                javascript_: std::ptr::null_mut(),
                jsgraph_: std::ptr::null_mut(),
                mcgraph_: mcgraph,
                schedule_: std::ptr::null_mut(),
                observe_node_manager_: std::ptr::null_mut(),
                ts_data_: None,

                instruction_zone_scope_: instruction_zone_scope,
                instruction_zone_: instruction_zone,
                sequence_: std::ptr::null_mut(),

                codegen_zone_scope_: codegen_zone_scope,
                codegen_zone_: codegen_zone,
                dependencies_: std::ptr::null_mut(),
                broker_: None,
                frame_: std::ptr::null_mut(),

                register_allocation_zone_scope_: register_allocation_zone_scope,
                register_allocation_zone_: register_allocation_zone,
                register_allocation_data_: std::ptr::null_mut(),

                source_position_output_: String::new(),

                jump_optimization_info_: std::ptr::null_mut(),
                assembler_options_: assembler_options.clone(),
                specialization_context_: Err("".to_string()),

                max_unoptimized_frame_height_: 0,
                max_pushed_argument_count_: 0,

                runtime_call_stats_: std::ptr::null_mut(),
                profile_data_: std::ptr::null(),

                wasm_module_for_inlining_: std::ptr::null(),
                js_wasm_calls_sidetable_: std::ptr::null_mut(),
            };

            let simplified = pipeline_data.graph_zone_.allocate(std::mem::size_of::<SimplifiedOperatorBuilder>()) as *mut SimplifiedOperatorBuilder;
            pipeline_data.simplified_ = simplified;
            let javascript = pipeline_data.graph_zone_.allocate(std::mem::size_of::<JSOperatorBuilder>()) as *mut JSOperatorBuilder;
            pipeline_data.javascript_ = javascript;
            let jsgraph = pipeline_data.graph_zone_.allocate(std::mem::size_of::<JSGraph>()) as *mut JSGraph;
            pipeline_data.jsgraph_ = jsgraph;

            pipeline_data
        }
    }

    pub fn new_csa(zone_stats: *mut ZoneStats, info: *mut OptimizedCompilationInfo,
                   isolate: *mut Isolate, allocator: *mut AccountingAllocator,
                   graph: *mut TFGraph, jsgraph: *mut JSGraph, schedule: *mut Schedule,
                   source_positions: *mut SourcePositionTable,
                   node_origins: *mut NodeOriginTable, jump_opt: *mut JumpOptimizationInfo,
                   assembler_options: &AssemblerOptions,
                   profile_data: *const ProfileDataFromFile) -> Self {
        unsafe {
            let debug_name = std::ffi::CString::new((*info).get_debug_name()).unwrap();
            let graph_zone = Box::new(Zone::new());
            let instruction_zone_scope = ZoneStatsScope::new();
            let instruction_zone = instruction_zone_scope.zone();
            let codegen_zone_scope = ZoneStatsScope::new();
            let codegen_zone = codegen_zone_scope.zone();
            let register_allocation_zone_scope = ZoneStatsScope::new();
            let register_allocation_zone = register_allocation_zone_scope.zone();

            let mut pipeline_data = TFPipelineData {
                isolate_: isolate,
                allocator_: allocator,
                info_: info,
                debug_name_: debug_name,
                may_have_unverifiable_graph_: false,
                zone_stats_: zone_stats,
                pipeline_statistics_: std::ptr::null_mut(),
                verify_graph_: false,
                start_source_position_: kNoSourcePosition,
                osr_helper_: None,
                code_: MaybeIndirectHandle::empty(),
                code_generator_: std::ptr::null_mut(),
                typer_: std::ptr::null_mut(),
                typer_flags_: 0,

                graph_zone_: graph_zone,
                graph_: graph,
                source_positions_: source_positions,
                node_origins_: node_origins,
                simplified_: std::ptr::null_mut(),
                machine_: std::ptr::null_mut(),
                common_: std::ptr::null_mut(),
                javascript_: std::ptr::null_mut(),
                jsgraph_: jsgraph,
                mcgraph_: std::ptr::null_mut(),
                schedule_: schedule,
                observe_node_manager_: std::ptr::null_mut(),
                ts_data_: None,

                instruction_zone_scope_: instruction_zone_scope,
                instruction_zone_: instruction_zone,
                sequence_: std::ptr::null_mut(),

                codegen_zone_scope_: codegen_zone_scope,
                codegen_zone_: codegen_zone,
                dependencies_: std::ptr::null_mut(),
                broker_: None,
                frame_: std::ptr::null_mut(),

                register_allocation_zone_scope_: register_allocation_zone_scope,
                register_allocation_zone_: register_allocation_zone,
                register_allocation_data_: std::ptr::null_mut(),

                source_position_output_: String::new(),

                jump_optimization_info_: jump_opt,
                assembler_options_: assembler_options.clone(),
                specialization_context_: Err("".to_string()),

                max_unoptimized_frame_height_: 0,
                max_pushed_argument_count_: 0,

                runtime_call_stats_: std::ptr::null_mut(),
                profile_data_: profile_data,

                #[cfg(v8_enable_webassembly)]
                wasm_module_for_inlining_: std::ptr::null(),
                #[cfg(v8_enable_webassembly)]
                js_wasm_calls_sidetable_: std::ptr::null_mut(),
            };

            if jsgraph == std::ptr::null_mut() {
                let simplified = pipeline_data.graph_zone_.allocate(std::mem::size_of::<SimplifiedOperatorBuilder>()) as *mut SimplifiedOperatorBuilder;
                pipeline_data.simplified_ = simplified;
                let machine = pipeline_data.graph_zone_.allocate(std::mem::size_of::<MachineOperatorBuilder>()) as *mut MachineOperatorBuilder;
                pipeline_data.machine_ = machine;
                let common = pipeline_data.graph_zone_.allocate(std::mem::size_of::<CommonOperatorBuilder>()) as *mut CommonOperatorBuilder;
                pipeline_data.common_ = common;
                let javascript = pipeline_data.graph_zone_.allocate(std::mem::size_of::<JSOperatorBuilder>()) as *mut JSOperatorBuilder;
                pipeline_data.javascript_ = javascript;
                let jsgraph = pipeline_data.graph_zone_.allocate(std::mem::size_of::<JSGraph>()) as *mut JSGraph;
                pipeline_data.jsgraph_ = jsgraph;
            }
            pipeline_data
        }
    }

    pub fn new_regalloc(zone_stats: *mut ZoneStats, info: *mut OptimizedCompilationInfo,
                       isolate: *mut Isolate, sequence: *mut InstructionSequence) -> Self {
        unsafe {
            let debug_name = std::ffi::CString::new((*info).get_debug_name()).unwrap();
            let allocator = (*isolate).allocator();
            let graph_zone = Box::new(Zone::new());
            let instruction_zone_scope = ZoneStatsScope::new();
            let codegen_zone_scope = ZoneStatsScope::new();
            let codegen_zone = codegen_zone_scope.zone();
            let register_allocation_zone_scope = ZoneStatsScope::new();
            let register_allocation_zone = register_allocation_zone_scope.zone();
            let assembler_options = AssemblerOptions::default(isolate);

            TFPipelineData {
                isolate_: isolate,
                allocator_: allocator,
                info_: info,
                debug_name_: debug_name,
                may_have_unverifiable_graph_: false,
                zone_stats_: zone_stats,
                pipeline_statistics_: std::ptr::null_mut(),
                verify_graph_: false,
                start_source_position_: kNoSourcePosition,
                osr_helper_: None,
                code_: MaybeIndirectHandle::empty(),
                code_generator_: std::ptr::null_mut(),
                typer_: std::ptr::null_mut(),
                typer_flags_: 0,

                graph_zone_: graph_zone,
                graph_: std::ptr::null_mut(),
                source_positions_: std::ptr::null_mut(),
                node_origins_: std::ptr::null_mut(),
                simplified_: std::ptr::null_mut(),
                machine_: std::ptr::null_mut(),
                common_: std::ptr::null_mut(),
                javascript_: std::ptr::null_mut(),
                jsgraph_: std::ptr::null_mut(),
                mcgraph_: std::ptr::null_mut(),
                schedule_: std::ptr::null_mut(),
                observe_node_manager_: std::ptr::null_mut(),
                ts_data_: None,

                instruction_zone_scope_: instruction_zone_scope,
                instruction_zone_: (*sequence).zone(),
                sequence_: sequence,

                codegen_zone_scope_: codegen_zone_scope,
                codegen_zone_: codegen_zone,
                dependencies_: std::ptr::null_mut(),
                broker_: None,
                frame_: std::ptr::null_mut(),

                register_allocation_zone_scope_: register_allocation_zone_scope,
                register_allocation_zone_: register_allocation_zone,
                register_allocation_data_: std::ptr::null_mut(),

                source_position_output_: String::new(),

                jump_optimization_info_: std::ptr::null_mut(),
                assembler_options_: assembler_options,
                specialization_context_: Err("".to_string()),

                max_unoptimized_frame_height_: 0,
                max_pushed_argument_count_: 0,

                runtime_call_stats_: std::ptr::null_mut(),
                profile_data_: std::ptr::null(),

                #[cfg(v8_enable_webassembly)]
                wasm_module_for_inlining_: std::ptr::null(),
                #[cfg(v8_enable_webassembly)]
                js_wasm_calls_sidetable_: std::ptr::null_mut(),
            }
        }
    }

    pub fn isolate(&self) -> *mut Isolate {
        self.isolate_
    }

    pub fn allocator(&self) -> *mut AccountingAllocator {
        self.allocator_
    }

    pub fn info(&self) -> *mut OptimizedCompilationInfo {
        self.info_
    }

    pub fn zone_stats(&self) -> *mut ZoneStats {
        self.zone_stats_
    }

    pub fn dependencies(&self) -> *mut CompilationDependencies {
        self.dependencies_
    }

    pub fn pipeline_statistics(&mut self) -> *mut TurbofanPipelineStatistics {
        self.pipeline_statistics_
    }

    pub fn osr_helper(&mut self) -> Option<&mut OsrHelper> {
        self.osr_helper_.as_mut().map(|rc| Rc::get_mut(rc).unwrap())
    }

    pub fn osr_helper_ptr(&self) -> Option<std::rc::Rc<OsrHelper>> {
        self.osr_helper_.clone()
    }

    pub fn verify_graph(&self) -> bool {
        self.verify_graph_
    }

    pub fn set_verify_graph(&mut self, value: bool) {
        self.verify_graph_ = value;
    }

    pub fn code(&self) -> MaybeIndirectHandle<Code> {
        self.code_
    }

    pub fn set_code(&mut self, code: MaybeIndirectHandle<Code>) {
        assert!(self.code_.is_null());
        self.code_ = code;
    }

    pub fn code_generator(&self) -> *mut CodeGenerator {
        self.code_generator_
    }

    pub fn may_have_unverifiable_graph(&self) -> bool {
        self.may_have_unverifiable_graph_
    }

    pub fn graph_zone(&mut self) -> &mut Zone {
        self.graph_zone_.as_mut()
    }

    pub fn graph(&self) -> *mut TFGraph {
        self.graph_
    }

    pub fn set_graph(&mut self, graph: *mut TFGraph) {
        self.graph_ = graph;
    }

    pub fn source_positions(&self) -> *mut SourcePositionTable {
        self.source_positions_
    }

    pub fn set_source_positions(&mut self, source_positions: *mut SourcePositionTable) {
        self.source_positions_ = source_positions;
    }

    pub fn node_origins(&self) -> *mut NodeOriginTable {
        self.node_origins_
    }

    pub fn set_node_origins(&mut self, node_origins: *mut NodeOriginTable) {
        self.node_origins_ = node_origins;
    }

    pub fn machine(&self) -> *mut MachineOperatorBuilder {
        self.machine_
    }

    pub fn simplified(&self) -> *mut SimplifiedOperatorBuilder {
        self.simplified_
    }

    pub fn common(&self) -> *mut CommonOperatorBuilder {
        self.common_
    }

    pub fn javascript(&self) -> *mut JSOperatorBuilder {
        self.javascript_
    }

    pub fn jsgraph(&self) -> *mut JSGraph {
        self.jsgraph_
    }

    pub fn mcgraph(&self) -> *mut MachineGraph {
        self.mcgraph_
    }

    pub fn native_context(&self) -> DirectHandle<NativeContext> {
        unsafe {
            DirectHandle::new((*self.info_).native_context(), self.isolate_)
        }
    }

    pub fn global_object(&self) -> DirectHandle<JSGlobalObject> {
        unsafe {
            DirectHandle::new((*self.info_).global_object(), self.isolate_)
        }
    }

    pub fn broker(&self) -> Option<&JSHeapBroker> {
        self.broker_.as_ref().map(|broker| Arc::as_ref(broker))
    }

    pub fn broker_ptr(&self) -> Option<Arc<JSHeapBroker>> {
        self.broker_.clone()
    }

    pub fn schedule(&self) -> *mut Schedule {
        self.schedule_
    }

    pub fn set_schedule(&mut self, schedule: *mut Schedule) {
        assert!(self.schedule_ == std::ptr::null_mut());
        self.schedule_ = schedule;
    }

    pub fn reset_schedule(&mut self) {
        self.schedule_ = std::ptr::null_mut();
    }

    pub fn observe_node_manager(&self) -> *mut ObserveNodeManager {
        self.observe_node_manager_
    }

    pub fn instruction_zone(&self) -> *mut Zone {
        self.instruction_zone_
    }

    pub fn codegen_zone(&self) -> *mut Zone {
        self.codegen_zone_
    }

    pub fn sequence(&self) -> *mut InstructionSequence {
        self.sequence_
    }

    pub fn frame(&self) -> *mut Frame {
        self.frame_
    }

    pub fn register_allocation_zone(&self) -> *mut Zone {
        self.register_allocation_zone_
    }

    pub fn register_allocation_data(&self) -> *mut RegisterAllocationData {
        self.register_allocation_data_
    }

    pub fn source_position_output(&self) -> &String {
        &self.source_position_output_
    }

    pub fn set_source_position_output(&mut self, source_position_output: String) {
        self.source_position_output_ = source_position_output;
    }

    pub fn jump_optimization_info(&self) -> *mut JumpOptimizationInfo {
        self.jump_optimization_info_
    }

    pub fn assembler_options(&self) -> &AssemblerOptions {
        &self.assembler_options_
    }

    pub fn choose_specialization_context(&mut self) {
        unsafe {
            if (*self.info_).function_context_specializing() {
                assert!((*self.info_).has_context());
                self.specialization_context_ = Ok(OuterContext::new(
                    (*self.info_).canonical_handle((*self.info_).context()), 0));
            } else {
                self.specialization_context_ = get_module_context(&mut *self.info_);
            }
        }
    }

    pub fn specialization_context(&self) -> &Result<OuterContext, String> {
        &self.specialization_context_
    }

    pub fn address_of_max_unoptimized_frame_height(&mut self) -> *mut usize {
        &mut self.max_unoptimized_frame_height_ as *mut usize
    }

    pub fn max_unoptimized_frame_height(&self) -> usize {
        self.max_unoptimized_frame_height_
    }

    pub fn address_of_max_pushed_argument_count(&mut self) -> *mut usize {
        &mut self.max_pushed_argument_count_ as *mut usize
    }

    pub fn max_pushed_argument_count(&self) -> usize {
        self.max_pushed_argument_count_
    }

    pub fn get_code_tracer(&self) -> *mut CodeTracer {
        #[cfg(v8_enable_webassembly)]
        unsafe {
            if (*self.info_).is_wasm() || (*self.info_).is_wasm_builtin() {
                return WasmEngine::get_wasm_engine().get_code_tracer();
            }
        }
        unsafe { (*self.isolate_).get_code_tracer() }
    }

    pub fn create_typer(&mut
