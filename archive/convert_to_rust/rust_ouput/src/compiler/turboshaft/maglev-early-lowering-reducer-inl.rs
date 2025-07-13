// Converted from V8 C++ source files:
// Header: maglev-early-lowering-reducer-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use v8::HandleScope;
use v8::{Local, Context, Value};
use crate::compiler::turboshaft::assembler::Label;
use crate::compiler::turboshaft::index::OpIndex;
use crate::compiler::turboshaft::representations::MemoryRepresentation;
use crate::objects::contexts::ContextSidePropertyCell;

use crate::compiler::feedback_source::FeedbackSource;
use crate::compiler::globals::DeoptimizeReason;
use crate::compiler::turboshaft::assembler::*;
use crate::compiler::turboshaft::index::*;
use crate::compiler::turboshaft::representations::*;
use crate::deoptimizer::deoptimize_reason::DeoptimizeReason as DeoptimizeReasonEnum;
use crate::objects::contexts::*;
use crate::objects::instance_type_inl::*;

use crate::compiler::map_inference::MapRef;
use crate::compiler::wasm_js_lowering_reducer::LazyDeoptOnThrow;
use crate::execution::isolate::Isolate;
use crate::objects::heap_object::HeapObject;
use crate::objects::map::Map;
use crate::compiler::js_heap_broker::JSHeapBroker;
use crate::objects::string::String;
use crate::compiler::turboshaft::operations::HeapObjectRef;
use crate::compiler::wasm_compiler_definitions::base;
use crate::objects::fixed_array::FixedArray;
use crate::objects::js_array::JSArray;
use crate::objects::property_array::PropertyArray;
use crate::objects::js_generator_object::JSGeneratorObject;
use crate::compiler::common_operator::CheckForMinusZeroMode;

// Define a trait for the 'Next' type.  This allows us to define
// MaglevEarlyLoweringReducer<Next> where Next is some other reducer.
trait NextTrait {
    fn data(&self) -> &ReducerData;
}

struct ReducerData {
    isolate: *mut Isolate,
    broker: *mut JSHeapBroker,
}

macro_rules! IF {
    ($condition:expr) => {
        if $condition {
            // Execute the code within the 'if' block
        }
    };
}

macro_rules! IF_NOT {
    ($condition:expr) => {
        if !$condition {
            // Execute the code within the 'if not' block
        }
    };
}

macro_rules! UNLIKELY {
    ($e:expr) => {
        $e
    };
}

macro_rules! LIKELY {
    ($e:expr) => {
        $e
    };
}

macro_rules! GOTO {
    ($label:ident, $($arg:expr),*) => {
        $label.goto($($arg),*);
        return;
    };
}

macro_rules! GOTO_IF {
    ($condition:expr, $label:ident, $($arg:expr),*) => {
        if $condition {
            $label.goto($($arg),*);
            return;
        }
    };
}

macro_rules! BIND {
    ($label:ident, $result:ident) => {
        // This should somehow define the binding.
    };
}

macro_rules! BIND_LOOP {
    ($label:ident, $var:ident) => {

    };
}

pub struct ScopedVar<T> {
    value: T,
}

impl<T> ScopedVar<T> {
    pub fn new(value: T) -> Self {
        ScopedVar { value }
    }
}

// Implementing Deref to automatically access the value within ScopedVar
impl<T> Deref for ScopedVar<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[derive(Debug, Clone)]
pub struct InternalizedStringRef {
    object: Rc<HeapObject>,
}

impl InternalizedStringRef {
    pub fn new(object: Rc<HeapObject>) -> Self {
        InternalizedStringRef { object }
    }

    pub fn object(&self) -> Rc<HeapObject> {
        self.object.clone()
    }
}

// Mock implementation for Map, Map::Bits3, RootIndex, etc.
// Replace with actual V8 data structures if available
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Word32(u32);
impl Word32 {
    fn bitwise_and(self, other: Self) -> Self {
        Word32(self.0 & other.0)
    }

    fn equal(self, other: Self) -> bool {
        self.0 == other.0
    }

    fn not_equal(self, other: Self) -> bool {
        self.0 != other.0
    }

    fn bitwise_or(self, other: Self) -> Self {
        Word32(self.0 | other.0)
    }

    fn sub(self, other: Self) -> Self {
        Word32(self.0 - other.0)
    }

    fn add(self, other: Self) -> Self {
        Word32(self.0 + other.0)
    }
    fn shift_left(self, shift: u32) -> Self {
        Word32(self.0 << shift)
    }
}

impl From<u32> for Word32 {
    fn from(val: u32) -> Self {
        Word32(val)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Boolean(bool);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Float64(f64);

// Mock implementation for Isolate and LocalIsolate
#[derive(Debug)]
pub struct LocalIsolate {
    factory: Rc<LocalFactory>,
}

impl LocalIsolate {
    pub fn new(factory: Rc<LocalFactory>) -> Self {
        LocalIsolate { factory }
    }
    pub fn factory(&self) -> &Rc<LocalFactory> {
        &self.factory
    }
}

#[derive(Debug)]
pub struct LocalFactory {
    true_value: Rc<HeapObject>,
    false_value: Rc<HeapObject>,
    undefined_value: Rc<HeapObject>,
    null_value: Rc<HeapObject>,
    property_array_map: Rc<Map>,
    heap_number_map: Rc<Map>,
}

impl LocalFactory {
    pub fn new() -> Self {
        LocalFactory {
            true_value: Rc::new(HeapObject {}),
            false_value: Rc::new(HeapObject {}),
            undefined_value: Rc::new(HeapObject {}),
            null_value: Rc::new(HeapObject {}),
            property_array_map: Rc::new(Map {}),
            heap_number_map: Rc::new(Map {}),
        }
    }
    pub fn true_value(&self) -> &Rc<HeapObject> {
        &self.true_value
    }
    pub fn false_value(&self) -> &Rc<HeapObject> {
        &self.false_value
    }
    pub fn undefined_value(&self) -> &Rc<HeapObject> {
        &self.undefined_value
    }
    pub fn null_value(&self) -> &Rc<HeapObject> {
        &self.null_value
    }
    pub fn property_array_map(&self) -> &Rc<Map> {
        &self.property_array_map
    }
    pub fn heap_number_map(&self) -> &Rc<Map> {
        &self.heap_number_map
    }
}

struct Flags {
    script_context_mutable_heap_int32: bool,
    script_context_mutable_heap_number: bool,
}

impl Flags {
    fn new() -> Self {
        Flags {
            script_context_mutable_heap_int32: false,
            script_context_mutable_heap_number: false,
        }
    }
}

#[derive(Clone)]
pub struct Smi(i32);
impl Smi {
    fn from_int(value: i32) -> Self {
        Smi(value)
    }
}

// Define the MaglevEarlyLoweringReducer struct, which takes a generic type
// 'Next' that must implement the 'NextTrait'.
pub struct MaglevEarlyLoweringReducer<Next: NextTrait> {
    next: Next,
    isolate_: *mut Isolate,
    local_isolate_: Rc<LocalIsolate>,
    broker_: *mut JSHeapBroker,
    factory_: Rc<LocalFactory>,
    flags: Flags,
}

impl<Next: NextTrait> MaglevEarlyLoweringReducer<Next> {
    // The reducer boilerplate macro.  We'll implement it as a function for now.
    fn turboshaft_reducer_boilerplate(_name: &str) -> String {
        "MaglevEarlyLoweringReducer".to_string()
    }

    pub fn new(next: Next, isolate: *mut Isolate, broker: *mut JSHeapBroker) -> Self {
        let local_isolate_ = Rc::new(LocalIsolate::new(Rc::new(LocalFactory::new())));
        let factory_ = local_isolate_.factory().clone();
        MaglevEarlyLoweringReducer {
            next,
            isolate_: isolate,
            local_isolate_: local_isolate_,
            broker_: broker,
            factory_: factory_,
            flags: Flags::new(),
        }
    }

    fn data(&self) -> &ReducerData {
        self.next.data()
    }

    fn check_instance_type(&self, input: V<Object>, frame_state: V<FrameState>,
                           feedback: &FeedbackSource,
                           first_instance_type: InstanceType,
                           last_instance_type: InstanceType, check_smi: bool) {
        if check_smi {
            self.deoptimize_if(self.is_smi(&input), frame_state,
                                 DeoptimizeReason::kWrongInstanceType, feedback);
        }

        let map = self.load_map_field(&input);

        if first_instance_type == last_instance_type {
            if self.unique_map_of_instance_type(first_instance_type).is_some() {
                let expected_index = self.unique_map_of_instance_type(first_instance_type).unwrap();
            }
            let instance_type = self.load_instance_type_field(&map);
            self.deoptimize_if_not(self.word32_equal(instance_type, first_instance_type as u32),
                                     frame_state, DeoptimizeReason::kWrongInstanceType, feedback);
        } else {
            self.deoptimize_if_not(self.check_instance_type_is_in_range(&map, first_instance_type,
                                                                           last_instance_type),
                                     frame_state, DeoptimizeReason::kWrongInstanceType, feedback);
        }
    }

    fn checked_internalized_string(&self, object: V<Object>, frame_state: V<FrameState>, check_smi: bool,
                                  feedback: &FeedbackSource) -> V<InternalizedString> {
        if check_smi {
            self.deoptimize_if(self.is_smi(&object), frame_state, DeoptimizeReason::kSmi,
                                 feedback);
        }

        let mut done: Label<InternalizedString> = Label::new(self);
        let map = self.load_map_field(&object);
        let instance_type = self.load_instance_type_field(&map);

        if UNLIKELY(self.word32_bitwise_and(instance_type,
                                                 (kIsNotStringMask as u32 | kIsNotInternalizedMask as u32)) != Word32::from(0))) {
            self.deoptimize_if(self.word32_bitwise_and(instance_type, kIsNotStringMask as u32) != Word32::from(0),
                                 frame_state, DeoptimizeReason::kWrongMap, feedback);
            self.deoptimize_if_not(self.word32_bitwise_and(instance_type, kThinStringTagBit as u32) != Word32::from(0),
                                     frame_state, DeoptimizeReason::kWrongMap, feedback);

            let intern_string = self.template_load_field::<InternalizedString>(
                &object, AccessBuilder::ForThinStringActual());
            GOTO!(done, intern_string);
        } else {
            GOTO!(done, V::<InternalizedString>::Cast(object));
        }

        BIND!(done, result);
        result
    }

    fn check_value_equals_string(&self, object: V<Object>, value: InternalizedStringRef,
                                  frame_state: V<FrameState>,
                                  feedback: &FeedbackSource) {
        IF_NOT (LIKELY(self.tagged_equal(&object, &value.object()))) {
            self.deoptimize_if_not(self.object_is_string(&object), frame_state,
                                     DeoptimizeReason::kNotAString, feedback);
            let is_same_string_bool =
                self.string_equal(V::<String>::Cast(object),
                                  self.template_heap_constant::<String>(value.object.clone()));
            self.deoptimize_if(
                self.root_equal(is_same_string_bool, RootIndex::kFalseValue, self.isolate_),
                frame_state, DeoptimizeReason::kWrongValue, feedback);
        }
    }

    fn check_construct_result(&self, construct_result: V<Object>,
                                implicit_receiver: V<Object>) -> V<Object> {
        let mut done: Label<Object> = Label::new(self);

        GOTO_IF!(self.root_equal(construct_result, RootIndex::kUndefinedValue, self.isolate_),
                 done, implicit_receiver);
        GOTO_IF!(self.is_smi(&construct_result), done, implicit_receiver);
        GOTO_IF!(self.js_any_is_not_primitive(V::<HeapObject>::Cast(construct_result)), done,
                 construct_result);
        GOTO!(done, implicit_receiver);

        BIND!(done, result);
        result
    }

    fn load_script_context_side_data(&self, script_context: V<Context>, index: i32) -> V<Object> {
        let side_table = self.template_load_tagged_field::<FixedArray>(
            &script_context,
            Context::OffsetOfElementAt(Context::CONTEXT_SIDE_TABLE_PROPERTY_INDEX));
        self.load_tagged_field(&side_table,
                                 FixedArray::OffsetOfElementAt(
                                     (index - Context::MIN_CONTEXT_EXTENDED_SLOTS) as usize))
    }

    fn load_script_context_property_from_side_data(&self, side_data: V<Object>) -> V<Object> {
        let property = ScopedVar::new(side_data);
        IF_NOT (self.is_smi(&side_data)) {
        }
        *property
    }

    fn load_heap_number_from_script_context(&self, script_context: V<Context>,
                                             index: i32,
                                             heap_number: V<HeapNumber>) -> V<Object> {
        let data = self.load_script_context_side_data(script_context, index);
        let property = self.load_script_context_property_from_side_data(data);
        let result = ScopedVar::new(heap_number);
        let mut done: Label<()> = Label::new(self);
        if self.flags.script_context_mutable_heap_int32 {
            IF (self.tagged_equal(
                    property,
                    self.smi_constant(ContextSidePropertyCell::MutableInt32()))) {
            }
            GOTO!(done);
        }
        IF (self.tagged_equal(
                property,
                self.smi_constant(ContextSidePropertyCell::MutableHeapNumber()))) {
        }
        GOTO!(done);
        BIND!(done);
        *result
    }

    fn store_script_context_slow_path(&self, script_context: V<Context>,
                                      old_value: V<Object>, new_value: V<Object>,
                                      side_data: V<Object>,
                                      frame_state: V<FrameState>,
                                      feedback: &FeedbackSource,
                                      done: &mut Label<>) {
        self.deoptimize_if(
            self.root_equal(side_data, RootIndex::kUndefinedValue, self.isolate_),
            frame_state, DeoptimizeReason::kWrongValue, feedback);
        let property = self.load_script_context_property_from_side_data(side_data);
        self.deoptimize_if(
            self.tagged_equal(property, self.smi_constant(ContextSidePropertyCell::Const())),
            frame_state, DeoptimizeReason::kWrongValue, feedback);

        if self.flags.script_context_mutable_heap_number {
            IF (self.tagged_equal(property, self.smi_constant(ContextSidePropertyCell::SmiMarker()))) {
                self.deoptimize_if_not(self.is_smi(&new_value), frame_state,
                                         DeoptimizeReason::kWrongValue, feedback);
            } else {
                if self.flags.script_context_mutable_heap_int32 {
                    IF (self.tagged_equal(property,
                                           self.smi_constant(ContextSidePropertyCell::MutableInt32()))) {
                        let number_value = ScopedVar::new(Word32(0));
                        IF (self.is_smi(&new_value)) {
                        } else {
                            let map = self.load_map_field(&new_value);
                            self.deoptimize_if_not(
                                self.tagged_equal(map, self.heap_constant(self.factory_.heap_number_map().clone())),
                                frame_state, DeoptimizeReason::kWrongValue, feedback);
                        }
                        GOTO!(done);
                    }
                }
                let number_value = ScopedVar::new(Float64(0.0));
                IF (self.is_smi(&new_value)) {
                } else {
                    let map = self.load_map_field(&new_value);
                    self.deoptimize_if_not(
                        self.tagged_equal(map, self.heap_constant(self.factory_.heap_number_map().clone())),
                        frame_state, DeoptimizeReason::kWrongValue, feedback);
                }
                GOTO!(done);
            }
        }
    }

    fn check_derived_construct_result(&self, construct_result: V<Object>,
                                        frame_state: V<FrameState>,
                                        native_context: V<NativeContext>,
                                        lazy_deopt_on_throw: LazyDeoptOnThrow) {
        let mut do_throw: Label<()> = Label::new(self);

        GOTO_IF!(self.is_smi(&construct_result), do_throw);

        IF_NOT (self.js_any_is_not_primitive(V::<HeapObject>::Cast(construct_result))) {
            GOTO!(do_throw);
            BIND!(do_throw,);
        }
    }

    fn update_js_array_length(&self, length_raw: V<Word32>, object: V<JSArray>,
                                index: V<Word32>) -> V<Smi> {
        let mut done: Label<Smi> = Label::new(self);
        IF (self.uint32_less_than(index, length_raw)) {
            GOTO!(done, self.tag_smi(length_raw));
        } else {
            let new_length_raw = self.word32_add(index, 1);
            let new_length_tagged = self.tag_smi(new_length_raw);
            self.store(object, new_length_tagged, StoreOp::Kind::TaggedBase(),
                       MemoryRepresentation::TaggedSigned(),
                       WriteBarrierKind::kNoWriteBarrier, JSArray::kLengthOffset);
            GOTO!(done, new_length_tagged);
        }

        BIND!(done, length_tagged);
        length_tagged
    }

    fn transition_multiple_elements_kind(&self,
                                            object: V<Object>, map: V<Map>,
                                            transition_sources: &Vec<MapRef>,
                                            transition_target: MapRef) -> V<Map> {
        let mut end: Label<Map> = Label::new(self);

        self.transition_elements_kind(object, map, transition_sources, transition_target,
                                        end);
        GOTO!(end, map);
        BIND!(end, result);
        result
    }

    fn transition_elements_kind(&self,
                                  object: V<Object>, map: V<Map>,
                                  transition_sources: &Vec<MapRef>,
                                  transition_target: MapRef, end: &mut Label<Map>) {
        let target_map = self.heap_constant(transition_target.object());

        for transition_source in transition_sources {
            let is_simple = self.is_simple_map_change_transition(
                transition_source.elements_kind(), transition_target.elements_kind());
            IF (self.tagged_equal(map, self.heap_constant(transition_source.object()))) {
                if is_simple {
                    self.store_field(object, AccessBuilder::ForMap(), target_map);
                } else {
                }
                GOTO!(end, target_map);
            }
        }
    }

    fn js_any_is_not_primitive(&self, heap_object: V<HeapObject>) -> V<Word32> {
        let map = self.load_map_field(&heap_object);
        return self.uint32_less_than_or_equal(
            InstanceTypeChecker::kNonJsReceiverMapLimit,
            self.truncate_word_ptr_to_word32(self.bitcast_tagged_to_word_ptr(&map)));
    }

    fn has_in_prototype_chain(&self, object: V<Object>, prototype: HeapObjectRef,
                                frame_state: V<FrameState>,
                                native_context: V<NativeContext>,
                                lazy_deopt_on_throw: LazyDeoptOnThrow) -> V<Boolean> {
        let mut done: Label<Boolean> = Label::new(self);

        let true_bool = self.heap_constant(self.factory_.true_value().clone());
        let false_bool = self.heap_constant(self.factory_.false_value().clone());
        let target_proto = self.heap_constant(prototype.object());

        GOTO_IF!(self.is_smi(&object), done, false_bool);

        let mut loop_: LoopLabel<Map> = LoopLabel::new(self);
        GOTO!(loop_, self.load_map_field(&object));

        BIND_LOOP!(loop_, map) {
            let mut object_is_direct: Label<()> = Label::new(self);

            IF (UNLIKELY(self.check_instance_type_is_in_range(map, FIRST_TYPE,
                                                             LAST_SPECIAL_RECEIVER_TYPE))) {
                let mut call_runtime: Label<()> = Label::new(self);
                let instance_type = self.load_instance_type_field(map);

                GOTO_IF!(self.word32_equal(instance_type, JS_PROXY_TYPE as u32), call_runtime);

                let bitfield =
                    self.template_load_field::<Word32>(map, AccessBuilder::ForMapBitField());
                let mask = Map::Bits1::HasNamedInterceptorBit::kMask |
                           Map::Bits1::IsAccessCheckNeededBit::kMask;
                GOTO_IF_NOT(self.word32_bitwise_and(bitfield, mask as u32) != Word32::from(0), object_is_direct);
                GOTO!(call_runtime);

                BIND!(call_runtime);
                //GOTO!(done, self.call_runtime_has_in_prototype_chain(
                 //      self.isolate_, frame_state, native_context,
                 //      lazy_deopt_on_throw, object, target_proto));
                 //TODO: Fix this call runtime
            }
            GOTO!(object_is_direct);

            BIND!(object_is_direct);
            let proto = self.template_load_field::<HeapObject>(
                map, AccessBuilder::ForMapPrototype());
            GOTO_IF!(self.root_equal(proto, RootIndex::kNullValue, self.isolate_), done,
                     false_bool);
            GOTO_IF!(self.tagged_equal(proto, target_proto), done, true_bool);

            GOTO!(loop_, self.load_map_field(proto));
        }

        BIND!(done, result);
        result
    }

    fn migrate_map_if_needed(&self, object: V<HeapObject>, map: V<Map>,
                                frame_state: V<FrameState>,
                                feedback: &FeedbackSource) -> V<Map> {
        let result = ScopedVar::new(map);

        let bitfield3 =
            self.template_load_field::<Word32>(map, AccessBuilder::ForMapBitField3());
        IF (UNLIKELY(self.word32_bitwise_and(bitfield3,
                                            Map::Bits3::IsDeprecatedBit::kMask as u32) != Word32::from(0))) {
            //let object_or_smi = self.call_runtime_try_migrate_instance(
             //   self.isolate_, self.no_context_constant(), object);
            //TODO: Fix this call runtime
            let object_or_smi = V::<Object>::default();
            self.deoptimize_if(self.object_is_smi(&object_or_smi), frame_state,
                                 DeoptimizeReason::kInstanceMigrationFailed, feedback);
            *result
        }

        *result
    }

    fn extend_properties_backing_store(&self,
                                        old_property_array: V<PropertyArray>, object: V<JSObject>, old_length: i32,
                                        frame_state: V<FrameState>, feedback: &FeedbackSource) -> V<PropertyArray> {
        let new_length = old_length + JSObject::kFieldsAdded;
        let new_property_array =
            self.template_allocate::<PropertyArray>(
                self.int_ptr_constant(PropertyArray::SizeFor(new_length) as i64),
                AllocationType::kYoung);
        self.initialize_field(new_property_array, AccessBuilder::ForMap(),
                                 self.heap_constant(self.factory_.property_array_map().clone()));

        for i in 0..old_length {
            let old_value = self.template_load_field::<Object>(
                old_property_array, AccessBuilder::ForPropertyArraySlot(i as usize));
            self.initialize_field(new_property_array,
                                     AccessBuilder::ForPropertyArraySlot(i as usize), old_value);
        }

        let undefined = self.heap_constant(self.factory_.undefined_value().clone());
        for i in 0..JSObject::kFieldsAdded {
            self.initialize_field(new_property_array,
                                     AccessBuilder::ForPropertyArraySlot((old_length + i) as usize), undefined);
        }

        let hash = ScopedVar::new(Word32(0));
        if old_length == 0 {
            let hash_obj = self.template_load_field::<Object>(
                object, AccessBuilder::ForJSObjectPropertiesOrHash());
            IF (self.is_smi(&hash_obj)) {
            } else {
                //hash = self.word32_constant(PropertyArray::kNoHashSentinel);
                let mut val: Word32 = Word32::from(PropertyArray::kNoHashSentinel);
                *hash = val;
            }
        } else {
            //let hash_smi = self.template_load_field::<Smi>(
            //    old_property_array, AccessBuilder::ForPropertyArrayLengthAndHash());
            //*hash = self.word32_bitwise_and(self.untag_smi(V::<Smi>::default()),
            //                             PropertyArray::HashField::kMask as u32);
        }

        let length_and_hash = self.word32_bitwise_or(*hash, new_length as u32);
        self.initialize_field(new_property_array,
                                 AccessBuilder::ForPropertyArrayLengthAndHash(),
                                 self.tag_smi(length_and_hash));

        let initialized_new_property_array =
            self.finish_initialization(new_property_array);

        self.store_field(object, AccessBuilder::ForJSObjectPropertiesOrHash(),
                           initialized_new_property_array);

        initialized_new_property_array
    }

    fn generator_store(&self, context: V<Context>, generator: V<JSGeneratorObject>,
                         parameters_and_registers: Vec<OpIndex>,
                         suspend_id: i32, bytecode_offset: i32) {
        let array = self.template_load_tagged_field::<FixedArray>(
            generator, JSGeneratorObject::kParametersAndRegistersOffset);
        for i in 0..parameters_and_registers.len() {
            self.store(array, parameters_and_registers[i], StoreOp::Kind::TaggedBase(),
                       MemoryRepresentation::AnyTagged(),
                       WriteBarrierKind::kFullWriteBarrier,
                       FixedArray::OffsetOfElementAt(i));
        }
        self.store(generator, self.smi_constant(Smi::from_int(suspend_id)),
                   StoreOp::Kind::TaggedBase(), MemoryRepresentation::TaggedSigned(),
                   WriteBarrierKind::kNoWriteBarrier,
                   JSGeneratorObject::kContinuationOffset);
        self.store(generator, self.smi_constant(Smi::from_int(bytecode_offset)),
                   StoreOp::Kind::TaggedBase(), MemoryRepresentation::TaggedSigned(),
                   WriteBarrierKind::kNoWriteBarrier,
                   JSGeneratorObject::kInputOrDebugPosOffset);

        self.store(generator, context, StoreOp::Kind::TaggedBase(),
                   MemoryRepresentation::AnyTagged(),
                   WriteBarrierKind::kFullWriteBarrier,
                   JSGeneratorObject::kContextOffset);
    }

    fn check_instance_type_is_in_range(&self, map: &V<Map>,
                                         first_instance_type: InstanceType,
                                         last_instance_type: InstanceType) -> V<Word32> {
        let instance_type = self.load_instance_type_field(map);

        if first_instance_type as u32 == 0 {
            self.uint32_less_than_or_equal(instance_type, last_instance_type as u32)
        } else {
            self.uint32_less_than_or_equal(
                self.word32_sub(instance_type, first_instance_type as u32),
                (last_instance_type as u32) - (first_instance_type as u32))
        }
    }

    // Dummy Implementations for methods used
    fn is_smi(&self, _object: &V<Object>) -> bool {
        false
    }
    fn deoptimize_if(&self, _condition: bool, _frame_state: V<FrameState>,
                       _reason: DeoptimizeReason, _feedback: &FeedbackSource) {}
    fn load_map_field(&self, _object: &V<Object>) -> V<Map> {
        V::<Map>::default()
    }
    fn unique_map_of_instance_type(&self, _instance_type: InstanceType) -> Option<RootIndex> {
        None
    }
    fn load_instance_type_field(&self, _map: &V<Map>) -> V<Word32> {
        V::<Word32>::default()
    }
    fn word32_equal(&self, _a: V<Word32>, _b: u32) -> bool {
        false
    }
    fn deoptimize_if_not(&self, _condition: bool, _frame_state: V<FrameState>,
                            _reason: DeoptimizeReason, _feedback: &FeedbackSource) {}
    fn check_instance_type_is_in_range(&self, _map: &V<Map>, _first: InstanceType, _last: InstanceType) -> V<Word32> {
        V::<Word32>::default()
    }
    fn template_load_field<T>(&self, _object: &V<Object>, _access: Access) -> V<T> {
        V::<T>::default()
    }
    fn object_is_string(&self, _object: &V<Object>) -> bool {
        false
    }
    fn string_equal(&self, _a: V<String>, _b: V<String>) -> V<Boolean> {
        V::<Boolean>::default()
    }
    fn root_equal(&self, _a: V<Boolean>, _root: RootIndex, _isolate: *mut Isolate) -> bool {
        false
    }
    fn no_context_constant(&self) -> V<Context> {
        V::<Context>::default()
    }
    fn load_tagged_field(&self, _side_table: &V<FixedArray>, _offset: usize) -> V<Object> {
        V::<Object>::default()
    }
    fn tagged_equal(&self, _a: &V<Object>, _b: &Rc<HeapObject>) -> bool {
        false
    }
    fn template_heap_constant<T>(&self, _object: Rc<HeapObject>) -> V<T> {
        V::<T>::default()
    }
    fn untag_smi(&self,
