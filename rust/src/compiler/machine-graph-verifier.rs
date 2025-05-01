// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/compiler/machine-graph-verifier.h (Rust module definition - incomplete)
mod machine_graph_verifier {
    // Placeholder - Replace with actual module content if header file is needed
    pub fn run() {
        // Placeholder implementation
        println!("MachineGraphVerifier::run() called.");
    }
}

// src/compiler/machine-graph-verifier.cc (Converted to Rust)
mod compiler {
    use std::collections::HashMap;
    use std::fmt;
    use std::rc::Rc;
    //use std::vec::Vec;

    //use crate::base::Flags; // Assuming base::Flags is ported as well

    // Placeholder types - replace with actual Rust equivalents
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum MachineRepresentation {
        kNone,
        kBit,
        kWord8,
        kWord16,
        kWord32,
        kWord64,
        kFloat32,
        kFloat64,
        kSimd128,
        kSimd256,
        kTagged,
        kTaggedSigned,
        kTaggedPointer,
        kCompressed,
        kCompressedPointer,
        kMapWord,
        kProtectedPointer,
        kIndirectPointer,
        kSandboxedPointer,
        kFloat16RawBits,
        kFloat16,
    }

    impl fmt::Display for MachineRepresentation {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum IrOpcode {
        kParameter,
        kReturn,
        kProjection,
        kTypedStateValues,
        kWord32AtomicLoad,
        kWord64AtomicLoad,
        kLoad,
        kLoadImmutable,
        kProtectedLoad,
        kLoadTrapOnNull,
        kLoadFramePointer,
        kLoadParentFramePointer,
        kStackSlot,
        kLoadRootRegister,
        kLoadStackPointer,
        kUnalignedLoad,
        kPhi,
        kCall,
        kTailCall,
        kWord32AtomicStore,
        kWord64AtomicStore,
        kWord32AtomicPairLoad,
        kWord32AtomicPairStore,
        kWord32AtomicPairAdd,
        kWord32AtomicPairSub,
        kWord32AtomicPairAnd,
        kWord32AtomicPairOr,
        kWord32AtomicPairXor,
        kWord32AtomicPairExchange,
        kWord32AtomicPairCompareExchange,
        kWord32AtomicExchange,
        kWord32AtomicCompareExchange,
        kWord32AtomicAdd,
        kWord32AtomicSub,
        kWord32AtomicAnd,
        kWord32AtomicOr,
        kWord32AtomicXor,
        kWord64AtomicExchange,
        kWord64AtomicCompareExchange,
        kWord64AtomicAdd,
        kWord64AtomicSub,
        kWord64AtomicAnd,
        kWord64AtomicOr,
        kWord64AtomicXor,
        kStore,
        kProtectedStore,
        kStoreTrapOnNull,
        kStoreIndirectPointer,
        kUnalignedStore,
        kHeapConstant,
        kNumberConstant,
        kChangeBitToTagged,
        kIfException,
        kOsrValue,
        kChangeInt32ToTagged,
        kChangeUint32ToTagged,
        kBitcastWordToTagged,
        kTaggedIndexConstant,
        kCompressedHeapConstant,
        kExternalConstant,
        kBitcastTaggedToWord,
        kBitcastTaggedToWordForTagAndSmiBits,
        kBitcastWordToTaggedSigned,
        kWord32Equal,
        kInt32LessThan,
        kInt32LessThanOrEqual,
        kUint32LessThan,
        kUint32LessThanOrEqual,
        kWord64Equal,
        kInt64LessThan,
        kInt64LessThanOrEqual,
        kUint64LessThan,
        kUint64LessThanOrEqual,
        kFloat32Equal,
        kFloat32LessThan,
        kFloat32LessThanOrEqual,
        kFloat64Equal,
        kFloat64LessThan,
        kFloat64LessThanOrEqual,
        kChangeTaggedToBit,
        kStackPointerGreaterThan,
        kTruncateInt64ToInt32,
        kTruncateFloat32ToInt32,
        kTruncateFloat32ToUint32,
        kBitcastFloat32ToInt32,
        kI32x4ExtractLane,
        kI16x8ExtractLaneU,
        kI16x8ExtractLaneS,
        kI8x16ExtractLaneU,
        kI8x16ExtractLaneS,
        kI8x16BitMask,
        kInt32Constant,
        kRelocatableInt32Constant,
        kTruncateFloat64ToWord32,
        kTruncateFloat64ToUint32,
        kChangeFloat64ToInt32,
        kChangeFloat64ToUint32,
        kRoundFloat64ToInt32,
        kFloat64ExtractLowWord32,
        kFloat64ExtractHighWord32,
        kWord32Popcnt,
        kChangeInt32ToInt64,
        kChangeUint32ToUint64,
        kBitcastWord32ToWord64,
        kInt64Constant,
        kRelocatableInt64Constant,
        kBitcastFloat64ToInt64,
        kChangeFloat64ToInt64,
        kChangeFloat64ToUint64,
        kTruncateFloat64ToInt64,
        kWord64Popcnt,
        kWord64Ctz,
        kWord64Clz,
        kRoundInt32ToFloat32,
        kRoundUint32ToFloat32,
        kRoundInt64ToFloat32,
        kRoundUint64ToFloat32,
        kBitcastInt32ToFloat32,
        kFloat32Constant,
        kTruncateFloat64ToFloat32,
        kRoundInt64ToFloat64,
        kRoundUint64ToFloat64,
        kBitcastInt64ToFloat64,
        kChangeFloat32ToFloat64,
        kChangeInt32ToFloat64,
        kChangeUint32ToFloat64,
        kFloat64InsertLowWord32,
        kFloat64InsertHighWord32,
        kFloat64Constant,
        kFloat64SilenceNaN,
        kI32x4ReplaceLane,
        kI32x4Splat,
        kI8x16Splat,
        kI8x16Eq,
        kAbortCSADcheck,
        kBranch,
        kSwitch,
        kSetStackPointer,
        kThrow,
        kFrameState,
        kStaticAssert,
        kTryTruncateFloat64ToInt64,
        kTryTruncateFloat64ToInt32,
        kTryTruncateFloat64ToUint32,
        kInt32PairAdd,
        kInt32PairSub,
        kStorePair
    }

    impl fmt::Display for IrOpcode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MachineType {
        representation: MachineRepresentation,
    }

    impl MachineType {
        pub fn pointer_representation() -> Self {
            MachineType {
                representation: if cfg!(target_pointer_width = "32") {
                    MachineRepresentation::kWord32
                } else {
                    MachineRepresentation::kWord64
                },
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Node {
        id: usize,
        opcode: IrOpcode,
        inputs: Vec<usize>, // Node IDs
        op: Box<dyn AnyOp>, // Store operator specific data
    }

    trait AnyOp: std::any::Any + fmt::Debug {}

    #[derive(Debug, Clone)]
    struct CommonOperator {
        value: i32, // Replace with correct data
    }
    impl AnyOp for CommonOperator {}

    #[derive(Debug, Clone)]
    struct MachineOperator {
        value: i32, // Replace with correct data
    }
    impl AnyOp for MachineOperator {}
    #[derive(Debug, Clone)]
    struct StoreRepresentation {
        representation: MachineRepresentation,
    }
    impl AnyOp for StoreRepresentation {}
    #[derive(Debug, Clone)]
    struct LoadRepresentation {
        representation: MachineRepresentation,
    }
    impl AnyOp for LoadRepresentation {}
    #[derive(Debug, Clone)]
    struct StorePairRepresentation {
        first: MachineType,
        second: MachineType,
    }
    impl AnyOp for StorePairRepresentation {}

    #[derive(Debug, Clone)]
    struct CallDescriptor {
        return_count: usize,
        input_types: Vec<MachineType>,
        return_types: Vec<MachineType>,
    }
    impl AnyOp for CallDescriptor {}

    impl CallDescriptor {
        pub fn input_count(&self) -> usize {
            self.input_types.len()
        }
        pub fn return_count(&self) -> usize {
            self.return_types.len()
        }
        pub fn get_input_type(&self, index: usize) -> MachineType {
            self.input_types[index]
        }
        pub fn get_return_type(&self, index: usize) -> MachineType {
            self.return_types[index]
        }
    }

    #[derive(Debug, Clone)]
    struct AtomicLoadParameters {
        representation: MachineType
    }
    impl AnyOp for AtomicLoadParameters {}
    #[derive(Debug, Clone)]
    struct AtomicStoreParameters {
        representation: MachineType
    }
    impl AnyOp for AtomicStoreParameters {}

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct BasicBlock {
        id: usize,
        nodes: Vec<usize>, // Node IDs
        control_input: Option<usize>,
    }

    impl BasicBlock {
        pub fn node_count(&self) -> usize {
            self.nodes.len()
        }

        pub fn node_at(&self, index: usize) -> Option<&usize> {
             self.nodes.get(index)
        }
        pub fn control_input(&self) -> Option<usize> {
            self.control_input
        }
    }

    impl fmt::Display for BasicBlock {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "BasicBlock #{}", self.id)
        }
    }

    #[derive(Debug, Clone)]
    pub struct Schedule {
        all_blocks: Vec<BasicBlock>,
    }

    impl Schedule {
        pub fn all_blocks(&self) -> &Vec<BasicBlock> {
            &self.all_blocks
        }
    }

    #[derive(Debug, Clone)]
    pub struct TFGraph {
        nodes: Vec<Node>,
    }

    impl TFGraph {
        pub fn node_count(&self) -> usize {
            self.nodes.len()
        }

        pub fn get_node(&self, id: usize) -> Option<&Node> {
             self.nodes.get(id)
        }
    }

    #[derive(Debug, Clone)]
    pub struct Linkage {
        incoming_descriptor: CallDescriptor,
        parameter_types: Vec<MachineType>,
        return_type: MachineType,
    }

    impl Linkage {
        pub fn get_incoming_descriptor(&self) -> &CallDescriptor {
            &self.incoming_descriptor
        }
        pub fn get_parameter_type(&self, index: usize) -> MachineType {
            self.parameter_types[index]
        }
        pub fn get_return_type(&self) -> MachineType {
            self.return_type
        }
    }

    #[derive(Debug, Clone)]
    pub struct NodeProperties {} // Placeholder

    impl NodeProperties {
        pub fn get_projection_type(_node: &Node) -> MachineRepresentation {
            // Replace with actual logic
            MachineRepresentation::kNone
        }
    }

    pub fn parameter_index_of(_op: &dyn AnyOp) -> usize {
        // Replace with actual logic
        0
    }

    pub fn load_representation_of(op: &dyn AnyOp) -> &LoadRepresentation {
        op.downcast_ref::<LoadRepresentation>().unwrap()
    }
    pub fn store_representation_of(op: &dyn AnyOp) -> &StoreRepresentation {
        op.downcast_ref::<StoreRepresentation>().unwrap()
    }
    pub fn store_pair_representation_of(op: &dyn AnyOp) -> &StorePairRepresentation {
        op.downcast_ref::<StorePairRepresentation>().unwrap()
    }
    pub fn unaligned_store_representation_of(_op: &dyn AnyOp) -> MachineRepresentation {
        // Replace with actual logic
        MachineRepresentation::kNone
    }

    pub fn phi_representation_of(_op: &dyn AnyOp) -> MachineRepresentation {
        // Replace with actual logic
        MachineRepresentation::kNone
    }

    pub fn call_descriptor_of(op: &dyn AnyOp) -> &CallDescriptor {
        op.downcast_ref::<CallDescriptor>().unwrap()
    }
    pub fn atomic_load_parameters_of(op: &dyn AnyOp) -> &AtomicLoadParameters {
        op.downcast_ref::<AtomicLoadParameters>().unwrap()
    }
    pub fn atomic_store_parameters_of(op: &dyn AnyOp) -> &AtomicStoreParameters {
        op.downcast_ref::<AtomicStoreParameters>().unwrap()
    }
    pub fn atomic_op_type(_op: &dyn AnyOp) -> MachineType {
        // Replace with actual logic
        MachineType { representation: MachineRepresentation::kNone }
    }
    pub fn element_size_log2_of(_rep: MachineType) -> i32 {
        // Replace with actual logic
        3
    }

    pub fn is_any_tagged(rep: MachineRepresentation) -> bool {
         match rep {
            MachineRepresentation::kTagged | MachineRepresentation::kTaggedPointer | MachineRepresentation::kTaggedSigned => true,
            _ => false,
        }
    }
    pub fn is_any_compressed(rep: MachineRepresentation) -> bool {
        match rep {
            MachineRepresentation::kCompressed | MachineRepresentation::kCompressedPointer => true,
            _ => false,
        }
    }

    const COMPRESS_POINTERS_BOOL: bool = false;
    const DEBUG_BOOL: bool = false;
    const DECOMPRESS_POINTER_BY_ADDRESSING_MODE: bool = false; // Placeholder
    // Placeholder for macro-based lists. Replace with actual lists or logic.
    macro_rules! machine_unop_32_list {
        ($callback:ident) => {
            // Example: $callback!(Add32);
        };
    }
    macro_rules! machine_binop_32_list {
        ($callback:ident) => {
            // Example: $callback!(And32);
        };
    }
    macro_rules! machine_binop_64_list {
        ($callback:ident) => {

        };
    }
    macro_rules! machine_float32_binop_list {
        ($callback:ident) => {

        };
    }
    macro_rules! machine_float32_unop_list {
        ($callback:ident) => {

        };
    }
    macro_rules! machine_float64_binop_list {
        ($callback:ident) => {

        };
    }
    macro_rules! machine_float64_unop_list {
        ($callback:ident) => {

        };
    }

    struct MachineRepresentationInferrer {
        schedule: Rc<Schedule>,
        linkage: Rc<Linkage>,
        representation_vector: HashMap<usize, MachineRepresentation>,
    }

    impl MachineRepresentationInferrer {
        fn new(schedule: Rc<Schedule>, graph: &TFGraph, linkage: Rc<Linkage>) -> Self {
            let representation_vector = HashMap::new();
            let mut inferrer = MachineRepresentationInferrer {
                schedule: schedule.clone(),
                linkage: linkage.clone(),
                representation_vector,
            };
            inferrer.run(graph);
            inferrer
        }

        fn call_descriptor(&self) -> &CallDescriptor {
            self.linkage.get_incoming_descriptor()
        }

        fn get_representation(&self, node_id: usize) -> MachineRepresentation {
            *self.representation_vector.get(&node_id).unwrap_or(&MachineRepresentation::kNone)
        }

        fn promote_representation(&self, rep: MachineRepresentation) -> MachineRepresentation {
            match rep {
                MachineRepresentation::kWord8
                | MachineRepresentation::kWord16
                | MachineRepresentation::kWord32 => MachineRepresentation::kWord32,
                MachineRepresentation::kSandboxedPointer => MachineRepresentation::kWord64,
                _ => rep,
            }
        }

        fn run(&mut self, graph: &TFGraph) {
            for block in self.schedule.all_blocks().iter() {
                for i in 0..=block.node_count() {
                    let node_id = if i < block.node_count() {
                        block.node_at(i).copied()
                    } else {
                        block.control_input()
                    };

                    let Some(node_id) = node_id else {
                        continue;
                    };

                    let Some(node) = graph.get_node(node_id) else {
                        continue;
                    };

                    match node.opcode {
                        IrOpcode::kParameter => {
                            self.representation_vector.insert(
                                node_id,
                                self.linkage
                                    .get_parameter_type(parameter_index_of(&*node.op))
                                    .representation,
                            );
                        }
                        IrOpcode::kReturn => {
                            self.representation_vector.insert(
                                node_id,
                                self.promote_representation(
                                    self.linkage.get_return_type().representation,
                                ),
                            );
                        }
                        IrOpcode::kProjection => {
                            self.representation_vector.insert(
                                node_id,
                                NodeProperties::get_projection_type(node),
                            );
                        }
                        IrOpcode::kTypedStateValues => {
                            self.representation_vector.insert(node_id, MachineRepresentation::kNone);
                        }
                        IrOpcode::kWord32AtomicLoad | IrOpcode::kWord64AtomicLoad => {
                            self.representation_vector.insert(
                                node_id,
                                self.promote_representation(
                                    atomic_load_parameters_of(&*node.op)
                                        .representation
                                        .representation,
                                ),
                            );
                        }
                        IrOpcode::kLoad
                        | IrOpcode::kLoadImmutable
                        | IrOpcode::kProtectedLoad
                        | IrOpcode::kLoadTrapOnNull => {
                            self.representation_vector.insert(
                                node_id,
                                self.promote_representation(
                                    load_representation_of(&*node.op).representation,
                                ),
                            );
                        }
                        IrOpcode::kLoadFramePointer
                        | IrOpcode::kLoadParentFramePointer
                        | IrOpcode::kStackSlot
                        | IrOpcode::kLoadRootRegister
                        | IrOpcode::kLoadStackPointer => {
                            self.representation_vector.insert(
                                node_id,
                                MachineType::pointer_representation().representation,
                            );
                        }
                        IrOpcode::kUnalignedLoad => {
                            self.representation_vector.insert(
                                node_id,
                                self.promote_representation(
                                    load_representation_of(&*node.op).representation,
                                ),
                            );
                        }
                        IrOpcode::kPhi => {
                            self.representation_vector
                                .insert(node_id, phi_representation_of(&*node.op));
                        }
                        IrOpcode::kCall => {
                            let call_descriptor = call_descriptor_of(&*node.op);
                            if call_descriptor.return_count() > 0 {
                                self.representation_vector.insert(
                                    node_id,
                                    call_descriptor.get_return_type(0).representation,
                                );
                            } else {
                                self.representation_vector
                                    .insert(node_id, MachineRepresentation::kTagged);
                            }
                        }
                        IrOpcode::kWord32AtomicStore | IrOpcode::kWord64AtomicStore => {
                            self.representation_vector.insert(
                                node_id,
                                self.promote_representation(
                                    atomic_store_parameters_of(&*node.op).representation,
                                ),
                            );
                        }
                        IrOpcode::kWord32AtomicPairLoad
                        | IrOpcode::kWord32AtomicPairStore
                        | IrOpcode::kWord32AtomicPairAdd
                        | IrOpcode::kWord32AtomicPairSub
                        | IrOpcode::kWord32AtomicPairAnd
                        | IrOpcode::kWord32AtomicPairOr
                        | IrOpcode::kWord32AtomicPairXor
                        | IrOpcode::kWord32AtomicPairExchange
                        | IrOpcode::kWord32AtomicPairCompareExchange => {
                            self.representation_vector
                                .insert(node_id, MachineRepresentation::kWord32);
                        }
                        IrOpcode::kWord32AtomicExchange
                        | IrOpcode::kWord32AtomicCompareExchange
                        | IrOpcode::kWord32AtomicAdd
                        | IrOpcode::kWord32AtomicSub
                        | IrOpcode::kWord32AtomicAnd
                        | IrOpcode::kWord32AtomicOr
                        | IrOpcode::kWord32AtomicXor
                        | IrOpcode::kWord64AtomicExchange
                        | IrOpcode::kWord64AtomicCompareExchange
                        | IrOpcode::kWord64AtomicAdd
                        | IrOpcode::kWord64AtomicSub
                        | IrOpcode::kWord64AtomicAnd
                        | IrOpcode::kWord64AtomicOr
                        | IrOpcode::kWord64AtomicXor => {
                            self.representation_vector.insert(
                                node_id,
                                self.promote_representation(atomic_op_type(&*node.op).representation),
                            );
                        }
                        IrOpcode::kStore
                        | IrOpcode::kProtectedStore
                        | IrOpcode::kStoreTrapOnNull
                        | IrOpcode::kStoreIndirectPointer => {
                            self.representation_vector.insert(
                                node_id,
                                self.promote_representation(
                                    store_representation_of(&*node.op).representation,
                                ),
                            );
                        }
                        IrOpcode::kUnalignedStore => {
                            self.representation_vector.insert(
                                node_id,
                                self.promote_representation(
                                    unaligned_store_representation_of(&*node.op),
                                ),
                            );
                        }
                        IrOpcode::kHeapConstant => {
                            self.representation_vector
                                .insert(node_id, MachineRepresentation::kTaggedPointer);
                        }
                        IrOpcode::kNumberConstant
                        | IrOpcode::kChangeBitToTagged
                        | IrOpcode::kIfException
                        | IrOpcode::kOsrValue
                        | IrOpcode::kChangeInt32ToTagged
                        | IrOpcode::kChangeUint32ToTagged
                        | IrOpcode::kBitcastWordToTagged
                        | IrOpcode::kTaggedIndexConstant => {
                            self.representation_vector
                                .insert(node_id, MachineRepresentation::kTagged);
                        }
                        IrOpcode::kCompressedHeapConstant => {
                            self.representation_vector
                                .insert(node_id, MachineRepresentation::kCompressedPointer);
                        }
                        IrOpcode::kExternalConstant => {
                            self.representation_vector.insert(
                                node_id,
                                MachineType::pointer_representation().representation,
                            );
                        }
                        IrOpcode::kBitcastTaggedToWord
                        | IrOpcode::kBitcastTaggedToWordForTagAndSmiBits => {
                            self.representation_vector.insert(
                                node_id,
                                MachineType::pointer_representation().representation,
                            );
                        }
                        IrOpcode::kBitcastWordToTaggedSigned => {
                            self.representation_vector
                                .insert(node_id, MachineRepresentation::kTaggedSigned);
                        }
                        IrOpcode::kWord32Equal
                        | IrOpcode::kInt32LessThan
                        | IrOpcode::kInt32LessThanOrEqual
                        | IrOpcode::kUint32LessThan
                        | IrOpcode::kUint32LessThanOrEqual
                        | IrOpcode::kWord64Equal
                        | IrOpcode::kInt64LessThan
                        | IrOpcode::kInt64LessThanOrEqual
                        | IrOpcode::kUint64LessThan
                        | IrOpcode::kUint64LessThanOrEqual
                        | IrOpcode::kFloat32Equal
                        | IrOpcode::kFloat32LessThan
                        | IrOpcode::kFloat32LessThanOrEqual
                        | IrOpcode::kFloat64Equal
                        | IrOpcode::kFloat64LessThan
                        | IrOpcode::kFloat64LessThanOrEqual
                        | IrOpcode::kChangeTaggedToBit
                        | IrOpcode::kStackPointerGreaterThan => {
                            self.representation_vector.insert(node_id, MachineRepresentation::kBit);
                        }
                        IrOpcode::kTruncateInt64ToInt32
                        | IrOpcode::kTruncateFloat32ToInt32
                        | IrOpcode::kTruncateFloat32ToUint32
                        | IrOpcode::kBitcastFloat32ToInt32
                        | IrOpcode::kI32x4ExtractLane
                        | IrOpcode::kI16x8ExtractLaneU
                        | IrOpcode::kI16x8ExtractLaneS
                        | IrOpcode::kI8x16ExtractLaneU
                        | IrOpcode::kI8x16ExtractLaneS
                        | IrOpcode::kI8x16BitMask
                        | IrOpcode::kInt32Constant
                        | IrOpcode::kRelocatableInt32Constant
                        | IrOpcode::kTruncateFloat64ToWord32
                        | IrOpcode::kTruncateFloat64ToUint32
                        | IrOpcode::kChangeFloat64ToInt32
                        | IrOpcode::kChangeFloat64ToUint32
                        | IrOpcode::kRoundFloat64ToInt32
                        | IrOpcode::kFloat64ExtractLowWord32
                        | IrOpcode::kFloat64ExtractHighWord32
                        | IrOpcode::kWord32Popcnt => {
                            self.representation_vector
                                .insert(node_id, MachineRepresentation::kWord32);
                        }
                        IrOpcode::kChangeInt32ToInt64
                        | IrOpcode::kChangeUint32ToUint64
                        | IrOpcode::kBitcastWord32ToWord64
                        | IrOpcode::kInt64Constant
                        | IrOpcode::kRelocatableInt64Constant
                        | IrOpcode::kBitcastFloat64ToInt64
                        | IrOpcode::kChangeFloat64ToInt64
                        | IrOpcode::kChangeFloat64ToUint64
                        | IrOpcode::kTruncateFloat64ToInt64
                        | IrOpcode::kWord64Popcnt
                        | IrOpcode::kWord64Ctz
                        | IrOpcode::kWord64Clz => {
                            self.representation_vector
                                .insert(node_id, MachineRepresentation::kWord64);
                        }
                        IrOpcode::kRoundInt32ToFloat32
                        | IrOpcode::kRoundUint32ToFloat32
                        | IrOpcode::kRoundInt64ToFloat32
                        | IrOpcode::kRoundUint64ToFloat32
                        | IrOpcode::kBitcastInt32ToFloat32
                        | IrOpcode::kFloat32Constant
                        | IrOpcode::kTruncateFloat64ToFloat32 => {
                            self.representation_vector
                                .insert(node_id, MachineRepresentation::kFloat32);
                        }
                        IrOpcode::kRoundInt64ToFloat64
                        | IrOpcode::kRoundUint64ToFloat64
                        | IrOpcode::kBitcastInt64ToFloat64
                        | IrOpcode::kChangeFloat32ToFloat64
                        | IrOpcode::kChangeInt32ToFloat64
                        | IrOpcode::kChangeUint32ToFloat64
                        | IrOpcode::kFloat64InsertLowWord32
                        | IrOpcode::kFloat64InsertHighWord32
                        | IrOpcode::kFloat64Constant
                        | IrOpcode::kFloat64SilenceNaN => {
                            self.representation_vector
                                .insert(node_id, MachineRepresentation::kFloat64);
                        }
                        IrOpcode::kI32x4ReplaceLane
                        | IrOpcode::kI32x4Splat
                        | IrOpcode::kI8x16Splat
                        | IrOpcode::kI8x16Eq => {
                            self.representation_vector
                                .insert(node_id, MachineRepresentation::kSimd128);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    struct MachineRepresentationChecker {
        schedule: Rc<Schedule>,
        inferrer: Rc<MachineRepresentationInferrer>,
        is_stub: bool,
        name: String,
        current_block: Option<Rc<BasicBlock>>,
    }

    impl MachineRepresentationChecker {
        fn new(
            schedule: Rc<Schedule>,
            inferrer: Rc<MachineRepresentationInferrer>,
            is_stub: bool,
            name: &str,
        ) -> Self {
            MachineRepresentationChecker {
                schedule,
                inferrer,
                is_stub,
                name: name.to_string(),
                current_block: None,
            }
        }

        fn run(&mut self, graph: &TFGraph) {
            for block in self.schedule.all_blocks().iter() {
                self.current_block = Some(Rc::new(block.clone()));
                for i in 0..=block.node_count() {
                    let node_id = if i < block.node_count() {
                         block.node_at(i).copied()
                    } else {
                        block.control_input()
                    };

                    let Some(node_id) = node_id else {
                        continue;
                    };

                    let Some(node) = graph.get_node(node_id) else {
                        continue;
                    };

                    match node.opcode {
                        IrOpcode::kCall | IrOpcode::kTailCall => {
                            self.check_call_inputs(node, graph);
                        }
                        IrOpcode::kChangeBitToTagged => {
                            self.check_value_input_representation_is(
                                node,
                                0,
                                MachineRepresentation::kBit,
                                graph,
                            );
                        }
                        IrOpcode::kChangeTaggedToBit => {
                            self.check_value_input_representation_is(
                                node,
                                0,
                                MachineRepresentation::kTagged,
                                graph,
                            );
                        }
                        IrOpcode::kRoundInt64ToFloat64
                        | IrOpcode::kRoundUint64ToFloat64
                        | IrOpcode::kRoundInt64ToFloat32
                        | IrOpcode::kRoundUint64ToFloat32
                        | IrOpcode::kTruncateInt64ToInt32
                        | IrOpcode::kBitcastInt64ToFloat64
                        | IrOpcode::kWord64Ctz
                        | IrOpcode::kWord64Clz
                        | IrOpcode::kWord64Popcnt => {
                            self.check_value_input_for_int64_op(node, 0, graph);
                        }
                        IrOpcode::kBitcastWordToTagged
                        | IrOpcode::kBitcastWordToTaggedSigned => {
                            self.check_value_input_representation_is(
                                node,
                                0,
                                MachineType::pointer_representation().representation,
                                graph,
                            );
                        }
                        IrOpcode::kBitcastTaggedToWord
                        | IrOpcode::kBitcastTaggedToWordForTagAndSmiBits => {
                            if COMPRESS_POINTERS_BOOL {
                                self.check_value_input_is_compressed_or_tagged(node, 0, graph);
                            } else {
                                self.check_value_input_is_tagged(node, 0, graph);
                            }
                        }
                        IrOpcode::kTruncateFloat64ToWord32
                        | IrOpcode::kTruncateFloat64ToUint32
                        | IrOpcode::kTruncateFloat64ToFloat32
                        | IrOpcode::kChangeFloat64ToInt32
                        | IrOpcode::kChangeFloat64ToUint32
                        | IrOpcode::kRoundFloat64ToInt32
                        | IrOpcode::kFloat64ExtractLowWord32
                        | IrOpcode::kFloat64ExtractHighWord32
                        | IrOpcode::kBitcastFloat64ToInt6