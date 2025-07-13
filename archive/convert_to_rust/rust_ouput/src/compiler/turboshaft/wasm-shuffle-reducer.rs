// Converted from V8 C++ source files:
// Header: wasm-shuffle-reducer.h
// Implementation: wasm-shuffle-reducer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
use std::bitset::BitSet;
use std::collections::HashSet;
use std::mem;
use std::ptr;

use crate::builtins::builtins::Isolate;
use crate::compiler::turboshaft::assembler::Code;
use crate::compiler::turboshaft::operations::{
    Block, BlockIndex, Graph, OpIndex, Operation, Simd128BinopOp, Simd128BinopOp_Kind,
    Simd128ShuffleOp, Simd128UnaryOp, Simd128UnaryOp_Kind,
};
use crate::compiler::turboshaft::phase::V8_EXPORT_PRIVATE;
use crate::zone::zone::Zone;
use v8::DirectHandle;
use v8::Local;
use v8::MaybeIndirectHandle;
use v8::Value;

const kSimd128Size: usize = 16;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct TurboshaftData {}

// The aim of this reducer is to reduce the size of shuffles, by looking at
// what elements are required and we do this by looking at their users:
// - Simd128UnaryOp ConvertLow ops
// - Simd128BinaryOp ExtMulLow ops
// - Simd128ShuffleOps
// If a shuffle is only used by an operation which only reads the low half of
// shuffle input, then we can reduce the shuffle to one which shuffles fewer
// bytes. When multiple ConvertLow and/or ExtMulLow are chained, then the
// required width of the shuffle can be further reduced.
// If a shuffle is only used by a shuffle which only uses half of a shuffle
// input, that input shuffle can also reduced.
// Used by the analysis to search back from uses to their defs, looking for
// shuffles that could be reduced.
pub struct DemandedElementAnalysis<'a> {
    phase_zone_: &'a Zone,
    input_graph_: &'a Graph,
    demanded_elements_: DemandedElementMap<'a>,
    visited_: HashSet<*const Operation>,
}

impl<'a> DemandedElementAnalysis<'a> {
    pub const K8X16: u16 = 0xFFFF;
    pub const K8X8_LOW: u16 = 0xFF;
    pub const K8X4_LOW: u16 = 0xF;
    pub const K8X2_LOW: u16 = 0x3;

    pub fn new(phase_zone: &'a Zone, input_graph: &'a Graph) -> Self {
        DemandedElementAnalysis {
            phase_zone_: phase_zone,
            input_graph_: input_graph,
            demanded_elements_: DemandedElementMap::new(phase_zone),
            visited_: HashSet::new(),
        }
    }

    pub fn add_unary_op(&mut self, unop: &Simd128UnaryOp, lanes: LaneBitSet) {
        if self.visited(unop) {
            return;
        }
        self.visited_.insert(unop as *const Simd128UnaryOp as *const Operation);

        let input = self.input_graph().get(unop.input());
        if !input.saturated_use_count.is_one() {
            return;
        }

        static LOW_HALF_OPS: [Simd128UnaryOp_Kind; 6] = [
            Simd128UnaryOp_Kind::kI16x8SConvertI8x16Low,
            Simd128UnaryOp_Kind::kI16x8UConvertI8x16Low,
            Simd128UnaryOp_Kind::kI32x4SConvertI16x8Low,
            Simd128UnaryOp_Kind::kI32x4UConvertI16x8Low,
            Simd128UnaryOp_Kind::kI64x2SConvertI32x4Low,
            Simd128UnaryOp_Kind::kI64x2UConvertI32x4Low,
        ];

        for kind in &LOW_HALF_OPS {
            if *kind == unop.kind {
                debug_assert!(
                    lanes == DemandedElementAnalysis::K8X16 as u64
                        || lanes == DemandedElementAnalysis::K8X8_LOW as u64
                        || lanes == DemandedElementAnalysis::K8X4_LOW as u64
                );
                let lanes = lanes >> (lanes.count() / 2);
                self.record_op(&input, lanes);
                return;
            }
        }
    }

    pub fn add_binary_op(&mut self, binop: &Simd128BinopOp, lanes: LaneBitSet) {
        if self.visited(binop) {
            return;
        }
        self.visited_.insert(binop as *const Simd128BinopOp as *const Operation);

        static LOW_HALF_OPS: [Simd128BinopOp_Kind; 6] = [
            Simd128BinopOp_Kind::kI16x8ExtMulLowI8x16S,
            Simd128BinopOp_Kind::kI16x8ExtMulLowI8x16U,
            Simd128BinopOp_Kind::kI32x4ExtMulLowI16x8S,
            Simd128BinopOp_Kind::kI32x4ExtMulLowI16x8U,
            Simd128BinopOp_Kind::kI64x2ExtMulLowI32x4S,
            Simd128BinopOp_Kind::kI64x2ExtMulLowI32x4U,
        ];
        let left = self.input_graph().get(binop.left());
        let right = self.input_graph().get(binop.right());

        for kind in &LOW_HALF_OPS {
            if *kind == binop.kind {
                debug_assert!(
                    lanes == DemandedElementAnalysis::K8X16 as u64
                        || lanes == DemandedElementAnalysis::K8X8_LOW as u64
                );
                let lanes = lanes >> (lanes.count() / 2);
                if left.saturated_use_count.is_one() {
                    self.record_op(&left, lanes);
                }
                if right.saturated_use_count.is_one() {
                    self.record_op(&right, lanes);
                }
            }
        }
    }

    pub fn record_op(&mut self, op: &Operation, lanes: LaneBitSet) {
        if let Some(unop) = op.try_cast::<Simd128UnaryOp>() {
            self.add_unary_op(unop, lanes);
        } else if let Some(binop) = op.try_cast::<Simd128BinopOp>() {
            self.add_binary_op(binop, lanes);
        } else if let Some(shuffle) = op.try_cast::<Simd128ShuffleOp>() {
            self.demanded_elements_
                .push((shuffle as *const Simd128ShuffleOp as *const Operation, lanes));
        }
    }

    pub fn demanded_elements(&self) -> &DemandedElementMap<'a> {
        &self.demanded_elements_
    }

    pub fn input_graph(&self) -> &Graph {
        &self.input_graph_
    }

    pub fn visited(&self, op: &Operation) -> bool {
        self.visited_.contains(&(op as *const Operation))
    }
}

pub type LaneBitSet = u64;
pub type DemandedElementMap<'a> = ZoneVector<'a, (*const Operation, LaneBitSet)>;

pub struct WasmShuffleAnalyzer<'a> {
    phase_zone_: &'a Zone,
    input_graph_: &'a Graph,
    demanded_element_analysis: DemandedElementAnalysis<'a>,
    shift_shuffles_: SmallZoneVector<'a, *const Simd128ShuffleOp>,
    low_half_shuffles_: SmallZoneVector<'a, *const Simd128ShuffleOp>,
    high_half_shuffles_: SmallZoneVector<'a, *const Simd128ShuffleOp>,
}

impl<'a> WasmShuffleAnalyzer<'a> {
    pub fn new(phase_zone: &'a Zone, input_graph: &'a Graph) -> Self {
        let mut analyzer = WasmShuffleAnalyzer {
            phase_zone_: phase_zone,
            input_graph_: input_graph,
            demanded_element_analysis: DemandedElementAnalysis::new(phase_zone, input_graph),
            shift_shuffles_: SmallZoneVector::new(phase_zone),
            low_half_shuffles_: SmallZoneVector::new(phase_zone),
            high_half_shuffles_: SmallZoneVector::new(phase_zone),
        };
        analyzer.run();
        analyzer
    }

    pub fn run(&mut self) {
        for processed in (1..=self.input_graph().block_count()).rev() {
            let block_index = (processed - 1) as BlockIndex;
            let block = self.input_graph().get(block_index);
            let idx_range = self.input_graph().operation_indices(block);
            for it in idx_range.rev() {
                let op = self.input_graph().get(*it);
                self.process(op);
            }
        }
    }

    pub fn process(&mut self, op: &Operation) {
        if self.should_skip_operation(op) {
            return;
        }

        if let Some(unop) = op.try_cast::<Simd128UnaryOp>() {
            self.process_unary(unop);
            return;
        }

        if let Some(binop) = op.try_cast::<Simd128BinopOp>() {
            self.process_binary(binop);
            return;
        }

        if let Some(shuffle_op) = op.try_cast::<Simd128ShuffleOp>() {
            self.process_shuffle(shuffle_op);
            return;
        }
    }

    pub fn process_unary(&mut self, unop: &Simd128UnaryOp) {
        self.demanded_element_analysis
            .add_unary_op(unop, DemandedElementAnalysis::K8X16 as u64);
    }

    pub fn process_binary(&mut self, binop: &Simd128BinopOp) {
        self.demanded_element_analysis
            .add_binary_op(binop, DemandedElementAnalysis::K8X16 as u64);
    }

    pub fn process_shuffle_of_shuffle(
        &mut self,
        shuffle_op: &Simd128ShuffleOp,
        shuffle: &Simd128ShuffleOp,
        lower_limit: u8,
        upper_limit: u8,
    ) {
        // Suppose we have two 16-byte shuffles:
        // |---a1---|---b3---|--------|--------|  shuffle_op = (a, b)
        //
        // |---a1---|---b3---|---c?---|---c?---|  shuffle = (shf0, c)
        //
        // As only half of the shf0 is used, it means that half the work of shf0 is
        // wasted so, here, we try to reduce shf0 to a more narrow kind. In the case
        // above we can simply truncate shf0.shuffle but there are other situations
        // which involve more work:
        //
        // In the following case, shf0.shuffle needs to be shifted left so that it
        // writes the required lanes to the low half of the result. This then means
        // that shf1.shuffle needs to be updated to read from the low half.
        //
        // |--------|--------|---a1---|---b3---|  shuffle_op = (a, b)
        //
        // |---a1---|---b3---|---c?---|---c?---|  shuffle = (shf0, c)
        //

        struct ShuffleHelper<'a> {
            shuffle: &'a [u8; kSimd128Size],
        }

        impl<'a> ShuffleHelper<'a> {
            fn new(shuffle: &'a [u8; kSimd128Size]) -> Self {
                ShuffleHelper { shuffle }
            }

            fn begin(&self) -> &[u8] {
                &self.shuffle[..]
            }

            fn midpoint(&self) -> &[u8] {
                let half_lanes = kSimd128Size / 2;
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
            view.end().iter().skip(kSimd128Size / 2).all(|&i| i >= lower_limit && i <= upper_limit)
        };

        // Test whether none of the low half of the shuffle contains lanes within the
        // inclusive range.
        let none_low_half = |lower_limit: u8, upper_limit: u8| {
            view.midpoint().iter().all(|&i| !(i >= lower_limit && i <= upper_limit))
        };

        // Test whether none of the high half of the shuffle contains lanes within the
        // inclusive range.
        let none_high_half = |lower_limit: u8, upper_limit: u8| {
            view.end().iter().skip(kSimd128Size / 2).all(|&i| !(i >= lower_limit && i <= upper_limit))
        };

        // lower_ and upper_limit and set from the caller depending on whether we're
        // examining the left or right operand of shuffle. So, here we check if
        // shuffle_op is being exclusively shuffled into the low or high half using
        // either the lower and upper limits of {0,15} or {16,31}.
        let shf_into_low_half = all_low_half(lower_limit, upper_limit) && none_high_half(lower_limit, upper_limit);
        let shf_into_high_half = all_high_half(lower_limit, upper_limit) && none_low_half(lower_limit, upper_limit);
        debug_assert!(!(shf_into_low_half && shf_into_high_half));

        let quarter_lanes = kSimd128Size / 4;
        if shf_into_low_half {
            if all_low_half(lower_limit + quarter_lanes as u8, upper_limit) {
                // Low half of shuffle is sourced from the high half of shuffle_op.
                self.demanded_element_analysis.record_op(
                    shuffle_op as *const Simd128ShuffleOp as *const Operation,
                    DemandedElementAnalysis::K8X8_LOW as u64,
                );
                self.shift_shuffles_.push(shuffle_op);
                self.low_half_shuffles_.push(shuffle);
            } else if all_low_half(lower_limit, upper_limit - quarter_lanes as u8) {
                // Low half of shuffle is sourced from the low half of shuffle_op.
                self.demanded_element_analysis.record_op(
                    shuffle_op as *const Simd128ShuffleOp as *const Operation,
                    DemandedElementAnalysis::K8X8_LOW as u64,
                );
            }
        } else if shf_into_high_half {
            if all_high_half(lower_limit + quarter_lanes as u8, upper_limit) {
                // High half of shuffle is sourced from the high half of shuffle_op.
                self.demanded_element_analysis.record_op(
                    shuffle_op as *const Simd128ShuffleOp as *const Operation,
                    DemandedElementAnalysis::K8X8_LOW as u64,
                );
                self.shift_shuffles_.push(shuffle_op);
                self.high_half_shuffles_.push(shuffle);
            } else if all_high_half(lower_limit, upper_limit - quarter_lanes as u8) {
                // High half of shuffle is sourced from the low half of shuffle_op.
                self.demanded_element_analysis.record_op(
                    shuffle_op as *const Simd128ShuffleOp as *const Operation,
                    DemandedElementAnalysis::K8X8_LOW as u64,
                );
            }
        }
    }

    pub fn process_shuffle(&mut self, shuffle: &Simd128ShuffleOp) {
        if shuffle.kind != Simd128ShuffleOp::Kind::kI8x16 {
            return;
        }
        let left = self.input_graph().get(shuffle.left());
        let right = self.input_graph().get(shuffle.right());

        let shuffle_left = left.try_cast::<Simd128ShuffleOp>();
        let shuffle_right = right.try_cast::<Simd128ShuffleOp>();
        if shuffle_left.is_none() && shuffle_right.is_none() {
            return;
        }
        const LEFT_LOWER: u8 = 0;
        const LEFT_UPPER: u8 = 15;
        const RIGHT_LOWER: u8 = 16;
        const RIGHT_UPPER: u8 = 31;
        if let Some(shuffle_left) = shuffle_left {
            if shuffle_left.kind == Simd128ShuffleOp::Kind::kI8x16
                && shuffle_left.saturated_use_count.is_one()
            {
                self.process_shuffle_of_shuffle(shuffle_left, shuffle, LEFT_LOWER, LEFT_UPPER);
            }
        }
        if let Some(shuffle_right) = shuffle_right {
            if shuffle_right.kind == Simd128ShuffleOp::Kind::kI8x16
                && shuffle_right.saturated_use_count.is_one()
            {
                self.process_shuffle_of_shuffle(shuffle_right, shuffle, RIGHT_LOWER, RIGHT_UPPER);
            }
        }
    }

    pub fn should_reduce(&self) -> bool {
        !self.demanded_element_analysis.demanded_elements().is_empty()
    }

    pub fn ops_to_reduce(&self) -> &DemandedElementMap<'a> {
        self.demanded_element_analysis.demanded_elements()
    }

    pub fn demanded_byte_lanes(&self, op: *const Operation) -> Option<LaneBitSet> {
        for (narrow_op, lanes) in self.ops_to_reduce() {
            if *narrow_op == op {
                return Some(*lanes);
            }
        }
        None
    }

    // Is only the top half (lanes 8...15) of the result of shuffle required?
    // If so shuffle will need to be modified so that it writes the designed data
    // into the low half lanes instead.
    pub fn should_rewrite_shuffle_to_low(&self, shuffle: *const Simd128ShuffleOp) -> bool {
        self.shift_shuffles_.iter().any(|&shift_shuffle| shift_shuffle == shuffle)
    }

    // Is the low half (lanes 0...7) result of shuffle coming exclusively from
    // the high half of one of its operands.
    pub fn does_shuffle_into_low_half(&self, shuffle: *const Simd128ShuffleOp) -> bool {
        self.low_half_shuffles_.iter().any(|&half_shuffle| half_shuffle == shuffle)
    }

    // Is the high half (lanes: 8...15) result of shuffle coming exclusively from
    // the high half of its operands.
    pub fn does_shuffle_into_high_half(&self, shuffle: *const Simd128ShuffleOp) -> bool {
        self.high_half_shuffles_.iter().any(|&half_shuffle| half_shuffle == shuffle)
    }

    pub fn input_graph(&self) -> &Graph {
        &self.input_graph_
    }

    fn should_skip_operation(&self, _op: &Operation) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct WasmShuffleReducer<'a, Next: TurboshaftReducer> {
    analyzer_: Option<WasmShuffleAnalyzer<'a>>,
    next: Next,
    phase_zone_: &'a Zone,
    input_graph_: &'a Graph,
}

impl<'a, Next: TurboshaftReducer> WasmShuffleReducer<'a, Next> {
    pub fn new(phase_zone: &'a Zone, input_graph: &'a Graph, next: Next) -> Self {
        WasmShuffleReducer {
            analyzer_: None,
            next,
            phase_zone_: phase_zone,
            input_graph_: input_graph,
        }
    }

    fn should_skip_optimization_step(&self) -> bool {
        false
    }
}

trait TurboshaftReducer {
    fn analyze(&mut self);
    fn reduce_input_graph_simd128shuffle(
        &mut self,
        ig_index: OpIndex,
        shuffle: &Simd128ShuffleOp,
    ) -> OpIndex;
    fn map_to_new_graph(&self, op_index: OpIndex) -> OpIndex;
    fn simd128shuffle(
        &mut self,
        left: OpIndex,
        right: OpIndex,
        kind: Simd128ShuffleOp::Kind,
        shuffle_bytes: &[u8],
    ) -> OpIndex;
}

impl<'a, Next: TurboshaftReducer> TurboshaftReducer for WasmShuffleReducer<'a, Next> {
    fn analyze(&mut self) {
        self.analyzer_ = Some(WasmShuffleAnalyzer::new(self.phase_zone_, self.input_graph_));
        self.next.analyze();
    }

    fn reduce_input_graph_simd128shuffle(
        &mut self,
        ig_index: OpIndex,
        shuffle: &Simd128ShuffleOp,
    ) -> OpIndex {
        if self.should_skip_optimization_step() {
            return self.next.reduce_input_graph_simd128shuffle(ig_index, shuffle);
        }

        if shuffle.kind != Simd128ShuffleOp::Kind::kI8x16 {
            return self.next.reduce_input_graph_simd128shuffle(ig_index, shuffle);
        }

        let og_left = self.next.map_to_new_graph(shuffle.left());
        let og_right = self.next.map_to_new_graph(shuffle.right());
        let mut shuffle_bytes: [u8; kSimd128Size] = [0; kSimd128Size];
        shuffle_bytes.copy_from_slice(&shuffle.shuffle);

        let half_lanes = kSimd128Size / 2;

        let analyzer = self.analyzer_.as_ref().unwrap();

        let does_shuffle_into_low_half =
            analyzer.does_shuffle_into_low_half(shuffle as *const Simd128ShuffleOp);
        let does_shuffle_into_high_half =
            analyzer.does_shuffle_into_high_half(shuffle as *const Simd128ShuffleOp);

        // Shuffles to adjust because one, or both, of their inputs have been
        // narrowed.
        if does_shuffle_into_low_half && does_shuffle_into_high_half {
            debug_assert!(analyzer.should_rewrite_shuffle_to_low(shuffle.left()));
            debug_assert!(analyzer.should_rewrite_shuffle_to_low(shuffle.right()));
            // We have a shuffle where both inputs have been reduced and shifted, so
            // something like this:
            // |--------|--------|---a1---|---b3---|  shf0 = (a, b)
            //
            // |--------|--------|---c2---|---d4---|  shf1 = (c, d)
            //
            // |---a1---|---b3---|---c2---|---d4---|  shf2 = (shf0, shf1)
            //
            // Is being changed into this:
            // |---a1---|---b3---|--------|--------|  shf0 = (a, b)
            //
            // |---c2---|---d4---|--------|--------|  shf1 = (c, d)
            //
            // |---a1---|---b3---|---c2---|---d4---|  shf2 = (shf0, shf1)
            for lane in shuffle_bytes.iter_mut() {
                *lane = lane.wrapping_sub(half_lanes as u8);
            }
        } else if does_shuffle_into_low_half {
            debug_assert!(
                analyzer.should_rewrite_shuffle_to_low(shuffle.left())
                    || analyzer.should_rewrite_shuffle_to_low(shuffle.right())
            );
            debug_assert_ne!(
                analyzer.should_rewrite_shuffle_to_low(shuffle.left()),
                analyzer.should_rewrite_shuffle_to_low(shuffle.right())
            );
            // We have a shuffle where both inputs have been reduced and one has
            // been shifted, so something like this:
            // |--------|--------|---a1---|---b3---|  shf0 = (a, b)
            //
            // |---c2---|---d4---|--------|--------|  shf1 = (c, d)
            //
            // |---a1---|---b3---|---c2---|---d4---|  shf2 = (shf0, shf1)
            //
            // Is being changed into this:
            // |---a1---|---b3---|--------|--------|  shf0 = (a, b)
            //
            // |---c2---|---d4---|--------|--------|  shf1 = (c, d)
            //
            // |---a1---|---b3---|---c2---|---d4---|  shf2 = (shf0, shf1)
            //
            // Original shf2 lane-wise shuffle: [2, 3, 4, 5]
            // Needs to be converted to: [0, 1, 4, 5]
            for lane in shuffle_bytes.iter_mut().take(half_lanes) {
                *lane = lane.wrapping_sub(half_lanes as u8);
            }
        } else if does_shuffle_into_high_half {
            debug_assert!(
                analyzer.should_rewrite_shuffle_to_low(shuffle.left())
                    || analyzer.should_rewrite_shuffle_to_low(shuffle.right())
            );
            debug_assert_ne!(
                analyzer.should_rewrite_shuffle_to_low(shuffle.left()),
                analyzer.should_rewrite_shuffle_to_low(shuffle.right())
            );
            // We have a shuffle where both inputs have been reduced and one has
            // been shifted, so something like this:
            // |---a1---|---b3---|--------|--------|  shf0 = (a, b)
            //
            // |--------|--------|---c2---|---d4---|  shf1 = (c, d)
            //
            // |---a1---|---b3---|---c2---|---d4---|  shf2 = (shf0, shf1)
            //
            // Is being changed into this:
            // |---a1---|---b3---|--------|--------|  shf0 = (a, b)
            //
            // |---c2---|---d4---|--------|--------|  shf1 = (c, d)
            //
            // |---a1---|---b3---|---c2---|---d4---|  shf2 = (shf0, shf1)
            for lane in shuffle_bytes.iter_mut().skip(half_lanes) {
                *lane = lane.wrapping_sub(half_lanes as u8);
            }
        }

        if does_shuffle_into_low_half || does_shuffle_into_high_half {
            return self.simd128shuffle(
                og_left,
                og_right,
                Simd128ShuffleOp::Kind::kI8x16,
                &shuffle_bytes,
            );
        }

        // Shuffles to narrow.
        if let Some(lanes) = analyzer.demanded_byte_lanes(shuffle as *const Simd128ShuffleOp as *const Operation) {
            if analyzer.should_rewrite_shuffle_to_low(shuffle as *const Simd128ShuffleOp) {
                debug_assert_eq!(lanes, DemandedElementAnalysis::K8X8_LOW as u64);
                // Take the top half of the shuffle bytes and these will now write
                // those values into the low half of the result instead.
                shuffle_bytes.copy_from_slice(&shuffle.shuffle[half_lanes..]);
            } else {
                // Just truncate the lower half.
                shuffle_bytes[..half_lanes].copy_from_slice(&shuffle.shuffle[..half_lanes]);
            }

            if lanes == DemandedElementAnalysis::K8X2_LOW as u64 {
                return self.simd128shuffle(
                    og_left,
                    og_right,
                    Simd128ShuffleOp::Kind::kI8x2,
                    &shuffle_bytes,
                );
            } else if lanes == DemandedElementAnalysis::K8X4_LOW as u64 {
                return self.simd128shuffle(
                    og_left,
                    og_right,
                    Simd128ShuffleOp::Kind::kI8x4,
                    &shuffle_bytes,
                );
            } else if lanes == DemandedElementAnalysis::K8X8_LOW as u64 {
                return self.simd128shuffle(
                    og_left,
                    og_right,
                    Simd128ShuffleOp::Kind::kI8x8,
                    &shuffle_bytes,
                );
            }
        }
        self.next.reduce_input_graph_simd128shuffle(ig_index, shuffle)
    }

    fn map_to_new_graph(&self, op_index: OpIndex) -> OpIndex {
        self.next.map_to_new_graph(op_index)
    }

    fn simd128shuffle(
        &mut self,
        left: OpIndex,
        right: OpIndex,
        kind: Simd128ShuffleOp::Kind,
        shuffle_bytes: &[u8],
    ) -> OpIndex {
        self.next.simd128shuffle(left, right, kind, shuffle_bytes)
    }
}

pub struct ZoneVector<'a, T> {
    zone: &'a Zone,
    vec: Vec<T>,
}

impl<'a, T> ZoneVector<'a, T> {
    pub fn new(zone: &'a Zone) -> Self {
        ZoneVector {
            zone,
            vec: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.vec.push(value);
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.vec.iter()
    }

    pub fn clear(&mut self) {
        self.vec.clear();
    }
}

impl<'a, T> std::ops::Deref for ZoneVector<'a, T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl<'a, T> std::ops::DerefMut for ZoneVector<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}

pub struct SmallZoneVector<'a, T> {
    zone: &'a Zone,
    vec: Vec<T>,
}

impl<'a, T> SmallZoneVector<'a, T> {
    pub fn new(zone: &'a Zone) -> Self {
        SmallZoneVector {
            zone,
            vec: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.vec.push(value);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.vec.iter()
    }
}

impl<'a, T> std::ops::Deref for SmallZoneVector<'a, T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl<'a, T> std::ops::DerefMut for SmallZoneVector<'a, T> {
    fn deref_
