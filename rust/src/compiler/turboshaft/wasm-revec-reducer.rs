// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: This is a partial translation. Some parts, especially related to
// the V8 graph representation and specific SIMD instruction matching, are
// simplified or stubbed out.  Full fidelity would require a complete
// reimplementation of the V8 compiler infrastructure in Rust.

use std::{
    any::Any,
    collections::{HashMap, HashSet},
    fmt,
    fmt::Write,
    ops::Sub,
};

// Stub for base::logging
macro_rules! trace {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            println!("Revec: {}", format_args!($($arg)*));
        }
    };
}

// Stub for v8_flags
mod v8_flags {
    pub const trace_wasm_revectorize: bool = true;
}

mod wasm {
    pub mod simd_shuffle {
        pub const kSimd128Size: usize = 16;
        pub fn canonicalize_shuffle(
            _: bool,
            _: &mut [u8],
            _: &mut bool,
            _: &mut bool,
        ) {
            todo!()
        }

        pub fn try_match32x8_shuffle(_: &[u8], _: &mut [u8]) -> bool {
            todo!()
        }

        pub fn try_match_vpshufd(_: &[u8], _: &mut u8) -> bool {
            todo!()
        }

        pub fn try_match_shufps256(_: &[u8], _: &mut u8) -> bool {
            todo!()
        }

        pub fn try_match_arch_shuffle(_: &[u8], _: bool, _: &*const ()) -> bool {
            todo!()
        }

        pub fn try_match_splat<const N: usize>(_: &[u8], _: &mut i32) -> bool {
            todo!()
        }
    }
}

pub mod opmasks {
    // This is a stub module. The real implementation would contain opmasks
    // related to specific SIMD instructions.
    pub trait OpmaskTrait {}
}

pub mod turboshaft {
    use super::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct OpIndex {
        id: u32,
        offset: u32, // Relative offset within the block
    }

    impl OpIndex {
        pub fn new(id: u32, offset: u32) -> Self {
            OpIndex { id, offset }
        }

        pub fn id(&self) -> u32 {
            self.id
        }

        pub fn offset(&self) -> u32 {
            self.offset
        }
    }

    // Placeholder for vector type. In real V8 codebase, it represents the SIMD
    // register index
    pub type V<T> = OpIndex;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Opcode {
        Simd128Unary,
        Simd128Binop,
        Simd128Shift,
        Simd128Ternary,
        Simd128Splat,
        Simd128Shuffle,
        Simd128ReplaceLane,
        Simd128LoadTransform,
        Load,
        Store,
        Phi,
        Simd128Constant,
    }

    pub fn opcode_name(opcode: Opcode) -> &'static str {
        match opcode {
            Opcode::Simd128Unary => "Simd128Unary",
            Opcode::Simd128Binop => "Simd128Binop",
            Opcode::Simd128Shift => "Simd128Shift",
            Opcode::Simd128Ternary => "Simd128Ternary",
            Opcode::Simd128Splat => "Simd128Splat",
            Opcode::Simd128Shuffle => "Simd128Shuffle",
            Opcode::Simd128ReplaceLane => "Simd128ReplaceLane",
            Opcode::Simd128LoadTransform => "Simd128LoadTransform",
            Opcode::Load => "Load",
            Opcode::Store => "Store",
            Opcode::Phi => "Phi",
            Opcode::Simd128Constant => "Simd128Constant",
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum MemoryRepresentation {
        Simd128,
        Word8,
        Word16,
        Word32,
        Word64,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum RegisterRepresentation {
        Simd128,
        Word32,
        Word64,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ChangeOpKind {
        SignedToFloat,
        UnsignedToFloat,
        ZeroExtend,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Simd128ExtractLaneOpKind {
        I8x16S,
        I8x16U,
        I16x8S,
        I16x8U,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct LoadKind {
        pub with_trap_handler: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TransformKind {
        K8x8S,
        K8x8U,
        K16x4S,
        K16x4U,
        K32x2S,
        K32x2U,
        K8Splat,
        K16Splat,
        K32Splat,
        K64Splat,
        K32Zero,
        K64Zero,
    }

    #[derive(Debug, Default, Clone)]
    pub struct EffectDimensions {
        pub control_flow: bool,
    }

    impl EffectDimensions {
        pub fn bits(&self) -> u32 {
            if self.control_flow {
                1
            } else {
                0
            }
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct OpEffects {
        pub consumes: EffectDimensions,
        pub produces: EffectDimensions,
    }

    impl OpEffects {
        pub fn new() -> Self {
            OpEffects {
                consumes: EffectDimensions::default(),
                produces: EffectDimensions::default(),
            }
        }
    }

    pub fn cannot_swap_operations(_first: OpEffects, _second: OpEffects) -> bool {
        false
    }

    // Trait for operations in the Turboshaft graph.
    pub trait Operation {
        fn opcode(&self) -> Opcode;
        fn inputs(&self) -> &[OpIndex];
        fn input(&self, index: usize) -> OpIndex {
            self.inputs()[index]
        }
        fn input_count(&self) -> usize {
            self.inputs().len()
        }
        fn print(&self) {}
        fn print_options(&self, _oss: &mut String) {}
        fn effects(&self) -> OpEffects {
            OpEffects::new()
        }
        fn as_any(&self) -> &dyn Any;
    }

    // Dummy implementation of constant operation.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum ConstantOpKind {
        Word32,
        Word64,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct ConstantOp {
        kind: ConstantOpKind,
        word32: u32,
        word64: u64,
    }

    impl ConstantOp {
        pub fn new_word32(word32: u32) -> Self {
            ConstantOp {
                kind: ConstantOpKind::Word32,
                word32,
                word64: 0,
            }
        }

        pub fn new_word64(word64: u64) -> Self {
            ConstantOp {
                kind: ConstantOpKind::Word64,
                word32: 0,
                word64,
            }
        }

        pub fn kind(&self) -> ConstantOpKind {
            self.kind
        }

        pub fn word32(&self) -> u32 {
            self.word32
        }

        pub fn word64(&self) -> u64 {
            self.word64
        }
    }

    impl Operation for ConstantOp {
        fn opcode(&self) -> Opcode {
            Opcode::Simd128Constant
        }

        fn inputs(&self) -> &[OpIndex] {
            &[]
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    // Dummy implementation of Simd128UnaryOp.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Simd128UnaryOpKind {
        Neg,
        Not,
        Abs,
        AnyTrue,
        AllTrue,
        // Sign Extension Operations
        I8x16SConvertI16x8Low,
        I8x16SConvertI16x8High,
        I16x8SConvertI32x4Low,
        I16x8SConvertI32x4High,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Simd128UnaryOp {
        kind: Simd128UnaryOpKind,
        input: OpIndex,
    }

    impl Simd128UnaryOp {
        pub fn new(kind: Simd128UnaryOpKind, input: OpIndex) -> Self {
            Simd128UnaryOp { kind, input }
        }

        pub fn kind(&self) -> Simd128UnaryOpKind {
            self.kind
        }

        pub fn input(&self) -> OpIndex {
            self.input
        }
    }

    impl Operation for Simd128UnaryOp {
        fn opcode(&self) -> Opcode {
            Opcode::Simd128Unary
        }

        fn inputs(&self) -> &[OpIndex] {
            &[self.input]
        }

        fn print_options(&self, oss: &mut String) {
            write!(oss, "{:?}", self.kind).unwrap();
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    // Dummy implementation of Simd128BinopOp.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Simd128BinopOpKind {
        Add,
        Sub,
        Mul,
        Div,
        And,
        Or,
        Xor,
        Min,
        Max,
        // Sign Extension Operations
        I16x8AddSaturateI8x16Low,
        I16x8AddSaturateI8x16High,
        I32x4AddSaturateI16x8Low,
        I32x4AddSaturateI16x8High,
    }

    impl Simd128BinopOpKind {
        pub fn is_commutative(self) -> bool {
            matches!(self, Simd128BinopOpKind::Add | Simd128BinopOpKind::Mul | Simd128BinopOpKind::And | Simd128BinopOpKind::Or | Simd128BinopOpKind::Xor)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Simd128BinopOp {
        kind: Simd128BinopOpKind,
        left: OpIndex,
        right: OpIndex,
    }

    impl Simd128BinopOp {
        pub fn new(kind: Simd128BinopOpKind, left: OpIndex, right: OpIndex) -> Self {
            Simd128BinopOp { kind, left, right }
        }

        pub fn kind(&self) -> Simd128BinopOpKind {
            self.kind
        }

        pub fn left(&self) -> OpIndex {
            self.left
        }

        pub fn right(&self) -> OpIndex {
            self.right
        }

        pub fn is_commutative(kind: Simd128BinopOpKind) -> bool {
            kind.is_commutative()
        }
    }

    impl Operation for Simd128BinopOp {
        fn opcode(&self) -> Opcode {
            Opcode::Simd128Binop
        }

        fn inputs(&self) -> &[OpIndex] {
            &[self.left, self.right]
        }

        fn print_options(&self, oss: &mut String) {
            write!(oss, "{:?}", self.kind).unwrap();
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    // Dummy implementation of Simd128ShiftOp.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Simd128ShiftOpKind {
        Left,
        Right,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Simd128ShiftOp {
        kind: Simd128ShiftOpKind,
        input: OpIndex,
        shift: OpIndex,
    }

    impl Simd128ShiftOp {
        pub fn new(kind: Simd128ShiftOpKind, input: OpIndex, shift: OpIndex) -> Self {
            Simd128ShiftOp { kind, input, shift }
        }

        pub fn kind(&self) -> Simd128ShiftOpKind {
            self.kind
        }

        pub fn shift(&self) -> OpIndex {
            self.shift
        }
    }

    impl Operation for Simd128ShiftOp {
        fn opcode(&self) -> Opcode {
            Opcode::Simd128Shift
        }

        fn inputs(&self) -> &[OpIndex] {
            &[self.input, self.shift]
        }

        fn print_options(&self, oss: &mut String) {
            write!(oss, "{:?}", self.kind).unwrap();
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    // Dummy implementation of Simd128TernaryOp.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Simd128TernaryOpKind {
        Bitselect,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Simd128TernaryOp {
        kind: Simd128TernaryOpKind,
        a: OpIndex,
        b: OpIndex,
        c: OpIndex,
    }

    impl Simd128TernaryOp {
        pub fn new(kind: Simd128TernaryOpKind, a: OpIndex, b: OpIndex, c: OpIndex) -> Self {
            Simd128TernaryOp { kind, a, b, c }
        }

        pub fn kind(&self) -> Simd128TernaryOpKind {
            self.kind
        }
    }

    impl Operation for Simd128TernaryOp {
        fn opcode(&self) -> Opcode {
            Opcode::Simd128Ternary
        }

        fn inputs(&self) -> &[OpIndex] {
            &[self.a, self.b, self.c]
        }

        fn print_options(&self, oss: &mut String) {
            write!(oss, "{:?}", self.kind).unwrap();
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    // Dummy implementation of Simd128SplatOp.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Simd128SplatOp {
        input: OpIndex,
    }

    impl Simd128SplatOp {
        pub fn new(input: OpIndex) -> Self {
            Simd128SplatOp { input }
        }

        pub fn input(&self) -> OpIndex {
            self.input
        }
    }

    impl Operation for Simd128SplatOp {
        fn opcode(&self) -> Opcode {
            Opcode::Simd128Splat
        }

        fn inputs(&self) -> &[OpIndex] {
            &[self.input]
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    // Dummy implementation of Simd128ShuffleOp.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Simd128ShuffleOpKind {
        I8x16,
    }

    #[derive(Debug, Clone)]
    pub struct Simd128ShuffleOp {
        pub kind: Simd128ShuffleOpKind,
        pub left: OpIndex,
        pub right: OpIndex,
        pub shuffle: [u8; 16],
    }

    impl Simd128ShuffleOp {
        pub fn new(kind: Simd128ShuffleOpKind, left: OpIndex, right: OpIndex, shuffle: [u8; 16]) -> Self {
            Simd128ShuffleOp { kind, left, right, shuffle }
        }

        pub fn left(&self) -> OpIndex {
            self.left
        }

        pub fn right(&self) -> OpIndex {
            self.right
        }
    }

    impl Operation for Simd128ShuffleOp {
        fn opcode(&self) -> Opcode {
            Opcode::Simd128Shuffle
        }

        fn inputs(&self) -> &[OpIndex] {
            &[self.left, self.right]
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    // Dummy implementation of Simd128ReplaceLaneOp.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Simd128ReplaceLaneOp {
        pub lane: u8,
        pub into: OpIndex,
        pub new_lane: OpIndex,
    }

    impl Simd128ReplaceLaneOp {
        pub fn new(lane: u8, into: OpIndex, new_lane: OpIndex) -> Self {
            Simd128ReplaceLaneOp { lane, into, new_lane }
        }
    }

    impl Operation for Simd128ReplaceLaneOp {
        fn opcode(&self) -> Opcode {
            Opcode::Simd128ReplaceLane
        }

        fn inputs(&self) -> &[OpIndex] {
            &[self.into, self.new_lane]
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    // Dummy implementation of LoadOp.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct LoadOp {
        pub loaded_rep: MemoryRepresentation,
        pub base: OpIndex,
        pub index: Option<OpIndex>,
        pub offset: i64,
        pub kind: LoadKind,
    }

    impl LoadOp {
        pub fn new(
            loaded_rep: MemoryRepresentation,
            base: OpIndex,
            index: Option<OpIndex>,
            offset: i64,
            kind: LoadKind,
        ) -> Self {
            LoadOp {
                loaded_rep,
                base,
                index,
                offset,
                kind,
            }
        }

        pub fn base(&self) -> OpIndex {
            self.base
        }
    }

    impl Operation for LoadOp {
        fn opcode(&self) -> Opcode {
            Opcode::Load
        }

        fn inputs(&self) -> &[OpIndex] {
            static EMPTY: [OpIndex; 0] = [];
            if self.index.is_some() {
                return &[self.base, self.index.unwrap()];
            }
            &EMPTY
        }

        fn effects(&self) -> OpEffects {
            let mut effects = OpEffects::new();
            effects.produces.control_flow = self.kind.with_trap_handler;
            effects
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    // Dummy implementation of StoreOp.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StoreOp {
        pub stored_rep: MemoryRepresentation,
        pub base: OpIndex,
        pub index: Option<OpIndex>,
        pub value: OpIndex,
        pub offset: i64,
        pub kind: LoadKind, // Using LoadKind for simplicity, consider creating a StoreKind if needed
        pub write_barrier: bool,
    }

    impl StoreOp {
        pub fn new(
            stored_rep: MemoryRepresentation,
            base: OpIndex,
            index: Option<OpIndex>,
            value: OpIndex,
            offset: i64,
            kind: LoadKind,
            write_barrier: bool,
        ) -> Self {
            StoreOp {
                stored_rep,
                base,
                index,
                value,
                offset,
                kind,
                write_barrier,
            }
        }

        pub fn base(&self) -> OpIndex {
            self.base
        }
    }

    impl Operation for StoreOp {
        fn opcode(&self) -> Opcode {
            Opcode::Store
        }

        fn inputs(&self) -> &[OpIndex] {
            static EMPTY: [OpIndex; 0] = [];
            if self.index.is_some() {
                return &[self.base, self.index.unwrap(), self.value];
            }
            &[self.base, self.value]
        }

        fn effects(&self) -> OpEffects {
            let mut effects = OpEffects::new();
            effects.consumes.control_flow = self.kind.with_trap_handler;
            effects
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    // Dummy implementation of Simd128LoadTransformOp.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Simd128LoadTransformOp {
        pub load_kind: LoadKind,
        pub transform_kind: TransformKind,
        pub base: OpIndex,
        pub index: OpIndex,
        pub offset: i64,
    }

    impl Simd128LoadTransformOp {
        pub fn new(
            load_kind: LoadKind,
            transform_kind: TransformKind,
            base: OpIndex,
            index: OpIndex,
            offset: i64,
        ) -> Self {
            Simd128LoadTransformOp {
                load_kind,
                transform_kind,
                base,
                index,
                offset,
            }
        }

        pub fn base(&self) -> OpIndex {
            self.base
        }

        pub fn index(&self) -> OpIndex {
            self.index
        }
    }

    impl Operation for Simd128LoadTransformOp {
        fn opcode(&self) -> Opcode {
            Opcode::Simd128LoadTransform
        }

        fn inputs(&self) -> &[OpIndex] {
            &[self.base, self.index]
        }

        fn effects(&self) -> OpEffects {
            let mut effects = OpEffects::new();
            effects.produces.control_flow = self.load_kind.with_trap_handler;
            effects
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    // Dummy implementation of PhiOp.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PhiOp {
        pub rep: RegisterRepresentation,
        pub inputs: [OpIndex; 2], // Simplified: Assuming a maximum of 2 inputs for Phi node
    }

    impl PhiOp {
        pub fn new(rep: RegisterRepresentation, inputs: [OpIndex; 2]) -> Self {
            PhiOp { rep, inputs }
        }
    }

    impl Operation for PhiOp {
        fn opcode(&self) -> Opcode {
            Opcode::Phi
        }

        fn inputs(&self) -> &[OpIndex] {
            &self.inputs
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    // Dummy implementation of ChangeOp.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ChangeOp {
        pub kind: ChangeOpKind,
        pub input: OpIndex,
    }

    impl ChangeOp {
        pub fn new(kind: ChangeOpKind, input: OpIndex) -> Self {
            ChangeOp { kind, input }
        }
    }

    impl Operation for ChangeOp {
        fn opcode(&self) -> Opcode {
            Opcode::Simd128Constant // Replace with actual opcode for ChangeOp if available
        }

        fn inputs(&self) -> &[OpIndex] {
            &[self.input]
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    // Dummy implementation of Simd128ExtractLaneOp.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Simd128ExtractLaneOp {
        pub kind: Simd128ExtractLaneOpKind,
        pub input: OpIndex,
        pub lane: u8,
    }

    impl Simd128ExtractLaneOp {
        pub fn new(kind: Simd128ExtractLaneOpKind, input: OpIndex, lane: u8) -> Self {
            Simd128ExtractLaneOp { kind, input, lane }
        }
    }

    impl Operation for Simd128ExtractLaneOp {
        fn opcode(&self) -> Opcode {
            Opcode::Simd128Constant // Replace with actual opcode if available
        }

        fn inputs(&self) -> &[OpIndex] {
            &[self.input]
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    // Dummy implementation of WordBinopOp
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum WordBinopOpKind {
        kAdd,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum WordRepresentation {
        Word64,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct WordBinopOp {
        pub kind: WordBinopOpKind,
        pub rep: WordRepresentation,
        pub left: OpIndex,
        pub right: OpIndex,
    }

    impl WordBinopOp {
        pub fn new(kind: WordBinopOpKind, rep: WordRepresentation, left: OpIndex, right: OpIndex) -> Self {
            WordBinopOp { kind, rep, left, right }
        }

        pub fn left(&self) -> OpIndex {
            self.left
        }

        pub fn right(&self) -> OpIndex {
            self.right
        }
    }

    impl Operation for WordBinopOp {
        fn opcode(&self) -> Opcode {
            Opcode::Simd128Constant // Replace with actual opcode for WordBinopOp if available
        }

        fn inputs(&self) -> &[OpIndex] {
            &[self.left, self.right]
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    // Simple Graph representation (replace the V8 Graph)
    pub struct Graph {
        operations: HashMap<OpIndex, Box<dyn Operation>>,
        blocks: Vec<Block>,
        previous: HashMap<OpIndex, OpIndex>,
        block_index_map: HashMap<OpIndex, usize>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                operations: HashMap::new(),
                blocks: Vec::new(),
                previous: HashMap::new(),
                block_index_map: HashMap::new(),
            }
        }

        pub fn insert(&mut self, op_index: OpIndex, operation: Box<dyn Operation>) {
            self.operations.insert(op_index, operation);
        }

        pub fn get(&self, op_index: OpIndex) -> &dyn Operation {
            self.operations
                .get(&op_index)
                .map(|op| op.as_ref())
                .expect("Operation not found")
        }

        pub fn try_cast<T: Operation + 'static>(&self, op_index: OpIndex) -> Option<&T> {
            self.operations
                .get(&op_index)
                .and_then(|op| op.as_any().downcast_ref::<T>())
        }

        pub fn previous_index(&self, op_index: OpIndex) -> OpIndex {
            *self.previous.get(&op_index).unwrap()
        }

        pub fn add_block(&mut self, block: Block) {
            let block_index = self.blocks.len();
            for &op_index in &block.operations {
                self.block_index_map.insert(op_index, block_index);
            }
            self.blocks.push(block);
        }

        pub fn block_index_of(&self, op_index: OpIndex) -> usize {
            *self.block_index_map.get(&op_index).unwrap()
        }

        pub fn blocks(&self) -> &[Block] {
            &self.blocks
        }

        pub fn operations(&self, block: &Block) -> impl Iterator<Item = &dyn Operation> {
            block.operations.iter().map(|&op_index| self.get(op_index))
        }

        pub fn index(&self, op: &dyn Operation) -> OpIndex {
            self.operations
                .iter()
                .find(|(_, value)| value.as_ref() as *const dyn Operation == op as *const dyn Operation)
                .map(|(index, _)| *index)
                .expect("Operation not found in graph")
        }

        pub fn set_previous(&mut self, op_index: OpIndex, prev_index: OpIndex) {
            self.previous.insert(op_index, prev_index);
        }
    }

    // Represents a basic block in the graph.
    #[derive(Debug, Clone)]
    pub struct Block {
        operations: Vec<OpIndex>, // List of operations within the block
    }

    impl Block {
        pub fn new(operations: Vec<OpIndex>) -> Self {
            Block { operations }
        }
    }

    // NodeGroup represents a group of nodes (e.g., for vectorization).
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NodeGroup {
        nodes: [OpIndex; 2],
    }

    impl NodeGroup {
        pub fn new(node0: OpIndex, node1: OpIndex) -> Self {
            NodeGroup { nodes: [node0, node1] }
        }

        pub fn size(&self) -> usize {
            self.nodes.len()
        }

        pub fn get(&self, index: usize) -> OpIndex {
            self.nodes[index]
        }

        pub fn as_slice(&self) -> &[OpIndex] {
            &self.nodes
        }
    }

    impl std::ops::Index<usize> for NodeGroup {
        type Output = OpIndex;

        fn index(&self, index: usize) -> &Self::Output {
            &self.nodes[index]
        }
    }

    // RevecAnalyzer finds vectorization opportunities.
    pub struct WasmRevecAnalyzer<'a> {
        graph_: &'a Graph,
        phase_zone_: &'a PhaseZone,
        revectorizable_node_: HashMap<OpIndex, PackNode<'a>>,
        revectorizable_intersect_node_: HashMap<OpIndex, ZoneVector<'a, PackNode<'a>>>,
        store_seeds_: Vec<std::pair::Pair<OpIndex, OpIndex>>,
        reduce_seeds_: Vec<std::pair::Pair<OpIndex, OpIndex>>,
        use_map_: *mut SimdUseMap<'a>,
        should_reduce_: bool,
    }

    impl<'a> WasmRevecAnalyzer<'a> {
        pub fn new(graph_: &'a Graph, phase_zone_: &'a PhaseZone) -> Self {
            WasmRevecAnalyzer {
                graph_,
                phase_zone_,
                revectorizable_node_: HashMap::new(),
                revectorizable_intersect_node_: HashMap::new(),
                store_seeds_: Vec::new(),
                reduce_seeds_: Vec::new(),
                use_map_: std::ptr::null_mut(),
                should_reduce_: false,
            }
        }

        pub fn run(&mut self) {
            self.store_seeds_.clear();
            self.reduce_seeds_.clear();
            for block in self.graph_.blocks().iter().rev() {
                self.process_block(block);
            }

            if self.store_seeds_.is_empty() && self.reduce_seeds_.is_empty() {
                trace!("Empty seed\n");
                return;
            }

            if v8_flags::trace_wasm_revectorize {
                println!("store seeds:");
                for pair in &self.store_seeds_ {
                    println!("{{");
                    println!("#{} ", pair.first.id());
                    self.graph_.get(pair.first).print();
                    