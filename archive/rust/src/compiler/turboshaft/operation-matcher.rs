// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{
    any::Any,
    convert::TryInto,
    fmt::{Debug, Display},
    marker::PhantomData,
    mem::size_of,
    num::FpCategory,
    ops::{BitAnd, BitOr, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub},
    optional::Option,
    sync::Arc,
};

pub mod turboshaft {
    use super::*;
    use std::{any::Any, variant::Variant};

    pub trait Repr {
        const REP: Self;
        fn bit_width(&self) -> usize;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum WordRepresentation {
        Word32,
        Word64,
    }

    impl WordRepresentation {
        pub fn value(&self) -> Self {
            *self
        }
        pub fn word32() -> Self {
            WordRepresentation::Word32
        }
        pub fn word64() -> Self {
            WordRepresentation::Word64
        }
    }
    impl Repr for WordRepresentation {
        const REP: Self = WordRepresentation::Word64;
        fn bit_width(&self) -> usize {
            match self {
                WordRepresentation::Word32 => 32,
                WordRepresentation::Word64 => 64,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum FloatRepresentation {
        Float32,
        Float64,
    }

    impl Repr for FloatRepresentation {
        const REP: Self = FloatRepresentation::Float64;
        fn bit_width(&self) -> usize {
            match self {
                FloatRepresentation::Float32 => 32,
                FloatRepresentation::Float64 => 64,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum RegisterRepresentation {
        Word32,
        Word64,
    }

    impl Repr for RegisterRepresentation {
        const REP: Self = RegisterRepresentation::Word64;
        fn bit_width(&self) -> usize {
            match self {
                RegisterRepresentation::Word32 => 32,
                RegisterRepresentation::Word64 => 64,
            }
        }
    }

    pub trait VType: 'static + Copy + Clone + Debug {
        const REP: Self;
    }

    macro_rules! define_v_type {
        ($name:ident, $rep:expr) => {
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
            pub struct $name;
            impl VType for $name {
                const REP: Self = $name;
            }
        };
    }

    define_v_type!(Any, ());
    define_v_type!(Word32, WordRepresentation::Word32);
    define_v_type!(Word64, WordRepresentation::Word64);
    define_v_type!(Float, FloatRepresentation::Float64);
    define_v_type!(AnyOrNone, ());

    pub struct V<T: VType> {
        index: OpIndex,
        _phantom: PhantomData<T>,
    }

    impl<T: VType> V<T> {
        pub fn new(index: OpIndex) -> Self {
            V {
                index,
                _phantom: PhantomData,
            }
        }
        pub fn cast(op_index: OpIndex) -> Self {
            V {
                index: op_index,
                _phantom: PhantomData,
            }
        }
    }

    impl<T: VType> Copy for V<T> {}

    impl<T: VType> Clone for V<T> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<T: VType> Debug for V<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("V")
                .field("index", &self.index)
                .field("type", &std::any::type_name::<T>())
                .finish()
        }
    }

    impl<T: VType> From<OpIndex> for V<T> {
        fn from(index: OpIndex) -> Self {
            V::new(index)
        }
    }

    impl<T: VType> V<T> {
        pub const rep: T = T::REP;

        pub fn get_index(&self) -> OpIndex {
            self.index
        }

        pub fn Cast(op_index: OpIndex) -> Self {
            V {
                index: op_index,
                _phantom: PhantomData,
            }
        }
    }

    pub type OpIndex = usize;

    pub trait Operation: Any {
        fn kind(&self) -> OperationKind;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum OperationKind {
        Constant,
        Change,
        WordBinop,
        Comparison,
        FloatUnary,
        FloatBinop,
        Shift,
        Phi,
    }

    pub type Handle<T> = Arc<T>;

    #[derive(Debug, Clone, Copy)]
    pub struct Float32 {
        pub value: f32,
    }

    impl Float32 {
        pub fn get_scalar(&self) -> f32 {
            self.value
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Float64 {
        pub value: f64,
    }

    impl Float64 {
        pub fn get_scalar(&self) -> f64 {
            self.value
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Smi {
        pub value: i64,
    }

    impl Smi {
        pub fn value(&self) -> i64 {
            self.value
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ConstantOpKind {
        Word32,
        Word64,
        Float32,
        Float64,
        Smi,
        HeapObject,
        CompressedHeapObject,
        External,
        RelocatableWasmCall,
        RelocatableWasmStubCall,
    }

    #[derive(Debug)]
    pub struct ConstantOp {
        pub kind: ConstantOpKind,
        pub storage: ConstantOpStorage,
        pub rep: WordRepresentation, // Added Representation field

        // Note: In the C++ code the 'storage' field is a union, meaning only one
        // of the fields within is valid at a time depending on the 'kind'.
        // In Rust, we can represent this using an enum.
    }

    #[derive(Debug)]
    pub enum ConstantOpStorage {
        Word32(u32),
        Word64(u64),
        Float32(Float32),
        Float64(Float64),
        Smi(Smi),
        HeapObject(Handle<HeapObject>),
        CompressedHeapObject(Handle<HeapObject>),
        External(ExternalReference),
        RelocatableWasmCall(u64),
        RelocatableWasmStubCall(u64),
    }

    impl Operation for ConstantOp {
        fn kind(&self) -> OperationKind {
            OperationKind::Constant
        }
    }

    impl ConstantOp {
        pub fn integral(&self) -> u64 {
            match self.kind {
                ConstantOpKind::Word32 => {
                    if let ConstantOpStorage::Word32(value) = self.storage {
                        value as u64
                    } else {
                        panic!("Unexpected storage type for Word32");
                    }
                }
                ConstantOpKind::Word64 => {
                    if let ConstantOpStorage::Word64(value) = self.storage {
                        value
                    } else {
                        panic!("Unexpected storage type for Word64");
                    }
                }
                ConstantOpKind::RelocatableWasmCall => {
                    if let ConstantOpStorage::RelocatableWasmCall(value) = self.storage {
                        value
                    } else {
                        panic!("Unexpected storage type for RelocatableWasmCall");
                    }
                }
                ConstantOpKind::RelocatableWasmStubCall => {
                    if let ConstantOpStorage::RelocatableWasmStubCall(value) = self.storage {
                        value
                    } else {
                        panic!("Unexpected storage type for RelocatableWasmStubCall");
                    }
                }
                _ => panic!("Integral access on non-integral ConstantOp"),
            }
        }

        pub fn signed_integral(&self) -> i64 {
            match self.kind {
                ConstantOpKind::Word32 => {
                    if let ConstantOpStorage::Word32(value) = self.storage {
                        value as i32 as i64
                    } else {
                        panic!("Unexpected storage type for Word32");
                    }
                }
                ConstantOpKind::Word64 => {
                    if let ConstantOpStorage::Word64(value) = self.storage {
                        value as i64
                    } else {
                        panic!("Unexpected storage type for Word64");
                    }
                }
                ConstantOpKind::RelocatableWasmCall => {
                    if let ConstantOpStorage::RelocatableWasmCall(value) = self.storage {
                        value as i64
                    } else {
                        panic!("Unexpected storage type for RelocatableWasmCall");
                    }
                }
                ConstantOpKind::RelocatableWasmStubCall => {
                    if let ConstantOpStorage::RelocatableWasmStubCall(value) = self.storage {
                        value as i64
                    } else {
                        panic!("Unexpected storage type for RelocatableWasmStubCall");
                    }
                }
                _ => panic!("Signed integral access on non-integral ConstantOp"),
            }
        }

        pub fn float32(&self) -> Float32 {
            match self.kind {
                ConstantOpKind::Float32 => {
                    if let ConstantOpStorage::Float32(value) = self.storage {
                        value
                    } else {
                        panic!("Unexpected storage type for Float32");
                    }
                }
                _ => panic!("Float32 access on non-Float32 ConstantOp"),
            }
        }

        pub fn float64(&self) -> Float64 {
            match self.kind {
                ConstantOpKind::Float64 => {
                    if let ConstantOpStorage::Float64(value) = self.storage {
                        value
                    } else {
                        panic!("Unexpected storage type for Float64");
                    }
                }
                _ => panic!("Float64 access on non-Float64 ConstantOp"),
            }
        }

        pub fn smi(&self) -> Smi {
            match self.kind {
                ConstantOpKind::Smi => {
                    if let ConstantOpStorage::Smi(value) = self.storage {
                        value
                    } else {
                        panic!("Unexpected storage type for Smi");
                    }
                }
                _ => panic!("Smi access on non-Smi ConstantOp"),
            }
        }

        pub fn handle(&self) -> Handle<HeapObject> {
            match &self.kind {
                ConstantOpKind::HeapObject => {
                    if let ConstantOpStorage::HeapObject(value) = &self.storage {
                        value.clone()
                    } else {
                        panic!("Unexpected storage type for HeapObject");
                    }
                }
                ConstantOpKind::CompressedHeapObject => {
                    if let ConstantOpStorage::CompressedHeapObject(value) = &self.storage {
                        value.clone()
                    } else {
                        panic!("Unexpected storage type for CompressedHeapObject");
                    }
                }
                _ => panic!("Handle access on non-HeapObject ConstantOp"),
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ChangeOpKind {
        kTruncate,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ChangeOpAssumption {
        //Assumption
    }

    #[derive(Debug)]
    pub struct ChangeOp {
        input_index: OpIndex,
        pub kind: ChangeOpKind,
        pub assumption: ChangeOpAssumption,
        pub from: RegisterRepresentation,
        pub to: RegisterRepresentation,
    }

    impl ChangeOp {
        pub fn input(&self) -> OpIndex {
            self.input_index
        }
    }
    impl Operation for ChangeOp {
        fn kind(&self) -> OperationKind {
            OperationKind::Change
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum WordBinopOpKind {
        kAdd,
        kSub,
        kMul,
        kBitwiseAnd,
    }

    #[derive(Debug)]
    pub struct WordBinopOp {
        left_index: OpIndex,
        right_index: OpIndex,
        pub kind: WordBinopOpKind,
        pub rep: WordRepresentation,
    }

    impl WordBinopOp {
        pub fn left(&self) -> OpIndex {
            self.left_index
        }
        pub fn right(&self) -> OpIndex {
            self.right_index
        }
    }

    impl Operation for WordBinopOp {
        fn kind(&self) -> OperationKind {
            OperationKind::WordBinop
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ComparisonOpKind {
        kEqual,
    }

    #[derive(Debug)]
    pub struct ComparisonOp {
        left_index: OpIndex,
        right_index: OpIndex,
        pub kind: ComparisonOpKind,
        pub rep: WordRepresentation, // Assuming WordRepresentation is the correct one
    }

    impl ComparisonOp {
        pub fn left(&self) -> OpIndex {
            self.left_index
        }
        pub fn right(&self) -> OpIndex {
            self.right_index
        }
    }
    impl Operation for ComparisonOp {
        fn kind(&self) -> OperationKind {
            OperationKind::Comparison
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum FloatUnaryOpKind {
        kRoundDown,
    }

    #[derive(Debug)]
    pub struct FloatUnaryOp {
        input_index: OpIndex,
        pub kind: FloatUnaryOpKind,
        pub rep: FloatRepresentation,
    }
    impl FloatUnaryOp {
        pub fn input(&self) -> OpIndex {
            self.input_index
        }
    }

    impl Operation for FloatUnaryOp {
        fn kind(&self) -> OperationKind {
            OperationKind::FloatUnary
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum FloatBinopOpKind {
        kSub,
    }

    #[derive(Debug)]
    pub struct FloatBinopOp {
        left_index: OpIndex,
        right_index: OpIndex,
        pub kind: FloatBinopOpKind,
        pub rep: FloatRepresentation,
    }

    impl FloatBinopOp {
        pub fn left(&self) -> OpIndex {
            self.left_index
        }
        pub fn right(&self) -> OpIndex {
            self.right_index
        }
    }

    impl Operation for FloatBinopOp {
        fn kind(&self) -> OperationKind {
            OperationKind::FloatBinop
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ShiftOpKind {
        kShiftLeft,
        kShiftRightLogical,
        kShiftRightArithmetic,
        kShiftRightArithmeticShiftOutZeros,
    }

    impl ShiftOpKind {
        pub fn is_right_shift(&self) -> bool {
            matches!(
                self,
                ShiftOpKind::kShiftRightArithmetic | ShiftOpKind::kShiftRightLogical | ShiftOpKind::kShiftRightArithmeticShiftOutZeros
            )
        }
    }

    #[derive(Debug)]
    pub struct ShiftOp {
        left_index: OpIndex,
        right_index: OpIndex,
        pub kind: ShiftOpKind,
        pub rep: WordRepresentation,
    }

    impl ShiftOp {
        pub fn left<T: VType>(&self) -> V<T> {
            V::<T>::Cast(self.left_index)
        }
        pub fn right(&self) -> OpIndex {
            self.right_index
        }

        pub fn AllowsWord64ToWord32Truncation(kind: ShiftOpKind) -> bool {
            matches!(kind, ShiftOpKind::kShiftRightArithmeticShiftOutZeros)
        }

        pub fn IsRightShift(kind: ShiftOpKind) -> bool {
            kind.is_right_shift()
        }
    }

    impl Operation for ShiftOp {
        fn kind(&self) -> OperationKind {
            OperationKind::Shift
        }
    }

    #[derive(Debug)]
    pub struct PhiOp {
        pub input_count: usize,
    }
    impl Operation for PhiOp {
        fn kind(&self) -> OperationKind {
            OperationKind::Phi
        }
    }

    #[derive(Debug, Default)]
    pub struct Graph {
        operations: Vec<Box<dyn Operation>>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                operations: Vec::new(),
            }
        }

        pub fn add<Op: Operation + 'static>(&mut self, op: Op) -> V<AnyOrNone> {
            self.operations.push(Box::new(op));
            V::from(self.operations.len() - 1)
        }

        pub fn Get(&self, op_idx: V<AnyOrNone>) -> &dyn Operation {
            self.operations[op_idx.get_index()].as_ref()
        }

        pub fn Index(&self, op: &dyn Operation) -> V<AnyOrNone> {
            for (index, boxed_op) in self.operations.iter().enumerate() {
                if boxed_op.as_ref() as *const dyn Operation == op {
                    return V::from(index);
                }
            }
            panic!("Operation not found in graph");
        }

        pub fn get_operation(&self, index: usize) -> Option<&dyn Operation> {
            self.operations.get(index).map(|op| op.as_ref())
        }
    }

    pub type ExternalReference = usize;

    pub type HeapObject = usize;

    pub struct OperationMatcher<'a> {
        graph_: &'a Graph,
    }

    impl<'a> OperationMatcher<'a> {
        pub fn new(graph: &'a Graph) -> Self {
            OperationMatcher { graph_: graph }
        }

        pub fn Is<Op: Operation + 'static>(&self, op_idx: V<AnyOrNone>) -> bool {
            self.graph_.get_operation(op_idx.get_index()).map_or(
                false,
                |op| op.type_id() == std::any::TypeId::of::<Op>(),
            )
        }

        pub fn TryCast<Op: Operation + 'static>(&self, op_idx: V<AnyOrNone>) -> Option<&Op> {
            self.graph_
                .get_operation(op_idx.get_index())
                .and_then(|op| (op as &dyn Any).downcast_ref::<Op>())
        }

        pub fn Cast<Op: Operation + 'static>(&self, op_idx: V<AnyOrNone>) -> &Op {
            self.TryCast(op_idx)
                .expect("Failed to cast operation to expected type")
        }

        pub fn Get(&self, op_idx: V<AnyOrNone>) -> &dyn Operation {
            self.graph_.Get(op_idx)
        }

        pub fn Index(&self, op: &dyn Operation) -> V<AnyOrNone> {
            self.graph_.Index(op)
        }

        pub fn MatchZero(&self, matched: V<Any>) -> bool {
            let op = self.TryCast::<ConstantOp>(matched);
            if op.is_none() {
                return false;
            }
            let op = op.unwrap();

            match op.kind {
                ConstantOpKind::Word32 => {
                    if let ConstantOpStorage::Word32(x) = op.storage {
                        x == 0
                    } else {
                        false
                    }
                }
                ConstantOpKind::Word64 => {
                    if let ConstantOpStorage::Word64(x) = op.storage {
                        x == 0
                    } else {
                        false
                    }
                }
                ConstantOpKind::Float32 => {
                    if let ConstantOpStorage::Float32(x) = op.storage {
                        x.get_scalar() == 0.0
                    } else {
                        false
                    }
                }
                ConstantOpKind::Float64 => {
                    if let ConstantOpStorage::Float64(x) = op.storage {
                        x.get_scalar() == 0.0
                    } else {
                        false
                    }
                }
                ConstantOpKind::Smi => {
                    if let ConstantOpStorage::Smi(x) = op.storage {
                        x.value() == 0
                    } else {
                        false
                    }
                }
                _ => false,
            }
        }

        pub fn MatchIntegralZero(&self, matched: V<Any>) -> bool {
            let mut constant = 0i64;
            self.MatchSignedIntegralConstant(matched, &mut constant) && constant == 0
        }

        pub fn MatchSmiZero(&self, matched: V<Any>) -> bool {
            let op = self.TryCast::<ConstantOp>(matched);
            if op.is_none() {
                return false;
            }
            let op = op.unwrap();
            if op.kind != ConstantOpKind::Smi {
                return false;
            }
            if let ConstantOpStorage::Smi(s) = op.storage {
                s.value() == 0
            } else {
                false
            }
        }

        pub fn MatchFloat32Constant(&self, matched: V<Any>, constant: &mut f32) -> bool {
            let op = self.TryCast::<ConstantOp>(matched);
            if op.is_none() {
                return false;
            }
            let op = op.unwrap();
            if op.kind != ConstantOpKind::Float32 {
                return false;
            }

            if let ConstantOpStorage::Float32(f) = op.storage {
                *constant = f.get_scalar();
                true
            } else {
                false
            }
        }

        pub fn MatchFloat32Constant2(&self, matched: V<Any>, constant: &mut Float32) -> bool {
            let op = self.TryCast::<ConstantOp>(matched);
            if op.is_none() {
                return false;
            }
            let op = op.unwrap();
            if op.kind != ConstantOpKind::Float32 {
                return false;
            }

            if let ConstantOpStorage::Float32(f) = op.storage {
                *constant = f;
                true
            } else {
                false
            }
        }

        pub fn MatchFloat64Constant(&self, matched: V<Any>, constant: &mut f64) -> bool {
            let op = self.TryCast::<ConstantOp>(matched);
            if op.is_none() {
                return false;
            }
            let op = op.unwrap();
            if op.kind != ConstantOpKind::Float64 {
                return false;
            }

            if let ConstantOpStorage::Float64(f) = op.storage {
                *constant = f.get_scalar();
                true
            } else {
                false
            }
        }

        pub fn MatchFloat64Constant2(&self, matched: V<Any>, constant: &mut Float64) -> bool {
            let op = self.TryCast::<ConstantOp>(matched);
            if op.is_none() {
                return false;
            }
            let op = op.unwrap();
            if op.kind != ConstantOpKind::Float64 {
                return false;
            }

            if let ConstantOpStorage::Float64(f) = op.storage {
                *constant = f;
                true
            } else {
                false
            }
        }

        pub fn MatchFloat(&self, matched: V<Any>, value: &mut f64) -> bool {
            let op = self.TryCast::<ConstantOp>(matched);
            if op.is_none() {
                return false;
            }
            let op = op.unwrap();
            if op.kind == ConstantOpKind::Float64 {
                if let ConstantOpStorage::Float64(f) = op.storage {
                    *value = f.get_scalar();
                    return true;
                }
            } else if op.kind == ConstantOpKind::Float32 {
                if let ConstantOpStorage::Float32(f) = op.storage {
                    *value = f.get_scalar() as f64;
                    return true;
                }
            }
            false
        }

        pub fn MatchFloat2(&self, matched: V<Any>, value: f64) -> bool {
            let mut k = 0.0;
            if !self.MatchFloat(matched, &mut k) {
                return false;
            }
            k.to_bits() == value.to_bits() || (k.is_nan() && value.is_nan())
        }

        pub fn MatchNaN(&self, matched: V<Float>) -> bool {
            let mut k = 0.0;
            self.MatchFloat(V::<Any>::cast(matched.index), &mut k) && k.is_nan()
        }

        pub fn MatchHeapConstant(
            &self,
            matched: V<Any>,
            tagged: &mut Option<Handle<HeapObject>>,
        ) -> bool {
            let op = self.TryCast::<ConstantOp>(matched);
            if op.is_none() {
                return false;
            }
            let op = op.unwrap();
            if !(op.kind == ConstantOpKind::HeapObject
                || op.kind == ConstantOpKind::CompressedHeapObject)
            {
                return false;
            }

            match &op.storage {
                ConstantOpStorage::HeapObject(handle)
                | ConstantOpStorage::CompressedHeapObject(handle) => {
                    *tagged = Some(handle.clone());
                    true
                }
                _ => false,
            }
        }

        pub fn MatchIntegralWordConstant(
            &self,
            matched: V<Any>,
            rep: WordRepresentation,
            unsigned_constant: &mut Option<u64>,
            signed_constant: &mut Option<i64>,
        ) -> bool {
            let op = self.TryCast::<ConstantOp>(matched);
            if op.is_none() {
                return false;
            }
            let op = op.unwrap();

            match op.kind {
                ConstantOpKind::Word32 | ConstantOpKind::Word64 | ConstantOpKind::RelocatableWasmCall | ConstantOpKind::RelocatableWasmStubCall => {
                    if rep == WordRepresentation::Word32 {
                        if let ConstantOpStorage::Word32(value) = op.storage {
                            if let Some(unsigned_constant) = unsigned_constant {
                                *unsigned_constant = value as u64;
                            }
                            if let Some(signed_constant) = signed_constant {
                                *signed_constant = value as i32 as i64;
                            }
                            return true;
                        } else {
                            return false;
                        }
                    } else if rep == WordRepresentation::Word64 {
                        match op.storage {
                            ConstantOpStorage::Word64(value) => {
                                if let Some(unsigned_constant) = unsigned_constant {
                                    *unsigned_constant = value;
                                }
                                if let Some(signed_constant) = signed_constant {
                                    *signed_constant = value as i64;
                                }
                                return true;
                            }
                            ConstantOpStorage::RelocatableWasmCall(value) => {
                                if let Some(unsigned_constant) = unsigned_constant {
                                    *unsigned_constant = value;
                                }
                                if let Some(signed_constant) = signed_constant {
                                    *signed_constant = value as i64;
                                }
                                return true;
                            }
                            ConstantOpStorage::RelocatableWasmStubCall(value) => {
                                if let Some(unsigned_constant) = unsigned_constant {
                                    *unsigned_constant = value;
                                }
                                if let Some(signed_constant) = signed_constant {
                                    *signed_constant = value as i64;
                                }
                                return true;
                            }
                            _ => return false,
                        }
                    }
                    false
                }
                _ => false,
            }
        }

        pub fn MatchIntegralWordConstant2(
            &self,
            matched: V<Any>,
            rep: WordRepresentation,
            signed_constant: &mut i64,
        ) -> bool {
            let mut unsigned_constant = None;
            let mut signed_constant_option = Some(signed_constant);
            let result = self.MatchIntegralWordConstant(
                matched,
                rep,
                &mut unsigned_constant,
                signed_constant_option,
            );
            if signed_constant_option.is_some() {
                *signed_constant = *signed_constant_option.unwrap();
            }
            result
        }

        pub fn MatchIntegralWord32Constant(&self, matched: V<Any>, constant: &mut u32) -> bool {
            let mut value: Option<u64> = None;
            if self.MatchIntegralWordConstant(
                matched,
                WordRepresentation::Word32,
                &mut value,
                &mut None,
            ) {
                if value.is_some() {
                    *constant = value.unwrap() as u32;
                    return true;
                }
            }
            false
        }

        pub fn MatchIntegralWord64Constant(&self, matched: V<Any>, constant: &mut u64) -> bool {
            let mut value: Option<u64> = None;
            self.MatchIntegralWordConstant(
                matched,
                WordRepresentation::Word64,
                &mut value,
                &mut None,
            )
        }

        pub fn MatchIntegralWord32Constant2(&self, matched: V<Any>, constant: u32) -> bool {
            let mut value: Option<u64> = None;
            if self.MatchIntegralWordConstant(
                matched,
                WordRepresentation::Word32,
                &mut value,
                &mut None,
            ) {
                if value.is_some() {
                    return value.unwrap() as u32 == constant;
                }
            }
            false
        }

        pub fn MatchIntegralWord64Constant2(&self, matched: V<Any>, constant: &mut i64) -> bool {
            self.MatchIntegralWordConstant(
                matched,
                WordRepresentation::Word64,
                &mut None,
                &mut Some(constant),
            )
        }

        pub fn MatchIntegralWord32Constant3(&self, matched: V<Any>, constant: &mut i32) -> bool {
            let mut value: i64 = 0;
            if self.MatchIntegralWordConstant2(matched, WordRepresentation::Word32, &mut value) {
                *constant = value as i32;
                return true;
            }
            false
        }

        pub fn MatchIntegralWordPtrConstant<T: TryFrom<i64>>(
            &self,
            matched: V<Any>,
            constant: &mut T,
        ) -> bool {
            if size_of::<T>() == size_of::<i64>() {
                let mut v = 0i64;
                if !self.MatchIntegralWord64Constant2(matched, &mut v) {
                    return false;
                }
                if let Ok(converted) = v.try_into() {
                    *constant = converted;
                    return true;
                } else {
                    return false;
                }
            } else {
                let mut v = 0i32;
                if !self.MatchIntegralWord32Constant3(matched, &mut v) {
                    return false;
                }
                if let Ok(converted) = v.try_into() {
                    *constant = converted;
                    return true;
                } else {
                    return false;
                }
            }
        }

        pub fn MatchSignedIntegralConstant(&self, matched: V<Any>, constant: &mut i64) -> bool {
            let op = self.TryCast::<ConstantOp>(matched);
            if op.is_none() {
                return false;
            }
            let op = op.unwrap();

            if op.kind == ConstantOpKind::Word32 || op.kind == ConstantOpKind::Word64 {
                *constant = op.signed_integral();
                return true;
            }

            false
        }

        pub fn MatchUnsignedIntegralConstant(
            &self,
            matched: V<Any>,
            constant: &mut u64,
        ) -> bool {
            let op = self.TryCast::<ConstantOp>(matched);
            if op.is_none() {
                return false;
            }
            let op = op.unwrap();

            if op.kind == ConstantOpKind::Word32 || op.kind == ConstantOpKind::Word64 {
                *constant = op.integral();
                return true;
            }
            false
        }

        pub fn Match