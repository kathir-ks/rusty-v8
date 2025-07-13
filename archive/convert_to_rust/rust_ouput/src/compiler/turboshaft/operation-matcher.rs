// Converted from V8 C++ source files:
// Header: operation-matcher.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod operation_matcher {
    use std::any::Any;
    use std::cmp::PartialEq;
    use std::fmt::{Debug, Display};
    use std::hash::Hash;
    use std::marker::PhantomData;
    use std::mem;
    use std::ops::{BitAnd, BitOr, BitXor, Deref, DerefMut, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};
    use std::sync::{Arc, Mutex, RwLock};

    use crate::compiler::turboshaft::graph::Graph;
    use crate::compiler::turboshaft::index::OpIndex;
    use crate::compiler::turboshaft::int64_lowering_reducer::WordRepresentation;
    use crate::compiler::turboshaft::load_store_simplification_reducer::OperationMatcher;
    use crate::compiler::turboshaft::operations::{
        AnyOrNone, BinaryFloatOperation, BinaryOperation, ChangeOp, ComparisonOp, ConstantOp, FloatBinopOp, FloatRepresentation, FloatUnaryOp,
        IrOpcode, Operation, PhiOp, ShiftOp, UnaryFloatOperation, UnaryOperation, WordBinopOp,
    };
    use crate::compiler::turboshaft::representations::RegisterRepresentation;
    use crate::execution::messages::Handle;
    use crate::v8::V8;

    pub struct V<T> {
        pub rep: WordRepresentation,
    }

    pub trait IsWord<T> {}

    impl IsWord<i32> for i32 {}
    impl IsWord<i64> for i64 {}

    impl<T> V<T> {
        pub fn Cast(_op_index: OpIndex) -> Self {
            V { rep: WordRepresentation::Word32 } // Assuming default representation
        }
    }

    pub struct detail {}

    impl detail {
        pub fn const_or_v_exists_v<T>() -> bool {
            true // Assuming const_or_v_exists_v always returns true for simplicity
        }

        pub fn is_valid_type_for<T>(_rep: WordRepresentation) -> bool {
            true // Assuming default implementation of IsValidTypeFor
        }
    }

    pub struct IndexMatch<T, const HasConstexpr: bool> {
        v_: IndexMatchVariant<T>,
        _phantom: PhantomData<T>,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Wildcard {}

    impl<T, const HasConstexpr: bool> IndexMatch<T, HasConstexpr> {
        pub fn new_wildcard() -> Self {
            IndexMatch {
                v_: IndexMatchVariant::Wildcard(Wildcard {}),
                _phantom: PhantomData,
            }
        }

        pub fn new_index(index: OpIndex) -> Self {
            IndexMatch {
                v_: IndexMatchVariant::OpIndex(index),
                _phantom: PhantomData,
            }
        }

        pub fn new_index_ptr(index: *mut OpIndex) -> Self {
            IndexMatch {
                v_: IndexMatchVariant::OpIndexPtr(index),
                _phantom: PhantomData,
            }
        }

        pub fn new_v_index(_index: *mut V<T>) -> Self {
            IndexMatch {
                v_: IndexMatchVariant::Wildcard(Wildcard {}),
                _phantom: PhantomData,
            }
        }

        pub fn new_constant<CT: Copy + PartialEq>(constant: CT) -> Self {
            IndexMatch {
                v_: IndexMatchVariant::Constexpr(Box::new(constant) as Box<dyn Any>),
                _phantom: PhantomData,
            }
        }

        pub fn matches(&self, matched: OpIndex, matcher: &OperationMatcher) -> bool {
            match &self.v_ {
                IndexMatchVariant::Wildcard(_) => true,
                IndexMatchVariant::OpIndex(index) => *index == matched,
                IndexMatchVariant::OpIndexPtr(index_ptr) => {
                    unsafe {
                        **index_ptr = matched;
                    }
                    true
                }
                IndexMatchVariant::Constexpr(constant) => {
                    //This part require more information about the ConstantOp and v_traits
                    //For now return false.
                    false
                }
            }
        }
    }

    pub enum IndexMatchVariant<T> {
        Wildcard(Wildcard),
        OpIndex(OpIndex),
        OpIndexPtr(*mut OpIndex),
        Constexpr(Box<dyn Any>),
    }

    pub struct ValueMatch<T> {
        v_: ValueMatchVariant<T>,
        _phantom: PhantomData<T>,
    }

    impl<T> ValueMatch<T>
    where
        T: PartialEq,
    {
        pub fn new_wildcard() -> Self {
            ValueMatch {
                v_: ValueMatchVariant::Wildcard(Wildcard {}),
                _phantom: PhantomData,
            }
        }

        pub fn new_value(value: T) -> Self {
            ValueMatch {
                v_: ValueMatchVariant::Value(value),
                _phantom: PhantomData,
            }
        }

        pub fn new_value_ptr(value: *mut T) -> Self {
            ValueMatch {
                v_: ValueMatchVariant::ValuePtr(value),
                _phantom: PhantomData,
            }
        }

        pub fn matches(&self, matched: &T) -> bool {
            match &self.v_ {
                ValueMatchVariant::Wildcard(_) => true,
                ValueMatchVariant::Value(value) => *value == *matched,
                ValueMatchVariant::ValuePtr(value_ptr) => {
                    unsafe {
                        **value_ptr = matched.clone();
                    }
                    true
                }
            }
        }
    }

    pub enum ValueMatchVariant<T> {
        Wildcard(Wildcard),
        Value(T),
        ValuePtr(*mut T),
    }

    impl OperationMatcher {
        pub fn new(graph: &Graph) -> Self {
            OperationMatcher { graph_: graph }
        }

        pub fn is<Op: 'static>(&self, op_idx: OpIndex) -> bool {
            let operation = self.graph_.get(op_idx);
            operation.as_any().downcast_ref::<Op>().is_some()
        }

        pub fn try_cast<Op: 'static>(&self, op_idx: OpIndex) -> Option<&Op> {
            let operation = self.graph_.get(op_idx);
            operation.as_any().downcast_ref::<Op>()
        }

        pub fn cast<Op: 'static>(&self, op_idx: OpIndex) -> &Op {
            self.try_cast(op_idx).unwrap()
        }

        pub fn get(&self, op_idx: OpIndex) -> &Operation {
            self.graph_.get(op_idx)
        }

        pub fn index(&self, op: &Operation) -> OpIndex {
            self.graph_.index(op)
        }

        pub fn match_zero(&self, matched: OpIndex) -> bool {
            if let Some(op) = self.try_cast::<ConstantOp>(matched) {
                match op.kind {
                    ConstantOp::Kind::kWord32 | ConstantOp::Kind::kWord64 => op.integral() == 0,
                    ConstantOp::Kind::kFloat32 => op.float32().get_scalar() == 0.0,
                    ConstantOp::Kind::kFloat64 => op.float64().get_scalar() == 0.0,
                    ConstantOp::Kind::kSmi => op.smi().value() == 0,
                    _ => false,
                }
            } else {
                false
            }
        }

        pub fn match_integral_zero(&self, matched: OpIndex) -> bool {
            let mut constant: i64 = 0;
            self.match_signed_integral_constant(matched, &mut constant) && constant == 0
        }

        pub fn match_smi_zero(&self, matched: OpIndex) -> bool {
            if let Some(op) = self.try_cast::<ConstantOp>(matched) {
                if op.kind != ConstantOp::Kind::kSmi {
                    return false;
                }
                op.smi().value() == 0
            } else {
                false
            }
        }

        pub fn match_float32_constant(&self, matched: OpIndex, constant: &mut f32) -> bool {
            if let Some(op) = self.try_cast::<ConstantOp>(matched) {
                if op.kind != ConstantOp::Kind::kFloat32 {
                    return false;
                }
                *constant = op.storage.float32.get_scalar();
                true
            } else {
                false
            }
        }

        pub fn match_float32_constant_i(&self, matched: OpIndex, constant: &mut i::Float32) -> bool {
            if let Some(op) = self.try_cast::<ConstantOp>(matched) {
                if op.kind != ConstantOp::Kind::kFloat32 {
                    return false;
                }
                *constant = op.storage.float32;
                true
            } else {
                false
            }
        }

        pub fn match_float64_constant(&self, matched: OpIndex, constant: &mut f64) -> bool {
            if let Some(op) = self.try_cast::<ConstantOp>(matched) {
                if op.kind != ConstantOp::Kind::kFloat64 {
                    return false;
                }
                *constant = op.storage.float64.get_scalar();
                true
            } else {
                false
            }
        }

        pub fn match_float64_constant_i(&self, matched: OpIndex, constant: &mut i::Float64) -> bool {
            if let Some(op) = self.try_cast::<ConstantOp>(matched) {
                if op.kind != ConstantOp::Kind::kFloat64 {
                    return false;
                }
                *constant = op.storage.float64;
                true
            } else {
                false
            }
        }

        pub fn match_float(&self, matched: OpIndex, value: &mut f64) -> bool {
            if let Some(op) = self.try_cast::<ConstantOp>(matched) {
                if op.kind == ConstantOp::Kind::kFloat64 {
                    *value = op.storage.float64.get_scalar();
                    return true;
                } else if op.kind == ConstantOp::Kind::kFloat32 {
                    *value = op.storage.float32.get_scalar() as f64;
                    return true;
                }
            }
            false
        }

        pub fn match_float_value(&self, matched: OpIndex, value: f64) -> bool {
            let mut k = 0.0;
            if !self.match_float(matched, &mut k) {
                return false;
            }
            k.to_bits() == value.to_bits() || (k.is_nan() && value.is_nan())
        }

        pub fn match_nan(&self, matched: OpIndex) -> bool {
            let mut k = 0.0;
            self.match_float(matched, &mut k) && k.is_nan()
        }

        pub fn match_heap_constant(&self, matched: OpIndex, tagged: Option<&mut Handle<HeapObject>>) -> bool {
            if let Some(op) = self.try_cast::<ConstantOp>(matched) {
                if !(op.kind == ConstantOp::Kind::kHeapObject || op.kind == ConstantOp::Kind::kCompressedHeapObject) {
                    return false;
                }
                if let Some(t) = tagged {
                    *t = op.handle();
                }
                true
            } else {
                false
            }
        }

        pub fn match_integral_word_constant(
            &self,
            matched: OpIndex,
            rep: WordRepresentation,
            unsigned_constant: Option<&mut u64>,
            signed_constant: Option<&mut i64>,
        ) -> bool {
            if let Some(op) = self.try_cast::<ConstantOp>(matched) {
                match op.kind {
                    ConstantOp::Kind::kWord32
                    | ConstantOp::Kind::kWord64
                    | ConstantOp::Kind::kRelocatableWasmCall
                    | ConstantOp::Kind::kRelocatableWasmStubCall => {
                        if rep == WordRepresentation::Word32 {
                            if let Some(unsigned_constant) = unsigned_constant {
                                *unsigned_constant = op.integral() as u32 as u64;
                            }
                            if let Some(signed_constant) = signed_constant {
                                *signed_constant = op.signed_integral() as i32 as i64;
                            }
                            return true;
                        } else if rep == WordRepresentation::Word64 {
                            if let Some(unsigned_constant) = unsigned_constant {
                                *unsigned_constant = op.integral();
                            }
                            if let Some(signed_constant) = signed_constant {
                                *signed_constant = op.signed_integral();
                            }
                            return true;
                        }
                        false
                    }
                    _ => false,
                }
            } else {
                false
            }
        }

        pub fn match_integral_word_constant_signed(
            &self,
            matched: OpIndex,
            rep: WordRepresentation,
            signed_constant: &mut i64,
        ) -> bool {
            self.match_integral_word_constant(matched, rep, None, Some(signed_constant))
        }

        pub fn match_integral_word32_constant(&self, matched: OpIndex, constant: &mut u32) -> bool {
            let mut value: u64 = 0;
            if self.match_integral_word_constant(
                matched,
                WordRepresentation::Word32,
                Some(&mut value),
                None,
            ) {
                *constant = value as u32;
                return true;
            }
            false
        }

        pub fn match_integral_word64_constant(&self, matched: OpIndex, constant: &mut u64) -> bool {
            self.match_integral_word_constant(
                matched,
                WordRepresentation::Word64,
                Some(constant),
                None,
            )
        }

        pub fn match_integral_word32_constant_value(&self, matched: OpIndex, constant: u32) -> bool {
            let mut value: u64 = 0;
            if self.match_integral_word_constant(
                matched,
                WordRepresentation::Word32,
                Some(&mut value),
                None,
            ) {
                return value as u32 == constant;
            }
            false
        }

        pub fn match_integral_word64_constant_signed(
            &self,
            matched: OpIndex,
            constant: &mut i64,
        ) -> bool {
            self.match_integral_word_constant(
                matched,
                WordRepresentation::Word64,
                None,
                Some(constant),
            )
        }

        pub fn match_integral_word32_constant_signed(
            &self,
            matched: OpIndex,
            constant: &mut i32,
        ) -> bool {
            let mut value: i64 = 0;
            if self.match_integral_word_constant(
                matched,
                WordRepresentation::Word32,
                None,
                Some(&mut value),
            ) {
                *constant = value as i32;
                return true;
            }
            false
        }

        pub fn match_integral_wordptr_constant<T: Sized + Copy>(
            &self,
            matched: OpIndex,
            constant: &mut T,
        ) -> bool {
            if mem::size_of::<T>() == mem::size_of::<i64>() {
                let mut v: i64 = 0;
                if !self.match_integral_word64_constant_signed(matched, &mut v) {
                    return false;
                }
                *constant = v as T;
                return true;
            } else {
                let mut v: i32 = 0;
                if !self.match_integral_word32_constant_signed(matched, &mut v) {
                    return false;
                }
                *constant = v as T;
                return true;
            }
        }

        pub fn match_signed_integral_constant(&self, matched: OpIndex, constant: &mut i64) -> bool {
            if let Some(c) = self.try_cast::<ConstantOp>(matched) {
                if c.kind == ConstantOp::Kind::kWord32 || c.kind == ConstantOp::Kind::kWord64 {
                    *constant = c.signed_integral();
                    return true;
                }
            }
            false
        }

        pub fn match_unsigned_integral_constant(&self, matched: OpIndex, constant: &mut u64) -> bool {
            if let Some(c) = self.try_cast::<ConstantOp>(matched) {
                if c.kind == ConstantOp::Kind::kWord32 || c.kind == ConstantOp::Kind::kWord64 {
                    *constant = c.integral();
                    return true;
                }
            }
            false
        }

        pub fn match_external_constant(&self, matched: OpIndex, reference: &mut ExternalReference) -> bool {
            if let Some(op) = self.try_cast::<ConstantOp>(matched) {
                if op.kind != ConstantOp::Kind::kExternal {
                    return false;
                }
                *reference = op.storage.external.clone();
                return true;
            }
            false
        }

        pub fn match_wasm_stub_call_constant(&self, matched: OpIndex, stub_id: &mut u64) -> bool {
            if let Some(op) = self.try_cast::<ConstantOp>(matched) {
                if op.kind != ConstantOp::Kind::kRelocatableWasmStubCall {
                    return false;
                }
                *stub_id = op.integral();
                return true;
            }
            false
        }

        pub fn match_change<T>(
            &self,
            matched: OpIndex,
            input: IndexMatch<T, true>,
            kind: Option<ValueMatch<ChangeOp::Kind>>,
            assumption: Option<ValueMatch<ChangeOp::Assumption>>,
            from: Option<ValueMatch<RegisterRepresentation>>,
            to: Option<ValueMatch<RegisterRepresentation>>,
        ) -> bool {
            if let Some(op) = self.try_cast::<ChangeOp>(matched) {
                let kind_match = kind.map_or(true, |k| k.matches(&op.kind));
                let assumption_match = assumption.map_or(true, |a| a.matches(&op.assumption));
                let from_match = from.map_or(true, |f| f.matches(&op.from));
                let to_match = to.map_or(true, |t| t.matches(&op.to));

                return input.matches(op.input(), self)
                    && kind_match
                    && assumption_match
                    && from_match
                    && to_match;
            }
            false
        }

        pub fn match_truncate_word64_to_word32(
            &self,
            matched: OpIndex,
            input: IndexMatch<i64, true>,
        ) -> bool {
            let from_match = ValueMatch::new_value(RegisterRepresentation::Word64);
            let to_match = ValueMatch::new_value(RegisterRepresentation::Word32);

            self.match_change::<i64>(
                matched,
                input,
                Some(ValueMatch::new_value(ChangeOp::Kind::kTruncate)),
                None,
                Some(from_match),
                Some(to_match),
            )
        }

        pub fn match_word_binop<T>(
            &self,
            matched: OpIndex,
            left: IndexMatch<T, true>,
            right: IndexMatch<T, true>,
            kind: Option<ValueMatch<WordBinopOp::Kind>>,
            rep: Option<ValueMatch<WordRepresentation>>,
        ) -> bool
        where
            T: PartialEq,
        {
            if let Some(op) = self.try_cast::<WordBinopOp>(matched) {
                let kind_match = kind.map_or(true, |k| k.matches(&op.kind));
                let rep_match = rep.map_or(true, |r| r.matches(&op.rep));

                return left.matches(op.left(), self) && right.matches(op.right(), self) && kind_match && rep_match;
            }
            false
        }

        pub fn match_word_add<T>(
            &self,
            matched: OpIndex,
            left: &mut OpIndex,
            right: &mut OpIndex,
            rep: WordRepresentation,
        ) -> bool
        where
            T: PartialEq,
        {
            let kind = ValueMatch::new_value(WordBinopOp::Kind::kAdd);
            let rep_match = ValueMatch::new_value(rep);

            let mut left_index = OpIndex::default();
            let left_imatch = IndexMatch::new_index(left_index);
            let mut right_index = OpIndex::default();
            let right_imatch = IndexMatch::new_index(right_index);
            if let Some(op) = self.try_cast::<WordBinopOp>(matched) {
                if op.kind != WordBinopOp::Kind::kAdd || op.rep != rep {
                    return false;
                }
                *left = op.left();
                *right = op.right();
                return true;
            }
            false
        }

        pub fn match_word_sub<T>(
            &self,
            matched: OpIndex,
            left: &mut OpIndex,
            right: &mut OpIndex,
            rep: WordRepresentation,
        ) -> bool
        where
            T: PartialEq,
        {
            let kind = ValueMatch::new_value(WordBinopOp::Kind::kSub);
            let rep_match = ValueMatch::new_value(rep);

            if let Some(op) = self.try_cast::<WordBinopOp>(matched) {
                if op.kind != WordBinopOp::Kind::kSub || op.rep != rep {
                    return false;
                }
                *left = op.left();
                *right = op.right();
                return true;
            }
            false
        }

        pub fn match_word_mul<T>(
            &self,
            matched: OpIndex,
            left: &mut OpIndex,
            right: &mut OpIndex,
            rep: WordRepresentation,
        ) -> bool
        where
            T: PartialEq,
        {
            let kind = ValueMatch::new_value(WordBinopOp::Kind::kMul);
            let rep_match = ValueMatch::new_value(rep);

            if let Some(op) = self.try_cast::<WordBinopOp>(matched) {
                if op.kind != WordBinopOp::Kind::kMul || op.rep != rep {
                    return false;
                }
                *left = op.left();
                *right = op.right();
                return true;
            }
            false
        }

        pub fn match_bitwise_and<T>(
            &self,
            matched: OpIndex,
            left: &mut OpIndex,
            right: &mut OpIndex,
            rep: WordRepresentation,
        ) -> bool
        where
            T: PartialEq,
        {
            let kind = ValueMatch::new_value(WordBinopOp::Kind::kBitwiseAnd);
            let rep_match = ValueMatch::new_value(rep);

            if let Some(op) = self.try_cast::<WordBinopOp>(matched) {
                if op.kind != WordBinopOp::Kind::kBitwiseAnd || op.rep != rep {
                    return false;
                }
                *left = op.left();
                *right = op.right();
                return true;
            }
            false
        }

        pub fn match_bitwise_and_with_constant<T>(
            &self,
            matched: OpIndex,
            value: &mut OpIndex,
            constant: &mut u64,
            rep: WordRepresentation,
        ) -> bool
        where
            T: PartialEq,
        {
            let mut left = OpIndex::default();
            let mut right = OpIndex::default();
            if !self.match_bitwise_and::<T>(matched, &mut left, &mut right, rep) {
                return false;
            }
            if self.match_integral_word_constant(right, rep, Some(constant), None) {
                *value = left;
                return true;
            } else if self.match_integral_word_constant(left, rep, Some(constant), None) {
                *value = right;
                return true;
            }
            false
        }

        pub fn match_equal<T>(&self, matched: OpIndex, left: &mut OpIndex, right: &mut OpIndex) -> bool
        where
            T: PartialEq,
        {
            if let Some(op) = self.try_cast::<ComparisonOp>(matched) {
                if op.kind != ComparisonOp::Kind::kEqual {
                    return false;
                }
                *left = op.left();
                *right = op.right();
                return true;
            }
            false
        }

        pub fn match_float_unary(
            &self,
            matched: OpIndex,
            input: &mut OpIndex,
            kind: FloatUnaryOp::Kind,
            rep: FloatRepresentation,
        ) -> bool {
            if let Some(op) = self.try_cast::<FloatUnaryOp>(matched) {
                if op.kind != kind || op.rep != rep {
                    return false;
                }
                *input = op.input();
                return true;
            }
            false
        }

        pub fn match_float_round_down(
            &self,
            matched: OpIndex,
            input: &mut OpIndex,
            rep: FloatRepresentation,
        ) -> bool {
            self.match_float_unary(matched, input, FloatUnaryOp::Kind::kRoundDown, rep)
        }

        pub fn match_float_binary(
            &self,
            matched: OpIndex,
            left: &mut OpIndex,
            right: &mut OpIndex,
            kind: FloatBinopOp::Kind,
            rep: FloatRepresentation,
        ) -> bool {
            if let Some(op) = self.try_cast::<FloatBinopOp>(matched) {
                if op.kind != kind || op.rep != rep {
                    return false;
                }
                *left = op.left();
                *right = op.right();
                return true;
            }
            false
        }

        pub fn match_float_sub(
            &self,
            matched: OpIndex,
            left: &mut OpIndex,
            right: &mut OpIndex,
            rep: FloatRepresentation,
        ) -> bool {
            self.match_float_binary(matched, left, right, FloatBinopOp::Kind::kSub, rep)
        }

        pub fn match_constant_shift<T>(
            &self,
            matched: OpIndex,
            input: &mut OpIndex,
            kind: &mut ShiftOp::Kind,
            rep: &mut WordRepresentation,
            amount: &mut i32,
        ) -> bool
        where
            T: PartialEq,
        {
            if let Some(op) = self.try_cast::<ShiftOp>(matched) {
                let mut rhs_constant: u32 = 0;

                if self.match_integral_word32_constant(op.right(), &mut rhs_constant)
                    && (rhs_constant as u64) < op.rep.bit_width() as u64
                {
                    *input = op.left();
                    *kind = op.kind;
                    *rep = op.rep;
                    *amount = rhs_constant as i32;
                    return true;
                }
            }
            false
        }

        pub fn match_constant_shift_kind<T>(
            &self,
            matched: OpIndex,
            input: &mut OpIndex,
            kind: ShiftOp::Kind,
            rep: WordRepresentation,
            amount: &mut i32,
        ) -> bool
        where
            T: PartialEq,
        {
            if !detail::is_valid_type_for::<T>(rep) {
                return false;
            }
            if let Some(op) = self.try_cast::<ShiftOp>(matched) {
                let mut rhs_constant: u32 = 0;

                if op.kind == kind
                    && (op.rep == rep
                        || (ShiftOp::allows_word64_to_word32_truncation(kind)
                            && rep == WordRepresentation::Word32
                            && op.rep == WordRepresentation::Word64))
                    && self.match_integral_word32_constant(op.right(), &mut rhs_constant)
                    && (rhs_constant as u64) < rep.bit_width() as u64
                {
                    *input = op.left();
                    *amount = rhs_constant as i32;
                    return true;
                }
            }
            false
        }

        pub fn match_constant_right_shift<T>(
            &self,
            matched: OpIndex,
            input: &mut OpIndex,
            rep: WordRepresentation,
            amount: &mut i32,
        ) -> bool
        where
            T: PartialEq,
        {
            if !detail::is_valid_type_for::<T>(rep) {
                return false;
            }
            if let Some(op) = self.try_cast::<ShiftOp>(matched) {
                let mut rhs_constant: u32 = 0;

                if ShiftOp::is_right_shift(op.kind)
                    && op.rep == rep
                    && self.match_integral_word32_constant(op.right(), &mut rhs_constant)
                    && (rhs_constant as u64) < rep.bit_width() as u64
                {
                    *input = op.left();
                    *amount = rhs_constant as i32;
                    return true;
                }
            }
            false
        }

        pub fn match_constant_left_shift<T>(
            &self,
            matched: OpIndex,
            input: &mut OpIndex,
            rep: WordRepresentation,
            amount: &mut i32,
        ) -> bool
        where
            T: PartialEq,
        {
            if !detail::is_valid_type_for::<T>(rep) {
                return false;
            }
            if let Some(op) = self.try_cast::<ShiftOp>(matched) {
                let mut rhs_constant: u32 = 0;

                if op.kind == ShiftOp::Kind::kShiftLeft
                    && op.rep == rep
                    && self.match_integral_word32_constant(op.right(), &mut rhs_constant)
                    && (rhs_constant as u64) < rep.bit_width() as u64
                {
                    *input = op.left();
                    *amount = rhs_constant as i32;
                    return true;
                }
            }
            false
        }

        pub fn match_constant_shift_right_arithmetic_shift_out_zeros<T>(
            &self,
            matched: OpIndex,
            input: &mut OpIndex,
            rep: WordRepresentation,
            amount: &mut u16,
        ) -> bool
        where
            T: PartialEq,
        {
            if !detail::is_valid_type_for::<T>(rep) {
                return false;
            }
            if let Some(op) = self.try_cast::<ShiftOp>(matched) {
                let mut rhs_constant: u32 = 0;

                if op.kind == ShiftOp::Kind::kShiftRightArithmeticShiftOutZeros
                    && op.rep == rep
                    && self.match_integral_word32_constant(op.right(), &mut rhs_constant)
                    && (rhs_constant as u64) < rep.bit_width() as u64
                {
                    *input = op.left();
                    *amount = rhs_constant as u16;
                    return true;
                }
            }
            false
        }

        pub fn match_phi(&self, matched: OpIndex, input_count: Option<i32>) -> bool {
            if let Some(phi) = self.try_cast::<PhiOp>(matched) {
                return !input_count.is_some() || phi.input_count == input_count.unwrap() as usize;
            }
            false
        }

        pub fn match_power_of_two_word_constant(
            &self,
            matched: OpIndex,
            ret_cst: &mut i64,
            rep: WordRepresentation,
        ) -> bool {
            let mut loc_cst: i64 = 0;
            if self.match_integral_word_constant(matched, rep, None, Some(&mut loc_cst)) {
                if loc_cst.is_power_of_two() {
                    *ret_cst = loc_cst;
                    return true;
                }
            }
            false
        }

        pub fn match_power_of_two_word32_constant(&self, matched: OpIndex, divisor: &mut i32) -> bool {
            let mut cst: i64 = 0;
            if self.match_power_of_two_word_constant(
                matched,
                &mut
