// NOTE: This is a partial conversion. Some parts require deeper understanding
// of the V8 architecture and surrounding code. UNIMPLEMENTED!() is used
// to mark code that cannot be directly translated.

use std::convert::TryInto;
use std::ops::{BitAnd, BitOr, BitXor};
use std::{
    marker::PhantomData,
    mem,
    num::Wrapping,
    ptr,
    sync::{Arc, Mutex},
};

// Placeholder for base/bits.h
mod base {
    pub mod bits {
        #[inline]
        pub fn is_uint5(value: i64) -> bool {
            (value & !0x1f) == 0
        }

        #[inline]
        pub fn is_uint6(value: i64) -> bool {
            (value & !0x3f) == 0
        }
    }
}

// Placeholder for src/codegen/machine-type.h
mod codegen {
    pub mod machine_type {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
            kFloat16,
            kSimd256,
            kCompressedPointer,
            kProtectedPointer,
            kSandboxedPointer,
            kCompressed,
            kMapWord,
            kIndirectPointer,
            kFloat16RawBits,
            kTaggedSigned,
            kTaggedPointer,
            kTagged,
        }

        impl MachineRepresentation {
            pub fn size(&self) -> usize {
                match self {
                    MachineRepresentation::kNone => 0,
                    MachineRepresentation::kBit => 1, // Assuming bit is 1 byte
                    MachineRepresentation::kWord8 => 1,
                    MachineRepresentation::kWord16 => 2,
                    MachineRepresentation::kWord32 => 4,
                    MachineRepresentation::kWord64 => 8,
                    MachineRepresentation::kFloat32 => 4,
                    MachineRepresentation::kFloat64 => 8,
                    MachineRepresentation::kSimd128 => 16,
                    MachineRepresentation::kFloat16 => 2,
                    MachineRepresentation::kSimd256 => 32,
                    MachineRepresentation::kCompressedPointer => 4, // Assuming compressed pointer is 4 bytes
                    MachineRepresentation::kProtectedPointer => 8,
                    MachineRepresentation::kSandboxedPointer => 8,
                    MachineRepresentation::kCompressed => 4,
                    MachineRepresentation::kMapWord => 8,
                    MachineRepresentation::kIndirectPointer => 8,
                    MachineRepresentation::kFloat16RawBits => 2,
                    MachineRepresentation::kTaggedSigned => 8,
                    MachineRepresentation::kTaggedPointer => 8,
                    MachineRepresentation::kTagged => 8,
                }
            }
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct LoadRepresentation {
            representation: MachineRepresentation,
            is_signed: bool,
        }

        impl LoadRepresentation {
            pub fn new(representation: MachineRepresentation, is_signed: bool) -> Self {
                Self {
                    representation,
                    is_signed,
                }
            }

            pub fn representation(&self) -> MachineRepresentation {
                self.representation
            }

            pub fn is_signed(&self) -> bool {
                self.is_signed
            }

            pub fn is_unsigned(&self) -> bool {
                !self.is_signed
            }
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum WriteBarrierKind {
            kNoWriteBarrier,
            kFullWriteBarrier,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct StoreRepresentation {
            representation: MachineRepresentation,
            write_barrier_kind: WriteBarrierKind,
        }

        impl StoreRepresentation {
            pub fn new(representation: MachineRepresentation, write_barrier_kind: WriteBarrierKind) -> Self {
                Self {
                    representation,
                    write_barrier_kind,
                }
            }

            pub fn representation(&self) -> MachineRepresentation {
                self.representation
            }

            pub fn write_barrier_kind(&self) -> WriteBarrierKind {
                self.write_barrier_kind
            }
        }

        pub fn can_be_tagged_pointer(rep: MachineRepresentation) -> bool {
            rep == MachineRepresentation::kTaggedSigned
                || rep == MachineRepresentation::kTaggedPointer
                || rep == MachineRepresentation::kTagged
        }
        pub fn can_be_tagged_or_compressed_pointer(rep: MachineRepresentation) -> bool {
            can_be_tagged_pointer(rep) || rep == MachineRepresentation::kCompressedPointer
        }
    }
}

// Placeholder for logging
mod logging {
    #[macro_export]
    macro_rules! trace {
        ($($arg:tt)*) => {
            println!($($arg)*);
        }
    }

    // Placeholder for PrintF
    #[macro_export]
    macro_rules! printf {
        ($($arg:tt)*) => {
            println!($($arg)*);
        }
    }
}

// Placeholder for turboshaft operations
mod turboshaft {
    pub mod operations {
        use super::super::codegen::machine_type::MachineRepresentation;

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum Opcode {
            kInvalid,
            kWord32Equal,
            kComparison,
            kOverflowCheckedBinop,
            kLoad,
            kStackPointerGreaterThan,
            kWord32BitwiseAnd,
            kWord64BitwiseAnd,
            kChangeInt32ToInt64,
            kChangeUint32ToUint64,
        }

        #[derive(Debug, Clone)]
        pub struct Operation {
            pub opcode: Opcode,
            pub inputs: Vec<OpIndex>,
            pub kind: OperationKind,
        }

        impl Operation {
            pub fn new(opcode: Opcode, inputs: Vec<OpIndex>, kind: OperationKind) -> Self {
                Self {
                    opcode,
                    inputs,
                    kind,
                }
            }
            pub fn is<T: Opmask>(&self) -> bool {
                T::check(self)
            }

            pub fn input(&self, index: usize) -> OpIndex {
                self.inputs[index]
            }

            pub fn try_cast<T: OpType>(&self) -> Option<&T> {
                T::try_from_operation(self)
            }

            pub fn cast<T: OpType>(&self) -> &T {
                T::from_operation(self)
            }
        }

        pub trait OpType {
            fn try_from_operation(op: &Operation) -> Option<&Self>;
            fn from_operation(op: &Operation) -> &Self {
                Self::try_from_operation(op).expect("Failed to cast Operation to OpType")
            }
        }

        pub trait Opmask {
            fn check(op: &Operation) -> bool;
        }

        impl<F: Fn(&Operation) -> bool> Opmask for F {
            fn check(op: &Operation) -> bool {
                (self)(op)
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum OperationKind {
            None,
            Constant(ConstantOpKind),
            Shift(ShiftOpKind),
            Comparison(ComparisonOpKind),
            Change(ChangeOpKind),
            Load(LoadOpKind),
            Store(StoreOpKind),
            OverflowCheckedBinop(OverflowCheckedBinopKind),
            StackPointerGreaterThan(StackPointerGreaterThanKind),
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct ConstantOpKind {
            pub kind: ConstantOpEnum,
            pub integral: i64,
            pub float32: Float32,
            pub float64: Float64,
            pub external_reference: u64,
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct ShiftOpKind {
            pub kind: ShiftOpEnum,
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct ComparisonOpKind {
            pub rep: Option<RegisterRepresentation>,
            pub kind: ComparisonOpEnum,
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct ChangeOpKind {}

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct LoadOpKind {
            pub base: OpIndex,
            pub index: Option<OpIndex>,
            pub offset: i64,
            pub element_size_log2: i32,
            pub loaded_rep: LoadRepresentation,
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct StoreOpKind {
            pub displacement: i64,
            pub stored_rep: StoreRepresentation,
            pub memory_order: Option<MemoryOrder>,
            pub access_kind: AccessKind,
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct OverflowCheckedBinopKind {
            pub rep: WordRepresentation,
            pub kind: OverflowCheckedBinopEnum,
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct StackPointerGreaterThanKind {
            pub kind: StackCheckKind,
            pub stack_limit: OpIndex,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum ConstantOpEnum {
            kInteger,
            kFloat32,
            kFloat64,
            kExternalReference,
            kIntegral,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum ShiftOpEnum {
            kShiftRightArithmetic,
            kShiftRightArithmeticShiftOutZeros,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum ComparisonOpEnum {
            kEqual,
            kSignedLessThan,
            kSignedLessThanOrEqual,
            kFloatLessThan,
            kFloatLessThanOrEqual,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum StackCheckKind {
            kJSFunctionEntry,
            kOther,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum MemoryOrder {
            kRelaxed,
            kAcquire,
            kRelease,
            kAcquireRelease,
            kSequentiallyConsistent,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum AccessKind {
            kNormal,
            kUnaligned,
            kAtomic,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum OverflowCheckedBinopEnum {
            kSignedAdd,
            kSignedSub,
            kSignedMul,
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Float32 {
            bits: u32,
        }

        impl Float32 {
            pub fn new(bits: u32) -> Self {
                Self { bits }
            }

            pub fn get_bits(&self) -> u32 {
                self.bits
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Float64 {
            bits: u64,
        }

        impl Float64 {
            pub fn new(bits: u64) -> Self {
                Self { bits }
            }

            pub fn get_bits(&self) -> u64 {
                self.bits
            }
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum RegisterRepresentation {
            Word32(),
            Word64(),
            Float32(),
            Float64(),
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum WordRepresentation {
            Word32(),
            Word64(),
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct ConstantOp {
            pub kind: ConstantOpEnum,
            pub integral: i64,
            pub float32: Float32,
            pub float64: Float64,
        }

        impl ConstantOp {
            pub fn new(kind: ConstantOpEnum, integral: i64, float32: Float32, float64: Float64) -> Self {
                Self {
                    kind,
                    integral,
                    float32,
                    float64,
                }
            }

            pub fn is_integral(&self) -> bool {
                self.kind == ConstantOpEnum::kIntegral
            }

            pub fn integral(&self) -> i64 {
                self.integral
            }

            pub fn float32(&self) -> Float32 {
                self.float32.clone()
            }

            pub fn float64(&self) -> Float64 {
                self.float64.clone()
            }
        }

        impl OpType for ConstantOp {
            fn try_from_operation(op: &Operation) -> Option<&Self> {
                if let OperationKind::Constant(kind) = &op.kind {
                    unsafe { Some(mem::transmute(op)) }
                } else {
                    None
                }
            }
        }

        pub struct ShiftOp {
            pub left: OpIndex,
            pub right: OpIndex,
            pub kind: ShiftOpEnum,
        }

        impl OpType for ShiftOp {
            fn try_from_operation(op: &Operation) -> Option<&Self> {
                if let OperationKind::Shift(kind) = &op.kind {
                    unsafe { Some(mem::transmute(op)) }
                } else {
                    None
                }
            }
        }

        pub struct ComparisonOp {
            pub rep: Option<RegisterRepresentation>,
            pub left: OpIndex,
            pub right: OpIndex,
            pub kind: ComparisonOpEnum,
        }

        impl OpType for ComparisonOp {
            fn try_from_operation(op: &Operation) -> Option<&Self> {
                if let OperationKind::Comparison(kind) = &op.kind {
                    unsafe { Some(mem::transmute(op)) }
                } else {
                    None
                }
            }
        }

        pub struct ChangeOp {
            pub input: OpIndex,
        }

        impl OpType for ChangeOp {
            fn try_from_operation(op: &Operation) -> Option<&Self> {
                if let OperationKind::Change(kind) = &op.kind {
                    unsafe { Some(mem::transmute(op)) }
                } else {
                    None
                }
            }
        }

        pub struct LoadOp {
            pub base: OpIndex,
            pub index: Option<OpIndex>,
            pub offset: i64,
            pub element_size_log2: i32,
            pub loaded_rep: LoadRepresentation,
        }

        impl OpType for LoadOp {
            fn try_from_operation(op: &Operation) -> Option<&Self> {
                if let OperationKind::Load(kind) = &op.kind {
                    unsafe { Some(mem::transmute(op)) }
                } else {
                    None
                }
            }
        }

        pub struct StoreOp {
            pub displacement: i64,
            pub index: OpIndex,
            pub value: OpIndex,
            pub stored_rep: StoreRepresentation,
            pub memory_order: Option<MemoryOrder>,
            pub access_kind: AccessKind,
        }

        impl OpType for StoreOp {
            fn try_from_operation(op: &Operation) -> Option<&Self> {
                if let OperationKind::Store(kind) = &op.kind {
                    unsafe { Some(mem::transmute(op)) }
                } else {
                    None
                }
            }
        }

        pub struct OverflowCheckedBinopOp {
            pub rep: WordRepresentation,
            pub left: OpIndex,
            pub right: OpIndex,
            pub kind: OverflowCheckedBinopEnum,
        }

        impl OpType for OverflowCheckedBinopOp {
            fn try_from_operation(op: &Operation) -> Option<&Self> {
                if let OperationKind::OverflowCheckedBinop(kind) = &op.kind {
                    unsafe { Some(mem::transmute(op)) }
                } else {
                    None
                }
            }
        }

        pub struct StackPointerGreaterThanOp {
            pub kind: StackCheckKind,
            pub stack_limit: OpIndex,
        }

        impl OpType for StackPointerGreaterThanOp {
            fn try_from_operation(op: &Operation) -> Option<&Self> {
                if let OperationKind::StackPointerGreaterThan(kind) = &op.kind {
                    unsafe { Some(mem::transmute(op)) }
                } else {
                    None
                }
            }
        }

        pub struct ProjectionOp {
            pub index: u32,
        }

        impl OpType for ProjectionOp {
            fn try_from_operation(op: &Operation) -> Option<&Self> {
                // UNIMPLEMENTED!()
                None
            }
        }

        pub struct AtomicRMWOp {
            // UNIMPLEMENTED!()
            pub memory_rep: MemoryRepresentation,
        }

        impl OpType for AtomicRMWOp {
            fn try_from_operation(op: &Operation) -> Option<&Self> {
                // UNIMPLEMENTED!()
                None
            }
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub struct OpIndex(pub usize);

        impl OpIndex {
            pub fn valid(&self) -> bool {
                self.0 != 0
            }
        }
    }

    pub mod opmasks {
        use super::operations::*;

        pub fn kExternalConstant(op: &Operation) -> bool {
            if let OperationKind::Constant(constant) = &op.kind {
                constant.kind == ConstantOpEnum::kExternalReference
            } else {
                false
            }
        }

        pub fn kWord32Constant(op: &Operation) -> bool {
            if let OperationKind::Constant(constant) = &op.kind {
                constant.kind == ConstantOpEnum::kIntegral
            } else {
                false
            }
        }

        pub fn kChangeInt32ToInt64(op: &Operation) -> bool {
            op.opcode == Opcode::kChangeInt32ToInt64
        }

        pub fn kChangeUint32ToUint64(op: &Operation) -> bool {
            op.opcode == Opcode::kChangeUint32ToUint64
        }

        pub fn kWord32ShiftRightArithmetic(op: &Operation) -> bool {
            if let OperationKind::Shift(kind) = &op.kind {
                kind.kind == ShiftOpEnum::kShiftRightArithmetic
            } else {
                false
            }
        }

        pub fn kWord32Equal(op: &Operation) -> bool {
            op.opcode == Opcode::kWord32Equal
        }

        pub fn kWord32BitwiseAnd(op: &Operation) -> bool {
            op.opcode == Opcode::kWord32BitwiseAnd
        }

        pub fn kWord64BitwiseAnd(op: &Operation) -> bool {
            op.opcode == Opcode::kWord64BitwiseAnd
        }

        pub fn kTruncateFloat32ToInt32OverflowToMin(op: &Operation) -> bool {
            // UNIMPLEMENTED!()
            false
        }

        pub fn kTruncateFloat32ToUint32OverflowToMin(op: &Operation) -> bool {
            // UNIMPLEMENTED!()
            false
        }

        pub fn kTruncateFloat64ToInt64OverflowToMin(op: &Operation) -> bool {
            // UNIMPLEMENTED!()
            false
        }
    }

    pub use operations::*;
}

// Placeholder for turboshaft adapter
mod turboshaft_adapter {
    use super::turboshaft::operations::LoadOp;
    use super::turboshaft::operations::StoreOp;

    pub struct LoadView<'a> {
        load: &'a LoadOp,
    }

    impl<'a> LoadView<'a> {
        pub fn new(load: &'a LoadOp) -> Self {
            Self { load }
        }

        pub fn loaded_rep(&self) -> super::codegen::machine_type::LoadRepresentation {
            self.load.loaded_rep
        }
    }

    pub struct StoreView<'a> {
        store: &'a StoreOp,
    }

    impl<'a> StoreView<'a> {
        pub fn new(store: &'a StoreOp) -> Self {
            Self { store }
        }

        pub fn displacement(&self) -> i64 {
            self.store.displacement
        }

        pub fn stored_rep(&self) -> super::codegen::machine_type::StoreRepresentation {
            self.store.stored_rep
        }

        pub fn index(&self) -> super::turboshaft::operations::OpIndex {
            self.store.index
        }

        pub fn memory_order(&self) -> Option<super::turboshaft::operations::MemoryOrder> {
            self.store.memory_order
        }

        pub fn access_kind(&self) -> super::turboshaft::operations::AccessKind {
            self.store.access_kind
        }
    }
}

// Placeholder for compiler backend instruction selector
mod compiler {
    pub mod backend {
        pub mod instruction_selector {
            pub struct InstructionSelectorT {}
        }
    }
}

use base::bits::{is_uint5, is_uint6};
use codegen::machine_type::{
    can_be_tagged_or_compressed_pointer, can_be_tagged_pointer, LoadRepresentation,
    MachineRepresentation, StoreRepresentation, WriteBarrierKind,
};
use logging::{printf, trace};
use turboshaft::{operations::*, opmasks};
use turboshaft_adapter::{LoadView, StoreView};

// Placeholder for flags
mod flags {
    pub static mut enable_unconditional_write_barriers: bool = false;
    pub static mut disable_write_barriers: bool = false;
}

// Placeholder for v8_flags
mod v8_flags {
    pub use super::flags::*;
}

// Placeholder for isolate
mod isolate {
    pub struct Isolate {}
}

/// Adds Mips-specific methods for generating InstructionOperands.
struct Mips64OperandGeneratorT<'a> {
    selector: &'a mut InstructionSelectorT,
}

impl<'a> Mips64OperandGeneratorT<'a> {
    pub fn new(selector: &'a mut InstructionSelectorT) -> Self {
        Self { selector }
    }

    fn selector(&self) -> &InstructionSelectorT {
        &self.selector
    }

    fn selector_mut(&mut self) -> &mut InstructionSelectorT {
        &mut self.selector
    }

    fn use_operand(&mut self, node: OpIndex, opcode: InstructionCode) -> InstructionOperand {
        if self.can_be_immediate(node, opcode) {
            self.use_immediate(node)
        } else {
            self.use_register(node)
        }
    }

    // Use the zero register if the node has the immediate value zero, otherwise
    // assign a register.
    fn use_register_or_immediate_zero(&mut self, node: OpIndex) -> InstructionOperand {
        if let Some(constant) = self.selector().get(node).try_cast::<ConstantOp>() {
            if (constant.is_integral() && constant.integral() == 0)
                || (constant.kind == ConstantOpEnum::kFloat32
                    && constant.float32().get_bits() == 0)
                || (constant.kind == ConstantOpEnum::kFloat64
                    && constant.float64().get_bits() == 0)
            {
                self.use_immediate(node)
            } else {
                self.use_register(node)
            }
        } else {
            self.use_register(node)
        }
    }

    fn is_integer_constant(&mut self, node: OpIndex) -> bool {
        let mut unused: i64 = 0;
        self.selector().match_signed_integral_constant(node, &mut unused)
    }

    fn get_optional_integer_constant(&mut self, operation: OpIndex) -> Option<i64> {
        let mut constant: i64 = 0;
        if self.selector().match_signed_integral_constant(operation, &mut constant) {
            Some(constant)
        } else {
            None
        }
    }

    fn can_be_immediate(&mut self, node: OpIndex, mode: InstructionCode) -> bool {
        if let Some(_constant) = self.selector().get(node).try_cast::<ConstantOp>() {
            let mut value: i64 = 0;
            self.selector().match_signed_integral_constant(node, &mut value)
                && self.can_be_immediate_value(value, mode)
        } else {
            false
        }
    }

    fn can_be_immediate_value(&self, value: i64, opcode: InstructionCode) -> bool {
        match ArchOpcodeField::decode(opcode) {
            ArchOpcode::kMips64Shl | ArchOpcode::kMips64Sar | ArchOpcode::kMips64Shr => {
                is_uint5(value)
            }
            ArchOpcode::kMips64Dshl | ArchOpcode::kMips64Dsar | ArchOpcode::kMips64Dshr => {
                is_uint6(value)
            }
            ArchOpcode::kMips64Add
            | ArchOpcode::kMips64And32
            | ArchOpcode::kMips64And
            | ArchOpcode::kMips64Dadd
            | ArchOpcode::kMips64Or32
            | ArchOpcode::kMips64Or
            | ArchOpcode::kMips64Tst
            | ArchOpcode::kMips64Xor => is_uint16(value),
            ArchOpcode::kMips64Lb
            | ArchOpcode::kMips64Lbu
            | ArchOpcode::kMips64Sb
            | ArchOpcode::kMips64Lh
            | ArchOpcode::kMips64Lhu
            | ArchOpcode::kMips64Sh
            | ArchOpcode::kMips64Lw
            | ArchOpcode::kMips64Sw
            | ArchOpcode::kMips64Ld
            | ArchOpcode::kMips64Sd
            | ArchOpcode::kMips64Lwc1
            | ArchOpcode::kMips64Swc1
            | ArchOpcode::kMips64Ldc1
            | ArchOpcode::kMips64Sdc1 => is_int32(value),
            _ => is_int16(value),
        }
    }

    fn immediate_fits_addr_mode1_instruction(&self, imm: i32) -> bool {
        trace!(
            "UNIMPLEMENTED instr_sel: {} at line {}\n",
            "immediate_fits_addr_mode1_instruction",
            line!()
        );
        false
    }

    fn define_as_register(&mut self, node: OpIndex) -> InstructionOperand {
        // UNIMPLEMENTED!()
        InstructionOperand {}
    }

    fn use_register(&mut self, node: OpIndex) -> InstructionOperand {
        // UNIMPLEMENTED!()
        InstructionOperand {}
    }

    fn use_immediate(&mut self, node: OpIndex) -> InstructionOperand {
        // UNIMPLEMENTED!()
        InstructionOperand {}
    }

    fn temp_immediate(&mut self, value: i32) -> InstructionOperand {
        // UNIMPLEMENTED!()
        InstructionOperand {}
    }

    fn use_fixed(&mut self, node: OpIndex, fixed: Register) -> InstructionOperand {
        // UNIMPLEMENTED!()
        InstructionOperand {}
    }

    fn no_output(&mut self) -> InstructionOperand {
        // UNIMPLEMENTED!()
        InstructionOperand {}
    }

    fn temp_register(&mut self) -> InstructionOperand {
        // UNIMPLEMENTED!()
        InstructionOperand {}
    }

    fn use_unique_register(&mut self, base: OpIndex) -> InstructionOperand {
        // UNIMPLEMENTED!()
        InstructionOperand {}
    }

    fn use(&mut self, hi: OpIndex) -> InstructionOperand {
        // UNIMPLEMENTED!()
        InstructionOperand {}
    }

    fn use_immediate64(&mut self, shift_by: i64) -> InstructionOperand {
        // UNIMPLEMENTED!()
        InstructionOperand {}
    }

    fn define_as_fixed(&mut self, node: OpIndex, register: Register) -> InstructionOperand {
        // UNIMPLEMENTED!()
        InstructionOperand {}
    }

    fn define_same_as_first(&mut self, node: OpIndex) -> InstructionOperand {
        // UNIMPLEMENTED!()
        InstructionOperand {}
    }

    fn use_register_with_mode(
        &mut self,
        value: OpIndex,
        register_mode: OperandGeneratorRegisterMode,
    ) -> InstructionOperand {
        // UNIMPLEMENTED!()
        InstructionOperand {}
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ArchOpcode {
    kArchNop,
    kArchStackSlot,
    kArchAbortCSADcheck,
    kArchPrepareCallCFunction,
    kArchStackPointerGreaterThan,
    kArchStoreWithWriteBarrier,
    kMips64Lwc1,
    kMips64Ldc1,
    kMips64Lbu,
    kMips64Lb,
    kMips64Lhu,
    kMips64Lh,
    kMips64Lw,
    kMips64Ld,
    kMips64Swc1,
    kMips64Sdc1,
    kMips64Sb,
    kMips64Sh,
    kMips64Sw,
    kMips64Sd,
    kMips64Shl,
    kMips64Shr,
    kMips64Sar,
    kMips64Dshl,
    kMips64Dshr,
    kMips64Dsar,
    kMips64Add,
    kMips64And32,
    kMips64And,
    kMips64Dadd,
    kMips64Or32,
    kMips64Or,
    kMips64Tst,
    kMips64Xor,
    kMips64Ulwc1,
    kMips64Uldc1,
    kMips64Ulhu,
    kMips64Ulh,
    kMips64Ulw,
    kMips64Uld,
    kMips64Uswc1,
    kMips64Usdc1,
    kMips64Ush,
    kMips64Usw,
    kMips64Usd,
    kMips64CmpS,
    kMips64CmpD,
    kMips64Cmp,
    kMips64Div,
    kMips64DivU,
    kMips64Mod,
    kMips64ModU,
    kMips64Ddiv,
    kMips64DdivU,
    kMips64Dmod,
    kMips64DmodU,
    kMips64CvtDS,
    kMips64CvtSW,
    kMips64CvtSUw,
    kMips64CvtDW,
    kMips64CvtDL,
    kMips64CvtDUw,
    kMips64TruncWS,
    kMips64TruncUwS,
    kMips64TruncWD,
    kMips64TruncLD,
    kMips64TruncUwD,
    kMips64TruncUlD,
    kMips64TruncLS,
    kMips64TruncUlS,
    kMips64CvtSD,
    kMips64StackClaim,
    kMips64Float64ExtractLowWord32,
    kMips64Float64ExtractHighWord32,
    kMips64Sync,
    kMips64Mul,
    kMips64Dmul,
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
    kMips64Float32Max,
    kMips6