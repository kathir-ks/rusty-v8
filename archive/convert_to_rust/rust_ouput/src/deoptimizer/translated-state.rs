// Converted from V8 C++ source files:
// Header: translated-state.h
// Implementation: translated-state.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/deoptimizer/translated-state.rs
#![allow(dead_code)]
#![allow(non_camel_case_types)]
use std::any::Any;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::fmt;
use std::io::Write;
use std::mem::size_of;
use std::optional::Option;
use std::rc::Rc;
use crate::common::simd128::Simd128;
use crate::deoptimizer::frame_translation_builder::DeoptimizationFrameTranslation;
use crate::objects::deoptimization_data::DeoptimizationData;
use crate::objects::feedback_vector::FeedbackVector;
use crate::objects::heap_object::HeapObject;
use crate::objects::shared_function_info::SharedFunctionInfo;
use crate::utils::boxed_float::Float64;
use crate::deoptimizer::translation_opcode::TranslationOpcode;
use crate::execution::frames::JavaScriptFrame;
use crate::execution::isolate::Isolate;
use crate::objects::object::Object;
use crate::objects::smi::Smi;
use crate::heap::heap::Heap;
use crate::objects::byte_array::ByteArray;
use crate::objects::map::Map;
use crate::objects::descriptor_array::DescriptorArray;
use crate::objects::fixed_array::FixedArray;
use crate::objects::fixed_double_array::FixedDoubleArray;
use crate::heap::factory::Factory;
use crate::execution::frames::OptimizedJSFrame;
use crate::heap::read_only_roots::ReadOnlyRoots;
use crate::deoptimizer::deoptimizer::LazyDeoptimizeReason;
use crate::objects::string::String;
use crate::objects::string::SlicedString;
use crate::objects::string::SeqString;
use crate::objects::string::ConsString;
use crate::execution::frames::StackFrame;
use crate::wasm::value_type::ValueKind;
use crate::objects::code::Code;
use crate::execution::safepoint_entry::SafepointEntry;
use crate::objects::feedback_vector::FeedbackSlot;
use crate::execution::feedback_nexus::FeedbackNexus;
use crate::execution::feedback_nexus::SpeculationMode;
use crate::execution::frames::CommonFrameConstants;
use crate::execution::isolate::Tagged_t;
use crate::deoptimizer::deoptimizer::Deoptimizer;
use crate::deoptimizer::translation_opcode::TranslationOpcodeOperandCount;

pub struct DeoptimizationLiteral {}

#[derive(Debug)]
pub enum TranslatedStateError {
    GenericError(String),
    AllocationError,
    InvalidArgument,
}

pub type Float32 = f32;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Address {
    address: usize,
}

impl Address {
    pub fn new(address: usize) -> Self {
        Address { address }
    }
    pub fn get_address(&self) -> usize { self.address }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:x}", self.address)
    }
}

pub struct RegisterValues {}

impl RegisterValues {
    pub fn GetRegister(&self, _reg_code: i32) -> i64 {
        0 // Placeholder implementation
    }
    pub fn GetFloatRegister(&self, _reg_code: i32) -> Float32 {
        0.0 // Placeholder implementation
    }
    pub fn GetDoubleRegister(&self, _reg_code: i32) -> Float64 {
        Float64::new(0.0) // Placeholder implementation
    }
    pub fn GetSimd128Register(&self, _reg_code: i32) -> Simd128 {
        Simd128::from([0; 16]) // Placeholder implementation
    }
}

#[derive(Debug)]
pub enum CreateArgumentsType {
    kMappedArguments,
    kUnmappedArguments,
    kRestParameter,
}

#[derive(Debug)]
pub enum DeoptimizationLiteralKind {
    kWasmInt32,
    kWasmInt64,
    kWasmFloat32,
    kWasmFloat64,
    kWasmI31Ref,
}

impl DeoptimizationLiteral {
    fn kind(&self) -> DeoptimizationLiteralKind {
        DeoptimizationLiteralKind::kWasmInt32
    }
    fn GetInt32(&self) -> i32 {
        0
    }
    fn GetInt64(&self) -> i64 {
        0
    }
    fn GetFloat32(&self) -> f32 {
        0.0
    }
    fn GetFloat64(&self) -> f64 {
        0.0
    }
    fn GetSmi(&self) -> Smi {
        Smi {}
    }
}

pub struct DeoptimizationLiteralArray {}
impl DeoptimizationLiteralArray {
    fn get(&self, _index: i32) -> Tagged<Object> {
        Tagged::<Object> {
            ptr_: 0,
        }
    }
}

pub struct ProtectedDeoptimizationLiteralArray {}
impl ProtectedDeoptimizationLiteralArray {
    fn get(&self, _index: i32) -> Tagged<Object> {
        Tagged::<Object> {
            ptr_: 0,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BytecodeOffset {
    offset: i32,
}

impl BytecodeOffset {
    pub fn new(offset: i32) -> Self {
        BytecodeOffset { offset }
    }
    pub fn ToInt(&self) -> i32 { self.offset }
    pub fn None() -> Self { BytecodeOffset { offset: 0 } }
}

pub struct Tagged<T> {
    ptr_: usize,
}
impl<T> Tagged<T> {
    pub fn is_null(&self) -> bool {
        self.ptr_ == 0
    }
    pub fn ptr(&self) -> usize {
        self.ptr_
    }
}
pub struct IndirectHandle<T> {
    ptr_: *mut T,
}
impl<T> IndirectHandle<T> {
    pub fn is_null(&self) -> bool {
        self.ptr_.is_null()
    }
}
pub struct DirectHandle<T> {
    ptr_: *mut T,
}
impl<T> DirectHandle<T> {
    pub fn is_null(&self) -> bool {
        self.ptr_.is_null()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Representation {
    Smi,
    HeapObject,
    Double,
    Bit,
}

impl Representation {
    pub fn IsDouble(&self) -> bool {
        *self == Representation::Double
    }
    pub fn IsHeapObject(&self) -> bool {
        *self == Representation::HeapObject
    }
}

pub struct FieldIndex {
    index_: i32,
}

impl FieldIndex {
    pub fn is_inobject(&self) -> bool {
        self.index_ > 0
    }
    pub fn outobject_array_index(&self) -> i32 {
        0
    }
    pub fn ForDescriptor(_map: Tagged<Map>, _i: InternalIndex) -> FieldIndex {
        FieldIndex { index_: 0 }
    }
    pub fn index(&self) -> i32 {
        self.index_
    }
}

pub struct InternalIndex {
    index_: i32,
}

impl InternalIndex {
    pub fn new(index: i32) -> Self {
        InternalIndex { index_ }
    }
    pub fn index(&self) -> i32 {
        self.index_
    }
}

pub struct DeoptimizationLiteralProvider {
    literals_on_heap_: Tagged<DeoptimizationLiteralArray>,
    literals_off_heap_: Vec<DeoptimizationLiteral>,
}

impl DeoptimizationLiteralProvider {
    pub fn new_on_heap(literal_array: Tagged<DeoptimizationLiteralArray>) -> Self {
        DeoptimizationLiteralProvider {
            literals_on_heap_: literal_array,
            literals_off_heap_: Vec::new(),
        }
    }

    pub fn new_off_heap(literals: Vec<DeoptimizationLiteral>) -> Self {
        DeoptimizationLiteralProvider {
            literals_on_heap_: Tagged { ptr_: 0 },
            literals_off_heap_: literals,
        }
    }

    pub fn get(&self, container: &mut TranslatedState, literal_index: i32) -> TranslatedValue {
        if self.literals_on_heap_.ptr() != 0 {
            return TranslatedValue::NewTagged(container, self.literals_on_heap_.get(literal_index));
        }

        let literal = &self.literals_off_heap_[literal_index as usize];
        match literal.kind() {
            DeoptimizationLiteralKind::kWasmInt32 => TranslatedValue::NewInt32(container, literal.GetInt32()),
            DeoptimizationLiteralKind::kWasmInt64 => TranslatedValue::NewInt64(container, literal.GetInt64()),
            DeoptimizationLiteralKind::kWasmFloat32 => TranslatedValue::NewFloat(container, literal.GetFloat32()),
            DeoptimizationLiteralKind::kWasmFloat64 => TranslatedValue::NewDouble(container, literal.GetFloat64()),
            DeoptimizationLiteralKind::kWasmI31Ref => TranslatedValue::NewTagged(container, literal.GetSmi()),
        }
    }

    pub fn get_on_heap_literals(&self) -> Tagged<DeoptimizationLiteralArray> {
        self.literals_on_heap_
    }
}

#[derive(Debug)]
struct TranslatedValue {
    kind_: Kind,
    materialization_state_: MaterializationState,
    container_: *mut TranslatedState, // Lifetime issues
    storage_: usize, //Handle<HeapObject>,
    raw_literal_: usize, // Tagged<Object>,
    uint32_value_: u32,
    int32_value_: i32,
    uint64_value_: u64,
    int64_value_: i64,
    float_value_: Float32,
    double_value_: Float64,
    materialization_info_: MaterializedObjectInfo,
    simd128_value_: Simd128,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Kind {
    kInvalid,
    kTagged,
    kInt32,
    kInt64,
    kInt64ToBigInt,
    kUint64ToBigInt,
    kUint32,
    kUint64,
    kBoolBit,
    kFloat,
    kDouble,
    kHoleyDouble,
    kSimd128,
    kCapturedObject,
    kDuplicatedObject,
    kCapturedStringConcat,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum MaterializationState {
    kUninitialized,
    kAllocated,
    kFinished,
}

#[derive(Debug, Default, Copy, Clone)]
struct MaterializedObjectInfo {
    id_: i32,
    length_: i32,
}

impl TranslatedValue {
    fn NewDeferredObject(container: *mut TranslatedState, length: i32, object_index: i32) -> TranslatedValue {
        TranslatedValue {
            kind_: Kind::kCapturedObject,
            materialization_state_: MaterializationState::kUninitialized,
            container_: container,
            storage_: 0,
            raw_literal_: 0,
            uint32_value_: 0,
            int32_value_: 0,
            uint64_value_: 0,
            int64_value_: 0,
            float_value_: 0.0,
            double_value_: Float64::new(0.0),
            materialization_info_: MaterializedObjectInfo { id_: object_index, length_: length },
            simd128_value_: Simd128::from([0; 16]),
        }
    }

    fn NewDuplicateObject(container: *mut TranslatedState, id: i32) -> TranslatedValue {
        TranslatedValue {
            kind_: Kind::kDuplicatedObject,
            materialization_state_: MaterializationState::kUninitialized,
            container_: container,
            storage_: 0,
            raw_literal_: 0,
            uint32_value_: 0,
            int32_value_: 0,
            uint64_value_: 0,
            int64_value_: 0,
            float_value_: 0.0,
            double_value_: Float64::new(0.0),
            materialization_info_: MaterializedObjectInfo { id_: id, length_: -1 },
            simd128_value_: Simd128::from([0; 16]),
        }
    }

    fn NewStringConcat(container: *mut TranslatedState, id: i32) -> TranslatedValue {
        TranslatedValue {
            kind_: Kind::kCapturedStringConcat,
            materialization_state_: MaterializationState::kUninitialized,
            container_: container,
            storage_: 0,
            raw_literal_: 0,
            uint32_value_: 0,
            int32_value_: 0,
            uint64_value_: 0,
            int64_value_: 0,
            float_value_: 0.0,
            double_value_: Float64::new(0.0),
            materialization_info_: MaterializedObjectInfo { id_: id, length_: -1 },
            simd128_value_: Simd128::from([0; 16]),
        }
    }

    fn NewFloat(container: *mut TranslatedState, value: Float32) -> TranslatedValue {
        TranslatedValue {
            kind_: Kind::kFloat,
            materialization_state_: MaterializationState::kUninitialized,
            container_: container,
            storage_: 0,
            raw_literal_: 0,
            uint32_value_: 0,
            int32_value_: 0,
            uint64_value_: 0,
            int64_value_: 0,
            float_value_: value,
            double_value_: Float64::new(0.0),
            materialization_info_: MaterializedObjectInfo::default(),
            simd128_value_: Simd128::from([0; 16]),
        }
    }

    fn NewDouble(container: *mut TranslatedState, value: Float64) -> TranslatedValue {
        TranslatedValue {
            kind_: Kind::kDouble,
            materialization_state_: MaterializationState::kUninitialized,
            container_: container,
            storage_: 0,
            raw_literal_: 0,
            uint32_value_: 0,
            int32_value_: 0,
            uint64_value_: 0,
            int64_value_: 0,
            float_value_: 0.0,
            double_value_: value,
            materialization_info_: MaterializedObjectInfo::default(),
            simd128_value_: Simd128::from([0; 16]),
        }
    }

    fn NewHoleyDouble(container: *mut TranslatedState, value: Float64) -> TranslatedValue {
        TranslatedValue {
            kind_: Kind::kHoleyDouble,
            materialization_state_: MaterializationState::kUninitialized,
            container_: container,
            storage_: 0,
            raw_literal_: 0,
            uint32_value_: 0,
            int32_value_: 0,
            uint64_value_: 0,
            int64_value_: 0,
            float_value_: 0.0,
            double_value_: value,
            materialization_info_: MaterializedObjectInfo::default(),
            simd128_value_: Simd128::from([0; 16]),
        }
    }

    fn NewSimd128(container: *mut TranslatedState, value: Simd128) -> TranslatedValue {
        TranslatedValue {
            kind_: Kind::kSimd128,
            materialization_state_: MaterializationState::kUninitialized,
            container_: container,
            storage_: 0,
            raw_literal_: 0,
            uint32_value_: 0,
            int32_value_: 0,
            uint64_value_: 0,
            int64_value_: 0,
            float_value_: 0.0,
            double_value_: Float64::new(0.0),
            materialization_info_: MaterializedObjectInfo::default(),
            simd128_value_: value,
        }
    }

    fn NewInt32(container: *mut TranslatedState, value: i32) -> TranslatedValue {
        TranslatedValue {
            kind_: Kind::kInt32,
            materialization_state_: MaterializationState::kUninitialized,
            container_: container,
            storage_: 0,
            raw_literal_: 0,
            uint32_value_: 0,
            int32_value_: value,
            uint64_value_: 0,
            int64_value_: 0,
            float_value_: 0.0,
            double_value_: Float64::new(0.0),
            materialization_info_: MaterializedObjectInfo::default(),
            simd128_value_: Simd128::from([0; 16]),
        }
    }

    fn NewInt64(container: *mut TranslatedState, value: i64) -> TranslatedValue {
        TranslatedValue {
            kind_: Kind::kInt64,
            materialization_state_: MaterializationState::kUninitialized,
            container_: container,
            storage_: 0,
            raw_literal_: 0,
            uint32_value_: 0,
            int32_value_: 0,
            uint64_value_: 0,
            int64_value_: value,
            float_value_: 0.0,
            double_value_: Float64::new(0.0),
            materialization_info_: MaterializedObjectInfo::default(),
            simd128_value_: Simd128::from([0; 16]),
        }
    }

    fn NewInt64ToBigInt(container: *mut TranslatedState, value: i64) -> TranslatedValue {
        TranslatedValue {
            kind_: Kind::kInt64ToBigInt,
            materialization_state_: MaterializationState::kUninitialized,
            container_: container,
            storage_: 0,
            raw_literal_: 0,
            uint32_value_: 0,
            int32_value_: 0,
            uint64_value_: 0,
            int64_value_: value,
            float_value_: 0.0,
            double_value_: Float64::new(0.0),
            materialization_info_: MaterializedObjectInfo::default(),
            simd128_value_: Simd128::from([0; 16]),
        }
    }

    fn NewUint64ToBigInt(container: *mut TranslatedState, value: u64) -> TranslatedValue {
        TranslatedValue {
            kind_: Kind::kUint64ToBigInt,
            materialization_state_: MaterializationState::kUninitialized,
            container_: container,
            storage_: 0,
            raw_literal_: 0,
            uint32_value_: 0,
            int32_value_: 0,
            uint64_value_: value,
            int64_value_: 0,
            float_value_: 0.0,
            double_value_: Float64::new(0.0),
            materialization_info_: MaterializedObjectInfo::default(),
            simd128_value_: Simd128::from([0; 16]),
        }
    }

    fn NewUint32(container: *mut TranslatedState, value: u32) -> TranslatedValue {
        TranslatedValue {
            kind_: Kind::kUint32,
            materialization_state_: MaterializationState::kUninitialized,
            container_: container,
            storage_: 0,
            raw_literal_: 0,
            uint32_value_: value,
            int32_value_: 0,
            uint64_value_: 0,
            int64_value_: 0,
            float_value_: 0.0,
            double_value_: Float64::new(0.0),
            materialization_info_: MaterializedObjectInfo::default(),
            simd128_value_: Simd128::from([0; 16]),
        }
    }
    fn NewUint64(container: *mut TranslatedState, value: u64) -> TranslatedValue {
        TranslatedValue {
            kind_: Kind::kUint64,
            materialization_state_: MaterializationState::kUninitialized,
            container_: container,
            storage_: 0,
            raw_literal_: 0,
            uint32_value_: 0,
            int32_value_: 0,
            uint64_value_: value,
            int64_value_: 0,
            float_value_: 0.0,
            double_value_: Float64::new(0.0),
            materialization_info_: MaterializedObjectInfo::default(),
            simd128_value_: Simd128::from([0; 16]),
        }
    }

    fn NewBool(container: *mut TranslatedState, value: u32) -> TranslatedValue {
        TranslatedValue {
            kind_: Kind::kBoolBit,
            materialization_state_: MaterializationState::kUninitialized,
            container_: container,
            storage_: 0,
            raw_literal_: 0,
            uint32_value_: value,
            int32_value_: 0,
            uint64_value_: 0,
            int64_value_: 0,
            float_value_: 0.0,
            double_value_: Float64::new(0.0),
            materialization_info_: MaterializedObjectInfo::default(),
            simd128_value_: Simd128::from([0; 16]),
        }
    }

    fn NewTagged(container: *mut TranslatedState, literal: Tagged<Object>) -> TranslatedValue {
        TranslatedValue {
            kind_: Kind::kTagged,
            materialization_state_: MaterializationState::kUninitialized,
            container_: container,
            storage_: 0,
            raw_literal_: literal.ptr(),
            uint32_value_: 0,
            int32_value_: 0,
            uint64_value_: 0,
            int64_value_: 0,
            float_value_: 0.0,
            double_value_: Float64::new(0.0),
            materialization_info_: MaterializedObjectInfo::default(),
            simd128_value_: Simd128::from([0; 16]),
        }
    }

    fn NewInvalid(container: *mut TranslatedState) -> TranslatedValue {
        TranslatedValue {
            kind_: Kind::kInvalid,
            materialization_state_: MaterializationState::kUninitialized,
            container_: container,
            storage_: 0,
            raw_literal_: 0,
            uint32_value_: 0,
            int32_value_: 0,
            uint64_value_: 0,
            int64_value_: 0,
            float_value_: 0.0,
            double_value_: Float64::new(0.0),
            materialization_info_: MaterializedObjectInfo::default(),
            simd128_value_: Simd128::from([0; 16]),
        }
    }

    fn raw_literal(&self) -> Tagged<Object> {
        assert_eq!(Kind::kTagged, self.kind_);
        Tagged::<Object> { ptr_: self.raw_literal_ }
    }

    fn int32_value(&self) -> i32 {
        assert_eq!(Kind::kInt32, self.kind_);
        self.int32_value_
    }

    fn int64_value(&self) -> i64 {
        assert!(self.kind_ == Kind::kInt64 || self.kind_ == Kind::kInt64ToBigInt);
        self.int64_value_
    }

    fn uint64_value(&self) -> u64 {
        assert_eq!(Kind::kUint64ToBigInt, self.kind_);
        self.uint64_value_
    }

    fn uint32_value(&self) -> u32 {
        assert!(self.kind_ == Kind::kUint32 || self.kind_ == Kind::kBoolBit);
        self.uint32_value_
    }

    fn float_value(&self) -> Float32 {
        assert_eq!(Kind::kFloat, self.kind_);
        self.float_value_
    }

    fn double_value(&self) -> Float64 {
        assert!(self.kind_ == Kind::kDouble || self.kind_ == Kind::kHoleyDouble);
        self.double_value_
    }

    fn simd_value(&self) -> Simd128 {
        assert_eq!(Kind::kSimd128, self.kind_);
        self.simd128_value_
    }

    fn object_length(&self) -> i32 {
        assert_eq!(Kind::kCapturedObject, self.kind_);
        self.materialization_info_.length_
    }

    fn object_index(&self) -> i32 {
        assert!(self.kind_ == Kind::kCapturedObject || self.kind_ == Kind::kDuplicatedObject || self.kind_ == Kind::kCapturedStringConcat);
        self.materialization_info_.id_
    }

    fn string_concat_index(&self) -> i32 {
        self.object_index()
    }
    fn kind(&self) -> Kind {
        self.kind_
    }
    fn materialization_state(&self) -> MaterializationState {
        self.materialization_state_
    }
    fn Handlify(&mut self) {
        todo!()
    }
    fn GetChildrenCount(&self) -> i32 {
        if self.kind() == Kind::kCapturedObject {
            return self.object_length();
        } else if self.kind() == Kind::kCapturedStringConcat {
            let kLeft = 1;
            let kRight = 1;
            return kLeft + kRight;
        } else {
            return 0;
        }
    }
    fn storage(&self) -> usize {
        self.storage_
    }
        fn set_initialized_storage(&mut self, storage: usize) {
        self.storage_ = storage;
        self.materialization_state_ = MaterializationState::kFinished;
    }
    fn GetRawValue(&self) -> Tagged<Object> {
        todo!()
    }
    fn GetValue(&mut self) -> Handle<Object> {
        todo!()
    }
    fn set_storage(&mut self, storage_: usize) {
        self.storage_ = storage_
    }
    fn ReplaceElementsArrayWithCopy(&mut self) {
        todo!()
    }
    fn mark_finished(&mut self) {
        self.materialization_state_ = MaterializationState::kFinished;
    }

    fn mark_allocated(&mut self) {
        self.materialization_state_ = MaterializationState::kAllocated;
    }
    fn IsMaterializedObject(&self) -> bool {
        match self.kind() {
            Kind::kCapturedObject | Kind::kDuplicatedObject | Kind::kCapturedStringConcat => true,
            _ => false,
        }
    }
        fn isolate(&self) -> *mut Isolate {
        self.container_ as *mut Isolate
    }

    fn IsMaterializableByDebugger(&self) -> bool {
           self.kind() == Kind::kDouble || self.kind() == Kind::kHoleyDouble
    }
}

#[derive(Debug)]
struct TranslatedFrame {
    kind_: Kind,
    bytecode_offset_: BytecodeOffset,
    raw_shared_info_: usize,
    shared_info_: usize,
    raw_bytecode_array_: usize,
    bytecode_array_: usize,
    height_: u32,
    return_value_offset_: i32,
    return_value_count_: i32,
    formal_parameter_count_: i32,
    handle_state_: HandleState,
    values_: Vec<TranslatedValue>,
    return_kind_: Option<ValueKind>,
    wasm_function_index_: i32,
}

#[derive(Debug, PartialEq)]
enum HandleState {
    kRawPointers,
    kHandles,
}

impl TranslatedFrame {
    fn UnoptimizedJSFrame(bytecode_offset: BytecodeOffset, shared_info: Tagged<SharedFunctionInfo>, bytecode_array: Tagged<ByteArray>, height: u32, return_value_offset: i32, return_value_count: i32) -> TranslatedFrame {
        TranslatedFrame {
            kind_: Kind::kUnoptimizedFunction,
            bytecode_offset_: bytecode_offset,
            raw_shared_info_: shared_info.ptr(),
            shared_info_: 0,
            raw_bytecode_array_: bytecode_array.ptr(),
            bytecode_array_: 0,
            height_: height,
            return_value_offset_: return_value_offset,
            return_value_count_: return_value_count,
            formal_parameter_count_: -1,
            handle_state_: HandleState::kRawPointers,
            values_: Vec::new(),
            return_kind_: None,
            wasm_function_index_: -1,
        }
    }
    fn InlinedExtraArguments(shared_info: Tagged<SharedFunctionInfo>, height: u32, formal_parameter_count: u32) -> TranslatedFrame {
        TranslatedFrame {
            kind_: Kind::kInlinedExtraArguments,
            bytecode_offset_: BytecodeOffset::None(),
            raw_shared_info_: shared_info.ptr(),
            shared_info_: 0,
            raw_bytecode_array_: 0,
            bytecode_array_: 0,
            height_: height,
            return_value_offset_: 0,
            return_value_count_: 0,
            formal_parameter_count_: formal_parameter_count as i32,
            handle_state_: HandleState::kRawPointers,
            values_: Vec::new(),
            return_kind_: None,
            wasm_function_index_: -1,
        }
    }
    fn kind(&self) -> Kind {
        self.kind_
    }

    fn GetValueCount(&self) -> i32 {
          let kTheFunction = 1;

        match self.kind() {
            Kind::kUnoptimizedFunction => {
                let parameter_count = 0;
                let kTheContext = 1;
                let kTheAccumulator = 1;
                (self.height() + parameter_count + kTheContext + kTheFunction + kTheAccumulator) as i32
            }
            Kind::kInlinedExtraArguments => (self.height() + kTheFunction) as i32,
            _ => {
                let kTheContext = 1;
                (self.height() + kTheContext + kTheFunction) as i32
            }
        }
    }

    fn height(&self) -> u32 {
        self.height_
    }
}

pub struct TranslatedState {
    purpose_: Purpose,
    frames_: Vec<TranslatedFrame>,
    isolate_: *mut Isolate,
    stack_frame_pointer_: usize,
    formal_parameter_count_: i32,
    actual_argument_count_: i32,
    object_positions_: Vec<ObjectPosition>,
    feedback_vector_handle_: IndirectHandle<FeedbackVector>,
    feedback_vector_: Tagged<FeedbackVector>,
    feedback_slot_: FeedbackSlot,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Purpose {
    kDeoptimization,
    kFrameInspection,
}

#[derive(Debug, Copy, Clone)]
struct ObjectPosition {
    frame_index_: i32,
    value_index_: i32,
}

impl TranslatedState {
    pub fn new() -> Self {
        TranslatedState {
            purpose_: Purpose::kDeoptimization,
            frames_: Vec::new(),
            isolate_: std::ptr::null_mut(),
            stack_frame_pointer_: 0,
            formal_parameter_count_: 0,
            actual_argument_count_: 0,
            object_positions_: Vec::new(),
            feedback_vector_handle_: IndirectHandle { ptr_: std::ptr::null_mut() },
            feedback_vector_: Tagged { ptr_: 0 },
            feedback_slot_: FeedbackSlot { index_: 0 },
        }
    }
    pub fn as_mut(&mut self) -> &mut Self {
        self
    }
    fn GetValueByObjectIndex(&self, object_index: i32) -> &TranslatedValue {
        let pos = &self.object_positions_[object_index as usize];
        &self.frames_[pos.frame_index_ as usize].values_[pos.value_index_ as usize]
    }
     fn UpdateFromPreviouslyMaterializedObjects(&mut self) {
        todo!()
    }
    fn StoreMaterializedValuesAndDeopt(&mut self, frame: *mut JavaScriptFrame) {
        todo!()
    }
}

struct Lock {}

