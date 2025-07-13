// Converted from V8 C++ source files:
// Header: js-inlining.h
// Implementation: js-inlining.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod js_inlining {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::compiler::all_nodes::AllNodes;
    use crate::compiler::bytecode_analysis::BytecodeOffset;
    use crate::compiler::graph_reducer::AdvancedReducer;
    use crate::compiler::js_graph::JSGraph;
    use crate::compiler::js_operator::JSOperatorBuilder;
    use crate::compiler::node_origin_table::NodeOriginTable;
    use crate::compiler::simplified_operator::SimplifiedOperatorBuilder;
    use crate::execution::isolate::Isolate;
    use crate::v8::internal::{
        BytecodeArrayRef, CallFrequency, FeedbackCellRef, FrameState,
        FrameStateType, JSConstructNode, JSCreateClosureNode, JSFunctionRef,
        NodeMatcher, OptionalBytecodeArrayRef, OptionalSharedFunctionInfoRef,
        SharedFunctionInfoRef,
    };

    use super::super::wasm;
    use super::{wasm::WasmModule, JSHeapBroker, Node, NodeVector, Reduction, TFGraph};

    pub struct SourcePositionTable;

    pub struct OptimizedCompilationInfo;

    pub struct JSWasmCallParameters;

    pub type JsWasmCallsSidetable =
        std::collections::HashMap<NodeId, *const JSWasmCallParameters>;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NodeId(usize);

    pub struct JSInliner<'a> {
        advanced_reducer: AdvancedReducer<'a>,
        local_zone_: Rc<RefCell<Zone>>,
        info_: *mut OptimizedCompilationInfo, // Raw pointer to C++ object
        jsgraph_: &'a JSGraph,
        broker_: *mut JSHeapBroker, // Raw pointer to C++ object
        source_positions_: *mut SourcePositionTable, // Raw pointer to C++ object
        node_origins_: *mut NodeOriginTable, // Raw pointer to C++ object
        wasm_module_: *const WasmModule, // Raw pointer to C++ object
        js_wasm_calls_sidetable_: *mut JsWasmCallsSidetable, // Raw pointer to C++ object
        inline_wasm_fct_if_supported_: bool,
    }

    impl<'a> JSInliner<'a> {
        pub fn new(
            editor: &'a mut Editor<'a>,
            local_zone: &Rc<RefCell<Zone>>,
            info: *mut OptimizedCompilationInfo,
            jsgraph: &'a JSGraph,
            broker: *mut JSHeapBroker,
            source_positions: *mut SourcePositionTable,
            node_origins: *mut NodeOriginTable,
            wasm_module: *const WasmModule,
            js_wasm_calls_sidetable: *mut JsWasmCallsSidetable,
            inline_wasm_fct_if_supported: bool,
        ) -> Self {
            JSInliner {
                advanced_reducer: AdvancedReducer::new(editor),
                local_zone_: local_zone.clone(),
                info_: info,
                jsgraph_: jsgraph,
                broker_: broker,
                source_positions_: source_positions,
                node_origins_: node_origins,
                wasm_module_: wasm_module,
                js_wasm_calls_sidetable_: js_wasm_calls_sidetable,
                inline_wasm_fct_if_supported_: inline_wasm_fct_if_supported,
            }
        }

        pub fn reducer_name(&self) -> &'static str {
            "JSInliner"
        }

        pub fn reduce(&mut self, _node: *mut Node) -> Reduction {
            unreachable!()
        }

        pub fn reduce_js_call(&mut self, node: *mut Node) -> Reduction {
            // Ensure node is not null
            assert!(!node.is_null());

            // Determine the opcode of the node
            let opcode = unsafe { (*node).opcode() };

            // Log the opcode for debugging purposes
            //println!("ReduceJSCall: Opcode = {:?}", opcode);

            // Call the internal implementation
            self.reduce_js_call_internal(node)
        }

        fn reduce_js_call_internal(&mut self, node: *mut Node) -> Reduction {
            #[cfg(V8_ENABLE_WEBASSEMBLY)]
            {
                if unsafe { (*node).opcode() } == IrOpcode::kJSWasmCall {
                    return Reduction::kNoChange; //self.reduce_js_wasm_call(node);
                }
            }

            let call = JSCallAccessor::new(node);

            // Determine the call target.
            let shared_info = self.determine_call_target(node);
            if shared_info.is_none() {
                return Reduction::kNoChange;
            }
            let shared_info = shared_info.unwrap();

            let outer_shared_info = unsafe {
                SharedFunctionInfoRef::make_ref(
                    (*self.broker_).isolate(),
                    (*self.info_).shared_info() as *mut SharedFunctionInfo,
                )
            };

            let inlineability = unsafe { shared_info.get_inlineability() };
            if inlineability != SharedFunctionInfo::Inlineability::kIsInlineable {
                // The function is no longer inlineable. The only way this can happen is if
                // the function had its optimization disabled in the meantime, e.g. because
                // another optimization job failed too often.
                assert_eq!(
                    inlineability,
                    SharedFunctionInfo::Inlineability::kHasOptimizationDisabled
                );
                //TRACE(format!("Not inlining {} into {} because it had its optimization disabled.", shared_info, outer_shared_info));
                return Reduction::kNoChange;
            }
            // NOTE: Even though we bailout in the kHasOptimizationDisabled case above, we
            // won't notice if the function's optimization is disabled after this point.

            // Constructor must be constructable.
            if unsafe { (*node).opcode() } == IrOpcode::kJSConstruct && !is_constructable(unsafe { shared_info.kind() })
            {
                //TRACE(format!("Not inlining {} into {} because constructor is not constructable.", shared_info, outer_shared_info));
                return Reduction::kNoChange;
            }

            // Class constructors are callable, but [[Call]] will raise an exception.
            // See ES6 section 9.2.1 [[Call]] ( thisArgument, argumentsList ).
            if unsafe { (*node).opcode() } == IrOpcode::kJSCall && is_class_constructor(unsafe { shared_info.kind() }) {
                //TRACE(format!("Not inlining {} into {} because callee is a class constructor.", shared_info, outer_shared_info));
                return Reduction::kNoChange;
            }

            // To ensure inlining always terminates, we have an upper limit on inlining
            // the nested calls.
            let mut nesting_level = 0;
            let mut frame_state_node = NodeProperties::get_frame_state_input(node);
            while unsafe { (*frame_state_node).opcode() } == IrOpcode::kFrameState {
                nesting_level += 1;
                if nesting_level > K_MAX_DEPTH_FOR_INLINING {
                    //TRACE(format!("Not inlining {} into {} because call has exceeded the maximum depth for function inlining.", shared_info, outer_shared_info));
                    return Reduction::kNoChange;
                }
                frame_state_node = unsafe {
                    (*(*frame_state_node).op())
                        .parameter::<FrameState>(0)
                        .outer_frame_state()
                };
            }

            let mut exception_target: *mut Node = std::ptr::null_mut();
            NodeProperties::is_exceptional_call(node, &mut exception_target);

            // JSInliningHeuristic has already filtered candidates without a BytecodeArray
            // based on SharedFunctionInfoRef::GetInlineability. For the inlineable ones
            // (kIsInlineable), the broker holds a reference to the bytecode array, which
            // prevents it from getting flushed.  Therefore, the following check should
            // always hold true.
            assert!(unsafe { shared_info.is_compiled() });

            if unsafe { (*self.info_).source_positions() }
                && unsafe {
                    !(*shared_info.object()).are_source_positions_available(
                        (*self.broker_).local_isolate_or_isolate(),
                    )
                }
            {
                // This case is expected to be very rare, since we generate source
                // positions for all functions when debugging or profiling are turned
                // on (see Isolate::NeedsDetailedOptimizedCodeLineInfo). Source
                // positions should only be missing here if there is a race between 1)
                // enabling/disabling the debugger/profiler, and 2) this compile job.
                // In that case, we simply don't inline.
                //TRACE(format!("Not inlining {} into {} because source positions are missing.", shared_info, outer_shared_info));
                return Reduction::kNoChange;
            }

            // Determine the target's feedback vector and its context.
            let mut context: *mut Node = std::ptr::null_mut();
            let feedback_cell = self.determine_call_context(node, &mut context);

            //TRACE(format!("Inlining {} into {} {}", shared_info, outer_shared_info, if exception_target != std::ptr::null_mut() { " (inside try-block)" } else { "" }));
            // ----------------------------------------------------------------
            // After this point, we've made a decision to inline this function.
            // We shall not bailout from inlining if we got here.

            let bytecode_array = unsafe { shared_info.get_bytecode_array() };

            // Remember that we inlined this function.
            let inlining_id = unsafe {
                (*self.info_).add_inlined_function(
                    shared_info.object(),
                    bytecode_array.object(),
                    (*self.source_positions_).get_source_position(node),
                )
            };
            if v8_flags.profile_guided_optimization
                && unsafe { feedback_cell.feedback_vector().has_value() }
                && unsafe {
                    (*feedback_cell.feedback_vector().value().object())
                        .invocation_count_before_stable(kRelaxedLoad)
                        > v8_flags.invocation_count_for_early_optimization
                }
            {
                unsafe { (*self.info_).set_could_not_inline_all_candidates() };
            }

            // Create the subgraph for the inlinee.
            let mut start_node: *mut Node = std::ptr::null_mut();
            let mut end: *mut Node = std::ptr::null_mut();
            {
                // Run the BytecodeGraphBuilder to create the subgraph.
                let scope = TFGraph::SubgraphScope::new(self.jsgraph_.graph());
                let mut flags =
                    BytecodeGraphBuilderFlags::kSkipFirstStackAndTierupCheck;
                if unsafe { (*self.info_).analyze_environment_liveness() } {
                    flags |= BytecodeGraphBuilderFlags::kAnalyzeEnvironmentLiveness;
                }
                if unsafe { (*self.info_).bailout_on_uninitialized() } {
                    flags |= BytecodeGraphBuilderFlags::kBailoutOnUninitialized;
                }
                {
                    let frequency = call.frequency();
                    //build_graph_from_bytecode(
                    //    self.broker_,
                    //    &self.local_zone_.borrow(),
                    //    shared_info,
                    //    bytecode_array,
                    //    feedback_cell,
                    //    BytecodeOffset::None(),
                    //    self.jsgraph_,
                    //    frequency,
                    //    self.source_positions_,
                    //    self.node_origins_,
                    //    inlining_id,
                    //    unsafe { (*self.info_).code_kind() },
                    //    flags,
                    //    unsafe { (*self.info_).tick_counter() },
                    //);
                }

                // Extract the inlinee start/end nodes.
                start_node = self.jsgraph_.graph().start();
                end = self.jsgraph_.graph().end();
            }
            let start = StartNode { node: start_node };

            // If we are inlining into a surrounding exception handler, we collect all
            // potentially throwing nodes within the inlinee that are not handled locally
            // by the inlinee itself. They are later wired into the surrounding handler.
            let mut uncaught_subcalls: NodeVector = Vec::new();
            if exception_target != std::ptr::null_mut() {
                // Find all uncaught 'calls' in the inlinee.
                let inlined_nodes = AllNodes::new(&self.local_zone_.borrow(), end, self.jsgraph_.graph());
                for subnode in inlined_nodes.reachable {
                    // Every possibly throwing node should get {IfSuccess} and {IfException}
                    // projections, unless there already is local exception handling.
                    if unsafe { (*subnode.clone()).op().has_property(Operator::kNoThrow) } {
                        continue;
                    }
                    if !NodeProperties::is_exceptional_call(subnode, &mut std::ptr::null_mut()) {
                        assert_eq!(unsafe { (*subnode.clone()).op().control_output_count() }, 2);
                        uncaught_subcalls.push(subnode);
                    }
                }
            }

            let frame_state = NodeProperties::get_frame_state_input(node);
            let new_target = self.jsgraph_.undefined_constant();

            // Inline {JSConstruct} requires some additional magic.
            if unsafe { (*node).opcode() } == IrOpcode::kJSConstruct {
                assert!(JSCallOrConstructNode::kHaveIdenticalLayouts);
                let construct_node = JSConstructNode::new(node);

                //let new_target = construct_node.new_target();

                // Insert nodes around the call that model the behavior required for a
                // constructor dispatch (allocate implicit receiver and check return value).
                // This models the behavior usually accomplished by our {JSConstructStub}.
                // Note that the context has to be the callers context (input to call node).
                // Also note that by splitting off the {JSCreate} piece of the constructor
                // call, we create an observable deoptimization point after the receiver
                // instantiation but before the invocation (i.e. inside {JSConstructStub}
                // where execution continues at {construct_stub_create_deopt_pc_offset}).
                let mut receiver = self.jsgraph_.the_hole_constant(); // Implicit receiver.
                let caller_context = NodeProperties::get_context_input(node);
                if needs_implicit_receiver(shared_info) {
                    //let effect = construct_node.effect();
                    //let control = construct_node.control();
                    let frame_state_inside: *mut Node;
                    let mut matcher = HeapObjectMatcher::new(new_target);
                    if matcher.has_resolved_value()
                        && unsafe { matcher.ref_(*self.broker_).is_js_function() }
                    {
                        // If {new_target} is a JSFunction, then we cannot deopt in the
                        // NewObject call. Therefore we do not need the artificial frame state.
                        frame_state_inside = frame_state;
                    } else {
                        frame_state_inside = self.create_artificial_frame_state(
                            node,
                            FrameState { node: frame_state },
                            construct_node.argument_count() as i32,
                            FrameStateType::kConstructCreateStub,
                            shared_info,
                            OptionalBytecodeArrayRef::None,
                            caller_context,
                        );
                    }
                    //let create = self.graph().new_node(
                    //    self.javascript().create(),
                    //    call.target(),
                    //    new_target,
                    //    caller_context,
                    //    frame_state_inside,
                    //    effect,
                    //    control,
                    //);
                    //uncaught_subcalls.push(create); // Adds {IfSuccess} & {IfException}.
                    //NodeProperties::replace_control_input(node, create);
                    //NodeProperties::replace_effect_input(node, create);
                    //// Placeholder to hold {node}'s value dependencies while {node} is
                    //// replaced.
                    //let dummy = self.graph().new_node(self.common().dead());
                    //NodeProperties::replace_uses(node, dummy, node, node, node);
                    //let result = self.graph().new_node(
                    //    self.common().select(MachineRepresentation::kTagged),
                    //    self.simplified().object_is_receiver(),
                    //    node,
                    //    create,
                    //);
                    //receiver = create; // The implicit receiver.
                    //replace_with_value(dummy, result);
                } else if is_derived_constructor(unsafe { shared_info.kind() }) {
                    //let node_success = NodeProperties::find_successful_control_projection(node);
                    //let is_receiver = self.graph().new_node(self.simplified().object_is_receiver(), node);
                    //let branch_is_receiver = self.graph().new_node(self.common().branch(), is_receiver, node_success);
                    //let branch_is_receiver_true = self.graph().new_node(self.common().if_true(), branch_is_receiver);
                    //let branch_is_receiver_false = self.graph().new_node(self.common().if_false(), branch_is_receiver);
                    //let caller_context = NodeProperties::get_context_input(node);
                    //branch_is_receiver_false = self.graph().new_node(
                    //    self.javascript().call_runtime(Runtime::kThrowConstructorReturnedNonObject),
                    //    caller_context,
                    //    NodeProperties::get_frame_state_input(node),
                    //    node,
                    //    branch_is_receiver_false,
                    //);
                    //uncaught_subcalls.push(branch_is_receiver_false);
                    //branch_is_receiver_false = self.graph().new_node(self.common().throw(), branch_is_receiver_false, branch_is_receiver_false);
                    //merge_control_to_end(self.graph(), self.common(), branch_is_receiver_false);
                    //replace_with_value(node_success, node_success, node_success, branch_is_receiver_true);
                    //// Fix input destroyed by the above {ReplaceWithValue} call.
                    //NodeProperties::replace_control_input(branch_is_receiver, node_success, 0);
                }
                //unsafe { (*node).replace_input(JSCallNode::ReceiverIndex() as i32, receiver) };

                //// Insert a construct stub frame into the chain of frame states. This will
                //// reconstruct the proper frame when deoptimizing within the constructor.
                //frame_state = self.create_artificial_frame_state(
                //    node,
                //    FrameState { node: frame_state },
                //    0,
                //    FrameStateType::kConstructInvokeStub,
                //    shared_info,
                //    bytecode_array,
                //    caller_context,
                //);
            }

            //// Insert a JSConvertReceiver node for sloppy callees. Note that the context
            //// passed into this node has to be the callees context (loaded above).
            //if unsafe { (*node).opcode() } == IrOpcode::kJSCall
            //    && is_sloppy(unsafe { shared_info.language_mode() })
            //    && !unsafe { shared_info.native() }
            //{
            //    let effect = Effect {
            //        node: NodeProperties::get_effect_input(node),
            //    };
            //    if NodeProperties::can_be_primitive(self.broker_, call.receiver(), effect) {
            //        let p = CallParametersOf(unsafe { (*node).op() });
            //        let global_proxy = self.jsgraph_.constant_no_hole(
            //            self.broker_
            //                .target_native_context()
            //                .global_proxy_object(self.broker_),
            //            self.broker_,
            //        );
            //        let receiver = effect.node = self.graph().new_node(
            //            self.simplified().convert_receiver(p.convert_mode()),
            //            call.receiver(),
            //            self.jsgraph_.constant_no_hole(
            //                self.broker_.target_native_context(),
            //                self.broker_,
            //            ),
            //            global_proxy,
            //            effect.node,
            //            self.jsgraph_.start(),
            //        );
            //        NodeProperties::replace_value_input(
            //            node,
            //            receiver,
            //            JSCallNode::ReceiverIndex() as i32,
            //        );
            //        NodeProperties::replace_effect_input(node, effect.node);
            //    }
            //}

            //// Insert inlined extra arguments if required. The callees formal parameter
            //// count have to match the number of arguments passed to the call.
            //let parameter_count = bytecode_array.parameter_count_without_receiver();
            //assert_eq!(
            //    parameter_count,
            //    shared_info.internal_formal_parameter_count_without_receiver() as i32
            //);
            //assert_eq!(parameter_count, start.formal_parameter_count_without_receiver());
            //if call.argument_count() as i32 != parameter_count {
            //    frame_state = self.create_artificial_frame_state(
            //        node,
            //        FrameState { node: frame_state },
            //        call.argument_count() as i32,
            //        FrameStateType::kInlinedExtraArguments,
            //        shared_info,
            //        bytecode_array,
            //    );
            //}

            self.inline_call(
                node,
                new_target,
                context,
                frame_state,
                start,
                end,
                exception_target,
                &uncaught_subcalls,
                call.argument_count() as i32,
            )
        }

        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        fn reduce_js_wasm_call(&mut self, node: *mut Node) -> Reduction {
            let call_node = JSWasmCallNode::new(node);
            let wasm_call_params = call_node.parameters();
            let fct_index = wasm_call_params.function_index();
            let native_module = wasm_call_params.native_module();
            let sig = wasm_call_params.signature();

            // Try "full" inlining of very simple wasm functions (mainly getters / setters
            // for wasm gc objects).
            let mut inline_result = WasmInlineResult {
                can_inline_body: false,
                body_start: std::ptr::null_mut(),
                body_end: std::ptr::null_mut(),
            };
            if self.inline_wasm_fct_if_supported_ && fct_index != -1 && !native_module.is_null()
                && !is_asmjs_module(unsafe { (*native_module).module() })
            {
                inline_result = self.try_wasm_inlining(&call_node);
            }

            // Create the subgraph for the wrapper inlinee.
            let mut wrapper_start_node: *mut Node = std::ptr::null_mut();
            let mut wrapper_end_node: *mut Node = std::ptr::null_mut();
            let mut subgraph_min_node_id: usize = 0;
            {
                let scope = TFGraph::SubgraphScope::new(self.jsgraph_.graph());
                self.jsgraph_.graph().set_end(std::ptr::null_mut());

                // Create a nested frame state inside the frame state attached to the
                // call; this will ensure that lazy deoptimizations at this point will
                // still return the result of the Wasm function call.
                //let continuation_frame_state =
                //    create_js_wasm_call_builtin_continuation_frame_state(
                //        self.jsgraph_,
                //        call_node.context(),
                //        call_node.frame_state(),
                //        sig,
                //    );

                // All the nodes inserted by the inlined subgraph will have
                // id >= subgraph_min_node_id. We use this later to avoid wire nodes that
                // are not inserted by the inlinee but were already part of the graph to the
                // surrounding exception handler, if present.
                subgraph_min_node_id = self.jsgraph_.graph().node_count();

                //// If we inline the body with Turboshaft later (instead of with TurboFan
                //// here), we don't know yet whether we can inline the body or not. Hence,
                //// don't set the thread-in-wasm flag now, and instead do that if _not_
                //// inlining later in Turboshaft.
                //let set_in_wasm_flag = !(inline_result.can_inline_body
                //    || crate::compiler::v8_flags.turboshaft_wasm_in_js_inlining);
                //build_inlined_js_to_wasm_wrapper(
                //    self.graph().zone(),
                //    self.jsgraph_,
                //    sig,
                //    self.isolate(),
                //    self.source_positions_,
                //    continuation_frame_state,
                //    set_in_wasm_flag,
                //);

                //// Extract the inlinee start/end nodes.
                wrapper_start_node = self.jsgraph_.graph().start();
                wrapper_end_node = self.jsgraph_.graph().end();
            }
            //let start = StartNode { node: wrapper_start_node };

            //let mut exception_target: *mut Node = std::ptr::null_mut();
            //NodeProperties::is_exceptional_call(node, &mut exception_target);

            //// If we are inlining into a surrounding exception handler, we collect all
            //// potentially throwing nodes within the inlinee that are not handled locally
            //// by the inlinee itself. They are later wired into the surrounding handler.
            //let mut uncaught_subcalls: NodeVector = Vec::new();
            //if exception_target != std::ptr::null_mut() {
            //    // Find all uncaught 'calls' in the inlinee.
            //    let inlined_nodes = AllNodes::new(&self.local_zone_.borrow(), wrapper_end_node, self.jsgraph_.graph());
            //    for subnode in inlined_nodes.reachable {
            //        // Ignore nodes that are not part of the inlinee.
            //        if subnode.id() < subgraph_min_node_id {
            //            continue;
            //        }

            //        // Every possibly throwing node should get {IfSuccess} and {IfException}
            //        // projections, unless there already is local exception handling.
            //        if unsafe { (*subnode.clone()).op().has_property(Operator::kNoThrow) } {
            //            continue;
            //        }
            //        if !NodeProperties::is_exceptional_call(subnode, &mut std::ptr::null_mut()) {
            //            assert_eq!(unsafe { (*subnode.clone()).op().control_output_count() }, 2);
            //            uncaught_subcalls.push(subnode);
            //        }
            //    }
            //}

            //// Search in inlined nodes for call to inline wasm.
            //// Note: We can only inline wasm functions of a single wasm module into any
            //// given JavaScript function (due to the WasmGCLowering being dependent on
            //// module-specific type indices).
            //let mut wasm_fct_call: *mut Node = std::ptr::null_mut();
            //if inline_result.can_inline_body || crate::compiler::v8_flags.turboshaft_wasm_in_js_inlining {
            //    let inlined_nodes = AllNodes::new(&self.local_zone_.borrow(), wrapper_end_node, self.jsgraph_.graph());
            //    for subnode in inlined_nodes.reachable {
            //        // Ignore nodes that are not part of the inlinee.
            //        if subnode.id() < subgraph_min_node_id {
            //            continue;
            //        }

            //        if unsafe { (*subnode.clone()).opcode() } == IrOpcode::kCall
            //            && unsafe { (*CallDescriptorOf(subnode.clone().op())).is_any_wasm_function_call() }
            //        {
            //            wasm_fct_call = subnode;
            //            break;
            //        }
            //    }
            //    assert!(!inline_result.can_inline_body || wasm_fct_call != std::ptr::null_mut());

            //    // Attach information about Wasm call target for Turboshaft Wasm-in-JS-
            //    // inlining (see https://crbug.com/353475584) in sidetable.
            //    if crate::compiler::v8_flags.turboshaft_wasm_in_js_inlining && wasm_fct_call != std::ptr::null_mut() {
            //        let [it, inserted] = unsafe {
            //            (*self.js_wasm_calls_sidetable_).insert(
            //                (wasm_fct_call.id(), &wasm_call_params),
            //            )
            //        };
            //        USE(it);
            //        assert!(inserted);
            //    }
            //}

            //let context = NodeProperties::get_context_input(node);
            //let frame_state = NodeProperties::get_frame_state_input(node);
            //let new_target = self.jsgraph_.undefined_constant();

            //// Inline the wasm wrapper.
            //let r = self.inline_js_wasm_call(
            //    node,
            //    new_target,
            //    context,
            //    frame_state,
            //    start,
            //    wrapper_end_node,
            //    exception_target,
            //    &uncaught_subcalls,
            //);
            //// Inline the wrapped wasm body if supported.
            //if inline_result.can_inline_body {
            //    self.inline_wasm_function(
            //        wasm_fct_call,
            //        inline_result.body_start,
            //        inline_result.body_end,
            //        call_node.frame_state(),
            //        wasm_call_params.shared_fct_info(),
            //        call_node.argument_count(),
            //        context,
            //    );
            //}
            Reduction::kNoChange //r
        }
    }

    const K_MAX_DEPTH_FOR_INLINING: i32 = 50;
    const KRelaxedLoad: usize = 0;

    //impl JSInliner {
    //    fn inline_js_wasm_call(
    //        &mut self,
    //        call: *mut Node,
    //        new_target: *mut Node,
    //        context: *mut Node,
    //        frame_state: *mut Node,
    //        start: StartNode,
    //        end: *mut Node,
    //        exception_target: *mut Node,
    //        uncaught_subcalls: &NodeVector,
    //    ) -> Reduction {
    //        let n = JSWasmCallNode::new(call);
    //        self.inline_call(
    //            call,
    //            new_target,
    //            context,
    //            frame_state,
    //            start,
    //            end,
    //            exception_target,
    //            uncaught_subcalls,
    //            n.parameters().signature().parameter_count() as i32,
    //        )
    //    }
    //}

    struct JSCallAccessor {
        call_: *mut Node,
    }

    impl JSCallAccessor {
        fn new(call: *mut Node) -> Self {
            assert!(
                unsafe { (*call).opcode() } == IrOpcode::kJSCall
                    || unsafe { (*call).opcode() } == IrOpcode::kJSConstruct
            );
            JSCallAccessor { call_: call }
        }

        fn target(&self) -> *mut Node {
            unsafe { (*self.call_).input_at(JSCallOrConstructNode::kTargetIndex as i32) }
        }

        fn receiver(&self) -> *mut Node {
            unsafe { JSCallNode::new(self.call_).receiver() }
        }

        fn new_target(&self) -> *mut Node {
            unsafe { JSConstructNode::new(self.call_).new_target() }
        }

        fn frame_state(&self) -> *mut Node {
            NodeProperties::get_frame_state_input(self.call_)
        }

        fn argument_count(&self) -> u32 {
            if unsafe { (*self.call_).opcode() } == IrOpcode::kJSCall {
                unsafe { JSCallNode::new(self.call_).argument_count() }
            } else {
                unsafe { JSConstructNode::new(self.call_).argument_count() }
            }
        }

        fn frequency(&self) -> CallFrequency {
            if unsafe { (*self.call_).opcode() } == IrOpcode::kJSCall {
                unsafe { JSCallNode::new(self.call_).parameters().frequency() }
            } else {
                unsafe { JSConstructNode::new(self.call_).parameters().frequency() }
            }
        }
    }

    impl<'a> JSInliner<'a> {
        fn inline_call(
            &mut self,
            call: *mut Node,
            new_target: *mut Node,
            context: *mut Node,
            frame_state: *mut Node,
            start: StartNode,
            end: *mut Node,
            exception_target: *mut Node,
            uncaught
