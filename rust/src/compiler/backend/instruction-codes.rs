// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: This Rust translation is a structural approximation and might require
// further refinement for complete functional equivalence.  Specifically,
// the target architecture includes and macro usage are not directly translated
// due to the complexity and lack of direct Rust equivalents.

pub mod instruction_codes {
    use std::fmt;
    use std::mem;

    // Placeholder for architecture-specific instruction codes.
    // In the original C++, these are included based on preprocessor directives
    // (e.g., #if V8_TARGET_ARCH_ARM).  In Rust, we would likely use conditional
    // compilation with features, but for now, we'll stub it.

    macro_rules! if_wasm {
        ($($tokens:tt)*) => {};
    }

    /// Modes for ArchStoreWithWriteBarrier below.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum RecordWriteMode {
        kValueIsMap,
        kValueIsPointer,
        kValueIsIndirectPointer,
        kValueIsEphemeronKey,
        kValueIsAny,
    }

    impl RecordWriteMode {
        pub fn write_barrier_kind_to_record_write_mode(
            write_barrier_kind: WriteBarrierKind,
        ) -> Self {
            match write_barrier_kind {
                WriteBarrierKind::kMapWriteBarrier => RecordWriteMode::kValueIsMap,
                WriteBarrierKind::kPointerWriteBarrier => RecordWriteMode::kValueIsPointer,
                WriteBarrierKind::kIndirectPointerWriteBarrier => {
                    RecordWriteMode::kValueIsIndirectPointer
                }
                WriteBarrierKind::kEphemeronKeyWriteBarrier => {
                    RecordWriteMode::kValueIsEphemeronKey
                }
                WriteBarrierKind::kFullWriteBarrier => RecordWriteMode::kValueIsAny,
                WriteBarrierKind::kNoWriteBarrier => {
                    panic!("Should not be passed as argument.");
                }
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum WriteBarrierKind {
        kMapWriteBarrier,
        kPointerWriteBarrier,
        kIndirectPointerWriteBarrier,
        kEphemeronKeyWriteBarrier,
        kFullWriteBarrier,
        kNoWriteBarrier,
    }

    macro_rules! common_arch_opcode_with_memory_access_mode_list {
        ($V:ident) => {
            $V!(AtomicExchangeInt8);
            $V!(AtomicExchangeUint8);
            $V!(AtomicExchangeInt16);
            $V!(AtomicExchangeUint16);
            $V!(AtomicExchangeWord32);
            $V!(AtomicCompareExchangeInt8);
            $V!(AtomicCompareExchangeUint8);
            $V!(AtomicCompareExchangeInt16);
            $V!(AtomicCompareExchangeUint16);
            $V!(AtomicCompareExchangeWord32);
            $V!(AtomicAddInt8);
            $V!(AtomicAddUint8);
            $V!(AtomicAddInt16);
            $V!(AtomicAddUint16);
            $V!(AtomicAddWord32);
            $V!(AtomicSubInt8);
            $V!(AtomicSubUint8);
            $V!(AtomicSubInt16);
            $V!(AtomicSubUint16);
            $V!(AtomicSubWord32);
            $V!(AtomicAndInt8);
            $V!(AtomicAndUint8);
            $V!(AtomicAndInt16);
            $V!(AtomicAndUint16);
            $V!(AtomicAndWord32);
            $V!(AtomicOrInt8);
            $V!(AtomicOrUint8);
            $V!(AtomicOrInt16);
            $V!(AtomicOrUint16);
            $V!(AtomicOrWord32);
            $V!(AtomicXorInt8);
            $V!(AtomicXorUint8);
            $V!(AtomicXorInt16);
            $V!(AtomicXorUint16);
            $V!(AtomicXorWord32);
            $V!(ArchStoreWithWriteBarrier);
            $V!(ArchAtomicStoreWithWriteBarrier);
            $V!(ArchStoreIndirectWithWriteBarrier);
            $V!(AtomicLoadInt8);
            $V!(AtomicLoadUint8);
            $V!(AtomicLoadInt16);
            $V!(AtomicLoadUint16);
            $V!(AtomicLoadWord32);
            $V!(AtomicStoreWord8);
            $V!(AtomicStoreWord16);
            $V!(AtomicStoreWord32);
        };
    }

    macro_rules! common_arch_opcode_list {
        ($V:ident) => {
            $V!(ArchTailCallCodeObject);
            $V!(ArchTailCallAddress);
            if_wasm!($V!(ArchTailCallWasm));
            if_wasm!($V!(ArchTailCallWasmIndirect));

            $V!(ArchCallCodeObject);
            $V!(ArchCallJSFunction);
            if_wasm!($V!(ArchCallWasmFunction));
            if_wasm!($V!(ArchCallWasmFunctionIndirect));
            $V!(ArchCallBuiltinPointer);

            $V!(ArchPrepareCallCFunction);
            $V!(ArchSaveCallerRegisters);
            $V!(ArchRestoreCallerRegisters);
            $V!(ArchCallCFunction);
            $V!(ArchCallCFunctionWithFrameState);
            $V!(ArchPrepareTailCall);
            $V!(ArchJmp);
            $V!(ArchBinarySearchSwitch);
            $V!(ArchTableSwitch);
            $V!(ArchNop);
            $V!(ArchAbortCSADcheck);
            $V!(ArchDebugBreak);
            $V!(ArchComment);
            $V!(ArchThrowTerminator);
            $V!(ArchDeoptimize);
            $V!(ArchRet);
            $V!(ArchFramePointer);
            if_wasm!($V!(ArchStackPointer));
            if_wasm!($V!(ArchSetStackPointer));
            $V!(ArchParentFramePointer);
            $V!(ArchTruncateDoubleToI);
            $V!(ArchStackSlot);
            $V!(ArchStackPointerGreaterThan);
            $V!(ArchStackCheckOffset);
            $V!(Ieee754Float64Acos);
            $V!(Ieee754Float64Acosh);
            $V!(Ieee754Float64Asin);
            $V!(Ieee754Float64Asinh);
            $V!(Ieee754Float64Atan);
            $V!(Ieee754Float64Atanh);
            $V!(Ieee754Float64Atan2);
            $V!(Ieee754Float64Cbrt);
            $V!(Ieee754Float64Cos);
            $V!(Ieee754Float64Cosh);
            $V!(Ieee754Float64Exp);
            $V!(Ieee754Float64Expm1);
            $V!(Ieee754Float64Log);
            $V!(Ieee754Float64Log1p);
            $V!(Ieee754Float64Log10);
            $V!(Ieee754Float64Log2);
            $V!(Ieee754Float64Pow);
            $V!(Ieee754Float64Sin);
            $V!(Ieee754Float64Sinh);
            $V!(Ieee754Float64Tan);
            $V!(Ieee754Float64Tanh);
            common_arch_opcode_with_memory_access_mode_list!($V);
        };
    }

    macro_rules! target_arch_opcode_list {
        ($V:ident) => {};
    }

    macro_rules! arch_opcode_list {
        ($V:ident) => {
            common_arch_opcode_list!($V);
            target_arch_opcode_list!($V);
        };
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u32)]
    pub enum ArchOpcode {
        // Opcodes are declared here
        #[allow(non_camel_case_types)]
        kArchTailCallCodeObject,
        #[allow(non_camel_case_types)]
        kArchTailCallAddress,
        #[allow(non_camel_case_types)]
        kArchCallCodeObject,
        #[allow(non_camel_case_types)]
        kArchCallJSFunction,
        #[allow(non_camel_case_types)]
        kArchCallBuiltinPointer,
        #[allow(non_camel_case_types)]
        kArchPrepareCallCFunction,
        #[allow(non_camel_case_types)]
        kArchSaveCallerRegisters,
        #[allow(non_camel_case_types)]
        kArchRestoreCallerRegisters,
        #[allow(non_camel_case_types)]
        kArchCallCFunction,
        #[allow(non_camel_case_types)]
        kArchCallCFunctionWithFrameState,
        #[allow(non_camel_case_types)]
        kArchPrepareTailCall,
        #[allow(non_camel_case_types)]
        kArchJmp,
        #[allow(non_camel_case_types)]
        kArchBinarySearchSwitch,
        #[allow(non_camel_case_types)]
        kArchTableSwitch,
        #[allow(non_camel_case_types)]
        kArchNop,
        #[allow(non_camel_case_types)]
        kArchAbortCSADcheck,
        #[allow(non_camel_case_types)]
        kArchDebugBreak,
        #[allow(non_camel_case_types)]
        kArchComment,
        #[allow(non_camel_case_types)]
        kArchThrowTerminator,
        #[allow(non_camel_case_types)]
        kArchDeoptimize,
        #[allow(non_camel_case_types)]
        kArchRet,
        #[allow(non_camel_case_types)]
        kArchFramePointer,
        #[allow(non_camel_case_types)]
        kArchParentFramePointer,
        #[allow(non_camel_case_types)]
        kArchTruncateDoubleToI,
        #[allow(non_camel_case_types)]
        kArchStackSlot,
        #[allow(non_camel_case_types)]
        kArchStackPointerGreaterThan,
        #[allow(non_camel_case_types)]
        kArchStackCheckOffset,
        #[allow(non_camel_case_types)]
        kIeee754Float64Acos,
        #[allow(non_camel_case_types)]
        kIeee754Float64Acosh,
        #[allow(non_camel_case_types)]
        kIeee754Float64Asin,
        #[allow(non_camel_case_types)]
        kIeee754Float64Asinh,
        #[allow(non_camel_case_types)]
        kIeee754Float64Atan,
        #[allow(non_camel_case_types)]
        kIeee754Float64Atanh,
        #[allow(non_camel_case_types)]
        kIeee754Float64Atan2,
        #[allow(non_camel_case_types)]
        kIeee754Float64Cbrt,
        #[allow(non_camel_case_types)]
        kIeee754Float64Cos,
        #[allow(non_camel_case_types)]
        kIeee754Float64Cosh,
        #[allow(non_camel_case_types)]
        kIeee754Float64Exp,
        #[allow(non_camel_case_types)]
        kIeee754Float64Expm1,
        #[allow(non_camel_case_types)]
        kIeee754Float64Log,
        #[allow(non_camel_case_types)]
        kIeee754Float64Log1p,
        #[allow(non_camel_case_types)]
        kIeee754Float64Log10,
        #[allow(non_camel_case_types)]
        kIeee754Float64Log2,
        #[allow(non_camel_case_types)]
        kIeee754Float64Pow,
        #[allow(non_camel_case_types)]
        kIeee754Float64Sin,
        #[allow(non_camel_case_types)]
        kIeee754Float64Sinh,
        #[allow(non_camel_case_types)]
        kIeee754Float64Tan,
        #[allow(non_camel_case_types)]
        kIeee754Float64Tanh,
        #[allow(non_camel_case_types)]
        kAtomicExchangeInt8,
        #[allow(non_camel_case_types)]
        kAtomicExchangeUint8,
        #[allow(non_camel_case_types)]
        kAtomicExchangeInt16,
        #[allow(non_camel_case_types)]
        kAtomicExchangeUint16,
        #[allow(non_camel_case_types)]
        kAtomicExchangeWord32,
        #[allow(non_camel_case_types)]
        kAtomicCompareExchangeInt8,
        #[allow(non_camel_case_types)]
        kAtomicCompareExchangeUint8,
        #[allow(non_camel_case_types)]
        kAtomicCompareExchangeInt16,
        #[allow(non_camel_case_types)]
        kAtomicCompareExchangeUint16,
        #[allow(non_camel_case_types)]
        kAtomicCompareExchangeWord32,
        #[allow(non_camel_case_types)]
        kAtomicAddInt8,
        #[allow(non_camel_case_types)]
        kAtomicAddUint8,
        #[allow(non_camel_case_types)]
        kAtomicAddInt16,
        #[allow(non_camel_case_types)]
        kAtomicAddUint16,
        #[allow(non_camel_case_types)]
        kAtomicAddWord32,
        #[allow(non_camel_case_types)]
        kAtomicSubInt8,
        #[allow(non_camel_case_types)]
        kAtomicSubUint8,
        #[allow(non_camel_case_types)]
        kAtomicSubInt16,
        #[allow(non_camel_case_types)]
        kAtomicSubUint16,
        #[allow(non_camel_case_types)]
        kAtomicSubWord32,
        #[allow(non_camel_case_types)]
        kAtomicAndInt8,
        #[allow(non_camel_case_types)]
        kAtomicAndUint8,
        #[allow(non_camel_case_types)]
        kAtomicAndInt16,
        #[allow(non_camel_case_types)]
        kAtomicAndUint16,
        #[allow(non_camel_case_types)]
        kAtomicAndWord32,
        #[allow(non_camel_case_types)]
        kAtomicOrInt8,
        #[allow(non_camel_case_types)]
        kAtomicOrUint8,
        #[allow(non_camel_case_types)]
        kAtomicOrInt16,
        #[allow(non_camel_case_types)]
        kAtomicOrUint16,
        #[allow(non_camel_case_types)]
        kAtomicOrWord32,
        #[allow(non_camel_case_types)]
        kAtomicXorInt8,
        #[allow(non_camel_case_types)]
        kAtomicXorUint8,
        #[allow(non_camel_case_types)]
        kAtomicXorInt16,
        #[allow(non_camel_case_types)]
        kAtomicXorUint16,
        #[allow(non_camel_case_types)]
        kAtomicXorWord32,
        #[allow(non_camel_case_types)]
        kArchStoreWithWriteBarrier,
        #[allow(non_camel_case_types)]
        kArchAtomicStoreWithWriteBarrier,
        #[allow(non_camel_case_types)]
        kArchStoreIndirectWithWriteBarrier,
        #[allow(non_camel_case_types)]
        kAtomicLoadInt8,
        #[allow(non_camel_case_types)]
        kAtomicLoadUint8,
        #[allow(non_camel_case_types)]
        kAtomicLoadInt16,
        #[allow(non_camel_case_types)]
        kAtomicLoadUint16,
        #[allow(non_camel_case_types)]
        kAtomicLoadWord32,
        #[allow(non_camel_case_types)]
        kAtomicStoreWord8,
        #[allow(non_camel_case_types)]
        kAtomicStoreWord16,
        #[allow(non_camel_case_types)]
        kAtomicStoreWord32,
        // End of opcodes
        kLastArchOpcode,
    }

    impl ArchOpcode {
        pub const COUNT: i32 = ArchOpcode::kLastArchOpcode as i32;
    }

    impl fmt::Display for ArchOpcode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    macro_rules! target_addressing_mode_list {
        ($V:ident) => {};
    }

    macro_rules! addressing_mode_list {
        ($V:ident) => {
            $V!(None);
            target_addressing_mode_list!($V);
        };
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum AddressingMode {
        #[allow(non_camel_case_types)]
        kMode_None,
        kLastAddressingMode,
    }

    impl AddressingMode {
        pub const COUNT: i32 = AddressingMode::kLastAddressingMode as i32;
    }

    impl fmt::Display for AddressingMode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    /// The mode of the flags continuation (see below).
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum FlagsMode {
        kFlags_none = 0,
        kFlags_branch = 1,
        kFlags_deoptimize = 2,
        kFlags_set = 3,
        kFlags_trap = 4,
        kFlags_select = 5,
        kFlags_conditional_set = 6,
        kFlags_conditional_branch = 7,
    }

    impl fmt::Display for FlagsMode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    /// The condition of flags continuation (see below).
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum FlagsCondition {
        kEqual,
        kNotEqual,
        kSignedLessThan,
        kSignedGreaterThanOrEqual,
        kSignedLessThanOrEqual,
        kSignedGreaterThan,
        kUnsignedLessThan,
        kUnsignedGreaterThanOrEqual,
        kUnsignedLessThanOrEqual,
        kUnsignedGreaterThan,
        kFloatLessThanOrUnordered,
        kFloatGreaterThanOrEqual,
        kFloatLessThanOrEqual,
        kFloatGreaterThanOrUnordered,
        kFloatLessThan,
        kFloatGreaterThanOrEqualOrUnordered,
        kFloatLessThanOrEqualOrUnordered,
        kFloatGreaterThan,
        kUnorderedEqual,
        kUnorderedNotEqual,
        kOverflow,
        kNotOverflow,
        kPositiveOrZero,
        kNegative,
        kIsNaN,
        kIsNotNaN,
    }

    impl fmt::Display for FlagsCondition {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    pub const K_STACK_POINTER_GREATER_THAN_CONDITION: FlagsCondition =
        FlagsCondition::kUnsignedGreaterThan;

    impl FlagsCondition {
        pub fn negate_flags_condition(condition: FlagsCondition) -> Self {
            unsafe { mem::transmute(condition as u8 ^ 1) }
        }

        pub fn commute_flags_condition(condition: FlagsCondition) -> Self {
            match condition {
                FlagsCondition::kSignedLessThan => FlagsCondition::kSignedGreaterThan,
                FlagsCondition::kSignedGreaterThan => FlagsCondition::kSignedLessThan,
                FlagsCondition::kSignedLessThanOrEqual => {
                    FlagsCondition::kSignedGreaterThanOrEqual
                }
                FlagsCondition::kSignedGreaterThanOrEqual => {
                    FlagsCondition::kSignedLessThanOrEqual
                }
                FlagsCondition::kUnsignedLessThan => FlagsCondition::kUnsignedGreaterThan,
                FlagsCondition::kUnsignedGreaterThan => FlagsCondition::kUnsignedLessThan,
                FlagsCondition::kUnsignedLessThanOrEqual => {
                    FlagsCondition::kUnsignedGreaterThanOrEqual
                }
                FlagsCondition::kUnsignedGreaterThanOrEqual => {
                    FlagsCondition::kUnsignedLessThanOrEqual
                }
                FlagsCondition::kFloatLessThan => FlagsCondition::kFloatGreaterThan,
                FlagsCondition::kFloatGreaterThan => FlagsCondition::kFloatLessThan,
                FlagsCondition::kFloatLessThanOrEqual => FlagsCondition::kFloatGreaterThanOrEqual,
                FlagsCondition::kFloatGreaterThanOrEqual => FlagsCondition::kFloatLessThanOrEqual,
                FlagsCondition::kFloatLessThanOrUnordered => {
                    FlagsCondition::kFloatGreaterThanOrUnordered
                }
                FlagsCondition::kFloatGreaterThanOrUnordered => {
                    FlagsCondition::kFloatLessThanOrUnordered
                }
                FlagsCondition::kFloatLessThanOrEqualOrUnordered => {
                    FlagsCondition::kFloatGreaterThan
                }
                FlagsCondition::kFloatGreaterThanOrEqualOrUnordered => {
                    FlagsCondition::kFloatLessThan
                }
                _ => condition,
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum MemoryAccessMode {
        kMemoryAccessDirect = 0,
        kMemoryAccessProtectedMemOutOfBounds = 1,
        kMemoryAccessProtectedNullDereference = 2,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum AtomicWidth {
        kWord32,
        kWord64,
    }

    impl AtomicWidth {
        pub fn atomic_width_size(width: AtomicWidth) -> usize {
            match width {
                AtomicWidth::kWord32 => 4,
                AtomicWidth::kWord64 => 8,
            }
        }
    }

    pub const K_LAZY_DEOPT_ON_THROW_SENTINEL: i32 = -1;

    /// The InstructionCode is an opaque, target-specific integer that encodes what
    /// code to emit for an instruction in the code generator. It is not interesting
    /// to the register allocator, as the inputs and flags on the instructions
    /// specify everything of interest.
    pub type InstructionCode = u32;

    pub mod base {
        pub struct BitField<T, const START: usize, const LENGTH: usize>;

        impl<T, const START: usize, const LENGTH: usize> BitField<T, START, LENGTH> {
            pub const fn is_valid(_value: T) -> bool {
                //In C++ this would be checked at compile time.
                true
            }
            pub type Next<U, const NEXT_LENGTH: usize> = BitField<U, { START + LENGTH }, NEXT_LENGTH>;

            pub const kMax: u32 = (1 << LENGTH) - 1;
            
            pub const kLastUsedBit: usize = START + LENGTH - 1;
        }
    }

    use base::BitField;
    use codegen::atomic_memory_order::AtomicMemoryOrder;

    pub type ArchOpcodeField = BitField<ArchOpcode, 0, 9>;
    pub type AddressingModeField = BitField<AddressingMode, 9, 5>;
    pub type FlagsModeField = BitField<FlagsMode, 14, 3>;
    pub type FlagsConditionField = BitField<FlagsCondition, 17, 5>;
    pub type AtomicWidthField = BitField<AtomicWidth, 22, 2>;
    pub type AtomicMemoryOrderField = BitField<AtomicMemoryOrder, 24, 2>;
    pub type AtomicStoreRecordWriteModeField = BitField<RecordWriteMode, 26, 4>;
    pub type RecordWriteModeField = BitField<RecordWriteMode, 22, 3>;

    #[cfg(target_arch = "x86_64")]
    pub mod x64 {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum LaneSize {
            kL8 = 0,
            kL16 = 1,
            kL32 = 2,
            kL64 = 3,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum VectorLength {
            kV128 = 0,
            kV256 = 1,
            kV512 = 3,
        }
    }

    #[cfg(target_arch = "x86_64")]
    use x64::{LaneSize, VectorLength};

    #[cfg(target_arch = "x86_64")]
    pub type LaneSizeField = BitField<LaneSize, 22, 2>;
    #[cfg(target_arch = "x86_64")]
    pub type VectorLengthField = BitField<VectorLength, 24, 2>;
    #[cfg(not(target_arch = "x86_64"))]
    pub type LaneSizeField = BitField<i32, 22, 8>;

    pub type AccessModeField = BitField<MemoryAccessMode, 30, 2>;
    pub type DeoptImmedArgsCountField = BitField<i32, 22, 2>;
    pub type DeoptFrameStateOffsetField = BitField<i32, 24, 8>;
    pub type ParamField = BitField<i32, 22, 5>;
    pub type FPParamField = BitField<i32, 27, 5>;
    pub type MiscField = BitField<i32, 22, 10>;

    pub fn has_memory_access_mode(opcode: ArchOpcode) -> bool {
        match opcode {
            ArchOpcode::kAtomicExchangeInt8
            | ArchOpcode::kAtomicExchangeUint8
            | ArchOpcode::kAtomicExchangeInt16
            | ArchOpcode::kAtomicExchangeUint16
            | ArchOpcode::kAtomicExchangeWord32
            | ArchOpcode::kAtomicCompareExchangeInt8
            | ArchOpcode::kAtomicCompareExchangeUint8
            | ArchOpcode::kAtomicCompareExchangeInt16
            | ArchOpcode::kAtomicCompareExchangeUint16
            | ArchOpcode::kAtomicCompareExchangeWord32
            | ArchOpcode::kAtomicAddInt8
            | ArchOpcode::kAtomicAddUint8
            | ArchOpcode::kAtomicAddInt16
            | ArchOpcode::kAtomicAddUint16
            | ArchOpcode::kAtomicAddWord32
            | ArchOpcode::kAtomicSubInt8
            | ArchOpcode::kAtomicSubUint8
            | ArchOpcode::kAtomicSubInt16
            | ArchOpcode::kAtomicSubUint16
            | ArchOpcode::kAtomicSubWord32
            | ArchOpcode::kAtomicAndInt8
            | ArchOpcode::kAtomicAndUint8
            | ArchOpcode::kAtomicAndInt16
            | ArchOpcode::kAtomicAndUint16
            | ArchOpcode::kAtomicAndWord32
            | ArchOpcode::kAtomicOrInt8
            | ArchOpcode::kAtomicOrUint8
            | ArchOpcode::kAtomicOrInt16
            | ArchOpcode::kAtomicOrUint16
            | ArchOpcode::kAtomicOrWord32
            | ArchOpcode::kAtomicXorInt8
            | ArchOpcode::kAtomicXorUint8
            | ArchOpcode::kAtomicXorInt16
            | ArchOpcode::kAtomicXorUint16
            | ArchOpcode::kAtomicXorWord32
            | ArchOpcode::kArchStoreWithWriteBarrier
            | ArchOpcode::kArchAtomicStoreWithWriteBarrier
            | ArchOpcode::kArchStoreIndirectWithWriteBarrier
            | ArchOpcode::kAtomicLoadInt8
            | ArchOpcode::kAtomicLoadUint8
            | ArchOpcode::kAtomicLoadInt16
            | ArchOpcode::kAtomicLoadUint16
            | ArchOpcode::kAtomicLoadWord32
            | ArchOpcode::kAtomicStoreWord8
            | ArchOpcode::kAtomicStoreWord16
            | ArchOpcode::kAtomicStoreWord32 => true,
            _ => false,
        }
    }
}

pub mod codegen {
    pub mod atomic_memory_order {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum AtomicMemoryOrder {
            Relaxed,
            Acquire,
            Release,
            AcqRel,
            SeqCst
        }
    }
}