// Converted from V8 C++ source files:
// Header: instruction.h
// Implementation: instruction.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]
#![allow(dead_code)]
use std::any::Any;
use std::cell::RefCell;
use std::cmp::{Eq, Ordering, PartialEq, PartialOrd};
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::Deref;
use std::ptr;
use std::rc::Rc;

use crate::compiler::instruction_codes::*;
use crate::compiler::opcodes::*;
use crate::compiler::turboshaft::graph::BlockIndex;
use crate::compiler::turboshaft::int64_lowering_reducer::Block;
use crate::compiler::turboshaft::int64_lowering_reducer::MachineRepresentation;
use crate::compiler::turboshaft::maglev_graph_building_phase::BytecodeOffset;
use crate::compiler::turboshaft::maglev_graph_building_phase::FeedbackSource;
use crate::execution::messages::Handle;
use crate::execution::microtask_queue::Isolate;
use crate::execution::thread_id::Zone;
use crate::objects::heap_object_inl::*;
use crate::utils::ostreams::StdoutStream;
use crate::V8;

pub struct RegisterConfiguration {}

pub enum AliasingKind {
    kOverlap,
    kIndependent,
    kCombine,
}

const kFPAliasing: AliasingKind = AliasingKind::kOverlap;

pub struct AccountingAllocator {}

pub type Address = usize;

pub struct Local<T> {}
impl<T> Local<T> {
    pub fn new() -> Self {
        Local {}
    }
}

pub struct FrameStateData {}

pub struct Frame {}

pub struct OptimizedCompilationInfo {}

pub struct InstructionSequence {}

pub struct Register {}

pub struct DoubleRegister {}

pub struct FloatRegister {}

pub struct Simd128Register {}

pub struct Simd256Register {}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RelocInfoMode {
    NO_INFO,
    // Add other modes as needed
}

#[derive(Debug, Copy, Clone)]
pub enum StateValueKind {
    kArgumentsElements,
    kArgumentsLength,
    kRestLength,
    kPlain,
    kOptimizedOut,
    kNestedObject,
    kDuplicate,
    kStringConcat,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ArgumentsStateType {
    kArgumentsStateType1, // Placeholder
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DeoptimizeKind {
    kLazy, // Placeholder
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DeoptimizeReason {
    kWrongType, // Placeholder
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub struct RpoNumber {
    index_: i32,
}

impl RpoNumber {
    pub const kInvalidRpoNumber: i32 = -1;

    pub fn new() -> Self {
        RpoNumber {
            index_: Self::kInvalidRpoNumber,
        }
    }

    pub fn ToInt(&self) -> i32 {
        assert!(self.IsValid());
        self.index_
    }
    pub fn ToSize(&self) -> usize {
        assert!(self.IsValid());
        self.index_ as usize
    }
    pub fn IsValid(&self) -> bool {
        self.index_ >= 0
    }
    pub fn FromInt(index: i32) -> Self {
        RpoNumber { index_: index }
    }
    pub fn Invalid() -> Self {
        RpoNumber {
            index_: Self::kInvalidRpoNumber,
        }
    }

    pub fn IsNext(&self, other: RpoNumber) -> bool {
        assert!(self.IsValid());
        other.index_ == self.index_ + 1
    }

    pub fn Next(&self) -> Self {
        assert!(self.IsValid());
        RpoNumber {
            index_: self.index_ + 1,
        }
    }

    // Comparison operators.
    pub fn operator_eq(&self, other: RpoNumber) -> bool {
        self.index_ == other.index_
    }
    pub fn operator_ne(&self, other: RpoNumber) -> bool {
        self.index_ != other.index_
    }
    pub fn operator_gt(&self, other: RpoNumber) -> bool {
        self.index_ > other.index_
    }
    pub fn operator_lt(&self, other: RpoNumber) -> bool {
        self.index_ < other.index_
    }
    pub fn operator_le(&self, other: RpoNumber) -> bool {
        self.index_ <= other.index_
    }
    pub fn operator_ge(&self, other: RpoNumber) -> bool {
        self.index_ >= other.index_
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SourcePosition {}

#[derive(Debug, Clone, Copy)]
pub enum MachineType {
    AnyTagged,
    AnyTaggedPointer,
    AnyCompressed,
    // Add other machine types as needed
}

impl MachineType {
    pub fn PointerRepresentation() -> MachineType {
        MachineType::AnyTaggedPointer
    }
    pub fn TaggedSigned() -> MachineType {
        MachineType::AnyTagged
    }
    pub fn AnyTagged() -> MachineType {
        MachineType::AnyTagged
    }
}

pub fn CanBeTaggedOrCompressedPointer(_mr: MachineRepresentation) -> bool {
    true
}

pub fn IsFloatingPoint(_mr: MachineRepresentation) -> bool {
    false
}

pub mod base {
    pub struct BitField64<T, const START: usize, const SIZE: usize>;

    impl<T, const START: usize, const SIZE: usize> BitField64<T, START, SIZE> {
        pub const kShift: usize = START;
    }
}

impl fmt::Display for RpoNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ToSize())
    }
}

impl PartialEq for BitVector {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl Eq for BitVector {}

impl PartialOrd for BitVector {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl Ord for BitVector {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

#[derive(Debug, Clone)]
pub struct BitVector {}

impl BitVector {
    pub fn Contains(&self, _vreg: i32) -> bool {
        true
    }

    pub fn Add(&mut self, _vreg: i32) {}
}
pub mod objects_inl {
    pub struct Object {}
}

pub mod isolate {
    pub type Address = usize;
}

mod wasm {
    #[derive(Debug, Copy, Clone)]
    pub enum ValueKind {
        WasmI32,
        WasmI64,
        WasmF32,
        WasmF64,
        WasmAnyRef,
        WasmFuncRef,
        WasmExternRef,
    }

    pub struct CanonicalSig {}

    pub fn WasmReturnTypeFromSignature(_signature: &CanonicalSig) -> Option<ValueKind> {
        None
    }
}

pub mod optimized_compilation_info {
    pub struct OptimizedCompilationInfo {}
}

mod ic {
    pub struct CodeKind {}
}

mod frame_state_function_info {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum FrameStateType {
        kUnoptimizedFunction,
        kBuiltinContinuation,
        kJavaScriptBuiltinContinuation,
        kJavaScriptBuiltinContinuationWithCatch,
        kInlinedExtraArguments,
        kConstructCreateStub,
        kConstructInvokeStub,
        kLiftoffFunction,
        kJSToWasmBuiltinContinuation,
        kWasmInlinedIntoJS,
    }

    pub fn IsJSFunctionType(_type: FrameStateType) -> bool {
        true
    }
}

pub mod unoptimized_frame_info {
    pub struct UnoptimizedFrameInfo {}

    impl UnoptimizedFrameInfo {
        pub fn Conservative(_parameters_count: i32, _locals_count: i32) -> UnoptimizedFrameInfo {
            UnoptimizedFrameInfo {}
        }

        pub fn GetStackSizeForAdditionalArguments(_parameters_count: i32) -> usize {
            0
        }

        pub fn frame_size_in_bytes(&self) -> usize {
            0
        }
    }
}

pub mod fast_construct_stub_frame_info {
    pub struct FastConstructStubFrameInfo {}

    impl FastConstructStubFrameInfo {
        pub fn Conservative() -> FastConstructStubFrameInfo {
            FastConstructStubFrameInfo {}
        }

        pub fn frame_size_in_bytes(&self) -> usize {
            0
        }
    }
}

pub mod construct_stub_frame_info {
    pub struct ConstructStubFrameInfo {}

    impl ConstructStubFrameInfo {
        pub fn Conservative(_parameters_count: i32) -> ConstructStubFrameInfo {
            ConstructStubFrameInfo {}
        }

        pub fn frame_size_in_bytes(&self) -> usize {
            0
        }
    }
}

pub mod builtin_continuation_frame_info {
    pub struct BuiltinContinuationFrameInfo {}

    impl BuiltinContinuationFrameInfo {
        pub fn Conservative(
            _parameters_count: i32,
            _call_interface_descriptor: ic::CodeKind,
            _config: &RegisterConfiguration,
        ) -> BuiltinContinuationFrameInfo {
            BuiltinContinuationFrameInfo {}
        }

        pub fn frame_size_in_bytes(&self) -> usize {
            0
        }
    }
}

// Add your Rust code here

const INSTRUCTION_OPERAND_ALIGN: usize = 8;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(align(8))]
pub struct InstructionOperand {
    value_: u64,
}

impl InstructionOperand {
    pub const kInvalidVirtualRegister: i32 = -1;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum Kind {
        INVALID,
        UNALLOCATED,
        CONSTANT,
        IMMEDIATE,
        PENDING,
        // Location operand kinds.
        ALLOCATED,
    }

    pub fn new() -> Self {
        InstructionOperand::new_invalid()
    }

    pub fn new_invalid() -> Self {
        InstructionOperand {
            value_: KindField::encode(Kind::INVALID),
        }
    }

    pub fn kind(&self) -> Kind {
        KindField::decode(self.value_)
    }

    pub fn IsInvalid(&self) -> bool {
        self.kind() == Kind::INVALID
    }
    // UnallocatedOperands are place-holder operands created before register
    // allocation. They later are assigned registers and become AllocatedOperands.
    pub fn IsUnallocated(&self) -> bool {
        self.kind() == Kind::UNALLOCATED
    }
    // Constant operands participate in register allocation. They are allocated to
    // registers but have a special "spilling" behavior. When a ConstantOperand
    // value must be rematerialized, it is loaded from an immediate constant
    // rather from an unspilled slot.
    pub fn IsConstant(&self) -> bool {
        self.kind() == Kind::CONSTANT
    }
    // ImmediateOperands do not participate in register allocation and are only
    // embedded directly in instructions, e.g. small integers and on some
    // platforms Objects.
    pub fn IsImmediate(&self) -> bool {
        self.kind() == Kind::IMMEDIATE
    }
    // PendingOperands are pending allocation during register allocation and
    // shouldn't be seen elsewhere. They chain together multiple operators that
    // will be replaced together with the same value when finalized.
    pub fn IsPending(&self) -> bool {
        self.kind() == Kind::PENDING
    }
    // AllocatedOperands are registers or stack slots that are assigned by the
    // register allocator and are always associated with a virtual register.
    pub fn IsAllocated(&self) -> bool {
        self.kind() == Kind::ALLOCATED
    }

    pub fn IsAnyLocationOperand(&self) -> bool {
        matches!(self.kind(), Self::Kind::ALLOCATED)
    }

    pub fn IsLocationOperand(&self) -> bool {
        self.IsAnyLocationOperand()
            && !IsFloatingPoint(LocationOperand::cast(self).representation())
    }

    pub fn IsFPLocationOperand(&self) -> bool {
        self.IsAnyLocationOperand()
            && IsFloatingPoint(LocationOperand::cast(self).representation())
    }

    pub fn IsAnyRegister(&self) -> bool {
        self.IsAnyLocationOperand()
            && LocationOperand::cast(self).location_kind() == LocationOperand::LocationKind::REGISTER
    }

    pub fn IsRegister(&self) -> bool {
        self.IsAnyRegister()
            && !IsFloatingPoint(LocationOperand::cast(self).representation())
    }

    pub fn IsFPRegister(&self) -> bool {
        self.IsAnyRegister()
            && IsFloatingPoint(LocationOperand::cast(self).representation())
    }

    pub fn IsFloatRegister(&self) -> bool {
        self.IsAnyRegister()
            && LocationOperand::cast(self).representation() == MachineRepresentation::kFloat32
    }

    pub fn IsDoubleRegister(&self) -> bool {
        self.IsAnyRegister()
            && LocationOperand::cast(self).representation() == MachineRepresentation::kFloat64
    }

    pub fn IsSimd128Register(&self) -> bool {
        self.IsAnyRegister()
            && LocationOperand::cast(self).representation() == MachineRepresentation::kSimd128
    }

    pub fn IsSimd256Register(&self) -> bool {
        self.IsAnyRegister()
            && LocationOperand::cast(self).representation() == MachineRepresentation::kSimd256
    }

    pub fn IsAnyStackSlot(&self) -> bool {
        self.IsAnyLocationOperand()
            && LocationOperand::cast(self).location_kind() == LocationOperand::LocationKind::STACK_SLOT
    }

    pub fn IsStackSlot(&self) -> bool {
        self.IsAnyStackSlot()
            && !IsFloatingPoint(LocationOperand::cast(self).representation())
    }

    pub fn IsFPStackSlot(&self) -> bool {
        self.IsAnyStackSlot()
            && IsFloatingPoint(LocationOperand::cast(self).representation())
    }

    pub fn IsFloatStackSlot(&self) -> bool {
        self.IsAnyLocationOperand()
            && LocationOperand::cast(self).location_kind() == LocationOperand::LocationKind::STACK_SLOT
            && LocationOperand::cast(self).representation() == MachineRepresentation::kFloat32
    }

    pub fn IsDoubleStackSlot(&self) -> bool {
        self.IsAnyLocationOperand()
            && LocationOperand::cast(self).location_kind() == LocationOperand::LocationKind::STACK_SLOT
            && LocationOperand::cast(self).representation() == MachineRepresentation::kFloat64
    }

    pub fn IsSimd128StackSlot(&self) -> bool {
        self.IsAnyLocationOperand()
            && LocationOperand::cast(self).location_kind() == LocationOperand::LocationKind::STACK_SLOT
            && LocationOperand::cast(self).representation() == MachineRepresentation::kSimd128
    }

    pub fn IsSimd256StackSlot(&self) -> bool {
        self.IsAnyLocationOperand()
            && LocationOperand::cast(self).location_kind() == LocationOperand::LocationKind::STACK_SLOT
            && LocationOperand::cast(self).representation() == MachineRepresentation::kSimd256
    }

    pub fn new_unallocated(policy: UnallocatedOperand::ExtendedPolicy, virtual_register: i32) -> Self {
        UnallocatedOperand::new(policy, virtual_register).into()
    }

    pub fn new_constant(virtual_register: i32) -> Self {
        ConstantOperand::new(virtual_register).into()
    }

    pub fn new_immediate(type_: ImmediateOperand::ImmediateType, value: i32) -> Self {
        ImmediateOperand::new(type_, value).into()
    }

    pub fn new_pending() -> Self {
        PendingOperand::new().into()
    }

    pub fn new_allocated(kind: LocationOperand::LocationKind, rep: MachineRepresentation, index: i32) -> Self {
        AllocatedOperand::new(kind, rep, index).into()
    }

    pub fn replace_with(&mut self, src: &InstructionOperand) {
        self.value_ = src.value_;
    }

    pub fn equals(&self, that: &InstructionOperand) -> bool {
        if self.IsPending() {
            // Pending operands are only equal if they are the same operand.
            return self as *const _ == that as *const _;
        }
        self.value_ == that.value_
    }

    pub fn compare(&self, that: &InstructionOperand) -> bool {
        self.value_ < that.value_
    }

    pub fn equals_canonicalized(&self, that: &InstructionOperand) -> bool {
        if self.IsPending() {
            // Pending operands can't be canonicalized, so just compare for equality.
            return self.equals(that);
        }
        self.get_canonicalized_value() == that.get_canonicalized_value()
    }

    pub fn compare_canonicalized(&self, that: &InstructionOperand) -> bool {
        assert!(!self.IsPending());
        self.get_canonicalized_value() < that.get_canonicalized_value()
    }

    pub fn interferes_with(&self, other: &InstructionOperand) -> bool {
        let combine_fp_aliasing =
            kFPAliasing == AliasingKind::kCombine && self.IsFPLocationOperand() && other.IsFPLocationOperand();
        let stack_slots = self.IsAnyStackSlot() && other.IsAnyStackSlot();
        if !combine_fp_aliasing && !stack_slots {
            return self.equals_canonicalized(other);
        }
        let loc = LocationOperand::cast(self);
        let other_loc = LocationOperand::cast(other);
        let rep = loc.representation();
        let other_rep = other_loc.representation();
        let kind = loc.location_kind();
        let other_kind = other_loc.location_kind();
        if kind != other_kind {
            return false;
        }

        if combine_fp_aliasing && !stack_slots {
            if rep == other_rep {
                return self.equals_canonicalized(other);
            }
            assert_eq!(kind, LocationOperand::LocationKind::REGISTER);
            // FP register-register interference.
            return GetRegConfig().AreAliases(rep, loc.register_code(), other_rep, other_loc.register_code());
        }

        assert!(stack_slots);
        let num_slots = ElementSizeInBytes(rep);
        let num_slots_other = ElementSizeInBytes(other_rep);
        let complex_stack_slot_interference = (num_slots > 1 || num_slots_other > 1);
        if !complex_stack_slot_interference {
            return self.equals_canonicalized(other);
        }

        // Complex multi-slot operand interference:
        // - slots of different FP reps can alias because the gap resolver may break a
        // move into 2 or 4 equivalent smaller moves,
        // - stack layout can be rearranged for tail calls
        assert_eq!(LocationOperand::LocationKind::STACK_SLOT, kind);
        let index_hi = loc.index();
        let index_lo = index_hi - AlignedSlotAllocator::NumSlotsForWidth(ElementSizeInBytes(rep)) as i32 + 1;
        let other_index_hi = other_loc.index();
        let other_index_lo = other_index_hi - AlignedSlotAllocator::NumSlotsForWidth(ElementSizeInBytes(other_rep)) as i32 + 1;
        other_index_hi >= index_lo && index_hi >= other_index_lo
    }

    pub fn print(&self) {
        StdoutStream{}.write(format!("{}", self).as_bytes());
    }

    pub fn operator_eq(&self, other: &InstructionOperand) -> bool {
        self.equals(other)
    }
    pub fn operator_ne(&self, other: &InstructionOperand) -> bool {
        !self.equals(other)
    }
}

impl From<UnallocatedOperand> for InstructionOperand {
    fn from(operand: UnallocatedOperand) -> Self {
        InstructionOperand { value_: operand.value_ }
    }
}

impl From<ConstantOperand> for InstructionOperand {
    fn from(operand: ConstantOperand) -> Self {
        InstructionOperand { value_: operand.value_ }
    }
}

impl From<ImmediateOperand> for InstructionOperand {
    fn from(operand: ImmediateOperand) -> Self {
        InstructionOperand { value_: operand.value_ }
    }
}

impl From<PendingOperand> for InstructionOperand {
    fn from(operand: PendingOperand) -> Self {
        InstructionOperand { value_: operand.value_ }
    }
}

impl From<AllocatedOperand> for InstructionOperand {
    fn from(operand: AllocatedOperand) -> Self {
        InstructionOperand { value_: operand.value_ }
    }
}

impl fmt::Display for InstructionOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind() {
            InstructionOperand::Kind::UNALLOCATED => {
                let unalloc = UnallocatedOperand::cast(self);
                write!(f, "v{}", unalloc.virtual_register())?;
                if unalloc.basic_policy() == UnallocatedOperand::BasicPolicy::FIXED_SLOT {
                    write!(f, "(={}S)", unalloc.fixed_slot_index())?;
                    return Ok(());
                }
                match unalloc.extended_policy() {
                    UnallocatedOperand::ExtendedPolicy::NONE => Ok(()),
                    UnallocatedOperand::ExtendedPolicy::FIXED_REGISTER => {
                        write!(f, "(=Register)")
                    }
                    UnallocatedOperand::ExtendedPolicy::FIXED_FP_REGISTER => {
                        write!(f, "(=FPRegister)")
                    }
                    UnallocatedOperand::ExtendedPolicy::MUST_HAVE_REGISTER => write!(f, "(R)"),
                    UnallocatedOperand::ExtendedPolicy::MUST_HAVE_SLOT => write!(f, "(S)"),
                    UnallocatedOperand::ExtendedPolicy::SAME_AS_INPUT => {
                        write!(f, "({})", unalloc.input_index())
                    }
                    UnallocatedOperand::ExtendedPolicy::REGISTER_OR_SLOT => write!(f, "(-)"),
                    UnallocatedOperand::ExtendedPolicy::REGISTER_OR_SLOT_OR_CONSTANT => {
                        write!(f, "(*)")
                    }
                }
            }
            InstructionOperand::Kind::CONSTANT => {
                write!(
                    f,
                    "[constant:v{}]",
                    ConstantOperand::cast(self).virtual_register()
                )
            }
            InstructionOperand::Kind::IMMEDIATE => {
                let imm = ImmediateOperand::cast(self);
                match imm.type_() {
                    ImmediateOperand::ImmediateType::INLINE_INT32 => {
                        write!(f, "#{}", imm.inline_int32_value())
                    }
                    ImmediateOperand::ImmediateType::INLINE_INT64 => {
                        write!(f, "#{}", imm.inline_int64_value())
                    }
                    ImmediateOperand::ImmediateType::INDEXED_RPO => {
                        write!(f, "[rpo_immediate:{}]", imm.indexed_value())
                    }
                    ImmediateOperand::ImmediateType::INDEXED_IMM => {
                        write!(f, "[immediate:{}]", imm.indexed_value())
                    }
                }
            }
            InstructionOperand::Kind::PENDING => {
                write!(f, "[pending: {:?}]", PendingOperand::cast(self).next())
            }
            InstructionOperand::Kind::ALLOCATED => {
                let allocated = LocationOperand::cast(self);
                if self.IsStackSlot() {
                    write!(f, "[stack:{}]", allocated.index())?;
                } else if self.IsFPStackSlot() {
                    write!(f, "[fp_stack:{}]", allocated.index())?;
                } else if self.IsRegister() {
                    let name = if allocated.register_code() < 10 {
                        "Register".to_string()
                    } else {
                        "SpecialRegister".to_string()
                    };
                    write!(f, "[{}|R", name)?;
                } else if self.IsDoubleRegister() {
                    write!(f, "[DoubleRegister|R")?;
                } else if self.IsFloatRegister() {
                    write!(f, "[FloatRegister|R")?;
                } else if self.IsSimd256Register() {
                    write!(f, "[Simd256Register|R")?;
                } else {
                    assert!(self.IsSimd128Register());
                    write!(f, "[Simd128Register|R")?;
                }
                match allocated.representation() {
                    MachineRepresentation::kNone => write!(f, "|-"),
                    MachineRepresentation::kBit => write!(f, "|b"),
                    MachineRepresentation::kWord8 => write!(f, "|w8"),
                    MachineRepresentation::kWord16 => write!(f, "|w16"),
                    MachineRepresentation::kWord32 => write!(f, "|w32"),
                    MachineRepresentation::kWord64 => write!(f, "|w64"),
                    MachineRepresentation::kFloat16 => write!(f, "|f16"),
                    MachineRepresentation::kFloat32 => write!(f, "|f32"),
                    MachineRepresentation::kFloat64 => write!(f, "|f64"),
                    MachineRepresentation::kSimd128 => write!(f, "|s128"),
                    MachineRepresentation::kSimd256 => write!(f, "|s256"),
                    MachineRepresentation::kTaggedSigned => write!(f, "|ts"),
                    MachineRepresentation::kTaggedPointer => write!(f, "|tp"),
                    MachineRepresentation::kTagged => write!(f, "|t"),
                    MachineRepresentation::kCompressedPointer => write!(f, "|cp"),
                    MachineRepresentation::kCompressed => write!(f, "|c"),
                    MachineRepresentation::kProtectedPointer => write!(f, "|pp"),
                    MachineRepresentation::kIndirectPointer => write!(f, "|ip"),
                    MachineRepresentation::kSandboxedPointer => write!(f, "|sb"),
                    MachineRepresentation::kMapWord => unreachable!(),
                    MachineRepresentation::kFloat16RawBits => unreachable!(),
                }?;
                write!(f, "]")
            }
            InstructionOperand::Kind::INVALID => write!(f, "(x)"),
        }
    }
}

impl InstructionOperand {
    fn get_canonicalized_value(&self) -> u64 {
        if self.IsAnyLocationOperand() {
            let mut canonical = MachineRepresentation::kNone;
            if self.IsFPRegister() {
                if kFPAliasing == AliasingKind::kOverlap {
                    // We treat all FP register operands the same for simple aliasing.
                    canonical = MachineRepresentation::kFloat64;
                } else if kFPAliasing == AliasingKind::kIndependent {
                    if self.IsSimd128Register() {
                        canonical = MachineRepresentation::kSimd128;
                    } else {
                        canonical = MachineRepresentation::kFloat64;
                    }
                } else {
                    // We need to distinguish FP register operands of different reps when
                    // aliasing is AliasingKind::kCombine (e.g. ARM).
                    assert_eq!(kFPAliasing, AliasingKind::kCombine);
                    canonical = LocationOperand::cast(self).representation();
                }
            }
            KindField::update(
                LocationOperand::RepresentationField::update(self.value_, canonical),
                InstructionOperand::Kind::ALLOCATED as InstructionOperand::Kind,
            )
        } else {
            self.value_
        }
    }
}

mod kind_field {
    use super::{base, InstructionOperand};

    pub type Kind = InstructionOperand::Kind;

    pub const START: usize = 0;
    pub const SIZE: usize = 3;
    pub type BitField = base::BitField64<Kind, START, SIZE>;

    pub fn encode(kind: Kind) -> u64 {
        kind as u64
    }

    pub fn decode(value: u64) -> Kind {
        match value & 0x7 {
            0 => Kind::INVALID,
            1 => Kind::UNALLOCATED,
            2 => Kind::CONSTANT,
            3 => Kind::IMMEDIATE,
            4 => Kind::PENDING,
            5 => Kind::ALLOCATED,
            _ => panic!("Invalid Kind value"),
        }
    }

    pub fn next<U, const SIZE_: usize>(_: U) -> base::BitField64<U, 3, SIZE_> {
        base::BitField64
    }
}

pub use kind_field as KindField;

pub type InstructionOperandVector = Vec<InstructionOperand>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UnallocatedOperand {
    value_: u64,
}

impl UnallocatedOperand {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum BasicPolicy {
        FIXED_SLOT,
        EXTENDED_POLICY,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum ExtendedPolicy {
        NONE,
        REGISTER_OR_SLOT,
        REGISTER_OR_SLOT_OR_CONSTANT,
        FIXED_REGISTER,
        FIXED_FP_REGISTER,
        MUST_HAVE_REGISTER,
        MUST_HAVE_SLOT,
        SAME_AS_INPUT,
    }

    // Lifetime of operand inside the instruction.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum Lifetime {
        // USED_AT_START operand is guaranteed to be live only at instruction start.
        // The register allocator is free to assign the same register to some other
        // operand used inside instruction (i.e. temporary or output).
        USED_AT_START,

        // USED_AT_END operand is treated as live until the end of instruction.
        // This means that register allocator will not reuse its register for any
        // other operand inside instruction.
        USED_AT_END,
    }

    pub fn new(policy: ExtendedPolicy, virtual_register: i32) -> Self {
        let mut value_: u64 = KindField::encode(InstructionOperand::Kind::UNALLOCATED);
        value_ |= BasicPolicyField::encode(BasicPolicy::EXTENDED_POLICY);
        value_ |= ExtendedPolicyField::encode(policy);
        value_ |= LifetimeField::encode(Lifetime::USED_AT_END);
        value_ |= VirtualRegisterField::encode(virtual_register as u32);

        UnallocatedOperand { value_ }
    }

    pub fn new_same_as_input(virtual_register: i32, input_index: i32) -> Self {
        let mut value_: u64 = KindField::encode(InstructionOperand::Kind::UNALLOCATED);
        value_ |= BasicPolicyField::encode(BasicPolicy::EXTENDED_POLICY);
        value_ |= ExtendedPolicyField::encode(ExtendedPolicy::SAME_AS_INPUT);
        value_ |= LifetimeField::encode(Lifetime::USED_AT_END);
        value_ |= VirtualRegisterField::encode(virtual_register as u32);
        value_ |= InputIndexField::encode(input_index);

        UnallocatedOperand { value_ }
    }

    pub fn new_fixed_slot(policy: BasicPolicy, index: i32, virtual_register: i32) -> Self {
        assert_eq!(policy, BasicPolicy::FIXED_SLOT);
        let mut value_: u64 = KindField::encode(InstructionOperand::Kind::UNALLOCATED);
        value_ |= BasicPolicyField::encode(policy);
        value_ |= (index as i64 as u64) << FixedSlotIndexField::kShift;
        value_ |= VirtualRegisterField::encode(virtual_register as u32);
        assert_eq!(Self::new_from_value(value_).fixed_slot_index(), index);

        UnallocatedOperand { value_ }
    }

    pub fn new_fixed_register(policy: ExtendedPolicy, index: i32, virtual_register: i32) -> Self {
        assert!(policy == ExtendedPolicy::FIXED_REGISTER || policy == ExtendedPolicy::FIXED_FP_REGISTER);
        let mut value_: u64 = KindField::encode(InstructionOperand::Kind::UNALLOCATED);
        value_ |= BasicPolicyField::encode(BasicPolicy::EXTENDED_POLICY);
        value_ |= ExtendedPolicyField::encode(policy);
        value_ |= LifetimeField::encode(Lifetime::USED_AT_END);
        value_ |= FixedRegisterField::encode(index);
        value_ |= VirtualRegisterField::encode(virtual_register as u32);

        UnallocatedOperand { value_ }
    }

    pub fn new_with_lifetime(
        policy: ExtendedPolicy,
        lifetime: Lifetime,
        virtual_register: i32,
    ) -> Self {
        let mut value_: u64 = KindField::encode(InstructionOperand::Kind::UNALLOCATED);
        value_ |= BasicPolicyField::encode(BasicPolicy::EXTENDED_POLICY);
        value_ |= ExtendedPolicyField::encode(policy);
        value_ |= LifetimeField::encode(lifetime);
        value_ |= VirtualRegisterField::encode(virtual_register as u32);


