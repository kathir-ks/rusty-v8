#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use std::{
    convert::TryInto,
    fmt,
    marker::PhantomData,
    mem,
    num::TryFromIntError,
    ops::{Add, BitAnd, Mul, Neg, Not, Sub},
    ptr,
    rc::Rc,
};

// use bitflags::bitflags; // Consider using this for bit flag enums if needed
// use thiserror::Error; // Consider using this for custom error enums

// Placeholder definitions - replace with actual crate usages or custom implementations
// For example:
// use crate::base::bits;
// use crate::codegen::cpu_features;

// Dummy implementations for now. Replace with actual implementations.
mod base {
    pub mod bits {
        pub fn WraparoundNeg32(x: i32) -> i32 {
            x.wrapping_neg()
        }

        pub fn WraparoundAdd32(x: i32, y: i32) -> i32 {
            x.wrapping_add(y)
        }
    }
}

mod codegen {
    pub mod cpu_features {
        pub fn IsSupported(feature: CpuFeature) -> bool {
            false // Placeholder
        }
    }

    pub mod ia32 {
        pub mod assembler_ia32 {
            // Implement AssemblerIa32 and related structs/enums
        }
        pub mod register_ia32 {
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub enum Register {
                eax,
                ecx,
                edx,
                ebx,
                esp,
                ebp,
                esi,
                edi,
            }
        }
    }

    pub mod machine_type {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum MachineRepresentation {
            kFloat32,
            kFloat64,
            kBit,
            kWord8,
            kWord16,
            kTaggedSigned,
            kTaggedPointer,
            kTagged,
            kWord32,
            kSimd128,
            kFloat16,
            kSimd256,
            kCompressedPointer,
            kCompressed,
            kProtectedPointer,
            kIndirectPointer,
            kSandboxedPointer,
            kWord64,
            kMapWord,
            kFloat16RawBits,
            kNone,
        }

        impl MachineRepresentation {
            pub fn representation(&self) -> MachineRepresentation {
                *self
            }

            pub fn is_signed(&self) -> bool {
                match self {
                    MachineRepresentation::kWord8 | MachineRepresentation::kWord16 => true,
                    _ => false,
                }
            }
        }

        impl From<LoadRepresentation> for MachineRepresentation {
            fn from(lr: LoadRepresentation) -> Self {
                lr.representation()
            }
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum MachineSemantic {
            kInt32,
            kUint32,
        }
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct MachineType(MachineRepresentation, MachineSemantic);

        impl MachineType {
            pub fn Int8() -> Self {
                MachineType(MachineRepresentation::kWord8, MachineSemantic::kInt32)
            }
            pub fn Uint8() -> Self {
                MachineType(MachineRepresentation::kWord8, MachineSemantic::kUint32)
            }
            pub fn Int16() -> Self {
                MachineType(MachineRepresentation::kWord16, MachineSemantic::kInt32)
            }
            pub fn Uint16() -> Self {
                MachineType(MachineRepresentation::kWord16, MachineSemantic::kUint32)
            }
            pub fn Int32() -> Self {
                MachineType(MachineRepresentation::kWord32, MachineSemantic::kInt32)
            }
            pub fn Uint32() -> Self {
                MachineType(MachineRepresentation::kWord32, MachineSemantic::kUint32)
            }
            pub fn None() -> Self {
                MachineType(MachineRepresentation::kNone, MachineSemantic::kInt32)
            }
            pub fn representation(&self) -> MachineRepresentation {
                self.0
            }

            pub fn semantic(&self) -> MachineSemantic {
                self.1
            }
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct LoadRepresentation(MachineRepresentation, bool);
        impl LoadRepresentation {
            pub fn new(representation: MachineRepresentation, is_signed: bool) -> Self {
                LoadRepresentation(representation, is_signed)
            }
            pub fn representation(&self) -> MachineRepresentation {
                self.0
            }
            pub fn is_signed(&self) -> bool {
                self.1
            }
            pub fn IsSigned(&self) -> bool {
                self.is_signed()
            }
            pub fn loaded_rep(&self) -> LoadRepresentation {
                *self
            }

            pub fn IsMapWord(&self) -> bool {
                false
            }
        }

        impl From<MachineRepresentation> for LoadRepresentation {
            fn from(rep: MachineRepresentation) -> Self {
                LoadRepresentation(rep, false) // Default is_signed to false, adjust as needed.
            }
        }
    }

    pub mod macro_assembler_base {
        pub const kRootRegisterOffsetForExternalReference: i32 = 0;
    }
}

mod common {
    pub mod globals {
        pub const kSystemPointerSize: usize = 4; // Assuming 32-bit architecture
    }
}

mod compiler {
    pub mod backend {
        pub mod instruction_codes {
            // Define instruction codes as constants or an enum
            pub type InstructionCode = u32; // Or use an enum

            pub const kIA32And: InstructionCode = 1;
            pub const kIA32Or: InstructionCode = 2;
            pub const kIA32Xor: InstructionCode = 3;
            pub const kIA32Add: InstructionCode = 4;
            pub const kIA32Sub: InstructionCode = 5;
            pub const kIA32Cmp: InstructionCode = 6;
            pub const kIA32Test: InstructionCode = 7;
            pub const kIA32Cmp16: InstructionCode = 8;
            pub const kIA32Test16: InstructionCode = 9;
            pub const kIA32Cmp8: InstructionCode = 10;
            pub const kIA32Test8: InstructionCode = 11;
            pub const kIA32Movl: InstructionCode = 12;
            pub const kIA32Movss: InstructionCode = 13;
            pub const kIA32Movsd: InstructionCode = 14;
            pub const kIA32Movsxbl: InstructionCode = 15;
            pub const kIA32Movzxbl: InstructionCode = 16;
            pub const kIA32Movsxwl: InstructionCode = 17;
            pub const kIA32Movzxwl: InstructionCode = 18;
            pub const kIA32Movdqu: InstructionCode = 19;
            pub const kIA32Movb: InstructionCode = 20;
            pub const kIA32Movw: InstructionCode = 21;
            pub const kIA32Lea: InstructionCode = 22;
            pub const kIA32Shl: InstructionCode = 23;
            pub const kIA32Shr: InstructionCode = 24;
            pub const kIA32Sar: InstructionCode = 25;
            pub const kIA32Rol: InstructionCode = 26;
            pub const kIA32Ror: InstructionCode = 27;
            pub const kIA32Imul: InstructionCode = 28;
            pub const kIA32ImulHigh: InstructionCode = 29;
            pub const kIA32UmulHigh: InstructionCode = 30;
            pub const kIA32Idiv: InstructionCode = 31;
            pub const kIA32Udiv: InstructionCode = 32;
            pub const kIA32Float32Cmp: InstructionCode = 33;
            pub const kIA32Float64Cmp: InstructionCode = 34;
            pub const kArchStackSlot: InstructionCode = 35;
            pub const kArchAbortCSADcheck: InstructionCode = 36;
            pub const kIA32Not: InstructionCode = 37;
            pub const kIA32Neg: InstructionCode = 38;
            pub const kSSEInt32ToFloat64: InstructionCode = 39;
            pub const kIA32Float32ToInt32: InstructionCode = 40;
            pub const kIA32Float64ToFloat32: InstructionCode = 41;
            pub const kIA32BitcastFI: InstructionCode = 42;
            pub const kIA32BitcastIF: InstructionCode = 43;
            pub const kIA32Float64ExtractLowWord32: InstructionCode = 44;
            pub const kIA32Float64ExtractHighWord32: InstructionCode = 45;
            pub const kIA32Float64ToInt32: InstructionCode = 46;
            pub const kIA32Float32ToFloat64: InstructionCode = 47;
            pub const kSSEInt32ToFloat32: InstructionCode = 48;
            pub const kIA32Float64ToUint32: InstructionCode = 49;
            pub const kIA32Float32ToUint32: InstructionCode = 50;
            pub const kIA32Uint32ToFloat64: InstructionCode = 51;
            pub const kIA32Float64Round: InstructionCode = 52;
            pub const kIA32Float32Round: InstructionCode = 53;
            pub const kIA32Lzcnt: InstructionCode = 54;
            pub const kIA32Tzcnt: InstructionCode = 55;
            pub const kIA32Popcnt: InstructionCode = 56;
            pub const kIA32Float32Sqrt: InstructionCode = 57;
            pub const kIA32Float64Sqrt: InstructionCode = 58;
            pub const kIA32Float64Mod: InstructionCode = 59;
            pub const kIA32Float32Max: InstructionCode = 60;
            pub const kIA32Float64Max: InstructionCode = 61;
            pub const kIA32Float32Min: InstructionCode = 62;
            pub const kIA32Float64Min: InstructionCode = 63;
            pub const kIA32Bswap: InstructionCode = 64;
            pub const kArchTruncateDoubleToI: InstructionCode = 65;
            pub const kIA32Float64FromWord32Pair: InstructionCode = 66;
            pub const kIA32Push: InstructionCode = 67;
            pub const kIA32Poke: InstructionCode = 68;
            pub const kIA32Peek: InstructionCode = 69;
            pub const kArchPrepareCallCFunction: InstructionCode = 70;

            pub const kIA32MFence: InstructionCode = 71;
            pub const kAtomicExchangeInt8: InstructionCode = 72;
            pub const kAtomicExchangeUint8: InstructionCode = 73;
            pub const kAtomicExchangeInt16: InstructionCode = 74;
            pub const kAtomicExchangeUint16: InstructionCode = 75;
            pub const kAtomicExchangeWord32: InstructionCode = 76;
            pub const kAtomicCompareExchangeInt8: InstructionCode = 77;
            pub const kAtomicCompareExchangeUint8: InstructionCode = 78;
            pub const kAtomicCompareExchangeInt16: InstructionCode = 79;
            pub const kAtomicCompareExchangeUint16: InstructionCode = 80;
            pub const kAtomicCompareExchangeWord32: InstructionCode = 81;
            pub const kAtomicAddInt8: InstructionCode = 82;
            pub const kAtomicAddUint8: InstructionCode = 83;
            pub const kAtomicAddInt16: InstructionCode = 84;
            pub const kAtomicAddUint16: InstructionCode = 85;
            pub const kAtomicAddWord32: InstructionCode = 86;
            pub const kAtomicSubInt8: InstructionCode = 87;
            pub const kAtomicSubUint8: InstructionCode = 88;
            pub const kAtomicSubInt16: InstructionCode = 89;
            pub const kAtomicSubUint16: InstructionCode = 90;
            pub const kAtomicSubWord32: InstructionCode = 91;
            pub const kAtomicAndInt8: InstructionCode = 92;
            pub const kAtomicAndUint8: InstructionCode = 93;
            pub const kAtomicAndInt16: InstructionCode = 94;
            pub const kAtomicAndUint16: InstructionCode = 95;
            pub const kAtomicAndWord32: InstructionCode = 96;
            pub const kAtomicOrInt8: InstructionCode = 97;
            pub const kAtomicOrUint8: InstructionCode = 98;
            pub const kAtomicOrInt16: InstructionCode = 99;
            pub const kAtomicOrUint16: InstructionCode = 100;
            pub const kAtomicOrWord32: InstructionCode = 101;
            pub const kAtomicXorInt8: InstructionCode = 102;
            pub const kAtomicXorUint8: InstructionCode = 103;
            pub const kAtomicXorInt16: InstructionCode = 104;
            pub const kAtomicXorUint16: InstructionCode = 105;
            pub const kAtomicXorWord32: InstructionCode = 106;

            pub const kArchStoreWithWriteBarrier: InstructionCode = 107;
            pub const kArchAtomicStoreWithWriteBarrier: InstructionCode = 108;
            pub const kIA32AddPair: InstructionCode = 109;
            pub const kIA32SubPair: InstructionCode = 110;
            pub const kIA32MulPair: InstructionCode = 111;
            pub const kIA32ShlPair: InstructionCode = 112;
            pub const kIA32ShrPair: InstructionCode = 113;
            pub const kIA32SarPair: InstructionCode = 114;

            pub const kIA32Word32AtomicPairLoad: InstructionCode = 115;
            pub const kIA32Word32ReleasePairStore: InstructionCode = 116;

            // SIMD lane ops
            pub const kIA32Pinsrb: InstructionCode = 117;
            pub const kIA32Pinsrw: InstructionCode = 118;
            pub const kIA32Pinsrd: InstructionCode = 119;
            pub const kIA32Movlps: InstructionCode = 120;
            pub const kIA32Movhps: InstructionCode = 121;
            pub const kIA32Pextrb: InstructionCode = 122;
            pub const kIA32Pextrw: InstructionCode = 123;
            pub const kIA32S128Store32Lane: InstructionCode = 124;

            // SIMD load transform ops
            pub const kIA32S128Load8x8S: InstructionCode = 125;
            pub const kIA32S128Load8x8U: InstructionCode = 126;
            pub const kIA32S128Load16x4S: InstructionCode = 127;
            pub const kIA32S128Load16x4U: InstructionCode = 128;
            pub const kIA32S128Load32x2S: InstructionCode = 129;
            pub const kIA32S128Load32x2U: InstructionCode = 130;
            pub const kIA32S128Load8Splat: InstructionCode = 131;
            pub const kIA32S128Load16Splat: InstructionCode = 132;
            pub const kIA32S128Load32Splat: InstructionCode = 133;
            pub const kIA32S128Load64Splat: InstructionCode = 134;

            // Floating point silence nan op
            pub const kIA32Float64SilenceNaN: InstructionCode = 135;

            pub const kIA32Float32Sqrt: InstructionCode = 136;
            pub const kIA32Float64Sqrt: InstructionCode = 137;

            pub const kIA32Nop: InstructionCode = 138; //added

            pub const kIA32Imul: InstructionCode = 139; //added

            pub const kArchStackPointerGreaterThan: InstructionCode = 140; //added

            pub const kIA32Uint32ToFloat32: InstructionCode = 141; // added
            
            pub const kIA32F64x2Sqrt: InstructionCode = 142; // added

            // SIMD ops
            pub const kIA32F32x4Round: InstructionCode = 143; //added
            pub const kIA32F64x2Round: InstructionCode = 144; //added
        }

        pub mod instruction_selector_adapter {
            // Implement InstructionSelectorAdapter and related traits/structs
        }

        pub mod instruction_selector_impl {
            // Implement InstructionSelectorImpl and related traits/structs
        }

        pub mod instruction_selector {
            // Implement InstructionSelector and related traits/structs
        }

        pub mod instruction {
            // Implement Instruction and related traits/structs
            #[derive(Debug, Clone)]
            pub struct InstructionOperand {}
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum AddressingMode {
                kMode_MRI,
                kMode_MR1I,
                kMode_MR2I,
                kMode_MR4I,
                kMode_MR8I,
                kMode_MRnI,
                kMode_MR,
                kMode_MR1,
                kMode_MR2,
                kMode_MR4,
                kMode_MR8,
                kMode_MnI,
                kMode_M2I,
                kMode_M4I,
                kMode_M8I,
                kMode_MI,
                kMode_M2,
                kMode_M4,
                kMode_M8,
                kMode_Root,
            }
        }
    }

    pub mod frame {
        // Implement Frame and related structs/enums
    }

    pub mod globals {
        // Implement compiler globals
    }

    pub mod linkage {
        // Implement Linkage and related structs/enums
        #[derive(Debug, Clone)]
        pub struct LinkageLocation {}
    }

    pub mod turboshaft {
        pub mod opmasks {
            // Implement opmasks
            pub const kWord32Sub: usize = 1;
            pub const kWord32BitwiseAnd: usize = 2;
        }
    }

    pub mod write_barrier_kind {
        // Implement WriteBarrierKind enum
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum WriteBarrierKind {
            kNoWriteBarrier,
            kFullWriteBarrier,
        }
    }
}

mod flags {
    pub mod flags {
        // Implement flags
        pub struct Flags {}
        impl Flags {
            pub fn new() -> Self {
                Flags {}
            }
        }
        pub static mut v8_flags: Flags = Flags {  };
    }
}

mod utils {
    pub mod utils {
        // Implement utils
    }
}

mod zone {
    pub mod zone_containers {
        // Implement zone containers
    }
}

// #[cfg(V8_ENABLE_WEBASSEMBLY)]
mod wasm {
    pub mod simd_shuffle {
        // Implement simd_shuffle
    }
}

use codegen::ia32::register_ia32::Register;
use codegen::machine_type::MachineRepresentation;
use codegen::machine_type::LoadRepresentation;
use compiler::backend::instruction::AddressingMode;
use flags::flags::v8_flags;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DisplacementMode {
    kPositiveDisplacement,
    kNegativeDisplacement,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct ScaledIndexMatch {
    base: OpIndex,
    index: OpIndex,
    scale: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct BaseWithScaledIndexAndDisplacementMatch {
    base: OpIndex,
    index: OpIndex,
    scale: i32,
    displacement: i32,
    displacement_mode: DisplacementMode,
}

impl Default for BaseWithScaledIndexAndDisplacementMatch {
    fn default() -> Self {
        BaseWithScaledIndexAndDisplacementMatch {
            base: OpIndex::invalid(),
            index: OpIndex::invalid(),
            scale: 0,
            displacement: 0,
            displacement_mode: DisplacementMode::kPositiveDisplacement,
        }
    }
}

type OpIndex = usize; // Placeholder

impl OpIndex {
    pub fn valid(&self) -> bool {
        *self != OpIndex::invalid()
    }
    pub fn invalid() -> OpIndex {
        usize::MAX
    }
}
struct LoadOp {}

impl LoadOp {
    fn base(&self) -> OpIndex {
        0
    }
    fn index(&self) -> Option<OpIndex> {
        None
    }
    fn offset(&self) -> i32 {
        0
    }
}

struct StoreOp {}

impl StoreOp {
    fn base(&self) -> OpIndex {
        0
    }
    fn index(&self) -> Option<OpIndex> {
        None
    }
    fn offset(&self) -> i32 {
        0
    }
}

trait OperationTrait {
    fn is<T>(&self) -> bool;
    fn try_cast<T>() -> Option<&T>;
    fn cast<T>() -> &T;
}

struct Operation {}

impl Operation {
    fn Is<T>(&self) -> bool {
        true
    }
    fn TryCast<T>(&self) -> Option<&T> {
        None
    }

    fn Cast<T>(&self) -> &T {
        panic!("cannot cast");
    }
}

struct LoadStoreView {
    base: OpIndex,
    index: Option<OpIndex>,
    offset: i32,
}

impl LoadStoreView {
    fn new(op: &Operation) -> Self {
        if op.Is::<LoadOp>() {
            let load: &LoadOp = op.Cast::<LoadOp>();
            LoadStoreView {
                base: load.base(),
                index: load.index(),
                offset: load.offset(),
            }
        } else {
            assert!(op.Is::<StoreOp>());
            let store: &StoreOp = op.Cast::<StoreOp>();
            LoadStoreView {
                base: store.base(),
                index: store.index(),
                offset: store.offset(),
            }
        }
    }
}

struct ConstantOp {}

impl ConstantOp {
    fn word32(&self) -> i32 {
        0
    }
}
struct WordBinopOp {}
struct ShiftOp {}
struct FlagsContinuation {}
struct StackSlotOp {}
struct StackPointerGreaterThanOp {}
struct ComparisonOp {}
struct ProjectionOp {}
struct OverflowCheckedBinopOp {}
struct MemoryBarrierOp {}
struct AtomicRMWOp {}
struct PushParameter {}
struct SwitchInfo {}
struct CallDescriptor {}
struct TurboshaftAdapter {}

impl TurboshaftAdapter {
    struct StoreView {}
}

enum CpuFeature {
    AVX,
    INTEL_ATOM
}

enum ArchOpcode {
    kIA32And,
    kIA32Or,
    kIA32Xor,
    kIA32Add,
    kIA32Sub,
    kIA32Cmp,
    kIA32Test,
    kIA32Cmp16,
    kIA32Test16,
    kIA32Cmp8,
    kIA32Test8,
    kIA32Movl,
    kIA32Movss,
    kIA32Movsd,
    kIA32Movsxbl,
    kIA32Movzxbl,
    kIA32Movsxwl,
    kIA32Movzxwl,
    kIA32Movdqu,
    kIA32Movb,
    kIA32Movw,
    kIA32Lea,
    kIA32Shl,
    kIA32Shr,
    kIA32Sar,
    kIA32Rol,
    kIA32Ror,
    kIA32Imul,
    kIA32ImulHigh,
    kIA32UmulHigh,
    kIA32Idiv,
    kIA32Udiv,
    kIA32Float32Cmp,
    kIA32Float64Cmp,
    kArchStackSlot,
    kArchAbortCSADcheck,
    kIA32Not,
    kIA32Neg,
    kSSEInt32ToFloat64,
    kIA32Float32ToInt32,
    kIA32Float64ToFloat32,
    kIA32BitcastFI,
    kIA32BitcastIF,
    kIA32Float64ExtractLowWord32,
    kIA32Float64ExtractHighWord32,
    kIA32Float64ToInt32,
    kIA32Float32ToFloat64,
    kSSEInt32ToFloat32,
    kIA32Float64ToUint32,
    kIA32Float32ToUint32,
    kIA32Uint32ToFloat64,
    kIA32Float64Round,
    kIA32Float32Round,
    kIA32Lzcnt,
    kIA32Tzcnt,
    kIA32Popcnt,
    kIA32Float32Sqrt,
    kIA32Float64Sqrt,
    kIA32Float64Mod,
    kIA32Float32Max,
    kIA32Float64Max,
    kIA32Float32Min,
    kIA32Float64Min,
    kIA32Bswap,
    kArchTruncateDoubleToI,
    kIA32Float64FromWord32Pair,
    kIA32Push,
    kIA32Poke,
    kIA32Peek,
    kArchPrepareCallCFunction,
    kIA32MFence,
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
    kArchStoreWithWriteBarrier,
    kArchAtomicStoreWithWriteBarrier,
    kIA32AddPair,
    kIA32SubPair,
    kIA32MulPair,
    kIA32ShlPair,
    kIA32ShrPair,
    kIA32SarPair,
    kIA32Word32AtomicPairLoad,
    kIA32Word32ReleasePairStore,
    kIA32Pinsrb,
    kIA32Pinsrw,
    kIA32Pinsrd,
    kIA32Movlps,
    kIA32Movhps,
    kIA32Pextrb,
    kIA32Pextrw,
    kIA32S128Store32Lane,
    kIA32S128Load8x8S,
    kIA32S128Load8x8U,
    kIA32S128Load16x4S,
    kIA32S128Load16x4U,
    kIA32S128Load32x2S,
    kIA32S128Load32x2U,
    kIA32S128Load8Splat,
    kIA32S128Load16Splat,
    kIA32S128Load32Splat,
    kIA32S128Load64Splat,
    kIA32Float64SilenceNaN,
    kIA32Nop,
    kIA32Imul2,
    kArchStackPointerGreaterThan,
    kIA32Uint32ToFloat32,
    kIA32F32x4Round,
    kIA32F64x2Round,
    kIA32F64x2Sqrt,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum RegisterMode {
    kRegister,
    kUniqueRegister,
}

struct IA32OperandGeneratorT<'a> {
    selector: &'a mut InstructionSelectorT,
}

impl<'a> IA32OperandGeneratorT<'a> {
    fn new(selector: &'a mut InstructionSelectorT) -> Self {
        IA32OperandGeneratorT { selector }
    }

    fn UseByteRegister(&self, node: OpIndex) -> InstructionOperand {
        self.UseFixed(node, Register::edx)
    }

    fn CanBeMemoryOperand(
        &self,
        opcode: ArchOpcode,
        node: OpIndex,
        input: OpIndex,
        effect_level: i32,
    ) -> bool {
        if !self.IsLoadOrLoadImmutable(input) {
            return false;
        }
        if !self.selector().CanCover(node, input) {
            return false;
        }
        if effect_level != self.selector().GetEffectLevel(input) {
            return false;
        }
        let rep = self.load_view(input).loaded_rep().representation();
        match opcode {
            ArchOpcode::kIA32And
            | ArchOpcode::kIA32Or
            | ArchOpcode::kIA32Xor
            | ArchOpcode::kIA32Add
            | ArchOpcode::kIA32Sub
            | ArchOpcode::kIA32Cmp
            | ArchOpcode::kIA32Test => rep == MachineRepresentation::kWord32 || self.IsAnyTagged(rep),
            ArchOpcode::kIA32Cmp16 | ArchOpcode::kIA32Test16 => rep == MachineRepresentation::kWord16,
            ArchOpcode::kIA32Cmp8 | ArchOpcode::kIA32Test8 => rep == MachineRepresentation::kWord8,
            _ => return false,
        }
    }

    fn CanBeImmediate(&self, node: OpIndex) -> bool {
        if self.IsExternalConstant(node) {
            return true;
        }
        // if let Some(constant) = self.Get(node).TryCast::<ConstantOp>() {
        //     match constant.kind {
        //         ConstantOp::Kind::kWord32
        //         | ConstantOp::Kind::kRelocatableWasmCall
        //         | ConstantOp::Kind::kRelocatableWasmStubCall
        //         | ConstantOp::Kind::kSmi => return true,
        //         ConstantOp::Kind::kNumber => return constant.number().get_bits() == 0,
        //         _ => (),
        //     }
        // }
        true
    }

    fn GetImmediateIntegerValue(&self, node: OpIndex) -> i32 {
        0
    }

    fn ValueFitsIntoImmediate(&self, value: i64) -> bool {
        i32::MIN < value as i32 && (value as i32) <= i32::MAX
    }

    fn GenerateMemoryOperandInputs(
        &mut self,
        index: Option<OpIndex>,
        scale: i32,
        base: OpIndex,
        displacement: i32,
        displacement_mode: DisplacementMode,
        inputs: &mut [InstructionOperand],
        input_count: &mut usize,
        register_mode: RegisterMode,
    ) -> AddressingMode {
        let mut mode = AddressingMode::kMode_MRI;
        let mut displacement_val =