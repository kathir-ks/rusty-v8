// TODO: Replace with actual implementations and crate usages.
// This is a placeholder with dummy structs and functions.

mod compiler {
    pub mod access_builder;
    pub mod allocation_builder;
    pub mod common_operator;
    pub mod compilation_dependencies;
    pub mod js_graph;
    pub mod js_heap_broker;
    pub mod js_operator;
    pub mod node_matchers;
    pub mod node_properties;
    pub mod node;
    pub mod simplified_operator;
    pub mod state_values_utils;

    use std::cell::RefCell;
    use std::rc::Rc;

    use self::access_builder::*;
    use self::allocation_builder::*;
    use self::common_operator::*;
    use self::compilation_dependencies::*;
    use self::js_graph::*;
    use self::js_heap_broker::*;
    use self::js_operator::*;
    use self::node::*;
    use self::node_properties::*;
    use self::simplified_operator::*;
    use self::state_values_utils::*;

    pub struct JSCreateLowering<'a> {
        jsgraph: &'a JSGraph<'a>,
        broker: &'a JSHeapBroker<'a>,
    }

    impl<'a> JSCreateLowering<'a> {
        pub fn new(jsgraph: &'a JSGraph<'a>, broker: &'a JSHeapBroker<'a>) -> Self {
            JSCreateLowering { jsgraph, broker }
        }

        pub fn reduce(&self, node: &mut Node) -> Reduction {
            match node.opcode() {
                IrOpcode::kJSCreate => self.reduce_js_create(node),
                IrOpcode::kJSCreateArguments => self.reduce_js_create_arguments(node),
                IrOpcode::kJSCreateArray => self.reduce_js_create_array(node),
                IrOpcode::kJSCreateArrayIterator => self.reduce_js_create_array_iterator(node),
                IrOpcode::kJSCreateAsyncFunctionObject => self.reduce_js_create_async_function_object(node),
                IrOpcode::kJSCreateBoundFunction => self.reduce_js_create_bound_function(node),
                IrOpcode::kJSCreateClosure => self.reduce_js_create_closure(node),
                IrOpcode::kJSCreateCollectionIterator => self.reduce_js_create_collection_iterator(node),
                IrOpcode::kJSCreateIterResultObject => self.reduce_js_create_iter_result_object(node),
                IrOpcode::kJSCreateStringIterator => self.reduce_js_create_string_iterator(node),
                IrOpcode::kJSCreateKeyValueArray => self.reduce_js_create_key_value_array(node),
                IrOpcode::kJSCreatePromise => self.reduce_js_create_promise(node),
                IrOpcode::kJSCreateLiteralArray => self.reduce_js_create_literal_array_or_object(node),
                IrOpcode::kJSCreateLiteralObject => self.reduce_js_create_literal_array_or_object(node),
                IrOpcode::kJSCreateLiteralRegExp => self.reduce_js_create_literal_reg_exp(node),
                IrOpcode::kJSGetTemplateObject => self.reduce_js_get_template_object(node),
                IrOpcode::kJSCreateEmptyLiteralArray => self.reduce_js_create_empty_literal_array(node),
                IrOpcode::kJSCreateEmptyLiteralObject => self.reduce_js_create_empty_literal_object(node),
                IrOpcode::kJSCreateFunctionContext => self.reduce_js_create_function_context(node),
                IrOpcode::kJSCreateWithContext => self.reduce_js_create_with_context(node),
                IrOpcode::kJSCreateCatchContext => self.reduce_js_create_catch_context(node),
                IrOpcode::kJSCreateBlockContext => self.reduce_js_create_block_context(node),
                IrOpcode::kJSCreateGeneratorObject => self.reduce_js_create_generator_object(node),
                IrOpcode::kJSCreateObject => self.reduce_js_create_object(node),
                IrOpcode::kJSCreateStringWrapper => self.reduce_js_create_string_wrapper(node),
                _ => Reduction::NoChange,
            }
        }

        fn reduce_js_create(&self, node: &mut Node) -> Reduction {
            //DCHECK_EQ(IrOpcode::kJSCreate, node.opcode());
            let new_target = node.value_input(1);
            //Node* const effect = NodeProperties::GetEffectInput(node);
            //Node* const control = NodeProperties::GetControlInput(node);

            let initial_map = NodeProperties::get_js_create_map(self.broker(), node);
            if initial_map.is_none() {
                return Reduction::NoChange;
            }

            /*
            JSFunctionRef original_constructor =
                HeapObjectMatcher(new_target).Ref(broker()).AsJSFunction();
            SlackTrackingPrediction slack_tracking_prediction =
                dependencies()->DependOnInitialMapInstanceSizePrediction(
                    original_constructor);

            // Emit code to allocate the JSObject instance for the
            // {original_constructor}.
            AllocationBuilder a(jsgraph(), broker(), effect, control);
            a.Allocate(slack_tracking_prediction.instance_size());
            a.Store(AccessBuilder::ForMap(), *initial_map);
            a.Store(AccessBuilder::ForJSObjectPropertiesOrHashKnownPointer(),
                    jsgraph()->EmptyFixedArrayConstant());
            a.Store(AccessBuilder::ForJSObjectElements(),
                    jsgraph()->EmptyFixedArrayConstant());
            for (int i = 0; i < slack_tracking_prediction.inobject_property_count();
                 ++i) {
                a.Store(AccessBuilder::ForJSObjectInObjectProperty(*initial_map, i),
                        jsgraph()->UndefinedConstant());
            }

            RelaxControls(node);
            a.FinishAndChange(node);
            */
            Reduction::Changed(node)
        }

        fn reduce_js_create_arguments(&self, node: &mut Node) -> Reduction {
            //DCHECK_EQ(IrOpcode::kJSCreateArguments, node.opcode());
            //CreateArgumentsType type = CreateArgumentsTypeOf(node->op());
            //FrameState frame_state{NodeProperties::GetFrameStateInput(node)};
            //Node* const control = graph()->start();
            //FrameStateInfo state_info = frame_state.frame_state_info();
            //SharedFunctionInfoRef shared =
            //    MakeRef(broker(), state_info.shared_info().ToHandleChecked());

            // Use the ArgumentsAccessStub for materializing both mapped and unmapped
            // arguments object, but only for non-inlined (i.e. outermost) frames.
            //if (frame_state.outer_frame_state()->opcode() != IrOpcode::kFrameState) {
            //    switch (type) {
            //        case CreateArgumentsType::kMappedArguments: {
            //            // TODO(turbofan): Duplicate parameters are not handled yet.
            //            if (shared.has_duplicate_parameters()) return NoChange();
            //            Node* const callee = NodeProperties::GetValueInput(node, 0);
            //            Node* const context = NodeProperties::GetContextInput(node);
            //            Node* effect = NodeProperties::GetEffectInput(node);
            //            Node* const arguments_length =
            //                graph()->NewNode(simplified()->ArgumentsLength());
            //            // Allocate the elements backing store.
            //            bool has_aliased_arguments = false;
            //            Node* const elements = effect = TryAllocateAliasedArguments(
            //                effect, control, context, arguments_length, shared,
            //                &has_aliased_arguments);
            //            if (elements == nullptr) return NoChange();
            //
            //            // Load the arguments object map.
            //            Node* const arguments_map = jsgraph()->ConstantNoHole(
            //                has_aliased_arguments
            //                    ? native_context().fast_aliased_arguments_map(broker())
            //                    : native_context().sloppy_arguments_map(broker()),
            //                broker());
            //            // Actually allocate and initialize the arguments object.
            //            AllocationBuilder a(jsgraph(), broker(), effect, control);
            //            static_assert(JSSloppyArgumentsObject::kSize == 5 * kTaggedSize);
            //            a.Allocate(JSSloppyArgumentsObject::kSize);
            //            a.Store(AccessBuilder::ForMap(), arguments_map);
            //            a.Store(AccessBuilder::ForJSObjectPropertiesOrHashKnownPointer(),
            //                    jsgraph()->EmptyFixedArrayConstant());
            //            a.Store(AccessBuilder::ForJSObjectElements(), elements);
            //            a.Store(AccessBuilder::ForArgumentsLength(), arguments_length);
            //            a.Store(AccessBuilder::ForArgumentsCallee(), callee);
            //            RelaxControls(node);
            //            a.FinishAndChange(node);
            //            return Changed(node);
            //        }
            //        case CreateArgumentsType::kUnmappedArguments: {
            //            Node* effect = NodeProperties::GetEffectInput(node);
            //            Node* const arguments_length =
            //                graph()->NewNode(simplified()->ArgumentsLength());
            //            // Allocate the elements backing store.
            //            Node* const elements = effect = graph()->NewNode(
            //                simplified()->NewArgumentsElements(
            //                    CreateArgumentsType::kUnmappedArguments,
            //                    shared.internal_formal_parameter_count_without_receiver()),
            //                arguments_length, effect);
            //            // Load the arguments object map.
            //            Node* const arguments_map = jsgraph()->ConstantNoHole(
            //                native_context().strict_arguments_map(broker()), broker());
            //            // Actually allocate and initialize the arguments object.
            //            AllocationBuilder a(jsgraph(), broker(), effect, control);
            //            static_assert(JSStrictArgumentsObject::kSize == 4 * kTaggedSize);
            //            a.Allocate(JSStrictArgumentsObject::kSize);
            //            a.Store(AccessBuilder::ForMap(), arguments_map);
            //            a.Store(AccessBuilder::ForJSObjectPropertiesOrHashKnownPointer(),
            //                    jsgraph()->EmptyFixedArrayConstant());
            //            a.Store(AccessBuilder::ForJSObjectElements(), elements);
            //            a.Store(AccessBuilder::ForArgumentsLength(), arguments_length);
            //            RelaxControls(node);
            //            a.FinishAndChange(node);
            //            return Changed(node);
            //        }
            //        case CreateArgumentsType::kRestParameter: {
            //            Node* effect = NodeProperties::GetEffectInput(node);
            //            Node* const arguments_length =
            //                graph()->NewNode(simplified()->ArgumentsLength());
            //            Node* const rest_length = graph()->NewNode(simplified()->RestLength(
            //                shared.internal_formal_parameter_count_without_receiver()));
            //            // Allocate the elements backing store.
            //            Node* const elements = effect = graph()->NewNode(
            //                simplified()->NewArgumentsElements(
            //                    CreateArgumentsType::kRestParameter,
            //                    shared.internal_formal_parameter_count_without_receiver()),
            //                arguments_length, effect);
            //            // Load the JSArray object map.
            //            Node* const jsarray_map = jsgraph()->ConstantNoHole(
            //                native_context().js_array_packed_elements_map(broker()), broker());
            //            // Actually allocate and initialize the jsarray.
            //            AllocationBuilder a(jsgraph(), broker(), effect, control);
            //            static_assert(JSArray::kHeaderSize == 4 * kTaggedSize);
            //            a.Allocate(JSArray::kHeaderSize);
            //            a.Store(AccessBuilder::ForMap(), jsarray_map);
            //            a.Store(AccessBuilder::ForJSObjectPropertiesOrHashKnownPointer(),
            //                    jsgraph()->EmptyFixedArrayConstant());
            //            a.Store(AccessBuilder::ForJSObjectElements(), elements);
            //            a.Store(AccessBuilder::ForJSArrayLength(PACKED_ELEMENTS), rest_length);
            //            RelaxControls(node);
            //            a.FinishAndChange(node);
            //            return Changed(node);
            //        }
            //    }
            //    //UNREACHABLE();
            //}
            //// Use inline allocation for all mapped arguments objects within inlined
            //// (i.e. non-outermost) frames, independent of the object size.
            //DCHECK_EQ(frame_state.outer_frame_state()->opcode(), IrOpcode::kFrameState);
            //switch (type) {
            //    case CreateArgumentsType::kMappedArguments: {
            //        Node* const callee = NodeProperties::GetValueInput(node, 0);
            //        Node* const context = NodeProperties::GetContextInput(node);
            //        Node* effect = NodeProperties::GetEffectInput(node);
            //        // TODO(turbofan): Duplicate parameters are not handled yet.
            //        if (shared.has_duplicate_parameters()) return NoChange();
            //        // Choose the correct frame state and frame state info depending on
            //        // whether there conceptually is an inlined arguments frame in the call
            //        // chain.
            //        FrameState args_state = GetArgumentsFrameState(frame_state);
            //        if (args_state.parameters()->opcode() == IrOpcode::kDeadValue) {
            //            // This protects against an incompletely propagated DeadValue node.
            //            // If the FrameState has a DeadValue input, then this node will be
            //            // pruned anyway.
            //            return NoChange();
            //        }
            //        FrameStateInfo args_state_info = args_state.frame_state_info();
            //        int length = args_state_info.parameter_count() - 1;  // Minus receiver.
            //        // Prepare element backing store to be used by arguments object.
            //        bool has_aliased_arguments = false;
            //        Node* const elements = TryAllocateAliasedArguments(
            //            effect, control, args_state, context, shared, &has_aliased_arguments);
            //        if (elements == nullptr) return NoChange();
            //        effect = elements->op()->EffectOutputCount() > 0 ? elements : effect;
            //        // Load the arguments object map.
            //        Node* const arguments_map = jsgraph()->ConstantNoHole(
            //            has_aliased_arguments
            //                ? native_context().fast_aliased_arguments_map(broker())
            //                : native_context().sloppy_arguments_map(broker()),
            //            broker());
            //        // Actually allocate and initialize the arguments object.
            //        AllocationBuilder a(jsgraph(), broker(), effect, control);
            //        static_assert(JSSloppyArgumentsObject::kSize == 5 * kTaggedSize);
            //        a.Allocate(JSSloppyArgumentsObject::kSize);
            //        a.Store(AccessBuilder::ForMap(), arguments_map);
            //        a.Store(AccessBuilder::ForJSObjectPropertiesOrHashKnownPointer(),
            //                jsgraph()->EmptyFixedArrayConstant());
            //        a.Store(AccessBuilder::ForJSObjectElements(), elements);
            //        a.Store(AccessBuilder::ForArgumentsLength(),
            //                jsgraph()->ConstantNoHole(length));
            //        a.Store(AccessBuilder::ForArgumentsCallee(), callee);
            //        RelaxControls(node);
            //        a.FinishAndChange(node);
            //        return Changed(node);
            //    }
            //    case CreateArgumentsType::kUnmappedArguments: {
            //        // Use inline allocation for all unmapped arguments objects within inlined
            //        // (i.e. non-outermost) frames, independent of the object size.
            //        Node* effect = NodeProperties::GetEffectInput(node);
            //        // Choose the correct frame state and frame state info depending on
            //        // whether there conceptually is an inlined arguments frame in the call
            //        // chain.
            //        FrameState args_state = GetArgumentsFrameState(frame_state);
            //        if (args_state.parameters()->opcode() == IrOpcode::kDeadValue) {
            //            // This protects against an incompletely propagated DeadValue node.
            //            // If the FrameState has a DeadValue input, then this node will be
            //            // pruned anyway.
            //            return NoChange();
            //        }
            //        FrameStateInfo args_state_info = args_state.frame_state_info();
            //        int length = args_state_info.parameter_count() - 1;  // Minus receiver.
            //        // Prepare element backing store to be used by arguments object.
            //        Node* const elements = TryAllocateArguments(effect, control, args_state);
            //        if (elements == nullptr) return NoChange();
            //        effect = elements->op()->EffectOutputCount() > 0 ? elements : effect;
            //        // Load the arguments object map.
            //        Node* const arguments_map = jsgraph()->ConstantNoHole(
            //            native_context().strict_arguments_map(broker()), broker());
            //        // Actually allocate and initialize the arguments object.
            //        AllocationBuilder a(jsgraph(), broker(), effect, control);
            //        static_assert(JSStrictArgumentsObject::kSize == 4 * kTaggedSize);
            //        a.Allocate(JSStrictArgumentsObject::kSize);
            //        a.Store(AccessBuilder::ForMap(), arguments_map);
            //        a.Store(AccessBuilder::ForJSObjectPropertiesOrHashKnownPointer(),
            //                jsgraph()->EmptyFixedArrayConstant());
            //        a.Store(AccessBuilder::ForJSObjectElements(), elements);
            //        a.Store(AccessBuilder::ForArgumentsLength(),
            //                jsgraph()->ConstantNoHole(length));
            //        RelaxControls(node);
            //        a.FinishAndChange(node);
            //        return Changed(node);
            //    }
            //    case CreateArgumentsType::kRestParameter: {
            //        int start_index =
            //            shared.internal_formal_parameter_count_without_receiver();
            //        // Use inline allocation for all unmapped arguments objects within inlined
            //        // (i.e. non-outermost) frames, independent of the object size.
            //        Node* effect = NodeProperties::GetEffectInput(node);
            //        // Choose the correct frame state and frame state info depending on
            //        // whether there conceptually is an inlined arguments frame in the call
            //        // chain.
            //        FrameState args_state = GetArgumentsFrameState(frame_state);
            //        if (args_state.parameters()->opcode() == IrOpcode::kDeadValue) {
            //            // This protects against an incompletely propagated DeadValue node.
            //            // If the FrameState has a DeadValue input, then this node will be
            //            // pruned anyway.
            //            return NoChange();
            //        }
            //        FrameStateInfo args_state_info = args_state.frame_state_info();
            //        // Prepare element backing store to be used by the rest array.
            //        Node* const elements =
            //            TryAllocateRestArguments(effect, control, args_state, start_index);
            //        if (elements == nullptr) return NoChange();
            //        effect = elements->op()->EffectOutputCount() > 0 ? elements : effect;
            //        // Load the JSArray object map.
            //        Node* const jsarray_map = jsgraph()->ConstantNoHole(
            //            native_context().js_array_packed_elements_map(broker()), broker());
            //        // Actually allocate and initialize the jsarray.
            //        AllocationBuilder a(jsgraph(), broker(), effect, control);
            //
            //        // -1 to minus receiver
            //        int argument_count = args_state_info.parameter_count() - 1;
            //        int length = std::max(0, argument_count - start_index);
            //        static_assert(JSArray::kHeaderSize == 4 * kTaggedSize);
            //        a.Allocate(ALIGN_TO_ALLOCATION_ALIGNMENT(JSArray::kHeaderSize));
            //        a.Store(AccessBuilder::ForMap(), jsarray_map);
            //        a.Store(AccessBuilder::ForJSObjectPropertiesOrHashKnownPointer(),
            //                jsgraph()->EmptyFixedArrayConstant());
            //        a.Store(AccessBuilder::ForJSObjectElements(), elements);
            //        a.Store(AccessBuilder::ForJSArrayLength(PACKED_ELEMENTS),
            //                jsgraph()->ConstantNoHole(length));
            //        RelaxControls(node);
            //        a.FinishAndChange(node);
            //        return Changed(node);
            //    }
            //}
            //UNREACHABLE();

            Reduction::NoChange
        }

        fn reduce_js_create_generator_object(&self, node: &mut Node) -> Reduction {
            //DCHECK_EQ(IrOpcode::kJSCreateGeneratorObject, node->opcode());
            //Node* const closure = NodeProperties::GetValueInput(node, 0);
            //Node* const receiver = NodeProperties::GetValueInput(node, 1);
            //Node* const context = NodeProperties::GetContextInput(node);
            //Type const closure_type = NodeProperties::GetType(closure);
            //Node* effect = NodeProperties::GetEffectInput(node);
            //Node* const control = NodeProperties::GetControlInput(node);
            //if (closure_type.IsHeapConstant()) {
            //    DCHECK(closure_type.AsHeapConstant()->Ref().IsJSFunction());
            //    JSFunctionRef js_function =
            //        closure_type.AsHeapConstant()->Ref().AsJSFunction();
            //    if (!js_function.has_initial_map(broker())) return NoChange();
            //
            //    SlackTrackingPrediction slack_tracking_prediction =
            //        dependencies()->DependOnInitialMapInstanceSizePrediction(js_function);
            //
            //    MapRef initial_map = js_function.initial_map(broker());
            //    DCHECK(initial_map.instance_type() == JS_GENERATOR_OBJECT_TYPE ||
            //           initial_map.instance_type() == JS_ASYNC_GENERATOR_OBJECT_TYPE);
            //
            //    // Allocate a register file.
            //    SharedFunctionInfoRef shared = js_function.shared(broker());
            //    DCHECK(shared.HasBytecodeArray());
            //    int parameter_count_no_receiver =
            //        shared.internal_formal_parameter_count_without_receiver();
            //    int length = parameter_count_no_receiver +
            //                 shared.GetBytecodeArray(broker()).register_count();
            //    MapRef fixed_array_map = broker()->fixed_array_map();
            //    AllocationBuilder ab(jsgraph(), broker(), effect, control);
            //    if (!ab.CanAllocateArray(length, fixed_array_map)) {
            //        return NoChange();
            //    }
            //    ab.AllocateArray(length, fixed_array_map);
            //    for (int i = 0; i < length; ++i) {
            //        ab.Store(AccessBuilder::ForFixedArraySlot(i),
            //                 jsgraph()->UndefinedConstant());
            //    }
            //    Node* parameters_and_registers = effect = ab.Finish();
            //
            //    // Emit code to allocate the JS[Async]GeneratorObject instance.
            //    AllocationBuilder a(jsgraph(), broker(), effect, control);
            //    a.Allocate(slack_tracking_prediction.instance_size());
            //    Node* undefined = jsgraph()->UndefinedConstant();
            //    a.Store(AccessBuilder::ForMap(), initial_map);
            //    a.Store(AccessBuilder::ForJSObjectPropertiesOrHashKnownPointer(),
            //            jsgraph()->EmptyFixedArrayConstant());
            //    a.Store(AccessBuilder::ForJSObjectElements(),
            //            jsgraph()->EmptyFixedArrayConstant());
            //    a.Store(AccessBuilder::ForJSGeneratorObjectContext(), context);
            //    a.Store(AccessBuilder::ForJSGeneratorObjectFunction(), closure);
            //    a.Store(AccessBuilder::ForJSGeneratorObjectReceiver(), receiver);
            //    a.Store(AccessBuilder::ForJSGeneratorObjectInputOrDebugPos(), undefined);
            //    a.Store(AccessBuilder::ForJSGeneratorObjectResumeMode(),
            //            jsgraph()->ConstantNoHole(JSGeneratorObject::kNext));
            //    a.Store(AccessBuilder::ForJSGeneratorObjectContinuation(),
            //            jsgraph()->ConstantNoHole(JSGeneratorObject::kGeneratorExecuting));
            //    a.Store(AccessBuilder::ForJSGeneratorObjectParametersAndRegisters(),
            //            parameters_and_registers);
            //
            //    if (initial_map.instance_type() == JS_ASYNC_GENERATOR_OBJECT_TYPE) {
            //        a.Store(AccessBuilder::ForJSAsyncGeneratorObjectQueue(), undefined);
            //        a.Store(AccessBuilder::ForJSAsyncGeneratorObjectIsAwaiting(),
            //                jsgraph()->ZeroConstant());
            //    }
            //
            //    // Handle in-object properties, too.
            //    for (int i = 0; i < slack_tracking_prediction.inobject_property_count();
            //         ++i) {
            //        a.Store(AccessBuilder::ForJSObjectInObjectProperty(initial_map, i),
            //                undefined);
            //    }
            //    a.FinishAndChange(node);
            //    return Changed(node);
            //}
            Reduction::NoChange
        }

        fn reduce_new_array(
            &self,
            node: &mut Node,
            length: &mut Node,
            initial_map: MapRef,
            elements_kind: ElementsKind,
            allocation: AllocationType,
            slack_tracking_prediction: &SlackTrackingPrediction,
        ) -> Reduction {
            //DCHECK_EQ(IrOpcode::kJSCreateArray, node.opcode());
            //Node* effect = NodeProperties::GetEffectInput(node);
            //Node* control = NodeProperties::GetControlInput(node);

            //// Constructing an Array via new Array(N) where N is an unsigned
            //// integer, always creates a holey backing store.
            //OptionalMapRef maybe_initial_map =
            //    initial_map.AsElementsKind(broker(), GetHoleyElementsKind(elements_kind));
            //if (!maybe_initial_map.has_value()) return NoChange();
            //initial_map = maybe_initial_map.value();

            //// Because CheckBounds performs implicit conversion from string to number, an
            //// additional CheckNumber is required to behave correctly for calls with a
            //// single string argument.
            //length = effect = graph()->NewNode(
            //    simplified()->CheckNumber(FeedbackSource{}), length, effect, control);

            //// Check that the {limit} is an unsigned integer in the valid range.
            //// This has to be kept in sync with src/runtime/runtime-array.cc,
            //// where this limit is protected.
            //length = effect = graph()->NewNode(
            //    simplified()->CheckBounds(FeedbackSource()), length,
            //    jsgraph()->ConstantNoHole(JSArray::kInitialMaxFastElementArray), effect,
            //    control);

            //// Construct elements and properties for the resulting JSArray.
            //Node* elements = effect =
            //    graph()->NewNode(IsDoubleElementsKind(initial_map.elements_kind())
            //                         ? simplified()->NewDoubleElements(allocation)
            //                         : simplified()->NewSmiOrObjectElements(allocation),
            //                     length, effect, control);

            //// Perform the allocation of the actual JSArray object.
            //AllocationBuilder a(jsgraph(), broker(), effect, control);
            //a.Allocate(slack_tracking_prediction.instance_size(), allocation);
            //a.Store(AccessBuilder::ForMap(), initial_map);
            //a.Store(AccessBuilder::ForJSObjectPropertiesOrHashKnownPointer(),
            //        jsgraph()->EmptyFixedArrayConstant());
            //a.Store(AccessBuilder::ForJSObjectElements(), elements);
            //a.Store(AccessBuilder::ForJSArrayLength(initial_map.elements_kind()), length);
            //for (int i = 0; i < slack_tracking_prediction.inobject_property_count();
            //     ++i) {
            //    a.Store(AccessBuilder::ForJSObjectInObjectProperty(initial_map, i),
            //            jsgraph()->UndefinedConstant());
            //}
            //RelaxControls(node);
            //a.FinishAndChange(node);

            Reduction::Changed(node)
        }

        fn reduce_new_array1(
            &self,
            node: &mut Node,
            length: &mut Node,
            capacity: i32,
            initial_map: MapRef,
            elements_kind: ElementsKind,
            allocation: AllocationType,
            slack_tracking_prediction: &SlackTrackingPrediction,
        ) -> Reduction {
            //DCHECK(node->opcode() == IrOpcode::kJSCreateArray ||
            //     node->opcode() == IrOpcode::kJSCreateEmptyLiteralArray);
            //DCHECK(NodeProperties::GetType(length).Is(Type::Number()));
            //Node* effect = NodeProperties::GetEffectInput(node);
            //Node* control = NodeProperties::GetControlInput(node);

            //// Determine the appropriate elements kind.
            //if (NodeProperties::GetType(length).Max() > 0.0) {
            //    elements_kind = GetHoleyElementsKind(elements_kind);
            //}

            //OptionalMapRef maybe_initial_map =
            //    initial_map.AsElementsKind(broker(), elements_kind);
            //if (!maybe_initial_map.has_value()) return NoChange();
            //initial_map = maybe_initial_map.value();

            //DCHECK(IsFastElementsKind(elements_kind));

            //// Setup elements and properties.
            //Node* elements;
            //if (capacity == 0) {
            //    elements = jsgraph()->EmptyFixedArrayConstant();
            //} else {
            //    elements = effect =
            //        AllocateElements(effect, control, elements_kind, capacity, allocation);
            //}

            //// Perform the allocation of the actual JSArray object.
            //AllocationBuilder a(jsgraph(), broker(), effect, control);
            //a.Allocate(slack_tracking_prediction.instance_size(), allocation);
            //a.Store(AccessBuilder::ForMap(), initial_map);
            //a.Store(AccessBuilder::ForJSObjectPropertiesOrHashKnownPointer(),
            //        jsgraph()->EmptyFixedArrayConstant());
            //a.Store(AccessBuilder::ForJSObjectElements(), elements);
            //a.Store(AccessBuilder::ForJSArrayLength(elements_kind), length);
            //for (int i = 0; i < slack_tracking_prediction.inobject_property_count();
            //     ++i) {
            //    a.Store(AccessBuilder::ForJSObjectInObjectProperty(initial_map, i),
            //            jsgraph()->UndefinedConstant());
            //}
            //RelaxControls(node);
            //a.FinishAndChange(node);

            Reduction::Changed(node)
        }

        fn reduce_new_array2(
            &self,
            node: &mut Node,
            values: Vec<&mut Node>,
            initial_map: MapRef,
            elements_kind: ElementsKind,
            allocation: AllocationType,
            slack_tracking_prediction: &SlackTrackingPrediction,
        ) -> Reduction {
            //DCHECK_EQ(IrOpcode::kJSCreateArray, node->opcode());
            //Node* effect = NodeProperties::GetEffectInput(node);
            //Node* control = NodeProperties::GetControlInput(node);

            //// Determine the appropriate elements kind.
            //DCHECK(IsFastElementsKind(elements_kind));

            //OptionalMapRef maybe_initial_map =
            //    initial_map.AsElementsKind(broker(), elements_kind);
            //if (!maybe_initial_map.has_value()) return NoChange();
            //initial_map = maybe_initial_map.value();

            //// Check {values} based on the {elements_kind}. These checks are guarded
            //// by the {elements_kind} feedback on the {site}, so it's safe to just
            //// deoptimize in this case.
            //if (IsSmiElementsKind(elements_kind)) {
            //    for (auto& value : values) {
            //        if (!NodeProperties::GetType(value).Is(Type::SignedSmall())) {
            //            value = effect = graph()->NewNode(
            //                simplified()->CheckSmi(FeedbackSource()), value, effect, control);
            //        }
            //    }
            //} else if (IsDoubleElementsKind(elements_kind)) {
            //    for (auto& value : values) {
            //        if (!NodeProperties::GetType(value).Is(Type::Number())) {
            //            value = effect =
            //                graph()->NewNode(simplified()->CheckNumber(FeedbackSource()), value,
            //                                 effect, control);
            //        }
            //        // Make sure we do not store signaling NaNs into double arrays.
            //        value = graph()->NewNode(simplified()->NumberSilenceNaN(), value);
            //    }
            //}

            //// Setup elements, properties and length.
            //Node* elements = effect =
            //    AllocateElements(effect, control, elements_kind, values, allocation);
            //Node* length = jsgraph()->ConstantNoHole(static_cast<int>(values.size()));

            //// Perform the allocation of the actual JSArray object.
            //AllocationBuilder a(jsgraph(), broker(), effect, control);
            //a.Allocate(slack_tracking_prediction.instance_size(), allocation);
            //a.Store(AccessBuilder::ForMap(), initial_map);
            //a.Store(AccessBuilder::ForJSObjectPropertiesOrHashKnownPointer(),
            //        jsgraph()->EmptyFixed