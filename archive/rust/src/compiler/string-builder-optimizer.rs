// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: Add equivalent Rust crates for C++ libraries like "src/base/bits.h", "src/base/logging.h", "src/base/small-vector.h", etc.
// For now, we will use standard Rust equivalents or custom implementations where necessary.

mod compiler {
    use std::cmp;
    use std::convert::TryFrom;
    use std::fmt;
    use std::num::TryFromIntError;
    use std::ops::{Add, Mul, Sub};

    // Placeholder for JSHeapBroker and other related types
    pub struct JSHeapBroker {}
    impl JSHeapBroker {
        pub fn new() -> Self {
            JSHeapBroker {}
        }
    }

    pub struct JSGraph {}
    impl JSGraph {
        pub fn new() -> Self {
            JSGraph {}
        }
        pub fn graph(&self) -> &Graph {
            &Graph::new() // Placeholder
        }
    }

    pub struct Graph {}
    impl Graph {
        pub fn new() -> Self {
            Graph {}
        }
        pub fn NodeCount(&self) -> usize {
            1024
        }
    }

    pub mod access_builder {
        // Placeholder
        pub struct AccessBuilder {}
    }

    pub mod graph_assembler {
        // Placeholder
        pub struct GraphAssembler {}
    }

    pub mod js_operator {
        // Placeholder
        pub struct JSOperator {}
    }

    pub mod node_matchers {
        // Placeholder
        pub struct HeapObjectMatcher<'a> {
            node: &'a Node,
        }
        impl<'a> HeapObjectMatcher<'a> {
            pub fn new(node: &'a Node) -> Self {
                HeapObjectMatcher { node }
            }
            pub fn HasResolvedValue(&self) -> bool {
                true
            }
            pub fn Ref(&self, _broker: &super::JSHeapBroker) -> HeapObjectRef {
                HeapObjectRef {}
            }
        }

        pub struct Float32Matcher<'a> {
            node: &'a Node,
        }
        impl<'a> Float32Matcher<'a> {
            pub fn new(node: &'a Node) -> Self {
                Float32Matcher { node }
            }
            pub fn HasResolvedValue(&self) -> bool {
                true
            }
            pub fn ResolvedValue(&self) -> f32 {
                1.0
            }
        }
        pub struct Float64Matcher<'a> {
            node: &'a Node,
        }
        impl<'a> Float64Matcher<'a> {
            pub fn new(node: &'a Node) -> Self {
                Float64Matcher { node }
            }
            pub fn HasResolvedValue(&self) -> bool {
                true
            }
            pub fn ResolvedValue(&self) -> f64 {
                1.0
            }
        }

        pub struct Int32Matcher<'a> {
            node: &'a Node,
        }
        impl<'a> Int32Matcher<'a> {
            pub fn new(node: &'a Node) -> Self {
                Int32Matcher { node }
            }
            pub fn HasResolvedValue(&self) -> bool {
                true
            }
            pub fn ResolvedValue(&self) -> i32 {
                1
            }
        }

        pub struct Int64Matcher<'a> {
            node: &'a Node,
        }
        impl<'a> Int64Matcher<'a> {
            pub fn new(node: &'a Node) -> Self {
                Int64Matcher { node }
            }
            pub fn HasResolvedValue(&self) -> bool {
                true
            }
            pub fn ResolvedValue(&self) -> i64 {
                1
            }
        }

        pub struct NumberMatcher<'a> {
            node: &'a Node,
        }
        impl<'a> NumberMatcher<'a> {
            pub fn new(node: &'a Node) -> Self {
                NumberMatcher { node }
            }
            pub fn HasResolvedValue(&self) -> bool {
                true
            }
            pub fn ResolvedValue(&self) -> f64 {
                1.0
            }
        }

        pub struct StringMatcher<'a> {
            node: &'a Node,
        }
        impl<'a> StringMatcher<'a> {
            pub fn new(node: &'a Node) -> Self {
                StringMatcher { node }
            }
            pub fn HasResolvedValue(&self) -> bool {
                true
            }
        }
    }

    pub mod node_properties {
        use super::Node;
        pub fn GetValueInput(node: &Node, i: i32) -> &Node {
            node
        }
    }

    pub mod turbofan_types {
        // Placeholder
        pub struct Type {}
    }

    pub mod objects {
        // Placeholder
        pub struct Code {}
        impl Code {
            pub fn is_builtin(&self) -> bool {
                true
            }
            pub fn builtin_id(&self) -> Builtin {
                Builtin::kMathRandom // Placeholder
            }
        }
        pub struct Map {}
    }

    pub mod utils {
        // Placeholder
        pub struct Utils {}
    }

    pub mod zone {
        // Placeholder
        pub struct Zone {}
        impl Zone {
            pub fn new() -> Self {
                Zone {}
            }
        }
    }

    pub mod schedule {
        use super::BasicBlock;
        use super::Node;

        pub struct Schedule {
            rpo_order: Vec<BasicBlock>,
            blocks: Vec<BasicBlock>,
        }

        impl Schedule {
            pub fn new(rpo_order: Vec<BasicBlock>, blocks: Vec<BasicBlock>) -> Self {
                Schedule { rpo_order, blocks }
            }

            pub fn rpo_order(&self) -> &Vec<BasicBlock> {
                &self.rpo_order
            }

            pub fn block(&self, node: &Node) -> &BasicBlock {
                &self.blocks[0] // Placeholder - in real implementation find block that includes the node
            }
        }
    }

    // Represents Builtin enum from C++
    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum Builtin {
        kMathRandom, // Placeholder
        kOtherBuiltin,
    }

    pub struct CodeRef {}
    impl CodeRef {
        pub fn object(&self) -> &objects::Code {
            &objects::Code {} // Placeholder
        }
    }
    pub struct HeapObjectRef {}
    impl HeapObjectRef {
        pub fn IsString(&self) -> bool {
            true
        }
        pub fn IsCode(&self) -> bool {
            true
        }
        pub fn AsString(&self) -> StringRef {
            StringRef {}
        }
        pub fn AsCode(&self) -> CodeRef {
            CodeRef {}
        }
    }

    pub struct StringRef {}
    impl StringRef {
        pub fn IsContentAccessible(&self) -> bool {
            true
        }
        pub fn object(&self) -> &HeapObject {
            &HeapObject {} // Placeholder
        }
    }

    pub struct HeapObject {}
    impl HeapObject {
        pub fn IsOneByteRepresentation(&self) -> bool {
            true
        }
        pub fn IsTwoByteRepresentation(&self) -> bool {
            false
        }
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum IrOpcode {
        kInvalid,
        kStringConcat,
        kNewConsString,
        kHeapConstant,
        kStringFromSingleCharCode,
        kPhi,
        kChangeTaggedToFloat64,
        kTruncateFloat64ToWord32,
        kInt32Add,
        kInt32AddWithOverflow,
        kInt64Add,
        kInt64AddWithOverflow,
        kFloat32Add,
        kFloat64Add,
        kInt32Sub,
        kInt32SubWithOverflow,
        kInt64Sub,
        kInt64SubWithOverflow,
        kFloat32Sub,
        kFloat64Sub,
        kWord32And,
        kWord64And,
        kInt32Mul,
        kInt32MulWithOverflow,
        kInt64Mul,
        kFloat32Mul,
        kFloat64Mul,
        kCall,
        kFloat32Constant,
        kFloat64Constant,
        kInt32Constant,
        kInt64Constant,
        kNumberConstant,
        kStringLength,
        kStringCharCodeAt,
        kStringCodePointAt,
        kStringIndexOf,
        kObjectIsString,
        kStringToLowerCaseIntl,
        kStringToNumber,
        kStringToUpperCaseIntl,
        kStringEqual,
        kStringLessThan,
        kStringLessThanOrEqual,
        kCheckString,
        kCheckStringOrStringWrapper,
        kTypedStateValues,
        kLast,
    }

    // Represents the Operator structure
    #[derive(Debug, Clone)]
    pub struct Operator {
        opcode: IrOpcode,
        value_input_count: i32,
        effect_output_count: i32,
        control_output_count: i32,
    }

    impl Operator {
        pub fn new(opcode: IrOpcode, value_input_count: i32, effect_output_count: i32, control_output_count: i32) -> Self {
            Operator {
                opcode,
                value_input_count,
                effect_output_count,
                control_output_count,
            }
        }
        pub fn ValueInputCount(&self) -> i32 {
            self.value_input_count
        }
        pub fn EffectOutputCount(&self) -> i32 {
            self.effect_output_count
        }
        pub fn ControlOutputCount(&self) -> i32 {
            self.control_output_count
        }

        pub fn opcode(&self) -> IrOpcode {
            self.opcode
        }
    }

    // Represents the Node structure
    #[derive(Debug, Clone)]
    pub struct Node {
        id: usize,
        opcode: IrOpcode,
        op: Operator,
        inputs: Vec<usize>,
        uses: Vec<usize>, // store node.id for uses
    }

    impl Node {
        pub fn new(id: usize, opcode: IrOpcode, op: Operator, inputs: Vec<usize>, uses: Vec<usize>) -> Self {
            Node { id, opcode, op, inputs, uses }
        }

        pub fn opcode(&self) -> IrOpcode {
            self.opcode
        }

        pub fn op(&self) -> &Operator {
            &self.op
        }

        pub fn id(&self) -> usize {
            self.id
        }

        pub fn InputAt(&self, index: usize) -> &Node {
            // Placeholder: Implement safe access to inputs.
            // Assuming inputs are valid for the purpose of this conversion.
            // In a real scenario, handle out-of-bounds access.
            // &self.inputs[index] // Original code would error
            // Using a dummy node for now to avoid errors
            let op = Operator::new(IrOpcode::kInvalid, 0, 0, 0);
            static DUMMY_NODE: Node = Node::new(0, IrOpcode::kInvalid, op, vec![], vec![]);
            &DUMMY_NODE
        }

        pub fn ReplaceInput(&mut self, index: usize, new_input: &Node) {
            // Placeholder: Replace the input at the given index with the new input.
            // Need a way to look up the node in the graph.
            // self.inputs[index] = new_input;
            println!("Replacing input at index {} with new input (id: {})", index, new_input.id());
        }

        pub fn UseCount(&self) -> usize {
            self.uses.len()
        }

        pub fn uses(&self) -> &Vec<usize> {
            &self.uses
        }
    }

    // Represents the BasicBlock structure
    #[derive(Debug, Clone)]
    pub struct BasicBlock {
        id: BlockId,
        nodes: Vec<Node>,
        predecessors: Vec<BasicBlock>,
        successors: Vec<BasicBlock>,
        loop_header: bool,
        loop_end: bool,
        dominator: Option<Box<BasicBlock>>,
        dominator_depth: i32,
        rpo_number: i32,
    }

    impl BasicBlock {
        pub fn new(id: BlockId, nodes: Vec<Node>, predecessors: Vec<BasicBlock>, successors: Vec<BasicBlock>, loop_header: bool, loop_end: bool, dominator: Option<Box<BasicBlock>>, dominator_depth: i32, rpo_number: i32) -> Self {
            BasicBlock {
                id,
                nodes,
                predecessors,
                successors,
                loop_header,
                loop_end,
                dominator,
                dominator_depth,
                rpo_number,
            }
        }

        pub fn id(&self) -> BlockId {
            self.id
        }

        pub fn nodes(&self) -> &Vec<Node> {
            &self.nodes
        }

        pub fn predecessors(&self) -> &Vec<BasicBlock> {
            &self.predecessors
        }

        pub fn successors(&self) -> &Vec<BasicBlock> {
            &self.successors
        }

        pub fn IsLoopHeader(&self) -> bool {
            self.loop_header
        }

        pub fn loop_end(&self) -> bool {
            self.loop_end
        }

        pub fn LoopContains(&self, other: &BasicBlock) -> bool {
            // Placeholder implementation - needs proper loop analysis
            true
        }

        pub fn dominator_depth(&self) -> i32 {
            self.dominator_depth
        }

        pub fn dominator(&self) -> &BasicBlock {
            match &self.dominator {
                Some(dominator) => dominator,
                None => self
            }
        }

        pub fn rpo_number(&self) -> i32 {
            self.rpo_number
        }
    }

    // Represents BlockId
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct BlockId {
        id: i32,
    }

    impl BlockId {
        pub fn new(id: i32) -> Self {
            BlockId { id }
        }

        pub fn ToInt(&self) -> usize {
            self.id as usize
        }
    }

    /// Represents the OneOrTwoByteAnalysis class.
    pub struct OneOrTwoByteAnalysis<'a> {
        graph: &'a Graph,
        temp_zone: &'a Zone,
        broker: &'a JSHeapBroker,
        states_: Vec<State>,
    }

    impl<'a> OneOrTwoByteAnalysis<'a> {
        /// Creates a new `OneOrTwoByteAnalysis`.
        pub fn new(graph: &'a Graph, temp_zone: &'a Zone, broker: &'a JSHeapBroker) -> Self {
            OneOrTwoByteAnalysis {
                graph,
                temp_zone,
                broker,
                states_: vec![State::kUnknown; graph.NodeCount()], //Initialize all nodes to Unknown state
            }
        }

        /// Represents the possible states for the analysis.
        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub enum State {
            kUnknown,
            kOneByte,
            kTwoByte,
            kCantKnow,
        }

        /// Determines the resulting state of concatenating two strings with known states.
        pub fn ConcatResultIsOneOrTwoByte(a: State, b: State) -> State {
            debug_assert!(a != State::kUnknown && b != State::kUnknown);
            if a == State::kOneByte && b == State::kOneByte {
                State::kOneByte
            } else if a == State::kTwoByte || b == State::kTwoByte {
                State::kTwoByte
            } else {
                State::kCantKnow
            }
        }

        /// Attempts to determine the range of values for a given node.
        pub fn TryGetRange(&self, node: &Node) -> Option<(i64, i64)> {
            match node.opcode() {
                IrOpcode::kChangeTaggedToFloat64 | IrOpcode::kTruncateFloat64ToWord32 => {
                    self.TryGetRange(node.InputAt(0))
                }

                IrOpcode::kInt32Add
                | IrOpcode::kInt32AddWithOverflow
                | IrOpcode::kInt64Add
                | IrOpcode::kInt64AddWithOverflow
                | IrOpcode::kFloat32Add
                | IrOpcode::kFloat64Add => {
                    let left = self.TryGetRange(node.InputAt(0));
                    let right = self.TryGetRange(node.InputAt(1));
                    if let (Some(left_range), Some(right_range)) = (left, right) {
                        // int32_t high_bound;
                        // if (base::bits::SignedAddOverflow32(static_cast<int32_t>(left->second),
                        //                                     static_cast<int32_t>(right->second),
                        //                                     &high_bound)) {
                        //   // The range would overflow a 32-bit integer.
                        //   return std::nullopt;
                        // }
                        let left_second = i32::try_from(left_range.1).ok()?;
                        let right_second = i32::try_from(right_range.1).ok()?;

                        let high_bound_result = left_second.checked_add(right_second)?;
                        Some((left_range.0 + right_range.0, i64::from(high_bound_result)))
                    } else {
                        None
                    }
                }

                IrOpcode::kInt32Sub
                | IrOpcode::kInt32SubWithOverflow
                | IrOpcode::kInt64Sub
                | IrOpcode::kInt64SubWithOverflow
                | IrOpcode::kFloat32Sub
                | IrOpcode::kFloat64Sub => {
                    let left = self.TryGetRange(node.InputAt(0));
                    let right = self.TryGetRange(node.InputAt(1));
                    if let (Some(left_range), Some(right_range)) = (left, right) {
                        if left_range.0 - right_range.1 < 0 {
                            // The range would contain negative values.
                            return None;
                        }
                        Some((left_range.0 - right_range.1, left_range.1 - right_range.0))
                    } else {
                        None
                    }
                }

                IrOpcode::kWord32And | IrOpcode::kWord64And => {
                    // Note that the minimal value for "a & b" is always 0, regardless of the
                    // max for "a" or "b". And the maximal value is the min of "max of a" and
                    // "max of b".
                    let left = self.TryGetRange(node.InputAt(0));
                    let right = self.TryGetRange(node.InputAt(1));
                    if let (Some(left_range), Some(right_range)) = (left, right) {
                        Some((0, cmp::min(left_range.1, right_range.1)))
                    } else if let Some(left_range) = left {
                        Some((0, left_range.1))
                    } else if let Some(right_range) = right {
                        Some((0, right_range.1))
                    } else {
                        None
                    }
                }

                IrOpcode::kInt32Mul
                | IrOpcode::kInt32MulWithOverflow
                | IrOpcode::kInt64Mul
                | IrOpcode::kFloat32Mul
                | IrOpcode::kFloat64Mul => {
                    let left = self.TryGetRange(node.InputAt(0));
                    let right = self.TryGetRange(node.InputAt(1));
                    if let (Some(left_range), Some(right_range)) = (left, right) {
                        // int32_t high_bound;
                        // if (base::bits::SignedMulOverflow32(static_cast<int32_t>(left->second),
                        //                                     static_cast<int32_t>(right->second),
                        //                                     &high_bound)) {
                        //   // The range would overflow a 32-bit integer.
                        //   return std::nullopt;
                        // }

                        let left_second = i32::try_from(left_range.1).ok()?;
                        let right_second = i32::try_from(right_range.1).ok()?;

                        let high_bound_result = left_second.checked_mul(right_second)?;
                        Some((left_range.0 * right_range.0, i64::from(high_bound_result)))
                    } else {
                        None
                    }
                }

                IrOpcode::kCall => {
                    use super::node_matchers::HeapObjectMatcher;
                    let m = HeapObjectMatcher::new(node.InputAt(0));
                    if m.HasResolvedValue() && m.Ref(self.broker).IsCode() {
                        let code = m.Ref(self.broker).AsCode();
                        if code.object().is_builtin() {
                            let builtin = code.object().builtin_id();
                            match builtin {
                                Builtin::kMathRandom => Some((0, 1)),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }

                IrOpcode::kFloat32Constant => {
                    use super::node_matchers::Float32Matcher;
                    let m = Float32Matcher::new(node);
                    if m.HasResolvedValue() {
                        let resolved_value = m.ResolvedValue() as f64;
                        if resolved_value < 0.0 || resolved_value >= i32::MIN as f64 {
                            return None;
                        }
                        Some((resolved_value as i64, resolved_value as i64))
                    } else {
                        None
                    }
                }
                IrOpcode::kFloat64Constant => {
                    use super::node_matchers::Float64Matcher;
                    let m = Float64Matcher::new(node);
                    if m.HasResolvedValue() {
                        let resolved_value = m.ResolvedValue();
                        if resolved_value < 0.0 || resolved_value >= i32::MIN as f64 {
                            return None;
                        }
                        Some((resolved_value as i64, resolved_value as i64))
                    } else {
                        None
                    }
                }
                IrOpcode::kInt32Constant => {
                    use super::node_matchers::Int32Matcher;
                    let m = Int32Matcher::new(node);
                    if m.HasResolvedValue() {
                        let resolved_value = m.ResolvedValue();
                        if resolved_value < 0 || resolved_value >= i32::MIN {
                            return None;
                        }
                        Some((resolved_value as i64, resolved_value as i64))
                    } else {
                        None
                    }
                }
                IrOpcode::kInt64Constant => {
                    use super::node_matchers::Int64Matcher;
                    let m = Int64Matcher::new(node);
                    if m.HasResolvedValue() {
                        let resolved_value = m.ResolvedValue();
                        if resolved_value < 0 || resolved_value >= i32::MIN as i64 {
                            return None;
                        }
                        Some((resolved_value, resolved_value))
                    } else {
                        None
                    }
                }
                IrOpcode::kNumberConstant => {
                    use super::node_matchers::NumberMatcher;
                    let m = NumberMatcher::new(node);
                    if m.HasResolvedValue() {
                        let resolved_value = m.ResolvedValue();
                        if resolved_value < 0.0 || resolved_value >= i32::MIN as f64 {
                            return None;
                        }
                        Some((resolved_value as i64, resolved_value as i64))
                    } else {
                        None
                    }
                }

                _ => None,
            }
        }

        /// Tries to determine whether a node represents a 1-byte or 2-byte string.
        pub fn OneOrTwoByte(&mut self, node: &Node) -> State {
            // TODO(v8:13785,dmercadier): once externalization can no longer convert a
            // 1-byte into a 2-byte string, compute the proper OneOrTwoByte state.
            return State::kCantKnow;
            // #if 0
            //   if (states_[node->id()] != State::kUnknown) {
            //     return states_[node->id()];
            //   }
            //   switch (node->opcode()) {
            //     case IrOpcode::kHeapConstant: {
            //       HeapObjectMatcher m(node);
            //       if (m.HasResolvedValue() && m.Ref(broker()).IsString()) {
            //         StringRef string = m.Ref(broker()).AsString();
            //         if (string.object()->IsOneByteRepresentation()) {
            //           states_[node->id()] = State::kOneByte;
            //           return State::kOneByte;
            //         } else {
            //           DCHECK(string.object()->IsTwoByteRepresentation());
            //           states_[node->id()] = State::kTwoByte;
            //           return State::kTwoByte;
            //         }
            //       } else {
            //         states_[node->id()] = State::kCantKnow;
            //         return State::kCantKnow;
            //       }
            //     }
            //
            //     case IrOpcode::kStringFromSingleCharCode: {
            //       Node* input = node->InputAt(0);
            //       switch (input->opcode()) {
            //         case IrOpcode::kStringCharCodeAt: {
            //           State state = OneOrTwoByte(input->InputAt(0));
            //           states_[node->id()] = state;
            //           return state;
            //         }
            //
            //         default: {
            //           std::optional<std::pair<int64_t, int64_t>> range =
            //               TryGetRange(input);
            //           if (!range.has_value()) {
            //             states_[node->id()] = State::kCantKnow;
            //             return State::kCantKnow;
            //           } else if (range->first >= 0 && range->second < 255) {
            //             states_[node->id()] = State::kOneByte;
            //             return State::kOneByte;
            //           } else {
            //             // For values greater than 0xFF, with the current analysis, we have
            //             // no way of knowing if the result will be on 1 or 2 bytes. For
            //             // instance, `String.fromCharCode(0x120064 & 0xffff)` will
            //             // be a 1-byte string, although the analysis will consider that its
            //             // range is [0, 0xffff].
            //             states_[node->id()] = State::kCantKnow;
            //             return State::kCantKnow;
            //           }
            //         }
            //       }
            //     }
            //
            //     case IrOpcode::kStringConcat:
            //     case IrOpcode::kNewConsString: {
            //       Node* lhs = node->InputAt(1);
            //       Node* rhs = node->InputAt(2);
            //
            //       DCHECK(IsLiteralString(rhs, broker()));
            //       State rhs_state = OneOrTwoByte(rhs);
            //
            //       // OneOrTwoByte is only called for Nodes that are part of a String
            //       // Builder. As a result, a StringConcat/NewConsString is either:
            //       //  - between 2 string literal if it is the 1st concatenation of the
            //       //    string builder.
            //       //  - between the beginning of the string builder and a literal string.
            //       // Thus, if {lhs} is not a literal string, we ignore its State: the
            //       // analysis should already have been done on its predecessors anyways.
            //       State lhs_state =
            //           IsLiteralString(lhs, broker()) ? OneOrTwoByte(lhs) : rhs_state;
            //
            //       State node_state = ConcatResultIsOneOrTwoByte(rhs_state, lhs_state);
            //       states_[node->id()] = node_state;
            //
            //       return node_state;
            //     }
            //
            //     default:
            //       states_[node->id()] = State::kCantKnow;
            //       return State::kCantKnow;
            //   }
            // #endif
        }
    }

    /// Represents the StringBuilderOptimizer class.
    pub struct StringBuilderOptimizer<'a> {
        jsgraph_: &'a JSGraph,
        schedule_: &'a schedule::Schedule,
        temp_zone_: &'a Zone,
        broker_: &'a JSHeapBroker,
        blocks_to_trimmings_map_: Vec<Option<ZoneVector<Node>>>,
        status_: Vec<Status>,
        string_builders_: ZoneVector<StringBuilder>,
        loop_headers_: ZoneVector<BasicBlock>,
        string_builder_count_: usize,
    }

    impl<'a> StringBuilderOptimizer<'a> {
        /// Creates a new `StringBuilderOptimizer`.
        pub fn new(
            jsgraph: &'a JSGraph,
            schedule: &'a schedule::Schedule,
            temp_zone: &'a Zone,
            broker: &'a JSHeapBroker,
        ) -> Self {
            let block_count = 1024; // TODO: schedule.BasicBlockCount();

            StringBuilderOptimizer {
                jsgraph_: jsgraph,
                schedule_: schedule,
                temp_zone_: temp_zone,
                broker_: broker,
                blocks_to_trimmings_map_: vec![None; block_count], // TODO schedule.BasicBlockCount()
                status_: vec![
                    Status {
                        id: kInvalidId,
                        state: State::kUnvisited,
                    };
                    jsgraph.graph().NodeCount()
                ],
                string_builders_: ZoneVector::new(),
                loop_headers_: ZoneVector::new(),
                string_builder_count_: 0,
            }
        }

        /// Checks if a block should finalize string builders.
        pub fn BlockShouldFinalizeStringBuilders(&self, block: &BasicBlock) -> bool {
            debug_assert!(block.id().ToInt() < self.blocks_to_trimmings_map_.len());
            self.blocks_to_trimmings_map_[block.id().ToInt()].is_some()
        }

        /// Retrieves the string builders to finalize for a given block.
        pub fn GetStringBuildersToFinalize(&self, block: &BasicBlock) -> ZoneVector<Node> {
            debug_assert!(self.BlockShouldFinalizeStringBuilders(block));
            self.blocks_to_trimmings_map_[block.id().ToInt()]
                .clone()
                .unwrap() // Safe due to assertion
        }

        /// Retrieves the OneOrTwoByte state for a node.
        pub fn GetOneOrTwoByte(&self, node: &Node) -> OneOrTwoByteAnalysis::State {
            debug_assert!(self.ConcatIsInStringBuilder(node));
            // TODO(v8:13785,dmercadier): once externalization can no longer convert a
            // 1-byte into a 2-byte string, return the proper OneOrTwoByte status for the
            // node (= remove the next line and uncomment the 2 after).
            OneOrTwoByteAnalysis::State::kCantKnow
            // int string_builder_number = GetStringBuilderIdForConcat(node);
            // return string_builders_[string_builder_number].one_or_two_bytes;
        }

        /// Checks if a node is the end of a string builder.
        pub fn IsStringBuilderEnd(&self, node: &Node) -> bool {
            let status = self.GetStatus(node);
            debug_assert!(
                (status.state == State::kEndStringBuilder || status.state == State::kEndStringBuilderLoopPhi)
                    == (status.id != kInvalidId && self.StringBuilderIsValid(self.string_builders_[status.id].clone()))
            );
            status.state == State::kEndStringBuilder || status.state == State::kEndStringBuilderLoopPhi
        }

        /// Checks if a node is a non-loop Phi string builder end.
        pub fn IsNonLoopPhiStringBuilderEnd(&self, node: &Node) -> bool {
            self.IsStringBuilderEnd(node) && !self.IsLoopPhi(node)
        }

        /// Checks if a node is a string builder concat input.
        pub fn IsStringBuilderConcatInput(&self, node: &Node) -> bool {
            let status = self.GetStatus(node);
            debug_assert!(
                status.state == State::kConfirmedInStringBuilder
                    == (status.id != kInvalidId && self.StringBuilderIsValid(self.string_builders_[status.id].clone()))
            );
            status.state == State::kConfirmedInStringBuilder
        }

        /// Checks if a concat node is part of a string builder.
        pub fn ConcatIsInStringBuilder(&self, node