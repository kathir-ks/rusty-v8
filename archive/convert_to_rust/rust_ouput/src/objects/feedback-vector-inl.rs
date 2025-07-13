// Converted from V8 C++ source files:
// Header: feedback-vector-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::marker::PhantomData;
use std::mem::size_of;

use crate::objects::fixed_array_inl::WriteBarrierMode;
use crate::objects::slots_atomic_inl::MaybeObjectSlot;
use crate::objects::type_hints::{BinaryOperationHint, CompareOperationHint, ForInHint};
use crate::objects::fixed_array_inl::TaggedField;
use crate::objects::fixed_array_inl::void;
use crate::objects::code_kind::CodeKind;
use crate::runtime::runtime_compiler::TieringState;
use crate::objects::debug_objects::flags;
use crate::objects::runtime_literals::FeedbackSlot;
use crate::objects::casting::Code;
use crate::objects::primitive_heap_object_inl::FixedArray;
use crate::objects::string_set::Handle;
use crate::objects::property::MaybeObjectDirectHandle;
use crate::objects::union::UseScratchRegisterScope;
use crate::objects::fixed_array_inl::DisallowGarbageCollection;
use crate::runtime::runtime_wasm::InstructionOperand;
use crate::objects::js_relative_time_format_inl::Numeric;
use crate::codegen::reglist_base::RegisterT;
use crate::objects::js_collection_inl::Object;
use crate::objects::off_heap_hash_table::InternalIndex;
use crate::objects::off_heap_hash_table::OffHeapObjectSlot;
use crate::objects::js_segment_iterator::SegmentIterator;
use crate::objects::struct_inl::Struct;
use crate::cppgc::internal::member_storage::Atomic;
use crate::objects::deoptimization_data::IndirectHandle;
use crate::objects::simd::FixedArrayBuilder;
use crate::objects::tagged::Address;
use crate::runtime::runtime_wasm::Register;
use crate::runtime::runtime_wasm::Operand;
use crate::runtime::runtime_wasm::Condition;
use crate::objects::js_function_inl::Tagged;
use crate::objects::map::Map;
use crate::objects::code::CodeWrapper;
use crate::heap::Heap;
use crate::objects::maybe_object_inl::MaybeObject;
use crate::objects::shared_function_info::SharedFunctionInfo;
use crate::objects::smi::Smi;
use crate::roots::roots_inl::ReadOnlyRoots;
use crate::torque::runtime_macro_shims::IsolateForSandbox;
use crate::torque::runtime_support::ClearedValue;
use crate::objects::heap_object::HeapObject;
use crate::objects::js_function::JSFunction;
use crate::runtime::runtime_wasm::GCType;
use crate::objects::module::Module;
use crate::runtime::runtime_regexp::DirectHandle;
use crate::objects::symbol::Symbol;
use crate::roots::roots_inl::RootIndex;
use crate::isolate::Isolate;

mod torque_generated_src_objects_feedback_vector_tq_inl;

const kHeaderSize: usize = 8;
const kInt32Size: usize = 4;
const kInvocationCountOffset: usize = kHeaderSize;
const kInvocationCountBeforeStableOffset: usize = kInvocationCountOffset + 4;
const kOsrStateOffset: usize = kInvocationCountBeforeStableOffset + 1;
const kFlagsOffset: usize = kOsrStateOffset + 1;
const kLengthOffset: usize = kFlagsOffset + 4;
const kRawFeedbackSlotsOffset: usize = kLengthOffset + 4;
const kMaxOsrUrgency: i32 = 7;
const kInvocationCountBeforeStableDeoptSentinel: u8 = 0xFF;

pub struct FeedbackVector {
    dummy: i32,
    phantom: PhantomData<()>,
}

pub struct FeedbackMetadata {
    dummy: i32,
    phantom: PhantomData<()>,
}

pub struct ClosureFeedbackCellArray {
    dummy: i32,
    phantom: PhantomData<()>,
}

impl FeedbackVector {
    pub const kFlagsTieringStateIsAnyRequested: u32 = 0x1;
    pub const kFlagsLogNextExecution: u32 = 0x2;
    pub const kFlagsMaybeHasTurbofanCode: u32 = 0x4;
    pub const kFlagsMaybeHasMaglevCode: u32 = 0x8;
    
    pub fn size(length: usize) -> usize {
        kRawFeedbackSlotsOffset + length * size_of::<Tagged<MaybeObject>>()
    }
}

impl FeedbackMetadata {
    pub const kSlotCountOffset: usize = kHeaderSize;
    pub const kCreateClosureSlotCountOffset: usize = kSlotCountOffset + 4;
    
    pub fn word_count(slot_count: i32) -> i32 {
        (slot_count + 1) / 2
    }
    
    pub fn SizeFor(slot_count: i32, create_closure_slot_count: i32) -> i32 {
        (kHeaderSize as i32 + Self::word_count(slot_count) * kInt32Size as i32)
    }
}

macro_rules! DEF_GETTER {
    ($struct_name:ident, $field_name:ident, $field_type:ty) => {
        impl $struct_name {
            pub fn $field_name(&self, _cage_base: i32) -> $field_type {
                todo!()
            }
        }
    };
}

macro_rules! DEF_ACQUIRE_GETTER {
    ($struct_name:ident, $field_name:ident, $field_type:ty) => {
        impl $struct_name {
            pub fn $field_name(&self, _cage_base: i32, _tag: AcquireLoadTag) -> $field_type {
                todo!()
            }
        }
    };
}

macro_rules! RELAXED_INT32_ACCESSORS {
    ($struct_name:ident, $field_name:ident, $offset:expr) => {
        impl $struct_name {
            pub fn $field_name(&self, _tag: RelaxedLoadTag) -> i32 {
                todo!()
            }
            pub fn set_$field_name(&self, _value: i32, _tag: RelaxedStoreTag) {
                todo!()
            }
        }
    };
}

macro_rules! RELAXED_UINT8_ACCESSORS {
    ($struct_name:ident, $field_name:ident, $offset:expr) => {
        impl $struct_name {
            pub fn $field_name(&self, _tag: RelaxedLoadTag) -> u8 {
                todo!()
            }
            pub fn set_$field_name(&self, _value: u8, _tag: RelaxedStoreTag) {
                todo!()
            }
        }
    };
}

struct OsrUrgencyBits {}
impl OsrUrgencyBits {
    const kMax: i32 = 7;
    fn decode(_osr_state: u8) -> i32 {
        todo!()
    }
    fn update(_osr_state: u8, _urgency: i32) -> u8 {
        todo!()
    }
}

struct MaybeHasMaglevOsrCodeBit {}
impl MaybeHasMaglevOsrCodeBit {
    fn decode(_osr_state: u8) -> bool {
        todo!()
    }
    fn update(_osr_state: u8, _value: bool) -> u8 {
        todo!()
    }
}

struct MaybeHasTurbofanOsrCodeBit {}
impl MaybeHasTurbofanOsrCodeBit {
    fn decode(_osr_state: u8) -> bool {
        todo!()
    }
    fn update(_osr_state: u8, _value: bool) -> u8 {
        todo!()
    }
}

struct InterruptBudgetResetByIcChangeBit {}
impl InterruptBudgetResetByIcChangeBit {
    fn decode(_flags: u32) -> bool {
        todo!()
    }
    fn update(_flags: u32, _value: bool) -> u32 {
        todo!()
    }
}

struct TieringInProgressBit {}
impl TieringInProgressBit {
    fn decode(_flags: u32) -> bool {
        todo!()
    }
}

struct TieringStateBits {}
impl TieringStateBits {
    fn decode(_flags: u32) -> TieringState {
        todo!()
    }
}

struct LogNextExecutionBit {}
impl LogNextExecutionBit {
    fn decode(_flags: u32) -> bool {
        todo!()
    }
    fn update(_flags: u32, _value: bool) -> u32 {
        todo!()
    }
}

impl FeedbackMetadata {
    pub fn slot_count(&self, _tag: AcquireLoadTag) -> i32 {
        todo!()
    }

    pub fn create_closure_slot_count(&self, _tag: AcquireLoadTag) -> i32 {
        todo!()
    }

    pub fn get(&self, index: int) -> int32_t {
        todo!()
    }

    pub fn set(&mut self, index: int, value: int32_t) {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        DCHECK_IMPLIES(self.slot_count() == 0, self.create_closure_slot_count() == 0);
        return self.slot_count() == 0;
    }

    pub fn AllocatedSize() -> int {
        return SizeFor(slot_count(kAcquireLoad),
                       create_closure_slot_count(kAcquireLoad));
    }

    pub fn word_count(&self) const -> int {
        return FeedbackMetadata::word_count(self.slot_count());
    }

    pub fn GetSlotSize(kind: FeedbackSlotKind) -> int {
        match kind {
            FeedbackSlotKind::kForIn |
            FeedbackSlotKind::kInstanceOf |
            FeedbackSlotKind::kTypeOf |
            FeedbackSlotKind::kCompareOp |
            FeedbackSlotKind::kBinaryOp |
            FeedbackSlotKind::kLiteral |
            FeedbackSlotKind::kJumpLoop => {
                return 1;
            }

            FeedbackSlotKind::kCall |
            FeedbackSlotKind::kCloneObject |
            FeedbackSlotKind::kLoadProperty |
            FeedbackSlotKind::kLoadGlobalInsideTypeof |
            FeedbackSlotKind::kLoadGlobalNotInsideTypeof |
            FeedbackSlotKind::kLoadKeyed |
            FeedbackSlotKind::kHasKeyed |
            FeedbackSlotKind::kSetNamedSloppy |
            FeedbackSlotKind::kSetNamedStrict |
            FeedbackSlotKind::kDefineNamedOwn |
            FeedbackSlotKind::kDefineKeyedOwn |
            FeedbackSlotKind::kStoreGlobalSloppy |
            FeedbackSlotKind::kStoreGlobalStrict |
            FeedbackSlotKind::kSetKeyedSloppy |
            FeedbackSlotKind::kSetKeyedStrict |
            FeedbackSlotKind::kStoreInArrayLiteral |
            FeedbackSlotKind::kDefineKeyedOwnPropertyInLiteral => {
                return 2;
            }

            FeedbackSlotKind::kInvalid => {
                UNREACHABLE();
            }
        }
        UNREACHABLE();
    }
}

impl FeedbackVector {
    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }
    pub fn invocation_count(&self, _tag: RelaxedLoadTag) -> i32 {
        todo!()
    }
    pub fn set_invocation_count(&self, _value: i32, _tag: RelaxedStoreTag) {
        todo!()
    }
    pub fn invocation_count_before_stable(&self, _tag: RelaxedLoadTag) -> u8 {
        todo!()
    }
    pub fn set_invocation_count_before_stable(&self, _value: u8, _tag: RelaxedStoreTag) {
        todo!()
    }
    pub fn osr_state(&self) -> u8 {
        todo!()
    }
    pub fn set_osr_state(&self, _value: u8) {
        todo!()
    }
    pub fn flags(&self) -> u32 {
        todo!()
    }
    pub fn set_flags(&self, _value: u32) {
        todo!()
    }
    pub fn length(&self) -> i32 {
        todo!()
    }
    pub fn raw_feedback_slots(&self, _index: i32, _tag: RelaxedLoadTag) -> Tagged<MaybeObject> {
        todo!()
    }
    pub fn raw_feedback_slots(&self, _cage_base: PtrComprCageBase, _index: i32, _tag: RelaxedLoadTag) -> Tagged<MaybeObject> {
        todo!()
    }
    pub fn set_raw_feedback_slots(&self, _index: i32, _value: Tagged<MaybeObject>, _tag: RelaxedStoreTag, _mode: WriteBarrierMode) {
        todo!()
    }
    fn get_index(&self, slot: FeedbackSlot) -> i32 {
        todo!()
    }

    pub fn clear_invocation_count(&self, tag: RelaxedStoreTag) {
        self.set_invocation_count(0, tag);
    }

    pub fn osr_urgency(&self) -> i32 {
        OsrUrgencyBits::decode(self.osr_state())
    }

    pub fn set_osr_urgency(&self, urgency: i32) {
        DCHECK(0 <= urgency && urgency <= FeedbackVector::kMaxOsrUrgency);
        static_assert!(FeedbackVector::kMaxOsrUrgency <= OsrUrgencyBits::kMax);
        self.set_osr_state(OsrUrgencyBits::update(self.osr_state(), urgency));
    }

    pub fn reset_osr_urgency(&self) {
        self.set_osr_urgency(0);
    }

    pub fn request_osr_at_next_opportunity(&self) {
        self.set_osr_urgency(kMaxOsrUrgency);
    }

    pub fn reset_osr_state(&self) {
        self.set_osr_state(0);
    }

    pub fn maybe_has_optimized_osr_code(&self) -> bool {
        self.maybe_has_maglev_osr_code() || self.maybe_has_turbofan_osr_code()
    }

    pub fn maybe_has_maglev_osr_code(&self) -> bool {
        MaybeHasMaglevOsrCodeBit::decode(self.osr_state())
    }

    pub fn maybe_has_turbofan_osr_code(&self) -> bool {
        MaybeHasTurbofanOsrCodeBit::decode(self.osr_state())
    }

    pub fn set_maybe_has_optimized_osr_code(&self, value: bool, code_kind: CodeKind) {
        if code_kind == CodeKind::MAGLEV {
            todo!();
            //CHECK(v8_flags.maglev_osr);
            self.set_osr_state(MaybeHasMaglevOsrCodeBit::update(self.osr_state(), value));
        } else {
            todo!();
            //CHECK_EQ(code_kind, CodeKind::TURBOFAN_JS);
            self.set_osr_state(MaybeHasTurbofanOsrCodeBit::update(self.osr_state(), value));
        }
    }

    pub fn interrupt_budget_reset_by_ic_change(&self) -> bool {
        InterruptBudgetResetByIcChangeBit::decode(self.flags())
    }

    pub fn set_interrupt_budget_reset_by_ic_change(&self, value: bool) {
        self.set_flags(InterruptBudgetResetByIcChangeBit::update(self.flags(), value));
    }

    pub fn was_once_deoptimized(&self) -> bool {
        self.invocation_count_before_stable(kRelaxedLoad) == kInvocationCountBeforeStableDeoptSentinel
    }

    pub fn set_was_once_deoptimized(&self) {
        self.set_invocation_count_before_stable(kInvocationCountBeforeStableDeoptSentinel, kRelaxedStore);
    }

    pub fn tiering_in_progress(&self) -> bool {
        TieringInProgressBit::decode(self.flags())
    }

    pub fn tiering_state(&self) -> TieringState {
        TieringStateBits::decode(self.flags())
    }

    pub fn reset_tiering_state(&self) {
        self.set_tiering_state(TieringState::kNone);
    }

    pub fn log_next_execution(&self) -> bool {
        LogNextExecutionBit::decode(self.flags())
    }

    pub fn set_log_next_execution(&self, value: bool) {
        self.set_flags(LogNextExecutionBit::update(self.flags(), value));
    }

    pub fn optimized_code(&self, isolate: IsolateForSandbox) -> Tagged<Code> {
        todo!()
    }

    pub fn has_optimized_code(&self) -> bool {
        todo!()
    }

    pub fn maybe_has_maglev_code(&self) -> bool {
        MaybeHasMaglevCodeBit::decode(self.flags())
    }

    pub fn set_maybe_has_maglev_code(&self, value: bool) {
        self.set_flags(MaybeHasMaglevCodeBit::update(self.flags(), value));
    }

    pub fn maybe_has_turbofan_code(&self) -> bool {
        MaybeHasTurbofanCodeBit::decode(self.flags())
    }

    pub fn set_maybe_has_turbofan_code(&self, value: bool) {
        self.set_flags(MaybeHasTurbofanCodeBit::update(self.flags(), value));
    }

    pub fn get_optimized_osr_code(
        isolate: &mut Isolate,
        slot: FeedbackSlot,
    ) -> Option<Tagged<Code>> {
        let maybe_code = Self::get(isolate, slot);
        if maybe_code.is_cleared() {
            return None;
        }

        todo!()
    }

    pub fn to_slot(index: intptr_t) -> FeedbackSlot {
        todo!()
    }

    pub fn is_of_legacy_type(value: Tagged<MaybeObject>) -> bool {
        todo!()
    }

    pub fn get(&self, slot: FeedbackSlot) -> Tagged<MaybeObject> {
        let value = self.raw_feedback_slots(self.get_index(slot), kRelaxedLoad);
        todo!()
    }

    pub fn get(&self, cage_base: PtrComprCageBase, slot: FeedbackSlot) -> Tagged<MaybeObject> {
        let value = self.raw_feedback_slots(cage_base, self.get_index(slot), kRelaxedLoad);
        todo!()
    }

    pub fn get_closure_feedback_cell(
        isolate: &mut Isolate,
        index: int,
    ) -> DirectHandle<FeedbackCell> {
        todo!()
    }

    pub fn closure_feedback_cell(&self, index: int) -> Tagged<FeedbackCell> {
        todo!()
    }

    pub fn synchronized_get(&self, slot: FeedbackSlot) -> Tagged<MaybeObject> {
        todo!()
    }

    pub fn synchronized_set(&self, slot: FeedbackSlot, value: Tagged<MaybeObject>, mode: WriteBarrierMode) {
        todo!()
    }

    pub fn set(&self, slot: FeedbackSlot, value: Tagged<MaybeObject>, mode: WriteBarrierMode) {
        todo!()
    }

    pub fn slots_start(&self) -> MaybeObjectSlot {
        todo!()
    }

    pub fn uninitialized_sentinel(isolate: &mut Isolate) -> DirectHandle<Symbol> {
        todo!()
    }

    pub fn megamorphic_sentinel(isolate: &mut Isolate) -> Handle<Symbol> {
        todo!()
    }

    pub fn mega_dom_sentinel(isolate: &mut Isolate) -> DirectHandle<Symbol> {
        todo!()
    }

    pub fn raw_uninitialized_sentinel(isolate: &mut Isolate) -> Tagged<Symbol> {
        todo!()
    }
}

pub enum FeedbackSlotKind {
    kInvalid,
    kForIn,
    kInstanceOf,
    kTypeOf,
    kCompareOp,
    kBinaryOp,
    kLiteral,
    kJumpLoop,
    kCall,
    kCloneObject,
    kLoadProperty,
    kLoadGlobalInsideTypeof,
    kLoadGlobalNotInsideTypeof,
    kLoadKeyed,
    kHasKeyed,
    kSetNamedSloppy,
    kSetNamedStrict,
    kDefineNamedOwn,
    kDefineKeyedOwn,
    kStoreGlobalSloppy,
    kStoreGlobalStrict,
    kSetKeyedSloppy,
    kSetKeyedStrict,
    kStoreInArrayLiteral,
    kDefineKeyedOwnPropertyInLiteral,
}

pub struct FeedbackMetadataIterator<'a> {
    metadata_: &'a FeedbackMetadata,
    cur_slot_: FeedbackSlot,
    next_slot_: FeedbackSlot,
    slot_kind_: FeedbackSlotKind,
}

impl<'a> FeedbackMetadataIterator<'a> {
    pub fn new(metadata: &'a FeedbackMetadata) -> Self {
        FeedbackMetadataIterator {
            metadata_: metadata,
            cur_slot_: FeedbackSlot { dummy: 0, phantom: PhantomData },
            next_slot_: FeedbackSlot { dummy: 0, phantom: PhantomData },
            slot_kind_: FeedbackSlotKind::kInvalid,
        }
    }

    pub fn metadata(&self) -> &'a FeedbackMetadata {
        self.metadata_
    }

    pub fn kind(&self) -> FeedbackSlotKind {
        self.slot_kind_
    }

    pub fn has_next(&self) -> bool {
        self.next_slot_.to_int() < self.metadata().slot_count()
    }

    pub fn next(&mut self) -> FeedbackSlot {
        DCHECK(self.has_next());
        self.cur_slot_ = self.next_slot_;
        self.slot_kind_ = self.metadata().get_kind(self.cur_slot_);
        self.next_slot_ = FeedbackSlot::new(self.next_slot_.to_int() + self.entry_size());
        self.cur_slot_
    }

    pub fn entry_size(&self) -> int {
        FeedbackMetadata::GetSlotSize(self.kind())
    }
}

impl FeedbackMetadata {
    pub fn get_kind(&self, _cur_slot_: FeedbackSlot) -> FeedbackSlotKind {
        todo!()
    }
}

pub struct NexusConfig<'a> {
    isolate_: &'a mut Isolate,
    local_heap_: &'a mut Heap,
}

impl<'a> NexusConfig<'a> {
    pub fn new(isolate: &'a mut Isolate, local_heap: &'a mut Heap) -> Self {
        NexusConfig {
            isolate_: isolate,
            local_heap_: local_heap,
        }
    }

    pub fn mode(&self) -> Mode {
        Mode::MainThread
    }

    pub fn can_write(&self) -> bool {
        true
    }

    pub fn isolate(&self) -> &mut Isolate {
        self.isolate_
    }

    pub fn local_heap(&self) -> &mut Heap {
        self.local_heap_
    }

    pub fn new_handle<T>(&self, object: Tagged<T>) -> Handle<T> {
        if self.mode() == Mode::MainThread {
            todo!()
        } else {
            todo!()
        }
    }

    pub fn get_feedback(&self, vector: Tagged<FeedbackVector>, slot: FeedbackSlot) -> Tagged<MaybeObject> {
        vector.synchronized_get(slot)
    }

    pub fn set_feedback(&self, vector: Tagged<FeedbackVector>, slot: FeedbackSlot, feedback: Tagged<MaybeObject>, mode: WriteBarrierMode) {
        todo!()
    }

    pub fn get_feedback_pair(&self, vector: Tagged<FeedbackVector>, slot: FeedbackSlot) -> (Tagged<MaybeObject>, Tagged<MaybeObject>) {
        todo!()
    }

    pub fn set_feedback_pair(&self, vector: Tagged<FeedbackVector>, slot: FeedbackSlot, feedback: Tagged<MaybeObject>, mode: WriteBarrierMode, feedback_extra: Tagged<MaybeObject>, mode_extra: WriteBarrierMode) {
        todo!()
    }
}

pub enum Mode {
    MainThread,
    BackgroundThread,
}

pub struct FeedbackNexus<'a> {
    config_: &'a NexusConfig<'a>,
    vector_: Tagged<FeedbackVector>,
    slot_: FeedbackSlot,
    kind_: FeedbackSlotKind,
    feedback_cache_: Option<(MaybeObjectHandle, MaybeObjectHandle)>,
}

impl<'a> FeedbackNexus<'a> {
    pub fn new(config: &'a NexusConfig<'a>, vector: Tagged<FeedbackVector>, slot: FeedbackSlot, kind: FeedbackSlotKind) -> Self {
        FeedbackNexus {
            config_: config,
            vector_: vector,
            slot_: slot,
            kind_: kind,
            feedback_cache_: None,
        }
    }

    pub fn config(&self) -> &'a NexusConfig<'a> {
        self.config_
    }

    pub fn vector(&self) -> Tagged<FeedbackVector> {
        self.vector_
    }

    pub fn slot(&self) -> FeedbackSlot {
        self.slot_
    }

    pub fn kind(&self) -> FeedbackSlotKind {
        self.kind_
    }

    pub fn uninitialized_sentinel(&self) -> Tagged<MaybeObject> {
        *FeedbackVector::UninitializedSentinel(self.config().isolate())
    }

    pub fn megamorphic_sentinel(&self) -> Tagged<MaybeObject> {
        *FeedbackVector::MegamorphicSentinel(self.config().isolate())
    }

    pub fn mega_dom_sentinel(&self) -> Tagged<MaybeObject> {
        *FeedbackVector::MegaDOMSentinel(self.config().isolate())
    }

    pub fn from_handle(&self, slot: MaybeObjectDirectHandle) -> Tagged<MaybeObject> {
        if slot.is_null() {
            ClearedValue(self.config().isolate())
        } else {
            *slot
        }
    }

    pub fn to_handle(&self, value: Tagged<MaybeObject>) -> MaybeObjectHandle {
        if value.is_cleared() {
            MaybeObjectHandle::null()
        } else {
            MaybeObjectHandle::new(self.config().new_handle(value))
        }
    }

    pub fn get_feedback(&self) -> Tagged<MaybeObject> {
        let pair = self.get_feedback_pair();
        pair.first
    }

    pub fn get_feedback_extra(&self) -> Tagged<MaybeObject> {
        let pair = self.get_feedback_pair();
        pair.second
    }

    pub fn get_feedback_pair(&self) -> (Tagged<MaybeObject>, Tagged<MaybeObject>) {
        if self.config().mode() == Mode::BackgroundThread && self.feedback_cache_.is_some() {
            let cache = self.feedback_cache_.as_ref().unwrap();
            return (self.from_handle(cache.0), self.from_handle(cache.1));
        }
        let pair = if FeedbackMetadata::GetSlotSize(self.kind()) == 2 {
            self.config().get_feedback_pair(self.vector(), self.slot())
        } else {
            (self.config().get_feedback(self.vector(), self.slot()), Tagged::<MaybeObject>::null())
        };
        if self.config().mode() == Mode::BackgroundThread && self.feedback_cache_.is_none() {
            self.feedback_cache_ = Some((self.to_handle(pair.0), self.to_handle(pair.1)));
        }
        pair
    }

    pub fn set_feedback<FeedbackType>(&self, feedback: Tagged<FeedbackType>, mode: WriteBarrierMode) {
        self.config().set_feedback(self.vector(), self.slot(), feedback, mode);
    }

    pub fn set_feedback_pair<FeedbackType, FeedbackExtraType>(&self, feedback: Tagged<FeedbackType>, mode: WriteBarrierMode, feedback_extra: Tagged<FeedbackExtraType>, mode_extra: WriteBarrierMode) {
        self.config().set_feedback_pair(self.vector(), self.slot(), feedback, mode, feedback_extra, mode_extra);
    }

    pub fn iterate_maps_with_uncleared_handler<F>(&self, function: F) where F: Fn(Handle<Map>) {
        todo!()
    }
}

pub struct MaybeObjectHandle {
    handle: Option<Handle<MaybeObject>>,
}

impl MaybeObjectHandle {
    pub fn new(handle: Handle<MaybeObject>) -> Self {
        MaybeObjectHandle { handle: Some(handle) }
    }

    pub fn null() -> Self {
        MaybeObjectHandle { handle: None }
    }

    pub fn is_null(&self) -> bool {
        self.handle.is_none()
    }

    pub fn handle(&self) -> Option<&Handle<MaybeObject>> {
        self.handle.as_ref()
    }
}

pub struct FeedbackIterator<'a> {
    nexus: &'a FeedbackNexus<'a>,
}

impl<'a> FeedbackIterator<'a> {
    pub fn new(nexus: &'a FeedbackNexus) -> Self {
        FeedbackIterator { nexus }
    }

    pub fn done(&self) -> bool {
        todo!()
    }

    pub fn advance(&mut self) {
        todo!()
    }

    pub fn map(&self) -> Tagged<Map> {
        todo!()
    }

    pub fn handler(&self) -> Tagged<MaybeObject> {
        todo!()
    }
}

pub type int = i32;
pub type intptr_t = isize;
pub type int32_t = i32;

pub struct PtrComprCageBase {}
impl PtrComprCageBase {
    pub fn dummy(&self) -> i32 {0}
}

pub struct AcquireLoadTag {}
pub struct RelaxedLoadTag {}
pub struct RelaxedStoreTag {}
