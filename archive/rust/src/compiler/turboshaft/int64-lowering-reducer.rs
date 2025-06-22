// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![cfg(feature = "wasm")] // Equivalent to #if !V8_ENABLE_WEBASSEMBLY, requires wasm feature

use std::any::Any;
use std::convert::TryInto;
use std::limits;
use std::ops::{BitAnd, BitOr, BitXor, Add, Sub, Mul, Shl, Shr};
use std::fmt;

//use crate::codegen::machine_type::MachineType; // Assuming a Rust equivalent exists
//use crate::compiler::turboshaft::assembler::Assembler; // Assuming a Rust equivalent exists
//use crate::compiler::turboshaft::operations::*; // Assuming Rust equivalents exist
//use crate::compiler::turboshaft::phase::Phase; // Assuming a Rust equivalent exists
//use crate::compiler::wasm_compiler; // Assuming a Rust equivalent exists
//use crate::compiler::wasm_graph_assembler; // Assuming a Rust equivalent exists
//use crate::wasm::wasm_engine; // Assuming a Rust equivalent exists

pub mod turboshaft {

  use std::any::Any;
  use std::convert::TryInto;
  use std::limits;
  use std::ops::{BitAnd, BitOr, BitXor, Add, Sub, Mul, Shl, Shr};
  use std::fmt;
  use std::vec::Vec;

  // Placeholder types and enums, replace with actual definitions
  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum MachineRepresentation {
    Word32,
    Word64,
    Float64,
    Int32,
    Int64,
    Uint64,
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum RegisterRepresentation {
    Word32,
    Word64,
    Float64,
    None,
  }

  impl RegisterRepresentation {
      pub fn from_machine_type(machine_type: MachineType) -> Self {
          match machine_type {
              MachineType::Int32 => RegisterRepresentation::Word32,
              MachineType::Int64 => RegisterRepresentation::Word64,
              MachineType::Float64 => RegisterRepresentation::Float64,
              _ => RegisterRepresentation::None,
          }
      }
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum MachineType {
      Int32,
      Int64,
      Float64,
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum WordRepresentation {
    Word32,
    Word64,
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum WordBinopOpKind {
    Add,
    Sub,
    Mul,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum ShiftOpKind {
    ShiftLeft,
    ShiftRightArithmetic,
    ShiftRightLogical,
    RotateRight,
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum ComparisonOpKind {
    Equal,
    SignedLessThan,
    SignedLessThanOrEqual,
    UnsignedLessThan,
    UnsignedLessThanOrEqual,
  }

  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct OpEffects {}
  impl OpEffects {
      pub fn CanCallAnything(&self) -> Self {
          OpEffects {}
      }
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum ConstantOpKind {
    Word64,
  }

  #[derive(Debug, Copy, Clone, PartialEq)]
  pub struct ConstantOpStorage {
    pub integral: u64,
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum WordUnaryOpKind {
    CountLeadingZeros,
    CountTrailingZeros,
    PopCount,
    SignExtend8,
    SignExtend16,
    ReverseBytes,
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum ChangeOpKind {
    ZeroExtend,
    SignExtend,
    Bitcast,
    Truncate,
  }
  
  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum ChangeOpAssumption {
    None,
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub struct LoadOpKind {
      pub is_atomic: bool,
      pub tagged_base: bool,
  }

  impl LoadOpKind {
      pub fn new(is_atomic: bool, tagged_base: bool) -> Self {
          LoadOpKind { is_atomic, tagged_base }
      }
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum MemoryRepresentation {
      Int32,
      Int64,
      Uint64,
      Float64,
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum StoreOpKind {
      Unkown,
      Atomic,
      Tagged,
  }

  impl StoreOpKind {
      pub fn new(kind: StoreOpKind, is_atomic: bool, tagged_base: bool) -> Self {
          match kind {
              StoreOpKind::Atomic => StoreOpKind::Atomic,
              StoreOpKind::Tagged => StoreOpKind::Tagged,
              _ => StoreOpKind::Unkown,
          }
      }

      pub fn is_atomic(&self) -> bool {
          match self {
              StoreOpKind::Atomic => true,
              _ => false,
          }
      }

      pub fn tagged_base(&self) -> bool {
          match self {
              StoreOpKind::Tagged => true,
              _ => false,
          }
      }
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum WriteBarrierKind {
    NoWriteBarrier,
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum IndirectPointerTag {
      kNone,
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum AtomicRMWOpBinOp {
    Add,
    CompareExchange,
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum MemoryAccessKind {
      kDefault,
  }

  #[derive(Debug, Clone)]
  pub struct Signature<T> {
    parameters: Vec<T>,
    returns: Vec<T>,
  }

  impl<T> Signature<T> {
      pub fn new(parameters: Vec<T>, returns: Vec<T>) -> Self {
          Signature { parameters, returns }
      }

      pub fn parameters(&self) -> &Vec<T> {
          &self.parameters
      }

      pub fn returns(&self) -> &Vec<T> {
          &self.returns
      }

      pub fn parameter_count(&self) -> usize {
          self.parameters.len()
      }

      pub fn return_count(&self) -> usize {
          self.returns.len()
      }

      pub fn GetParam(&self, index: usize) -> &T {
          &self.parameters[index]
      }

      pub fn GetReturn(&self, index: usize) -> &T {
          &self.returns[index]
      }
  }

  //struct AssemblerData {}
  pub struct AssemblerData {
      pub js_to_wasm: bool,
  }

  impl AssemblerData {
      pub fn is_js_to_wasm(&self) -> bool {
          self.js_to_wasm
      }

      pub fn wasm_module_sig(&self) -> Option<Signature<MachineRepresentation>> {
          None
      }

      pub fn wasm_canonical_sig(&self) -> Option<Signature<MachineRepresentation>> {
          None
      }
  }

  pub struct TSCallDescriptor {
      pub descriptor: CallDescriptor,
      pub can_throw: bool,
      // other fields
  }

  impl TSCallDescriptor {
      pub fn Create(descriptor: CallDescriptor, can_throw: bool, lazy_deopt_on_throw: LazyDeoptOnThrow, zone: &Zone) -> Self {
          TSCallDescriptor {
              descriptor,
              can_throw,
          }
      }
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum LazyDeoptOnThrow {
    kNo,
  }

  #[derive(Debug, Clone)]
  pub struct CallDescriptor {
      parameters: Vec<MachineType>,
      returns: Vec<MachineType>,
  }

  impl CallDescriptor {
      pub fn new(parameters: Vec<MachineType>, returns: Vec<MachineType>) -> Self {
          CallDescriptor { parameters, returns }
      }

      pub fn ParameterCount(&self) -> usize {
          self.parameters.len()
      }

      pub fn ReturnCount(&self) -> usize {
          self.returns.len()
      }

      pub fn GetParameterType(&self, index: usize) -> MachineType {
          self.parameters[index]
      }

      pub fn GetReturnType(&self, index: usize) -> MachineType {
          self.returns[index]
      }
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub struct FrameState {}
  impl FrameState {
      pub fn Nullopt() -> Option<Self> {
          None
      }
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum FrameStateType {
    kLiftoffFunction,
  }

  pub struct FrameStateFunctionInfo {
    function_type: FrameStateType,
    parameter_count: u16,
    max_arguments: i32,
    local_count: i32,
    shared_info: u32,
    wasm_liftoff_frame_size: i32,
    wasm_function_index: i32,
  }

  impl FrameStateFunctionInfo {
    pub fn new(function_type: FrameStateType, parameter_count: u16, max_arguments: i32, local_count: i32, shared_info: u32, wasm_liftoff_frame_size: i32, wasm_function_index: i32) -> Self {
        FrameStateFunctionInfo {
            function_type,
            parameter_count,
            max_arguments,
            local_count,
            shared_info,
            wasm_liftoff_frame_size,
            wasm_function_index,
        }
    }

    pub fn parameter_count(&self) -> u16 {
        self.parameter_count
    }

    pub fn local_count(&self) -> i32 {
        self.local_count
    }
    pub fn shared_info(&self) -> u32 {
        self.shared_info
    }
    pub fn max_arguments(&self) -> i32 {
        self.max_arguments
    }
    pub fn wasm_liftoff_frame_size(&self) -> i32 {
        self.wasm_liftoff_frame_size
    }
    pub fn wasm_function_index(&self) -> i32 {
        self.wasm_function_index
    }
  }

  pub struct FrameStateInfo {
      bailout_id: i32,
      state_combine: i32,
      function_info: FrameStateFunctionInfo,
  }

  impl FrameStateInfo {
      pub fn new(bailout_id: i32, state_combine: i32, function_info: FrameStateFunctionInfo) -> Self {
          FrameStateInfo {
              bailout_id,
              state_combine,
              function_info,
          }
      }

      pub fn bailout_id(&self) -> i32 {
          self.bailout_id
      }
      pub fn state_combine(&self) -> i32 {
          self.state_combine
      }
      pub fn function_info(&self) -> &FrameStateFunctionInfo {
          &self.function_info
      }
  }

  pub struct FrameStateData {
      pub machine_types: Vec<MachineType>,
      pub frame_state_info: FrameStateInfo,
  }

  impl FrameStateData {
    pub struct Builder {
      inputs: Vec<OpIndex>,
      inlined: bool,
    }

    impl Builder {
        pub fn new() -> Self {
            Builder {
                inputs: Vec::new(),
                inlined: false,
            }
        }

        pub fn AddParentFrameState(&mut self, frame_state: V<FrameState>) {
            self.inputs.push(frame_state.index);
            self.inlined = true;
        }

        pub fn AddInput(&mut self, machine_type: MachineType, input: OpIndex) {
            self.inputs.push(input);
        }

        pub fn Inputs(&self) -> &Vec<OpIndex> {
            &self.inputs
        }

        pub fn inlined(&self) -> bool {
            self.inlined
        }

        pub fn AllocateFrameStateData(&self, frame_state_info: FrameStateInfo, zone: &Zone) -> FrameStateData {
            let mut machine_types: Vec<MachineType> = Vec::new();
            for _ in 0..self.inputs.len() {
                machine_types.push(MachineType::Int32);
            }

            FrameStateData {
                machine_types,
                frame_state_info,
            }
        }
    }
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum Simd128SplatOpKind {
    kI64x2,
    kI32x4,
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum Simd128ExtractLaneOpKind {
    kI64x2,
    kI32x4,
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum Simd128ReplaceLaneOpKind {
    kI64x2,
    kI32x4,
  }

  pub trait NextReducer {
      fn reduce_word_binop(&mut self, left: OpIndex, right: OpIndex, kind: WordBinopOpKind, rep: WordRepresentation) -> OpIndex;
      fn reduce_shift(&mut self, left: OpIndex, right: V<Word32>, kind: ShiftOpKind, rep: WordRepresentation) -> OpIndex;
      fn reduce_comparison(&mut self, left: V<Any>, right: V<Any>, kind: ComparisonOpKind, rep: RegisterRepresentation) -> V<Word32>;
      fn reduce_call(&mut self, callee: V<CallTarget>, frame_state: Option<FrameState>, arguments: &Vec<OpIndex>, descriptor: &TSCallDescriptor, effects: OpEffects) -> V<Any>;
      fn reduce_tail_call(&mut self, callee: OpIndex, arguments: &Vec<OpIndex>, descriptor: &TSCallDescriptor) -> OpIndex;
      fn reduce_constant(&mut self, kind: ConstantOpKind, value: ConstantOpStorage) -> OpIndex;
      fn reduce_parameter(&mut self, parameter_index: i32, rep: RegisterRepresentation, debug_name: &str) -> OpIndex;
      fn reduce_return(&mut self, pop_count: V<Word32>, return_values: &Vec<OpIndex>, spill_caller_frame_slots: bool) -> V<None>;
      fn reduce_word_unary(&mut self, input: OpIndex, kind: WordUnaryOpKind, rep: WordRepresentation) -> OpIndex;
      fn reduce_change(&mut self, input: OpIndex, kind: ChangeOpKind, assumption: ChangeOpAssumption, from: RegisterRepresentation, to: RegisterRepresentation) -> OpIndex;
      fn reduce_load(&mut self, base: OpIndex, index: Option<OpIndex>, kind: LoadOpKind, loaded_rep: MemoryRepresentation, result_rep: RegisterRepresentation, offset: i32, element_scale: u8) -> OpIndex;
      fn reduce_store(&mut self, base: OpIndex, index: Option<OpIndex>, value: OpIndex, kind: StoreOpKind, stored_rep: MemoryRepresentation, write_barrier: WriteBarrierKind, offset: i32, element_size_log2: u8, maybe_initializing_or_transitioning: bool, maybe_indirect_pointer_tag: IndirectPointerTag) -> V<None>;
      fn reduce_atomic_rmw(&mut self, base: OpIndex, index: OpIndex, value: OpIndex, expected: Option<OpIndex>, bin_op: AtomicRMWOpBinOp, in_out_rep: RegisterRepresentation, memory_rep: MemoryRepresentation, kind: MemoryAccessKind) -> OpIndex;
      fn reduce_phi(&mut self, inputs: &Vec<OpIndex>, rep: RegisterRepresentation) -> OpIndex;
      fn reduce_pending_loop_phi(&mut self, input: OpIndex, rep: RegisterRepresentation) -> OpIndex;
      fn fix_loop_phi(&mut self, input_phi: &PhiOp, output_index: OpIndex, output_graph_loop: &Block);
      fn reduce_simd128_splat(&mut self, input: V<Any>, kind: Simd128SplatOpKind) -> V<Simd128>;
      fn reduce_simd128_extract_lane(&mut self, input: V<Simd128>, kind: Simd128ExtractLaneOpKind, lane: u8) -> V<Any>;
      fn reduce_simd128_replace_lane(&mut self, into_: V<Simd128>, new_lane: V<Any>, kind: Simd128ReplaceLaneOpKind, lane: u8) -> V<Simd128>;
      fn reduce_frame_state(&mut self, inputs: &Vec<OpIndex>, inlined: bool, data: &FrameStateData) -> V<FrameState>;
  }

  pub struct Block {}
  impl Block {
      pub fn Contains(&self, phi_index: OpIndex) -> bool {
          true // placeholder
      }
  }

  pub struct PhiOp {
      pub rep: RegisterRepresentation,
      inputs: Vec<OpIndex>,
  }
  impl PhiOp {
      pub fn input(&self, index: usize) -> OpIndex {
          self.inputs[index]
      }
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
  pub struct OpIndex(usize);

  impl OpIndex {
      pub fn Invalid() -> Self {
          OpIndex(usize::MAX)
      }
  }

  pub struct V<T> {
    index: OpIndex,
    _phantom: std::marker::PhantomData<T>,
  }

  impl<T> V<T> {
      pub fn new(index: OpIndex) -> Self {
          V { index, _phantom: std::marker::PhantomData }
      }

      pub fn Cast<U>(self) -> V<U> {
          V { index: self.index, _phantom: std::marker::PhantomData }
      }

      pub fn Invalid() -> Self {
          V { index: OpIndex::Invalid(), _phantom: std::marker::PhantomData }
      }
  }

  impl V<None> {
      pub fn Invalid() -> Self {
          V { index: OpIndex::Invalid(), _phantom: std::marker::PhantomData }
      }
  }

  impl V<Word32> {
      pub fn Cast<T>(self) -> V<T> {
          V { index: self.index, _phantom: std::marker::PhantomData }
      }
  }

  pub struct CallTarget {}
  pub struct Word32 {}
  pub struct Word32Pair {}
  pub struct Simd128 {}
  pub struct None {}
  pub struct AnyType {}

  pub trait OperationMatcherMethods {
      fn TryCast<T>(&self, input: V<Word32Pair>) -> Option<&T>;
      fn Is<T>(&self, input: V<Word32Pair>) -> bool;
      fn MatchIntegralWord32Constant(&self, shift: V<Word32>, constant_shift: &mut u32) -> bool;
  }

  pub struct OperationMatcher {}
  impl OperationMatcherMethods for OperationMatcher {
        fn TryCast<T>(&self, input: V<Word32Pair>) -> Option<&T> {
            None // Placeholder
        }
        fn Is<T>(&self, input: V<Word32Pair>) -> bool {
            false // Placeholder
        }
        fn MatchIntegralWord32Constant(&self, shift: V<Word32>, constant_shift: &mut u32) -> bool {
            false // Placeholder
        }
  }

  pub struct TupleOp {
      pub input_count: usize,
  }

  pub struct DidntThrowOp {}

  pub struct Word32PairBinopOp {}

  pub trait AssemblerMethods {
      fn data(&self) -> &AssemblerData;
      fn graph_zone(&self) -> &Zone;
      fn phase_zone(&self) -> &Zone;
      fn matcher(&self) -> &OperationMatcher;
      fn output_graph(&self) -> &mut OutputGraph;
  }

  pub struct Assembler {
      data: AssemblerData,
      graph_zone: Zone,
      phase_zone: Zone,
      matcher: OperationMatcher,
      output_graph: OutputGraph,
  }
  impl AssemblerMethods for Assembler {
      fn data(&self) -> &AssemblerData {
          &self.data
      }
      fn graph_zone(&self) -> &Zone {
          &self.graph_zone
      }
      fn phase_zone(&self) -> &Zone {
          &self.phase_zone
      }
      fn matcher(&self) -> &OperationMatcher {
          &self.matcher
      }
      fn output_graph(&self) -> &mut OutputGraph {
          &mut self.output_graph
      }
  }

  pub struct OutputGraph {}
  impl OutputGraph {
      fn template Replace<PhiOp>(&mut self, phi_index: OpIndex, inputs: Vec<OpIndex>, word32: RegisterRepresentation) {}
  }

  pub trait Int64LoweringReducerMethods {
      fn word32_equal(&self, left: V<Word32>, right: i32) -> V<Word32>;
      fn word32_bitwise_or(&self, left: V<Word32>, right: V<Word32>) -> V<Word32>;
      fn word32_bitwise_xor(&self, left: V<Word32>, right: V<Word32>) -> V<Word32>;
      fn int32_less_than(&self, left: V<Word32>, right: V<Word32>) -> V<Word32>;
      fn uint32_less_than(&self, left: V<Word32>, right: V<Word32>) -> V<Word32>;
      fn uint32_less_than_or_equal(&self, left: V<Word32>, right: V<Word32>) -> V<Word32>;
      fn tuple(&self, first: V<Word32>, second: V<Word32>) -> V<Word32Pair>;
      fn word32_constant(&self, value: u32) -> V<Word32>;
      fn float64_extract_low_word32(&self, input: V<Any>) -> V<Word32>;
      fn float64_extract_high_word32(&self, input: V<Any>) -> V<Word32>;
      fn bitcast_word32_pair_to_float64(&self, high: V<Word32>, low: V<Word32>) -> V<Any>;
      fn word32_sign_extend8(&self, input: V<Word32>) -> V<Word32>;
      fn word32_sign_extend16(&self, input: V<Word32>) -> V<Word32>;
      fn word32_reverse_bytes(&self, input: V<Word32>) -> V<Word32>;
      fn word32_shift_right_arithmetic(&self, input: V<Word32>, shift: i32) -> V<Word32>;
      fn word32_count_leading_zeros(&self, input: V<Word32>) -> V<Word32>;
      fn word32_count_trailing_zeros(&self, input: V<Word32>) -> V<Word32>;
      fn word32_pop_count(&self, input: V<Word32>) -> V<Word32>;
      fn word32_add(&self, left: V<Word32>, right: V<Word32>) -> V<Word32>;
      fn word32_pair_binop(&self, left_low: V<Word32>, left_high: V<Word32>, right_low: V<Word32>, right_high: V<Word32>, kind: Word32PairBinopOpKind) -> V<Word32Pair>;
      fn word32_bitwise_and(&self, left: V<Word32>, right: V<Word32>) -> V<Word32>;
      fn word32_shift_right_logical(&self, input: V<Word32>, shift: V<Word32>) -> V<Word32>;
      fn word32_shift_left(&self, input: V<Word32>, shift: V<Word32>) -> V<Word32>;
      fn word32_rotate_right(&self, input: V<Word32>, shift: V<Word32>) -> V<Word32>;
      fn pending_loop_phi(&self, input: OpIndex) -> V<Word32>;
      fn projection<const I: usize>(&self, input_w32p: V<Word32Pair>) -> V<Word32>;
      fn load(&mut self, base: OpIndex, index: Option<OpIndex>, kind: LoadOpKind, loaded_rep: MemoryRepresentation, result_rep: RegisterRepresentation, offset: i32, element_scale: u8) -> OpIndex;
      fn atomic_word32_pair_load(&mut self, base: OpIndex, index: Option<OpIndex>, offset: i32) -> OpIndex;
      fn atomic_word32_pair_store(&mut self, base: OpIndex, index: Option<OpIndex>, low: V<Word32>, high: V<Word32>, offset: i32) -> V<None>;
      fn atomic_word32_pair_binop(&mut self, base: OpIndex, index: OpIndex, value_low: V<Word32>, value_high: V<Word32>, bin_op: AtomicRMWOpBinOp) -> OpIndex;
      fn atomic_word32_pair_compare_exchange(&mut self, base: OpIndex, index: OpIndex, value_low: V<Word32>, value_high: V<Word32>, expected_low: V<Word32>, expected_high: V<Word32>) -> OpIndex;
      fn simd128_splat(&self, input: V<Word32>, kind: Simd128SplatOpKind) -> V<Simd128>;
      fn simd128_replace_lane(&self, into_: V<Simd128>, new_lane: V<Word32>, kind: Simd128ReplaceLaneOpKind, lane: u8) -> V<Simd128>;
      fn simd128_extract_lane(&self, input: V<Simd128>, kind: Simd128ExtractLaneOpKind, lane: u8) -> V<Any>;
  }

  pub struct Int64LoweringReducer<'a, N: NextReducer> {
    next: &'a mut N,
    sig: Option<Signature<MachineRepresentation>>, // Assuming Signature is defined
    zone: Zone,
    param_index_map: Vec<i32>,
    returns_i64: bool,
    matcher: OperationMatcher,
    assembler: &'a Assembler,
  }

  impl<'a, N: NextReducer> Int64LoweringReducer<'a, N> {
    pub fn new(next: &'a mut N, assembler: &'a Assembler) -> Self {
      let data = assembler.data();
      let origin = if data.is_js_to_wasm() {
        //wasm::kCalledFromJS
        panic!("wasm::kCalledFromJS");
      } else {
        //wasm::kCalledFromWasm
        panic!("wasm::kCalledFromWasm");
      };

      let sig = if let Some(wasm_module_sig) = data.wasm_module_sig() {
          //CreateMachineSignature(zone, data.wasm_module_sig(), origin)
          Some(wasm_module_sig)
      } else if let Some(wasm_canonical_sig) = data.wasm_canonical_sig() {
          //CreateMachineSignature(zone, data.wasm_canonical_sig(), origin)
          Some(wasm_canonical_sig)
      } else {
          None
      };

      let mut reducer = Int64LoweringReducer {
        next,
        sig: sig,
        zone: Zone::new(),
        param_index_map: Vec::new(),
        returns_i64: false,
        matcher: OperationMatcher{},
        assembler: assembler,
      };

      reducer.initialize_index_maps();
      reducer
    }

    fn reduce_word_binop(&mut self, left: OpIndex, right: OpIndex, kind: WordBinopOpKind, rep: WordRepresentation) -> OpIndex {
      if rep == WordRepresentation::Word64 {
        let left_pair: V<Word32Pair> = V::new(left).Cast();
        let right_pair: V<Word32Pair> = V::new(right).Cast();
        match kind {
          WordBinopOpKind::Add => {
            self.lower_pair_bin_op(left_pair, right_pair, Word32PairBinopOpKind::kAdd)
          }
          WordBinopOpKind::Sub => {
            self.lower_pair_bin_op(left_pair, right_pair, Word32PairBinopOpKind::kSub)
          }
          WordBinopOpKind::Mul => {
            self.lower_pair_bin_op(left_pair, right_pair, Word32PairBinopOpKind::kMul)
          }
          WordBinopOpKind::BitwiseAnd => {
            self.lower_bitwise_and(left_pair, right_pair)
          }
          WordBinopOpKind::BitwiseOr => {
            self.lower_bitwise_or(left_pair, right_pair)
          }
          WordBinopOpKind::BitwiseXor => {
            self.lower_bitwise_xor(left_pair, right_pair)
          }
        }
      } else {
        self.next.reduce_word_binop(left, right, kind, rep)
      }
    }

    fn reduce_shift(&mut self, left: OpIndex, right: V<Word32>, kind: ShiftOpKind, rep: WordRepresentation) -> OpIndex {
      if rep == WordRepresentation::Word64 {
        let left_pair: V<Word32Pair> = V::new(left).Cast();
        match kind {
          ShiftOpKind::ShiftLeft => {
            self.lower_pair_shift_op(left_pair, right, Word32PairBinopOpKind::kShiftLeft)
          }
          ShiftOpKind::ShiftRightArithmetic => {
            self.lower_pair_shift_op(left_pair, right, Word32PairBinopOpKind::kShiftRightArithmetic)
          }
          ShiftOpKind::ShiftRightLogical => {
            self.lower_pair_shift_op(left_pair, right, Word32PairBinopOpKind::kShiftRightLogical)
          }
          ShiftOpKind::RotateRight => {
            self.lower_rotate_right(left_pair, right)
          }
        }
      } else {
        self.next.reduce_shift(left, right, kind, rep)
      }
    }

    fn reduce_comparison(&mut self, left: V<Any>, right: V<Any>, kind: ComparisonOpKind, rep: RegisterRepresentation) -> V<Word32> {
      if rep != RegisterRepresentation::Word64 {
        return self.next.reduce_comparison(left, right, kind, rep);
      }

      let (left_low, left_high) = self.unpack(V::<Word32Pair>::Cast(left));
      let (right_low, right_high) = self.unpack(V::<Word32Pair>::Cast(right));
      let high_comparison: V<Word32>;
      let low_comparison: V<Word32>;

      match kind {
        ComparisonOpKind::Equal => {
          // TODO(wasm): Use explicit comparisons and && here?
          return self.word32_equal(
            self.word32_bitwise_or(self.word32_bitwise_xor(left_low, right_low),
                                      self.word32_bitwise_xor(left_high, right_high)),
            0);
        }
        ComparisonOpKind::SignedLessThan => {
          high_comparison = self.int32_less_than(left_high, right_high);
          low_comparison = self.uint32_less_than(left_low, right_low);
        }
        ComparisonOpKind::SignedLessThanOrEqual => {
          high_comparison = self.int32_less_than(left_high, right_high);
          low_comparison = self.uint32_less_than_or_equal(left_low, right_low);
        }
        ComparisonOpKind::UnsignedLessThan => {
          high_comparison = self.uint32_less_than(left_high, right_high);
          low_comparison = self.uint32_less_than(left_low, right_low);
        }
        ComparisonOpKind::