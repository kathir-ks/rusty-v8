// Converted from V8 C++ source files:
// Header: instruction-selection-normalization-reducer.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod instruction_selection_normalization_reducer {
    use crate::base::bits;
    use crate::compiler::turboshaft::assembler::Assembler;
    use crate::compiler::turboshaft::copying_phase::RegisterRepresentation;
    use crate::compiler::turboshaft::index::V;
    use crate::compiler::turboshaft::operations::{
        ChangeOp, ComparisonOp, ConstantOp, Opcode, Operation, TaggedBitcastOp, WordBinopOp,
    };
    use crate::compiler::turboshaft::representations::{Any, Word, Word32, WordRepresentation};
    use std::mem;

    pub trait InstructionSelectionNormalizationReducerTrait<Next> {
        fn reduce_word_binop(
            &mut self,
            left: V<Word>,
            right: V<Word>,
            kind: WordBinopOp::Kind,
            rep: WordRepresentation,
        ) -> V<Word>;
        fn reduce_comparison(
            &mut self,
            left: V<Any>,
            right: V<Any>,
            kind: ComparisonOp::Kind,
            rep: RegisterRepresentation,
        ) -> V<Word32>;
        fn get(&self, index: V<Any>) -> &Operation;
    }

    pub struct InstructionSelectionNormalizationReducer<Next> {
        next: Next,
    }

    impl<Next> InstructionSelectionNormalizationReducer<Next> {
        pub fn new(next: Next) -> Self {
            Self { next }
        }
    }

    impl<Next: InstructionSelectionNormalizationReducerTrait<Next>>
        InstructionSelectionNormalizationReducer<Next>
    {
        pub fn reduce_word_binop(
            &mut self,
            left: V<Word>,
            right: V<Word>,
            kind: WordBinopOp::Kind,
            rep: WordRepresentation,
        ) -> V<Word> {
            // Putting constant on the right side.
            if WordBinopOp::IsCommutative(kind) {
                if !self.is_simple_constant(right) && self.is_simple_constant(left) {
                    mem::swap(&mut left, &mut right);
                } else if !self.is_complex_constant(right) && self.is_complex_constant(left) {
                    mem::swap(&mut left, &mut right);
                }
            }

            // Transforming multiplications by power of two constants into shifts
            if kind == WordBinopOp::Kind::kMul {
                if let Some((cst, _)) = self.match_power_of_two_word_constant(right, rep) {
                    if cst < rep.bit_width() as i64 {
                        return self.shift_left(left, bits::which_power_of_two(cst as u64) as i32, rep);
                    }
                }
            }

            self.next.reduce_word_binop(left, right, kind, rep)
        }

        pub fn reduce_comparison(
            &mut self,
            left: V<Any>,
            right: V<Any>,
            kind: ComparisonOp::Kind,
            rep: RegisterRepresentation,
        ) -> V<Word32> {
            if ComparisonOp::IsCommutative(kind) {
                if !self.is_simple_constant(right) && self.is_simple_constant(left) {
                    mem::swap(&mut left, &mut right);
                } else if !self.is_complex_constant(right) && self.is_complex_constant(left) {
                    mem::swap(&mut left, &mut right);
                }
            }
            self.next.reduce_comparison(left, right, kind, rep)
        }

        // Return true if {index} is a literal ConstantOp.
        fn is_simple_constant(&self, index: V<Any>) -> bool {
            matches!(self.next.get(index).opcode, Opcode::kConstant)
        }
        // Return true if {index} is a ConstantOp or a (chain of) Change/Cast/Bitcast
        // of a ConstantOp. Such an operation is succeptible to be recognized as a
        // constant by the instruction selector, and as such should rather be on the
        // right-hande side of commutative binops.
        fn is_complex_constant(&self, index: V<Any>) -> bool {
            let op = self.next.get(index);
            match op.opcode {
                Opcode::kConstant => true,
                Opcode::kChange => {
                    let change_op = op.cast::<ChangeOp>();
                    self.is_complex_constant(change_op.input())
                }
                Opcode::kTaggedBitcast => {
                    let bitcast_op = op.cast::<TaggedBitcastOp>();
                    self.is_complex_constant(bitcast_op.input())
                }
                Opcode::kTryChange => {
                    let change_op = op.cast::<ChangeOp>();
                    self.is_complex_constant(change_op.input())
                }
                _ => false,
            }
        }

        fn match_power_of_two_word_constant(
            &self,
            right: V<Word>,
            rep: WordRepresentation,
        ) -> Option<(i64, WordRepresentation)> {
            let op = self.next.get(right);
            if let Opcode::kConstant = op.opcode {
                // Assuming constant op holds an i64 value
                let constant_op: &ConstantOp = op.cast();
                if let Some(cst) = constant_op.value().number() {
                    if cst > 0 && (cst & (cst - 1.0) == 0.0) {
                        return Some((cst as i64, rep));
                    }
                }
            }
            None
        }

        fn shift_left(&self, left: V<Word>, shift: i32, rep: WordRepresentation) -> V<Word> {
            // Assuming we can create a ShiftLeft operation
            V::default()
        }
    }

    trait CastableOperation {
        fn cast<T>(&self) -> &T;
    }

    impl Operation {
        fn cast<T>(&self) -> &T {
            unsafe { &*(self as *const Self as *const T) }
        }
    }
}
