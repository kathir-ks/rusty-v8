// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod stack_check_lowering_reducer {
    use crate::compiler::turboshaft::assembler::*;
    use crate::compiler::turboshaft::graph::*;
    use crate::compiler::turboshaft::index::*;
    use crate::compiler::turboshaft::operations::*;
    use crate::compiler::turboshaft::phase::*;
    use crate::compiler::turboshaft::representations::*;
    use crate::compiler::turboshaft::uniform_reducer_adapter::*;
    use v8::base::Flags;
    use v8::compiler::{
        Builtin, CallDescriptor, CanThrow, ExternalReference, IsolateData,
        LazyDeoptOnThrow, Linkage, NoContextDescriptor, Operator, StackCheckKind,
        StubCallMode,
    };
    use v8::internal::isolate::Isolate;
    use v8::platform::MemoryRepresentation;

    macro_rules! define_assembler_macros {
        () => {};
    }

    macro_rules! undef_assembler_macros {
        () => {};
    }

    pub struct StackCheckLoweringReducer<Next> {
        next: Next,
        isolate_: Option<*mut Isolate>,
    }

    impl<Next> StackCheckLoweringReducer<Next> {
        pub fn new(next: Next) -> Self {
            StackCheckLoweringReducer {
                next,
                isolate_: None,
            }
        }
    }

    impl<Next> StackCheckLoweringReducer<Next>
    where
        Next: UniformReducer<JSStackCheckOp, Output = V<None>>,
        Next: ReduceWasmStackCheckTrait,
    {
        // TURBOSHAFT_REDUCER_BOILERPLATE(StackCheckLowering)
        // The macro is not fully translated, as it would require more context, specifically regarding the GraphReducer trait.
        // Implementing a simplified version based on the available context.
        // type Output = V<None>;
        // fn reduce(&mut self, op: &Operation) -> Self::Output {
        //     match op {
        //         Operation::JSStackCheck(context, frame_state, kind) => {
        //             self.reduce_js_stack_check(context, frame_state, kind)
        //         },
        //         Operation::WasmStackCheck(kind) => {
        //             self.reduce_wasm_stack_check(kind)
        //         }
        //         _ => panic!("Unexpected operation type"),
        //     }
        // }
        //

        fn reduce_js_stack_check(
            &mut self,
            context: V<Context>,
            frame_state: Option<V<FrameState>>,
            kind: JSStackCheckOp::Kind,
        ) -> V<None> {
            let mut assembler = Assembler::default();

            match kind {
                JSStackCheckOp::Kind::kFunctionEntry => {
                    // Loads of the stack limit should not be load-eliminated as it can be
                    // modified by another thread.
                    let limit = assembler.load(
                        assembler.external_constant(ExternalReference::address_of_jslimit(
                            self.isolate(),
                        )),
                        LoadOp::Kind::RawAligned().not_load_eliminable(),
                        MemoryRepresentation::UintPtr,
                    );

                    if !assembler.stack_pointer_greater_than(
                        limit,
                        StackCheckKind::kJSFunctionEntry,
                    ) {
                        assembler.call_runtime_stack_guard_with_gap(
                            self.isolate(),
                            frame_state.unwrap(),
                            context,
                            assembler.stack_check_offset(),
                        );
                    }
                }
                JSStackCheckOp::Kind::kBuiltinEntry => {
                    let stack_limit = assembler.load_off_heap(
                        assembler.external_constant(ExternalReference::address_of_jslimit(
                            self.isolate(),
                        )),
                        MemoryRepresentation::UintPtr,
                    );
                    if !assembler.stack_pointer_greater_than(
                        stack_limit,
                        StackCheckKind::kCodeStubAssembler,
                    ) {
                        assembler.call_runtime_stack_guard(self.isolate(), context);
                    }
                }
                JSStackCheckOp::Kind::kLoop => {
                    let limit = assembler.load(
                        assembler.external_constant(
                            ExternalReference::address_of_no_heap_write_interrupt_request(
                                self.isolate(),
                            ),
                        ),
                        LoadOp::Kind::RawAligned().not_load_eliminable(),
                        MemoryRepresentation::Uint8,
                    );

                    if !assembler.word32_equal(limit, 0) {
                        assembler.call_runtime_handle_no_heap_writes_interrupts(
                            self.isolate(),
                            frame_state.unwrap(),
                            context,
                        );
                    }
                }
            }

            V::<None>::Invalid()
        }

        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        fn reduce_wasm_stack_check(&mut self, kind: WasmStackCheckOp::Kind) -> V<None> {
            let mut assembler = Assembler::default();

            if kind == WasmStackCheckOp::Kind::kFunctionEntry && assembler.is_leaf_function() {
                return V::<None>::Invalid();
            }

            if kind == WasmStackCheckOp::Kind::kFunctionEntry && Flags::experimental_wasm_growable_stacks.load(std::sync::atomic::Ordering::Relaxed) {
                // WasmStackCheck should be lowered by GrowableStacksReducer
                // in a special way.
                return self.next.reduce_wasm_stack_check(kind);
            }

            // Loads of the stack limit should not be load-eliminated as it can be
            // modified by another thread.
            let limit = assembler.load(
                assembler.load_root_register(),
                LoadOp::Kind::RawAligned().not_load_eliminable(),
                MemoryRepresentation::UintPtr,
                IsolateData::jslimit_offset(),
            );

            if !assembler.stack_pointer_greater_than(limit, StackCheckKind::kWasm) {
                // TODO(14108): Cache descriptor.
                let call_descriptor = Linkage::get_stub_call_descriptor(
                    assembler.graph_zone(), // zone
                    NoContextDescriptor {},  // descriptor
                    0,                       // stack parameter count
                    CallDescriptor::kNoFlags, // flags
                    Operator::kNoProperties,  // properties
                    StubCallMode::kCallWasmRuntimeStub, // stub call mode
                );
                let ts_call_descriptor = TSCallDescriptor::create(
                    call_descriptor,
                    CanThrow::kNo,
                    LazyDeoptOnThrow::kNo,
                    assembler.graph_zone(),
                );
                let builtin = assembler
                    .relocatable_wasm_builtin_call_target(Builtin::kWasmStackGuard);
                // Pass custom effects to the `Call` node to mark it as non-writing.
                assembler.call(
                    builtin,
                    vec![],
                    &ts_call_descriptor,
                    OpEffects::new()
                        .can_read_memory()
                        .required_when_unused()
                        .can_create_identity(),
                );
            }

            V::<None>::Invalid()
        }

        #[cfg(not(V8_ENABLE_WEBASSEMBLY))]
        fn reduce_wasm_stack_check(&mut self, _kind: WasmStackCheckOp::Kind) -> V<None> {
            V::<None>::Invalid()
        }

        fn isolate(&mut self) -> *mut Isolate {
            if self.isolate_.is_none() {
                self.isolate_ = Some(unsafe { (**self.next.data()).isolate() });
            }
            self.isolate_.unwrap()
        }
    }

    pub trait ReduceWasmStackCheckTrait {
        fn reduce_wasm_stack_check(&mut self, kind: WasmStackCheckOp::Kind) -> V<None>;
        fn data(&self) -> &Box<dyn TurboshaftIsolateData>;
    }

    pub trait UniformReducer<T, Output> {
        // Placeholder for the reduction interface.
    }

    pub trait TurboshaftIsolateData {
        unsafe fn isolate(&self) -> *mut Isolate;
    }

    //Dummy Implementations
    pub struct Context {}
    pub struct FrameState {}
    pub struct JSStackCheckOp {
        kind: JSStackCheckOp::Kind,
    }
    impl JSStackCheckOp {
        pub enum Kind {
            kFunctionEntry,
            kBuiltinEntry,
            kLoop,
        }
    }
    pub struct WasmStackCheckOp {
        kind: WasmStackCheckOp::Kind,
    }
    impl WasmStackCheckOp {
        pub enum Kind {
            kFunctionEntry,
        }
    }
    pub struct V<T> {
        _phantom: std::marker::PhantomData<T>,
        is_invalid: bool,
    }
    impl<T> V<T> {
        fn Invalid() -> Self {
            V {
                _phantom: std::marker::PhantomData,
                is_invalid: true,
            }
        }
    }
    pub struct OpEffects {
        can_read_memory: bool,
        required_when_unused: bool,
        can_create_identity: bool,
    }
    impl OpEffects {
        fn new() -> Self {
            OpEffects {
                can_read_memory: false,
                required_when_unused: false,
                can_create_identity: false,
            }
        }
        fn can_read_memory(mut self) -> Self {
            self.can_read_memory = true;
            self
        }
        fn required_when_unused(mut self) -> Self {
            self.required_when_unused = true;
            self
        }
        fn can_create_identity(mut self) -> Self {
            self.can_create_identity = true;
            self
        }
    }
    pub struct TSCallDescriptor {}
    impl TSCallDescriptor {
        fn create(
            _call_descriptor: &CallDescriptor,
            _can_throw: CanThrow,
            _lazy_deopt_on_throw: LazyDeoptOnThrow,
            _graph_zone: *mut GraphZone,
        ) -> Self {
            TSCallDescriptor {}
        }
    }
    pub struct GraphZone {}
}