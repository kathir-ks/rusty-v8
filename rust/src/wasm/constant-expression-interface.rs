// src/wasm/constant_expression_interface.rs

use std::sync::Arc;

// Placeholder for FullDecoder. Needs more context on its structure and functionality
pub struct FullDecoder {}

// Placeholder for Value. Needs more context on its structure and functionality
#[derive(Clone)]
pub struct Value {
    pub runtime_value: WasmValue,
    pub ty: ValueType
}

impl Value {
    pub fn new(_pc: usize, ty: ValueType) -> Self {
        Value{
            runtime_value: WasmValue::I32(0), // Default
            ty
        }
    }
}


// Placeholder for WasmOpcode enum. Needs more context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmOpcode {
    kExprI32Add,
    kExprI32Sub,
    kExprI32Mul,
    kExprI64Add,
    kExprI64Sub,
    kExprI64Mul,
    kExprExternConvertAny,
    kExprAnyConvertExtern,
}

// Placeholder for Simd128Immediate struct. Needs more context.
pub struct Simd128Immediate {
    pub value: Simd128,
}

// Placeholder for WasmValue enum. Needs more context on its full structure
#[derive(Clone, Copy)]
pub enum WasmValue {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    S128(Simd128),
    Ref(usize), // Placeholder for now, replace with proper Ref type
}

impl WasmValue {
    pub fn to_i32(&self) -> i32 {
        match self {
            WasmValue::I32(x) => *x,
            _ => panic!("Unexpected type"),
        }
    }
    pub fn to_i64(&self) -> i64 {
        match self {
            WasmValue::I64(x) => *x,
            _ => panic!("Unexpected type"),
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            WasmValue::I32(x) => *x as u32,
            _ => panic!("Unexpected type"),
        }
    }
}

// Placeholder for Simd128 struct. Needs more context.
#[derive(Clone, Copy)]
pub struct Simd128 {}

// Placeholder for GlobalIndexImmediate struct. Needs more context.
pub struct GlobalIndexImmediate {
    pub index: usize,
}

// Placeholder for StructIndexImmediate struct. Needs more context.
pub struct StructIndexImmediate {
    pub index: ModuleTypeIndex,
    pub struct_type: Box<StructType>
}

// Placeholder for StringConstImmediate struct. Needs more context.
pub struct StringConstImmediate {
    pub index: usize,
}

// Placeholder for ArrayIndexImmediate struct. Needs more context.
pub struct ArrayIndexImmediate {
    pub index: ModuleTypeIndex,
    pub array_type: Box<ArrayType>
}

// Placeholder for IndexImmediate struct. Needs more context.
pub struct IndexImmediate {
    pub index: usize,
}

// Placeholder for WasmGlobal struct. Needs more context.
pub struct WasmGlobal {
    pub mutability: bool,
    pub offset: usize,
    pub shared: bool,
    pub ty: ValueType
}

// Placeholder for WasmModule struct. Needs more context.
pub struct WasmModule {
    pub functions: Vec<FunctionData>,
    pub globals: Vec<WasmGlobal>,
    pub stringref_literals: Vec<WasmStringRefLiteral>,
    pub elem_segments: Vec<WasmElemSegment>,
    pub data_segments: Vec<WasmDataSegment>
}

impl WasmModule {
    fn canonical_type(&self, value_type: ValueType) -> CanonicalValueType {
        CanonicalValueType::I32
    }

    fn type_(&self, index: ModuleTypeIndex) -> TypeInfo {
        TypeInfo{
            is_shared: false
        }
    }

    fn canonical_type_id(&self, index: ModuleTypeIndex) -> usize {
        0
    }
}

// Placeholder for FunctionData struct. Needs more context.
pub struct FunctionData {
    pub declared: bool,
    pub sig_index: ModuleTypeIndex,
}

// Placeholder for WasmStringRefLiteral struct. Needs more context.
pub struct WasmStringRefLiteral {
    pub source: SourceRange,
}

// Placeholder for SourceRange struct. Needs more context.
pub struct SourceRange {
    pub offset: usize,
    pub end_offset: usize,
}

// Placeholder for WasmElemSegment struct. Needs more context.
pub struct WasmElemSegment {
    pub status: WasmElemSegmentStatus,
    pub element_count: usize,
}

// Placeholder for WasmElemSegmentStatus enum. Needs more context.
#[derive(PartialEq)]
pub enum WasmElemSegmentStatus {
    kStatusPassive,
    KStatusActive,
    KStatusDeclarative
}

// Placeholder for WasmDataSegment struct. Needs more context.
pub struct WasmDataSegment {
    pub source: SourceRange,
}

// Placeholder for ValueType enum. Needs more context.
#[derive(Clone, Copy)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
    Ref(HeapType),
    RefNull,
    I8,
    I16,
    F16,
    S128,
    Void,
    Top,
    Bottom,
}

impl ValueType {
    pub fn kind(&self) -> ValueKind {
        match self {
            ValueType::I32 => ValueKind::I32,
            ValueType::I64 => ValueKind::I64,
            ValueType::F32 => ValueKind::F32,
            ValueType::F64 => ValueKind::F64,
            ValueType::Ref(_) => ValueKind::Ref,
            ValueType::RefNull => ValueKind::RefNull,
            ValueType::I8 => ValueKind::I8,
            ValueType::I16 => ValueKind::I16,
            ValueType::F16 => ValueKind::F16,
            ValueType::S128 => ValueKind::S128,
            ValueType::Void => ValueKind::Void,
            ValueType::Top => ValueKind::Top,
            ValueType::Bottom => ValueKind::Bottom,
        }
    }

    pub fn use_wasm_null(&self) -> bool {
        false // Placeholder
    }

    fn value_kind_size(&self) -> u32 {
        4 // Placeholder
    }
}

// Placeholder for ValueKind enum
#[derive(Clone, Copy)]
pub enum ValueKind {
    I32,
    I64,
    F32,
    F64,
    Ref,
    RefNull,
    I8,
    I16,
    F16,
    S128,
    Void,
    Top,
    Bottom,
}

// Placeholder for CanonicalValueType enum. Needs more context.
#[derive(Clone, Copy)]
pub enum CanonicalValueType {
    I32,
    RefMaybeNull(RefType, Nullability),
    Ref(usize, bool, RefTypeKind)
}

// Placeholder for Nullability enum. Needs more context.
#[derive(Clone, Copy)]
pub enum Nullability {
    Null,
    NonNull
}

// Placeholder for RefType enum. Needs more context.
#[derive(Clone, Copy)]
pub enum RefType {
    kWasmExternRef,
    kWasmAnyRef,
}

// Placeholder for RefTypeKind enum
#[derive(Clone, Copy)]
pub enum RefTypeKind {
    kFunction,
}

// Placeholder for HeapType enum
#[derive(Clone, Copy)]
pub enum HeapType {
    Func,
    String,
    I31
}

const KWASM_REF_STRING: ValueType = ValueType::Ref(HeapType::String);
const KWASM_REF_I31: ValueType = ValueType::Ref(HeapType::I31);

// Placeholder for WasmToJSObject function. Needs more context.
fn wasm_to_js_object(_isolate: &Isolate, _wasm_ref: usize) -> usize {
    0
}

// Placeholder for JSToWasmObject function. Needs more context.
fn js_to_wasm_object(_isolate: &Isolate, _js_ref: usize, _wasm_type: RefType, _error_message: &mut Option<&str>) -> Result<usize, String> {
    Ok(0)
}

// Placeholder for DirectHandle struct. Needs more context.
#[derive(Clone)]
pub struct DirectHandle<T>(T);

impl DirectHandle<WasmFuncRef> {
    
}

// Placeholder for WasmFuncRef struct. Needs more context.
pub struct WasmFuncRef {}

// Placeholder for WasmTrustedInstanceData struct. Needs more context.
pub struct WasmTrustedInstanceData {
    native_module_: Box<NativeModule>,
    untagged_globals_buffer_: Box<UntaggedGlobalsBuffer>,
    tagged_globals_buffer_: Box<TaggedGlobalsBuffer>
}

impl WasmTrustedInstanceData {
    fn get_or_create_func_ref(_isolate: &Isolate, _data: &WasmTrustedInstanceData, _index: u32) -> DirectHandle<WasmFuncRef> {
        DirectHandle(WasmFuncRef{})
    }

    fn native_module(&self) -> &NativeModule {
        &self.native_module_
    }

    fn untagged_globals_buffer(&self) -> &UntaggedGlobalsBuffer {
        &self.untagged_globals_buffer_
    }

    fn tagged_globals_buffer(&self) -> &TaggedGlobalsBuffer {
        &self.tagged_globals_buffer_
    }
}

// Placeholder for NativeModule struct
pub struct NativeModule {
    wire_bytes: Vec<u8>
}

impl NativeModule {
    fn wire_bytes(&self) -> &Vec<u8> {
        &self.wire_bytes
    }
}

// Placeholder for UntaggedGlobalsBuffer struct
pub struct UntaggedGlobalsBuffer {
    backing_store: Vec<u8>
}

impl UntaggedGlobalsBuffer {
    fn backing_store(&self) -> &Vec<u8> {
        &self.backing_store
    }
}

// Placeholder for TaggedGlobalsBuffer struct
pub struct TaggedGlobalsBuffer {
    data: Vec<usize>
}

impl TaggedGlobalsBuffer {
    fn get(&self, offset: usize) -> usize {
        self.data[offset]
    }
}

// Placeholder for Isolate struct. Needs more context.
pub struct Isolate {
    factory_: Box<Factory>
}

impl Isolate {
    fn factory(&self) -> &Factory {
        &self.factory_
    }
}

// Placeholder for Factory struct. Needs more context.
pub struct Factory {}

impl Factory {
    fn wasm_null(&self) -> usize {
        0 // Placeholder
    }

    fn null_value(&self) -> usize {
        0 // Placeholder
    }

    fn new_wasm_struct(&self, _struct_type: &StructType, _field_values: &mut [WasmValue], _rtt: DirectHandle<Map>) -> usize {
        0 // Placeholder
    }

    fn new_string_from_utf8(&self, _string_bytes: &Vec<u8>, _variant: usize) -> Result<DirectHandle<String>, String>{
        Ok(DirectHandle(String{}))
    }

    fn new_wasm_array(&self, _element_type: ValueType, _length: u32, _initial_value: WasmValue, _rtt: DirectHandle<Map>) -> usize {
        0 // Placeholder
    }

    fn new_wasm_array_from_elements(&self, _array_type: &ArrayType, _element_values: Vec<WasmValue>, _rtt: DirectHandle<Map>) -> usize {
        0 // Placeholder
    }

    fn new_wasm_array_from_memory(&self, _length: u32, _rtt: DirectHandle<Map>, _element_type: CanonicalValueType, _source: usize) -> DirectHandle<WasmArray> {
        DirectHandle(WasmArray{})
    }

    fn new_wasm_array_from_element_segment(&self, _trusted_instance_data: &WasmTrustedInstanceData, _shared_trusted_instance_data: &WasmTrustedInstanceData, _segment_imm_index: usize, _offset: u32, _length: u32, _rtt: DirectHandle<Map>, _element_type: CanonicalValueType) -> DirectHandle<Object> {
        DirectHandle(Object{})
    }
}

// Placeholder for String struct. Needs more context.
pub struct String {}

// Placeholder for ModuleTypeIndex struct
#[derive(Clone, Copy)]
pub struct ModuleTypeIndex {
    index: usize
}

// Placeholder for TypeInfo struct
pub struct TypeInfo {
    is_shared: bool
}

// Placeholder for StructType struct
pub struct StructType {
    field_count_: u32,
    field_: Vec<ValueType>
}

impl StructType {
    fn field_count(&self) -> usize {
        self.field_count_ as usize
    }

    fn field(&self, i: u32) -> ValueType {
        self.field_[i as usize]
    }
}

// Placeholder for ArrayType struct
pub struct ArrayType {
    element_type_: ValueType
}

impl ArrayType {
    fn element_type(&self) -> ValueType {
        self.element_type_
    }
}

// Placeholder for Map struct
pub struct Map {
    wasm_type_info_: Box<WasmTypeInfo>
}

impl Map {
    fn wasm_type_info(&self) -> &WasmTypeInfo {
        &self.wasm_type_info_
    }
}

// Placeholder for WasmTypeInfo struct
pub struct WasmTypeInfo {
    type_index_: ModuleTypeIndex,
    element_type_: CanonicalValueType,
    ty_: CanonicalValueType
}

impl WasmTypeInfo {
    fn type_index(&self) -> ModuleTypeIndex {
        self.type_index_
    }

    fn element_type(&self) -> CanonicalValueType {
        self.element_type_
    }

    fn type_(&self) -> CanonicalValueType {
        self.ty_
    }
}

// Placeholder for WasmArray struct
pub struct WasmArray {}

impl WasmArray {
    fn max_length(_array_type: &ArrayType) -> i32 {
        1024 // Placeholder
    }
}

// Placeholder for Object struct.
pub struct Object {}

// Placeholder for Smi struct
pub struct Smi {}

impl Smi {
    fn value(&self) -> i32 {
        0 // Placeholder
    }
}

fn add_with_wraparound(lhs: i32, rhs: i32) -> i32 {
    lhs.wrapping_add(rhs)
}

fn sub_with_wraparound(lhs: i32, rhs: i32) -> i32 {
    lhs.wrapping_sub(rhs)
}

fn mul_with_wraparound(lhs: i32, rhs: i32) -> i32 {
    lhs.wrapping_mul(rhs)
}

fn add_with_wraparound_i64(lhs: i64, rhs: i64) -> i64 {
    lhs.wrapping_add(rhs)
}

fn sub_with_wraparound_i64(lhs: i64, rhs: i64) -> i64 {
    lhs.wrapping_sub(rhs)
}

fn mul_with_wraparound_i64(lhs: i64, rhs: i64) -> i64 {
    lhs.wrapping_mul(rhs)
}

// Placeholder for Cast function
fn cast<T>(_obj: usize) -> T {
    // Placeholder
}

const KV8_MAX_WASM_STRING_LITERALS: usize = 256;

macro_rules! static_assert {
    ($condition:expr, $message:expr) => {
        const _: [(); 0 - !($condition) as usize] = [$message; 0];
    };
}

const SMI_VALUES_ARE_31_BITS: bool = true;
const SMI_VALUES_ARE_32_BITS: bool = false;

const K_SMI_TAG_SIZE: usize = 1;
const K_SMI_SHIFT_SIZE: usize = 0;

const UNREACHABLE_MSG: &str = "This code should not be reachable";

pub struct ConstantExpressionInterface<'a> {
    isolate_: &'a Isolate,
    outer_module_: &'a WasmModule,
    module_: &'a WasmModule,
    trusted_instance_data_: &'a WasmTrustedInstanceData,
    shared_trusted_instance_data_: &'a WasmTrustedInstanceData,
    generate_value_: bool,
    computed_value_: WasmValue,
    end_found_: bool,
    error_: Option<MessageTemplate>
}

impl<'a> ConstantExpressionInterface<'a> {
    pub fn new(
        isolate: &'a Isolate,
        outer_module: &'a WasmModule,
        module: &'a WasmModule,
        trusted_instance_data: &'a WasmTrustedInstanceData,
        shared_trusted_instance_data: &'a WasmTrustedInstanceData,
        generate_value: bool,
    ) -> Self {
        ConstantExpressionInterface {
            isolate_: isolate,
            outer_module_: outer_module,
            module_: module,
            trusted_instance_data_: trusted_instance_data,
            shared_trusted_instance_data_: shared_trusted_instance_data,
            generate_value_: generate_value,
            computed_value_: WasmValue::I32(0),
            end_found_: false,
            error_: None
        }
    }

    fn generate_value(&self) -> bool {
        self.generate_value_
    }

    pub fn i32_const(&mut self, _decoder: &mut FullDecoder, result: &mut Value, value: i32) {
        if self.generate_value() {
            result.runtime_value = WasmValue::I32(value);
        }
    }

    pub fn i64_const(&mut self, _decoder: &mut FullDecoder, result: &mut Value, value: i64) {
        if self.generate_value() {
            result.runtime_value = WasmValue::I64(value);
        }
    }

    pub fn f32_const(&mut self, _decoder: &mut FullDecoder, result: &mut Value, value: f32) {
        if self.generate_value() {
            result.runtime_value = WasmValue::F32(value);
        }
    }

    pub fn f64_const(&mut self, _decoder: &mut FullDecoder, result: &mut Value, value: f64) {
        if self.generate_value() {
            result.runtime_value = WasmValue::F64(value);
        }
    }

    pub fn s128_const(&mut self, _decoder: &mut FullDecoder, imm: &Simd128Immediate, result: &mut Value) {
        if !self.generate_value() {
            return;
        }
        result.runtime_value = WasmValue::S128(imm.value);
    }

    pub fn un_op(&mut self, _decoder: &mut FullDecoder, opcode: WasmOpcode, input: &Value, result: &mut Value) {
        if !self.generate_value() {
            return;
        }
        match opcode {
            WasmOpcode::kExprExternConvertAny => {
                result.runtime_value = WasmValue::Ref(wasm_to_js_object(self.isolate_, input.runtime_value.to_i32()));
            }
            WasmOpcode::kExprAnyConvertExtern => {
                let mut error_message: Option<&str> = None;
                match js_to_wasm_object(self.isolate_, input.runtime_value.to_i32(), RefType::kWasmAnyRef, &mut error_message) {
                    Ok(wasm_object) => {
                        result.runtime_value = WasmValue::Ref(wasm_object);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            _ => panic!("{}", UNREACHABLE_MSG),
        }
    }

    pub fn bin_op(&mut self, _decoder: &mut FullDecoder, opcode: WasmOpcode, lhs: &Value, rhs: &Value, result: &mut Value) {
        if !self.generate_value() {
            return;
        }
        match opcode {
            WasmOpcode::kExprI32Add => {
                result.runtime_value = WasmValue::I32(add_with_wraparound(lhs.runtime_value.to_i32(), rhs.runtime_value.to_i32()));
            }
            WasmOpcode::kExprI32Sub => {
                result.runtime_value = WasmValue::I32(sub_with_wraparound(lhs.runtime_value.to_i32(), rhs.runtime_value.to_i32()));
            }
            WasmOpcode::kExprI32Mul => {
                result.runtime_value = WasmValue::I32(mul_with_wraparound(lhs.runtime_value.to_i32(), rhs.runtime_value.to_i32()));
            }
            WasmOpcode::kExprI64Add => {
                result.runtime_value = WasmValue::I64(add_with_wraparound_i64(lhs.runtime_value.to_i64(), rhs.runtime_value.to_i64()));
            }
            WasmOpcode::kExprI64Sub => {
                result.runtime_value = WasmValue::I64(sub_with_wraparound_i64(lhs.runtime_value.to_i64(), rhs.runtime_value.to_i64()));
            }
            WasmOpcode::kExprI64Mul => {
                result.runtime_value = WasmValue::I64(mul_with_wraparound_i64(lhs.runtime_value.to_i64(), rhs.runtime_value.to_i64()));
            }
            _ => panic!("{}", UNREACHABLE_MSG),
        }
    }

    pub fn ref_null(&mut self, _decoder: &mut FullDecoder, type_: ValueType, result: &mut Value) {
        if !self.generate_value() {
            return;
        }

        result.runtime_value = WasmValue::Ref(self.isolate_.factory().wasm_null());
    }

    pub fn ref_func(&mut self, decoder: &mut FullDecoder, function_index: u32, result: &mut Value) {
        if self.isolate_ == unsafe { std::mem::transmute(std::ptr::null::<Isolate>()) } {
            decoder.module_.functions[function_index as usize].declared = true;
            return;
        }
        if !self.generate_value() {
            return;
        }
        let sig_index = decoder.module_.functions[function_index as usize].sig_index;
        let function_is_shared = decoder.module_.type_(sig_index).is_shared;
        let type_ = CanonicalValueType::Ref(decoder.module_.canonical_type_id(sig_index), function_is_shared, RefTypeKind::kFunction);
        let func_ref = WasmTrustedInstanceData::get_or_create_func_ref(
            self.isolate_,
            if function_is_shared {
                self.shared_trusted_instance_data_
            } else {
                self.trusted_instance_data_
            },
            function_index,
        );
        result.runtime_value = WasmValue::Ref(0); //fixme
    }

    pub fn global_get(&mut self, _decoder: &mut FullDecoder, result: &mut Value, imm: &GlobalIndexImmediate) {
        if !self.generate_value() {
            return;
        }
        let global = &self.module_.globals[imm.index];
        assert!(!global.mutability);
        let data = if global.shared {
            self.shared_trusted_instance_data_
        } else {
            self.trusted_instance_data_
        };
        let type_ = self.module_.canonical_type(global.ty);
        result.runtime_value = match type_ {
            CanonicalValueType::I32 => {
                let offset = global.offset;
                WasmValue::I32(0) //FIXME
            },
            _ => {
                WasmValue::Ref(0) //FIXME
            }
        };
    }

    pub fn struct_new(&mut self, decoder: &mut FullDecoder, imm: &StructIndexImmediate, args: &[Value], result: &mut Value) {
        if !self.generate_value() {
            return;
        }
        // let data = self.get_trusted_instance_data_for_type_index(imm.index);
        let mut field_values: Vec<WasmValue> = Vec::new();
        for i in 0..imm.struct_type.field_count() {
            field_values.push(args[i].runtime_value);
        }
        result.runtime_value = WasmValue::Ref(0); //fixme
    }

    pub fn string_const(&mut self, decoder: &mut FullDecoder, imm: &StringConstImmediate, result: &mut Value) {
        if !self.generate_value() {
            return;
        }

        let literal = &self.module_.stringref_literals[imm.index];
        let module_bytes = self.trusted_instance_data_.native_module().wire_bytes();
        let string_bytes = module_bytes.iter().skip(literal.source.offset).take(literal.source.end_offset - literal.source.offset).cloned().collect::<Vec<u8>>();
        let string = self.isolate_.factory().new_string_from_utf8(&string_bytes, 0).unwrap();
        result.runtime_value = WasmValue::Ref(0); //fixme
    }

    pub fn struct_new_default(&mut self, decoder: &mut FullDecoder, imm: &StructIndexImmediate, result: &mut Value) {
        if !self.generate_value() {
            return;
        }
        // let data = self.get_trusted_instance_data_for_type_index(imm.index);
        let mut field_values: Vec<WasmValue> = Vec::new();
        for i in 0..imm.struct_type.field_count() {
            field_values.push(default_value_for_type(imm.struct_type.field(i as u32), self.isolate_, self.module_));
        }
        result.runtime_value = WasmValue::Ref(0); //fixme
    }

    pub fn array_new(&mut self, decoder: &mut FullDecoder, imm: &ArrayIndexImmediate, length: &Value, initial_value: &Value, result: &mut Value) {
        if !self.generate_value() {
            return;
        }

        if length.runtime_value.to_u32() > WasmArray::max_length(&ArrayType{element_type_: ValueType::I32}) as u32 {
            self.error_ = Some(MessageTemplate::kWasmTrapArrayTooLarge);
            return;
        }

        result.runtime_value = WasmValue::Ref(0); //fixme
    }

    pub fn array_new_default(&mut self, decoder: &mut FullDecoder, imm: &ArrayIndexImmediate, length: &Value, result: &mut Value) {
        if !self.generate_value() {
            return;
        }

        let mut initial_value = Value::new(0, imm.array_type.element_type());
        initial_value.runtime_value = default_value_for_type(imm.array_type.element_type(), self.isolate_, decoder.module_);

        self.array_new(decoder, imm, length, &initial_value, result);
    }

    pub fn array_new_fixed(&mut self, decoder: &mut FullDecoder, array_imm: &ArrayIndexImmediate, length_imm: &IndexImmediate, elements: &[Value], result: &mut Value) {
        if !self.generate_value() {
            return;
        }

        // let data = self.get_trusted_instance_data_for_type_index(array_imm.index);

        let mut element_values: Vec<WasmValue> = Vec::new();
        for i in 0..length_imm.index {
            element_values.push(elements[i].runtime_value);
        }

        result.runtime_value = WasmValue::Ref(0); //fixme
    }

    pub fn array_new_segment(&mut self, decoder: &mut FullDecoder, array_imm: &ArrayIndexImmediate, segment_imm: &IndexImmediate, offset_value: &Value, length_value: &Value, result: &mut Value) {
        if !self.generate_value() {
            return;
        }

        // let data = self.get_trusted_instance_data_for_type_index(array_imm.index);
        let length = length_value.runtime_value.to_u32();
        let offset = offset_value.runtime_value.to_u32();

        if length > WasmArray::max_length(&ArrayType{element_type_: ValueType::I32}) as u32 {
            self.error_ = Some(MessageTemplate::kWasmTrapArrayTooLarge);
            return;
        }

        let element_type = CanonicalValueType::I32; //fixme
        let result_type = CanonicalValueType::I32; //fixme

        if true { //fixme element_type.is_numeric()
            let data_segment = &self.module_.data_segments[segment_imm.index];
            let length_in_bytes = length * 4; //fixme array_imm.array_type.element_type().value_kind_size();

            if true { //fixme !base::IsInBounds<uint32_t>(offset, length_in_bytes, data_segment.source.length())
                self.error_ = Some(MessageTemplate::kWasmTrapDataSegmentOutOfBounds);
                return;
            }

            // let source = data.data_segment_starts().get(segment_imm.index) + offset;

            //FIXME
        } else {
            let elem_segment = &decoder.module_.elem_segments[segment_imm.index];
            if true { //fixme !base::IsInBounds<size_t>(offset, length,elem_segment->status == WasmElemSegment::kStatusPassive
                self.error_ = Some(MessageTemplate::kWasmTrapElementSegmentOutOfBounds);
                return;
            }

            // FIXME
        }

        result.runtime_value = WasmValue::Ref(0); //FIXME
    }

    pub fn ref_i31(&mut self, _decoder: &mut FullDecoder, input: &Value, result: &mut Value) {
        if !self.generate_value() {
            return;
        }
        // Address raw = input.runtime_value.to_i32();
        // We have to craft the Smi manually because we accept out-of-bounds inputs.
        // For 32-bit Smi builds, set the topmost bit to sign-extend the second bit.
        // This way, interpretation in JS (if this value escapes there) will be the
        // same as i31.get_s.
        result.runtime_value = WasmValue::Ref(0); //fixme
    }

    pub fn do_return(&mut self, decoder: &mut FullDecoder, _drop_values: u32) {
        self.end_found_ = true;
        decoder.set_end(decoder.pc() + 1); // Placeholder for decoder.set_end()
        if self.generate_value() {
            self.computed_value_ = decoder.stack_value(1).runtime_value;
        }
    }

    fn get_trusted_instance_data_for_type_index(&self, index: ModuleTypeIndex) -> &'a WasmTrustedInstanceData {
        let type_is_shared = self.module_.type_(index).is_shared;
        if type_is_shared {
            self.shared_trusted_instance_data_
        } else {
            self.trusted_instance_data_
        }
    }
}

// Placeholder for WasmArray::MaxLength
// Placeholder for base::IsInBounds

// Placeholder for MessageTemplate enum. Needs more context.
#[derive(Debug, PartialEq)]
pub enum MessageTemplate {
    kWasmTrapArrayTooLarge,
    kWasmTrapDataSegmentOutOfBounds,
    kWasmTrapElementSegmentOutOfBounds,
}

impl FullDecoder {
    fn stack_value(&self, _i: i32) -> Value {
        Value{runtime_value: WasmValue::I32(0), ty: ValueType::I32}
    }

    fn set_end(&mut self, _pc: usize) {}

    fn pc(&self) -> usize { 0 }
}

fn default_value_for_type(type_: ValueType, isolate: &Isolate, module: &WasmModule) -> WasmValue {
    match type_.kind() {
        ValueKind::I32 | ValueKind::I8 | ValueKind::I16 => WasmValue::I32(0),
        ValueKind::I64 => WasmValue::I64(0),
        ValueKind::F16 | ValueKind::F32 => WasmValue::F32(0.0),
        ValueKind::F64