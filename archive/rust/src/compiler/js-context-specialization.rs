// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/compiler/js-context-specialization.h (module definition)
mod js_context_specialization {
    use std::rc::Rc;

    use crate::compiler::{
        access_builder::AccessBuilder,
        common_operator::CommonOperatorBuilder,
        compilation_dependencies::CompilationDependencies,
        feedback_source::FeedbackSource,
        js_graph::JsGraph,
        js_heap_broker::JSHeapBroker,
        js_operator::JSOperatorBuilder,
        linkage::ParameterIndexOf,
        node_properties::NodeProperties,
        property_access_builder::PropertyAccessBuilder,
        simplified_operator::SimplifiedOperatorBuilder,
    };
    use crate::deoptimizer::deoptimize_reason::DeoptimizeReason;
    use crate::objects::contexts::Context;
    use crate::objects::property_cell::ContextSidePropertyCell;
    use crate::v8::{HeapObjectRef, Isolate, MakeRef, MakeRefAssumeMemoryFence, OptionalObjectRef};
    use std::convert::TryInto;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IrOpcode {
        Parameter,
        JSLoadContext,
        JSLoadScriptContext,
        JSStoreContext,
        JSStoreScriptContext,
        JSGetImportMeta,
        HeapConstant,
        // Add other opcodes as needed
    }

    #[derive(Clone, Copy, Debug)]
    pub struct ContextAccess {
        depth: usize,
        index: usize,
        immutable: bool,
    }

    impl ContextAccess {
        pub fn new(depth: usize, index: usize, immutable: bool) -> Self {
            ContextAccess {
                depth,
                index,
                immutable,
            }
        }

        pub fn depth(&self) -> usize {
            self.depth
        }

        pub fn index(&self) -> usize {
            self.index
        }

        pub fn immutable(&self) -> bool {
            self.immutable
        }
    }

    pub struct OuterContext {
        distance: usize,
        context: Rc<Context>, // Assuming Context is heap-allocated
    }

    impl OuterContext {
        pub fn new(distance: usize, context: Rc<Context>) -> Self {
            OuterContext { distance, context }
        }
    }

    pub type OptionalContextRef = Option<HeapObjectRef>;

    pub struct ReductionResult {
        changed: bool,
        replacement: Option<NodeId>,
    }

    impl ReductionResult {
        pub fn no_change() -> Self {
            ReductionResult {
                changed: false,
                replacement: None,
            }
        }

        pub fn changed(replacement: NodeId) -> Self {
            ReductionResult {
                changed: true,
                replacement: Some(replacement),
            }
        }
    }

    // Node ID is used as a substitute for Node*
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct NodeId(usize); // Using usize as a simple ID

    // Dummy implementation for node related functions
    // NodeProperties::GetContextInput(node)
    fn get_context_input(_node: NodeId) -> NodeId {
        NodeId(0)
    }
    // NodeProperties::GetOuterContext(node, depth)
    fn get_outer_context(_node: NodeId, depth: &mut usize) -> NodeId {
        NodeId(0)
    }
    fn get_value_input(_node: NodeId, _idx: usize) -> NodeId {
        NodeId(0)
    }
    // NodeProperties::ReplaceContextInput(node, new_context)
    fn replace_context_input(_node: NodeId, _new_context: NodeId) {}
    // NodeProperties::ChangeOp(node, op)
    fn change_op(_node: NodeId, _op: &Operator) {}

    // NodeProperties::GetValueInput
    fn node_properties_get_value_input(node: NodeId, index: usize) -> NodeId {
        // Placeholder implementation
        println!("NodeProperties::GetValueInput({}, {})", node.0, index);
        NodeId(0) // Dummy return value
    }

    struct StartNode {
        node_id: NodeId,
    }

    impl StartNode {
        fn new(node_id: NodeId) -> Self {
            StartNode { node_id }
        }

        fn context_parameter_index_maybe_non_standard_layout(&self) -> usize {
            // Placeholder implementation
            println!(
                "StartNode::ContextParameterIndex_MaybeNonStandardLayout({})",
                self.node_id.0
            );
            0 // Dummy return value
        }
    }

    // dummy Effect and Control struct
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Effect {
        id: usize,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Control {
        id: usize,
    }

    // Dummy functions for effect/control input
    fn node_properties_get_effect_input(node: NodeId) -> Effect {
        Effect { id: 0 }
    }
    fn node_properties_get_control_input(node: NodeId) -> Control {
        Control { id: 0 }
    }

    // Dummy function for NodeProperties::GetValueInput
    fn node_properties_get_value_input_effect_control(node: NodeId) -> NodeId {
        NodeId(0) // Placeholder
    }

    #[derive(Debug)]
    pub struct Operator {
        opcode: IrOpcode,
        // Add other operator fields as needed
    }

    impl Operator {
        pub fn opcode(&self) -> IrOpcode {
            self.opcode
        }
    }

    fn parameter_index_of(_op: &Operator) -> i32 {
        0
    }

    fn heap_constant_of(_op: &Operator) -> String {
        "Heap Constant".to_string()
    }

    fn context_access_of(_op: &Operator) -> ContextAccess {
        ContextAccess {
            depth: 0,
            index: 0,
            immutable: false,
        }
    }

    struct V8Flags {
        script_context_mutable_heap_number: bool,
        const_tracking_let: bool,
    }

    impl V8Flags {
        fn new() -> Self {
            V8Flags {
                script_context_mutable_heap_number: false,
                const_tracking_let: false,
            }
        }
    }

    lazy_static::lazy_static! {
        static ref V8_FLAGS: V8Flags = V8Flags::new();
    }

    pub struct JSContextSpecialization {
        jsgraph_: Rc<JsGraph>,
        broker_: Rc<JSHeapBroker>,
        closure_: Option<Rc<Context>>, // Assuming Closure is Rc<JSFunction>
        outer_: Option<OuterContext>,
    }

    impl JSContextSpecialization {
        pub fn new(
            jsgraph: Rc<JsGraph>,
            broker: Rc<JSHeapBroker>,
            closure: Option<Rc<Context>>,
            outer: Option<OuterContext>,
        ) -> Self {
            JSContextSpecialization {
                jsgraph_: jsgraph,
                broker_: broker,
                closure_: closure,
                outer_: outer,
            }
        }

        fn jsgraph(&self) -> &Rc<JsGraph> {
            &self.jsgraph_
        }

        fn broker(&self) -> &Rc<JSHeapBroker> {
            &self.broker_
        }

        fn closure(&self) -> &Option<Rc<Context>> {
            &self.closure_
        }

        fn outer(&self) -> &Option<OuterContext> {
            &self.outer_
        }

        pub fn reduce(&self, node: NodeId) -> ReductionResult {
            let opcode = self.get_opcode(node);

            match opcode {
                IrOpcode::Parameter => self.reduce_parameter(node),
                IrOpcode::JSLoadContext => self.reduce_js_load_context(node),
                IrOpcode::JSLoadScriptContext => self.reduce_js_load_script_context(node),
                IrOpcode::JSStoreContext => self.reduce_js_store_context(node),
                IrOpcode::JSStoreScriptContext => self.reduce_js_store_script_context(node),
                IrOpcode::JSGetImportMeta => self.reduce_js_get_import_meta(node),
                _ => ReductionResult::no_change(),
            }
        }

        fn get_opcode(&self, node: NodeId) -> IrOpcode {
            // Dummy implementation
            IrOpcode::Parameter
        }

        fn reduce_parameter(&self, node: NodeId) -> ReductionResult {
            let index = parameter_index_of(self.get_operator(node));
            if index == 0 {
                //Linkage::kJSCallClosureParamIndex
                if let Some(function) = &self.closure() {
                    // If closure is actually JSFunction, then it should be handled like this.
                    // let value = self.jsgraph().constant_no_hole(MakeRef(self.broker(), function), self.broker());
                    // Replace(value)
                    return ReductionResult::changed(NodeId(1)); // Dummy NodeId
                }
            }
            ReductionResult::no_change()
        }

        fn get_operator(&self, _node: NodeId) -> &Operator {
            // Dummy implementation
            unimplemented!()
        }

        fn simplify_js_load_context(
            &self,
            node: NodeId,
            new_context: NodeId,
            new_depth: usize,
        ) -> ReductionResult {
            let access = context_access_of(self.get_operator(node));
            if new_depth == access.depth() && new_context == get_context_input(node) {
                return ReductionResult::no_change();
            }

            // let op = self.jsgraph_.javascript().load_context(
            //     new_depth,
            //     access.index(),
            //     access.immutable(),
            // );
            replace_context_input(node, new_context);
            // change_op(node, op);

            ReductionResult::changed(node)
        }

        fn simplify_js_load_script_context(
            &self,
            node: NodeId,
            new_context: NodeId,
            new_depth: usize,
        ) -> ReductionResult {
            let access = context_access_of(self.get_operator(node));
            if new_depth == access.depth() && new_context == get_context_input(node) {
                return ReductionResult::no_change();
            }

            // let op = self
            //     .jsgraph_
            //     .javascript()
            //     .load_script_context(new_depth, access.index());
            replace_context_input(node, new_context);
            // change_op(node, op);
            ReductionResult::changed(node)
        }

        fn simplify_js_store_context(
            &self,
            node: NodeId,
            new_context: NodeId,
            new_depth: usize,
        ) -> ReductionResult {
            let access = context_access_of(self.get_operator(node));
            if new_depth == access.depth() && new_context == get_context_input(node) {
                return ReductionResult::no_change();
            }

            // let op = self
            //     .jsgraph_
            //     .javascript()
            //     .store_context(new_depth, access.index());
            replace_context_input(node, new_context);
            // change_op(node, op);
            ReductionResult::changed(node)
        }

        fn simplify_js_store_script_context(
            &self,
            node: NodeId,
            new_context: NodeId,
            new_depth: usize,
        ) -> ReductionResult {
            let access = context_access_of(self.get_operator(node));
            if new_depth == access.depth() && new_context == get_context_input(node) {
                return ReductionResult::no_change();
            }

            // let op = self
            //     .jsgraph_
            //     .javascript()
            //     .store_script_context(new_depth, access.index());
            replace_context_input(node, new_context);
            // change_op(node, op);
            ReductionResult::changed(node)
        }

        fn reduce_js_load_context(&self, node: NodeId) -> ReductionResult {
            let access = context_access_of(self.get_operator(node));
            let mut depth = access.depth();
            let context = get_outer_context(node, &mut depth);

            let maybe_concrete = Self::get_specialization_context(
                self.broker(),
                context,
                &mut depth,
                self.outer().cloned(),
            );
            if maybe_concrete.is_none() {
                return self.simplify_js_load_context(node, context, depth);
            }

            let mut concrete = maybe_concrete.unwrap();
            //concrete = concrete.previous(self.broker(), &mut depth);

            if depth > 0 {
                // Replace with dummy values
                return self.simplify_js_load_context(
                    node,
                    NodeId(1), //self.jsgraph().constant_no_hole(concrete, self.broker()),
                    depth,
                );
            }

            if !access.immutable() {
                // !self.broker()
                //     .dependencies()
                //     .depend_on_script_context_slot_property(
                //         concrete,
                //         access.index(),
                //         ContextSidePropertyCell::kConst,
                //         self.broker(),
                //     )
                {
                    return self.simplify_js_load_context(
                        node,
                        NodeId(1), //self.jsgraph().constant_no_hole(concrete, self.broker()),
                        depth,
                    );
                }
            }
            // This will hold the final value, if we can figure it out.
            //let maybe_value;
            //maybe_value = concrete.get(self.broker(), access.index().try_into().unwrap());

            //replace_with_value(node, constant);
            ReductionResult::changed(NodeId(1))
        }

        fn reduce_js_load_script_context(&self, node: NodeId) -> ReductionResult {
            let access = context_access_of(self.get_operator(node));
            let mut depth = access.depth();
            let _effect = node_properties_get_effect_input(node);
            let _control = node_properties_get_control_input(node);
            let context = get_outer_context(node, &mut depth);

            let maybe_concrete = Self::get_specialization_context(
                self.broker(),
                context,
                &mut depth,
                self.outer().cloned(),
            );
            if !maybe_concrete.is_some() {
                return self.simplify_js_load_script_context(node, context, depth);
            }

            let mut concrete = maybe_concrete.unwrap();
            //concrete = concrete.previous(self.broker(), &mut depth);

            if depth > 0 {
                //TRACE_BROKER_MISSING(broker(), "previous value for context " << concrete);
                return self.simplify_js_load_script_context(
                    node,
                    NodeId(1), //self.jsgraph().constant_no_hole(concrete, self.broker()),
                    depth,
                );
            }

            //DCHECK(concrete.object()->IsScriptContext());
            //auto maybe_property =
            //  concrete.object()->GetScriptContextSideProperty(access.index());
            //if (!maybe_property) {
            //  return SimplifyJSLoadScriptContext(
            //      node, jsgraph()->ConstantNoHole(concrete, broker()), depth);
            //}
            //auto property = maybe_property.value();
            //switch (property) {
            //  case ContextSidePropertyCell::kConst: {
            //    OptionalObjectRef maybe_value =
            //        concrete.get(broker(), static_cast<int>(access.index()));
            //    if (!maybe_value.has_value()) {
            //      TRACE_BROKER_MISSING(broker(), "slot value " << access.index()
            //                                                   << " for context "
            //                                                   << concrete);
            //      return SimplifyJSLoadScriptContext(
            //          node, jsgraph()->ConstantNoHole(concrete, broker()), depth);
            //    }
            //    broker()->dependencies()->DependOnScriptContextSlotProperty(
            //        concrete, access.index(), property, broker());
            //    Node* constant = jsgraph_->ConstantNoHole(*maybe_value, broker());
            //    ReplaceWithValue(node, constant, effect, control);
            //    return Changed(node);
            //  }
            //  case ContextSidePropertyCell::kSmi: {
            //    broker()->dependencies()->DependOnScriptContextSlotProperty(
            //        concrete, access.index(), property, broker());
            //    Node* load = effect = jsgraph_->graph()->NewNode(
            //        jsgraph_->simplified()->LoadField(
            //            AccessBuilder::ForContextSlotSmi(access.index())),
            //        jsgraph_->ConstantNoHole(concrete, broker()), effect, control);
            //    ReplaceWithValue(node, load, effect, control);
            //    return Changed(node);
            //  }
            //  case ContextSidePropertyCell::kMutableInt32: {
            //    Node* mutable_heap_number;
            //    if (auto concrete_heap_number =
            //            concrete.get(broker(), static_cast<int>(access.index()))) {
            //      if (!concrete_heap_number->IsHeapNumber()) {
            //        // TODO(victorgomes): In case the tag is out of date by now we could
            //        // retry this reduction.
            //        return NoChange();
            //      }
            //      mutable_heap_number = jsgraph_->ConstantMutableHeapNumber(
            //          concrete_heap_number->AsHeapNumber(), broker());
            //    } else {
            //      mutable_heap_number = effect = jsgraph_->graph()->NewNode(
            //          jsgraph_->simplified()->LoadField(
            //              AccessBuilder::ForContextSlot(access.index())),
            //          jsgraph_->ConstantNoHole(concrete, broker()), effect, control);
            //    }
            //    broker()->dependencies()->DependOnScriptContextSlotProperty(
            //        concrete, access.index(), property, broker());
            //    Node* int32_load = effect = jsgraph_->graph()->NewNode(
            //        jsgraph_->simplified()->LoadField(AccessBuilder::ForHeapInt32Value()),
            //        mutable_heap_number, effect, control);
            //    ReplaceWithValue(node, int32_load, effect, control);
            //    return Changed(node);
            //  }
            //  case ContextSidePropertyCell::kMutableHeapNumber: {
            //    Node* mutable_heap_number;
            //    if (auto concrete_heap_number =
            //            concrete.get(broker(), static_cast<int>(access.index()))) {
            //      if (!concrete_heap_number->IsHeapNumber()) {
            //        // TODO(victorgomes): In case the tag is out of date by now we could
            //        // retry this reduction.
            //        return NoChange();
            //      }
            //      mutable_heap_number = jsgraph_->ConstantMutableHeapNumber(
            //          concrete_heap_number->AsHeapNumber(), broker());
            //    } else {
            //      mutable_heap_number = effect = jsgraph_->graph()->NewNode(
            //          jsgraph_->simplified()->LoadField(
            //              AccessBuilder::ForContextSlot(access.index())),
            //          jsgraph_->ConstantNoHole(concrete, broker()), effect, control);
            //    }
            //    broker()->dependencies()->DependOnScriptContextSlotProperty(
            //        concrete, access.index(), property, broker());
            //    Node* double_load = effect =
            //        jsgraph_->graph()->NewNode(jsgraph_->simplified()->LoadField(
            //                                       AccessBuilder::ForHeapNumberValue()),
            //                                   mutable_heap_number, effect, control);
            //    ReplaceWithValue(node, double_load, effect, control);
            //    return Changed(node);
            //  }
            //  case ContextSidePropertyCell::kOther: {
            //    // Do a normal context load.
            //    Node* load = effect = jsgraph_->graph()->NewNode(
            //        jsgraph_->simplified()->LoadField(
            //            AccessBuilder::ForContextSlot(access.index())),
            //        jsgraph_->ConstantNoHole(concrete, broker()), effect, control);
            //    ReplaceWithValue(node, load, effect, control);
            //    return Changed(node);
            //  }
            //  default:
            //    UNREACHABLE();
            //}
            ReductionResult::changed(NodeId(1))
        }

        fn reduce_js_store_context(&self, node: NodeId) -> ReductionResult {
            let access = context_access_of(self.get_operator(node));
            let mut depth = access.depth();

            let context = get_outer_context(node, &mut depth);

            let maybe_concrete = Self::get_specialization_context(
                self.broker(),
                context,
                &mut depth,
                self.outer().cloned(),
            );
            if maybe_concrete.is_none() {
                return self.simplify_js_store_context(node, context, depth);
            }

            let mut concrete = maybe_concrete.unwrap();
            //concrete = concrete.previous(self.broker(), &mut depth);

            if depth > 0 {
                //TRACE_BROKER_MISSING(broker(), "previous value for context " << concrete);
                return self.simplify_js_store_context(
                    node,
                    NodeId(1), //self.jsgraph().constant_no_hole(concrete, self.broker()),
                    depth,
                );
            }

            self.simplify_js_store_context(
                node,
                NodeId(1), //self.jsgraph().constant_no_hole(concrete, self.broker()),
                depth,
            )
        }

        fn reduce_js_store_script_context(&self, node: NodeId) -> ReductionResult {
            //DCHECK(v8_flags.script_context_mutable_heap_number ||
            //       v8_flags.const_tracking_let);
            let access = context_access_of(self.get_operator(node));
            let mut depth = access.depth();

            // First walk up the context chain in the graph until we reduce the depth to 0
            // or hit a node that does not have a CreateXYZContext operator.
            let context = get_outer_context(node, &mut depth);
            let _value = node_properties_get_value_input(node, 0);
            let _effect = node_properties_get_effect_input(node);
            let _control = node_properties_get_control_input(node);

            let maybe_concrete = Self::get_specialization_context(
                self.broker(),
                context,
                &mut depth,
                self.outer().cloned(),
            );
            if !maybe_concrete.is_some() {
                // We do not have a concrete context object, so we can only partially reduce
                // the load by folding-in the outer context node.
                return self.simplify_js_store_script_context(node, context, depth);
            }

            let mut concrete = maybe_concrete.unwrap();
            //concrete = concrete.previous(self.broker(), &mut depth);

            if depth > 0 {
                //TRACE_BROKER_MISSING(broker(), "previous value for context " << concrete);
                return self.simplify_js_store_script_context(
                    node,
                    NodeId(1), //self.jsgraph().constant_no_hole(concrete, self.broker()),
                    depth,
                );
            }
            //DCHECK(concrete.object()->IsScriptContext());
            //auto maybe_property =
            //  concrete.object()->GetScriptContextSideProperty(access.index());
            //if (!maybe_property) {
            //  return SimplifyJSStoreScriptContext(
            //      node, jsgraph()->ConstantNoHole(concrete, broker()), depth);
            //}
            //auto property = maybe_property.value();
            //PropertyAccessBuilder access_builder(jsgraph(), broker());
            //if (property == ContextSidePropertyCell::kConst) {
            //  compiler::OptionalObjectRef constant =
            //      concrete.get(broker(), static_cast<int>(access.index()));
            //  if (!constant.has_value() ||
            //      (constant->IsString() && !constant->IsInternalizedString())) {
            //    return SimplifyJSStoreScriptContext(
            //        node, jsgraph()->ConstantNoHole(concrete, broker()), depth);
            //  }
            //  broker()->dependencies()->DependOnScriptContextSlotProperty(
            //      concrete, access.index(), property, broker());
            //  access_builder.BuildCheckValue(value, &effect, control, *constant);
            //  ReplaceWithValue(node, effect, effect, control);
            //  return Changed(node);
            //}

            //if (!v8_flags.script_context_mutable_heap_number) {
            //  // Do a normal context store.
            //  Node* store = jsgraph()->graph()->NewNode(
            //      jsgraph()->simplified()->StoreField(
            //          AccessBuilder::ForContextSlot(access.index())),
            //      jsgraph()->ConstantNoHole(concrete, broker()), value, effect, control);
            //  ReplaceWithValue(node, store, store, control);
            //  return Changed(node);
            //}

            //switch (property) {
            //  case ContextSidePropertyCell::kConst:
            //    UNREACHABLE();
            //  case ContextSidePropertyCell::kSmi: {
            //    broker()->dependencies()->DependOnScriptContextSlotProperty(
            //        concrete, access.index(), property, broker());
            //    Node* smi_value = access_builder.BuildCheckSmi(value, &effect, control);
            //    Node* smi_store = jsgraph()->graph()->NewNode(
            //        jsgraph()->simplified()->StoreField(
            //            AccessBuilder::ForContextSlotSmi(access.index())),
            //        jsgraph()->ConstantNoHole(concrete, broker()), smi_value, effect,
            //        control);
            //    ReplaceWithValue(node, smi_store, smi_store, control);
            //    return Changed(node);
            //  }
            //  case ContextSidePropertyCell::kMutableInt32: {
            //    Node* mutable_heap_number;
            //    if (auto concrete_heap_number =
            //            concrete.get(broker(), static_cast<int>(access.index()))) {
            //      if (!concrete_heap_number->IsHeapNumber()) {
            //        // TODO(victorgomes): In case the tag is out of date by now we could
            //        // retry this reduction.
            //        return NoChange();
            //      }
            //      mutable_heap_number = jsgraph_->ConstantMutableHeapNumber(
            //          concrete_heap_number->AsHeapNumber(), broker());
            //    } else {
            //      mutable_heap_number = effect = jsgraph_->graph()->NewNode(
            //          jsgraph_->simplified()->LoadField(
            //              AccessBuilder::ForContextSlot(access.index())),
            //          jsgraph_->ConstantNoHole(concrete, broker()), effect, control);
            //    }
            //    broker()->dependencies()->DependOnScriptContextSlotProperty(
            //        concrete, access.index(), property, broker());
            //    Node* input_number =
            //        access_builder.BuildCheckNumberFitsInt32(value, &effect, control);
            //    Node* double_store = jsgraph()->graph()->NewNode(
            //        jsgraph()->simplified()->StoreField(
            //            AccessBuilder::ForHeapInt32Value()),
            //        mutable_heap_number, input_number, effect, control);
            //    ReplaceWithValue(node, double_store, double_store, control);
            //    return Changed(node);
            //  }
            //  case ContextSidePropertyCell::kMutableHeapNumber: {
            //    Node* mutable_heap_number;
            //    if (auto concrete_heap_number =
            //            concrete.get(broker(), static_cast<int>(access.index()))) {
            //      if (!concrete_heap_number->IsHeapNumber()) {
            //        // TODO(victorgomes): In case the tag is out of date by now we could
            //        // retry this reduction.
            //        return NoChange();
            //      }
            //      mutable_heap_number = jsgraph_->ConstantMutableHeapNumber(
            //          concrete_heap_number->AsHeapNumber(), broker());
            //    } else {
            //      mutable_heap_number = effect = jsgraph_->graph()->NewNode(
            //          jsgraph_->simplified()->LoadField(
            //              AccessBuilder::ForContextSlot(access.index())),
            //          jsgraph_->ConstantNoHole(concrete, broker()), effect, control);
            //    }
            //    broker()->dependencies()->DependOnScriptContextSlotProperty(
            //        concrete, access.index(), property, broker());
            //    Node* input_number =
            //        access_builder.BuildCheckNumber(value, &effect, control);
            //    Node* double_store = jsgraph()->graph()->NewNode(
            //        jsgraph()->simplified()->StoreField(
            //            AccessBuilder::ForHeapNumberValue()),
            //        mutable_heap_number, input_number, effect, control);
            //    ReplaceWithValue(node, double_store, double_store, control);
            //    return Changed(node);
            //  }
            //  case ContextSidePropertyCell::kOther: {
            //    // Do a normal context store.
            //    Node* store = jsgraph()->graph()->NewNode(
            //        jsgraph()->simplified()->StoreField(
            //            AccessBuilder::ForContextSlot(access.index())),
            //        jsgraph()->ConstantNoHole(concrete, broker()), value, effect,
            //        control);
            //    ReplaceWithValue(node, store, store, control);
            //    return Changed(node);
            //  }
            //  default:
            //    UNREACHABLE();
            //}
            self.simplify_js_store_script_context(
                node,
                NodeId(1), //self.jsgraph().constant_no_hole(concrete, self.broker()),
                depth,
            )
        }

        fn reduce_js_get_import_meta(&self, node: NodeId) -> ReductionResult {
            let maybe_context = Self::get_module_context(self.broker(), node, self.outer().cloned());
            if maybe_context.is_none() {
                return ReductionResult::no_change();
            }

            //let context = maybe_context.unwrap();
            //OptionalObjectRef module = context.get(self.broker(), Context::EXTENSION_INDEX);
            //if (!module.has_value()) return NoChange();
            //OptionalObjectRef import_meta =
            //    module->AsSourceTextModule().import_meta(self.broker());
            //if (!import_meta.has_value()) return NoChange();
            //if (!import_meta->IsJSObject()) {
            //  DCHECK(import_meta->IsTheHole());
            //  // The import.meta object has not yet been created. Let JSGenericLowering
            //  // replace the operator with a runtime call.
            //  return NoChange();
            //}

            //Node* import_meta_const = jsgraph()->ConstantNoHole(*import_meta, self.broker());
            //ReplaceWithValue(node, import_meta_const);
            //return Changed(import_meta_const);
            ReductionResult::changed(NodeId(1))
        }

        fn get_specialization_context(
            broker: &JSHeapBroker,
            node: NodeId,
            distance: &mut usize,
            maybe_outer: Option<OuterContext>,
        ) -> OptionalContextRef {
            let opcode = IrOpcode::Parameter; // Dummy opcode, should come from the node
            match opcode {
                IrOpcode::HeapConstant => {
                    // let object = MakeRefAssumeMemoryFence(broker, HeapConstantOf(node.op()));
                    // if (object.IsContext()) return object.AsContext();
                    None
                }
                IrOpcode::Parameter => {
                    if let Some(outer) = maybe_outer {
                        if /*IsContextParameter(node) &&*/ *distance >= outer.distance {
                            *distance -= outer.distance;
                            //return MakeRef(broker, outer.context);
                            None
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }

        fn get_module_context(
            broker: &JSHeapBroker,
            node: NodeId,
            maybe_context: Option<OuterContext>,
        ) -> OptionalContextRef {
            let mut depth = std::usize::MAX;
            let context = get_outer_context(node, &mut depth);

            let find_context = |c: HeapObjectRef| -> HeapObjectRef {
                let mut current = c;
                //while (c.map(broker).instance_type() != MODULE_CONTEXT_TYPE) {
                //  size_t depth = 1;
                //  c = c.previous(broker, &depth);
                