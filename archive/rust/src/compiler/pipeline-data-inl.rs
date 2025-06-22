// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod pipeline_data {
    use std::any::Any;
    use std::cell::{Cell, RefCell};
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::sync::Arc;

    //use crate::builtins::profile_data_reader::ProfileDataReader; // Assuming a Rust equivalent exists
    //use crate::codegen::assembler::Assembler; // Assuming a Rust equivalent exists
    //use crate::codegen::optimized_compilation_info::OptimizedCompilationInfo; // Assuming a Rust equivalent exists
    //use crate::common::globals::v8_flags; // Assuming a Rust equivalent exists
    //use crate::compiler::backend::code_generator::CodeGenerator; // Assuming a Rust equivalent exists
    //use crate::compiler::backend::instruction_selector::InstructionSelector; // Assuming a Rust equivalent exists
    //use crate::compiler::backend::instruction::Instruction; // Assuming a Rust equivalent exists
    //use crate::compiler::backend::register_allocator::RegisterAllocator; // Assuming a Rust equivalent exists
    //use crate::compiler::common_operator::CommonOperatorBuilder; // Assuming a Rust equivalent exists
    //use crate::compiler::compilation_dependencies::CompilationDependencies; // Assuming a Rust equivalent exists
    //use crate::compiler::compiler_source_position_table::SourcePositionTable; // Assuming a Rust equivalent exists
    //use crate::compiler::js_context_specialization::OuterContext; // Assuming a Rust equivalent exists
    //use crate::compiler::js_heap_broker::JSHeapBroker; // Assuming a Rust equivalent exists
    //use crate::compiler::js_inlining::JsInlining; // Assuming a Rust equivalent exists
    //use crate::compiler::js_operator::JSOperatorBuilder; // Assuming a Rust equivalent exists
    //use crate::compiler::machine_graph::MachineGraph; // Assuming a Rust equivalent exists
    //use crate::compiler::machine_operator::MachineOperatorBuilder; // Assuming a Rust equivalent exists
    //use crate::compiler::node_observer::NodeObserver; // Assuming a Rust equivalent exists
    //use crate::compiler::node_origin_table::NodeOriginTable; // Assuming a Rust equivalent exists
    //use crate::compiler::phase::PhaseScope; // Assuming a Rust equivalent exists
    //use crate::compiler::pipeline_statistics::TurbofanPipelineStatistics; // Assuming a Rust equivalent exists
    //use crate::compiler::schedule::Schedule; // Assuming a Rust equivalent exists
    //use crate::compiler::simplified_operator::SimplifiedOperatorBuilder; // Assuming a Rust equivalent exists
    //use crate::compiler::turbofan_typer::Typer; // Assuming a Rust equivalent exists
    //use crate::compiler::turboshaft::phase::TurboshaftPhase; // Assuming a Rust equivalent exists
    //use crate::compiler::turboshaft::zone_with_name::ZoneWithName; // Assuming a Rust equivalent exists
    //use crate::compiler::zone_stats::ZoneStats; // Assuming a Rust equivalent exists
    //use crate::execution::isolate::Isolate; // Assuming a Rust equivalent exists
    //use crate::handles::handles_inl::MaybeIndirectHandle; // Assuming a Rust equivalent exists
    //use crate::objects::objects_inl::Context; // Assuming a Rust equivalent exists

    //#[cfg(V8_ENABLE_WEBASSEMBLY)]
    //use crate::wasm::wasm_engine::WasmEngine; // Assuming a Rust equivalent exists

    const K_GRAPH_ZONE_NAME: &str = "GraphZone";
    const K_INSTRUCTION_ZONE_NAME: &str = "InstructionZone";
    const K_CODEGEN_ZONE_NAME: &str = "CodegenZone";
    const K_REGISTER_ALLOCATION_ZONE_NAME: &str = "RegisterAllocationZone";
    const K_COMPRESS_GRAPH_ZONE: bool = true;
    const K_NO_SOURCE_POSITION: i32 = -1;

    // Placeholder structs and enums for types used in the original C++ code.
    // Replace these with actual Rust equivalents.
    pub struct OptimizedCompilationInfo {}
    impl OptimizedCompilationInfo {
        pub fn GetDebugName(&self) -> std::unique_ptr<[u8]> {
            std::unique_ptr::new(b"DebugName".to_vec().into_boxed_slice())
        }
         pub fn closure(&self) -> &Closure {
            unimplemented!()
        }
        pub fn context(&self) -> &Context {
            unimplemented!()
        }

        pub fn CanonicalHandle<T>(&self, current: &T, isolate: &Isolate) -> &T {
            unimplemented!()
        }

        pub fn native_context(&self) -> &NativeContext {
            unimplemented!()
        }
        pub fn global_object(&self) -> &JSGlobalObject {
            unimplemented!()
        }
        pub fn code_kind(&self) -> i32 {
            unimplemented!()
        }
         pub fn trace_heap_broker(&self) -> bool {
            unimplemented!()
        }
         pub fn trace_turbo_json(&self) -> bool {
            unimplemented!()
        }
        pub fn node_observer(&self) -> Option<&NodeObserver> {
            unimplemented!()
        }
        pub fn zone(&self) -> &Zone {
            unimplemented!()
        }
        pub fn function_context_specializing(&self) -> bool {
            unimplemented!()
        }
        pub fn has_context(&self) -> bool {
            unimplemented!()
        }

         pub fn tick_counter(&self) -> &TickCounter {
            unimplemented!()
         }
         pub fn builtin(&self) -> bool {
            unimplemented!()
         }
        pub fn IsWasm(&self) -> bool {
            unimplemented!()
        }
        pub fn IsWasmBuiltin(&self) -> bool {
            unimplemented!()
        }

    }

    pub struct Context {}
    impl Context {
        pub fn IsModuleContext(&self) -> bool {
            unimplemented!()
        }
        pub fn previous(&self) -> &Context {
            unimplemented!()
        }

        pub fn GetIsolate(&self) -> &Isolate {
            unimplemented!()
        }
    }
    pub struct NativeContext {}
    pub struct JSGlobalObject {}
    pub struct Closure {}
    pub struct TickCounter {}

    pub struct OuterContext {
        pub handle: Box<Context>,
        pub distance: usize,
    }
    impl OuterContext {
        pub fn new(handle: Box<Context>, distance: usize) -> Self {
            OuterContext { handle, distance }
        }
    }
    pub struct ZoneStats {}
    pub struct Isolate {}
    pub struct AccountingAllocator {}
    pub struct TurbofanPipelineStatistics {}
    pub struct Zone {}
    pub struct TFGraph {}
    pub struct SourcePositionTable {}
    pub struct NodeOriginTable {}
    pub struct SimplifiedOperatorBuilder {}
    pub struct MachineOperatorBuilder {}
    pub struct CommonOperatorBuilder {}
    pub struct JSOperatorBuilder {}
    pub struct JSGraph {}
    pub struct ObserveNodeManager {}
    pub struct CompilationDependencies {}
    pub struct Schedule {}
    pub struct InstructionSequence {}
    pub struct Frame {}
    pub struct RegisterAllocationData {}
    pub struct JumpOptimizationInfo {}
    pub struct AssemblerOptions {}
    pub struct ProfileDataFromFile {}
    pub struct Code {}
    pub struct CallDescriptor {}
    pub struct InstructionBlocks {}
    pub struct RegisterConfiguration {}
    pub struct CodeGenerator {}
    pub struct Linkage {}
    pub struct OsrHelper {}
    pub struct MachineGraph {}
    pub struct JsWasmCallsSidetable {}

    //#[cfg(V8_ENABLE_WEBASSEMBLY)]
    pub mod wasm {
        pub struct WasmEngine {}
        pub struct WasmModule {}
    }

    impl AssemblerOptions {
        pub fn Default(_isolate: &Isolate) -> Self {
            AssemblerOptions {}
        }
    }

    impl InstructionSequence {
        pub fn InstructionBlocksFor(_zone: &Zone, _schedule: &Schedule) -> *mut InstructionBlocks {
            unimplemented!()
        }

        pub fn instruction_blocks(&self) -> &[*mut InstructionBlocks] {
            unimplemented!()
        }

        pub fn zone(&self) -> &Zone {
            unimplemented!()
        }
    }

    impl InstructionBlocks {
        pub fn mark_needs_frame(&mut self) {
            unimplemented!()
        }
    }

     impl CallDescriptor {
        pub fn RequiresFrameAsIncoming(&self) -> bool {
            unimplemented!()
        }
        pub fn CalleeSavedFPRegisters(&self) -> FPRegisters {
            unimplemented!()
        }
        pub fn CalculateFixedFrameSize(&self, code_kind: i32) -> i32 {
            unimplemented!()
        }
    }

    pub struct FPRegisters {
        
    }
    impl FPRegisters {
        pub fn is_empty(&self) -> bool {
            unimplemented!()
        }
    }
    pub struct Typer {
        flags: TyperFlags,
    }
    #[derive(Clone, Copy)]
    pub enum TyperFlags {
        kNoFlags,
        Flag
    }
    impl Typer {
        pub fn new(_broker: &JSHeapBroker, _flags: TyperFlags, _graph: &TFGraph, _tick_counter: &TickCounter) -> Self {
            Typer {
                flags: TyperFlags::kNoFlags,
            }
        }

    }
     pub struct CodeTracer {}
    impl Isolate {
        pub fn GetCodeTracer(&self) -> &CodeTracer {
            unimplemented!()
        }
    }
    pub struct RuntimeCallStats {}
    impl ModuleContext {
        pub fn new(outer_context: OuterContext) -> Self {
            ModuleContext {
                outer_context
            }
        }
    }
    pub struct ModuleContext {
        outer_context: OuterContext
    }
    pub fn IsNativeContext(_context: &Context) -> bool {
        unimplemented!()
    }
    pub type Maybe<T> = Option<T>;
    pub fn Just<T>(value: T) -> Maybe<T> {
        Some(value)
    }

    pub fn Nothing<T>() -> Maybe<T> {
        None
    }

    pub fn GetModuleContext(info: &OptimizedCompilationInfo) -> Maybe<OuterContext> {
        let mut current = info.closure().context();
        let mut distance = 0;
        while !IsNativeContext(current) {
            if current.IsModuleContext() {
                return Just(OuterContext::new(
                    Box::new(Context{}), distance)); //info.CanonicalHandle(current, current.GetIsolate()), distance));
            }
            current = current.previous();
            distance += 1;
        }
        Nothing()
    }
    
    #[derive(Debug)]
    pub struct UniquePtr<T> {
        ptr: *mut T,
        _marker: std::marker::PhantomData<T>,
    }
    
    impl<T> UniquePtr<T> {
        pub fn new(ptr: *mut T) -> Self {
            UniquePtr {
                ptr,
                _marker: std::marker::PhantomData,
            }
        }
        pub fn get(&self) -> *mut T {
            self.ptr
        }
        pub fn as_ref(&self) -> Option<&T> {
            if self.ptr.is_null() {
                None
            } else {
                unsafe { Some(&*self.ptr) }
            }
        }
    }
    
    impl<T> Drop for UniquePtr<T> {
        fn drop(&mut self) {
            // This may cause double free if not correctly handled
            // For cases where UniquePtr holds a pointer to V8's internal objects.
            //if !self.ptr.is_null() {
            //    unsafe {
            //        drop(Box::from_raw(self.ptr));
            //    }
            //}
        }
    }
    
    unsafe impl<T: Send> Send for UniquePtr<T> {}
    unsafe impl<T: Sync> Sync for UniquePtr<T> {}

    pub struct TFPipelineData {
        isolate: Option<&'static Isolate>, // Assuming Isolate has a static lifetime
        allocator: *mut AccountingAllocator,
        info: *mut OptimizedCompilationInfo,
        debug_name: std::unique_ptr<[u8]>,
        may_have_unverifiable_graph: bool,
        zone_stats: *mut ZoneStats,
        pipeline_statistics: *mut TurbofanPipelineStatistics,
        graph_zone: GraphZone,
        instruction_zone_scope: ZoneStatsScope,
        instruction_zone: *mut Zone,
        codegen_zone_scope: ZoneStatsScope,
        codegen_zone: *mut Zone,
        broker: Option<Box<JSHeapBroker>>,
        register_allocation_zone_scope: ZoneStatsScope,
        register_allocation_zone: *mut Zone,
        assembler_options: AssemblerOptions,
        graph: *mut TFGraph,
        source_positions: *mut SourcePositionTable,
        node_origins: *mut NodeOriginTable,
        simplified: *mut SimplifiedOperatorBuilder,
        machine: *mut MachineOperatorBuilder,
        common: *mut CommonOperatorBuilder,
        javascript: *mut JSOperatorBuilder,
        jsgraph: *mut JSGraph,
        mcgraph: *mut MachineGraph,
        observe_node_manager: *mut ObserveNodeManager,
        dependencies: *mut CompilationDependencies,
        schedule: *mut Schedule,
        sequence: *mut InstructionSequence,
        frame: *mut Frame,
        register_allocation_data: *mut RegisterAllocationData,
        source_position_output: String,
        jump_optimization_info: *mut JumpOptimizationInfo,
        specialization_context: Maybe<OuterContext>,
        max_unoptimized_frame_height: usize,
        max_pushed_argument_count: usize,
        runtime_call_stats: *mut RuntimeCallStats,
        profile_data: *mut ProfileDataFromFile,
        code: MaybeIndirectHandle<Code>,
        code_generator: *mut CodeGenerator,
         typer_: *mut Typer,
        typer_flags_: TyperFlags,
        osr_helper_: Option<std::shared_ptr::Weak<OsrHelper>>,

        //#[cfg(V8_ENABLE_WEBASSEMBLY)]
        wasm_module_for_inlining_: *const wasm::WasmModule,
        //#[cfg(V8_ENABLE_WEBASSEMBLY)]
        js_wasm_calls_sidetable_: *mut JsWasmCallsSidetable,
        
        start_source_position_: i32
    }

    //Implement Drop for properly cleaning the heap after usage
    impl Drop for TFPipelineData {
        fn drop(&mut self) {
            unsafe {
                 //delete code_generator_;
                 if !self.code_generator.is_null() {
                    drop(Box::from_raw(self.code_generator));
                    self.code_generator = std::ptr::null_mut();
                }

                //DeleteTyper();
                if !self.typer_.is_null() {
                    drop(Box::from_raw(self.typer_));
                    self.typer_ = std::ptr::null_mut();
                }

                 //DeleteRegisterAllocationZone();
                if !self.register_allocation_zone.is_null() {
                    self.register_allocation_zone_scope.Destroy();
                    self.register_allocation_zone = std::ptr::null_mut();
                    self.register_allocation_data = std::ptr::null_mut();
                }

                //DeleteInstructionZone();
                if !self.instruction_zone.is_null() {
                    self.instruction_zone_scope.Destroy();
                    self.instruction_zone = std::ptr::null_mut();
                    self.sequence = std::ptr::null_mut();
                }

                //DeleteCodegenZone();
                if !self.codegen_zone.is_null() {
                    self.codegen_zone_scope.Destroy();
                    self.codegen_zone = std::ptr::null_mut();
                    self.dependencies = std::ptr::null_mut();
                    self.broker.take(); //delete broker_.reset();
                    self.frame = std::ptr::null_mut();
                }

                //DeleteGraphZone();
                //#[cfg(V8_ENABLE_WEBASSEMBLY)]
                self.js_wasm_calls_sidetable_ = std::ptr::null_mut();
                self.graph = std::ptr::null_mut();
                self.source_positions = std::ptr::null_mut();
                self.node_origins = std::ptr::null_mut();
                self.simplified = std::ptr::null_mut();
                self.machine = std::ptr::null_mut();
                self.common = std::ptr::null_mut();
                self.javascript = std::ptr::null_mut();
                self.jsgraph = std::ptr::null_mut();
                self.mcgraph = std::ptr::null_mut();
                self.schedule = std::ptr::null_mut();
                self.graph_zone.Destroy();
            }
        }
    }

    impl TFPipelineData {
        pub fn new(
            zone_stats: *mut ZoneStats,
            isolate: &'static Isolate,
            info: *mut OptimizedCompilationInfo,
            pipeline_statistics: *mut TurbofanPipelineStatistics,
        ) -> Self {
            //let _scope = PhaseScope::new(pipeline_statistics, "V8.TFInitPipelineData"); // Assuming PhaseScope can be constructed like this
            let info_ref = unsafe { &*info };
            let debug_name = info_ref.GetDebugName();

            let graph_zone = GraphZone::new(zone_stats, K_GRAPH_ZONE_NAME, K_COMPRESS_GRAPH_ZONE);
            let instruction_zone_scope = ZoneStatsScope::new(zone_stats, K_INSTRUCTION_ZONE_NAME);
            let instruction_zone = instruction_zone_scope.zone();
            let codegen_zone_scope = ZoneStatsScope::new(zone_stats, K_CODEGEN_ZONE_NAME);
            let codegen_zone = codegen_zone_scope.zone();
            let broker = Some(Box::new(JSHeapBroker::new(isolate, unsafe { &*info }.zone(), unsafe { &*info }.trace_heap_broker(), unsafe { &*info }.code_kind())));
            let register_allocation_zone_scope = ZoneStatsScope::new(zone_stats, K_REGISTER_ALLOCATION_ZONE_NAME);
            let register_allocation_zone = register_allocation_zone_scope.zone();
            let assembler_options = AssemblerOptions::Default(isolate);

            let graph = graph_zone.New::<TFGraph>();
            let source_positions = graph_zone.New::<SourcePositionTable>();
            let node_origins = if unsafe { &*info }.trace_turbo_json() {
                graph_zone.New::<NodeOriginTable>()
            } else {
                std::ptr::null_mut()
            };

            //#[cfg(V8_ENABLE_WEBASSEMBLY)]
            let js_wasm_calls_sidetable = graph_zone.New::<JsWasmCallsSidetable>();

            let simplified = graph_zone.New::<SimplifiedOperatorBuilder>();
            let machine = graph_zone.New::<MachineOperatorBuilder>();
            let common = graph_zone.New::<CommonOperatorBuilder>();
            let javascript = graph_zone.New::<JSOperatorBuilder>();
            let jsgraph = graph_zone.New::<JSGraph>();
            let observe_node_manager = match unsafe { &*info }.node_observer() {
                Some(_) => graph_zone.New::<ObserveNodeManager>(),
                None => std::ptr::null_mut(),
            };
            let dependencies = unsafe { &*info }.zone().New::<CompilationDependencies>();
            
            TFPipelineData {
                isolate: Some(isolate),
                allocator: isolate as *const Isolate as *mut AccountingAllocator,
                info,
                debug_name,
                may_have_unverifiable_graph: true,
                zone_stats,
                pipeline_statistics,
                graph_zone,
                instruction_zone_scope,
                instruction_zone,
                codegen_zone_scope,
                codegen_zone,
                broker,
                register_allocation_zone_scope,
                register_allocation_zone,
                assembler_options,
                graph,
                source_positions,
                node_origins,
                simplified,
                machine,
                common,
                javascript,
                jsgraph,
                mcgraph: std::ptr::null_mut(), // Correct initialization for mcgraph
                observe_node_manager,
                dependencies,
                schedule: std::ptr::null_mut(),
                sequence: std::ptr::null_mut(),
                frame: std::ptr::null_mut(),
                register_allocation_data: std::ptr::null_mut(),
                source_position_output: String::new(),
                jump_optimization_info: std::ptr::null_mut(),
                specialization_context: Nothing(),
                max_unoptimized_frame_height: 0,
                max_pushed_argument_count: 0,
                runtime_call_stats: std::ptr::null_mut(),
                profile_data: std::ptr::null_mut(),
                code: Nothing(),
                code_generator: std::ptr::null_mut(),
                 typer_: std::ptr::null_mut(),
                typer_flags_: TyperFlags::kNoFlags,
                osr_helper_: None,

                //#[cfg(V8_ENABLE_WEBASSEMBLY)]
                wasm_module_for_inlining_: std::ptr::null(),
                //#[cfg(V8_ENABLE_WEBASSEMBLY)]
                js_wasm_calls_sidetable_: js_wasm_calls_sidetable,
                start_source_position_: K_NO_SOURCE_POSITION
            }
        }

        //#[cfg(V8_ENABLE_WEBASSEMBLY)]
        pub fn new_wasm(
            zone_stats: *mut ZoneStats,
            wasm_engine: *mut wasm::WasmEngine,
            info: *mut OptimizedCompilationInfo,
            mcgraph: *mut MachineGraph,
            pipeline_statistics: *mut TurbofanPipelineStatistics,
            source_positions: *mut SourcePositionTable,
            node_origins: *mut NodeOriginTable,
            assembler_options: &AssemblerOptions,
        ) -> Self {
            let info_ref = unsafe { &*info };
            let debug_name = info_ref.GetDebugName();
            let graph = unsafe { &*mcgraph }.graph();
            let machine = unsafe { &*mcgraph }.machine();
            let common = unsafe { &*mcgraph }.common();

            let graph_zone = GraphZone::new(zone_stats, K_GRAPH_ZONE_NAME, K_COMPRESS_GRAPH_ZONE);
            let instruction_zone_scope = ZoneStatsScope::new(zone_stats, K_INSTRUCTION_ZONE_NAME);
            let instruction_zone = instruction_zone_scope.zone();
            let codegen_zone_scope = ZoneStatsScope::new(zone_stats, K_CODEGEN_ZONE_NAME);
            let codegen_zone = codegen_zone_scope.zone();
            let register_allocation_zone_scope = ZoneStatsScope::new(zone_stats, K_REGISTER_ALLOCATION_ZONE_NAME);
            let register_allocation_zone = register_allocation_zone_scope.zone();

            let simplified = graph_zone.New::<SimplifiedOperatorBuilder>();
            let javascript = graph_zone.New::<JSOperatorBuilder>();
            let jsgraph = graph_zone.New::<JSGraph>();

            TFPipelineData {
                isolate: None,
                allocator: unsafe { &*wasm_engine }.allocator(),
                info,
                debug_name,
                may_have_unverifiable_graph: true,
                zone_stats,
                pipeline_statistics,
                graph_zone,
                instruction_zone_scope,
                instruction_zone,
                codegen_zone_scope,
                codegen_zone,
                broker: None,
                register_allocation_zone_scope,
                register_allocation_zone,
                assembler_options: assembler_options.clone(),
                graph,
                source_positions,
                node_origins,
                simplified,
                machine,
                common,
                javascript,
                jsgraph,
                mcgraph,
                observe_node_manager: std::ptr::null_mut(), // Correct initialization
                dependencies: std::ptr::null_mut(), // Correct initialization
                schedule: std::ptr::null_mut(),
                sequence: std::ptr::null_mut(),
                frame: std::ptr::null_mut(),
                register_allocation_data: std::ptr::null_mut(),
                source_position_output: String::new(),
                jump_optimization_info: std::ptr::null_mut(),
                specialization_context: Nothing(),
                max_unoptimized_frame_height: 0,
                max_pushed_argument_count: 0,
                runtime_call_stats: std::ptr::null_mut(),
                profile_data: std::ptr::null_mut(),
                code: Nothing(),
                code_generator: std::ptr::null_mut(),
                 typer_: std::ptr::null_mut(),
                typer_flags_: TyperFlags::kNoFlags,
                 osr_helper_: None,
                //#[cfg(V8_ENABLE_WEBASSEMBLY)]
                wasm_module_for_inlining_: std::ptr::null(),
                //#[cfg(V8_ENABLE_WEBASSEMBLY)]
                js_wasm_calls_sidetable_: std::ptr::null_mut(),
                start_source_position_: K_NO_SOURCE_POSITION
            }
        }

        pub fn new_for_testing(
            zone_stats: *mut ZoneStats,
            info: *mut OptimizedCompilationInfo,
            isolate: &'static Isolate,
            allocator: *mut AccountingAllocator,
            graph: *mut TFGraph,
            jsgraph: *mut JSGraph,
            schedule: *mut Schedule,
            source_positions: *mut SourcePositionTable,
            node_origins: *mut NodeOriginTable,
            jump_opt: *mut JumpOptimizationInfo,
            assembler_options: &AssemblerOptions,
            profile_data: *mut ProfileDataFromFile,
        ) -> Self {
            let info_ref = unsafe { &*info };
            let debug_name = info_ref.GetDebugName();

            let graph_zone = GraphZone::new(zone_stats, K_GRAPH_ZONE_NAME, K_COMPRESS_GRAPH_ZONE);
            let instruction_zone_scope = ZoneStatsScope::new(zone_stats, K_INSTRUCTION_ZONE_NAME);
            let instruction_zone = instruction_zone_scope.zone();
            let codegen_zone_scope = ZoneStatsScope::new(zone_stats, K_CODEGEN_ZONE_NAME);
            let codegen_zone = codegen_zone_scope.zone();
            let register_allocation_zone_scope = ZoneStatsScope::new(zone_stats, K_REGISTER_ALLOCATION_ZONE_NAME);
            let register_allocation_zone = register_allocation_zone_scope.zone();

            let (simplified, machine, common, javascript) = if !jsgraph.is_null() {
                unsafe {
                    let jsgraph_ref = &*jsgraph;
                    (
                        jsgraph_ref.simplified(),
                        jsgraph_ref.machine(),
                        jsgraph_ref.common(),
                        jsgraph_ref.javascript(),
                    )
                }
            } else if !graph.is_null() {
                let simplified = graph_zone.New::<SimplifiedOperatorBuilder>();
                let machine = graph_zone.New::<MachineOperatorBuilder>();
                let common = graph_zone.New::<CommonOperatorBuilder>();
                let javascript = graph_zone.New::<JSOperatorBuilder>();
                (simplified, machine, common, javascript)
            } else {
                (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut())
            };

            let new_jsgraph = if !jsgraph.is_null(){
                jsgraph
            } else {
                graph_zone.New::<JSGraph>()
            };
            
            TFPipelineData {
                isolate: Some(isolate),
                allocator,
                info,
                debug_name,
                may_have_unverifiable_graph: true,
                zone_stats,
                pipeline_statistics: std::ptr::null_mut(),
                graph_zone,
                instruction_zone_scope,
                instruction_zone,
                codegen_zone_scope,
                codegen_zone,
                broker: None,
                register_allocation_zone_scope,
                register_allocation_zone,
                assembler_options: assembler_options.clone(),
                graph,
                source_positions,
                node_origins,
                simplified,
                machine,
                common,
                javascript,
                jsgraph: new_jsgraph,
                mcgraph: std::ptr::null_mut(),
                observe_node_manager: std::ptr::null_mut(),
                dependencies: std::ptr::null_mut(),
                schedule,
                sequence: std::ptr::null_mut(),
                frame: std::ptr::null_mut(),
                register_allocation_data: std::ptr::null_mut(),
                source_position_output: String::new(),
                jump_optimization_info: jump_opt,
                specialization_context: Nothing(),
                max_unoptimized_frame_height: 0,
                max_pushed_argument_count: 0,
                runtime_call_stats: std::ptr::null_mut(),
                profile_data,
                code: Nothing(),
                code_generator: std::ptr::null_mut(),
                 typer_: std::ptr::null_mut(),
                typer_flags_: TyperFlags::kNoFlags,
                 osr_helper_: None,
                //#[cfg(V8_ENABLE_WEBASSEMBLY)]
                wasm_module_for_inlining_: std::ptr::null(),
                //#[cfg(V8_ENABLE_WEBASSEMBLY)]
                js_wasm_calls_sidetable_: std::ptr::null_mut(),
                start_source_position_: K_NO_SOURCE_POSITION
            }
        }

        pub fn new_for_register_allocation_testing(
            zone_stats: *mut ZoneStats,
            info: *mut OptimizedCompilationInfo,
            isolate: &'static Isolate,
            sequence: *mut InstructionSequence,
        ) -> Self {
            let info_ref = unsafe { &*info };
            let debug_name = info_ref.GetDebugName();

            let graph_zone = GraphZone::new(zone_stats, K_GRAPH_ZONE_NAME, K_COMPRESS_GRAPH_ZONE);
            let instruction_zone_scope = ZoneStatsScope::new(zone_stats, K_INSTRUCTION_ZONE_NAME);
            let instruction_zone = unsafe { &*sequence }.zone();
            let codegen_zone_scope = ZoneStatsScope::new(zone_stats, K_CODEGEN_ZONE_NAME);
            let codegen_zone = codegen_zone_scope.zone();
            let register_allocation_zone_scope = ZoneStatsScope::new(zone_stats, K_REGISTER_ALLOCATION_ZONE_NAME);
            let register_allocation_zone = register_allocation_zone_scope.zone();
            let assembler_options = AssemblerOptions::Default(isolate);

            TFPipelineData {
                isolate: Some(isolate),
                allocator: isolate as *const Isolate as *mut AccountingAllocator,
                info,
                debug_name,
                may_have_unverifiable_graph: true,
                zone_stats,
                pipeline_statistics: std::ptr::null_mut(),
                graph_zone,
                instruction_zone_scope,
                instruction_zone,
                codegen_zone_scope,
                codegen_zone,
                broker: None,
                register_allocation_zone_scope,
                register_allocation_zone,
                assembler_options,
                graph: std::ptr::null_mut(),
                source_positions: std::ptr::null_mut(),
                node_origins: std::ptr::null_mut(),
                simplified: std::ptr::null_mut(),
                machine: std::ptr::null_mut(),
                common: std::ptr::null_mut(),
                javascript: std::ptr::null_mut(),
                jsgraph: std::ptr::null_mut(),
                mcgraph: std::ptr::null_mut(),
                observe_node_manager: std::ptr::null_mut(),
                dependencies: std::ptr::null_mut(),
                schedule: std::ptr::null_mut(),
                sequence,
                frame: std::ptr::null_mut(),
                register_allocation_data: std::ptr::null_mut(),
                source_position_output: String::new(),
                jump_optimization_info: std::ptr::null_mut(),
                specialization_context: Nothing(),
                max_unoptimized_frame_height: 0,
                max_pushed_argument_count: 0,
                runtime_call_stats: std::ptr::null_mut(),
                profile_data: std::ptr::null_mut(),
                code: Nothing(),
                code_generator: std::ptr::null_mut(),
                 typer_: std::ptr::null_mut(),
                typer_flags_: TyperFlags::kNoFlags,
                 osr_helper_: None,
                //#[cfg(V8_ENABLE_WEBASSEMBLY)]
                wasm_module_for_inlining_: std::ptr::null(),
                //#[cfg(V8_ENABLE_WEBASSEMBLY)]
                js_wasm_calls_sidetable_: std::ptr::null_mut(),
                start_source_position_: K_NO_SOURCE_POSITION
            }
        }

        pub fn isolate(&self) -> Option<&'static Isolate> {
            self.isolate
        }

        pub fn allocator(&self) -> *mut AccountingAllocator {
            self.allocator
        }

        pub fn info(&self) -> &OptimizedCompilationInfo {
            unsafe { &*self.info }
        }

        pub fn zone_stats(&self) -> *mut ZoneStats {
            self.zone_stats
        }

        pub fn dependencies(&self) -> &CompilationDependencies {
            unsafe {