#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::{
    any::Any,
    convert::TryInto,
    fmt,
    mem::{self, MaybeUninit},
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Shl, Shr, Sub},
    ptr,
    rc::Rc,
};

//use bitflags::bitflags; // Consider using for bit field manipulation

// Placeholder for v8::internal namespace.  Adjust as needed.
mod internal {
    // Placeholder for v8::internal::compiler namespace.
    pub mod compiler {
        // Placeholder for turboshaft namespace.
        pub mod turboshaft {
            pub type OpIndex = usize; // Define OpIndex as usize

            pub mod opmasks {
                pub trait Operation {}
                pub trait ChangeOp: Operation {
                    fn input(&self) -> super::OpIndex;
                }
                pub trait ConstantOp: Operation {
                    fn is_integral(&self) -> bool;
                    fn integral(&self) -> i64;
                }
                
                pub trait LoadOp: Operation {
                    fn base(&self) -> super::OpIndex;
                    fn index(&self) -> Option<super::OpIndex>;
                }
                pub trait ShiftOp: Operation {
                    fn left(&self) -> super::OpIndex;
                    fn right(&self) -> super::OpIndex;
                }

                pub trait OverflowCheckedBinopOp: Operation {}
                
                // Trait for external constant operation
                pub trait ExternalConstantOp : Operation {
                    fn external_reference(&self) -> usize;  // Placeholder for ExternalReference
                }

                pub trait Tagged {}

                pub trait Word32Constant : ConstantOp {}

                pub trait ChangeInt32ToInt64: ChangeOp {}
                pub trait ChangeUint32ToUint64: ChangeOp {}

                pub trait Word64ShiftRightArithmetic: Operation {}

                pub trait Word32BitwiseAnd: Operation {}
                pub trait Word64BitwiseAnd: Operation {}

                pub trait TruncateFloat32ToInt32OverflowToMin: Operation {}
                pub trait TruncateFloat32ToUint32OverflowToMin: Operation {}
                pub trait TruncateFloat64ToInt64OverflowToMin: Operation {}
            }

            #[derive(Debug, Clone, Copy)]
            pub enum ShiftOpKind {
                kShiftRightArithmetic,
                kShiftRightArithmeticShiftOutZeros,
            }

            #[derive(Debug, Clone)]
            pub struct ShiftOp {
                pub kind: ShiftOpKind,
                pub left: OpIndex,
                pub right: OpIndex,
            }

            impl ShiftOp {
                pub fn new(kind: ShiftOpKind, left: OpIndex, right: OpIndex) -> Self {
                    ShiftOp { kind, left, right }
                }
            }

            #[derive(Debug, Clone, Copy)]
            pub enum ConstantOpKind {
                kCompressedHeapObject,
                kFloat32,
                kFloat64,
                kIntegral,
            }

            #[derive(Debug, Clone)]
            pub struct ConstantOp {
                pub kind: ConstantOpKind,
                pub integral_value: i64, // Assuming signed integral
                pub float32_value: f32,
                pub float64_value: f64,
                pub handle_value: usize, //Placeholder type for Handle<HeapObject>
            }

            impl ConstantOp {
                 pub fn new_integral(value: i64) -> Self {
                    ConstantOp {
                        kind: ConstantOpKind::kIntegral,
                        integral_value: value,
                        float32_value: 0.0,
                        float64_value: 0.0,
                        handle_value: 0,
                    }
                }
            
                 pub fn new_float32(value: f32) -> Self {
                    ConstantOp {
                        kind: ConstantOpKind::kFloat32,
                        integral_value: 0,
                        float32_value: value,
                        float64_value: 0.0,
                        handle_value: 0,
                    }
                }
            
                 pub fn new_float64(value: f64) -> Self {
                    ConstantOp {
                        kind: ConstantOpKind::kFloat64,
                        integral_value: 0,
                        float32_value: 0.0,
                        float64_value: value,
                        handle_value: 0,
                    }
                }

                pub fn signed_integral(&self) -> i64 {
                    self.integral_value
                }

                 pub fn float32(&self) -> f32 {
                    self.float32_value
                }
            
                pub fn float64(&self) -> f64 {
                    self.float64_value
                }

                pub fn is_integral(&self) -> bool {
                    match self.kind {
                        ConstantOpKind::kIntegral => true,
                        _ => false,
                    }
                }

                pub fn integral(&self) -> i64 {
                    self.integral_value
                }

                pub fn handle(&self) -> usize {
                    self.handle_value
                }
            }

             #[derive(Debug, Clone)]
            pub struct LoadOp {
                pub base: OpIndex,
                pub index: Option<OpIndex>,
                pub offset: i32,
                pub element_size_log2: u8,
            }

            impl LoadOp {
                pub fn new(base: OpIndex, index: Option<OpIndex>, offset: i32, element_size_log2: u8) -> Self {
                    LoadOp { base, index, offset, element_size_log2 }
                }

                pub fn base(&self) -> OpIndex {
                    self.base
                }
            }

            #[derive(Debug, Clone)]
            pub struct Operation {
                pub opcode: Opcode,
            }

            impl Operation {
                pub fn is<T: 'static>(&self) -> bool {
                    false
                }
                pub fn input(&self, index: usize) -> OpIndex {
                    0
                }

                pub fn cast<T: 'static>(&self) -> &T {
                    panic!("Failed to cast Operation to specific type");
                }

                pub fn try_cast<T: 'static>(&self) -> Option<&T> {
                    None
                }
            }

            #[derive(Debug, Clone)]
            pub struct StackSlotOp {
                pub size: i32,
                pub alignment: i32,
                pub is_tagged: bool,
            }

            #[derive(Debug, Clone, Copy)]
            pub enum Opcode {
                kLoad,
                kStackSlot,
                kComparison,
                kOverflowCheckedBinop,
                kProjection,
                kWord32BitwiseAnd,
                kWord64BitwiseAnd,
                kExternalConstant,
                kLoadRootRegister,
                kChangeInt32ToInt64,
                kWord64ShiftRightArithmetic,
                kStackPointerGreaterThan,
                kBitcastWord32PairToFloat64,
            }

            impl Operation {
                pub fn Is<T: 'static>(&self) -> bool {
                    // This is a placeholder implementation. In a real implementation,
                    // you would use dynamic casting or a similar mechanism to check the type.
                    false
                }

                pub fn IsLoadRootRegisterOp(&self) -> bool {
                    self.opcode == Opcode::kLoadRootRegister
                }

                pub fn IsExternalConstant(&self) -> bool {
                    self.opcode == Opcode::kExternalConstant
                }
            }

            #[derive(Debug, Clone)]
            pub struct ComparisonOp {
                pub rep: RegisterRepresentation,
                pub kind: ComparisonOpKind,
                pub left: OpIndex,
                pub right: OpIndex,
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum ComparisonOpKind {
                kEqual,
                kSignedLessThan,
                kSignedLessThanOrEqual,
            }

            impl ComparisonOp {
                pub fn new(rep: RegisterRepresentation, kind: ComparisonOpKind, left: OpIndex, right: OpIndex) -> Self {
                    ComparisonOp { rep, kind, left, right }
                }
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum RegisterRepresentation {
                Word32,
                Word64,
                Float32,
                Float64,
                Compressed,
                Tagged,
            }

            #[derive(Debug, Clone)]
            pub struct OverflowCheckedBinopOp {
                pub rep: WordRepresentation,
                pub kind: OverflowCheckedBinopOpKind,
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum OverflowCheckedBinopOpKind {
                kSignedAdd,
                kSignedSub,
                kSignedMul,
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum WordRepresentation {
                Word32,
                Word64,
            }

             #[derive(Debug, Clone)]
            pub struct ProjectionOp {
                pub index: u32,
                pub input: OpIndex,
            }

            impl ProjectionOp {
                pub fn new(index: u32, input: OpIndex) -> Self {
                    ProjectionOp { index, input }
                }
            }

            #[derive(Debug, Clone)]
            pub struct LoadRootRegisterOp {}

            impl LoadRootRegisterOp {
                pub fn new() -> Self {
                    LoadRootRegisterOp {}
                }
            }
        }

        // Placeholder for MachineType enum.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum MachineType {
            Int32,
            Int64,
            Float32,
            Float64,
            Simd128,
        }

        // Placeholder for RegisterRepresentation enum.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum RegisterRepresentation {
            Word32,
            Word64,
            Float32,
            Float64,
            Compressed,
            Tagged,
        }

        // Placeholder for MemoryRepresentation enum.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum MemoryRepresentation {
            Int8,
            Uint8,
            Int16,
            Uint16,
            Int32,
            Uint32,
            Int64,
            Uint64,
            Float16,
            Float32,
            Float64,
            AnyTagged,
            TaggedPointer,
            TaggedSigned,
            AnyUncompressedTagged,
            UncompressedTaggedPointer,
            UncompressedTaggedSigned,
            ProtectedPointer,
            IndirectPointer,
            SandboxedPointer,
            Simd128,
            Simd256,
            Compressed, // Added Compressed Representation
        }

        impl MemoryRepresentation {
            pub fn write_barrier_kind(&self) -> WriteBarrierKind {
                match self {
                    MemoryRepresentation::AnyTagged
                    | MemoryRepresentation::TaggedPointer
                    | MemoryRepresentation::TaggedSigned => WriteBarrierKind::kFullWriteBarrier, // Or appropriate barrier
                    _ => WriteBarrierKind::kNoWriteBarrier,
                }
            }

            pub fn representation(&self) -> MachineRepresentation {
                match self {
                    MemoryRepresentation::Int8 | MemoryRepresentation::Uint8 => MachineRepresentation::kWord8,
                    MemoryRepresentation::Int16 | MemoryRepresentation::Uint16 => MachineRepresentation::kWord16,
                    MemoryRepresentation::Int32 | MemoryRepresentation::Uint32 => MachineRepresentation::kWord32,
                    MemoryRepresentation::Int64 | MemoryRepresentation::Uint64 => MachineRepresentation::kWord64,
                    MemoryRepresentation::Float32 => MachineRepresentation::kFloat32,
                    MemoryRepresentation::Float64 => MachineRepresentation::kFloat64,
                    MemoryRepresentation::AnyTagged | MemoryRepresentation::TaggedPointer | MemoryRepresentation::TaggedSigned => MachineRepresentation::kTagged,
                    MemoryRepresentation::AnyUncompressedTagged | MemoryRepresentation::UncompressedTaggedPointer | MemoryRepresentation::UncompressedTaggedSigned => MachineRepresentation::kTagged,
                    MemoryRepresentation::ProtectedPointer => MachineRepresentation::kProtectedPointer,
                    MemoryRepresentation::IndirectPointer => MachineRepresentation::kIndirectPointer,
                    MemoryRepresentation::SandboxedPointer => MachineRepresentation::kSandboxedPointer,
                    MemoryRepresentation::Simd128 => MachineRepresentation::kSimd128,
                    MemoryRepresentation::Simd256 => MachineRepresentation::kSimd256,
                    MemoryRepresentation::Compressed => MachineRepresentation::kCompressed,
                    _ => MachineRepresentation::kNone,
                }
            }

            pub fn IsSigned(&self) -> bool {
                match self {
                    MemoryRepresentation::Int8 | MemoryRepresentation::Int16 | MemoryRepresentation::Int32 | MemoryRepresentation::Int64 | MemoryRepresentation::TaggedSigned => true,
                    _ => false,
                }
            }

            pub fn IsUnsigned(&self) -> bool {
                match self {
                    MemoryRepresentation::Uint8 | MemoryRepresentation::Uint16 | MemoryRepresentation::Uint32 | MemoryRepresentation::Uint64 => true,
                    _ => false,
                }
            }
        }

        // Placeholder for LinkageLocation struct.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct LinkageLocation {}

        impl LinkageLocation {
            pub fn GetLocation(&self) -> i32 {
                0
            }

            pub fn GetSizeInPointers(&self) -> i32 {
                0
            }

            pub fn GetType(&self) -> MachineType {
                MachineType::Int32
            }

            pub fn IsCallerFrameSlot(&self) -> bool {
                false
            }
        }

        // Placeholder for RecordWriteMode enum.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum RecordWriteMode {
            kNoRecordWrite,
            kRecordWriteWithSmiCheck,
            kRecordWrite,
        }

        // Placeholder for WriteBarrierKind enum.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum WriteBarrierKind {
            kNoWriteBarrier,
            kFullWriteBarrier,
            kIndirectPointerWriteBarrier,
        }

        // Placeholder for MachineRepresentation enum.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            kProtectedPointer,
            kIndirectPointer,
            kSandboxedPointer,
            kCompressed, // Added Compressed Representation
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum MemoryAccessKind {
            kNormalMemoryAccess,
            kProtectedByTrapHandler,
        }

        // Placeholder for MemoryOrder enum.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum MemoryOrder {
           kRelaxed,
        }

        //Placeholder for RootsTable
        pub struct RootsTable {}

        impl RootsTable {
            pub fn IsReadOnly(_: usize) -> bool {
                false
            }

            pub fn IsRootHandle(_: usize, _: &mut usize) -> bool {
                false
            }
        }

        //Placeholder for Isolate
        pub struct Isolate {}

        impl Isolate {
            pub fn roots_table(&self) -> RootsTable {
                RootsTable {}
            }

            pub fn bootstrapper(&self) -> bool {
                false
            }
        }

        pub fn WriteBarrierKindToRecordWriteMode(kind: WriteBarrierKind) -> RecordWriteMode {
            match kind {
                WriteBarrierKind::kNoWriteBarrier => RecordWriteMode::kNoRecordWrite,
                WriteBarrierKind::kFullWriteBarrier => RecordWriteMode::kRecordWrite,
                _ => RecordWriteMode::kNoRecordWrite, // Adjust as needed
            }
        }

        // Placeholder for CallDescriptor struct
        pub struct CallDescriptor {}

        impl CallDescriptor {
            pub fn IsCFunctionCall(&self) -> bool {
                false
            }

            pub fn GPParameterCount(&self) -> usize {
                0
            }

            pub fn FPParameterCount(&self) -> usize {
                0
            }

            pub fn ParameterSlotCount(&self) -> usize {
                0
            }

            pub fn GetOffsetToReturns(&self) -> i32 {
                0
            }
        }

        //Placeholder for Constant
        pub struct Constant(i32);

        impl Constant {
            pub fn new(value: i32) -> Self {
                Constant(value)
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum IndirectPointerTag {}

        // TurboshaftAdapter Struct and Methods
        pub mod adapter {
            use super::{MemoryAccessKind, MemoryOrder, MemoryRepresentation, WriteBarrierKind};

            pub struct StoreView {}
            
            impl StoreView {
                pub fn displacement(&self) -> i32 {
                    0
                }

                pub fn base(&self) -> usize {
                    0
                }

                pub fn index(&self) -> usize {
                    0
                }

                pub fn value(&self) -> usize {
                    0
                }

                pub fn ts_stored_rep(&self) -> StoredRep {
                    StoredRep {}
                }

                pub fn is_store_trap_on_null(&self) -> bool {
                    false
                }

                pub fn access_kind(&self) -> MemoryAccessKind {
                    MemoryAccessKind::kNormalMemoryAccess
                }

                pub fn indirect_pointer_tag(&self) -> usize {
                    0
                }
            }
        
            pub struct StoredRep {}

            impl StoredRep {
                pub fn write_barrier_kind(&self) -> WriteBarrierKind {
                    WriteBarrierKind::kNoWriteBarrier
                }

                pub fn representation(&self) -> MachineRepresentation {
                    MachineRepresentation::kNone
                }
            }

             pub struct LoadView {}

            impl LoadView {
                pub fn ts_loaded_rep(&self) -> LoadedRep {
                    LoadedRep {}
                }

                 pub fn is_protected(&self, _: &mut bool) -> bool {
                    false
                 }

                 pub fn loaded_rep(&self) -> LoadedRep {
                    LoadedRep {}
                }
            }

            pub struct LoadedRep {}

            impl LoadedRep {
                pub fn IsUnsigned(&self) -> bool {
                    false
                }

                pub fn representation(&self) -> MachineRepresentation {
                    MachineRepresentation::kNone
                }
            }
        }
    }
}

// Globals Declaration
const COMPRESS_POINTERS_BOOL: bool = false;
const V8_STATIC_ROOTS_BOOL: bool = false;
const V8_ENABLE_SANDBOX_BOOL: bool = false;
const kTaggedSize: i32 = 8;
const kSystemPointerSizeLog2: i32 = 3;

//Implement Macros

macro_rules! DCHECK {
    ($condition:expr) => {
        if !$condition {
            panic!("DCHECK failed: {}", stringify!($condition));
        }
    };
    ($condition:expr, $($arg:tt)*) => {
        if !$condition {
            panic!("DCHECK failed: {}: {}", stringify!($condition), format_args!($($arg)*));
        }
    };
}

macro_rules! UNIMPLEMENTED {
    () => {
        panic!("UNIMPLEMENTED");
    };
    ($($arg:tt)*) => {
        panic!("UNIMPLEMENTED: {}", format_args!($($arg)*));
    };
}

macro_rules! UNREACHABLE {
    () => {
        panic!("UNREACHABLE");
    };
}

macro_rules! arraysize {
    ($array:expr) => {
        ($array).len()
    };
}

// Flags Struct
struct Flags {
    enable_unconditional_write_barriers: bool,
    disable_write_barriers: bool,
}

// Globals declaration for Flags
static v8_flags: Flags = Flags {
    enable_unconditional_write_barriers: false,
    disable_write_barriers: false,
};

// ArchOpcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchOpcode {
    kLoong64Cmp32,
    kLoong64Cmp64,
    kLoong64Sll_w,
    kLoong64Srl_w,
    kLoong64Sra_w,
    kLoong64Sll_d,
    kLoong64Srl_d,
    kLoong64Sra_d,
    kLoong64And,
    kLoong64And32,
    kLoong64Or,
    kLoong64Or32,
    kLoong64Xor,
    kLoong64Xor32,
    kLoong64Tst,
    kLoong64Ld_w,
    kLoong64St_w,
    kLoong64Ld_d,
    kLoong64St_d,
    kAtomicLoadWord32,
    kAtomicStoreWord32,
    kLoong64Word64AtomicLoadUint64,
    kLoong64Word64AtomicStoreWord64,
    kLoong64StoreCompressTagged,
    kArchNop,
    kArchStackSlot,
    kArchAbortCSADcheck,
    kLoong64Ld_b,
    kLoong64Ld_bu,
    kLoong64Ld_h,
    kLoong64Ld_hu,
    kLoong64Fld_s,
    kLoong64Fld_d,
    kLoong64LoadDecompressTagged,
    kLoong64LoadDecompressTaggedSigned,
    kLoong64Ld_wu,
    kLoong64LoadDecompressProtected,
    kLoong64LoadDecodeSandboxedPointer,
    kLoong64St_b,
    kLoong64St_h,
    kLoong64Fst_s,
    kLoong64Fst_d,
    kLoong64StoreIndirectPointer,
    kLoong64StoreEncodeSandboxedPointer,
    kLoong64ByteSwap32,
    kLoong64ByteSwap64,
    kLoong64Clz_w,
    kLoong64Clz_d,
    kLoong64Add_w,
    kLoong64Add_d,
    kLoong64Sub_w,
    kLoong64Sub_d,
    kLoong64Mul_w,
    kLoong64Mul_d,
    kLoong64Mulh_w,
    kLoong64Mulh_d,
    kLoong64Mulh_wu,
    kLoong64Mulh_du,
    kLoong64Div_w,
    kLoong64Div_wu,
    kLoong64Mod_w,
    kLoong64Mod_wu,
    kLoong64Div_d,
    kLoong64Div_du,
    kLoong64Mod_d,
    kLoong64Mod_du,
    kLoong64Float32ToFloat64,
    kLoong64Int32ToFloat32,
    kLoong64Uint32ToFloat32,
    kLoong64Int32ToFloat64,
    kLoong64Int64ToFloat64,
    kLoong64Uint32ToFloat64,
    kLoong64Float32ToInt32,
    kLoong64Float32ToUint32,
    kLoong64Float64ToInt32,
    kLoong64Float64ToInt64,
    kLoong64Float64ToUint32,
    kLoong64Float64ToUint64,
    kLoong64Float64ToFloat32,
    kLoong64Int64ToFloat32,
    kLoong64Int64ToFloat64,
    kLoong64Uint64ToFloat32,
    kLoong64Uint64ToFloat64,
    kLoong64Float64ExtractLowWord32,
    kLoong64Float64ExtractHighWord32,
    kLoong64Float64FromWord32Pair,
    kLoong64Float64SilenceNaN,
    kArchTruncateDoubleToI,
    kLoong64Float64ToInt32,
    kLoong64Bstrpick_d,
    kLoong64Sra_w,
    kLoong64Sra_d,
    kLoong64Rotr_w,
    kLoong64Rotr_d,
    kLoong64AddOvf_d,
    kLoong64SubOvf_d,
    kLoong64MulOvf_w,
    kLoong64MulOvf_d,
    kLoong64Sll_w,
    kLoong64Sll_d,
    kLoong64Ld_wu,
    kLoong64Sra_d2,
    kLoong64BitcastDL,
    kLoong64BitcastLD,
    kLoong64Float32Add,
    kLoong64Float64Add,
    kLoong64Float32Sub,
    kLoong64Float64Sub,
    kLoong64Float32Mul,
    kLoong64Float64Mul,
    kLoong64Float32Div,
    kLoong64Float64Div,
    kLoong64Float64Mod,
    kLoong64Float32Max,
    kLoong64Float64Max,
    kLoong64Float32Min,
    kLoong64Float64Min,
    kLoong64Float32Abs,
    kLoong64Float64Abs,
    kLoong64Float32Sqrt,
    kLoong64Float64Sqrt,
    kLoong64Float32RoundDown,
    kLoong64Float64RoundDown,
    kLoong64Float32RoundUp,
    kLoong64Float64RoundUp,
    kLoong64Float32RoundTruncate,
    kLoong64Float64RoundTruncate,
    kLoong64Float32RoundTiesEven,
    kLoong64Float64RoundTiesEven,
    kLoong64Float32Neg,
    kLoong64Float64Neg,
    kLoong64Dbar,
    kArchStackPointerGreaterThan,
    kArchPrepareCallCFunction,
    kLoong64Poke,
    kLoong64StackClaim,
    kLoong64Peek,
    kLoong64AtomicExchangeInt8,
    kLoong64AtomicExchangeUint8,
    kLoong64AtomicExchangeInt16,
    kLoong64AtomicExchangeUint16,
    kLoong64AtomicExchangeWord32,
    kLoong64Word64AtomicExchangeUint64,
    kLoong64AtomicCompareExchangeInt8,
    kLoong64AtomicCompareExchangeUint8,
    kLoong64AtomicCompareExchangeInt16,
    kLoong64AtomicCompareExchangeUint16,
    kLoong64AtomicCompareExchangeWord32,
    kLoong64Word64AtomicCompareExchangeUint64,
    kArchAtomicStoreWithWriteBarrier,
    kLoong64AtomicStoreCompressTagged,
    kAtomicExchangeInt8,
    kAtomicExchangeUint8,
    kAtomicExchangeInt16,
    kAtomicExchangeUint16,
    kAtomicExchangeWord32,
    kAtomicCompareExchangeInt8,
    kAtomicCompareExchangeUint8,
    kAtomicCompareExchangeInt16,
    kAtomicCompareExchangeUint16,
    kAtomicCompareExchangeWord32,
    kAtomicAddInt8,
    kAtomicAddUint8,
    kAtomicAddInt16,
    kAtomicAddUint16,
    kAtomicAddWord32,
    kAtomicSubInt8,
    kAtomicSubUint8,
    kAtomicSubInt16,
    kAtomicSubUint16,
    kAtomicSubWord32,
    kAtomicAndInt8,
    kAtomicAndUint8,
    kAtomicAndInt16,
    kAtomicAndUint16,
    kAtomicAndWord32,
    kAtomicOrInt8,
    kAtomicOrUint8,
    kAtomicOrInt16,
    kAtomicOrUint16,
    kAtomicOrWord32,
    kAtomicXorInt8,
    kAtomicXorUint8,
    kAtomicXorInt16,
    kAtomicXorUint16,
    kAtomicXorWord32,
    kLoong64Word64AtomicAddUint64,
    kLoong64Word64AtomicSubUint64,
    kLoong64Word64AtomicAndUint64,
    kLoong64Word64AtomicOrUint64,
    kLoong64Word64AtomicXorUint64,
    kLoong64Daddi,
    kLoong64Float32Cmp,
    kLoong64Float64Cmp,
    kLoong64F64x2Abs,
    kLoong64F64x2Neg,
    kLoong64F64x2Sqrt,
    kLoong64F64x2Ceil,
    kLoong64F64x2Floor,
    kLoong64F64x2Trunc,
    kLoong64F64x2NearestInt,
    kLoong64I64x2Neg,
    kLoong64I64x2BitMask,
    kLoong64F64x2ConvertLowI32x4S,
    kLoong64F64x2ConvertLowI32x4U,
    kLoong64F64x2PromoteLowF32x4,
    kLoong64F32x4SConvertI32x4,
    kLoong64F32x4UConvertI32x4,
    kLoong64F32x4Abs,
    kLoong64F32x4Neg,
    kLoong64F32x4Sqrt,
    kLoong64F32x4Ceil,
    kLoong64F32x4Floor,
    kLoong64F32x4Trunc,
    kLoong64F32x4NearestInt,
    kLoong64F32x4DemoteF64x2Zero,
    kLoong64I64x2Abs,
    kLoong64I64x2SConvertI32x4Low,
    kLoong64I64x2SConvertI32x4High,
    kLoong64I64x2UConvertI32x4Low,
    kLoong64I64x2UConvertI32x4High,
    kLoong64I32x4SConvertF32x4,
    kLoong64I32x4UConvertF32x4,
    kLoong64I32x4Neg,
    kLoong64I32x4SConvertI16x8Low,
    kLoong64I32x4SConvertI16x8High,
    kLoong64I32x4UConvertI16x8Low,
    kLoong64I32x4UConvertI16x8High,
    kLoong64I32x4Abs,
    kLoong64I32x4BitMask,
    kLoong64I32x4TruncSatF64x2SZero,
    kLoong64I32x4TruncSatF64x2UZero,
    kLoong64I32x4RelaxedTruncF32x4S,
    kLoong64I32x4RelaxedTruncF32x4U,
    kLoong64I32x4RelaxedTruncF64x2SZero,
    kLoong64I32x4RelaxedTruncF64x2UZero,
    kLoong64I16x8Neg,
    kLoong64I16x8SConvertI8x16Low,
    kLoong64I16x8SConvertI8x16High,
    kLoong64I16x8UConvertI8x16Low,
    kLoong64I16x8UConvertI8x16High,
    kLoong64I16x8Abs,
    kLoong64I16x8BitMask,
    kLoong64I8x16Neg,
    kLoong64I8x16Abs,
    kLoong64I8x16Popcnt,
    kLoong64I8x16BitMask,
    kLoong64S128Not,
    kLoong64I64x2AllTrue,
    kLoong64I32x4AllTrue,
    kLoong64I16x8AllTrue,
    kLoong64I8x16AllTrue,
    kLoong64V128AnyTrue,
    kLoong64I64x2Shl,
    kLoong64I64x2ShrS,
