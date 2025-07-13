// Converted from V8 C++ source files:
// Header: decompression-optimization.h
// Implementation: decompression-optimization.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod decompression_optimization {
    use std::any::Any;
    use std::cmp;
    use std::collections::HashSet;
    use std::ops::Deref;

    use crate::codegen::machine_type::MachineRepresentation;
    use crate::compiler::turboshaft::copying_phase::CopyingPhase;
    use crate::compiler::turboshaft::operations::{
        Block, BlockIndex, ChangeOp, ComparisonOp, ConstantOp, LoadOp, OpIndex, Operation, Opcode,
        PhiOp, ShiftOp, StoreOp, TaggedBitcastOp, WordBinopOp,
    };
    use crate::compiler::turboshaft::representations::{
        RegisterRepresentation, Representation, WordRepresentation,
    };
    use crate::execution::isolate::Isolate;
    use crate::objects::heap_object::HeapObject;
    use crate::objects::tagged::Tagged;
    use crate::utils::bit_vector::BitVector;
    use crate::utils::utils::FixedOpIndexSidetable;
    use crate::v8::internal::Zone;
    use crate::v8::internal::compiler::turboshaft::Graph;
    use crate::v8::internal::compiler::turboshaft::V;

    pub fn any_of<T>(a: T, b: T) -> T {
        a
    }

    const DECOMPRESS_POINTER_BY_ADDRESSING_MODE: bool = true;

    struct DecompressionAnalyzer<'a> {
        graph: &'a Graph,
        phase_zone: *mut Zone,
        needs_decompression: FixedOpIndexSidetable<u8>,
        candidates: Vec<OpIndex>,
    }

    impl<'a> DecompressionAnalyzer<'a> {
        fn new(graph: &'a Graph, phase_zone: *mut Zone) -> Self {
            let op_id_count = graph.op_id_count();
            let needs_decompression = FixedOpIndexSidetable::new(op_id_count, phase_zone, graph);
            let candidates = Vec::with_capacity(op_id_count / 8);
            Self {
                graph,
                phase_zone,
                needs_decompression,
                candidates,
            }
        }

        fn run(&mut self) {
            let mut next_block_id = self.graph.block_count() as i32 - 1;
            while next_block_id >= 0 {
                let block_index = BlockIndex(next_block_id as usize);
                next_block_id -= 1;
                let block = self.graph.get(block_index);
                if block.is_loop() {
                    self.process_block::<true>(block, &mut next_block_id);
                } else {
                    self.process_block::<false>(block, &mut next_block_id);
                }
            }
        }

        fn needs_decompression(&self, op: OpIndex) -> bool {
            self.needs_decompression.get(op) != 0
        }

        fn needs_decompression_op(&self, op: &Operation) -> bool {
            self.needs_decompression(self.graph.index_operation(op))
        }

        fn mark_as_needs_decompression(&mut self, op: OpIndex) -> bool {
            self.needs_decompression.set(op, 1);
            true
        }

        fn process_block<const IS_LOOP: bool>(
            &mut self,
            block: &Block,
            next_block_id: &mut i32,
        ) {
            for op in self.graph.operations_reversed(block) {
                if IS_LOOP && op.is::<PhiOp>() && self.needs_decompression_op(op) {
                    let phi = op.cast::<PhiOp>();
                    if !self.needs_decompression(phi.input(1)) {
                        let backedge = block.last_predecessor();
                        *next_block_id =
                            cmp::max(*next_block_id, backedge.index().0 as i32);
                    }
                }
                self.process_operation(op);
            }
        }

        fn process_operation(&mut self, op: &Operation) {
            match op.opcode {
                Opcode::kStore => {
                    let store = op.cast::<StoreOp>();
                    self.mark_as_needs_decompression(store.base());
                    if store.index().valid() {
                        self.mark_as_needs_decompression(store.index().value());
                    }
                    if !store.stored_rep.is_compressible_tagged() {
                        self.mark_as_needs_decompression(store.value());
                    }
                }
                Opcode::kFrameState => {
                    // The deopt code knows how to handle compressed inputs.
                }
                Opcode::kPhi => {
                    // Replicate the phi's state for its inputs.
                    let phi = op.cast::<PhiOp>();
                    if self.needs_decompression_op(op) {
                        for input in phi.inputs() {
                            self.mark_as_needs_decompression(input);
                        }
                    } else {
                        self.candidates.push(self.graph.index_operation(op));
                    }
                }
                Opcode::kComparison => {
                    let comp = op.cast::<ComparisonOp>();
                    if comp.rep == WordRepresentation::Word64() {
                        self.mark_as_needs_decompression(comp.left());
                        self.mark_as_needs_decompression(comp.right());
                    }
                }
                Opcode::kWordBinop => {
                    let binary_op = op.cast::<WordBinopOp>();
                    if binary_op.rep == WordRepresentation::Word64() {
                        self.mark_as_needs_decompression(binary_op.left());
                        self.mark_as_needs_decompression(binary_op.right());
                    }
                }
                Opcode::kShift => {
                    let shift_op = op.cast::<ShiftOp>();
                    if shift_op.rep == WordRepresentation::Word64() {
                        self.mark_as_needs_decompression(shift_op.left());
                    }
                }
                Opcode::kChange => {
                    let change = op.cast::<ChangeOp>();
                    if change.to == WordRepresentation::Word64() && self.needs_decompression_op(op)
                    {
                        self.mark_as_needs_decompression(change.input());
                    }
                }
                Opcode::kTaggedBitcast => {
                    let bitcast = op.cast::<TaggedBitcastOp>();
                    if bitcast.kind != TaggedBitcastOp::Kind::kSmi && self.needs_decompression_op(op)
                    {
                        self.mark_as_needs_decompression(bitcast.input());
                    } else {
                        self.candidates.push(self.graph.index_operation(op));
                    }
                }
                Opcode::kConstant => {
                    if !self.needs_decompression_op(op) {
                        self.candidates.push(self.graph.index_operation(op));
                    }
                }
                Opcode::kLoad => {
                    if !self.needs_decompression_op(op) {
                        self.candidates.push(self.graph.index_operation(op));
                    }
                    let load = op.cast::<LoadOp>();
                    if DECOMPRESS_POINTER_BY_ADDRESSING_MODE && !load.index().valid()
                        && self.graph.get(load.base()).saturated_use_count.is_one()
                    {
                        // On x64, if the Index is invalid, we can rely on complex addressing
                        // mode to decompress the base, and can thus keep it compressed.
                        // We only do this if the use-count of the base is 1, in order to avoid
                        // having to decompress multiple time the same value.
                        self.mark_addressing_base(load.base());
                    } else {
                        self.mark_as_needs_decompression(load.base());
                        if load.index().valid() {
                            self.mark_as_needs_decompression(load.index().value());
                        }
                    }
                }
                _ => {
                    for input in op.inputs() {
                        self.mark_as_needs_decompression(input);
                    }
                }
            }
        }

        fn mark_addressing_base(&mut self, base_idx: OpIndex) {
            assert!(DECOMPRESS_POINTER_BY_ADDRESSING_MODE);
            let base = self.graph.get(base_idx);
            if let Some(load) = base.try_cast::<LoadOp>() {
                if load.loaded_rep.is_compressible_tagged() {
                    // We can keep {load} (the base) as compressed and untag with complex
                    // addressing mode.
                    return;
                }
            }
            if base.is::<PhiOp>() {
                let mut keep_compressed = true;
                for input_idx in base.inputs() {
                    let input = self.graph.get(input_idx);
                    if !input.is::<LoadOp>()
                        || !base.is_only_user_of(input, self.graph)
                        || !input.cast::<LoadOp>().loaded_rep.is_compressible_tagged()
                    {
                        keep_compressed = false;
                        break;
                    }
                }
                if keep_compressed {
                    return;
                }
            }
            self.mark_as_needs_decompression(base_idx);
        }
    }

    pub fn run_decompression_optimization(graph: &mut Graph, phase_zone: *mut Zone) {
        let mut analyzer = DecompressionAnalyzer::new(graph, phase_zone);
        analyzer.run();
        for op_idx in analyzer.candidates.iter() {
            let mut op = graph.get_mut(*op_idx);
            if analyzer.needs_decompression(*op_idx) {
                continue;
            }
            match op.opcode {
                Opcode::kConstant => {
                    let constant = op.cast_mut::<ConstantOp>();
                    if constant.kind == ConstantOp::Kind::kHeapObject {
                        constant.kind = ConstantOp::Kind::kCompressedHeapObject;
                    }
                }
                Opcode::kPhi => {
                    let phi = op.cast_mut::<PhiOp>();
                    if phi.rep == RegisterRepresentation::Tagged() {
                        phi.rep = RegisterRepresentation::Compressed();
                    }
                }
                Opcode::kLoad => {
                    let load = op.cast_mut::<LoadOp>();
                    if load.loaded_rep.is_compressible_tagged() {
                        assert_eq!(
                            load.result_rep,
                            any_of(
                                RegisterRepresentation::Tagged(),
                                RegisterRepresentation::Compressed()
                            )
                        );
                        load.result_rep = RegisterRepresentation::Compressed();
                    }
                }
                Opcode::kTaggedBitcast => {
                    let bitcast = op.cast_mut::<TaggedBitcastOp>();
                    if bitcast.from == RegisterRepresentation::Tagged()
                        && (bitcast.to == RegisterRepresentation::WordPtr()
                            || bitcast.kind == TaggedBitcastOp::Kind::kSmi)
                    {
                        bitcast.from = RegisterRepresentation::Compressed();
                        bitcast.to = RegisterRepresentation::Word32();
                    }
                }
                _ => {}
            }
        }
    }
}
