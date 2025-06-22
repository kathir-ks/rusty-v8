// NOTE: This is a partial conversion and may not be fully functional.

use std::{
    cmp,
    convert::TryFrom,
    i32,
    i64,
    mem,
    num::Wrapping,
    ops::{Add, BitAnd, BitOr, BitXor, Mul, Neg, Shl, Shr, Sub},
};

// use crate::base::bounds; // TODO: Implement or find suitable crate
// use crate::base::iterator; // TODO: Implement or find suitable crate
// use crate::base::logging; // TODO: Implement or find suitable crate
// use crate::base::overflowing_math; // TODO: Implement or find suitable crate
// use crate::codegen::cpu_features; // TODO: Implement or find suitable crate
// use crate::codegen::machine_type; // TODO: Implement or find suitable crate
// use crate::common::assert_scope; // TODO: Implement or find suitable crate
// use crate::common::globals; // TODO: Implement or find suitable crate
// use crate::compiler::backend::instruction_codes; // TODO: Implement or find suitable crate
// use crate::compiler::backend::instruction_selector_adapter; // TODO: Implement or find suitable crate
// use crate::compiler::backend::instruction_selector_impl; // TODO: Implement or find suitable crate
// use crate::compiler::backend::instruction_selector; // TODO: Implement or find suitable crate
// use crate::compiler::backend::instruction; // TODO: Implement or find suitable crate
// use crate::compiler::machine_operator; // TODO: Implement or find suitable crate
// use crate::compiler::turboshaft::load_store_simplification_reducer; // TODO: Implement or find suitable crate
// use crate::compiler::turboshaft::operations; // TODO: Implement or find suitable crate
// use crate::compiler::turboshaft::opmasks; // TODO: Implement or find suitable crate
// use crate::compiler::turboshaft::representations; // TODO: Implement or find suitable crate
// use crate::handles::handles_inl; // TODO: Implement or find suitable crate
// use crate::objects::slots_inl; // TODO: Implement or find suitable crate
// use crate::roots::roots_inl; // TODO: Implement or find suitable crate

// #[cfg(V8_ENABLE_WEBASSEMBLY)]
// use crate::wasm::simd_shuffle; // TODO: Implement or find suitable crate

// Dummy definitions for types that are not converted.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct OpIndex(usize);

impl OpIndex {
    fn valid(&self) -> bool {
        self.0 != 0
    }
    fn invalid() -> Self {
        OpIndex(0)
    }
    fn value_or_invalid(&self) -> OpIndex {
        if self.valid() { *self } else { OpIndex::invalid() }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct OptionalOpIndex(OpIndex);

impl OptionalOpIndex {
    fn valid(&self) -> bool {
        self.0.valid()
    }

    fn value_or_invalid(&self) -> OpIndex {
        self.0.value_or_invalid()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum MachineRepresentation {
    kNone,
    kBit,
    kWord8,
    kWord16,
    kWord32,
    kWord64,
    kFloat32,
    kFloat64,
    kTagged,
    kTaggedSigned,
    kTaggedPointer,
    kMapWord,
    kIndirectPointer,
    kSimd128,
    kSimd256,
    kCompressed,
    kCompressedPointer,
    kProtectedPointer,
    kSandboxedPointer,
    kAnyTagged,
    kAnyUncompressedTagged,
    kUncompressedTaggedPointer,
    kUncompressedTaggedSigned,
    kFloat16,
    kFloat16RawBits,
}

impl MachineRepresentation {
    fn is_compressed(&self) -> bool {
        *self == MachineRepresentation::kCompressed || *self == MachineRepresentation::kCompressedPointer
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum DisplacementMode {
    kPositiveDisplacement,
    kNegativeDisplacement,
}

const kPositiveDisplacement: DisplacementMode = DisplacementMode::kPositiveDisplacement;
// const kNegativeDisplacement: DisplacementMode = DisplacementMode::kNegativeDisplacement;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ArchOpcode {
    kArchNop,
    kArchStackSlot,
    kArchAbortCSADcheck,
    kX64Movsxbl,
    kX64Movzxbl,
    kX64Movsxwl,
    kX64Movzxwl,
    kX64Movl,
    kX64Movq,
    kX64Movsh,
    kX64Movss,
    kX64Movsd,
    kX64MovqDecompressTagged,
    kX64MovqDecompressTaggedSigned,
    kX64MovqCompressTagged,
    kX64MovqDecompressProtected,
    kX64MovqDecodeSandboxedPointer,
    kX64MovqStoreIndirectPointer,
    kX64MovqEncodeSandboxedPointer,
    kX64Movdqu,
    kX64Movdqu256,
    kX64And,
    kX64Or,
    kX64Xor,
    kX64Add,
    kX64Sub,
    kX64Push,
    kX64Cmp,
    kX64Test,
    kX64And32,
    kX64Or32,
    kX64Xor32,
    kX64Add32,
    kX64Sub32,
    kX64Cmp32,
    kX64Test32,
    kX64Cmp16,
    kX64Test16,
    kX64Cmp8,
    kX64Test8,
    kX64Shl32,
    kX64Shr32,
    kX64Sar32,
    kX64Rol32,
    kX64Ror32,
    kX64Shl,
    kX64Shr,
    kX64Sar,
    kX64Rol,
    kX64Ror,
    kX64Bswap,
    kX64Bswap32,
    kX64Imul32,
    kX64Imul,
    kX64ImulHigh32,
    kX64ImulHigh64,
    kX64Idiv32,
    kX64Idiv,
    kX64Udiv32,
    kX64Udiv,
    kX64UmulHigh32,
    kX64UmulHigh64,
    kX64Pinsrb,
    kX64Pinsrw,
    kX64Pinsrd,
    kX64Pinsrq,
    kX64Pextrb,
    kX64Pextrw,
    kSSEFloat32ToInt64,
    kSSEFloat64ToInt64,
    kSSEFloat32ToUint64,
    kSSEFloat64ToUint64,
    kSSEFloat64ToInt32,
    kX64Lea32,
    kX64Lea,
    kX64Neg,
    kX64Movsxlq,
    kX64Movzxbq,
    kX64Movzxwq,
    kX64MovzxbqForDecompressTagged,
    kX64Not32,
    kX64Not,
    kX64S128Load8x8S,
    kX64S128Load8x8U,
    kX64S128Load16x4S,
    kX64S128Load16x4U,
    kX64S128Load32x2S,
    kX64S128Load32x2U,
    kX64S128Load8Splat,
    kX64S128Load16Splat,
    kX64S128Load32Splat,
    kX64S128Load64Splat,
    kX64S128Store32Lane,
    kX64S128Store64Lane,
    kX64MovsxlqForDecompressTagged,
    kAtomicStoreWord8,
    kAtomicStoreWord16,
    kAtomicStoreWord32,
    kX64Word64AtomicStoreWord64,
    kArchAtomicStoreWithWriteBarrier,
    kArchStoreWithWriteBarrier,
    kArchStoreIndirectWithWriteBarrier,
    kX64S256Load8x16S,
    kX64S256Load8x16U,
    kX64S256Load8x8U,
    kX64S256Load16x8S,
    kX64S256Load16x8U,
    kX64S256Load32x4S,
    kX64S256Load32x4U,
    kX64S256Load8Splat,
    kX64S256Load16Splat,
    kX64S256Load32Splat,
    kX64S256Load64Splat,
    kX64S256Const,
    kX64SZero,
    kX64SAllOnes,
    kX64Vpshufd,
    kX64Shufps,
    kX64S32x8UnpackHigh,
    kX64S32x8UnpackLow,
    kX64InsertI128,
    kX64Minps,
    kX64Maxps,
    kX64Minpd,
    kX64Maxpd,
    kArchStackPointerGreaterThan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AddressingMode {
    kMode_None,
    kMode_MR,
    kMode_MRI,
    kMode_MR1,
    kMode_MR2,
    kMode_MR4,
    kMode_MR8,
    kMode_MR1I,
    kMode_MR2I,
    kMode_MR4I,
    kMode_MR8I,
    kMode_Root,
    kMode_M1,
    kMode_M2,
    kMode_M4,
    kMode_M8,
    kMode_M1I,
    kMode_M2I,
    kMode_M4I,
    kMode_M8I,
    kMode_MCR,
    kMode_MCRI,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RegisterRepresentation {
    Word32,
    Word64,
    Float32,
    Float64,
    Tagged,
    Simd128,
    Simd256,
    Compressed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MemoryRepresentation {
    Int8,
    Uint8,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
    Float32,
    Float64,
    TaggedSigned,
    TaggedPointer,
    Tagged,
    MapWord,
    IndirectPointer,
    Simd128,
    Simd256,
    AnyTagged,
    AnyUncompressedTagged,
    UncompressedTaggedPointer,
    UncompressedTaggedSigned,
    ProtectedPointer,
    SandboxedPointer,
    Float16,
    Float16RawBits,
    Bit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct LoadRepresentation(MachineRepresentation, bool);

impl LoadRepresentation {
    fn representation(&self) -> MachineRepresentation {
        self.0
    }

    fn is_signed(&self) -> bool {
        self.1
    }

    fn to_machine_type(&self) -> MachineRepresentation {
        self.0
    }

    fn is_protected(&self, _traps_on_null: &mut bool) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct StoreRepresentation(MachineRepresentation);

impl StoreRepresentation {
    fn representation(&self) -> MachineRepresentation {
        self.0
    }

    fn write_barrier_kind(&self) -> WriteBarrierKind {
        WriteBarrierKind::kNoWriteBarrier
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum WriteBarrierKind {
    kNoWriteBarrier,
    kFullWriteBarrier,
    kIndirectPointerWriteBarrier,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum AtomicMemoryOrder {
    kRelaxed,
    kAcquire,
    kRelease,
    kAcquireRelease,
    kSeqCst,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum MemoryAccessKind {
    kDirect,
    kProtectedByTrapHandler,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum IndirectPointerTag {
    Address,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct RegisterUseKind {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct InstructionOperand {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct ImmediateOperand {}

impl ImmediateOperand {
    const INLINE_INT32: i32 = 0;
    fn new(_type: i32, _value: i32) -> Self {
        ImmediateOperand {}
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct LoadStoreSimplificationConfiguration {}

impl LoadStoreSimplificationConfiguration {
    const kMinOffset: i64 = i32::MIN as i64 + 1;
    const kMaxOffset: i64 = i32::MAX as i64;
}

trait InstructionSelector {
    fn Get(&self, node: OpIndex) -> OperationView;
    fn is_load(&self, node: OpIndex) -> bool;
    fn IsPhi(&self, node: OpIndex) -> bool;
    fn phi_representation_of(&self, node: OpIndex) -> MachineRepresentation;
    fn is_store(&self, _node: OpIndex) -> bool {
        false
    }
    fn store_view(&self, node: OpIndex) -> StoreView;
    fn load_view(&self, node: OpIndex) -> LoadView;
    fn atomic_rmw_view(&self, node: OpIndex) -> AtomicRmwView;
    fn value_input_count(&self, node: OpIndex) -> usize;
    fn input_at(&self, node: OpIndex, index: usize) -> OpIndex;
    fn turboshaft_graph(&self) -> &TurboshaftGraph;
    fn CanAddressRelativeToRootsRegister(&self, reference: ExternalReference) -> bool;
    fn isolate(&self) -> &Isolate;
    fn frame_allocate_spill_slot(&mut self, size: i32, alignment: i32, is_tagged: bool) -> i32;
    fn Emit(&mut self, code: ArchOpcode, output_count: usize, outputs: *const InstructionOperand, input_count: usize, inputs: *const InstructionOperand);
    fn EmitWithContinuation(&mut self, opcode: ArchOpcode, output_count: usize, outputs: *const InstructionOperand, input_count: usize, inputs: *const InstructionOperand, cont: &FlagsContinuation);
    fn IsCommutative(&self, node: OpIndex) -> bool;
    fn CanCover(&self, node: OpIndex, other: OpIndex) -> bool;
    fn GetEffectLevel(&self, node: OpIndex, cont: &FlagsContinuation) -> i32;
    fn GetEffectLevel_no_cont(&self, node: OpIndex) -> i32;
    fn GetEffectLevel(&self, node: OpIndex) -> i32;
    fn IsReallyLive(&self, node: OpIndex) -> bool;
    fn MatchSignedIntegralConstant(&self, node: OpIndex, out: &mut i64) -> bool;
    fn MatchUnsignedIntegralConstant(&self, node: OpIndex, out: &mut u64) -> bool;
    fn MatchTruncateWord64ToWord32(&self, node: OpIndex, out: &mut OpIndex) -> bool;
    fn is_live(&self, node: OpIndex) -> bool;
    fn zero_extends_word32_to_word64(&self, node: OpIndex) -> bool;
    fn is_supported(&self, avx: AVX) -> bool;
    fn MatchIntegralWord32Constant(&self, node: OpIndex, value: i32) -> bool;
    fn MatchIntegralZero(&self, node: OpIndex) -> bool;
    fn find_projection(&self, node: OpIndex, index: i32) -> Option<OpIndex>;
    fn zero_extends_word32_to_word64_no_phis(&self, node: OpIndex) -> bool;
    fn inputs<T>(&self, node: OpIndex) -> (OpIndex, OpIndex) where T: InstructionSelector;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Constant {
    value: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct StackCheckKind {}

struct TurboshaftAdapter {}

impl TurboshaftAdapter {
  #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
  struct LoadView {
    ts_loaded_rep: MemoryRepresentation,
    ts_result_rep: RegisterRepresentation,
  }

  impl LoadView {
      fn loaded_rep(&self) -> MemoryRepresentation {
          self.ts_loaded_rep
      }
      fn result_rep(&self) -> RegisterRepresentation {
          self.ts_result_rep
      }
      fn is_protected(&self, traps_on_null: &mut bool) -> bool {
        *traps_on_null = false; // Dummy for now
        false
      }
      fn ts_loaded_rep(&self) -> MemoryRepresentation {
        self.ts_loaded_rep
      }
      fn ts_result_rep(&self) -> RegisterRepresentation {
        self.ts_result_rep
      }
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
  struct StoreView {
    ts_stored_rep: MemoryRepresentation
  }

    impl StoreView {
        fn stored_rep(&self) -> StoreRepresentation {
            StoreRepresentation(self.ts_stored_rep)
        }
        fn base(&self) -> OpIndex {
            OpIndex::invalid() // Dummy for now
        }
        fn index(&self) -> OptionalOpIndex {
            OptionalOpIndex(OpIndex::invalid()) // Dummy for now
        }
        fn value(&self) -> OpIndex {
            OpIndex::invalid() // Dummy for now
        }
        fn displacement(&self) -> i32 {
            0 // Dummy for now
        }
        fn element_size_log2(&self) -> u8 {
            0 // Dummy for now
        }
        fn memory_order(&self) -> Option<AtomicMemoryOrder> {
            None // Dummy for now
        }
        fn access_kind(&self) -> MemoryAccessKind {
            MemoryAccessKind::kDirect // Dummy for now
        }
        fn is_store_trap_on_null(&self) -> bool {
            false
        }
        fn indirect_pointer_tag(&self) -> IndirectPointerTag {
            IndirectPointerTag::Address
        }

        fn is_protected(&self, traps_on_null: &mut bool) -> bool {
          *traps_on_null = false; // Dummy for now
          false
        }

      fn ts_stored_rep(&self) -> MemoryRepresentation {
        self.ts_stored_rep
      }
    }
    const AllowsImplicitWord64ToWord32Truncation: bool = true;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct FlagsContinuation {}

impl FlagsContinuation {
    fn ForSet(_overflow: Overflow, _ovf: OpIndex) -> Self {
        FlagsContinuation {}
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct AtomicWidth {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Frame {}

impl Frame {
    fn AllocateSpillSlot(&self, _size: i32, _alignment: i32, _is_tagged: bool) -> i32 {
        0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct OperationView {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct TurboshaftGraph {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Isolate {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct ExternalReference {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct RootsTable {}

impl RootsTable {
    fn IsReadOnly(_root_index: i32) -> bool {
        true
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct HeapObject {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct RootIndex {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Smi {}

impl Smi {
    fn ptr(&self) -> i64 {
        0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Number {
    bits: u64,
}

impl Number {
    fn get_bits(&self) -> u64 {
        self.bits
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Bootstrapper {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct RootRegisterOffsetForExternalReference {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct DisplacementMatchResult {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum VectorLength {
    kV128,
    kV256,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct MiscField {}

impl MiscField {
  fn encode(_kind: i32) -> Self {
    MiscField {}
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct AddressingModeField {}

impl AddressingModeField {
  fn encode(_kind: AddressingMode) -> Self {
    AddressingModeField {}
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct AtomicWidthField {}

impl AtomicWidthField {
  fn encode(_kind: AtomicWidth) -> Self {
    AtomicWidthField {}
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct VectorLengthField {}

impl VectorLengthField {
    fn encode(_len: VectorLength) -> Self {
        VectorLengthField {}
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct RecordWriteMode {}

impl RecordWriteMode {
    fn encode(_mode: RecordWriteMode) -> Self {
        RecordWriteMode {}
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct AccessModeField {}

impl AccessModeField {
    fn encode(_mode: MemoryAccessMode) -> Self {
        AccessModeField {}
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum MemoryAccessMode {
    kMemoryAccessDirect,
    kMemoryAccessProtectedNullDereference,
    kMemoryAccessProtectedMemOutOfBounds,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct AccessMode {}

impl AccessMode {
    fn encode(_mode: AccessMode) -> Self {
        AccessMode {}
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct StackSlotOp {}

impl StackSlotOp {
  fn size(&self) -> i32 {
    0
  }
  fn alignment(&self) -> i32 {
    0
  }
  fn is_tagged(&self) -> bool {
    false
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct StoreOp {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct LoadOp {}

impl LoadOp {
  fn base(&self) -> OpIndex {
    OpIndex::invalid()
  }

  fn index(&self) -> Option<OpIndex> {
    None
  }

  fn offset(&self) -> i32 {
    0
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct WordBinopOp {}

impl WordBinopOp {
  fn left(&self) -> OpIndex {
    OpIndex::invalid()
  }

  fn right(&self) -> OpIndex {
    OpIndex::invalid()
  }

  fn IsCommutative(&self) -> bool {
    false
  }

  fn kind(&self) -> WordBinopOpKind {
    WordBinopOpKind::kAdd
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum WordBinopOpKind {
    kAdd,
    kSub,
    kMul,
    kDiv,
    kMod,
    kBitwiseAnd,
    kBitwiseOr,
    kBitwiseXor,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct ShiftOp {}

impl ShiftOp {
  fn left(&self) -> OpIndex {
    OpIndex::invalid()
  }
  fn right(&self) -> OpIndex {
    OpIndex::invalid()
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct LoadRootRegisterOp {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct ConstantOp {
    kind: ConstantOpKind,
    word32: i32,
    word64: i64,
    smi: Smi,
    number: Number,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum ConstantOpKind {
    kWord32,
    kWord64,
    kSmi,
    kNumber,
    kCompressedHeapObject
}

impl ConstantOp {
    fn word32(&self) -> i32 {
        self.word32
    }

    fn word64(&self) -> i64 {
        self.word64
    }

    fn smi(&self) -> Smi {
        self.smi
    }

    fn number(&self) -> Number {
        self.number
    }

    fn handle(&self) -> HeapObject {
        HeapObject {}
    }

    fn integral(&self) -> u64 {
        0 // Dummy for now
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct OverflowCheckedBinopOp {}

impl OverflowCheckedBinopOp {
    fn kind(&self) -> OverflowCheckedBinopOpKind {
        OverflowCheckedBinopOpKind::kSignedAdd
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum OverflowCheckedBinopOpKind {
  kSignedAdd,
  kSignedSub,
  kSignedMul,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct ProjectionOp {}

impl ProjectionOp {
  fn input(&self) -> OpIndex {
    OpIndex::invalid()
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct CompareOp {}

impl CompareOp {
  fn kind(&self) -> ComparisonOpKind {
    ComparisonOpKind::kEqual
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct ComparisonOp {}

impl ComparisonOp {
  fn kind(&self) -> ComparisonOpKind {
    ComparisonOpKind::kEqual
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum ComparisonOpKind {
    kEqual,
    kSignedLessThan,
    kSignedLessThanOrEqual,
    kUnsignedLessThan,
    kUnsignedLessThanOrEqual,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct StackPointerGreaterThanOp {}

impl StackPointerGreaterThanOp {
  fn kind(&self) -> StackCheckKind {
    StackCheckKind {}
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Simd128LaneMemoryOp {}

impl Simd128LaneMemoryOp {
    fn value(&self) -> OpIndex {
        OpIndex::invalid()
    }
    fn lane(&self) -> i32 {
        0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Simd128LoadTransformOp {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Simd256LoadTransformOp {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Simd256ConstantOp {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Simd256ShufdOp {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Simd256ShufpsOp {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Simd256UnpackOp {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct SimdPack128To256Op {}

impl SimdPack128To256Op {
  fn input(&self, _index: i32) -> OpIndex {
    OpIndex::invalid()
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct AtomicRmwView {}

impl AtomicRmwView {
    fn value(&self) -> OpIndex {
        OpIndex::invalid()
    }

    fn base(&self) -> OpIndex {
        OpIndex::invalid()
    }

    fn index(&self) -> OpIndex {
        OpIndex::invalid()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct VectorLengthFieldWrapper {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct InstructionCode {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum OperandGeneratorImmediateMode {
  kUseImmediate,
  kUseAny,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum AVX {
    AVX,
    NO_AVX
}

const COMPRESS_POINTERS_BOOL: bool = false; // Dummy for now
const V8_STATIC_ROOTS_BOOL: bool = false;
const V8_ENABLE_SANDBOX_BOOL: bool = false;
const V8_COMPRESS_POINTERS: bool = false;
const V8_IS_TSAN: bool = false;
const kHeapObjectTag: i32 = 0;
const kSystemPointerSizeLog2: i32 = 3;
const rax: i32 = 0;
const rdx: i32 = 1;
const rcx: i32 = 2;

struct Flags {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Overflow {
    kOverflow,
}

fn Is64() -> bool {
    true
}

fn IsAnyTagged(_rep: MachineRepresentation) -> bool {
  false
}

fn IsAnyCompressed(_rep: MachineRepresentation) -> bool {
  false
}

fn CanBeTaggedOrCompressedPointer(_rep: MachineRepresentation) -> bool {
  false
}

fn CanBeTaggedOrCompressedOrIndirectPointer(_rep: MachineRepresentation) -> bool {
  false
}

fn ElementSizeLog2Of(_rep: MachineRepresentation) -> i32 {
  0
}

fn WriteBarrierKindToRecordWriteMode(_kind: WriteBarrierKind) -> RecordWriteMode {
    RecordWriteMode {}
}

macro_rules! V {
  ($t:ident) => {
      struct $t {
          op_index: OpIndex,
      }
  };
}

V!(Word32);
V!(Word64);

mod v8_flags {
  pub static enable_unconditional_write_barriers: bool = false;
  pub static disable_write_barriers: bool = false;
}

fn is_int32(value: i64) -> bool {
    (i32::MIN as i64 <= value) && (value <= i32::MAX as i64)
}

fn MatchExternalConstant(_base: OpIndex, _reference: &mut ExternalReference) -> bool {
    false
}

fn RootsTableIsReadOnly(_root_index: i32) -> bool {
    true
}

fn SmiValuesAre31Bits() -> bool {
    true
}

fn MatchWordBinop<T>(_node: OpIndex, _left: &mut T, _const_value: i32) -> bool {
    false
}

fn MatchIntegralZero(_node: OpIndex) -> bool {
    false
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct AddressingModeFieldType {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum TurboshaftOpCode {
    kStackSlot,
    kWord32And
}

pub struct