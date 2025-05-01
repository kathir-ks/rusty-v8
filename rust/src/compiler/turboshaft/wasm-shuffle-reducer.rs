// Copyright 2025 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/compiler/turboshaft/wasm-shuffle-reducer.h (equivalent module declaration)
pub mod wasm_shuffle_reducer {
    use std::collections::HashSet;
    use std::ops::{BitAnd, BitOr, BitOrAssign, Not, Shl, Shr};
    use std::convert::TryFrom;
    use std::iter::IntoIterator;
    use std::vec;
    
    // Placeholder type for Operation, InputGraph, Block, etc.  Replace with actual definitions.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct OperationId(u32);
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BlockIndex(u32);

    pub struct Operation {
        id: OperationId,
        pub saturated_use_count: SaturatedUseCount,
        kind: OperationKind,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum OperationKind {
        Simd128UnaryOp(Simd128UnaryOpKind),
        Simd128BinopOp(Simd128BinopOpKind),
        Simd128ShuffleOp(Simd128ShuffleOpKind),
        Other,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Simd128UnaryOpKind {
        kI16x8SConvertI8x16Low,
        kI16x8UConvertI8x16Low,
        kI32x4SConvertI16x8Low,
        kI32x4UConvertI16x8Low,
        kI64x2SConvertI32x4Low,
        kI64x2UConvertI32x4Low,
        Other,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Simd128BinopOpKind {
        kI16x8ExtMulLowI8x16S,
        kI16x8ExtMulLowI8x16U,
        kI32x4ExtMulLowI16x8S,
        kI32x4ExtMulLowI16x8U,
        kI64x2ExtMulLowI32x4S,
        kI64x2ExtMulLowI32x4U,
        Other,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Simd128ShuffleOpKind {
        kI8x16,
        Other,
    }

    impl Operation {
        pub fn try_cast<T: TryFrom<&Operation>>(&self) -> Option<T> {
            T::try_from(self).ok()
        }
    }

    pub struct Simd128UnaryOp {
        input: OperationId,
        kind: Simd128UnaryOpKind,
    }

    impl TryFrom<&Operation> for Simd128UnaryOp {
        type Error = ();

        fn try_from(op: &Operation) -> Result<Self, Self::Error> {
            match op.kind {
                OperationKind::Simd128UnaryOp(kind) => {
                    // Assuming a way to retrieve the input OperationId
                    Ok(Simd128UnaryOp { input: OperationId(0), kind })
                }
                _ => Err(()),
            }
        }
    }

    pub struct Simd128BinopOp {
        left: OperationId,
        right: OperationId,
        kind: Simd128BinopOpKind,
    }

    impl TryFrom<&Operation> for Simd128BinopOp {
        type Error = ();

        fn try_from(op: &Operation) -> Result<Self, Self::Error> {
            match op.kind {
                OperationKind::Simd128BinopOp(kind) => {
                    // Assuming a way to retrieve the left and right OperationIds
                    Ok(Simd128BinopOp {
                        left: OperationId(0),
                        right: OperationId(0),
                        kind,
                    })
                }
                _ => Err(()),
            }
        }
    }

    pub struct Simd128ShuffleOp {
        left: OperationId,
        right: OperationId,
        kind: Simd128ShuffleOpKind,
        pub shuffle: [u8; k_simd128_size],
        pub saturated_use_count: SaturatedUseCount,

    }

    impl TryFrom<&Operation> for Simd128ShuffleOp {
        type Error = ();

        fn try_from(op: &Operation) -> Result<Self, Self::Error> {
            match op.kind {
                OperationKind::Simd128ShuffleOp(kind) => {
                    // Assuming a way to retrieve the left and right OperationIds and shuffle data
                    Ok(Simd128ShuffleOp {
                        left: OperationId(0),
                        right: OperationId(0),
                        kind,
                        shuffle: [0u8; k_simd128_size],
                        saturated_use_count: SaturatedUseCount::One,
                    })
                }
                _ => Err(()),
            }
        }
    }

    pub struct InputGraph {
        operations: Vec<Operation>,
        blocks: Vec<Block>,
        operation_indices: Vec<(usize, usize)>, // (start_index, end_index) for each block
    }

    impl InputGraph {
        pub fn get(&self, id: OperationId) -> &Operation {
            &self.operations[id.0 as usize]
        }

        pub fn get_block(&self, index: BlockIndex) -> &Block {
            &self.blocks[index.0 as usize]
        }

        pub fn block_count(&self) -> u32 {
            self.blocks.len() as u32
        }

        pub fn operation_indices(&self, block: &Block) -> std::ops::Range<usize> {
            let (start, end) = self.operation_indices[block.index.0 as usize];
            start..end
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Block {
        index: BlockIndex,
    }
    
    // Placeholder for SaturatedUseCount
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SaturatedUseCount {
        Zero,
        One,
        Many,
    }

    impl SaturatedUseCount {
        pub fn is_one(&self) -> bool {
            *self == SaturatedUseCount::One
        }
    }

    // Represents which lanes are demanded.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct LaneBitSet(u32);

    impl LaneBitSet {
        pub fn count(&self) -> u32 {
            self.0.count_ones()
        }
    }

    impl Shr<u32> for LaneBitSet {
        type Output = Self;

        fn shr(self, rhs: u32) -> Self::Output {
            LaneBitSet(self.0 >> rhs)
        }
    }

    // Constants for LaneBitSet.
    pub const K8X16: LaneBitSet = LaneBitSet(0xFFFF); // Assuming 16 lanes
    pub const K8X8_LOW: LaneBitSet = LaneBitSet(0x00FF); // Assuming first 8 lanes
    pub const K8X4_LOW: LaneBitSet = LaneBitSet(0x000F); // Assuming first 4 lanes

    pub const K_SIMD128_SIZE: usize = 16;

    /// Analyzes demanded elements and records shuffle operations.
    pub struct DemandedElementAnalysis<'a> {
        input_graph: &'a InputGraph,
        visited_: HashSet<*const Operation>,
        demanded_elements_: Vec<(&'a Simd128ShuffleOp, LaneBitSet)>,
    }

    impl<'a> DemandedElementAnalysis<'a> {
        pub const K8X16: LaneBitSet = LaneBitSet(0xFFFF); // Assuming 16 lanes
        pub const K8X8_LOW: LaneBitSet = LaneBitSet(0x00FF); // Assuming first 8 lanes
        pub const K8X4_LOW: LaneBitSet = LaneBitSet(0x000F); // Assuming first 4 lanes

        /// Creates a new `DemandedElementAnalysis`.
        pub fn new(input_graph: &'a InputGraph) -> Self {
            DemandedElementAnalysis {
                input_graph,
                visited_: HashSet::new(),
                demanded_elements_: Vec::new(),
            }
        }

        fn input_graph(&self) -> &InputGraph {
            self.input_graph
        }

        fn visited(&self, op: *const Operation) -> bool {
            self.visited_.contains(&op)
        }

        fn add_unary_op(&mut self, unop: &'a Simd128UnaryOp, lanes: LaneBitSet) {
            if self.visited(unop as *const Simd128UnaryOp as *const Operation) {
                return;
            }
            self.visited_.insert(unop as *const Simd128UnaryOp as *const Operation);

            let input = self.input_graph().get(unop.input);
            if !input.saturated_use_count.is_one() {
                return;
            }

            const LOW_HALF_OPS: [Simd128UnaryOpKind; 6] = [
                Simd128UnaryOpKind::kI16x8SConvertI8x16Low,
                Simd128UnaryOpKind::kI16x8UConvertI8x16Low,
                Simd128UnaryOpKind::kI32x4SConvertI16x8Low,
                Simd128UnaryOpKind::kI32x4UConvertI16x8Low,
                Simd128UnaryOpKind::kI64x2SConvertI32x4Low,
                Simd128UnaryOpKind::kI64x2UConvertI32x4Low,
            ];

            for &kind in &LOW_HALF_OPS {
                if kind == unop.kind {
                   // debug_assert!(lanes == K8X16 || lanes == K8X8_LOW || lanes == K8X4_LOW); // Using debug_assert! instead of CHECK
                    let new_lanes = lanes >> (lanes.count() / 2);
                    self.record_op(input, new_lanes);
                    return;
                }
            }
        }

        fn add_binary_op(&mut self, binop: &'a Simd128BinopOp, lanes: LaneBitSet) {
            if self.visited(binop as *const Simd128BinopOp as *const Operation) {
                return;
            }
            self.visited_.insert(binop as *const Simd128BinopOp as *const Operation);

            const LOW_HALF_OPS: [Simd128BinopOpKind; 6] = [
                Simd128BinopOpKind::kI16x8ExtMulLowI8x16S,
                Simd128BinopOpKind::kI16x8ExtMulLowI8x16U,
                Simd128BinopOpKind::kI32x4ExtMulLowI16x8S,
                Simd128BinopOpKind::kI32x4ExtMulLowI16x8U,
                Simd128BinopOpKind::kI64x2ExtMulLowI32x4S,
                Simd128BinopOpKind::kI64x2ExtMulLowI32x4U,
            ];

            let left = self.input_graph().get(binop.left);
            let right = self.input_graph().get(binop.right);

            for &kind in &LOW_HALF_OPS {
                if kind == binop.kind {
                    //debug_assert!(lanes == K8X16 || lanes == K8X8_LOW); // Using debug_assert! instead of CHECK
                    let new_lanes = lanes >> (lanes.count() / 2);
                    if left.saturated_use_count.is_one() {
                        self.record_op(left, new_lanes);
                    }
                    if right.saturated_use_count.is_one() {
                        self.record_op(right, new_lanes);
                    }
                }
            }
        }

        fn record_op(&mut self, op: &Operation, lanes: LaneBitSet) {
            if let Some(unop) = op.try_cast::<&Simd128UnaryOp>() {
                self.add_unary_op(unop, lanes);
            } else if let Some(binop) = op.try_cast::<&Simd128BinopOp>() {
                self.add_binary_op(binop, lanes);
            } else if let Some(shuffle) = op.try_cast::<&Simd128ShuffleOp>() {
                self.demanded_elements_.push((shuffle, lanes));
            }
        }
    }

    /// Analyzes wasm shuffles for potential reductions.
    pub struct WasmShuffleAnalyzer<'a> {
        input_graph: &'a InputGraph,
        demanded_element_analysis: DemandedElementAnalysis<'a>,
        shift_shuffles_: Vec<&'a Operation>,
        low_half_shuffles_: Vec<&'a Simd128ShuffleOp>,
        high_half_shuffles_: Vec<&'a Simd128ShuffleOp>,
    }

    impl<'a> WasmShuffleAnalyzer<'a> {
        /// Creates a new `WasmShuffleAnalyzer`.
        pub fn new(input_graph: &'a InputGraph) -> Self {
            WasmShuffleAnalyzer {
                input_graph,
                demanded_element_analysis: DemandedElementAnalysis::new(input_graph),
                shift_shuffles_: Vec::new(),
                low_half_shuffles_: Vec::new(),
                high_half_shuffles_: Vec::new(),
            }
        }

        fn input_graph(&self) -> &InputGraph {
            self.input_graph
        }

        /// Runs the shuffle analysis.
        pub fn run(&mut self) {
            let block_count = self.input_graph().block_count();
            for processed in (0..block_count).rev() {
                let block_index = BlockIndex(processed);
                let block = self.input_graph().get_block(block_index);
                let idx_range = self.input_graph().operation_indices(block);
                for it in idx_range.rev() {
                    let op = self.input_graph().get(OperationId(it as u32));
                    self.process(op);
                }
            }
        }

        fn process(&mut self, op: &Operation) {
            if self.should_skip_operation(op) {
                return;
            }

            if let Some(unop) = op.try_cast::<&Simd128UnaryOp>() {
                self.process_unary(unop);
                return;
            }

            if let Some(binop) = op.try_cast::<&Simd128BinopOp>() {
                self.process_binary(binop);
                return;
            }

            if let Some(shuffle_op) = op.try_cast::<&Simd128ShuffleOp>() {
                self.process_shuffle(shuffle_op);
                return;
            }
        }

        fn process_unary(&mut self, unop: &Simd128UnaryOp) {
            self.demanded_element_analysis.add_unary_op(unop, DemandedElementAnalysis::K8X16);
        }

        fn process_binary(&mut self, binop: &Simd128BinopOp) {
            self.demanded_element_analysis.add_binary_op(binop, DemandedElementAnalysis::K8X16);
        }

        fn process_shuffle_of_shuffle(
            &mut self,
            shuffle_op: &Simd128ShuffleOp,
            shuffle: &Simd128ShuffleOp,
            lower_limit: u8,
            upper_limit: u8,
        ) {
            struct ShuffleHelper<'a> {
                shuffle: &'a [u8; k_simd128_size],
            }

            impl<'a> ShuffleHelper<'a> {
                fn new(shuffle: &'a [u8; k_simd128_size]) -> Self {
                    ShuffleHelper { shuffle }
                }

                fn begin(&self) -> &[u8] {
                    &self.shuffle[..]
                }

                fn midpoint(&self) -> &[u8] {
                    let half_lanes = k_simd128_size / 2;
                    &self.shuffle[..half_lanes]
                }

                fn end(&self) -> &[u8] {
                    &self.shuffle[..]
                }
            }

            let view = ShuffleHelper::new(&shuffle.shuffle);

            // Test whether the low half of the shuffle is within the inclusive range.
            let all_low_half = |lower_limit: u8, upper_limit: u8| {
                view.midpoint().iter().all(|&i| i >= lower_limit && i <= upper_limit)
            };

            // Test whether the high half of the shuffle is within the inclusive range.
            let all_high_half = |lower_limit: u8, upper_limit: u8| {
                view.end().iter().skip(k_simd128_size / 2).all(|&i| i >= lower_limit && i <= upper_limit)
            };

            // Test whether none of the low half of the shuffle contains lanes within the
            // inclusive range.
            let none_low_half = |lower_limit: u8, upper_limit: u8| {
                view.midpoint().iter().all(|&i| !(i >= lower_limit && i <= upper_limit))
            };

            // Test whether none of the high half of the shuffle contains lanes within the
            // inclusive range.
            let none_high_half = |lower_limit: u8, upper_limit: u8| {
                view.end().iter().skip(k_simd128_size / 2).all(|&i| !(i >= lower_limit && i <= upper_limit))
            };

            let shf_into_low_half = all_low_half(lower_limit, upper_limit) && none_high_half(lower_limit, upper_limit);
            let shf_into_high_half = all_high_half(lower_limit, upper_limit) && none_low_half(lower_limit, upper_limit);
            //debug_assert!(!(shf_into_low_half && shf_into_high_half)); // Using debug_assert! instead of CHECK

            let quarter_lanes = k_simd128_size / 4;
            if shf_into_low_half {
                if all_low_half(lower_limit + quarter_lanes as u8, upper_limit) {
                    // Low half of shuffle is sourced from the high half of shuffle_op.
                    self.demanded_element_analysis.record_op(shuffle_op, DemandedElementAnalysis::K8X8_LOW);
                    self.shift_shuffles_.push(shuffle_op as *const Simd128ShuffleOp as *const Operation);
                    self.low_half_shuffles_.push(shuffle);
                } else if all_low_half(lower_limit, upper_limit - quarter_lanes as u8) {
                    // Low half of shuffle is sourced from the low half of shuffle_op.
                    self.demanded_element_analysis.record_op(shuffle_op, DemandedElementAnalysis::K8X8_LOW);
                }
            } else if shf_into_high_half {
                if all_high_half(lower_limit + quarter_lanes as u8, upper_limit) {
                    // High half of shuffle is sourced from the high half of shuffle_op.
                    self.demanded_element_analysis.record_op(shuffle_op, DemandedElementAnalysis::K8X8_LOW);
                    self.shift_shuffles_.push(shuffle_op as *const Simd128ShuffleOp as *const Operation);
                    self.high_half_shuffles_.push(shuffle);
                } else if all_high_half(lower_limit, upper_limit - quarter_lanes as u8) {
                    // High half of shuffle is sourced from the low half of shuffle_op.
                    self.demanded_element_analysis.record_op(shuffle_op, DemandedElementAnalysis::K8X8_LOW);
                }
            }
        }

        fn process_shuffle(&mut self, shuffle: &Simd128ShuffleOp) {
            if shuffle.kind != Simd128ShuffleOpKind::kI8x16 {
                return;
            }

            let left = self.input_graph().get(shuffle.left);
            let right = self.input_graph().get(shuffle.right);

            let shuffle_left = left.try_cast::<&Simd128ShuffleOp>();
            let shuffle_right = right.try_cast::<&Simd128ShuffleOp>();

            if shuffle_left.is_none() && shuffle_right.is_none() {
                return;
            }

            const LEFT_LOWER: u8 = 0;
            const LEFT_UPPER: u8 = 15;
            const RIGHT_LOWER: u8 = 16;
            const RIGHT_UPPER: u8 = 31;

            if let Some(shuffle_left) = shuffle_left {
                if shuffle_left.kind == Simd128ShuffleOpKind::kI8x16 && shuffle_left.saturated_use_count.is_one() {
                    self.process_shuffle_of_shuffle(shuffle_left, shuffle, LEFT_LOWER, LEFT_UPPER);
                }
            }

            if let Some(shuffle_right) = shuffle_right {
                if shuffle_right.kind == Simd128ShuffleOpKind::kI8x16 && shuffle_right.saturated_use_count.is_one() {
                    self.process_shuffle_of_shuffle(shuffle_right, shuffle, RIGHT_LOWER, RIGHT_UPPER);
                }
            }
        }

        fn should_skip_operation(&self, _op: &Operation) -> bool {
            // Placeholder for the skip logic.
            false
        }
    }
}